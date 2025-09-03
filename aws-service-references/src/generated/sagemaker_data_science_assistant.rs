// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum SagemakerDataScienceAssistantActions {
    SendConversation,
}
impl std::fmt::Display for SagemakerDataScienceAssistantActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SagemakerDataScienceAssistantActions::SendConversation => {
                write!(f, "sagemaker-data-science-assistant:SendConversation")
            }
        }
    }
}
