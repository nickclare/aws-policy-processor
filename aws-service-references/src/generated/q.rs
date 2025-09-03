// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum QActions {
    AssociateConnectorResource,
    CreateAssignment,
    CreateAuthGrant,
    CreateOAuthAppConnection,
    CreatePlugin,
    DeleteAssignment,
    DeleteConversation,
    DeleteOAuthAppConnection,
    DeletePlugin,
    GenerateCodeFromCommands,
    GenerateCodeRecommendations,
    GetConnector,
    GetConversation,
    GetIdentityMetadata,
    GetPlugin,
    GetTroubleshootingResults,
    ListConversations,
    ListDashboardMetrics,
    ListPluginProviders,
    ListPlugins,
    ListTagsForResource,
    PassRequest,
    RejectConnector,
    SendEvent,
    SendMessage,
    StartConversation,
    StartTroubleshootingAnalysis,
    StartTroubleshootingResolutionExplanation,
    TagResource,
    UntagResource,
    UpdateAuthGrant,
    UpdateConversation,
    UpdateOAuthAppConnection,
    UpdatePlugin,
    UpdateTroubleshootingCommandResult,
    UsePlugin,
    VerifyOAuthAppConnection,
}
impl std::fmt::Display for QActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QActions::AssociateConnectorResource => write!(f, "q:AssociateConnectorResource"),
            QActions::CreateAssignment => write!(f, "q:CreateAssignment"),
            QActions::CreateAuthGrant => write!(f, "q:CreateAuthGrant"),
            QActions::CreateOAuthAppConnection => write!(f, "q:CreateOAuthAppConnection"),
            QActions::CreatePlugin => write!(f, "q:CreatePlugin"),
            QActions::DeleteAssignment => write!(f, "q:DeleteAssignment"),
            QActions::DeleteConversation => write!(f, "q:DeleteConversation"),
            QActions::DeleteOAuthAppConnection => write!(f, "q:DeleteOAuthAppConnection"),
            QActions::DeletePlugin => write!(f, "q:DeletePlugin"),
            QActions::GenerateCodeFromCommands => write!(f, "q:GenerateCodeFromCommands"),
            QActions::GenerateCodeRecommendations => write!(f, "q:GenerateCodeRecommendations"),
            QActions::GetConnector => write!(f, "q:GetConnector"),
            QActions::GetConversation => write!(f, "q:GetConversation"),
            QActions::GetIdentityMetadata => write!(f, "q:GetIdentityMetadata"),
            QActions::GetPlugin => write!(f, "q:GetPlugin"),
            QActions::GetTroubleshootingResults => write!(f, "q:GetTroubleshootingResults"),
            QActions::ListConversations => write!(f, "q:ListConversations"),
            QActions::ListDashboardMetrics => write!(f, "q:ListDashboardMetrics"),
            QActions::ListPluginProviders => write!(f, "q:ListPluginProviders"),
            QActions::ListPlugins => write!(f, "q:ListPlugins"),
            QActions::ListTagsForResource => write!(f, "q:ListTagsForResource"),
            QActions::PassRequest => write!(f, "q:PassRequest"),
            QActions::RejectConnector => write!(f, "q:RejectConnector"),
            QActions::SendEvent => write!(f, "q:SendEvent"),
            QActions::SendMessage => write!(f, "q:SendMessage"),
            QActions::StartConversation => write!(f, "q:StartConversation"),
            QActions::StartTroubleshootingAnalysis => write!(f, "q:StartTroubleshootingAnalysis"),
            QActions::StartTroubleshootingResolutionExplanation => {
                write!(f, "q:StartTroubleshootingResolutionExplanation")
            }
            QActions::TagResource => write!(f, "q:TagResource"),
            QActions::UntagResource => write!(f, "q:UntagResource"),
            QActions::UpdateAuthGrant => write!(f, "q:UpdateAuthGrant"),
            QActions::UpdateConversation => write!(f, "q:UpdateConversation"),
            QActions::UpdateOAuthAppConnection => write!(f, "q:UpdateOAuthAppConnection"),
            QActions::UpdatePlugin => write!(f, "q:UpdatePlugin"),
            QActions::UpdateTroubleshootingCommandResult => {
                write!(f, "q:UpdateTroubleshootingCommandResult")
            }
            QActions::UsePlugin => write!(f, "q:UsePlugin"),
            QActions::VerifyOAuthAppConnection => write!(f, "q:VerifyOAuthAppConnection"),
        }
    }
}
