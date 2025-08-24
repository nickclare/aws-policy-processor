// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum DevopsGuruActions {
    AddNotificationChannel,
    DeleteInsight,
    DescribeAccountHealth,
    DescribeAccountOverview,
    DescribeAnomaly,
    DescribeEventSourcesConfig,
    DescribeFeedback,
    DescribeInsight,
    DescribeOrganizationHealth,
    DescribeOrganizationOverview,
    DescribeOrganizationResourceCollectionHealth,
    DescribeResourceCollectionHealth,
    DescribeServiceIntegration,
    GetCostEstimation,
    GetResourceCollection,
    ListAnomaliesForInsight,
    ListAnomalousLogGroups,
    ListEvents,
    ListInsights,
    ListMonitoredResources,
    ListNotificationChannels,
    ListOrganizationInsights,
    ListRecommendations,
    PutFeedback,
    RemoveNotificationChannel,
    SearchInsights,
    SearchOrganizationInsights,
    StartCostEstimation,
    UpdateEventSourcesConfig,
    UpdateResourceCollection,
    UpdateServiceIntegration,
}
impl std::fmt::Display for DevopsGuruActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DevopsGuruActions::AddNotificationChannel => {
                write!(f, "devops-guru:AddNotificationChannel")
            }
            DevopsGuruActions::DeleteInsight => write!(f, "devops-guru:DeleteInsight"),
            DevopsGuruActions::DescribeAccountHealth => {
                write!(f, "devops-guru:DescribeAccountHealth")
            }
            DevopsGuruActions::DescribeAccountOverview => {
                write!(f, "devops-guru:DescribeAccountOverview")
            }
            DevopsGuruActions::DescribeAnomaly => write!(f, "devops-guru:DescribeAnomaly"),
            DevopsGuruActions::DescribeEventSourcesConfig => {
                write!(f, "devops-guru:DescribeEventSourcesConfig")
            }
            DevopsGuruActions::DescribeFeedback => write!(f, "devops-guru:DescribeFeedback"),
            DevopsGuruActions::DescribeInsight => write!(f, "devops-guru:DescribeInsight"),
            DevopsGuruActions::DescribeOrganizationHealth => {
                write!(f, "devops-guru:DescribeOrganizationHealth")
            }
            DevopsGuruActions::DescribeOrganizationOverview => {
                write!(f, "devops-guru:DescribeOrganizationOverview")
            }
            DevopsGuruActions::DescribeOrganizationResourceCollectionHealth => write!(
                f,
                "devops-guru:DescribeOrganizationResourceCollectionHealth"
            ),
            DevopsGuruActions::DescribeResourceCollectionHealth => {
                write!(f, "devops-guru:DescribeResourceCollectionHealth")
            }
            DevopsGuruActions::DescribeServiceIntegration => {
                write!(f, "devops-guru:DescribeServiceIntegration")
            }
            DevopsGuruActions::GetCostEstimation => write!(f, "devops-guru:GetCostEstimation"),
            DevopsGuruActions::GetResourceCollection => {
                write!(f, "devops-guru:GetResourceCollection")
            }
            DevopsGuruActions::ListAnomaliesForInsight => {
                write!(f, "devops-guru:ListAnomaliesForInsight")
            }
            DevopsGuruActions::ListAnomalousLogGroups => {
                write!(f, "devops-guru:ListAnomalousLogGroups")
            }
            DevopsGuruActions::ListEvents => write!(f, "devops-guru:ListEvents"),
            DevopsGuruActions::ListInsights => write!(f, "devops-guru:ListInsights"),
            DevopsGuruActions::ListMonitoredResources => {
                write!(f, "devops-guru:ListMonitoredResources")
            }
            DevopsGuruActions::ListNotificationChannels => {
                write!(f, "devops-guru:ListNotificationChannels")
            }
            DevopsGuruActions::ListOrganizationInsights => {
                write!(f, "devops-guru:ListOrganizationInsights")
            }
            DevopsGuruActions::ListRecommendations => write!(f, "devops-guru:ListRecommendations"),
            DevopsGuruActions::PutFeedback => write!(f, "devops-guru:PutFeedback"),
            DevopsGuruActions::RemoveNotificationChannel => {
                write!(f, "devops-guru:RemoveNotificationChannel")
            }
            DevopsGuruActions::SearchInsights => write!(f, "devops-guru:SearchInsights"),
            DevopsGuruActions::SearchOrganizationInsights => {
                write!(f, "devops-guru:SearchOrganizationInsights")
            }
            DevopsGuruActions::StartCostEstimation => write!(f, "devops-guru:StartCostEstimation"),
            DevopsGuruActions::UpdateEventSourcesConfig => {
                write!(f, "devops-guru:UpdateEventSourcesConfig")
            }
            DevopsGuruActions::UpdateResourceCollection => {
                write!(f, "devops-guru:UpdateResourceCollection")
            }
            DevopsGuruActions::UpdateServiceIntegration => {
                write!(f, "devops-guru:UpdateServiceIntegration")
            }
        }
    }
}
