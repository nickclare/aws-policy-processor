// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum DiscoveryActions {
    AssociateConfigurationItemsToApplication,
    BatchDeleteAgents,
    BatchDeleteImportData,
    CreateApplication,
    CreateTags,
    DeleteApplications,
    DeleteTags,
    DescribeAgents,
    DescribeBatchDeleteConfigurationTask,
    DescribeConfigurations,
    DescribeContinuousExports,
    DescribeExportConfigurations,
    DescribeExportTasks,
    DescribeImportTasks,
    DescribeTags,
    DisassociateConfigurationItemsFromApplication,
    ExportConfigurations,
    GetDiscoverySummary,
    GetNetworkConnectionGraph,
    ListConfigurations,
    ListServerNeighbors,
    StartBatchDeleteConfigurationTask,
    StartContinuousExport,
    StartDataCollectionByAgentIds,
    StartExportTask,
    StartImportTask,
    StopContinuousExport,
    StopDataCollectionByAgentIds,
    UpdateApplication,
}
impl std::fmt::Display for DiscoveryActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DiscoveryActions::AssociateConfigurationItemsToApplication => {
                write!(f, "discovery:AssociateConfigurationItemsToApplication")
            }
            DiscoveryActions::BatchDeleteAgents => write!(f, "discovery:BatchDeleteAgents"),
            DiscoveryActions::BatchDeleteImportData => write!(f, "discovery:BatchDeleteImportData"),
            DiscoveryActions::CreateApplication => write!(f, "discovery:CreateApplication"),
            DiscoveryActions::CreateTags => write!(f, "discovery:CreateTags"),
            DiscoveryActions::DeleteApplications => write!(f, "discovery:DeleteApplications"),
            DiscoveryActions::DeleteTags => write!(f, "discovery:DeleteTags"),
            DiscoveryActions::DescribeAgents => write!(f, "discovery:DescribeAgents"),
            DiscoveryActions::DescribeBatchDeleteConfigurationTask => {
                write!(f, "discovery:DescribeBatchDeleteConfigurationTask")
            }
            DiscoveryActions::DescribeConfigurations => {
                write!(f, "discovery:DescribeConfigurations")
            }
            DiscoveryActions::DescribeContinuousExports => {
                write!(f, "discovery:DescribeContinuousExports")
            }
            DiscoveryActions::DescribeExportConfigurations => {
                write!(f, "discovery:DescribeExportConfigurations")
            }
            DiscoveryActions::DescribeExportTasks => write!(f, "discovery:DescribeExportTasks"),
            DiscoveryActions::DescribeImportTasks => write!(f, "discovery:DescribeImportTasks"),
            DiscoveryActions::DescribeTags => write!(f, "discovery:DescribeTags"),
            DiscoveryActions::DisassociateConfigurationItemsFromApplication => {
                write!(f, "discovery:DisassociateConfigurationItemsFromApplication")
            }
            DiscoveryActions::ExportConfigurations => write!(f, "discovery:ExportConfigurations"),
            DiscoveryActions::GetDiscoverySummary => write!(f, "discovery:GetDiscoverySummary"),
            DiscoveryActions::GetNetworkConnectionGraph => {
                write!(f, "discovery:GetNetworkConnectionGraph")
            }
            DiscoveryActions::ListConfigurations => write!(f, "discovery:ListConfigurations"),
            DiscoveryActions::ListServerNeighbors => write!(f, "discovery:ListServerNeighbors"),
            DiscoveryActions::StartBatchDeleteConfigurationTask => {
                write!(f, "discovery:StartBatchDeleteConfigurationTask")
            }
            DiscoveryActions::StartContinuousExport => write!(f, "discovery:StartContinuousExport"),
            DiscoveryActions::StartDataCollectionByAgentIds => {
                write!(f, "discovery:StartDataCollectionByAgentIds")
            }
            DiscoveryActions::StartExportTask => write!(f, "discovery:StartExportTask"),
            DiscoveryActions::StartImportTask => write!(f, "discovery:StartImportTask"),
            DiscoveryActions::StopContinuousExport => write!(f, "discovery:StopContinuousExport"),
            DiscoveryActions::StopDataCollectionByAgentIds => {
                write!(f, "discovery:StopDataCollectionByAgentIds")
            }
            DiscoveryActions::UpdateApplication => write!(f, "discovery:UpdateApplication"),
        }
    }
}
