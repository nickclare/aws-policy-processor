// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum ElasticacheActions {
    AddTagsToResource,
    AuthorizeCacheSecurityGroupIngress,
    BatchApplyUpdateAction,
    BatchStopUpdateAction,
    CompleteMigration,
    Connect,
    CopyServerlessCacheSnapshot,
    CopySnapshot,
    CreateCacheCluster,
    CreateCacheParameterGroup,
    CreateCacheSecurityGroup,
    CreateCacheSubnetGroup,
    CreateGlobalReplicationGroup,
    CreateReplicationGroup,
    CreateServerlessCache,
    CreateServerlessCacheSnapshot,
    CreateSnapshot,
    CreateUser,
    CreateUserGroup,
    DecreaseNodeGroupsInGlobalReplicationGroup,
    DecreaseReplicaCount,
    DeleteCacheCluster,
    DeleteCacheParameterGroup,
    DeleteCacheSecurityGroup,
    DeleteCacheSubnetGroup,
    DeleteGlobalReplicationGroup,
    DeleteReplicationGroup,
    DeleteServerlessCache,
    DeleteServerlessCacheSnapshot,
    DeleteSnapshot,
    DeleteUser,
    DeleteUserGroup,
    DescribeCacheClusters,
    DescribeCacheEngineVersions,
    DescribeCacheParameterGroups,
    DescribeCacheParameters,
    DescribeCacheSecurityGroups,
    DescribeCacheSubnetGroups,
    DescribeEngineDefaultParameters,
    DescribeEvents,
    DescribeGlobalReplicationGroups,
    DescribeReplicationGroups,
    DescribeReservedCacheNodes,
    DescribeReservedCacheNodesOfferings,
    DescribeServerlessCacheSnapshots,
    DescribeServerlessCaches,
    DescribeServiceUpdates,
    DescribeSnapshots,
    DescribeUpdateActions,
    DescribeUserGroups,
    DescribeUsers,
    DisassociateGlobalReplicationGroup,
    ExportServerlessCacheSnapshot,
    FailoverGlobalReplicationGroup,
    IncreaseNodeGroupsInGlobalReplicationGroup,
    IncreaseReplicaCount,
    InterruptClusterAzPower,
    ListAllowedNodeTypeModifications,
    ListTagsForResource,
    ModifyCacheCluster,
    ModifyCacheParameterGroup,
    ModifyCacheSubnetGroup,
    ModifyGlobalReplicationGroup,
    ModifyReplicationGroup,
    ModifyReplicationGroupShardConfiguration,
    ModifyServerlessCache,
    ModifyUser,
    ModifyUserGroup,
    PurchaseReservedCacheNodesOffering,
    RebalanceSlotsInGlobalReplicationGroup,
    RebootCacheCluster,
    RemoveTagsFromResource,
    ResetCacheParameterGroup,
    RevokeCacheSecurityGroupIngress,
    StartMigration,
    TestFailover,
    TestMigration,
}
impl std::fmt::Display for ElasticacheActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ElasticacheActions::AddTagsToResource => write!(f, "elasticache:AddTagsToResource"),
            ElasticacheActions::AuthorizeCacheSecurityGroupIngress => {
                write!(f, "elasticache:AuthorizeCacheSecurityGroupIngress")
            }
            ElasticacheActions::BatchApplyUpdateAction => {
                write!(f, "elasticache:BatchApplyUpdateAction")
            }
            ElasticacheActions::BatchStopUpdateAction => {
                write!(f, "elasticache:BatchStopUpdateAction")
            }
            ElasticacheActions::CompleteMigration => write!(f, "elasticache:CompleteMigration"),
            ElasticacheActions::Connect => write!(f, "elasticache:Connect"),
            ElasticacheActions::CopyServerlessCacheSnapshot => {
                write!(f, "elasticache:CopyServerlessCacheSnapshot")
            }
            ElasticacheActions::CopySnapshot => write!(f, "elasticache:CopySnapshot"),
            ElasticacheActions::CreateCacheCluster => write!(f, "elasticache:CreateCacheCluster"),
            ElasticacheActions::CreateCacheParameterGroup => {
                write!(f, "elasticache:CreateCacheParameterGroup")
            }
            ElasticacheActions::CreateCacheSecurityGroup => {
                write!(f, "elasticache:CreateCacheSecurityGroup")
            }
            ElasticacheActions::CreateCacheSubnetGroup => {
                write!(f, "elasticache:CreateCacheSubnetGroup")
            }
            ElasticacheActions::CreateGlobalReplicationGroup => {
                write!(f, "elasticache:CreateGlobalReplicationGroup")
            }
            ElasticacheActions::CreateReplicationGroup => {
                write!(f, "elasticache:CreateReplicationGroup")
            }
            ElasticacheActions::CreateServerlessCache => {
                write!(f, "elasticache:CreateServerlessCache")
            }
            ElasticacheActions::CreateServerlessCacheSnapshot => {
                write!(f, "elasticache:CreateServerlessCacheSnapshot")
            }
            ElasticacheActions::CreateSnapshot => write!(f, "elasticache:CreateSnapshot"),
            ElasticacheActions::CreateUser => write!(f, "elasticache:CreateUser"),
            ElasticacheActions::CreateUserGroup => write!(f, "elasticache:CreateUserGroup"),
            ElasticacheActions::DecreaseNodeGroupsInGlobalReplicationGroup => {
                write!(f, "elasticache:DecreaseNodeGroupsInGlobalReplicationGroup")
            }
            ElasticacheActions::DecreaseReplicaCount => {
                write!(f, "elasticache:DecreaseReplicaCount")
            }
            ElasticacheActions::DeleteCacheCluster => write!(f, "elasticache:DeleteCacheCluster"),
            ElasticacheActions::DeleteCacheParameterGroup => {
                write!(f, "elasticache:DeleteCacheParameterGroup")
            }
            ElasticacheActions::DeleteCacheSecurityGroup => {
                write!(f, "elasticache:DeleteCacheSecurityGroup")
            }
            ElasticacheActions::DeleteCacheSubnetGroup => {
                write!(f, "elasticache:DeleteCacheSubnetGroup")
            }
            ElasticacheActions::DeleteGlobalReplicationGroup => {
                write!(f, "elasticache:DeleteGlobalReplicationGroup")
            }
            ElasticacheActions::DeleteReplicationGroup => {
                write!(f, "elasticache:DeleteReplicationGroup")
            }
            ElasticacheActions::DeleteServerlessCache => {
                write!(f, "elasticache:DeleteServerlessCache")
            }
            ElasticacheActions::DeleteServerlessCacheSnapshot => {
                write!(f, "elasticache:DeleteServerlessCacheSnapshot")
            }
            ElasticacheActions::DeleteSnapshot => write!(f, "elasticache:DeleteSnapshot"),
            ElasticacheActions::DeleteUser => write!(f, "elasticache:DeleteUser"),
            ElasticacheActions::DeleteUserGroup => write!(f, "elasticache:DeleteUserGroup"),
            ElasticacheActions::DescribeCacheClusters => {
                write!(f, "elasticache:DescribeCacheClusters")
            }
            ElasticacheActions::DescribeCacheEngineVersions => {
                write!(f, "elasticache:DescribeCacheEngineVersions")
            }
            ElasticacheActions::DescribeCacheParameterGroups => {
                write!(f, "elasticache:DescribeCacheParameterGroups")
            }
            ElasticacheActions::DescribeCacheParameters => {
                write!(f, "elasticache:DescribeCacheParameters")
            }
            ElasticacheActions::DescribeCacheSecurityGroups => {
                write!(f, "elasticache:DescribeCacheSecurityGroups")
            }
            ElasticacheActions::DescribeCacheSubnetGroups => {
                write!(f, "elasticache:DescribeCacheSubnetGroups")
            }
            ElasticacheActions::DescribeEngineDefaultParameters => {
                write!(f, "elasticache:DescribeEngineDefaultParameters")
            }
            ElasticacheActions::DescribeEvents => write!(f, "elasticache:DescribeEvents"),
            ElasticacheActions::DescribeGlobalReplicationGroups => {
                write!(f, "elasticache:DescribeGlobalReplicationGroups")
            }
            ElasticacheActions::DescribeReplicationGroups => {
                write!(f, "elasticache:DescribeReplicationGroups")
            }
            ElasticacheActions::DescribeReservedCacheNodes => {
                write!(f, "elasticache:DescribeReservedCacheNodes")
            }
            ElasticacheActions::DescribeReservedCacheNodesOfferings => {
                write!(f, "elasticache:DescribeReservedCacheNodesOfferings")
            }
            ElasticacheActions::DescribeServerlessCacheSnapshots => {
                write!(f, "elasticache:DescribeServerlessCacheSnapshots")
            }
            ElasticacheActions::DescribeServerlessCaches => {
                write!(f, "elasticache:DescribeServerlessCaches")
            }
            ElasticacheActions::DescribeServiceUpdates => {
                write!(f, "elasticache:DescribeServiceUpdates")
            }
            ElasticacheActions::DescribeSnapshots => write!(f, "elasticache:DescribeSnapshots"),
            ElasticacheActions::DescribeUpdateActions => {
                write!(f, "elasticache:DescribeUpdateActions")
            }
            ElasticacheActions::DescribeUserGroups => write!(f, "elasticache:DescribeUserGroups"),
            ElasticacheActions::DescribeUsers => write!(f, "elasticache:DescribeUsers"),
            ElasticacheActions::DisassociateGlobalReplicationGroup => {
                write!(f, "elasticache:DisassociateGlobalReplicationGroup")
            }
            ElasticacheActions::ExportServerlessCacheSnapshot => {
                write!(f, "elasticache:ExportServerlessCacheSnapshot")
            }
            ElasticacheActions::FailoverGlobalReplicationGroup => {
                write!(f, "elasticache:FailoverGlobalReplicationGroup")
            }
            ElasticacheActions::IncreaseNodeGroupsInGlobalReplicationGroup => {
                write!(f, "elasticache:IncreaseNodeGroupsInGlobalReplicationGroup")
            }
            ElasticacheActions::IncreaseReplicaCount => {
                write!(f, "elasticache:IncreaseReplicaCount")
            }
            ElasticacheActions::InterruptClusterAzPower => {
                write!(f, "elasticache:InterruptClusterAzPower")
            }
            ElasticacheActions::ListAllowedNodeTypeModifications => {
                write!(f, "elasticache:ListAllowedNodeTypeModifications")
            }
            ElasticacheActions::ListTagsForResource => write!(f, "elasticache:ListTagsForResource"),
            ElasticacheActions::ModifyCacheCluster => write!(f, "elasticache:ModifyCacheCluster"),
            ElasticacheActions::ModifyCacheParameterGroup => {
                write!(f, "elasticache:ModifyCacheParameterGroup")
            }
            ElasticacheActions::ModifyCacheSubnetGroup => {
                write!(f, "elasticache:ModifyCacheSubnetGroup")
            }
            ElasticacheActions::ModifyGlobalReplicationGroup => {
                write!(f, "elasticache:ModifyGlobalReplicationGroup")
            }
            ElasticacheActions::ModifyReplicationGroup => {
                write!(f, "elasticache:ModifyReplicationGroup")
            }
            ElasticacheActions::ModifyReplicationGroupShardConfiguration => {
                write!(f, "elasticache:ModifyReplicationGroupShardConfiguration")
            }
            ElasticacheActions::ModifyServerlessCache => {
                write!(f, "elasticache:ModifyServerlessCache")
            }
            ElasticacheActions::ModifyUser => write!(f, "elasticache:ModifyUser"),
            ElasticacheActions::ModifyUserGroup => write!(f, "elasticache:ModifyUserGroup"),
            ElasticacheActions::PurchaseReservedCacheNodesOffering => {
                write!(f, "elasticache:PurchaseReservedCacheNodesOffering")
            }
            ElasticacheActions::RebalanceSlotsInGlobalReplicationGroup => {
                write!(f, "elasticache:RebalanceSlotsInGlobalReplicationGroup")
            }
            ElasticacheActions::RebootCacheCluster => write!(f, "elasticache:RebootCacheCluster"),
            ElasticacheActions::RemoveTagsFromResource => {
                write!(f, "elasticache:RemoveTagsFromResource")
            }
            ElasticacheActions::ResetCacheParameterGroup => {
                write!(f, "elasticache:ResetCacheParameterGroup")
            }
            ElasticacheActions::RevokeCacheSecurityGroupIngress => {
                write!(f, "elasticache:RevokeCacheSecurityGroupIngress")
            }
            ElasticacheActions::StartMigration => write!(f, "elasticache:StartMigration"),
            ElasticacheActions::TestFailover => write!(f, "elasticache:TestFailover"),
            ElasticacheActions::TestMigration => write!(f, "elasticache:TestMigration"),
        }
    }
}
