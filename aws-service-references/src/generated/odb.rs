// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum OdbActions {
    AcceptMarketplaceRegistration,
    CreateCloudAutonomousVmCluster,
    CreateCloudExadataInfrastructure,
    CreateCloudVmCluster,
    CreateDbNode,
    CreateOdbNetwork,
    CreateOdbPeeringConnection,
    CreateOutboundIntegration,
    DeleteCloudAutonomousVmCluster,
    DeleteCloudExadataInfrastructure,
    DeleteCloudVmCluster,
    DeleteDbNode,
    DeleteOdbNetwork,
    DeleteOdbPeeringConnection,
    DeleteResourcePolicy,
    GetCloudAutonomousVmCluster,
    GetCloudExadataInfrastructure,
    GetCloudExadataInfrastructureUnallocatedResources,
    GetCloudVmCluster,
    GetDbNode,
    GetDbServer,
    GetOciOnboardingStatus,
    GetOdbNetwork,
    GetOdbPeeringConnection,
    GetResourcePolicy,
    InitializeService,
    ListAutonomousVirtualMachines,
    ListCloudAutonomousVmClusters,
    ListCloudExadataInfrastructures,
    ListCloudVmClusters,
    ListDbNodes,
    ListDbServers,
    ListDbSystemShapes,
    ListGiVersions,
    ListOdbNetworks,
    ListOdbPeeringConnections,
    ListSystemVersions,
    ListTagsForResource,
    PutResourcePolicy,
    RebootDbNode,
    StartDbNode,
    StopDbNode,
    TagResource,
    UntagResource,
    UpdateCloudExadataInfrastructure,
    UpdateOdbNetwork,
}
impl std::fmt::Display for OdbActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OdbActions::AcceptMarketplaceRegistration => {
                write!(f, "odb:AcceptMarketplaceRegistration")
            }
            OdbActions::CreateCloudAutonomousVmCluster => {
                write!(f, "odb:CreateCloudAutonomousVmCluster")
            }
            OdbActions::CreateCloudExadataInfrastructure => {
                write!(f, "odb:CreateCloudExadataInfrastructure")
            }
            OdbActions::CreateCloudVmCluster => write!(f, "odb:CreateCloudVmCluster"),
            OdbActions::CreateDbNode => write!(f, "odb:CreateDbNode"),
            OdbActions::CreateOdbNetwork => write!(f, "odb:CreateOdbNetwork"),
            OdbActions::CreateOdbPeeringConnection => write!(f, "odb:CreateOdbPeeringConnection"),
            OdbActions::CreateOutboundIntegration => write!(f, "odb:CreateOutboundIntegration"),
            OdbActions::DeleteCloudAutonomousVmCluster => {
                write!(f, "odb:DeleteCloudAutonomousVmCluster")
            }
            OdbActions::DeleteCloudExadataInfrastructure => {
                write!(f, "odb:DeleteCloudExadataInfrastructure")
            }
            OdbActions::DeleteCloudVmCluster => write!(f, "odb:DeleteCloudVmCluster"),
            OdbActions::DeleteDbNode => write!(f, "odb:DeleteDbNode"),
            OdbActions::DeleteOdbNetwork => write!(f, "odb:DeleteOdbNetwork"),
            OdbActions::DeleteOdbPeeringConnection => write!(f, "odb:DeleteOdbPeeringConnection"),
            OdbActions::DeleteResourcePolicy => write!(f, "odb:DeleteResourcePolicy"),
            OdbActions::GetCloudAutonomousVmCluster => write!(f, "odb:GetCloudAutonomousVmCluster"),
            OdbActions::GetCloudExadataInfrastructure => {
                write!(f, "odb:GetCloudExadataInfrastructure")
            }
            OdbActions::GetCloudExadataInfrastructureUnallocatedResources => {
                write!(f, "odb:GetCloudExadataInfrastructureUnallocatedResources")
            }
            OdbActions::GetCloudVmCluster => write!(f, "odb:GetCloudVmCluster"),
            OdbActions::GetDbNode => write!(f, "odb:GetDbNode"),
            OdbActions::GetDbServer => write!(f, "odb:GetDbServer"),
            OdbActions::GetOciOnboardingStatus => write!(f, "odb:GetOciOnboardingStatus"),
            OdbActions::GetOdbNetwork => write!(f, "odb:GetOdbNetwork"),
            OdbActions::GetOdbPeeringConnection => write!(f, "odb:GetOdbPeeringConnection"),
            OdbActions::GetResourcePolicy => write!(f, "odb:GetResourcePolicy"),
            OdbActions::InitializeService => write!(f, "odb:InitializeService"),
            OdbActions::ListAutonomousVirtualMachines => {
                write!(f, "odb:ListAutonomousVirtualMachines")
            }
            OdbActions::ListCloudAutonomousVmClusters => {
                write!(f, "odb:ListCloudAutonomousVmClusters")
            }
            OdbActions::ListCloudExadataInfrastructures => {
                write!(f, "odb:ListCloudExadataInfrastructures")
            }
            OdbActions::ListCloudVmClusters => write!(f, "odb:ListCloudVmClusters"),
            OdbActions::ListDbNodes => write!(f, "odb:ListDbNodes"),
            OdbActions::ListDbServers => write!(f, "odb:ListDbServers"),
            OdbActions::ListDbSystemShapes => write!(f, "odb:ListDbSystemShapes"),
            OdbActions::ListGiVersions => write!(f, "odb:ListGiVersions"),
            OdbActions::ListOdbNetworks => write!(f, "odb:ListOdbNetworks"),
            OdbActions::ListOdbPeeringConnections => write!(f, "odb:ListOdbPeeringConnections"),
            OdbActions::ListSystemVersions => write!(f, "odb:ListSystemVersions"),
            OdbActions::ListTagsForResource => write!(f, "odb:ListTagsForResource"),
            OdbActions::PutResourcePolicy => write!(f, "odb:PutResourcePolicy"),
            OdbActions::RebootDbNode => write!(f, "odb:RebootDbNode"),
            OdbActions::StartDbNode => write!(f, "odb:StartDbNode"),
            OdbActions::StopDbNode => write!(f, "odb:StopDbNode"),
            OdbActions::TagResource => write!(f, "odb:TagResource"),
            OdbActions::UntagResource => write!(f, "odb:UntagResource"),
            OdbActions::UpdateCloudExadataInfrastructure => {
                write!(f, "odb:UpdateCloudExadataInfrastructure")
            }
            OdbActions::UpdateOdbNetwork => write!(f, "odb:UpdateOdbNetwork"),
        }
    }
}
