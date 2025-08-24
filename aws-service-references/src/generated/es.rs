// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum EsActions {
    AcceptInboundConnection,
    AcceptInboundCrossClusterSearchConnection,
    AddDataSource,
    AddDirectQueryDataSource,
    AddTags,
    AssociatePackage,
    AssociatePackages,
    AuthorizeVpcEndpointAccess,
    CancelDomainConfigChange,
    CancelElasticsearchServiceSoftwareUpdate,
    CancelServiceSoftwareUpdate,
    CreateApplication,
    CreateDomain,
    CreateElasticsearchDomain,
    CreateElasticsearchServiceRole,
    CreateOutboundConnection,
    CreateOutboundCrossClusterSearchConnection,
    CreatePackage,
    CreateServiceRole,
    CreateVpcEndpoint,
    DeleteApplication,
    DeleteDataSource,
    DeleteDirectQueryDataSource,
    DeleteDomain,
    DeleteElasticsearchDomain,
    DeleteElasticsearchServiceRole,
    DeleteInboundConnection,
    DeleteInboundCrossClusterSearchConnection,
    DeleteOutboundConnection,
    DeleteOutboundCrossClusterSearchConnection,
    DeletePackage,
    DeleteVpcEndpoint,
    DescribeDomain,
    DescribeDomainAutoTunes,
    DescribeDomainChangeProgress,
    DescribeDomainConfig,
    DescribeDomainHealth,
    DescribeDomainNodes,
    DescribeDomains,
    DescribeDryRunProgress,
    DescribeElasticsearchDomain,
    DescribeElasticsearchDomainConfig,
    DescribeElasticsearchDomains,
    DescribeElasticsearchInstanceTypeLimits,
    DescribeInboundConnections,
    DescribeInboundCrossClusterSearchConnections,
    DescribeInstanceTypeLimits,
    DescribeOutboundConnections,
    DescribeOutboundCrossClusterSearchConnections,
    DescribePackages,
    DescribeReservedElasticsearchInstanceOfferings,
    DescribeReservedElasticsearchInstances,
    DescribeReservedInstanceOfferings,
    DescribeReservedInstances,
    DescribeVpcEndpoints,
    DissociatePackage,
    DissociatePackages,
    EsCrossClusterGet,
    EsHttpDelete,
    EsHttpGet,
    EsHttpHead,
    EsHttpPatch,
    EsHttpPost,
    EsHttpPut,
    GetApplication,
    GetCompatibleElasticsearchVersions,
    GetCompatibleVersions,
    GetDataSource,
    GetDirectQueryDataSource,
    GetDomainMaintenanceStatus,
    GetPackageVersionHistory,
    GetUpgradeHistory,
    GetUpgradeStatus,
    ListApplications,
    ListDataSources,
    ListDirectQueryDataSources,
    ListDomainMaintenances,
    ListDomainNames,
    ListDomainsForPackage,
    ListElasticsearchInstanceTypeDetails,
    ListElasticsearchInstanceTypes,
    ListElasticsearchVersions,
    ListInstanceTypeDetails,
    ListPackagesForDomain,
    ListScheduledActions,
    ListTags,
    ListVersions,
    ListVpcEndpointAccess,
    ListVpcEndpoints,
    ListVpcEndpointsForDomain,
    PurchaseReservedElasticsearchInstanceOffering,
    PurchaseReservedInstanceOffering,
    RejectInboundConnection,
    RejectInboundCrossClusterSearchConnection,
    RemoveTags,
    RevokeVpcEndpointAccess,
    StartDomainMaintenance,
    StartElasticsearchServiceSoftwareUpdate,
    StartServiceSoftwareUpdate,
    UpdateApplication,
    UpdateDataSource,
    UpdateDirectQueryDataSource,
    UpdateDomainConfig,
    UpdateElasticsearchDomainConfig,
    UpdatePackage,
    UpdatePackageScope,
    UpdateScheduledAction,
    UpdateVpcEndpoint,
    UpgradeDomain,
    UpgradeElasticsearchDomain,
}
impl std::fmt::Display for EsActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EsActions::AcceptInboundConnection => write!(f, "es:AcceptInboundConnection"),
            EsActions::AcceptInboundCrossClusterSearchConnection => {
                write!(f, "es:AcceptInboundCrossClusterSearchConnection")
            }
            EsActions::AddDataSource => write!(f, "es:AddDataSource"),
            EsActions::AddDirectQueryDataSource => write!(f, "es:AddDirectQueryDataSource"),
            EsActions::AddTags => write!(f, "es:AddTags"),
            EsActions::AssociatePackage => write!(f, "es:AssociatePackage"),
            EsActions::AssociatePackages => write!(f, "es:AssociatePackages"),
            EsActions::AuthorizeVpcEndpointAccess => write!(f, "es:AuthorizeVpcEndpointAccess"),
            EsActions::CancelDomainConfigChange => write!(f, "es:CancelDomainConfigChange"),
            EsActions::CancelElasticsearchServiceSoftwareUpdate => {
                write!(f, "es:CancelElasticsearchServiceSoftwareUpdate")
            }
            EsActions::CancelServiceSoftwareUpdate => write!(f, "es:CancelServiceSoftwareUpdate"),
            EsActions::CreateApplication => write!(f, "es:CreateApplication"),
            EsActions::CreateDomain => write!(f, "es:CreateDomain"),
            EsActions::CreateElasticsearchDomain => write!(f, "es:CreateElasticsearchDomain"),
            EsActions::CreateElasticsearchServiceRole => {
                write!(f, "es:CreateElasticsearchServiceRole")
            }
            EsActions::CreateOutboundConnection => write!(f, "es:CreateOutboundConnection"),
            EsActions::CreateOutboundCrossClusterSearchConnection => {
                write!(f, "es:CreateOutboundCrossClusterSearchConnection")
            }
            EsActions::CreatePackage => write!(f, "es:CreatePackage"),
            EsActions::CreateServiceRole => write!(f, "es:CreateServiceRole"),
            EsActions::CreateVpcEndpoint => write!(f, "es:CreateVpcEndpoint"),
            EsActions::DeleteApplication => write!(f, "es:DeleteApplication"),
            EsActions::DeleteDataSource => write!(f, "es:DeleteDataSource"),
            EsActions::DeleteDirectQueryDataSource => write!(f, "es:DeleteDirectQueryDataSource"),
            EsActions::DeleteDomain => write!(f, "es:DeleteDomain"),
            EsActions::DeleteElasticsearchDomain => write!(f, "es:DeleteElasticsearchDomain"),
            EsActions::DeleteElasticsearchServiceRole => {
                write!(f, "es:DeleteElasticsearchServiceRole")
            }
            EsActions::DeleteInboundConnection => write!(f, "es:DeleteInboundConnection"),
            EsActions::DeleteInboundCrossClusterSearchConnection => {
                write!(f, "es:DeleteInboundCrossClusterSearchConnection")
            }
            EsActions::DeleteOutboundConnection => write!(f, "es:DeleteOutboundConnection"),
            EsActions::DeleteOutboundCrossClusterSearchConnection => {
                write!(f, "es:DeleteOutboundCrossClusterSearchConnection")
            }
            EsActions::DeletePackage => write!(f, "es:DeletePackage"),
            EsActions::DeleteVpcEndpoint => write!(f, "es:DeleteVpcEndpoint"),
            EsActions::DescribeDomain => write!(f, "es:DescribeDomain"),
            EsActions::DescribeDomainAutoTunes => write!(f, "es:DescribeDomainAutoTunes"),
            EsActions::DescribeDomainChangeProgress => write!(f, "es:DescribeDomainChangeProgress"),
            EsActions::DescribeDomainConfig => write!(f, "es:DescribeDomainConfig"),
            EsActions::DescribeDomainHealth => write!(f, "es:DescribeDomainHealth"),
            EsActions::DescribeDomainNodes => write!(f, "es:DescribeDomainNodes"),
            EsActions::DescribeDomains => write!(f, "es:DescribeDomains"),
            EsActions::DescribeDryRunProgress => write!(f, "es:DescribeDryRunProgress"),
            EsActions::DescribeElasticsearchDomain => write!(f, "es:DescribeElasticsearchDomain"),
            EsActions::DescribeElasticsearchDomainConfig => {
                write!(f, "es:DescribeElasticsearchDomainConfig")
            }
            EsActions::DescribeElasticsearchDomains => write!(f, "es:DescribeElasticsearchDomains"),
            EsActions::DescribeElasticsearchInstanceTypeLimits => {
                write!(f, "es:DescribeElasticsearchInstanceTypeLimits")
            }
            EsActions::DescribeInboundConnections => write!(f, "es:DescribeInboundConnections"),
            EsActions::DescribeInboundCrossClusterSearchConnections => {
                write!(f, "es:DescribeInboundCrossClusterSearchConnections")
            }
            EsActions::DescribeInstanceTypeLimits => write!(f, "es:DescribeInstanceTypeLimits"),
            EsActions::DescribeOutboundConnections => write!(f, "es:DescribeOutboundConnections"),
            EsActions::DescribeOutboundCrossClusterSearchConnections => {
                write!(f, "es:DescribeOutboundCrossClusterSearchConnections")
            }
            EsActions::DescribePackages => write!(f, "es:DescribePackages"),
            EsActions::DescribeReservedElasticsearchInstanceOfferings => {
                write!(f, "es:DescribeReservedElasticsearchInstanceOfferings")
            }
            EsActions::DescribeReservedElasticsearchInstances => {
                write!(f, "es:DescribeReservedElasticsearchInstances")
            }
            EsActions::DescribeReservedInstanceOfferings => {
                write!(f, "es:DescribeReservedInstanceOfferings")
            }
            EsActions::DescribeReservedInstances => write!(f, "es:DescribeReservedInstances"),
            EsActions::DescribeVpcEndpoints => write!(f, "es:DescribeVpcEndpoints"),
            EsActions::DissociatePackage => write!(f, "es:DissociatePackage"),
            EsActions::DissociatePackages => write!(f, "es:DissociatePackages"),
            EsActions::EsCrossClusterGet => write!(f, "es:ESCrossClusterGet"),
            EsActions::EsHttpDelete => write!(f, "es:ESHttpDelete"),
            EsActions::EsHttpGet => write!(f, "es:ESHttpGet"),
            EsActions::EsHttpHead => write!(f, "es:ESHttpHead"),
            EsActions::EsHttpPatch => write!(f, "es:ESHttpPatch"),
            EsActions::EsHttpPost => write!(f, "es:ESHttpPost"),
            EsActions::EsHttpPut => write!(f, "es:ESHttpPut"),
            EsActions::GetApplication => write!(f, "es:GetApplication"),
            EsActions::GetCompatibleElasticsearchVersions => {
                write!(f, "es:GetCompatibleElasticsearchVersions")
            }
            EsActions::GetCompatibleVersions => write!(f, "es:GetCompatibleVersions"),
            EsActions::GetDataSource => write!(f, "es:GetDataSource"),
            EsActions::GetDirectQueryDataSource => write!(f, "es:GetDirectQueryDataSource"),
            EsActions::GetDomainMaintenanceStatus => write!(f, "es:GetDomainMaintenanceStatus"),
            EsActions::GetPackageVersionHistory => write!(f, "es:GetPackageVersionHistory"),
            EsActions::GetUpgradeHistory => write!(f, "es:GetUpgradeHistory"),
            EsActions::GetUpgradeStatus => write!(f, "es:GetUpgradeStatus"),
            EsActions::ListApplications => write!(f, "es:ListApplications"),
            EsActions::ListDataSources => write!(f, "es:ListDataSources"),
            EsActions::ListDirectQueryDataSources => write!(f, "es:ListDirectQueryDataSources"),
            EsActions::ListDomainMaintenances => write!(f, "es:ListDomainMaintenances"),
            EsActions::ListDomainNames => write!(f, "es:ListDomainNames"),
            EsActions::ListDomainsForPackage => write!(f, "es:ListDomainsForPackage"),
            EsActions::ListElasticsearchInstanceTypeDetails => {
                write!(f, "es:ListElasticsearchInstanceTypeDetails")
            }
            EsActions::ListElasticsearchInstanceTypes => {
                write!(f, "es:ListElasticsearchInstanceTypes")
            }
            EsActions::ListElasticsearchVersions => write!(f, "es:ListElasticsearchVersions"),
            EsActions::ListInstanceTypeDetails => write!(f, "es:ListInstanceTypeDetails"),
            EsActions::ListPackagesForDomain => write!(f, "es:ListPackagesForDomain"),
            EsActions::ListScheduledActions => write!(f, "es:ListScheduledActions"),
            EsActions::ListTags => write!(f, "es:ListTags"),
            EsActions::ListVersions => write!(f, "es:ListVersions"),
            EsActions::ListVpcEndpointAccess => write!(f, "es:ListVpcEndpointAccess"),
            EsActions::ListVpcEndpoints => write!(f, "es:ListVpcEndpoints"),
            EsActions::ListVpcEndpointsForDomain => write!(f, "es:ListVpcEndpointsForDomain"),
            EsActions::PurchaseReservedElasticsearchInstanceOffering => {
                write!(f, "es:PurchaseReservedElasticsearchInstanceOffering")
            }
            EsActions::PurchaseReservedInstanceOffering => {
                write!(f, "es:PurchaseReservedInstanceOffering")
            }
            EsActions::RejectInboundConnection => write!(f, "es:RejectInboundConnection"),
            EsActions::RejectInboundCrossClusterSearchConnection => {
                write!(f, "es:RejectInboundCrossClusterSearchConnection")
            }
            EsActions::RemoveTags => write!(f, "es:RemoveTags"),
            EsActions::RevokeVpcEndpointAccess => write!(f, "es:RevokeVpcEndpointAccess"),
            EsActions::StartDomainMaintenance => write!(f, "es:StartDomainMaintenance"),
            EsActions::StartElasticsearchServiceSoftwareUpdate => {
                write!(f, "es:StartElasticsearchServiceSoftwareUpdate")
            }
            EsActions::StartServiceSoftwareUpdate => write!(f, "es:StartServiceSoftwareUpdate"),
            EsActions::UpdateApplication => write!(f, "es:UpdateApplication"),
            EsActions::UpdateDataSource => write!(f, "es:UpdateDataSource"),
            EsActions::UpdateDirectQueryDataSource => write!(f, "es:UpdateDirectQueryDataSource"),
            EsActions::UpdateDomainConfig => write!(f, "es:UpdateDomainConfig"),
            EsActions::UpdateElasticsearchDomainConfig => {
                write!(f, "es:UpdateElasticsearchDomainConfig")
            }
            EsActions::UpdatePackage => write!(f, "es:UpdatePackage"),
            EsActions::UpdatePackageScope => write!(f, "es:UpdatePackageScope"),
            EsActions::UpdateScheduledAction => write!(f, "es:UpdateScheduledAction"),
            EsActions::UpdateVpcEndpoint => write!(f, "es:UpdateVpcEndpoint"),
            EsActions::UpgradeDomain => write!(f, "es:UpgradeDomain"),
            EsActions::UpgradeElasticsearchDomain => write!(f, "es:UpgradeElasticsearchDomain"),
        }
    }
}
