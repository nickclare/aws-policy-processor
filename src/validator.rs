//! Basic validation rules for policy / policy statements

use crate::model::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ErrorKind {
    ResourceNotPermitted,
    PrincipalNotPermitted,
    ConditionNotPermitted,
    InvalidResource(String),
    InvalidAction(String),
    NotActionNotPermitted,
}

#[derive(Clone, Debug)]
pub struct Diagnostic {
    pub error: ErrorKind,
    pub sid: Option<String>,
}

impl Diagnostic {
    pub fn for_statement(statement: &Statement, error: ErrorKind) -> Self {
        Self {
            error,
            sid: statement.sid.clone(),
        }
    }
}

impl Statement {
    pub fn diagnostic(&self, error: ErrorKind) -> Diagnostic {
        Diagnostic::for_statement(self, error)
    }
}

/// Validate a single statement, according to some set of rules for its policy type
pub fn validate_statement(statement: &Statement, kind: PolicyKind) -> Vec<Diagnostic> {
    todo!()
}

fn validate_resource_statement(statement: &Statement) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    if statement.resources.is_some() {
        diagnostics.push(statement.diagnostic(ErrorKind::ResourceNotPermitted))
    }

    diagnostics
}

fn validate_service_control_statement(statement: &Statement) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    // Allow statements have a number of limitations
    if statement.effect == Effect::Allow {
        if matches!(statement.actions, Actions::NotActions(_)) {
            diagnostics.push(statement.diagnostic(ErrorKind::NotActionNotPermitted));
        }
        if matches!(statement.condition, Some(_)) {
            diagnostics.push(statement.diagnostic(ErrorKind::ConditionNotPermitted));
        }
        // todo: make this allow `"Resource": "*"`
        if matches!(statement.resources, Some(_)) {
            diagnostics.push(statement.diagnostic(ErrorKind::ResourceNotPermitted));
        }
    }

    diagnostics
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
        let diagnostics = validate_service_control_statement(&statement);

        let error_of_kind = |kind: ErrorKind| diagnostics.iter().find(|e| e.error == kind);
        assert!(
            error_of_kind(ErrorKind::NotActionNotPermitted).is_some(),
            "expected an error that NotAction is not allowed"
        );
        assert!(
            error_of_kind(ErrorKind::PrincipalNotPermitted).is_none(),
            "didn't expect a PrincipalNotPermitted error"
        );

        Ok(())
    }
}
