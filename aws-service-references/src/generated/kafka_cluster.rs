// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum KafkaClusterActions {
    AlterCluster,
    AlterClusterDynamicConfiguration,
    AlterGroup,
    AlterTopic,
    AlterTopicDynamicConfiguration,
    AlterTransactionalId,
    Connect,
    CreateTopic,
    DeleteGroup,
    DeleteTopic,
    DescribeCluster,
    DescribeClusterDynamicConfiguration,
    DescribeGroup,
    DescribeTopic,
    DescribeTopicDynamicConfiguration,
    DescribeTransactionalId,
    ReadData,
    WriteData,
    WriteDataIdempotently,
}
impl std::fmt::Display for KafkaClusterActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KafkaClusterActions::AlterCluster => write!(f, "kafka-cluster:AlterCluster"),
            KafkaClusterActions::AlterClusterDynamicConfiguration => {
                write!(f, "kafka-cluster:AlterClusterDynamicConfiguration")
            }
            KafkaClusterActions::AlterGroup => write!(f, "kafka-cluster:AlterGroup"),
            KafkaClusterActions::AlterTopic => write!(f, "kafka-cluster:AlterTopic"),
            KafkaClusterActions::AlterTopicDynamicConfiguration => {
                write!(f, "kafka-cluster:AlterTopicDynamicConfiguration")
            }
            KafkaClusterActions::AlterTransactionalId => {
                write!(f, "kafka-cluster:AlterTransactionalId")
            }
            KafkaClusterActions::Connect => write!(f, "kafka-cluster:Connect"),
            KafkaClusterActions::CreateTopic => write!(f, "kafka-cluster:CreateTopic"),
            KafkaClusterActions::DeleteGroup => write!(f, "kafka-cluster:DeleteGroup"),
            KafkaClusterActions::DeleteTopic => write!(f, "kafka-cluster:DeleteTopic"),
            KafkaClusterActions::DescribeCluster => write!(f, "kafka-cluster:DescribeCluster"),
            KafkaClusterActions::DescribeClusterDynamicConfiguration => {
                write!(f, "kafka-cluster:DescribeClusterDynamicConfiguration")
            }
            KafkaClusterActions::DescribeGroup => write!(f, "kafka-cluster:DescribeGroup"),
            KafkaClusterActions::DescribeTopic => write!(f, "kafka-cluster:DescribeTopic"),
            KafkaClusterActions::DescribeTopicDynamicConfiguration => {
                write!(f, "kafka-cluster:DescribeTopicDynamicConfiguration")
            }
            KafkaClusterActions::DescribeTransactionalId => {
                write!(f, "kafka-cluster:DescribeTransactionalId")
            }
            KafkaClusterActions::ReadData => write!(f, "kafka-cluster:ReadData"),
            KafkaClusterActions::WriteData => write!(f, "kafka-cluster:WriteData"),
            KafkaClusterActions::WriteDataIdempotently => {
                write!(f, "kafka-cluster:WriteDataIdempotently")
            }
        }
    }
}
