// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum ObservabilityadminActions {
    CreateTelemetryRule,
    CreateTelemetryRuleForOrganization,
    DeleteTelemetryRule,
    DeleteTelemetryRuleForOrganization,
    GetTelemetryEvaluationStatus,
    GetTelemetryEvaluationStatusForOrganization,
    GetTelemetryRule,
    GetTelemetryRuleForOrganization,
    ListResourceTelemetry,
    ListResourceTelemetryForOrganization,
    ListTagsForResource,
    ListTelemetryRules,
    ListTelemetryRulesForOrganization,
    StartTelemetryEvaluation,
    StartTelemetryEvaluationForOrganization,
    StopTelemetryEvaluation,
    StopTelemetryEvaluationForOrganization,
    TagResource,
    UntagResource,
    UpdateTelemetryRule,
    UpdateTelemetryRuleForOrganization,
}
impl std::fmt::Display for ObservabilityadminActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ObservabilityadminActions::CreateTelemetryRule => {
                write!(f, "observabilityadmin:CreateTelemetryRule")
            }
            ObservabilityadminActions::CreateTelemetryRuleForOrganization => {
                write!(f, "observabilityadmin:CreateTelemetryRuleForOrganization")
            }
            ObservabilityadminActions::DeleteTelemetryRule => {
                write!(f, "observabilityadmin:DeleteTelemetryRule")
            }
            ObservabilityadminActions::DeleteTelemetryRuleForOrganization => {
                write!(f, "observabilityadmin:DeleteTelemetryRuleForOrganization")
            }
            ObservabilityadminActions::GetTelemetryEvaluationStatus => {
                write!(f, "observabilityadmin:GetTelemetryEvaluationStatus")
            }
            ObservabilityadminActions::GetTelemetryEvaluationStatusForOrganization => write!(
                f,
                "observabilityadmin:GetTelemetryEvaluationStatusForOrganization"
            ),
            ObservabilityadminActions::GetTelemetryRule => {
                write!(f, "observabilityadmin:GetTelemetryRule")
            }
            ObservabilityadminActions::GetTelemetryRuleForOrganization => {
                write!(f, "observabilityadmin:GetTelemetryRuleForOrganization")
            }
            ObservabilityadminActions::ListResourceTelemetry => {
                write!(f, "observabilityadmin:ListResourceTelemetry")
            }
            ObservabilityadminActions::ListResourceTelemetryForOrganization => {
                write!(f, "observabilityadmin:ListResourceTelemetryForOrganization")
            }
            ObservabilityadminActions::ListTagsForResource => {
                write!(f, "observabilityadmin:ListTagsForResource")
            }
            ObservabilityadminActions::ListTelemetryRules => {
                write!(f, "observabilityadmin:ListTelemetryRules")
            }
            ObservabilityadminActions::ListTelemetryRulesForOrganization => {
                write!(f, "observabilityadmin:ListTelemetryRulesForOrganization")
            }
            ObservabilityadminActions::StartTelemetryEvaluation => {
                write!(f, "observabilityadmin:StartTelemetryEvaluation")
            }
            ObservabilityadminActions::StartTelemetryEvaluationForOrganization => write!(
                f,
                "observabilityadmin:StartTelemetryEvaluationForOrganization"
            ),
            ObservabilityadminActions::StopTelemetryEvaluation => {
                write!(f, "observabilityadmin:StopTelemetryEvaluation")
            }
            ObservabilityadminActions::StopTelemetryEvaluationForOrganization => write!(
                f,
                "observabilityadmin:StopTelemetryEvaluationForOrganization"
            ),
            ObservabilityadminActions::TagResource => write!(f, "observabilityadmin:TagResource"),
            ObservabilityadminActions::UntagResource => {
                write!(f, "observabilityadmin:UntagResource")
            }
            ObservabilityadminActions::UpdateTelemetryRule => {
                write!(f, "observabilityadmin:UpdateTelemetryRule")
            }
            ObservabilityadminActions::UpdateTelemetryRuleForOrganization => {
                write!(f, "observabilityadmin:UpdateTelemetryRuleForOrganization")
            }
        }
    }
}
