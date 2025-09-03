// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum QbusinessActions {
    AllowVendedLogDeliveryForResource,
    AssociatePermission,
    BatchDeleteDocument,
    BatchPutDocument,
    CancelSubscription,
    Chat,
    ChatSync,
    CheckDocumentAccess,
    CreateAnonymousWebExperienceUrl,
    CreateApplication,
    CreateChatResponseConfiguration,
    CreateDataAccessor,
    CreateDataAccessorWithTti,
    CreateDataSource,
    CreateIndex,
    CreateIntegration,
    CreatePlugin,
    CreateRetriever,
    CreateSubscription,
    CreateUser,
    CreateWebExperience,
    DeleteApplication,
    DeleteAttachment,
    DeleteChatControlsConfiguration,
    DeleteChatResponseConfiguration,
    DeleteConversation,
    DeleteDataAccessor,
    DeleteDataSource,
    DeleteGroup,
    DeleteIndex,
    DeleteIntegration,
    DeletePlugin,
    DeleteRetriever,
    DeleteUser,
    DeleteWebExperience,
    DisableAclOnDataSource,
    DisassociatePermission,
    GetApplication,
    GetChatControlsConfiguration,
    GetChatResponseConfiguration,
    GetDataAccessor,
    GetDataSource,
    GetDocumentContent,
    GetGroup,
    GetIndex,
    GetIntegration,
    GetMedia,
    GetPlugin,
    GetPolicy,
    GetRetriever,
    GetUser,
    GetWebExperience,
    ListApplications,
    ListAttachments,
    ListChatResponseConfigurations,
    ListConversations,
    ListDataAccessors,
    ListDataSourceSyncJobs,
    ListDataSources,
    ListDocuments,
    ListGroups,
    ListIndices,
    ListIntegrations,
    ListMessages,
    ListPluginActions,
    ListPluginTypeActions,
    ListPluginTypeMetadata,
    ListPlugins,
    ListRetrievers,
    ListSubscriptions,
    ListTagsForResource,
    ListWebExperiences,
    PutFeedback,
    PutGroup,
    PutResourcePolicy,
    SearchRelevantContent,
    StartDataSourceSyncJob,
    StartDeployment,
    StopDataSourceSyncJob,
    TagResource,
    UntagResource,
    UpdateApplication,
    UpdateChatControlsConfiguration,
    UpdateChatResponseConfiguration,
    UpdateDataAccessor,
    UpdateDataSource,
    UpdateIndex,
    UpdateIntegration,
    UpdatePlugin,
    UpdateRetriever,
    UpdateSubscription,
    UpdateUser,
    UpdateWebExperience,
}
impl std::fmt::Display for QbusinessActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QbusinessActions::AllowVendedLogDeliveryForResource => {
                write!(f, "qbusiness:AllowVendedLogDeliveryForResource")
            }
            QbusinessActions::AssociatePermission => write!(f, "qbusiness:AssociatePermission"),
            QbusinessActions::BatchDeleteDocument => write!(f, "qbusiness:BatchDeleteDocument"),
            QbusinessActions::BatchPutDocument => write!(f, "qbusiness:BatchPutDocument"),
            QbusinessActions::CancelSubscription => write!(f, "qbusiness:CancelSubscription"),
            QbusinessActions::Chat => write!(f, "qbusiness:Chat"),
            QbusinessActions::ChatSync => write!(f, "qbusiness:ChatSync"),
            QbusinessActions::CheckDocumentAccess => write!(f, "qbusiness:CheckDocumentAccess"),
            QbusinessActions::CreateAnonymousWebExperienceUrl => {
                write!(f, "qbusiness:CreateAnonymousWebExperienceUrl")
            }
            QbusinessActions::CreateApplication => write!(f, "qbusiness:CreateApplication"),
            QbusinessActions::CreateChatResponseConfiguration => {
                write!(f, "qbusiness:CreateChatResponseConfiguration")
            }
            QbusinessActions::CreateDataAccessor => write!(f, "qbusiness:CreateDataAccessor"),
            QbusinessActions::CreateDataAccessorWithTti => {
                write!(f, "qbusiness:CreateDataAccessorWithTti")
            }
            QbusinessActions::CreateDataSource => write!(f, "qbusiness:CreateDataSource"),
            QbusinessActions::CreateIndex => write!(f, "qbusiness:CreateIndex"),
            QbusinessActions::CreateIntegration => write!(f, "qbusiness:CreateIntegration"),
            QbusinessActions::CreatePlugin => write!(f, "qbusiness:CreatePlugin"),
            QbusinessActions::CreateRetriever => write!(f, "qbusiness:CreateRetriever"),
            QbusinessActions::CreateSubscription => write!(f, "qbusiness:CreateSubscription"),
            QbusinessActions::CreateUser => write!(f, "qbusiness:CreateUser"),
            QbusinessActions::CreateWebExperience => write!(f, "qbusiness:CreateWebExperience"),
            QbusinessActions::DeleteApplication => write!(f, "qbusiness:DeleteApplication"),
            QbusinessActions::DeleteAttachment => write!(f, "qbusiness:DeleteAttachment"),
            QbusinessActions::DeleteChatControlsConfiguration => {
                write!(f, "qbusiness:DeleteChatControlsConfiguration")
            }
            QbusinessActions::DeleteChatResponseConfiguration => {
                write!(f, "qbusiness:DeleteChatResponseConfiguration")
            }
            QbusinessActions::DeleteConversation => write!(f, "qbusiness:DeleteConversation"),
            QbusinessActions::DeleteDataAccessor => write!(f, "qbusiness:DeleteDataAccessor"),
            QbusinessActions::DeleteDataSource => write!(f, "qbusiness:DeleteDataSource"),
            QbusinessActions::DeleteGroup => write!(f, "qbusiness:DeleteGroup"),
            QbusinessActions::DeleteIndex => write!(f, "qbusiness:DeleteIndex"),
            QbusinessActions::DeleteIntegration => write!(f, "qbusiness:DeleteIntegration"),
            QbusinessActions::DeletePlugin => write!(f, "qbusiness:DeletePlugin"),
            QbusinessActions::DeleteRetriever => write!(f, "qbusiness:DeleteRetriever"),
            QbusinessActions::DeleteUser => write!(f, "qbusiness:DeleteUser"),
            QbusinessActions::DeleteWebExperience => write!(f, "qbusiness:DeleteWebExperience"),
            QbusinessActions::DisableAclOnDataSource => {
                write!(f, "qbusiness:DisableAclOnDataSource")
            }
            QbusinessActions::DisassociatePermission => {
                write!(f, "qbusiness:DisassociatePermission")
            }
            QbusinessActions::GetApplication => write!(f, "qbusiness:GetApplication"),
            QbusinessActions::GetChatControlsConfiguration => {
                write!(f, "qbusiness:GetChatControlsConfiguration")
            }
            QbusinessActions::GetChatResponseConfiguration => {
                write!(f, "qbusiness:GetChatResponseConfiguration")
            }
            QbusinessActions::GetDataAccessor => write!(f, "qbusiness:GetDataAccessor"),
            QbusinessActions::GetDataSource => write!(f, "qbusiness:GetDataSource"),
            QbusinessActions::GetDocumentContent => write!(f, "qbusiness:GetDocumentContent"),
            QbusinessActions::GetGroup => write!(f, "qbusiness:GetGroup"),
            QbusinessActions::GetIndex => write!(f, "qbusiness:GetIndex"),
            QbusinessActions::GetIntegration => write!(f, "qbusiness:GetIntegration"),
            QbusinessActions::GetMedia => write!(f, "qbusiness:GetMedia"),
            QbusinessActions::GetPlugin => write!(f, "qbusiness:GetPlugin"),
            QbusinessActions::GetPolicy => write!(f, "qbusiness:GetPolicy"),
            QbusinessActions::GetRetriever => write!(f, "qbusiness:GetRetriever"),
            QbusinessActions::GetUser => write!(f, "qbusiness:GetUser"),
            QbusinessActions::GetWebExperience => write!(f, "qbusiness:GetWebExperience"),
            QbusinessActions::ListApplications => write!(f, "qbusiness:ListApplications"),
            QbusinessActions::ListAttachments => write!(f, "qbusiness:ListAttachments"),
            QbusinessActions::ListChatResponseConfigurations => {
                write!(f, "qbusiness:ListChatResponseConfigurations")
            }
            QbusinessActions::ListConversations => write!(f, "qbusiness:ListConversations"),
            QbusinessActions::ListDataAccessors => write!(f, "qbusiness:ListDataAccessors"),
            QbusinessActions::ListDataSourceSyncJobs => {
                write!(f, "qbusiness:ListDataSourceSyncJobs")
            }
            QbusinessActions::ListDataSources => write!(f, "qbusiness:ListDataSources"),
            QbusinessActions::ListDocuments => write!(f, "qbusiness:ListDocuments"),
            QbusinessActions::ListGroups => write!(f, "qbusiness:ListGroups"),
            QbusinessActions::ListIndices => write!(f, "qbusiness:ListIndices"),
            QbusinessActions::ListIntegrations => write!(f, "qbusiness:ListIntegrations"),
            QbusinessActions::ListMessages => write!(f, "qbusiness:ListMessages"),
            QbusinessActions::ListPluginActions => write!(f, "qbusiness:ListPluginActions"),
            QbusinessActions::ListPluginTypeActions => write!(f, "qbusiness:ListPluginTypeActions"),
            QbusinessActions::ListPluginTypeMetadata => {
                write!(f, "qbusiness:ListPluginTypeMetadata")
            }
            QbusinessActions::ListPlugins => write!(f, "qbusiness:ListPlugins"),
            QbusinessActions::ListRetrievers => write!(f, "qbusiness:ListRetrievers"),
            QbusinessActions::ListSubscriptions => write!(f, "qbusiness:ListSubscriptions"),
            QbusinessActions::ListTagsForResource => write!(f, "qbusiness:ListTagsForResource"),
            QbusinessActions::ListWebExperiences => write!(f, "qbusiness:ListWebExperiences"),
            QbusinessActions::PutFeedback => write!(f, "qbusiness:PutFeedback"),
            QbusinessActions::PutGroup => write!(f, "qbusiness:PutGroup"),
            QbusinessActions::PutResourcePolicy => write!(f, "qbusiness:PutResourcePolicy"),
            QbusinessActions::SearchRelevantContent => write!(f, "qbusiness:SearchRelevantContent"),
            QbusinessActions::StartDataSourceSyncJob => {
                write!(f, "qbusiness:StartDataSourceSyncJob")
            }
            QbusinessActions::StartDeployment => write!(f, "qbusiness:StartDeployment"),
            QbusinessActions::StopDataSourceSyncJob => write!(f, "qbusiness:StopDataSourceSyncJob"),
            QbusinessActions::TagResource => write!(f, "qbusiness:TagResource"),
            QbusinessActions::UntagResource => write!(f, "qbusiness:UntagResource"),
            QbusinessActions::UpdateApplication => write!(f, "qbusiness:UpdateApplication"),
            QbusinessActions::UpdateChatControlsConfiguration => {
                write!(f, "qbusiness:UpdateChatControlsConfiguration")
            }
            QbusinessActions::UpdateChatResponseConfiguration => {
                write!(f, "qbusiness:UpdateChatResponseConfiguration")
            }
            QbusinessActions::UpdateDataAccessor => write!(f, "qbusiness:UpdateDataAccessor"),
            QbusinessActions::UpdateDataSource => write!(f, "qbusiness:UpdateDataSource"),
            QbusinessActions::UpdateIndex => write!(f, "qbusiness:UpdateIndex"),
            QbusinessActions::UpdateIntegration => write!(f, "qbusiness:UpdateIntegration"),
            QbusinessActions::UpdatePlugin => write!(f, "qbusiness:UpdatePlugin"),
            QbusinessActions::UpdateRetriever => write!(f, "qbusiness:UpdateRetriever"),
            QbusinessActions::UpdateSubscription => write!(f, "qbusiness:UpdateSubscription"),
            QbusinessActions::UpdateUser => write!(f, "qbusiness:UpdateUser"),
            QbusinessActions::UpdateWebExperience => write!(f, "qbusiness:UpdateWebExperience"),
        }
    }
}
