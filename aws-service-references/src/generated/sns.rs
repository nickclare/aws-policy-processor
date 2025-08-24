// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum SnsActions {
    AddPermission,
    CheckIfPhoneNumberIsOptedOut,
    ConfirmSubscription,
    CreatePlatformApplication,
    CreatePlatformEndpoint,
    CreateSmsSandboxPhoneNumber,
    CreateTopic,
    DeleteEndpoint,
    DeletePlatformApplication,
    DeleteSmsSandboxPhoneNumber,
    DeleteTopic,
    GetDataProtectionPolicy,
    GetEndpointAttributes,
    GetPlatformApplicationAttributes,
    GetSmsAttributes,
    GetSmsSandboxAccountStatus,
    GetSubscriptionAttributes,
    GetTopicAttributes,
    ListEndpointsByPlatformApplication,
    ListOriginationNumbers,
    ListPhoneNumbersOptedOut,
    ListPlatformApplications,
    ListSmsSandboxPhoneNumbers,
    ListSubscriptions,
    ListSubscriptionsByTopic,
    ListTagsForResource,
    ListTopics,
    OptInPhoneNumber,
    Publish,
    PutDataProtectionPolicy,
    RemovePermission,
    SetEndpointAttributes,
    SetPlatformApplicationAttributes,
    SetSmsAttributes,
    SetSubscriptionAttributes,
    SetTopicAttributes,
    Subscribe,
    TagResource,
    Unsubscribe,
    UntagResource,
    VerifySmsSandboxPhoneNumber,
}
impl std::fmt::Display for SnsActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SnsActions::AddPermission => write!(f, "sns:AddPermission"),
            SnsActions::CheckIfPhoneNumberIsOptedOut => {
                write!(f, "sns:CheckIfPhoneNumberIsOptedOut")
            }
            SnsActions::ConfirmSubscription => write!(f, "sns:ConfirmSubscription"),
            SnsActions::CreatePlatformApplication => write!(f, "sns:CreatePlatformApplication"),
            SnsActions::CreatePlatformEndpoint => write!(f, "sns:CreatePlatformEndpoint"),
            SnsActions::CreateSmsSandboxPhoneNumber => write!(f, "sns:CreateSMSSandboxPhoneNumber"),
            SnsActions::CreateTopic => write!(f, "sns:CreateTopic"),
            SnsActions::DeleteEndpoint => write!(f, "sns:DeleteEndpoint"),
            SnsActions::DeletePlatformApplication => write!(f, "sns:DeletePlatformApplication"),
            SnsActions::DeleteSmsSandboxPhoneNumber => write!(f, "sns:DeleteSMSSandboxPhoneNumber"),
            SnsActions::DeleteTopic => write!(f, "sns:DeleteTopic"),
            SnsActions::GetDataProtectionPolicy => write!(f, "sns:GetDataProtectionPolicy"),
            SnsActions::GetEndpointAttributes => write!(f, "sns:GetEndpointAttributes"),
            SnsActions::GetPlatformApplicationAttributes => {
                write!(f, "sns:GetPlatformApplicationAttributes")
            }
            SnsActions::GetSmsAttributes => write!(f, "sns:GetSMSAttributes"),
            SnsActions::GetSmsSandboxAccountStatus => write!(f, "sns:GetSMSSandboxAccountStatus"),
            SnsActions::GetSubscriptionAttributes => write!(f, "sns:GetSubscriptionAttributes"),
            SnsActions::GetTopicAttributes => write!(f, "sns:GetTopicAttributes"),
            SnsActions::ListEndpointsByPlatformApplication => {
                write!(f, "sns:ListEndpointsByPlatformApplication")
            }
            SnsActions::ListOriginationNumbers => write!(f, "sns:ListOriginationNumbers"),
            SnsActions::ListPhoneNumbersOptedOut => write!(f, "sns:ListPhoneNumbersOptedOut"),
            SnsActions::ListPlatformApplications => write!(f, "sns:ListPlatformApplications"),
            SnsActions::ListSmsSandboxPhoneNumbers => write!(f, "sns:ListSMSSandboxPhoneNumbers"),
            SnsActions::ListSubscriptions => write!(f, "sns:ListSubscriptions"),
            SnsActions::ListSubscriptionsByTopic => write!(f, "sns:ListSubscriptionsByTopic"),
            SnsActions::ListTagsForResource => write!(f, "sns:ListTagsForResource"),
            SnsActions::ListTopics => write!(f, "sns:ListTopics"),
            SnsActions::OptInPhoneNumber => write!(f, "sns:OptInPhoneNumber"),
            SnsActions::Publish => write!(f, "sns:Publish"),
            SnsActions::PutDataProtectionPolicy => write!(f, "sns:PutDataProtectionPolicy"),
            SnsActions::RemovePermission => write!(f, "sns:RemovePermission"),
            SnsActions::SetEndpointAttributes => write!(f, "sns:SetEndpointAttributes"),
            SnsActions::SetPlatformApplicationAttributes => {
                write!(f, "sns:SetPlatformApplicationAttributes")
            }
            SnsActions::SetSmsAttributes => write!(f, "sns:SetSMSAttributes"),
            SnsActions::SetSubscriptionAttributes => write!(f, "sns:SetSubscriptionAttributes"),
            SnsActions::SetTopicAttributes => write!(f, "sns:SetTopicAttributes"),
            SnsActions::Subscribe => write!(f, "sns:Subscribe"),
            SnsActions::TagResource => write!(f, "sns:TagResource"),
            SnsActions::Unsubscribe => write!(f, "sns:Unsubscribe"),
            SnsActions::UntagResource => write!(f, "sns:UntagResource"),
            SnsActions::VerifySmsSandboxPhoneNumber => write!(f, "sns:VerifySMSSandboxPhoneNumber"),
        }
    }
}
