// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum NetworkmanagerChatActions {
    CancelMessageResponse,
    CreateConversation,
    DeleteConversation,
    ListConversationMessages,
    ListConversations,
    NotifyConversationIsActive,
    SendConversationMessage,
}
impl std::fmt::Display for NetworkmanagerChatActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NetworkmanagerChatActions::CancelMessageResponse => {
                write!(f, "networkmanager-chat:CancelMessageResponse")
            }
            NetworkmanagerChatActions::CreateConversation => {
                write!(f, "networkmanager-chat:CreateConversation")
            }
            NetworkmanagerChatActions::DeleteConversation => {
                write!(f, "networkmanager-chat:DeleteConversation")
            }
            NetworkmanagerChatActions::ListConversationMessages => {
                write!(f, "networkmanager-chat:ListConversationMessages")
            }
            NetworkmanagerChatActions::ListConversations => {
                write!(f, "networkmanager-chat:ListConversations")
            }
            NetworkmanagerChatActions::NotifyConversationIsActive => {
                write!(f, "networkmanager-chat:NotifyConversationIsActive")
            }
            NetworkmanagerChatActions::SendConversationMessage => {
                write!(f, "networkmanager-chat:SendConversationMessage")
            }
        }
    }
}
