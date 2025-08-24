// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum WorkmailmessageflowActions {
    GetRawMessageContent,
    PutRawMessageContent,
}
impl std::fmt::Display for WorkmailmessageflowActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WorkmailmessageflowActions::GetRawMessageContent => {
                write!(f, "workmailmessageflow:GetRawMessageContent")
            }
            WorkmailmessageflowActions::PutRawMessageContent => {
                write!(f, "workmailmessageflow:PutRawMessageContent")
            }
        }
    }
}
