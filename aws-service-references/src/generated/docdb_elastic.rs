// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum DocdbElasticActions {
    ApplyPendingMaintenanceAction,
    CopyClusterSnapshot,
    CreateCluster,
    CreateClusterSnapshot,
    DeleteCluster,
    DeleteClusterSnapshot,
    GetCluster,
    GetClusterSnapshot,
    GetPendingMaintenanceAction,
    ListClusterSnapshots,
    ListClusters,
    ListPendingMaintenanceActions,
    ListTagsForResource,
    RestoreClusterFromSnapshot,
    StartCluster,
    StopCluster,
    TagResource,
    UntagResource,
    UpdateCluster,
}
impl std::fmt::Display for DocdbElasticActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DocdbElasticActions::ApplyPendingMaintenanceAction => {
                write!(f, "docdb-elastic:ApplyPendingMaintenanceAction")
            }
            DocdbElasticActions::CopyClusterSnapshot => {
                write!(f, "docdb-elastic:CopyClusterSnapshot")
            }
            DocdbElasticActions::CreateCluster => write!(f, "docdb-elastic:CreateCluster"),
            DocdbElasticActions::CreateClusterSnapshot => {
                write!(f, "docdb-elastic:CreateClusterSnapshot")
            }
            DocdbElasticActions::DeleteCluster => write!(f, "docdb-elastic:DeleteCluster"),
            DocdbElasticActions::DeleteClusterSnapshot => {
                write!(f, "docdb-elastic:DeleteClusterSnapshot")
            }
            DocdbElasticActions::GetCluster => write!(f, "docdb-elastic:GetCluster"),
            DocdbElasticActions::GetClusterSnapshot => {
                write!(f, "docdb-elastic:GetClusterSnapshot")
            }
            DocdbElasticActions::GetPendingMaintenanceAction => {
                write!(f, "docdb-elastic:GetPendingMaintenanceAction")
            }
            DocdbElasticActions::ListClusterSnapshots => {
                write!(f, "docdb-elastic:ListClusterSnapshots")
            }
            DocdbElasticActions::ListClusters => write!(f, "docdb-elastic:ListClusters"),
            DocdbElasticActions::ListPendingMaintenanceActions => {
                write!(f, "docdb-elastic:ListPendingMaintenanceActions")
            }
            DocdbElasticActions::ListTagsForResource => {
                write!(f, "docdb-elastic:ListTagsForResource")
            }
            DocdbElasticActions::RestoreClusterFromSnapshot => {
                write!(f, "docdb-elastic:RestoreClusterFromSnapshot")
            }
            DocdbElasticActions::StartCluster => write!(f, "docdb-elastic:StartCluster"),
            DocdbElasticActions::StopCluster => write!(f, "docdb-elastic:StopCluster"),
            DocdbElasticActions::TagResource => write!(f, "docdb-elastic:TagResource"),
            DocdbElasticActions::UntagResource => write!(f, "docdb-elastic:UntagResource"),
            DocdbElasticActions::UpdateCluster => write!(f, "docdb-elastic:UpdateCluster"),
        }
    }
}
