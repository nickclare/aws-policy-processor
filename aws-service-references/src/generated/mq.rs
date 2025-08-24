// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum MqActions {
    CreateBroker,
    CreateConfiguration,
    CreateReplicaBroker,
    CreateTags,
    CreateUser,
    DeleteBroker,
    DeleteConfiguration,
    DeleteTags,
    DeleteUser,
    DescribeBroker,
    DescribeBrokerEngineTypes,
    DescribeBrokerInstanceOptions,
    DescribeConfiguration,
    DescribeConfigurationRevision,
    DescribeUser,
    ListBrokers,
    ListConfigurationRevisions,
    ListConfigurations,
    ListTags,
    ListUsers,
    Promote,
    RebootBroker,
    UpdateBroker,
    UpdateConfiguration,
    UpdateUser,
}
impl std::fmt::Display for MqActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MqActions::CreateBroker => write!(f, "mq:CreateBroker"),
            MqActions::CreateConfiguration => write!(f, "mq:CreateConfiguration"),
            MqActions::CreateReplicaBroker => write!(f, "mq:CreateReplicaBroker"),
            MqActions::CreateTags => write!(f, "mq:CreateTags"),
            MqActions::CreateUser => write!(f, "mq:CreateUser"),
            MqActions::DeleteBroker => write!(f, "mq:DeleteBroker"),
            MqActions::DeleteConfiguration => write!(f, "mq:DeleteConfiguration"),
            MqActions::DeleteTags => write!(f, "mq:DeleteTags"),
            MqActions::DeleteUser => write!(f, "mq:DeleteUser"),
            MqActions::DescribeBroker => write!(f, "mq:DescribeBroker"),
            MqActions::DescribeBrokerEngineTypes => write!(f, "mq:DescribeBrokerEngineTypes"),
            MqActions::DescribeBrokerInstanceOptions => {
                write!(f, "mq:DescribeBrokerInstanceOptions")
            }
            MqActions::DescribeConfiguration => write!(f, "mq:DescribeConfiguration"),
            MqActions::DescribeConfigurationRevision => {
                write!(f, "mq:DescribeConfigurationRevision")
            }
            MqActions::DescribeUser => write!(f, "mq:DescribeUser"),
            MqActions::ListBrokers => write!(f, "mq:ListBrokers"),
            MqActions::ListConfigurationRevisions => write!(f, "mq:ListConfigurationRevisions"),
            MqActions::ListConfigurations => write!(f, "mq:ListConfigurations"),
            MqActions::ListTags => write!(f, "mq:ListTags"),
            MqActions::ListUsers => write!(f, "mq:ListUsers"),
            MqActions::Promote => write!(f, "mq:Promote"),
            MqActions::RebootBroker => write!(f, "mq:RebootBroker"),
            MqActions::UpdateBroker => write!(f, "mq:UpdateBroker"),
            MqActions::UpdateConfiguration => write!(f, "mq:UpdateConfiguration"),
            MqActions::UpdateUser => write!(f, "mq:UpdateUser"),
        }
    }
}
