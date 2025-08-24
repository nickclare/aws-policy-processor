// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum Ec2messagesActions {
    AcknowledgeMessage,
    DeleteMessage,
    FailMessage,
    GetEndpoint,
    GetMessages,
    SendReply,
}
impl std::fmt::Display for Ec2messagesActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Ec2messagesActions::AcknowledgeMessage => write!(f, "ec2messages:AcknowledgeMessage"),
            Ec2messagesActions::DeleteMessage => write!(f, "ec2messages:DeleteMessage"),
            Ec2messagesActions::FailMessage => write!(f, "ec2messages:FailMessage"),
            Ec2messagesActions::GetEndpoint => write!(f, "ec2messages:GetEndpoint"),
            Ec2messagesActions::GetMessages => write!(f, "ec2messages:GetMessages"),
            Ec2messagesActions::SendReply => write!(f, "ec2messages:SendReply"),
        }
    }
}
