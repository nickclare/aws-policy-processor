// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum IvschatActions {
    CreateChatToken,
    CreateLoggingConfiguration,
    CreateRoom,
    DeleteLoggingConfiguration,
    DeleteMessage,
    DeleteRoom,
    DisconnectUser,
    GetLoggingConfiguration,
    GetRoom,
    ListLoggingConfigurations,
    ListRooms,
    ListTagsForResource,
    SendEvent,
    TagResource,
    UntagResource,
    UpdateLoggingConfiguration,
    UpdateRoom,
}
impl std::fmt::Display for IvschatActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IvschatActions::CreateChatToken => write!(f, "ivschat:CreateChatToken"),
            IvschatActions::CreateLoggingConfiguration => {
                write!(f, "ivschat:CreateLoggingConfiguration")
            }
            IvschatActions::CreateRoom => write!(f, "ivschat:CreateRoom"),
            IvschatActions::DeleteLoggingConfiguration => {
                write!(f, "ivschat:DeleteLoggingConfiguration")
            }
            IvschatActions::DeleteMessage => write!(f, "ivschat:DeleteMessage"),
            IvschatActions::DeleteRoom => write!(f, "ivschat:DeleteRoom"),
            IvschatActions::DisconnectUser => write!(f, "ivschat:DisconnectUser"),
            IvschatActions::GetLoggingConfiguration => write!(f, "ivschat:GetLoggingConfiguration"),
            IvschatActions::GetRoom => write!(f, "ivschat:GetRoom"),
            IvschatActions::ListLoggingConfigurations => {
                write!(f, "ivschat:ListLoggingConfigurations")
            }
            IvschatActions::ListRooms => write!(f, "ivschat:ListRooms"),
            IvschatActions::ListTagsForResource => write!(f, "ivschat:ListTagsForResource"),
            IvschatActions::SendEvent => write!(f, "ivschat:SendEvent"),
            IvschatActions::TagResource => write!(f, "ivschat:TagResource"),
            IvschatActions::UntagResource => write!(f, "ivschat:UntagResource"),
            IvschatActions::UpdateLoggingConfiguration => {
                write!(f, "ivschat:UpdateLoggingConfiguration")
            }
            IvschatActions::UpdateRoom => write!(f, "ivschat:UpdateRoom"),
        }
    }
}
