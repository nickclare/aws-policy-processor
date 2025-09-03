// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum SecuritylakeActions {
    CreateAwsLogSource,
    CreateCustomLogSource,
    CreateDataLake,
    CreateDataLakeExceptionSubscription,
    CreateDataLakeOrganizationConfiguration,
    CreateSubscriber,
    CreateSubscriberNotification,
    DeleteAwsLogSource,
    DeleteCustomLogSource,
    DeleteDataLake,
    DeleteDataLakeExceptionSubscription,
    DeleteDataLakeOrganizationConfiguration,
    DeleteSubscriber,
    DeleteSubscriberNotification,
    DeregisterDataLakeDelegatedAdministrator,
    GetDataLakeExceptionSubscription,
    GetDataLakeOrganizationConfiguration,
    GetDataLakeSources,
    GetSubscriber,
    ListDataLakeExceptions,
    ListDataLakes,
    ListLogSources,
    ListSubscribers,
    ListTagsForResource,
    RegisterDataLakeDelegatedAdministrator,
    TagResource,
    UntagResource,
    UpdateDataLake,
    UpdateDataLakeExceptionSubscription,
    UpdateSubscriber,
    UpdateSubscriberNotification,
}
impl std::fmt::Display for SecuritylakeActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SecuritylakeActions::CreateAwsLogSource => write!(f, "securitylake:CreateAwsLogSource"),
            SecuritylakeActions::CreateCustomLogSource => {
                write!(f, "securitylake:CreateCustomLogSource")
            }
            SecuritylakeActions::CreateDataLake => write!(f, "securitylake:CreateDataLake"),
            SecuritylakeActions::CreateDataLakeExceptionSubscription => {
                write!(f, "securitylake:CreateDataLakeExceptionSubscription")
            }
            SecuritylakeActions::CreateDataLakeOrganizationConfiguration => {
                write!(f, "securitylake:CreateDataLakeOrganizationConfiguration")
            }
            SecuritylakeActions::CreateSubscriber => write!(f, "securitylake:CreateSubscriber"),
            SecuritylakeActions::CreateSubscriberNotification => {
                write!(f, "securitylake:CreateSubscriberNotification")
            }
            SecuritylakeActions::DeleteAwsLogSource => write!(f, "securitylake:DeleteAwsLogSource"),
            SecuritylakeActions::DeleteCustomLogSource => {
                write!(f, "securitylake:DeleteCustomLogSource")
            }
            SecuritylakeActions::DeleteDataLake => write!(f, "securitylake:DeleteDataLake"),
            SecuritylakeActions::DeleteDataLakeExceptionSubscription => {
                write!(f, "securitylake:DeleteDataLakeExceptionSubscription")
            }
            SecuritylakeActions::DeleteDataLakeOrganizationConfiguration => {
                write!(f, "securitylake:DeleteDataLakeOrganizationConfiguration")
            }
            SecuritylakeActions::DeleteSubscriber => write!(f, "securitylake:DeleteSubscriber"),
            SecuritylakeActions::DeleteSubscriberNotification => {
                write!(f, "securitylake:DeleteSubscriberNotification")
            }
            SecuritylakeActions::DeregisterDataLakeDelegatedAdministrator => {
                write!(f, "securitylake:DeregisterDataLakeDelegatedAdministrator")
            }
            SecuritylakeActions::GetDataLakeExceptionSubscription => {
                write!(f, "securitylake:GetDataLakeExceptionSubscription")
            }
            SecuritylakeActions::GetDataLakeOrganizationConfiguration => {
                write!(f, "securitylake:GetDataLakeOrganizationConfiguration")
            }
            SecuritylakeActions::GetDataLakeSources => write!(f, "securitylake:GetDataLakeSources"),
            SecuritylakeActions::GetSubscriber => write!(f, "securitylake:GetSubscriber"),
            SecuritylakeActions::ListDataLakeExceptions => {
                write!(f, "securitylake:ListDataLakeExceptions")
            }
            SecuritylakeActions::ListDataLakes => write!(f, "securitylake:ListDataLakes"),
            SecuritylakeActions::ListLogSources => write!(f, "securitylake:ListLogSources"),
            SecuritylakeActions::ListSubscribers => write!(f, "securitylake:ListSubscribers"),
            SecuritylakeActions::ListTagsForResource => {
                write!(f, "securitylake:ListTagsForResource")
            }
            SecuritylakeActions::RegisterDataLakeDelegatedAdministrator => {
                write!(f, "securitylake:RegisterDataLakeDelegatedAdministrator")
            }
            SecuritylakeActions::TagResource => write!(f, "securitylake:TagResource"),
            SecuritylakeActions::UntagResource => write!(f, "securitylake:UntagResource"),
            SecuritylakeActions::UpdateDataLake => write!(f, "securitylake:UpdateDataLake"),
            SecuritylakeActions::UpdateDataLakeExceptionSubscription => {
                write!(f, "securitylake:UpdateDataLakeExceptionSubscription")
            }
            SecuritylakeActions::UpdateSubscriber => write!(f, "securitylake:UpdateSubscriber"),
            SecuritylakeActions::UpdateSubscriberNotification => {
                write!(f, "securitylake:UpdateSubscriberNotification")
            }
        }
    }
}
