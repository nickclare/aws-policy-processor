// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum PcsActions {
    AllowVendedLogDeliveryForResource,
    CreateCluster,
    CreateComputeNodeGroup,
    CreateQueue,
    DeleteCluster,
    DeleteComputeNodeGroup,
    DeleteQueue,
    GetCluster,
    GetComputeNodeGroup,
    GetQueue,
    ListClusters,
    ListComputeNodeGroups,
    ListQueues,
    ListTagsForResource,
    RegisterComputeNodeGroupInstance,
    TagResource,
    UntagResource,
    UpdateComputeNodeGroup,
    UpdateQueue,
}
impl std::fmt::Display for PcsActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PcsActions::AllowVendedLogDeliveryForResource => {
                write!(f, "pcs:AllowVendedLogDeliveryForResource")
            }
            PcsActions::CreateCluster => write!(f, "pcs:CreateCluster"),
            PcsActions::CreateComputeNodeGroup => write!(f, "pcs:CreateComputeNodeGroup"),
            PcsActions::CreateQueue => write!(f, "pcs:CreateQueue"),
            PcsActions::DeleteCluster => write!(f, "pcs:DeleteCluster"),
            PcsActions::DeleteComputeNodeGroup => write!(f, "pcs:DeleteComputeNodeGroup"),
            PcsActions::DeleteQueue => write!(f, "pcs:DeleteQueue"),
            PcsActions::GetCluster => write!(f, "pcs:GetCluster"),
            PcsActions::GetComputeNodeGroup => write!(f, "pcs:GetComputeNodeGroup"),
            PcsActions::GetQueue => write!(f, "pcs:GetQueue"),
            PcsActions::ListClusters => write!(f, "pcs:ListClusters"),
            PcsActions::ListComputeNodeGroups => write!(f, "pcs:ListComputeNodeGroups"),
            PcsActions::ListQueues => write!(f, "pcs:ListQueues"),
            PcsActions::ListTagsForResource => write!(f, "pcs:ListTagsForResource"),
            PcsActions::RegisterComputeNodeGroupInstance => {
                write!(f, "pcs:RegisterComputeNodeGroupInstance")
            }
            PcsActions::TagResource => write!(f, "pcs:TagResource"),
            PcsActions::UntagResource => write!(f, "pcs:UntagResource"),
            PcsActions::UpdateComputeNodeGroup => write!(f, "pcs:UpdateComputeNodeGroup"),
            PcsActions::UpdateQueue => write!(f, "pcs:UpdateQueue"),
        }
    }
}
