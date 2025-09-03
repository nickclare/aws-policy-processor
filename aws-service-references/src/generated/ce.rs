// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum CeActions {
    CreateAnomalyMonitor,
    CreateAnomalySubscription,
    CreateCostCategoryDefinition,
    CreateNotificationSubscription,
    CreateReport,
    DeleteAnomalyMonitor,
    DeleteAnomalySubscription,
    DeleteCostCategoryDefinition,
    DeleteNotificationSubscription,
    DeleteReport,
    DescribeCostCategoryDefinition,
    DescribeNotificationSubscription,
    DescribeReport,
    GetAnomalies,
    GetAnomalyMonitors,
    GetAnomalySubscriptions,
    GetApproximateUsageRecords,
    GetCommitmentPurchaseAnalysis,
    GetConsoleActionSetEnforced,
    GetCostAndUsage,
    GetCostAndUsageComparisons,
    GetCostAndUsageWithResources,
    GetCostCategories,
    GetCostComparisonDrivers,
    GetCostForecast,
    GetDimensionValues,
    GetPreferences,
    GetReservationCoverage,
    GetReservationPurchaseRecommendation,
    GetReservationUtilization,
    GetRightsizingRecommendation,
    GetSavingsPlanPurchaseRecommendationDetails,
    GetSavingsPlansCoverage,
    GetSavingsPlansPurchaseRecommendation,
    GetSavingsPlansUtilization,
    GetSavingsPlansUtilizationDetails,
    GetTags,
    GetUsageForecast,
    ListCommitmentPurchaseAnalyses,
    ListCostAllocationTagBackfillHistory,
    ListCostAllocationTags,
    ListCostCategoryDefinitions,
    ListSavingsPlansPurchaseRecommendationGeneration,
    ListTagsForResource,
    ProvideAnomalyFeedback,
    StartCommitmentPurchaseAnalysis,
    StartCostAllocationTagBackfill,
    StartSavingsPlansPurchaseRecommendationGeneration,
    TagResource,
    UntagResource,
    UpdateAnomalyMonitor,
    UpdateAnomalySubscription,
    UpdateConsoleActionSetEnforced,
    UpdateCostAllocationTagsStatus,
    UpdateCostCategoryDefinition,
    UpdateNotificationSubscription,
    UpdatePreferences,
    UpdateReport,
}
impl std::fmt::Display for CeActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CeActions::CreateAnomalyMonitor => write!(f, "ce:CreateAnomalyMonitor"),
            CeActions::CreateAnomalySubscription => write!(f, "ce:CreateAnomalySubscription"),
            CeActions::CreateCostCategoryDefinition => write!(f, "ce:CreateCostCategoryDefinition"),
            CeActions::CreateNotificationSubscription => {
                write!(f, "ce:CreateNotificationSubscription")
            }
            CeActions::CreateReport => write!(f, "ce:CreateReport"),
            CeActions::DeleteAnomalyMonitor => write!(f, "ce:DeleteAnomalyMonitor"),
            CeActions::DeleteAnomalySubscription => write!(f, "ce:DeleteAnomalySubscription"),
            CeActions::DeleteCostCategoryDefinition => write!(f, "ce:DeleteCostCategoryDefinition"),
            CeActions::DeleteNotificationSubscription => {
                write!(f, "ce:DeleteNotificationSubscription")
            }
            CeActions::DeleteReport => write!(f, "ce:DeleteReport"),
            CeActions::DescribeCostCategoryDefinition => {
                write!(f, "ce:DescribeCostCategoryDefinition")
            }
            CeActions::DescribeNotificationSubscription => {
                write!(f, "ce:DescribeNotificationSubscription")
            }
            CeActions::DescribeReport => write!(f, "ce:DescribeReport"),
            CeActions::GetAnomalies => write!(f, "ce:GetAnomalies"),
            CeActions::GetAnomalyMonitors => write!(f, "ce:GetAnomalyMonitors"),
            CeActions::GetAnomalySubscriptions => write!(f, "ce:GetAnomalySubscriptions"),
            CeActions::GetApproximateUsageRecords => write!(f, "ce:GetApproximateUsageRecords"),
            CeActions::GetCommitmentPurchaseAnalysis => {
                write!(f, "ce:GetCommitmentPurchaseAnalysis")
            }
            CeActions::GetConsoleActionSetEnforced => write!(f, "ce:GetConsoleActionSetEnforced"),
            CeActions::GetCostAndUsage => write!(f, "ce:GetCostAndUsage"),
            CeActions::GetCostAndUsageComparisons => write!(f, "ce:GetCostAndUsageComparisons"),
            CeActions::GetCostAndUsageWithResources => write!(f, "ce:GetCostAndUsageWithResources"),
            CeActions::GetCostCategories => write!(f, "ce:GetCostCategories"),
            CeActions::GetCostComparisonDrivers => write!(f, "ce:GetCostComparisonDrivers"),
            CeActions::GetCostForecast => write!(f, "ce:GetCostForecast"),
            CeActions::GetDimensionValues => write!(f, "ce:GetDimensionValues"),
            CeActions::GetPreferences => write!(f, "ce:GetPreferences"),
            CeActions::GetReservationCoverage => write!(f, "ce:GetReservationCoverage"),
            CeActions::GetReservationPurchaseRecommendation => {
                write!(f, "ce:GetReservationPurchaseRecommendation")
            }
            CeActions::GetReservationUtilization => write!(f, "ce:GetReservationUtilization"),
            CeActions::GetRightsizingRecommendation => write!(f, "ce:GetRightsizingRecommendation"),
            CeActions::GetSavingsPlanPurchaseRecommendationDetails => {
                write!(f, "ce:GetSavingsPlanPurchaseRecommendationDetails")
            }
            CeActions::GetSavingsPlansCoverage => write!(f, "ce:GetSavingsPlansCoverage"),
            CeActions::GetSavingsPlansPurchaseRecommendation => {
                write!(f, "ce:GetSavingsPlansPurchaseRecommendation")
            }
            CeActions::GetSavingsPlansUtilization => write!(f, "ce:GetSavingsPlansUtilization"),
            CeActions::GetSavingsPlansUtilizationDetails => {
                write!(f, "ce:GetSavingsPlansUtilizationDetails")
            }
            CeActions::GetTags => write!(f, "ce:GetTags"),
            CeActions::GetUsageForecast => write!(f, "ce:GetUsageForecast"),
            CeActions::ListCommitmentPurchaseAnalyses => {
                write!(f, "ce:ListCommitmentPurchaseAnalyses")
            }
            CeActions::ListCostAllocationTagBackfillHistory => {
                write!(f, "ce:ListCostAllocationTagBackfillHistory")
            }
            CeActions::ListCostAllocationTags => write!(f, "ce:ListCostAllocationTags"),
            CeActions::ListCostCategoryDefinitions => write!(f, "ce:ListCostCategoryDefinitions"),
            CeActions::ListSavingsPlansPurchaseRecommendationGeneration => {
                write!(f, "ce:ListSavingsPlansPurchaseRecommendationGeneration")
            }
            CeActions::ListTagsForResource => write!(f, "ce:ListTagsForResource"),
            CeActions::ProvideAnomalyFeedback => write!(f, "ce:ProvideAnomalyFeedback"),
            CeActions::StartCommitmentPurchaseAnalysis => {
                write!(f, "ce:StartCommitmentPurchaseAnalysis")
            }
            CeActions::StartCostAllocationTagBackfill => {
                write!(f, "ce:StartCostAllocationTagBackfill")
            }
            CeActions::StartSavingsPlansPurchaseRecommendationGeneration => {
                write!(f, "ce:StartSavingsPlansPurchaseRecommendationGeneration")
            }
            CeActions::TagResource => write!(f, "ce:TagResource"),
            CeActions::UntagResource => write!(f, "ce:UntagResource"),
            CeActions::UpdateAnomalyMonitor => write!(f, "ce:UpdateAnomalyMonitor"),
            CeActions::UpdateAnomalySubscription => write!(f, "ce:UpdateAnomalySubscription"),
            CeActions::UpdateConsoleActionSetEnforced => {
                write!(f, "ce:UpdateConsoleActionSetEnforced")
            }
            CeActions::UpdateCostAllocationTagsStatus => {
                write!(f, "ce:UpdateCostAllocationTagsStatus")
            }
            CeActions::UpdateCostCategoryDefinition => write!(f, "ce:UpdateCostCategoryDefinition"),
            CeActions::UpdateNotificationSubscription => {
                write!(f, "ce:UpdateNotificationSubscription")
            }
            CeActions::UpdatePreferences => write!(f, "ce:UpdatePreferences"),
            CeActions::UpdateReport => write!(f, "ce:UpdateReport"),
        }
    }
}
