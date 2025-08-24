// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum KafkaconnectActions {
    CreateConnector,
    CreateCustomPlugin,
    CreateWorkerConfiguration,
    DeleteConnector,
    DeleteCustomPlugin,
    DeleteWorkerConfiguration,
    DescribeConnector,
    DescribeConnectorOperation,
    DescribeCustomPlugin,
    DescribeWorkerConfiguration,
    ListConnectorOperations,
    ListConnectors,
    ListCustomPlugins,
    ListTagsForResource,
    ListWorkerConfigurations,
    TagResource,
    UntagResource,
    UpdateConnector,
}
impl std::fmt::Display for KafkaconnectActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KafkaconnectActions::CreateConnector => write!(f, "kafkaconnect:CreateConnector"),
            KafkaconnectActions::CreateCustomPlugin => write!(f, "kafkaconnect:CreateCustomPlugin"),
            KafkaconnectActions::CreateWorkerConfiguration => {
                write!(f, "kafkaconnect:CreateWorkerConfiguration")
            }
            KafkaconnectActions::DeleteConnector => write!(f, "kafkaconnect:DeleteConnector"),
            KafkaconnectActions::DeleteCustomPlugin => write!(f, "kafkaconnect:DeleteCustomPlugin"),
            KafkaconnectActions::DeleteWorkerConfiguration => {
                write!(f, "kafkaconnect:DeleteWorkerConfiguration")
            }
            KafkaconnectActions::DescribeConnector => write!(f, "kafkaconnect:DescribeConnector"),
            KafkaconnectActions::DescribeConnectorOperation => {
                write!(f, "kafkaconnect:DescribeConnectorOperation")
            }
            KafkaconnectActions::DescribeCustomPlugin => {
                write!(f, "kafkaconnect:DescribeCustomPlugin")
            }
            KafkaconnectActions::DescribeWorkerConfiguration => {
                write!(f, "kafkaconnect:DescribeWorkerConfiguration")
            }
            KafkaconnectActions::ListConnectorOperations => {
                write!(f, "kafkaconnect:ListConnectorOperations")
            }
            KafkaconnectActions::ListConnectors => write!(f, "kafkaconnect:ListConnectors"),
            KafkaconnectActions::ListCustomPlugins => write!(f, "kafkaconnect:ListCustomPlugins"),
            KafkaconnectActions::ListTagsForResource => {
                write!(f, "kafkaconnect:ListTagsForResource")
            }
            KafkaconnectActions::ListWorkerConfigurations => {
                write!(f, "kafkaconnect:ListWorkerConfigurations")
            }
            KafkaconnectActions::TagResource => write!(f, "kafkaconnect:TagResource"),
            KafkaconnectActions::UntagResource => write!(f, "kafkaconnect:UntagResource"),
            KafkaconnectActions::UpdateConnector => write!(f, "kafkaconnect:UpdateConnector"),
        }
    }
}
