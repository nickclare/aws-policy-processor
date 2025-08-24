// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum WisdomActions {
    ActivateMessageTemplate,
    AllowVendedLogDeliveryForResource,
    CreateAiAgent,
    CreateAiAgentVersion,
    CreateAiGuardrail,
    CreateAiGuardrailVersion,
    CreateAiPrompt,
    CreateAiPromptVersion,
    CreateAssistant,
    CreateAssistantAssociation,
    CreateContent,
    CreateContentAssociation,
    CreateKnowledgeBase,
    CreateMessageTemplate,
    CreateMessageTemplateAttachment,
    CreateMessageTemplateVersion,
    CreateQuickResponse,
    CreateSession,
    DeactivateMessageTemplate,
    DeleteAiAgent,
    DeleteAiAgentVersion,
    DeleteAiGuardrail,
    DeleteAiGuardrailVersion,
    DeleteAiPrompt,
    DeleteAiPromptVersion,
    DeleteAssistant,
    DeleteAssistantAssociation,
    DeleteContent,
    DeleteContentAssociation,
    DeleteImportJob,
    DeleteKnowledgeBase,
    DeleteMessageTemplate,
    DeleteMessageTemplateAttachment,
    DeleteQuickResponse,
    GetAiAgent,
    GetAiGuardrail,
    GetAiPrompt,
    GetAssistant,
    GetAssistantAssociation,
    GetContent,
    GetContentAssociation,
    GetContentSummary,
    GetImportJob,
    GetKnowledgeBase,
    GetMessageTemplate,
    GetNextMessage,
    GetQuickResponse,
    GetRecommendations,
    GetSession,
    ListAiAgentVersions,
    ListAiAgents,
    ListAiGuardrailVersions,
    ListAiGuardrails,
    ListAiPromptVersions,
    ListAiPrompts,
    ListAssistantAssociations,
    ListAssistants,
    ListContentAssociations,
    ListContents,
    ListImportJobs,
    ListKnowledgeBases,
    ListMessageTemplateVersions,
    ListMessageTemplates,
    ListMessages,
    ListQuickResponses,
    ListTagsForResource,
    NotifyRecommendationsReceived,
    PutFeedback,
    QueryAssistant,
    RemoveAssistantAiAgent,
    RemoveKnowledgeBaseTemplateUri,
    RenderMessageTemplate,
    SearchContent,
    SearchMessageTemplates,
    SearchQuickResponses,
    SearchSessions,
    SendMessage,
    StartContentUpload,
    StartImportJob,
    TagResource,
    UntagResource,
    UpdateAiAgent,
    UpdateAiGuardrail,
    UpdateAiPrompt,
    UpdateAssistantAiAgent,
    UpdateContent,
    UpdateKnowledgeBaseTemplateUri,
    UpdateMessageTemplate,
    UpdateMessageTemplateMetadata,
    UpdateQuickResponse,
    UpdateSession,
    UpdateSessionData,
}
impl std::fmt::Display for WisdomActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WisdomActions::ActivateMessageTemplate => write!(f, "wisdom:ActivateMessageTemplate"),
            WisdomActions::AllowVendedLogDeliveryForResource => {
                write!(f, "wisdom:AllowVendedLogDeliveryForResource")
            }
            WisdomActions::CreateAiAgent => write!(f, "wisdom:CreateAIAgent"),
            WisdomActions::CreateAiAgentVersion => write!(f, "wisdom:CreateAIAgentVersion"),
            WisdomActions::CreateAiGuardrail => write!(f, "wisdom:CreateAIGuardrail"),
            WisdomActions::CreateAiGuardrailVersion => write!(f, "wisdom:CreateAIGuardrailVersion"),
            WisdomActions::CreateAiPrompt => write!(f, "wisdom:CreateAIPrompt"),
            WisdomActions::CreateAiPromptVersion => write!(f, "wisdom:CreateAIPromptVersion"),
            WisdomActions::CreateAssistant => write!(f, "wisdom:CreateAssistant"),
            WisdomActions::CreateAssistantAssociation => {
                write!(f, "wisdom:CreateAssistantAssociation")
            }
            WisdomActions::CreateContent => write!(f, "wisdom:CreateContent"),
            WisdomActions::CreateContentAssociation => write!(f, "wisdom:CreateContentAssociation"),
            WisdomActions::CreateKnowledgeBase => write!(f, "wisdom:CreateKnowledgeBase"),
            WisdomActions::CreateMessageTemplate => write!(f, "wisdom:CreateMessageTemplate"),
            WisdomActions::CreateMessageTemplateAttachment => {
                write!(f, "wisdom:CreateMessageTemplateAttachment")
            }
            WisdomActions::CreateMessageTemplateVersion => {
                write!(f, "wisdom:CreateMessageTemplateVersion")
            }
            WisdomActions::CreateQuickResponse => write!(f, "wisdom:CreateQuickResponse"),
            WisdomActions::CreateSession => write!(f, "wisdom:CreateSession"),
            WisdomActions::DeactivateMessageTemplate => {
                write!(f, "wisdom:DeactivateMessageTemplate")
            }
            WisdomActions::DeleteAiAgent => write!(f, "wisdom:DeleteAIAgent"),
            WisdomActions::DeleteAiAgentVersion => write!(f, "wisdom:DeleteAIAgentVersion"),
            WisdomActions::DeleteAiGuardrail => write!(f, "wisdom:DeleteAIGuardrail"),
            WisdomActions::DeleteAiGuardrailVersion => write!(f, "wisdom:DeleteAIGuardrailVersion"),
            WisdomActions::DeleteAiPrompt => write!(f, "wisdom:DeleteAIPrompt"),
            WisdomActions::DeleteAiPromptVersion => write!(f, "wisdom:DeleteAIPromptVersion"),
            WisdomActions::DeleteAssistant => write!(f, "wisdom:DeleteAssistant"),
            WisdomActions::DeleteAssistantAssociation => {
                write!(f, "wisdom:DeleteAssistantAssociation")
            }
            WisdomActions::DeleteContent => write!(f, "wisdom:DeleteContent"),
            WisdomActions::DeleteContentAssociation => write!(f, "wisdom:DeleteContentAssociation"),
            WisdomActions::DeleteImportJob => write!(f, "wisdom:DeleteImportJob"),
            WisdomActions::DeleteKnowledgeBase => write!(f, "wisdom:DeleteKnowledgeBase"),
            WisdomActions::DeleteMessageTemplate => write!(f, "wisdom:DeleteMessageTemplate"),
            WisdomActions::DeleteMessageTemplateAttachment => {
                write!(f, "wisdom:DeleteMessageTemplateAttachment")
            }
            WisdomActions::DeleteQuickResponse => write!(f, "wisdom:DeleteQuickResponse"),
            WisdomActions::GetAiAgent => write!(f, "wisdom:GetAIAgent"),
            WisdomActions::GetAiGuardrail => write!(f, "wisdom:GetAIGuardrail"),
            WisdomActions::GetAiPrompt => write!(f, "wisdom:GetAIPrompt"),
            WisdomActions::GetAssistant => write!(f, "wisdom:GetAssistant"),
            WisdomActions::GetAssistantAssociation => write!(f, "wisdom:GetAssistantAssociation"),
            WisdomActions::GetContent => write!(f, "wisdom:GetContent"),
            WisdomActions::GetContentAssociation => write!(f, "wisdom:GetContentAssociation"),
            WisdomActions::GetContentSummary => write!(f, "wisdom:GetContentSummary"),
            WisdomActions::GetImportJob => write!(f, "wisdom:GetImportJob"),
            WisdomActions::GetKnowledgeBase => write!(f, "wisdom:GetKnowledgeBase"),
            WisdomActions::GetMessageTemplate => write!(f, "wisdom:GetMessageTemplate"),
            WisdomActions::GetNextMessage => write!(f, "wisdom:GetNextMessage"),
            WisdomActions::GetQuickResponse => write!(f, "wisdom:GetQuickResponse"),
            WisdomActions::GetRecommendations => write!(f, "wisdom:GetRecommendations"),
            WisdomActions::GetSession => write!(f, "wisdom:GetSession"),
            WisdomActions::ListAiAgentVersions => write!(f, "wisdom:ListAIAgentVersions"),
            WisdomActions::ListAiAgents => write!(f, "wisdom:ListAIAgents"),
            WisdomActions::ListAiGuardrailVersions => write!(f, "wisdom:ListAIGuardrailVersions"),
            WisdomActions::ListAiGuardrails => write!(f, "wisdom:ListAIGuardrails"),
            WisdomActions::ListAiPromptVersions => write!(f, "wisdom:ListAIPromptVersions"),
            WisdomActions::ListAiPrompts => write!(f, "wisdom:ListAIPrompts"),
            WisdomActions::ListAssistantAssociations => {
                write!(f, "wisdom:ListAssistantAssociations")
            }
            WisdomActions::ListAssistants => write!(f, "wisdom:ListAssistants"),
            WisdomActions::ListContentAssociations => write!(f, "wisdom:ListContentAssociations"),
            WisdomActions::ListContents => write!(f, "wisdom:ListContents"),
            WisdomActions::ListImportJobs => write!(f, "wisdom:ListImportJobs"),
            WisdomActions::ListKnowledgeBases => write!(f, "wisdom:ListKnowledgeBases"),
            WisdomActions::ListMessageTemplateVersions => {
                write!(f, "wisdom:ListMessageTemplateVersions")
            }
            WisdomActions::ListMessageTemplates => write!(f, "wisdom:ListMessageTemplates"),
            WisdomActions::ListMessages => write!(f, "wisdom:ListMessages"),
            WisdomActions::ListQuickResponses => write!(f, "wisdom:ListQuickResponses"),
            WisdomActions::ListTagsForResource => write!(f, "wisdom:ListTagsForResource"),
            WisdomActions::NotifyRecommendationsReceived => {
                write!(f, "wisdom:NotifyRecommendationsReceived")
            }
            WisdomActions::PutFeedback => write!(f, "wisdom:PutFeedback"),
            WisdomActions::QueryAssistant => write!(f, "wisdom:QueryAssistant"),
            WisdomActions::RemoveAssistantAiAgent => write!(f, "wisdom:RemoveAssistantAIAgent"),
            WisdomActions::RemoveKnowledgeBaseTemplateUri => {
                write!(f, "wisdom:RemoveKnowledgeBaseTemplateUri")
            }
            WisdomActions::RenderMessageTemplate => write!(f, "wisdom:RenderMessageTemplate"),
            WisdomActions::SearchContent => write!(f, "wisdom:SearchContent"),
            WisdomActions::SearchMessageTemplates => write!(f, "wisdom:SearchMessageTemplates"),
            WisdomActions::SearchQuickResponses => write!(f, "wisdom:SearchQuickResponses"),
            WisdomActions::SearchSessions => write!(f, "wisdom:SearchSessions"),
            WisdomActions::SendMessage => write!(f, "wisdom:SendMessage"),
            WisdomActions::StartContentUpload => write!(f, "wisdom:StartContentUpload"),
            WisdomActions::StartImportJob => write!(f, "wisdom:StartImportJob"),
            WisdomActions::TagResource => write!(f, "wisdom:TagResource"),
            WisdomActions::UntagResource => write!(f, "wisdom:UntagResource"),
            WisdomActions::UpdateAiAgent => write!(f, "wisdom:UpdateAIAgent"),
            WisdomActions::UpdateAiGuardrail => write!(f, "wisdom:UpdateAIGuardrail"),
            WisdomActions::UpdateAiPrompt => write!(f, "wisdom:UpdateAIPrompt"),
            WisdomActions::UpdateAssistantAiAgent => write!(f, "wisdom:UpdateAssistantAIAgent"),
            WisdomActions::UpdateContent => write!(f, "wisdom:UpdateContent"),
            WisdomActions::UpdateKnowledgeBaseTemplateUri => {
                write!(f, "wisdom:UpdateKnowledgeBaseTemplateUri")
            }
            WisdomActions::UpdateMessageTemplate => write!(f, "wisdom:UpdateMessageTemplate"),
            WisdomActions::UpdateMessageTemplateMetadata => {
                write!(f, "wisdom:UpdateMessageTemplateMetadata")
            }
            WisdomActions::UpdateQuickResponse => write!(f, "wisdom:UpdateQuickResponse"),
            WisdomActions::UpdateSession => write!(f, "wisdom:UpdateSession"),
            WisdomActions::UpdateSessionData => write!(f, "wisdom:UpdateSessionData"),
        }
    }
}
