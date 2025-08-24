// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum DsqlActions {
    AddPeerCluster,
    CreateCluster,
    DbConnect,
    DbConnectAdmin,
    DeleteCluster,
    GetBackupJob,
    GetCluster,
    GetRestoreJob,
    GetVpcEndpointServiceName,
    InjectError,
    ListClusters,
    ListTagsForResource,
    PutMultiRegionProperties,
    PutWitnessRegion,
    RemovePeerCluster,
    StartBackupJob,
    StartRestoreJob,
    StopBackupJob,
    StopRestoreJob,
    TagResource,
    UntagResource,
    UpdateCluster,
}
impl std::fmt::Display for DsqlActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DsqlActions::AddPeerCluster => write!(f, "dsql:AddPeerCluster"),
            DsqlActions::CreateCluster => write!(f, "dsql:CreateCluster"),
            DsqlActions::DbConnect => write!(f, "dsql:DbConnect"),
            DsqlActions::DbConnectAdmin => write!(f, "dsql:DbConnectAdmin"),
            DsqlActions::DeleteCluster => write!(f, "dsql:DeleteCluster"),
            DsqlActions::GetBackupJob => write!(f, "dsql:GetBackupJob"),
            DsqlActions::GetCluster => write!(f, "dsql:GetCluster"),
            DsqlActions::GetRestoreJob => write!(f, "dsql:GetRestoreJob"),
            DsqlActions::GetVpcEndpointServiceName => write!(f, "dsql:GetVpcEndpointServiceName"),
            DsqlActions::InjectError => write!(f, "dsql:InjectError"),
            DsqlActions::ListClusters => write!(f, "dsql:ListClusters"),
            DsqlActions::ListTagsForResource => write!(f, "dsql:ListTagsForResource"),
            DsqlActions::PutMultiRegionProperties => write!(f, "dsql:PutMultiRegionProperties"),
            DsqlActions::PutWitnessRegion => write!(f, "dsql:PutWitnessRegion"),
            DsqlActions::RemovePeerCluster => write!(f, "dsql:RemovePeerCluster"),
            DsqlActions::StartBackupJob => write!(f, "dsql:StartBackupJob"),
            DsqlActions::StartRestoreJob => write!(f, "dsql:StartRestoreJob"),
            DsqlActions::StopBackupJob => write!(f, "dsql:StopBackupJob"),
            DsqlActions::StopRestoreJob => write!(f, "dsql:StopRestoreJob"),
            DsqlActions::TagResource => write!(f, "dsql:TagResource"),
            DsqlActions::UntagResource => write!(f, "dsql:UntagResource"),
            DsqlActions::UpdateCluster => write!(f, "dsql:UpdateCluster"),
        }
    }
}
