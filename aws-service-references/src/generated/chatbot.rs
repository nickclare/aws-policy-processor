// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum ChatbotActions {
    AssociateToConfiguration,
    CreateChimeWebhookConfiguration,
    CreateCustomAction,
    CreateMicrosoftTeamsChannelConfiguration,
    CreateSlackChannelConfiguration,
    DeleteChimeWebhookConfiguration,
    DeleteCustomAction,
    DeleteMicrosoftTeamsChannelConfiguration,
    DeleteMicrosoftTeamsConfiguredTeam,
    DeleteMicrosoftTeamsUserIdentity,
    DeleteSlackChannelConfiguration,
    DeleteSlackUserIdentity,
    DeleteSlackWorkspaceAuthorization,
    DescribeChimeWebhookConfigurations,
    DescribeSlackChannelConfigurations,
    DescribeSlackChannels,
    DescribeSlackUserIdentities,
    DescribeSlackWorkspaces,
    DisassociateFromConfiguration,
    GetAccountPreferences,
    GetCustomAction,
    GetMicrosoftTeamsChannelConfiguration,
    GetMicrosoftTeamsOauthParameters,
    GetSlackOauthParameters,
    ListAssociations,
    ListCustomActions,
    ListMicrosoftTeamsChannelConfigurations,
    ListMicrosoftTeamsConfiguredTeams,
    ListMicrosoftTeamsUserIdentities,
    ListTagsForResource,
    RedeemMicrosoftTeamsOauthCode,
    RedeemSlackOauthCode,
    TagResource,
    UntagResource,
    UpdateAccountPreferences,
    UpdateChimeWebhookConfiguration,
    UpdateCustomAction,
    UpdateMicrosoftTeamsChannelConfiguration,
    UpdateSlackChannelConfiguration,
}
impl std::fmt::Display for ChatbotActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChatbotActions::AssociateToConfiguration => {
                write!(f, "chatbot:AssociateToConfiguration")
            }
            ChatbotActions::CreateChimeWebhookConfiguration => {
                write!(f, "chatbot:CreateChimeWebhookConfiguration")
            }
            ChatbotActions::CreateCustomAction => write!(f, "chatbot:CreateCustomAction"),
            ChatbotActions::CreateMicrosoftTeamsChannelConfiguration => {
                write!(f, "chatbot:CreateMicrosoftTeamsChannelConfiguration")
            }
            ChatbotActions::CreateSlackChannelConfiguration => {
                write!(f, "chatbot:CreateSlackChannelConfiguration")
            }
            ChatbotActions::DeleteChimeWebhookConfiguration => {
                write!(f, "chatbot:DeleteChimeWebhookConfiguration")
            }
            ChatbotActions::DeleteCustomAction => write!(f, "chatbot:DeleteCustomAction"),
            ChatbotActions::DeleteMicrosoftTeamsChannelConfiguration => {
                write!(f, "chatbot:DeleteMicrosoftTeamsChannelConfiguration")
            }
            ChatbotActions::DeleteMicrosoftTeamsConfiguredTeam => {
                write!(f, "chatbot:DeleteMicrosoftTeamsConfiguredTeam")
            }
            ChatbotActions::DeleteMicrosoftTeamsUserIdentity => {
                write!(f, "chatbot:DeleteMicrosoftTeamsUserIdentity")
            }
            ChatbotActions::DeleteSlackChannelConfiguration => {
                write!(f, "chatbot:DeleteSlackChannelConfiguration")
            }
            ChatbotActions::DeleteSlackUserIdentity => write!(f, "chatbot:DeleteSlackUserIdentity"),
            ChatbotActions::DeleteSlackWorkspaceAuthorization => {
                write!(f, "chatbot:DeleteSlackWorkspaceAuthorization")
            }
            ChatbotActions::DescribeChimeWebhookConfigurations => {
                write!(f, "chatbot:DescribeChimeWebhookConfigurations")
            }
            ChatbotActions::DescribeSlackChannelConfigurations => {
                write!(f, "chatbot:DescribeSlackChannelConfigurations")
            }
            ChatbotActions::DescribeSlackChannels => write!(f, "chatbot:DescribeSlackChannels"),
            ChatbotActions::DescribeSlackUserIdentities => {
                write!(f, "chatbot:DescribeSlackUserIdentities")
            }
            ChatbotActions::DescribeSlackWorkspaces => write!(f, "chatbot:DescribeSlackWorkspaces"),
            ChatbotActions::DisassociateFromConfiguration => {
                write!(f, "chatbot:DisassociateFromConfiguration")
            }
            ChatbotActions::GetAccountPreferences => write!(f, "chatbot:GetAccountPreferences"),
            ChatbotActions::GetCustomAction => write!(f, "chatbot:GetCustomAction"),
            ChatbotActions::GetMicrosoftTeamsChannelConfiguration => {
                write!(f, "chatbot:GetMicrosoftTeamsChannelConfiguration")
            }
            ChatbotActions::GetMicrosoftTeamsOauthParameters => {
                write!(f, "chatbot:GetMicrosoftTeamsOauthParameters")
            }
            ChatbotActions::GetSlackOauthParameters => write!(f, "chatbot:GetSlackOauthParameters"),
            ChatbotActions::ListAssociations => write!(f, "chatbot:ListAssociations"),
            ChatbotActions::ListCustomActions => write!(f, "chatbot:ListCustomActions"),
            ChatbotActions::ListMicrosoftTeamsChannelConfigurations => {
                write!(f, "chatbot:ListMicrosoftTeamsChannelConfigurations")
            }
            ChatbotActions::ListMicrosoftTeamsConfiguredTeams => {
                write!(f, "chatbot:ListMicrosoftTeamsConfiguredTeams")
            }
            ChatbotActions::ListMicrosoftTeamsUserIdentities => {
                write!(f, "chatbot:ListMicrosoftTeamsUserIdentities")
            }
            ChatbotActions::ListTagsForResource => write!(f, "chatbot:ListTagsForResource"),
            ChatbotActions::RedeemMicrosoftTeamsOauthCode => {
                write!(f, "chatbot:RedeemMicrosoftTeamsOauthCode")
            }
            ChatbotActions::RedeemSlackOauthCode => write!(f, "chatbot:RedeemSlackOauthCode"),
            ChatbotActions::TagResource => write!(f, "chatbot:TagResource"),
            ChatbotActions::UntagResource => write!(f, "chatbot:UntagResource"),
            ChatbotActions::UpdateAccountPreferences => {
                write!(f, "chatbot:UpdateAccountPreferences")
            }
            ChatbotActions::UpdateChimeWebhookConfiguration => {
                write!(f, "chatbot:UpdateChimeWebhookConfiguration")
            }
            ChatbotActions::UpdateCustomAction => write!(f, "chatbot:UpdateCustomAction"),
            ChatbotActions::UpdateMicrosoftTeamsChannelConfiguration => {
                write!(f, "chatbot:UpdateMicrosoftTeamsChannelConfiguration")
            }
            ChatbotActions::UpdateSlackChannelConfiguration => {
                write!(f, "chatbot:UpdateSlackChannelConfiguration")
            }
        }
    }
}
