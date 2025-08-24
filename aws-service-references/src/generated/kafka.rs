// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum KafkaActions {
    BatchAssociateScramSecret,
    BatchDisassociateScramSecret,
    CreateCluster,
    CreateClusterV2,
    CreateConfiguration,
    CreateReplicator,
    CreateVpcConnection,
    DeleteCluster,
    DeleteClusterPolicy,
    DeleteConfiguration,
    DeleteReplicator,
    DeleteVpcConnection,
    DescribeCluster,
    DescribeClusterOperation,
    DescribeClusterOperationV2,
    DescribeClusterV2,
    DescribeConfiguration,
    DescribeConfigurationRevision,
    DescribeReplicator,
    DescribeVpcConnection,
    GetBootstrapBrokers,
    GetClusterPolicy,
    GetCompatibleKafkaVersions,
    ListClientVpcConnections,
    ListClusterOperations,
    ListClusterOperationsV2,
    ListClusters,
    ListClustersV2,
    ListConfigurationRevisions,
    ListConfigurations,
    ListKafkaVersions,
    ListNodes,
    ListReplicators,
    ListScramSecrets,
    ListTagsForResource,
    ListVpcConnections,
    PutClusterPolicy,
    RebootBroker,
    RejectClientVpcConnection,
    TagResource,
    UntagResource,
    UpdateBrokerCount,
    UpdateBrokerStorage,
    UpdateBrokerType,
    UpdateClusterConfiguration,
    UpdateClusterKafkaVersion,
    UpdateConfiguration,
    UpdateConnectivity,
    UpdateMonitoring,
    UpdateReplicationInfo,
    UpdateSecurity,
    UpdateStorage,
}
impl std::fmt::Display for KafkaActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KafkaActions::BatchAssociateScramSecret => write!(f, "kafka:BatchAssociateScramSecret"),
            KafkaActions::BatchDisassociateScramSecret => {
                write!(f, "kafka:BatchDisassociateScramSecret")
            }
            KafkaActions::CreateCluster => write!(f, "kafka:CreateCluster"),
            KafkaActions::CreateClusterV2 => write!(f, "kafka:CreateClusterV2"),
            KafkaActions::CreateConfiguration => write!(f, "kafka:CreateConfiguration"),
            KafkaActions::CreateReplicator => write!(f, "kafka:CreateReplicator"),
            KafkaActions::CreateVpcConnection => write!(f, "kafka:CreateVpcConnection"),
            KafkaActions::DeleteCluster => write!(f, "kafka:DeleteCluster"),
            KafkaActions::DeleteClusterPolicy => write!(f, "kafka:DeleteClusterPolicy"),
            KafkaActions::DeleteConfiguration => write!(f, "kafka:DeleteConfiguration"),
            KafkaActions::DeleteReplicator => write!(f, "kafka:DeleteReplicator"),
            KafkaActions::DeleteVpcConnection => write!(f, "kafka:DeleteVpcConnection"),
            KafkaActions::DescribeCluster => write!(f, "kafka:DescribeCluster"),
            KafkaActions::DescribeClusterOperation => write!(f, "kafka:DescribeClusterOperation"),
            KafkaActions::DescribeClusterOperationV2 => {
                write!(f, "kafka:DescribeClusterOperationV2")
            }
            KafkaActions::DescribeClusterV2 => write!(f, "kafka:DescribeClusterV2"),
            KafkaActions::DescribeConfiguration => write!(f, "kafka:DescribeConfiguration"),
            KafkaActions::DescribeConfigurationRevision => {
                write!(f, "kafka:DescribeConfigurationRevision")
            }
            KafkaActions::DescribeReplicator => write!(f, "kafka:DescribeReplicator"),
            KafkaActions::DescribeVpcConnection => write!(f, "kafka:DescribeVpcConnection"),
            KafkaActions::GetBootstrapBrokers => write!(f, "kafka:GetBootstrapBrokers"),
            KafkaActions::GetClusterPolicy => write!(f, "kafka:GetClusterPolicy"),
            KafkaActions::GetCompatibleKafkaVersions => {
                write!(f, "kafka:GetCompatibleKafkaVersions")
            }
            KafkaActions::ListClientVpcConnections => write!(f, "kafka:ListClientVpcConnections"),
            KafkaActions::ListClusterOperations => write!(f, "kafka:ListClusterOperations"),
            KafkaActions::ListClusterOperationsV2 => write!(f, "kafka:ListClusterOperationsV2"),
            KafkaActions::ListClusters => write!(f, "kafka:ListClusters"),
            KafkaActions::ListClustersV2 => write!(f, "kafka:ListClustersV2"),
            KafkaActions::ListConfigurationRevisions => {
                write!(f, "kafka:ListConfigurationRevisions")
            }
            KafkaActions::ListConfigurations => write!(f, "kafka:ListConfigurations"),
            KafkaActions::ListKafkaVersions => write!(f, "kafka:ListKafkaVersions"),
            KafkaActions::ListNodes => write!(f, "kafka:ListNodes"),
            KafkaActions::ListReplicators => write!(f, "kafka:ListReplicators"),
            KafkaActions::ListScramSecrets => write!(f, "kafka:ListScramSecrets"),
            KafkaActions::ListTagsForResource => write!(f, "kafka:ListTagsForResource"),
            KafkaActions::ListVpcConnections => write!(f, "kafka:ListVpcConnections"),
            KafkaActions::PutClusterPolicy => write!(f, "kafka:PutClusterPolicy"),
            KafkaActions::RebootBroker => write!(f, "kafka:RebootBroker"),
            KafkaActions::RejectClientVpcConnection => write!(f, "kafka:RejectClientVpcConnection"),
            KafkaActions::TagResource => write!(f, "kafka:TagResource"),
            KafkaActions::UntagResource => write!(f, "kafka:UntagResource"),
            KafkaActions::UpdateBrokerCount => write!(f, "kafka:UpdateBrokerCount"),
            KafkaActions::UpdateBrokerStorage => write!(f, "kafka:UpdateBrokerStorage"),
            KafkaActions::UpdateBrokerType => write!(f, "kafka:UpdateBrokerType"),
            KafkaActions::UpdateClusterConfiguration => {
                write!(f, "kafka:UpdateClusterConfiguration")
            }
            KafkaActions::UpdateClusterKafkaVersion => write!(f, "kafka:UpdateClusterKafkaVersion"),
            KafkaActions::UpdateConfiguration => write!(f, "kafka:UpdateConfiguration"),
            KafkaActions::UpdateConnectivity => write!(f, "kafka:UpdateConnectivity"),
            KafkaActions::UpdateMonitoring => write!(f, "kafka:UpdateMonitoring"),
            KafkaActions::UpdateReplicationInfo => write!(f, "kafka:UpdateReplicationInfo"),
            KafkaActions::UpdateSecurity => write!(f, "kafka:UpdateSecurity"),
            KafkaActions::UpdateStorage => write!(f, "kafka:UpdateStorage"),
        }
    }
}
