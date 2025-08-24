// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum FisActions {
    CreateExperimentTemplate,
    CreateTargetAccountConfiguration,
    DeleteExperimentTemplate,
    DeleteTargetAccountConfiguration,
    GetAction,
    GetExperiment,
    GetExperimentTargetAccountConfiguration,
    GetExperimentTemplate,
    GetSafetyLever,
    GetTargetAccountConfiguration,
    GetTargetResourceType,
    InjectApiInternalError,
    InjectApiThrottleError,
    InjectApiUnavailableError,
    ListActions,
    ListExperimentResolvedTargets,
    ListExperimentTargetAccountConfigurations,
    ListExperimentTemplates,
    ListExperiments,
    ListTagsForResource,
    ListTargetAccountConfigurations,
    ListTargetResourceTypes,
    StartExperiment,
    StopExperiment,
    TagResource,
    UntagResource,
    UpdateExperimentTemplate,
    UpdateSafetyLeverState,
    UpdateTargetAccountConfiguration,
}
impl std::fmt::Display for FisActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FisActions::CreateExperimentTemplate => write!(f, "fis:CreateExperimentTemplate"),
            FisActions::CreateTargetAccountConfiguration => {
                write!(f, "fis:CreateTargetAccountConfiguration")
            }
            FisActions::DeleteExperimentTemplate => write!(f, "fis:DeleteExperimentTemplate"),
            FisActions::DeleteTargetAccountConfiguration => {
                write!(f, "fis:DeleteTargetAccountConfiguration")
            }
            FisActions::GetAction => write!(f, "fis:GetAction"),
            FisActions::GetExperiment => write!(f, "fis:GetExperiment"),
            FisActions::GetExperimentTargetAccountConfiguration => {
                write!(f, "fis:GetExperimentTargetAccountConfiguration")
            }
            FisActions::GetExperimentTemplate => write!(f, "fis:GetExperimentTemplate"),
            FisActions::GetSafetyLever => write!(f, "fis:GetSafetyLever"),
            FisActions::GetTargetAccountConfiguration => {
                write!(f, "fis:GetTargetAccountConfiguration")
            }
            FisActions::GetTargetResourceType => write!(f, "fis:GetTargetResourceType"),
            FisActions::InjectApiInternalError => write!(f, "fis:InjectApiInternalError"),
            FisActions::InjectApiThrottleError => write!(f, "fis:InjectApiThrottleError"),
            FisActions::InjectApiUnavailableError => write!(f, "fis:InjectApiUnavailableError"),
            FisActions::ListActions => write!(f, "fis:ListActions"),
            FisActions::ListExperimentResolvedTargets => {
                write!(f, "fis:ListExperimentResolvedTargets")
            }
            FisActions::ListExperimentTargetAccountConfigurations => {
                write!(f, "fis:ListExperimentTargetAccountConfigurations")
            }
            FisActions::ListExperimentTemplates => write!(f, "fis:ListExperimentTemplates"),
            FisActions::ListExperiments => write!(f, "fis:ListExperiments"),
            FisActions::ListTagsForResource => write!(f, "fis:ListTagsForResource"),
            FisActions::ListTargetAccountConfigurations => {
                write!(f, "fis:ListTargetAccountConfigurations")
            }
            FisActions::ListTargetResourceTypes => write!(f, "fis:ListTargetResourceTypes"),
            FisActions::StartExperiment => write!(f, "fis:StartExperiment"),
            FisActions::StopExperiment => write!(f, "fis:StopExperiment"),
            FisActions::TagResource => write!(f, "fis:TagResource"),
            FisActions::UntagResource => write!(f, "fis:UntagResource"),
            FisActions::UpdateExperimentTemplate => write!(f, "fis:UpdateExperimentTemplate"),
            FisActions::UpdateSafetyLeverState => write!(f, "fis:UpdateSafetyLeverState"),
            FisActions::UpdateTargetAccountConfiguration => {
                write!(f, "fis:UpdateTargetAccountConfiguration")
            }
        }
    }
}
