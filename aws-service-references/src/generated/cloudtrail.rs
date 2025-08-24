// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum CloudtrailActions {
    AddTags,
    CancelQuery,
    CreateChannel,
    CreateDashboard,
    CreateEventDataStore,
    CreateServiceLinkedChannel,
    CreateTrail,
    DeleteChannel,
    DeleteDashboard,
    DeleteEventDataStore,
    DeleteResourcePolicy,
    DeleteServiceLinkedChannel,
    DeleteTrail,
    DeregisterOrganizationDelegatedAdmin,
    DescribeQuery,
    DescribeTrails,
    DisableFederation,
    EnableFederation,
    GenerateQuery,
    GenerateQueryResultsSummary,
    GetChannel,
    GetDashboard,
    GetEventConfiguration,
    GetEventDataStore,
    GetEventDataStoreData,
    GetEventSelectors,
    GetImport,
    GetInsightSelectors,
    GetQueryResults,
    GetResourcePolicy,
    GetServiceLinkedChannel,
    GetTrail,
    GetTrailStatus,
    ListChannels,
    ListDashboards,
    ListEventDataStores,
    ListImportFailures,
    ListImports,
    ListPublicKeys,
    ListQueries,
    ListServiceLinkedChannels,
    ListTags,
    ListTrails,
    LookupEvents,
    PutEventConfiguration,
    PutEventSelectors,
    PutInsightSelectors,
    PutResourcePolicy,
    RegisterOrganizationDelegatedAdmin,
    RemoveTags,
    RestoreEventDataStore,
    SearchSampleQueries,
    StartDashboardRefresh,
    StartEventDataStoreIngestion,
    StartImport,
    StartLogging,
    StartQuery,
    StopEventDataStoreIngestion,
    StopImport,
    StopLogging,
    UpdateChannel,
    UpdateDashboard,
    UpdateEventDataStore,
    UpdateServiceLinkedChannel,
    UpdateTrail,
}
impl std::fmt::Display for CloudtrailActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CloudtrailActions::AddTags => write!(f, "cloudtrail:AddTags"),
            CloudtrailActions::CancelQuery => write!(f, "cloudtrail:CancelQuery"),
            CloudtrailActions::CreateChannel => write!(f, "cloudtrail:CreateChannel"),
            CloudtrailActions::CreateDashboard => write!(f, "cloudtrail:CreateDashboard"),
            CloudtrailActions::CreateEventDataStore => write!(f, "cloudtrail:CreateEventDataStore"),
            CloudtrailActions::CreateServiceLinkedChannel => {
                write!(f, "cloudtrail:CreateServiceLinkedChannel")
            }
            CloudtrailActions::CreateTrail => write!(f, "cloudtrail:CreateTrail"),
            CloudtrailActions::DeleteChannel => write!(f, "cloudtrail:DeleteChannel"),
            CloudtrailActions::DeleteDashboard => write!(f, "cloudtrail:DeleteDashboard"),
            CloudtrailActions::DeleteEventDataStore => write!(f, "cloudtrail:DeleteEventDataStore"),
            CloudtrailActions::DeleteResourcePolicy => write!(f, "cloudtrail:DeleteResourcePolicy"),
            CloudtrailActions::DeleteServiceLinkedChannel => {
                write!(f, "cloudtrail:DeleteServiceLinkedChannel")
            }
            CloudtrailActions::DeleteTrail => write!(f, "cloudtrail:DeleteTrail"),
            CloudtrailActions::DeregisterOrganizationDelegatedAdmin => {
                write!(f, "cloudtrail:DeregisterOrganizationDelegatedAdmin")
            }
            CloudtrailActions::DescribeQuery => write!(f, "cloudtrail:DescribeQuery"),
            CloudtrailActions::DescribeTrails => write!(f, "cloudtrail:DescribeTrails"),
            CloudtrailActions::DisableFederation => write!(f, "cloudtrail:DisableFederation"),
            CloudtrailActions::EnableFederation => write!(f, "cloudtrail:EnableFederation"),
            CloudtrailActions::GenerateQuery => write!(f, "cloudtrail:GenerateQuery"),
            CloudtrailActions::GenerateQueryResultsSummary => {
                write!(f, "cloudtrail:GenerateQueryResultsSummary")
            }
            CloudtrailActions::GetChannel => write!(f, "cloudtrail:GetChannel"),
            CloudtrailActions::GetDashboard => write!(f, "cloudtrail:GetDashboard"),
            CloudtrailActions::GetEventConfiguration => {
                write!(f, "cloudtrail:GetEventConfiguration")
            }
            CloudtrailActions::GetEventDataStore => write!(f, "cloudtrail:GetEventDataStore"),
            CloudtrailActions::GetEventDataStoreData => {
                write!(f, "cloudtrail:GetEventDataStoreData")
            }
            CloudtrailActions::GetEventSelectors => write!(f, "cloudtrail:GetEventSelectors"),
            CloudtrailActions::GetImport => write!(f, "cloudtrail:GetImport"),
            CloudtrailActions::GetInsightSelectors => write!(f, "cloudtrail:GetInsightSelectors"),
            CloudtrailActions::GetQueryResults => write!(f, "cloudtrail:GetQueryResults"),
            CloudtrailActions::GetResourcePolicy => write!(f, "cloudtrail:GetResourcePolicy"),
            CloudtrailActions::GetServiceLinkedChannel => {
                write!(f, "cloudtrail:GetServiceLinkedChannel")
            }
            CloudtrailActions::GetTrail => write!(f, "cloudtrail:GetTrail"),
            CloudtrailActions::GetTrailStatus => write!(f, "cloudtrail:GetTrailStatus"),
            CloudtrailActions::ListChannels => write!(f, "cloudtrail:ListChannels"),
            CloudtrailActions::ListDashboards => write!(f, "cloudtrail:ListDashboards"),
            CloudtrailActions::ListEventDataStores => write!(f, "cloudtrail:ListEventDataStores"),
            CloudtrailActions::ListImportFailures => write!(f, "cloudtrail:ListImportFailures"),
            CloudtrailActions::ListImports => write!(f, "cloudtrail:ListImports"),
            CloudtrailActions::ListPublicKeys => write!(f, "cloudtrail:ListPublicKeys"),
            CloudtrailActions::ListQueries => write!(f, "cloudtrail:ListQueries"),
            CloudtrailActions::ListServiceLinkedChannels => {
                write!(f, "cloudtrail:ListServiceLinkedChannels")
            }
            CloudtrailActions::ListTags => write!(f, "cloudtrail:ListTags"),
            CloudtrailActions::ListTrails => write!(f, "cloudtrail:ListTrails"),
            CloudtrailActions::LookupEvents => write!(f, "cloudtrail:LookupEvents"),
            CloudtrailActions::PutEventConfiguration => {
                write!(f, "cloudtrail:PutEventConfiguration")
            }
            CloudtrailActions::PutEventSelectors => write!(f, "cloudtrail:PutEventSelectors"),
            CloudtrailActions::PutInsightSelectors => write!(f, "cloudtrail:PutInsightSelectors"),
            CloudtrailActions::PutResourcePolicy => write!(f, "cloudtrail:PutResourcePolicy"),
            CloudtrailActions::RegisterOrganizationDelegatedAdmin => {
                write!(f, "cloudtrail:RegisterOrganizationDelegatedAdmin")
            }
            CloudtrailActions::RemoveTags => write!(f, "cloudtrail:RemoveTags"),
            CloudtrailActions::RestoreEventDataStore => {
                write!(f, "cloudtrail:RestoreEventDataStore")
            }
            CloudtrailActions::SearchSampleQueries => write!(f, "cloudtrail:SearchSampleQueries"),
            CloudtrailActions::StartDashboardRefresh => {
                write!(f, "cloudtrail:StartDashboardRefresh")
            }
            CloudtrailActions::StartEventDataStoreIngestion => {
                write!(f, "cloudtrail:StartEventDataStoreIngestion")
            }
            CloudtrailActions::StartImport => write!(f, "cloudtrail:StartImport"),
            CloudtrailActions::StartLogging => write!(f, "cloudtrail:StartLogging"),
            CloudtrailActions::StartQuery => write!(f, "cloudtrail:StartQuery"),
            CloudtrailActions::StopEventDataStoreIngestion => {
                write!(f, "cloudtrail:StopEventDataStoreIngestion")
            }
            CloudtrailActions::StopImport => write!(f, "cloudtrail:StopImport"),
            CloudtrailActions::StopLogging => write!(f, "cloudtrail:StopLogging"),
            CloudtrailActions::UpdateChannel => write!(f, "cloudtrail:UpdateChannel"),
            CloudtrailActions::UpdateDashboard => write!(f, "cloudtrail:UpdateDashboard"),
            CloudtrailActions::UpdateEventDataStore => write!(f, "cloudtrail:UpdateEventDataStore"),
            CloudtrailActions::UpdateServiceLinkedChannel => {
                write!(f, "cloudtrail:UpdateServiceLinkedChannel")
            }
            CloudtrailActions::UpdateTrail => write!(f, "cloudtrail:UpdateTrail"),
        }
    }
}
