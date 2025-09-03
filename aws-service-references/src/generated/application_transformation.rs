// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum ApplicationTransformationActions {
    GetContainerization,
    GetDeployment,
    GetGroupingAssessment,
    GetPortingCompatibilityAssessment,
    GetPortingRecommendationAssessment,
    GetRuntimeAssessment,
    PutLogData,
    PutMetricData,
    StartContainerization,
    StartDeployment,
    StartGroupingAssessment,
    StartPortingCompatibilityAssessment,
    StartPortingRecommendationAssessment,
    StartRuntimeAssessment,
}
impl std::fmt::Display for ApplicationTransformationActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApplicationTransformationActions::GetContainerization => {
                write!(f, "application-transformation:GetContainerization")
            }
            ApplicationTransformationActions::GetDeployment => {
                write!(f, "application-transformation:GetDeployment")
            }
            ApplicationTransformationActions::GetGroupingAssessment => {
                write!(f, "application-transformation:GetGroupingAssessment")
            }
            ApplicationTransformationActions::GetPortingCompatibilityAssessment => write!(
                f,
                "application-transformation:GetPortingCompatibilityAssessment"
            ),
            ApplicationTransformationActions::GetPortingRecommendationAssessment => write!(
                f,
                "application-transformation:GetPortingRecommendationAssessment"
            ),
            ApplicationTransformationActions::GetRuntimeAssessment => {
                write!(f, "application-transformation:GetRuntimeAssessment")
            }
            ApplicationTransformationActions::PutLogData => {
                write!(f, "application-transformation:PutLogData")
            }
            ApplicationTransformationActions::PutMetricData => {
                write!(f, "application-transformation:PutMetricData")
            }
            ApplicationTransformationActions::StartContainerization => {
                write!(f, "application-transformation:StartContainerization")
            }
            ApplicationTransformationActions::StartDeployment => {
                write!(f, "application-transformation:StartDeployment")
            }
            ApplicationTransformationActions::StartGroupingAssessment => {
                write!(f, "application-transformation:StartGroupingAssessment")
            }
            ApplicationTransformationActions::StartPortingCompatibilityAssessment => write!(
                f,
                "application-transformation:StartPortingCompatibilityAssessment"
            ),
            ApplicationTransformationActions::StartPortingRecommendationAssessment => write!(
                f,
                "application-transformation:StartPortingRecommendationAssessment"
            ),
            ApplicationTransformationActions::StartRuntimeAssessment => {
                write!(f, "application-transformation:StartRuntimeAssessment")
            }
        }
    }
}
