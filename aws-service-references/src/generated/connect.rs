// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum ConnectActions {
    ActivateEvaluationForm,
    AdminGetEmergencyAccessToken,
    AssociateAnalyticsDataSet,
    AssociateApprovedOrigin,
    AssociateBot,
    AssociateCustomerProfilesDomain,
    AssociateDefaultVocabulary,
    AssociateFlow,
    AssociateInstanceStorageConfig,
    AssociateLambdaFunction,
    AssociateLexBot,
    AssociatePhoneNumberContactFlow,
    AssociateQueueQuickConnects,
    AssociateRoutingProfileQueues,
    AssociateSecurityKey,
    AssociateTrafficDistributionGroupUser,
    AssociateUserProficiencies,
    BatchAssociateAnalyticsDataSet,
    BatchDisassociateAnalyticsDataSet,
    BatchGetAttachedFileMetadata,
    BatchGetFlowAssociation,
    BatchPutContact,
    ClaimPhoneNumber,
    CompleteAttachedFileUpload,
    CreateAgentStatus,
    CreateAuthenticationProfile,
    CreateContact,
    CreateContactFlow,
    CreateContactFlowModule,
    CreateContactFlowVersion,
    CreateEmailAddress,
    CreateEvaluationForm,
    CreateHoursOfOperation,
    CreateHoursOfOperationOverride,
    CreateInstance,
    CreateIntegrationAssociation,
    CreateParticipant,
    CreatePersistentContactAssociation,
    CreatePredefinedAttribute,
    CreatePrompt,
    CreatePushNotificationRegistration,
    CreateQueue,
    CreateQuickConnect,
    CreateRoutingProfile,
    CreateRule,
    CreateSecurityProfile,
    CreateTaskTemplate,
    CreateTrafficDistributionGroup,
    CreateUseCase,
    CreateUser,
    CreateUserHierarchyGroup,
    CreateView,
    CreateViewVersion,
    CreateVocabulary,
    DeactivateEvaluationForm,
    DeleteAttachedFile,
    DeleteContactEvaluation,
    DeleteContactFlow,
    DeleteContactFlowModule,
    DeleteContactFlowVersion,
    DeleteEmailAddress,
    DeleteEvaluationForm,
    DeleteHoursOfOperation,
    DeleteHoursOfOperationOverride,
    DeleteInstance,
    DeleteIntegrationAssociation,
    DeletePredefinedAttribute,
    DeletePrompt,
    DeletePushNotificationRegistration,
    DeleteQueue,
    DeleteQuickConnect,
    DeleteRoutingProfile,
    DeleteRule,
    DeleteSecurityProfile,
    DeleteTaskTemplate,
    DeleteTrafficDistributionGroup,
    DeleteUseCase,
    DeleteUser,
    DeleteUserHierarchyGroup,
    DeleteView,
    DeleteViewVersion,
    DeleteVocabulary,
    DescribeAgentStatus,
    DescribeAuthenticationProfile,
    DescribeContact,
    DescribeContactEvaluation,
    DescribeContactFlow,
    DescribeContactFlowModule,
    DescribeEmailAddress,
    DescribeEvaluationForm,
    DescribeForecastingPlanningSchedulingIntegration,
    DescribeHoursOfOperation,
    DescribeHoursOfOperationOverride,
    DescribeInstance,
    DescribeInstanceAttribute,
    DescribeInstanceStorageConfig,
    DescribePhoneNumber,
    DescribePredefinedAttribute,
    DescribePrompt,
    DescribeQueue,
    DescribeQuickConnect,
    DescribeRoutingProfile,
    DescribeRule,
    DescribeSecurityProfile,
    DescribeTrafficDistributionGroup,
    DescribeUser,
    DescribeUserHierarchyGroup,
    DescribeUserHierarchyStructure,
    DescribeView,
    DescribeVocabulary,
    DisassociateAnalyticsDataSet,
    DisassociateApprovedOrigin,
    DisassociateBot,
    DisassociateCustomerProfilesDomain,
    DisassociateFlow,
    DisassociateInstanceStorageConfig,
    DisassociateLambdaFunction,
    DisassociateLexBot,
    DisassociatePhoneNumberContactFlow,
    DisassociateQueueQuickConnects,
    DisassociateRoutingProfileQueues,
    DisassociateSecurityKey,
    DisassociateTrafficDistributionGroupUser,
    DisassociateUserProficiencies,
    DismissUserContact,
    GetAttachedFile,
    GetContactAttributes,
    GetContactMetrics,
    GetCurrentMetricData,
    GetCurrentUserData,
    GetEffectiveHoursOfOperations,
    GetFederationToken,
    GetFlowAssociation,
    GetMetricData,
    GetMetricDataV2,
    GetPromptFile,
    GetTaskTemplate,
    GetTrafficDistribution,
    ImportPhoneNumber,
    ListAgentStatuses,
    ListAnalyticsDataAssociations,
    ListAnalyticsDataLakeDataSets,
    ListApprovedOrigins,
    ListAssociatedContacts,
    ListAuthenticationProfiles,
    ListBots,
    ListContactEvaluations,
    ListContactFlowModules,
    ListContactFlowVersions,
    ListContactFlows,
    ListContactReferences,
    ListDefaultVocabularies,
    ListEvaluationFormVersions,
    ListEvaluationForms,
    ListFlowAssociations,
    ListHoursOfOperationOverrides,
    ListHoursOfOperations,
    ListInstanceAttributes,
    ListInstanceStorageConfigs,
    ListInstances,
    ListIntegrationAssociations,
    ListLambdaFunctions,
    ListLexBots,
    ListPhoneNumbers,
    ListPhoneNumbersV2,
    ListPredefinedAttributes,
    ListPrompts,
    ListQueueQuickConnects,
    ListQueues,
    ListQuickConnects,
    ListRealtimeContactAnalysisSegments,
    ListRealtimeContactAnalysisSegmentsV2,
    ListRoutingProfileQueues,
    ListRoutingProfiles,
    ListRules,
    ListSecurityKeys,
    ListSecurityProfileApplications,
    ListSecurityProfilePermissions,
    ListSecurityProfiles,
    ListTagsForResource,
    ListTaskTemplates,
    ListTrafficDistributionGroupUsers,
    ListTrafficDistributionGroups,
    ListUseCases,
    ListUserHierarchyGroups,
    ListUserProficiencies,
    ListUsers,
    ListViewVersions,
    ListViews,
    MonitorContact,
    PauseContact,
    PutUserStatus,
    ReleasePhoneNumber,
    ReplicateInstance,
    ResumeContact,
    ResumeContactRecording,
    SearchAgentStatuses,
    SearchAvailablePhoneNumbers,
    SearchContactFlowModules,
    SearchContactFlows,
    SearchContacts,
    SearchEmailAddresses,
    SearchHoursOfOperationOverrides,
    SearchHoursOfOperations,
    SearchPredefinedAttributes,
    SearchPrompts,
    SearchQueues,
    SearchQuickConnects,
    SearchResourceTags,
    SearchRoutingProfiles,
    SearchSecurityProfiles,
    SearchUserHierarchyGroups,
    SearchUsers,
    SearchVocabularies,
    SendChatIntegrationEvent,
    SendIntegrationEvent,
    SendOutboundEmail,
    StartAttachedFileUpload,
    StartChatContact,
    StartContactEvaluation,
    StartContactRecording,
    StartContactStreaming,
    StartEmailContact,
    StartForecastingPlanningSchedulingIntegration,
    StartOutboundChatContact,
    StartOutboundEmailContact,
    StartOutboundVoiceContact,
    StartScreenSharing,
    StartTaskContact,
    StartWebRtcContact,
    StopContact,
    StopContactRecording,
    StopContactStreaming,
    StopForecastingPlanningSchedulingIntegration,
    SubmitContactEvaluation,
    SuspendContactRecording,
    TagContact,
    TagResource,
    TransferContact,
    UntagContact,
    UntagResource,
    UpdateAgentStatus,
    UpdateAuthenticationProfile,
    UpdateContact,
    UpdateContactAttributes,
    UpdateContactEvaluation,
    UpdateContactFlowContent,
    UpdateContactFlowMetadata,
    UpdateContactFlowModuleContent,
    UpdateContactFlowModuleMetadata,
    UpdateContactFlowName,
    UpdateContactRoutingData,
    UpdateContactSchedule,
    UpdateEmailAddressMetadata,
    UpdateEvaluationForm,
    UpdateHoursOfOperation,
    UpdateHoursOfOperationOverride,
    UpdateInstanceAttribute,
    UpdateInstanceStorageConfig,
    UpdateParticipantAuthentication,
    UpdateParticipantRoleConfig,
    UpdatePhoneNumber,
    UpdatePhoneNumberMetadata,
    UpdatePredefinedAttribute,
    UpdatePrompt,
    UpdateQueueHoursOfOperation,
    UpdateQueueMaxContacts,
    UpdateQueueName,
    UpdateQueueOutboundCallerConfig,
    UpdateQueueOutboundEmailConfig,
    UpdateQueueStatus,
    UpdateQuickConnectConfig,
    UpdateQuickConnectName,
    UpdateRoutingProfileAgentAvailabilityTimer,
    UpdateRoutingProfileConcurrency,
    UpdateRoutingProfileDefaultOutboundQueue,
    UpdateRoutingProfileName,
    UpdateRoutingProfileQueues,
    UpdateRule,
    UpdateSecurityProfile,
    UpdateTaskTemplate,
    UpdateTrafficDistribution,
    UpdateUserHierarchy,
    UpdateUserHierarchyGroupName,
    UpdateUserHierarchyStructure,
    UpdateUserIdentityInfo,
    UpdateUserPhoneConfig,
    UpdateUserProficiencies,
    UpdateUserRoutingProfile,
    UpdateUserSecurityProfiles,
    UpdateViewContent,
    UpdateViewMetadata,
}
impl std::fmt::Display for ConnectActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConnectActions::ActivateEvaluationForm => write!(f, "connect:ActivateEvaluationForm"),
            ConnectActions::AdminGetEmergencyAccessToken => {
                write!(f, "connect:AdminGetEmergencyAccessToken")
            }
            ConnectActions::AssociateAnalyticsDataSet => {
                write!(f, "connect:AssociateAnalyticsDataSet")
            }
            ConnectActions::AssociateApprovedOrigin => write!(f, "connect:AssociateApprovedOrigin"),
            ConnectActions::AssociateBot => write!(f, "connect:AssociateBot"),
            ConnectActions::AssociateCustomerProfilesDomain => {
                write!(f, "connect:AssociateCustomerProfilesDomain")
            }
            ConnectActions::AssociateDefaultVocabulary => {
                write!(f, "connect:AssociateDefaultVocabulary")
            }
            ConnectActions::AssociateFlow => write!(f, "connect:AssociateFlow"),
            ConnectActions::AssociateInstanceStorageConfig => {
                write!(f, "connect:AssociateInstanceStorageConfig")
            }
            ConnectActions::AssociateLambdaFunction => write!(f, "connect:AssociateLambdaFunction"),
            ConnectActions::AssociateLexBot => write!(f, "connect:AssociateLexBot"),
            ConnectActions::AssociatePhoneNumberContactFlow => {
                write!(f, "connect:AssociatePhoneNumberContactFlow")
            }
            ConnectActions::AssociateQueueQuickConnects => {
                write!(f, "connect:AssociateQueueQuickConnects")
            }
            ConnectActions::AssociateRoutingProfileQueues => {
                write!(f, "connect:AssociateRoutingProfileQueues")
            }
            ConnectActions::AssociateSecurityKey => write!(f, "connect:AssociateSecurityKey"),
            ConnectActions::AssociateTrafficDistributionGroupUser => {
                write!(f, "connect:AssociateTrafficDistributionGroupUser")
            }
            ConnectActions::AssociateUserProficiencies => {
                write!(f, "connect:AssociateUserProficiencies")
            }
            ConnectActions::BatchAssociateAnalyticsDataSet => {
                write!(f, "connect:BatchAssociateAnalyticsDataSet")
            }
            ConnectActions::BatchDisassociateAnalyticsDataSet => {
                write!(f, "connect:BatchDisassociateAnalyticsDataSet")
            }
            ConnectActions::BatchGetAttachedFileMetadata => {
                write!(f, "connect:BatchGetAttachedFileMetadata")
            }
            ConnectActions::BatchGetFlowAssociation => write!(f, "connect:BatchGetFlowAssociation"),
            ConnectActions::BatchPutContact => write!(f, "connect:BatchPutContact"),
            ConnectActions::ClaimPhoneNumber => write!(f, "connect:ClaimPhoneNumber"),
            ConnectActions::CompleteAttachedFileUpload => {
                write!(f, "connect:CompleteAttachedFileUpload")
            }
            ConnectActions::CreateAgentStatus => write!(f, "connect:CreateAgentStatus"),
            ConnectActions::CreateAuthenticationProfile => {
                write!(f, "connect:CreateAuthenticationProfile")
            }
            ConnectActions::CreateContact => write!(f, "connect:CreateContact"),
            ConnectActions::CreateContactFlow => write!(f, "connect:CreateContactFlow"),
            ConnectActions::CreateContactFlowModule => write!(f, "connect:CreateContactFlowModule"),
            ConnectActions::CreateContactFlowVersion => {
                write!(f, "connect:CreateContactFlowVersion")
            }
            ConnectActions::CreateEmailAddress => write!(f, "connect:CreateEmailAddress"),
            ConnectActions::CreateEvaluationForm => write!(f, "connect:CreateEvaluationForm"),
            ConnectActions::CreateHoursOfOperation => write!(f, "connect:CreateHoursOfOperation"),
            ConnectActions::CreateHoursOfOperationOverride => {
                write!(f, "connect:CreateHoursOfOperationOverride")
            }
            ConnectActions::CreateInstance => write!(f, "connect:CreateInstance"),
            ConnectActions::CreateIntegrationAssociation => {
                write!(f, "connect:CreateIntegrationAssociation")
            }
            ConnectActions::CreateParticipant => write!(f, "connect:CreateParticipant"),
            ConnectActions::CreatePersistentContactAssociation => {
                write!(f, "connect:CreatePersistentContactAssociation")
            }
            ConnectActions::CreatePredefinedAttribute => {
                write!(f, "connect:CreatePredefinedAttribute")
            }
            ConnectActions::CreatePrompt => write!(f, "connect:CreatePrompt"),
            ConnectActions::CreatePushNotificationRegistration => {
                write!(f, "connect:CreatePushNotificationRegistration")
            }
            ConnectActions::CreateQueue => write!(f, "connect:CreateQueue"),
            ConnectActions::CreateQuickConnect => write!(f, "connect:CreateQuickConnect"),
            ConnectActions::CreateRoutingProfile => write!(f, "connect:CreateRoutingProfile"),
            ConnectActions::CreateRule => write!(f, "connect:CreateRule"),
            ConnectActions::CreateSecurityProfile => write!(f, "connect:CreateSecurityProfile"),
            ConnectActions::CreateTaskTemplate => write!(f, "connect:CreateTaskTemplate"),
            ConnectActions::CreateTrafficDistributionGroup => {
                write!(f, "connect:CreateTrafficDistributionGroup")
            }
            ConnectActions::CreateUseCase => write!(f, "connect:CreateUseCase"),
            ConnectActions::CreateUser => write!(f, "connect:CreateUser"),
            ConnectActions::CreateUserHierarchyGroup => {
                write!(f, "connect:CreateUserHierarchyGroup")
            }
            ConnectActions::CreateView => write!(f, "connect:CreateView"),
            ConnectActions::CreateViewVersion => write!(f, "connect:CreateViewVersion"),
            ConnectActions::CreateVocabulary => write!(f, "connect:CreateVocabulary"),
            ConnectActions::DeactivateEvaluationForm => {
                write!(f, "connect:DeactivateEvaluationForm")
            }
            ConnectActions::DeleteAttachedFile => write!(f, "connect:DeleteAttachedFile"),
            ConnectActions::DeleteContactEvaluation => write!(f, "connect:DeleteContactEvaluation"),
            ConnectActions::DeleteContactFlow => write!(f, "connect:DeleteContactFlow"),
            ConnectActions::DeleteContactFlowModule => write!(f, "connect:DeleteContactFlowModule"),
            ConnectActions::DeleteContactFlowVersion => {
                write!(f, "connect:DeleteContactFlowVersion")
            }
            ConnectActions::DeleteEmailAddress => write!(f, "connect:DeleteEmailAddress"),
            ConnectActions::DeleteEvaluationForm => write!(f, "connect:DeleteEvaluationForm"),
            ConnectActions::DeleteHoursOfOperation => write!(f, "connect:DeleteHoursOfOperation"),
            ConnectActions::DeleteHoursOfOperationOverride => {
                write!(f, "connect:DeleteHoursOfOperationOverride")
            }
            ConnectActions::DeleteInstance => write!(f, "connect:DeleteInstance"),
            ConnectActions::DeleteIntegrationAssociation => {
                write!(f, "connect:DeleteIntegrationAssociation")
            }
            ConnectActions::DeletePredefinedAttribute => {
                write!(f, "connect:DeletePredefinedAttribute")
            }
            ConnectActions::DeletePrompt => write!(f, "connect:DeletePrompt"),
            ConnectActions::DeletePushNotificationRegistration => {
                write!(f, "connect:DeletePushNotificationRegistration")
            }
            ConnectActions::DeleteQueue => write!(f, "connect:DeleteQueue"),
            ConnectActions::DeleteQuickConnect => write!(f, "connect:DeleteQuickConnect"),
            ConnectActions::DeleteRoutingProfile => write!(f, "connect:DeleteRoutingProfile"),
            ConnectActions::DeleteRule => write!(f, "connect:DeleteRule"),
            ConnectActions::DeleteSecurityProfile => write!(f, "connect:DeleteSecurityProfile"),
            ConnectActions::DeleteTaskTemplate => write!(f, "connect:DeleteTaskTemplate"),
            ConnectActions::DeleteTrafficDistributionGroup => {
                write!(f, "connect:DeleteTrafficDistributionGroup")
            }
            ConnectActions::DeleteUseCase => write!(f, "connect:DeleteUseCase"),
            ConnectActions::DeleteUser => write!(f, "connect:DeleteUser"),
            ConnectActions::DeleteUserHierarchyGroup => {
                write!(f, "connect:DeleteUserHierarchyGroup")
            }
            ConnectActions::DeleteView => write!(f, "connect:DeleteView"),
            ConnectActions::DeleteViewVersion => write!(f, "connect:DeleteViewVersion"),
            ConnectActions::DeleteVocabulary => write!(f, "connect:DeleteVocabulary"),
            ConnectActions::DescribeAgentStatus => write!(f, "connect:DescribeAgentStatus"),
            ConnectActions::DescribeAuthenticationProfile => {
                write!(f, "connect:DescribeAuthenticationProfile")
            }
            ConnectActions::DescribeContact => write!(f, "connect:DescribeContact"),
            ConnectActions::DescribeContactEvaluation => {
                write!(f, "connect:DescribeContactEvaluation")
            }
            ConnectActions::DescribeContactFlow => write!(f, "connect:DescribeContactFlow"),
            ConnectActions::DescribeContactFlowModule => {
                write!(f, "connect:DescribeContactFlowModule")
            }
            ConnectActions::DescribeEmailAddress => write!(f, "connect:DescribeEmailAddress"),
            ConnectActions::DescribeEvaluationForm => write!(f, "connect:DescribeEvaluationForm"),
            ConnectActions::DescribeForecastingPlanningSchedulingIntegration => write!(
                f,
                "connect:DescribeForecastingPlanningSchedulingIntegration"
            ),
            ConnectActions::DescribeHoursOfOperation => {
                write!(f, "connect:DescribeHoursOfOperation")
            }
            ConnectActions::DescribeHoursOfOperationOverride => {
                write!(f, "connect:DescribeHoursOfOperationOverride")
            }
            ConnectActions::DescribeInstance => write!(f, "connect:DescribeInstance"),
            ConnectActions::DescribeInstanceAttribute => {
                write!(f, "connect:DescribeInstanceAttribute")
            }
            ConnectActions::DescribeInstanceStorageConfig => {
                write!(f, "connect:DescribeInstanceStorageConfig")
            }
            ConnectActions::DescribePhoneNumber => write!(f, "connect:DescribePhoneNumber"),
            ConnectActions::DescribePredefinedAttribute => {
                write!(f, "connect:DescribePredefinedAttribute")
            }
            ConnectActions::DescribePrompt => write!(f, "connect:DescribePrompt"),
            ConnectActions::DescribeQueue => write!(f, "connect:DescribeQueue"),
            ConnectActions::DescribeQuickConnect => write!(f, "connect:DescribeQuickConnect"),
            ConnectActions::DescribeRoutingProfile => write!(f, "connect:DescribeRoutingProfile"),
            ConnectActions::DescribeRule => write!(f, "connect:DescribeRule"),
            ConnectActions::DescribeSecurityProfile => write!(f, "connect:DescribeSecurityProfile"),
            ConnectActions::DescribeTrafficDistributionGroup => {
                write!(f, "connect:DescribeTrafficDistributionGroup")
            }
            ConnectActions::DescribeUser => write!(f, "connect:DescribeUser"),
            ConnectActions::DescribeUserHierarchyGroup => {
                write!(f, "connect:DescribeUserHierarchyGroup")
            }
            ConnectActions::DescribeUserHierarchyStructure => {
                write!(f, "connect:DescribeUserHierarchyStructure")
            }
            ConnectActions::DescribeView => write!(f, "connect:DescribeView"),
            ConnectActions::DescribeVocabulary => write!(f, "connect:DescribeVocabulary"),
            ConnectActions::DisassociateAnalyticsDataSet => {
                write!(f, "connect:DisassociateAnalyticsDataSet")
            }
            ConnectActions::DisassociateApprovedOrigin => {
                write!(f, "connect:DisassociateApprovedOrigin")
            }
            ConnectActions::DisassociateBot => write!(f, "connect:DisassociateBot"),
            ConnectActions::DisassociateCustomerProfilesDomain => {
                write!(f, "connect:DisassociateCustomerProfilesDomain")
            }
            ConnectActions::DisassociateFlow => write!(f, "connect:DisassociateFlow"),
            ConnectActions::DisassociateInstanceStorageConfig => {
                write!(f, "connect:DisassociateInstanceStorageConfig")
            }
            ConnectActions::DisassociateLambdaFunction => {
                write!(f, "connect:DisassociateLambdaFunction")
            }
            ConnectActions::DisassociateLexBot => write!(f, "connect:DisassociateLexBot"),
            ConnectActions::DisassociatePhoneNumberContactFlow => {
                write!(f, "connect:DisassociatePhoneNumberContactFlow")
            }
            ConnectActions::DisassociateQueueQuickConnects => {
                write!(f, "connect:DisassociateQueueQuickConnects")
            }
            ConnectActions::DisassociateRoutingProfileQueues => {
                write!(f, "connect:DisassociateRoutingProfileQueues")
            }
            ConnectActions::DisassociateSecurityKey => write!(f, "connect:DisassociateSecurityKey"),
            ConnectActions::DisassociateTrafficDistributionGroupUser => {
                write!(f, "connect:DisassociateTrafficDistributionGroupUser")
            }
            ConnectActions::DisassociateUserProficiencies => {
                write!(f, "connect:DisassociateUserProficiencies")
            }
            ConnectActions::DismissUserContact => write!(f, "connect:DismissUserContact"),
            ConnectActions::GetAttachedFile => write!(f, "connect:GetAttachedFile"),
            ConnectActions::GetContactAttributes => write!(f, "connect:GetContactAttributes"),
            ConnectActions::GetContactMetrics => write!(f, "connect:GetContactMetrics"),
            ConnectActions::GetCurrentMetricData => write!(f, "connect:GetCurrentMetricData"),
            ConnectActions::GetCurrentUserData => write!(f, "connect:GetCurrentUserData"),
            ConnectActions::GetEffectiveHoursOfOperations => {
                write!(f, "connect:GetEffectiveHoursOfOperations")
            }
            ConnectActions::GetFederationToken => write!(f, "connect:GetFederationToken"),
            ConnectActions::GetFlowAssociation => write!(f, "connect:GetFlowAssociation"),
            ConnectActions::GetMetricData => write!(f, "connect:GetMetricData"),
            ConnectActions::GetMetricDataV2 => write!(f, "connect:GetMetricDataV2"),
            ConnectActions::GetPromptFile => write!(f, "connect:GetPromptFile"),
            ConnectActions::GetTaskTemplate => write!(f, "connect:GetTaskTemplate"),
            ConnectActions::GetTrafficDistribution => write!(f, "connect:GetTrafficDistribution"),
            ConnectActions::ImportPhoneNumber => write!(f, "connect:ImportPhoneNumber"),
            ConnectActions::ListAgentStatuses => write!(f, "connect:ListAgentStatuses"),
            ConnectActions::ListAnalyticsDataAssociations => {
                write!(f, "connect:ListAnalyticsDataAssociations")
            }
            ConnectActions::ListAnalyticsDataLakeDataSets => {
                write!(f, "connect:ListAnalyticsDataLakeDataSets")
            }
            ConnectActions::ListApprovedOrigins => write!(f, "connect:ListApprovedOrigins"),
            ConnectActions::ListAssociatedContacts => write!(f, "connect:ListAssociatedContacts"),
            ConnectActions::ListAuthenticationProfiles => {
                write!(f, "connect:ListAuthenticationProfiles")
            }
            ConnectActions::ListBots => write!(f, "connect:ListBots"),
            ConnectActions::ListContactEvaluations => write!(f, "connect:ListContactEvaluations"),
            ConnectActions::ListContactFlowModules => write!(f, "connect:ListContactFlowModules"),
            ConnectActions::ListContactFlowVersions => write!(f, "connect:ListContactFlowVersions"),
            ConnectActions::ListContactFlows => write!(f, "connect:ListContactFlows"),
            ConnectActions::ListContactReferences => write!(f, "connect:ListContactReferences"),
            ConnectActions::ListDefaultVocabularies => write!(f, "connect:ListDefaultVocabularies"),
            ConnectActions::ListEvaluationFormVersions => {
                write!(f, "connect:ListEvaluationFormVersions")
            }
            ConnectActions::ListEvaluationForms => write!(f, "connect:ListEvaluationForms"),
            ConnectActions::ListFlowAssociations => write!(f, "connect:ListFlowAssociations"),
            ConnectActions::ListHoursOfOperationOverrides => {
                write!(f, "connect:ListHoursOfOperationOverrides")
            }
            ConnectActions::ListHoursOfOperations => write!(f, "connect:ListHoursOfOperations"),
            ConnectActions::ListInstanceAttributes => write!(f, "connect:ListInstanceAttributes"),
            ConnectActions::ListInstanceStorageConfigs => {
                write!(f, "connect:ListInstanceStorageConfigs")
            }
            ConnectActions::ListInstances => write!(f, "connect:ListInstances"),
            ConnectActions::ListIntegrationAssociations => {
                write!(f, "connect:ListIntegrationAssociations")
            }
            ConnectActions::ListLambdaFunctions => write!(f, "connect:ListLambdaFunctions"),
            ConnectActions::ListLexBots => write!(f, "connect:ListLexBots"),
            ConnectActions::ListPhoneNumbers => write!(f, "connect:ListPhoneNumbers"),
            ConnectActions::ListPhoneNumbersV2 => write!(f, "connect:ListPhoneNumbersV2"),
            ConnectActions::ListPredefinedAttributes => {
                write!(f, "connect:ListPredefinedAttributes")
            }
            ConnectActions::ListPrompts => write!(f, "connect:ListPrompts"),
            ConnectActions::ListQueueQuickConnects => write!(f, "connect:ListQueueQuickConnects"),
            ConnectActions::ListQueues => write!(f, "connect:ListQueues"),
            ConnectActions::ListQuickConnects => write!(f, "connect:ListQuickConnects"),
            ConnectActions::ListRealtimeContactAnalysisSegments => {
                write!(f, "connect:ListRealtimeContactAnalysisSegments")
            }
            ConnectActions::ListRealtimeContactAnalysisSegmentsV2 => {
                write!(f, "connect:ListRealtimeContactAnalysisSegmentsV2")
            }
            ConnectActions::ListRoutingProfileQueues => {
                write!(f, "connect:ListRoutingProfileQueues")
            }
            ConnectActions::ListRoutingProfiles => write!(f, "connect:ListRoutingProfiles"),
            ConnectActions::ListRules => write!(f, "connect:ListRules"),
            ConnectActions::ListSecurityKeys => write!(f, "connect:ListSecurityKeys"),
            ConnectActions::ListSecurityProfileApplications => {
                write!(f, "connect:ListSecurityProfileApplications")
            }
            ConnectActions::ListSecurityProfilePermissions => {
                write!(f, "connect:ListSecurityProfilePermissions")
            }
            ConnectActions::ListSecurityProfiles => write!(f, "connect:ListSecurityProfiles"),
            ConnectActions::ListTagsForResource => write!(f, "connect:ListTagsForResource"),
            ConnectActions::ListTaskTemplates => write!(f, "connect:ListTaskTemplates"),
            ConnectActions::ListTrafficDistributionGroupUsers => {
                write!(f, "connect:ListTrafficDistributionGroupUsers")
            }
            ConnectActions::ListTrafficDistributionGroups => {
                write!(f, "connect:ListTrafficDistributionGroups")
            }
            ConnectActions::ListUseCases => write!(f, "connect:ListUseCases"),
            ConnectActions::ListUserHierarchyGroups => write!(f, "connect:ListUserHierarchyGroups"),
            ConnectActions::ListUserProficiencies => write!(f, "connect:ListUserProficiencies"),
            ConnectActions::ListUsers => write!(f, "connect:ListUsers"),
            ConnectActions::ListViewVersions => write!(f, "connect:ListViewVersions"),
            ConnectActions::ListViews => write!(f, "connect:ListViews"),
            ConnectActions::MonitorContact => write!(f, "connect:MonitorContact"),
            ConnectActions::PauseContact => write!(f, "connect:PauseContact"),
            ConnectActions::PutUserStatus => write!(f, "connect:PutUserStatus"),
            ConnectActions::ReleasePhoneNumber => write!(f, "connect:ReleasePhoneNumber"),
            ConnectActions::ReplicateInstance => write!(f, "connect:ReplicateInstance"),
            ConnectActions::ResumeContact => write!(f, "connect:ResumeContact"),
            ConnectActions::ResumeContactRecording => write!(f, "connect:ResumeContactRecording"),
            ConnectActions::SearchAgentStatuses => write!(f, "connect:SearchAgentStatuses"),
            ConnectActions::SearchAvailablePhoneNumbers => {
                write!(f, "connect:SearchAvailablePhoneNumbers")
            }
            ConnectActions::SearchContactFlowModules => {
                write!(f, "connect:SearchContactFlowModules")
            }
            ConnectActions::SearchContactFlows => write!(f, "connect:SearchContactFlows"),
            ConnectActions::SearchContacts => write!(f, "connect:SearchContacts"),
            ConnectActions::SearchEmailAddresses => write!(f, "connect:SearchEmailAddresses"),
            ConnectActions::SearchHoursOfOperationOverrides => {
                write!(f, "connect:SearchHoursOfOperationOverrides")
            }
            ConnectActions::SearchHoursOfOperations => write!(f, "connect:SearchHoursOfOperations"),
            ConnectActions::SearchPredefinedAttributes => {
                write!(f, "connect:SearchPredefinedAttributes")
            }
            ConnectActions::SearchPrompts => write!(f, "connect:SearchPrompts"),
            ConnectActions::SearchQueues => write!(f, "connect:SearchQueues"),
            ConnectActions::SearchQuickConnects => write!(f, "connect:SearchQuickConnects"),
            ConnectActions::SearchResourceTags => write!(f, "connect:SearchResourceTags"),
            ConnectActions::SearchRoutingProfiles => write!(f, "connect:SearchRoutingProfiles"),
            ConnectActions::SearchSecurityProfiles => write!(f, "connect:SearchSecurityProfiles"),
            ConnectActions::SearchUserHierarchyGroups => {
                write!(f, "connect:SearchUserHierarchyGroups")
            }
            ConnectActions::SearchUsers => write!(f, "connect:SearchUsers"),
            ConnectActions::SearchVocabularies => write!(f, "connect:SearchVocabularies"),
            ConnectActions::SendChatIntegrationEvent => {
                write!(f, "connect:SendChatIntegrationEvent")
            }
            ConnectActions::SendIntegrationEvent => write!(f, "connect:SendIntegrationEvent"),
            ConnectActions::SendOutboundEmail => write!(f, "connect:SendOutboundEmail"),
            ConnectActions::StartAttachedFileUpload => write!(f, "connect:StartAttachedFileUpload"),
            ConnectActions::StartChatContact => write!(f, "connect:StartChatContact"),
            ConnectActions::StartContactEvaluation => write!(f, "connect:StartContactEvaluation"),
            ConnectActions::StartContactRecording => write!(f, "connect:StartContactRecording"),
            ConnectActions::StartContactStreaming => write!(f, "connect:StartContactStreaming"),
            ConnectActions::StartEmailContact => write!(f, "connect:StartEmailContact"),
            ConnectActions::StartForecastingPlanningSchedulingIntegration => {
                write!(f, "connect:StartForecastingPlanningSchedulingIntegration")
            }
            ConnectActions::StartOutboundChatContact => {
                write!(f, "connect:StartOutboundChatContact")
            }
            ConnectActions::StartOutboundEmailContact => {
                write!(f, "connect:StartOutboundEmailContact")
            }
            ConnectActions::StartOutboundVoiceContact => {
                write!(f, "connect:StartOutboundVoiceContact")
            }
            ConnectActions::StartScreenSharing => write!(f, "connect:StartScreenSharing"),
            ConnectActions::StartTaskContact => write!(f, "connect:StartTaskContact"),
            ConnectActions::StartWebRtcContact => write!(f, "connect:StartWebRTCContact"),
            ConnectActions::StopContact => write!(f, "connect:StopContact"),
            ConnectActions::StopContactRecording => write!(f, "connect:StopContactRecording"),
            ConnectActions::StopContactStreaming => write!(f, "connect:StopContactStreaming"),
            ConnectActions::StopForecastingPlanningSchedulingIntegration => {
                write!(f, "connect:StopForecastingPlanningSchedulingIntegration")
            }
            ConnectActions::SubmitContactEvaluation => write!(f, "connect:SubmitContactEvaluation"),
            ConnectActions::SuspendContactRecording => write!(f, "connect:SuspendContactRecording"),
            ConnectActions::TagContact => write!(f, "connect:TagContact"),
            ConnectActions::TagResource => write!(f, "connect:TagResource"),
            ConnectActions::TransferContact => write!(f, "connect:TransferContact"),
            ConnectActions::UntagContact => write!(f, "connect:UntagContact"),
            ConnectActions::UntagResource => write!(f, "connect:UntagResource"),
            ConnectActions::UpdateAgentStatus => write!(f, "connect:UpdateAgentStatus"),
            ConnectActions::UpdateAuthenticationProfile => {
                write!(f, "connect:UpdateAuthenticationProfile")
            }
            ConnectActions::UpdateContact => write!(f, "connect:UpdateContact"),
            ConnectActions::UpdateContactAttributes => write!(f, "connect:UpdateContactAttributes"),
            ConnectActions::UpdateContactEvaluation => write!(f, "connect:UpdateContactEvaluation"),
            ConnectActions::UpdateContactFlowContent => {
                write!(f, "connect:UpdateContactFlowContent")
            }
            ConnectActions::UpdateContactFlowMetadata => {
                write!(f, "connect:UpdateContactFlowMetadata")
            }
            ConnectActions::UpdateContactFlowModuleContent => {
                write!(f, "connect:UpdateContactFlowModuleContent")
            }
            ConnectActions::UpdateContactFlowModuleMetadata => {
                write!(f, "connect:UpdateContactFlowModuleMetadata")
            }
            ConnectActions::UpdateContactFlowName => write!(f, "connect:UpdateContactFlowName"),
            ConnectActions::UpdateContactRoutingData => {
                write!(f, "connect:UpdateContactRoutingData")
            }
            ConnectActions::UpdateContactSchedule => write!(f, "connect:UpdateContactSchedule"),
            ConnectActions::UpdateEmailAddressMetadata => {
                write!(f, "connect:UpdateEmailAddressMetadata")
            }
            ConnectActions::UpdateEvaluationForm => write!(f, "connect:UpdateEvaluationForm"),
            ConnectActions::UpdateHoursOfOperation => write!(f, "connect:UpdateHoursOfOperation"),
            ConnectActions::UpdateHoursOfOperationOverride => {
                write!(f, "connect:UpdateHoursOfOperationOverride")
            }
            ConnectActions::UpdateInstanceAttribute => write!(f, "connect:UpdateInstanceAttribute"),
            ConnectActions::UpdateInstanceStorageConfig => {
                write!(f, "connect:UpdateInstanceStorageConfig")
            }
            ConnectActions::UpdateParticipantAuthentication => {
                write!(f, "connect:UpdateParticipantAuthentication")
            }
            ConnectActions::UpdateParticipantRoleConfig => {
                write!(f, "connect:UpdateParticipantRoleConfig")
            }
            ConnectActions::UpdatePhoneNumber => write!(f, "connect:UpdatePhoneNumber"),
            ConnectActions::UpdatePhoneNumberMetadata => {
                write!(f, "connect:UpdatePhoneNumberMetadata")
            }
            ConnectActions::UpdatePredefinedAttribute => {
                write!(f, "connect:UpdatePredefinedAttribute")
            }
            ConnectActions::UpdatePrompt => write!(f, "connect:UpdatePrompt"),
            ConnectActions::UpdateQueueHoursOfOperation => {
                write!(f, "connect:UpdateQueueHoursOfOperation")
            }
            ConnectActions::UpdateQueueMaxContacts => write!(f, "connect:UpdateQueueMaxContacts"),
            ConnectActions::UpdateQueueName => write!(f, "connect:UpdateQueueName"),
            ConnectActions::UpdateQueueOutboundCallerConfig => {
                write!(f, "connect:UpdateQueueOutboundCallerConfig")
            }
            ConnectActions::UpdateQueueOutboundEmailConfig => {
                write!(f, "connect:UpdateQueueOutboundEmailConfig")
            }
            ConnectActions::UpdateQueueStatus => write!(f, "connect:UpdateQueueStatus"),
            ConnectActions::UpdateQuickConnectConfig => {
                write!(f, "connect:UpdateQuickConnectConfig")
            }
            ConnectActions::UpdateQuickConnectName => write!(f, "connect:UpdateQuickConnectName"),
            ConnectActions::UpdateRoutingProfileAgentAvailabilityTimer => {
                write!(f, "connect:UpdateRoutingProfileAgentAvailabilityTimer")
            }
            ConnectActions::UpdateRoutingProfileConcurrency => {
                write!(f, "connect:UpdateRoutingProfileConcurrency")
            }
            ConnectActions::UpdateRoutingProfileDefaultOutboundQueue => {
                write!(f, "connect:UpdateRoutingProfileDefaultOutboundQueue")
            }
            ConnectActions::UpdateRoutingProfileName => {
                write!(f, "connect:UpdateRoutingProfileName")
            }
            ConnectActions::UpdateRoutingProfileQueues => {
                write!(f, "connect:UpdateRoutingProfileQueues")
            }
            ConnectActions::UpdateRule => write!(f, "connect:UpdateRule"),
            ConnectActions::UpdateSecurityProfile => write!(f, "connect:UpdateSecurityProfile"),
            ConnectActions::UpdateTaskTemplate => write!(f, "connect:UpdateTaskTemplate"),
            ConnectActions::UpdateTrafficDistribution => {
                write!(f, "connect:UpdateTrafficDistribution")
            }
            ConnectActions::UpdateUserHierarchy => write!(f, "connect:UpdateUserHierarchy"),
            ConnectActions::UpdateUserHierarchyGroupName => {
                write!(f, "connect:UpdateUserHierarchyGroupName")
            }
            ConnectActions::UpdateUserHierarchyStructure => {
                write!(f, "connect:UpdateUserHierarchyStructure")
            }
            ConnectActions::UpdateUserIdentityInfo => write!(f, "connect:UpdateUserIdentityInfo"),
            ConnectActions::UpdateUserPhoneConfig => write!(f, "connect:UpdateUserPhoneConfig"),
            ConnectActions::UpdateUserProficiencies => write!(f, "connect:UpdateUserProficiencies"),
            ConnectActions::UpdateUserRoutingProfile => {
                write!(f, "connect:UpdateUserRoutingProfile")
            }
            ConnectActions::UpdateUserSecurityProfiles => {
                write!(f, "connect:UpdateUserSecurityProfiles")
            }
            ConnectActions::UpdateViewContent => write!(f, "connect:UpdateViewContent"),
            ConnectActions::UpdateViewMetadata => write!(f, "connect:UpdateViewMetadata"),
        }
    }
}
