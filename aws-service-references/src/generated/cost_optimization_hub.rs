// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum CostOptimizationHubActions {
    GetPreferences,
    GetRecommendation,
    ListEnrollmentStatuses,
    ListRecommendationSummaries,
    ListRecommendations,
    UpdateEnrollmentStatus,
    UpdatePreferences,
}
impl std::fmt::Display for CostOptimizationHubActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CostOptimizationHubActions::GetPreferences => {
                write!(f, "cost-optimization-hub:GetPreferences")
            }
            CostOptimizationHubActions::GetRecommendation => {
                write!(f, "cost-optimization-hub:GetRecommendation")
            }
            CostOptimizationHubActions::ListEnrollmentStatuses => {
                write!(f, "cost-optimization-hub:ListEnrollmentStatuses")
            }
            CostOptimizationHubActions::ListRecommendationSummaries => {
                write!(f, "cost-optimization-hub:ListRecommendationSummaries")
            }
            CostOptimizationHubActions::ListRecommendations => {
                write!(f, "cost-optimization-hub:ListRecommendations")
            }
            CostOptimizationHubActions::UpdateEnrollmentStatus => {
                write!(f, "cost-optimization-hub:UpdateEnrollmentStatus")
            }
            CostOptimizationHubActions::UpdatePreferences => {
                write!(f, "cost-optimization-hub:UpdatePreferences")
            }
        }
    }
}
