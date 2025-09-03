// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum SupportappActions {
    CreateSlackChannelConfiguration,
    DeleteAccountAlias,
    DeleteSlackChannelConfiguration,
    DeleteSlackWorkspaceConfiguration,
    DescribeSlackChannels,
    GetAccountAlias,
    GetSlackOauthParameters,
    ListSlackChannelConfigurations,
    ListSlackWorkspaceConfigurations,
    PutAccountAlias,
    RedeemSlackOauthCode,
    RegisterSlackWorkspaceForOrganization,
    UpdateSlackChannelConfiguration,
}
impl std::fmt::Display for SupportappActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SupportappActions::CreateSlackChannelConfiguration => {
                write!(f, "supportapp:CreateSlackChannelConfiguration")
            }
            SupportappActions::DeleteAccountAlias => write!(f, "supportapp:DeleteAccountAlias"),
            SupportappActions::DeleteSlackChannelConfiguration => {
                write!(f, "supportapp:DeleteSlackChannelConfiguration")
            }
            SupportappActions::DeleteSlackWorkspaceConfiguration => {
                write!(f, "supportapp:DeleteSlackWorkspaceConfiguration")
            }
            SupportappActions::DescribeSlackChannels => {
                write!(f, "supportapp:DescribeSlackChannels")
            }
            SupportappActions::GetAccountAlias => write!(f, "supportapp:GetAccountAlias"),
            SupportappActions::GetSlackOauthParameters => {
                write!(f, "supportapp:GetSlackOauthParameters")
            }
            SupportappActions::ListSlackChannelConfigurations => {
                write!(f, "supportapp:ListSlackChannelConfigurations")
            }
            SupportappActions::ListSlackWorkspaceConfigurations => {
                write!(f, "supportapp:ListSlackWorkspaceConfigurations")
            }
            SupportappActions::PutAccountAlias => write!(f, "supportapp:PutAccountAlias"),
            SupportappActions::RedeemSlackOauthCode => write!(f, "supportapp:RedeemSlackOauthCode"),
            SupportappActions::RegisterSlackWorkspaceForOrganization => {
                write!(f, "supportapp:RegisterSlackWorkspaceForOrganization")
            }
            SupportappActions::UpdateSlackChannelConfiguration => {
                write!(f, "supportapp:UpdateSlackChannelConfiguration")
            }
        }
    }
}
