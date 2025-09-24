// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum ObservabilityadminActions {
    CreateCentralizationRuleForOrganization,
    CreateTelemetryRule,
    CreateTelemetryRuleForOrganization,
    DeleteCentralizationRuleForOrganization,
    DeleteTelemetryRule,
    DeleteTelemetryRuleForOrganization,
    GetCentralizationRuleForOrganization,
    GetTelemetryEnrichmentStatus,
    GetTelemetryEvaluationStatus,
    GetTelemetryEvaluationStatusForOrganization,
    GetTelemetryRule,
    GetTelemetryRuleForOrganization,
    ListCentralizationRulesForOrganization,
    ListResourceTelemetry,
    ListResourceTelemetryForOrganization,
    ListTagsForResource,
    ListTelemetryRules,
    ListTelemetryRulesForOrganization,
    StartTelemetryEnrichment,
    StartTelemetryEvaluation,
    StartTelemetryEvaluationForOrganization,
    StopTelemetryEnrichment,
    StopTelemetryEvaluation,
    StopTelemetryEvaluationForOrganization,
    TagResource,
    UntagResource,
    UpdateCentralizationRuleForOrganization,
    UpdateTelemetryRule,
    UpdateTelemetryRuleForOrganization,
}
impl std::fmt::Display for ObservabilityadminActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ObservabilityadminActions::CreateCentralizationRuleForOrganization => write!(
                f,
                "observabilityadmin:CreateCentralizationRuleForOrganization"
            ),
            ObservabilityadminActions::CreateTelemetryRule => {
                write!(f, "observabilityadmin:CreateTelemetryRule")
            }
            ObservabilityadminActions::CreateTelemetryRuleForOrganization => {
                write!(f, "observabilityadmin:CreateTelemetryRuleForOrganization")
            }
            ObservabilityadminActions::DeleteCentralizationRuleForOrganization => write!(
                f,
                "observabilityadmin:DeleteCentralizationRuleForOrganization"
            ),
            ObservabilityadminActions::DeleteTelemetryRule => {
                write!(f, "observabilityadmin:DeleteTelemetryRule")
            }
            ObservabilityadminActions::DeleteTelemetryRuleForOrganization => {
                write!(f, "observabilityadmin:DeleteTelemetryRuleForOrganization")
            }
            ObservabilityadminActions::GetCentralizationRuleForOrganization => {
                write!(f, "observabilityadmin:GetCentralizationRuleForOrganization")
            }
            ObservabilityadminActions::GetTelemetryEnrichmentStatus => {
                write!(f, "observabilityadmin:GetTelemetryEnrichmentStatus")
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
            ObservabilityadminActions::ListCentralizationRulesForOrganization => write!(
                f,
                "observabilityadmin:ListCentralizationRulesForOrganization"
            ),
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
            ObservabilityadminActions::StartTelemetryEnrichment => {
                write!(f, "observabilityadmin:StartTelemetryEnrichment")
            }
            ObservabilityadminActions::StartTelemetryEvaluation => {
                write!(f, "observabilityadmin:StartTelemetryEvaluation")
            }
            ObservabilityadminActions::StartTelemetryEvaluationForOrganization => write!(
                f,
                "observabilityadmin:StartTelemetryEvaluationForOrganization"
            ),
            ObservabilityadminActions::StopTelemetryEnrichment => {
                write!(f, "observabilityadmin:StopTelemetryEnrichment")
            }
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
            ObservabilityadminActions::UpdateCentralizationRuleForOrganization => write!(
                f,
                "observabilityadmin:UpdateCentralizationRuleForOrganization"
            ),
            ObservabilityadminActions::UpdateTelemetryRule => {
                write!(f, "observabilityadmin:UpdateTelemetryRule")
            }
            ObservabilityadminActions::UpdateTelemetryRuleForOrganization => {
                write!(f, "observabilityadmin:UpdateTelemetryRuleForOrganization")
            }
        }
    }
}
