// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum MemorydbActions {
    BatchUpdateCluster,
    Connect,
    CopySnapshot,
    CreateAcl,
    CreateCluster,
    CreateMultiRegionCluster,
    CreateParameterGroup,
    CreateSnapshot,
    CreateSubnetGroup,
    CreateUser,
    DeleteAcl,
    DeleteCluster,
    DeleteMultiRegionCluster,
    DeleteParameterGroup,
    DeleteSnapshot,
    DeleteSubnetGroup,
    DeleteUser,
    DescribeAcls,
    DescribeClusters,
    DescribeEngineVersions,
    DescribeEvents,
    DescribeMultiRegionClusters,
    DescribeMultiRegionParameterGroups,
    DescribeMultiRegionParameters,
    DescribeParameterGroups,
    DescribeParameters,
    DescribeReservedNodes,
    DescribeReservedNodesOfferings,
    DescribeServiceUpdates,
    DescribeSnapshots,
    DescribeSubnetGroups,
    DescribeUsers,
    FailoverShard,
    ListAllowedMultiRegionClusterUpdates,
    ListAllowedNodeTypeUpdates,
    ListTags,
    PauseMultiRegionClusterReplication,
    PurchaseReservedNodesOffering,
    ResetParameterGroup,
    TagResource,
    UntagResource,
    UpdateAcl,
    UpdateCluster,
    UpdateMultiRegionCluster,
    UpdateParameterGroup,
    UpdateSubnetGroup,
    UpdateUser,
}
impl std::fmt::Display for MemorydbActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MemorydbActions::BatchUpdateCluster => write!(f, "memorydb:BatchUpdateCluster"),
            MemorydbActions::Connect => write!(f, "memorydb:Connect"),
            MemorydbActions::CopySnapshot => write!(f, "memorydb:CopySnapshot"),
            MemorydbActions::CreateAcl => write!(f, "memorydb:CreateAcl"),
            MemorydbActions::CreateCluster => write!(f, "memorydb:CreateCluster"),
            MemorydbActions::CreateMultiRegionCluster => {
                write!(f, "memorydb:CreateMultiRegionCluster")
            }
            MemorydbActions::CreateParameterGroup => write!(f, "memorydb:CreateParameterGroup"),
            MemorydbActions::CreateSnapshot => write!(f, "memorydb:CreateSnapshot"),
            MemorydbActions::CreateSubnetGroup => write!(f, "memorydb:CreateSubnetGroup"),
            MemorydbActions::CreateUser => write!(f, "memorydb:CreateUser"),
            MemorydbActions::DeleteAcl => write!(f, "memorydb:DeleteAcl"),
            MemorydbActions::DeleteCluster => write!(f, "memorydb:DeleteCluster"),
            MemorydbActions::DeleteMultiRegionCluster => {
                write!(f, "memorydb:DeleteMultiRegionCluster")
            }
            MemorydbActions::DeleteParameterGroup => write!(f, "memorydb:DeleteParameterGroup"),
            MemorydbActions::DeleteSnapshot => write!(f, "memorydb:DeleteSnapshot"),
            MemorydbActions::DeleteSubnetGroup => write!(f, "memorydb:DeleteSubnetGroup"),
            MemorydbActions::DeleteUser => write!(f, "memorydb:DeleteUser"),
            MemorydbActions::DescribeAcls => write!(f, "memorydb:DescribeAcls"),
            MemorydbActions::DescribeClusters => write!(f, "memorydb:DescribeClusters"),
            MemorydbActions::DescribeEngineVersions => write!(f, "memorydb:DescribeEngineVersions"),
            MemorydbActions::DescribeEvents => write!(f, "memorydb:DescribeEvents"),
            MemorydbActions::DescribeMultiRegionClusters => {
                write!(f, "memorydb:DescribeMultiRegionClusters")
            }
            MemorydbActions::DescribeMultiRegionParameterGroups => {
                write!(f, "memorydb:DescribeMultiRegionParameterGroups")
            }
            MemorydbActions::DescribeMultiRegionParameters => {
                write!(f, "memorydb:DescribeMultiRegionParameters")
            }
            MemorydbActions::DescribeParameterGroups => {
                write!(f, "memorydb:DescribeParameterGroups")
            }
            MemorydbActions::DescribeParameters => write!(f, "memorydb:DescribeParameters"),
            MemorydbActions::DescribeReservedNodes => write!(f, "memorydb:DescribeReservedNodes"),
            MemorydbActions::DescribeReservedNodesOfferings => {
                write!(f, "memorydb:DescribeReservedNodesOfferings")
            }
            MemorydbActions::DescribeServiceUpdates => write!(f, "memorydb:DescribeServiceUpdates"),
            MemorydbActions::DescribeSnapshots => write!(f, "memorydb:DescribeSnapshots"),
            MemorydbActions::DescribeSubnetGroups => write!(f, "memorydb:DescribeSubnetGroups"),
            MemorydbActions::DescribeUsers => write!(f, "memorydb:DescribeUsers"),
            MemorydbActions::FailoverShard => write!(f, "memorydb:FailoverShard"),
            MemorydbActions::ListAllowedMultiRegionClusterUpdates => {
                write!(f, "memorydb:ListAllowedMultiRegionClusterUpdates")
            }
            MemorydbActions::ListAllowedNodeTypeUpdates => {
                write!(f, "memorydb:ListAllowedNodeTypeUpdates")
            }
            MemorydbActions::ListTags => write!(f, "memorydb:ListTags"),
            MemorydbActions::PauseMultiRegionClusterReplication => {
                write!(f, "memorydb:PauseMultiRegionClusterReplication")
            }
            MemorydbActions::PurchaseReservedNodesOffering => {
                write!(f, "memorydb:PurchaseReservedNodesOffering")
            }
            MemorydbActions::ResetParameterGroup => write!(f, "memorydb:ResetParameterGroup"),
            MemorydbActions::TagResource => write!(f, "memorydb:TagResource"),
            MemorydbActions::UntagResource => write!(f, "memorydb:UntagResource"),
            MemorydbActions::UpdateAcl => write!(f, "memorydb:UpdateAcl"),
            MemorydbActions::UpdateCluster => write!(f, "memorydb:UpdateCluster"),
            MemorydbActions::UpdateMultiRegionCluster => {
                write!(f, "memorydb:UpdateMultiRegionCluster")
            }
            MemorydbActions::UpdateParameterGroup => write!(f, "memorydb:UpdateParameterGroup"),
            MemorydbActions::UpdateSubnetGroup => write!(f, "memorydb:UpdateSubnetGroup"),
            MemorydbActions::UpdateUser => write!(f, "memorydb:UpdateUser"),
        }
    }
}
