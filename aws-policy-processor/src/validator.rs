//! Basic validation rules for policy / policy statements

use miette::{Diagnostic, Result};
use thiserror::Error;

use crate::model::*;

#[derive(Clone, Debug, PartialEq, Eq, Error, Diagnostic)]
pub struct ValidationError {
    sid: Option<String>,

    #[related]
    errors: Vec<ValidationErrorKind>,
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.sid {
            None => write!(f, "validation error."),
            Some(sid) => write!(f, "validation error in statement `{sid}`."),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Error, Diagnostic)]
pub enum ValidationErrorKind {
    #[error("`Condition` not permitted")]
    ConditionNotPermitted,
    #[error("Action value '{0}' is invalid")]
    InvalidAction(String),
    #[error("Resource value '{0}' is invalid")]
    InvalidResource(String),
    #[error("`NotAction` not permitted")]
    NotActionNotPermitted,
    #[error("`Principal` not permitted")]
    PrincipalNotPermitted,
    #[error("`Resource` not permitted")]
    ResourceNotPermitted,
}

trait ValidationRule {
    fn validate(&self, statement: &Statement) -> Vec<ValidationErrorKind>;
}

struct FnValidationRule(Box<dyn Fn(&Statement) -> Vec<ValidationErrorKind>>);

impl ValidationRule for FnValidationRule {
    fn validate(&self, statement: &Statement) -> Vec<ValidationErrorKind> {
        self.0(statement)
    }
}

impl<F> From<F> for FnValidationRule
where
    F: Fn(&Statement) -> Vec<ValidationErrorKind> + 'static,
{
    fn from(value: F) -> Self {
        FnValidationRule(Box::new(value))
    }
}

struct RuleSet {
    rules: Vec<Box<dyn ValidationRule>>,
}

impl RuleSet {
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }

    pub fn append<R: ValidationRule + 'static>(&mut self, rule: R) -> &mut Self {
        self.rules.push(Box::new(rule));
        self
    }

    pub fn append_fn<F: Into<FnValidationRule>>(&mut self, rule: F) -> &mut Self {
        self.append(rule.into())
    }
}

impl Default for RuleSet {
    fn default() -> Self {
        Self::new()
    }
}

mod rules {
    use std::sync::LazyLock;

    use regex::Regex;

    use super::*;

    /// Check if the statement has a (Not)Resource value.
    pub(crate) fn resource_not_permitted(statement: &Statement) -> Vec<ValidationErrorKind> {
        if let Some(ref _resources) = statement.resources {
            // todo: check its not just a `*`, which may be allowed
            vec![ValidationErrorKind::ResourceNotPermitted]
        } else {
            vec![]
        }
    }

    /// Check if the statement has NotAction instead of Action.
    pub(crate) fn not_actions_not_permitted(statement: &Statement) -> Vec<ValidationErrorKind> {
        if matches!(statement.actions, Actions::NotActions(_)) {
            vec![ValidationErrorKind::NotActionNotPermitted]
        } else {
            vec![]
        }
    }

    /// Check if the statement has any condition element
    pub(crate) fn condition_not_permitted(statement: &Statement) -> Vec<ValidationErrorKind> {
        if let Some(ref _condition) = statement.condition {
            vec![ValidationErrorKind::ConditionNotPermitted]
        } else {
            vec![]
        }
    }

    /// Check that all actions only have wildcards at the end of the action name.
    pub(crate) fn wildcards_only_at_end(statement: &Statement) -> Vec<ValidationErrorKind> {
        static ALLOWED_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new(r"^[a-zA-Z0-9]+:[a-zA-Z0-9]+(\*?)$").expect("regex should compile")
        });
        statement
            .actions
            .get_actions()
            .filter_map(|action| {
                if !ALLOWED_PATTERN.is_match(action) {
                    Some(ValidationErrorKind::InvalidAction(action.clone()))
                } else {
                    None
                }
            })
            .collect()
    }
}

/// Validate a single statement, according to some set of rules for its policy type
pub fn validate_statement(statement: &Statement, kind: PolicyKind) -> Result<()> {
    match kind {
        PolicyKind::Iam => todo!(),
        PolicyKind::Resource => validate_resource_statement(statement),
        PolicyKind::ServiceControl => validate_service_control_statement(statement),
        PolicyKind::ResourceControl => todo!(),
    }
}

/// Validate the given statement against a [RuleSet]
fn validate_against_rules(statement: &Statement, rules: &RuleSet) -> Result<()> {
    let mut errors = Vec::new();
    for r in &rules.rules {
        errors.append(&mut r.validate(statement));
    }

    into_result(errors, &statement.sid)
}

macro_rules! ruleset {
    ( $( $r:expr ),*) => {
        {
            let mut result = RuleSet::default();
            $(
                result.append_fn($r);
            )*
            result
        }
    };
}

fn into_result(errors: Vec<ValidationErrorKind>, sid: &Option<String>) -> Result<()> {
    if !errors.is_empty() {
        Err(ValidationError {
            sid: sid.clone(),
            errors,
        }
        .into())
    } else {
        Ok(())
    }
}

fn validate_resource_statement(statement: &Statement) -> Result<()> {
    validate_against_rules(statement, &ruleset![rules::resource_not_permitted])
}

fn validate_service_control_statement(statement: &Statement) -> Result<()> {
    use rules::*;
    match statement.effect {
        Effect::Allow => validate_against_rules(
            statement,
            &ruleset![
                not_actions_not_permitted,
                condition_not_permitted,
                resource_not_permitted,
                wildcards_only_at_end
            ],
        ),
        Effect::Deny => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use color_eyre::eyre;
    use indoc::indoc;

    #[test]
    fn test_scp_validation() -> eyre::Result<()> {
        let invalid_policy = indoc! {r#"
            {
                "Sid": "WeirdStatement",
                "Effect": "Allow",
                "Resource": "my-arn",
                "Condition": "panic",
                "NotAction": [
                    "ec2:panic",
                    "rds:deletealldata"
                ]
            }                    
        "#};
        let statement: Statement = serde_json::from_str(invalid_policy)?;
        if let Err(err) = validate_service_control_statement(&statement) {
            let err = err.downcast::<ValidationError>().unwrap();
            assert_eq!(err.sid, Some("WeirdStatement".into()));
            let diagnostics = err.errors;

            let error_of_kind =
                |kind: ValidationErrorKind| diagnostics.iter().find(|e| **e == kind);
            assert!(
                error_of_kind(ValidationErrorKind::NotActionNotPermitted).is_some(),
                "expected an error that NotAction is not allowed"
            );
            assert!(
                error_of_kind(ValidationErrorKind::PrincipalNotPermitted).is_none(),
                "didn't expect a PrincipalNotPermitted error"
            );
        }

        Ok(())
    }
}
