// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum ChimeActions {
    AcceptDelegate,
    ActivateUsers,
    AddDomain,
    AddOrUpdateGroups,
    AssociateChannelFlow,
    AssociatePhoneNumberWithUser,
    AssociatePhoneNumbersWithVoiceConnector,
    AssociatePhoneNumbersWithVoiceConnectorGroup,
    AssociateSigninDelegateGroupsWithAccount,
    AssociateVoiceConnectorConnect,
    AuthorizeDirectory,
    BatchCreateAttendee,
    BatchCreateChannelMembership,
    BatchCreateRoomMembership,
    BatchDeletePhoneNumber,
    BatchSuspendUser,
    BatchUnsuspendUser,
    BatchUpdateAttendeeCapabilitiesExcept,
    BatchUpdatePhoneNumber,
    BatchUpdateUser,
    ChannelFlowCallback,
    Connect,
    ConnectDirectory,
    CreateAccount,
    CreateApiKey,
    CreateAppInstance,
    CreateAppInstanceAdmin,
    CreateAppInstanceBot,
    CreateAppInstanceUser,
    CreateAttendee,
    CreateBot,
    CreateCdrBucket,
    CreateChannel,
    CreateChannelBan,
    CreateChannelFlow,
    CreateChannelMembership,
    CreateChannelModerator,
    CreateConnectAnalyticsConnector,
    CreateConnectCallTransferConnector,
    CreateMediaCapturePipeline,
    CreateMediaConcatenationPipeline,
    CreateMediaInsightsPipeline,
    CreateMediaInsightsPipelineConfiguration,
    CreateMediaLiveConnectorPipeline,
    CreateMediaPipelineKinesisVideoStreamPool,
    CreateMediaStreamPipeline,
    CreateMeeting,
    CreateMeetingDialOut,
    CreateMeetingWithAttendees,
    CreatePhoneNumberOrder,
    CreateProxySession,
    CreateRoom,
    CreateRoomMembership,
    CreateSipMediaApplication,
    CreateSipMediaApplicationCall,
    CreateSipRule,
    CreateUser,
    CreateVoiceConnector,
    CreateVoiceConnectorGroup,
    CreateVoiceProfile,
    CreateVoiceProfileDomain,
    DeleteAccount,
    DeleteAccountOpenIdConfig,
    DeleteApiKey,
    DeleteAppInstance,
    DeleteAppInstanceAdmin,
    DeleteAppInstanceBot,
    DeleteAppInstanceStreamingConfigurations,
    DeleteAppInstanceUser,
    DeleteAttendee,
    DeleteCdrBucket,
    DeleteChannel,
    DeleteChannelBan,
    DeleteChannelFlow,
    DeleteChannelMembership,
    DeleteChannelMessage,
    DeleteChannelModerator,
    DeleteDelegate,
    DeleteDomain,
    DeleteEventsConfiguration,
    DeleteGroups,
    DeleteMediaCapturePipeline,
    DeleteMediaInsightsPipelineConfiguration,
    DeleteMediaPipeline,
    DeleteMediaPipelineKinesisVideoStreamPool,
    DeleteMeeting,
    DeleteMessagingStreamingConfigurations,
    DeletePhoneNumber,
    DeleteProxySession,
    DeleteRoom,
    DeleteRoomMembership,
    DeleteSipMediaApplication,
    DeleteSipRule,
    DeleteVoiceConnector,
    DeleteVoiceConnectorEmergencyCallingConfiguration,
    DeleteVoiceConnectorExternalSystemsConfiguration,
    DeleteVoiceConnectorGroup,
    DeleteVoiceConnectorOrigination,
    DeleteVoiceConnectorProxy,
    DeleteVoiceConnectorStreamingConfiguration,
    DeleteVoiceConnectorTermination,
    DeleteVoiceConnectorTerminationCredentials,
    DeleteVoiceProfile,
    DeleteVoiceProfileDomain,
    DeregisterAppInstanceUserEndpoint,
    DescribeAppInstance,
    DescribeAppInstanceAdmin,
    DescribeAppInstanceBot,
    DescribeAppInstanceUser,
    DescribeAppInstanceUserEndpoint,
    DescribeChannel,
    DescribeChannelBan,
    DescribeChannelFlow,
    DescribeChannelMembership,
    DescribeChannelMembershipForAppInstanceUser,
    DescribeChannelModeratedByAppInstanceUser,
    DescribeChannelModerator,
    DisassociateChannelFlow,
    DisassociatePhoneNumberFromUser,
    DisassociatePhoneNumbersFromVoiceConnector,
    DisassociatePhoneNumbersFromVoiceConnectorGroup,
    DisassociateSigninDelegateGroupsFromAccount,
    DisassociateVoiceConnectorConnect,
    DisconnectDirectory,
    GetAccount,
    GetAccountResource,
    GetAccountSettings,
    GetAccountWithOpenIdConfig,
    GetAppInstanceRetentionSettings,
    GetAppInstanceStreamingConfigurations,
    GetAttendee,
    GetBot,
    GetCdrBucket,
    GetChannelMembershipPreferences,
    GetChannelMessage,
    GetChannelMessageStatus,
    GetDomain,
    GetEventsConfiguration,
    GetGlobalSettings,
    GetMediaCapturePipeline,
    GetMediaInsightsPipelineConfiguration,
    GetMediaPipeline,
    GetMediaPipelineKinesisVideoStreamPool,
    GetMeeting,
    GetMeetingDetail,
    GetMessagingSessionEndpoint,
    GetMessagingStreamingConfigurations,
    GetPhoneNumber,
    GetPhoneNumberOrder,
    GetPhoneNumberSettings,
    GetProxySession,
    GetRetentionSettings,
    GetRoom,
    GetSipMediaApplication,
    GetSipMediaApplicationAlexaSkillConfiguration,
    GetSipMediaApplicationLoggingConfiguration,
    GetSipRule,
    GetSpeakerSearchTask,
    GetTelephonyLimits,
    GetUser,
    GetUserActivityReportData,
    GetUserByEmail,
    GetUserSettings,
    GetVoiceConnector,
    GetVoiceConnectorEmergencyCallingConfiguration,
    GetVoiceConnectorExternalSystemsConfiguration,
    GetVoiceConnectorGroup,
    GetVoiceConnectorLoggingConfiguration,
    GetVoiceConnectorOrigination,
    GetVoiceConnectorProxy,
    GetVoiceConnectorStreamingConfiguration,
    GetVoiceConnectorTermination,
    GetVoiceConnectorTerminationHealth,
    GetVoiceProfile,
    GetVoiceProfileDomain,
    GetVoiceToneAnalysisTask,
    InviteDelegate,
    InviteUsers,
    InviteUsersFromProvider,
    ListAccountUsageReportData,
    ListAccounts,
    ListApiKeys,
    ListAppInstanceAdmins,
    ListAppInstanceBots,
    ListAppInstanceUserEndpoints,
    ListAppInstanceUsers,
    ListAppInstances,
    ListAttendeeTags,
    ListAttendees,
    ListAvailableVoiceConnectorRegions,
    ListBots,
    ListCallingRegions,
    ListCdrBucket,
    ListChannelBans,
    ListChannelFlows,
    ListChannelMemberships,
    ListChannelMembershipsForAppInstanceUser,
    ListChannelMessages,
    ListChannelModerators,
    ListChannels,
    ListChannelsAssociatedWithChannelFlow,
    ListChannelsModeratedByAppInstanceUser,
    ListDelegates,
    ListDirectories,
    ListDomains,
    ListGroups,
    ListMediaCapturePipelines,
    ListMediaInsightsPipelineConfigurations,
    ListMediaPipelineKinesisVideoStreamPools,
    ListMediaPipelines,
    ListMeetingEvents,
    ListMeetingTags,
    ListMeetings,
    ListMeetingsReportData,
    ListPhoneNumberOrders,
    ListPhoneNumbers,
    ListProxySessions,
    ListRoomMemberships,
    ListRooms,
    ListSipMediaApplications,
    ListSipRules,
    ListSubChannels,
    ListSupportedPhoneNumberCountries,
    ListTagsForResource,
    ListUsers,
    ListVoiceConnectorGroups,
    ListVoiceConnectorTerminationCredentials,
    ListVoiceConnectors,
    ListVoiceProfileDomains,
    ListVoiceProfiles,
    LogoutUser,
    PutAppInstanceRetentionSettings,
    PutAppInstanceStreamingConfigurations,
    PutAppInstanceUserExpirationSettings,
    PutChannelExpirationSettings,
    PutChannelMembershipPreferences,
    PutEventsConfiguration,
    PutMessagingStreamingConfigurations,
    PutRetentionSettings,
    PutSipMediaApplicationAlexaSkillConfiguration,
    PutSipMediaApplicationLoggingConfiguration,
    PutVoiceConnectorEmergencyCallingConfiguration,
    PutVoiceConnectorExternalSystemsConfiguration,
    PutVoiceConnectorLoggingConfiguration,
    PutVoiceConnectorOrigination,
    PutVoiceConnectorProxy,
    PutVoiceConnectorStreamingConfiguration,
    PutVoiceConnectorTermination,
    PutVoiceConnectorTerminationCredentials,
    RedactChannelMessage,
    RedactConversationMessage,
    RedactRoomMessage,
    RegenerateSecurityToken,
    RegisterAppInstanceUserEndpoint,
    RenameAccount,
    RenewDelegate,
    ResetAccountResource,
    ResetPersonalPin,
    RestorePhoneNumber,
    RetrieveDataExports,
    SearchAvailablePhoneNumbers,
    SearchChannels,
    SendChannelMessage,
    StartDataExport,
    StartMeetingTranscription,
    StartSpeakerSearchTask,
    StartVoiceToneAnalysisTask,
    StopMeetingTranscription,
    StopSpeakerSearchTask,
    StopVoiceToneAnalysisTask,
    SubmitSupportRequest,
    SuspendUsers,
    TagAttendee,
    TagMeeting,
    TagResource,
    UnauthorizeDirectory,
    UntagAttendee,
    UntagMeeting,
    UntagResource,
    UpdateAccount,
    UpdateAccountOpenIdConfig,
    UpdateAccountResource,
    UpdateAccountSettings,
    UpdateAppInstance,
    UpdateAppInstanceBot,
    UpdateAppInstanceUser,
    UpdateAppInstanceUserEndpoint,
    UpdateAttendeeCapabilities,
    UpdateBot,
    UpdateCdrSettings,
    UpdateChannel,
    UpdateChannelFlow,
    UpdateChannelMessage,
    UpdateChannelReadMarker,
    UpdateGlobalSettings,
    UpdateMediaInsightsPipelineConfiguration,
    UpdateMediaInsightsPipelineStatus,
    UpdateMediaPipelineKinesisVideoStreamPool,
    UpdatePhoneNumber,
    UpdatePhoneNumberSettings,
    UpdateProxySession,
    UpdateRoom,
    UpdateRoomMembership,
    UpdateSipMediaApplication,
    UpdateSipMediaApplicationCall,
    UpdateSipRule,
    UpdateSupportedLicenses,
    UpdateUser,
    UpdateUserLicenses,
    UpdateUserSettings,
    UpdateVoiceConnector,
    UpdateVoiceConnectorGroup,
    UpdateVoiceProfile,
    UpdateVoiceProfileDomain,
    ValidateAccountResource,
    ValidateE911Address,
}
impl std::fmt::Display for ChimeActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChimeActions::AcceptDelegate => write!(f, "chime:AcceptDelegate"),
            ChimeActions::ActivateUsers => write!(f, "chime:ActivateUsers"),
            ChimeActions::AddDomain => write!(f, "chime:AddDomain"),
            ChimeActions::AddOrUpdateGroups => write!(f, "chime:AddOrUpdateGroups"),
            ChimeActions::AssociateChannelFlow => write!(f, "chime:AssociateChannelFlow"),
            ChimeActions::AssociatePhoneNumberWithUser => {
                write!(f, "chime:AssociatePhoneNumberWithUser")
            }
            ChimeActions::AssociatePhoneNumbersWithVoiceConnector => {
                write!(f, "chime:AssociatePhoneNumbersWithVoiceConnector")
            }
            ChimeActions::AssociatePhoneNumbersWithVoiceConnectorGroup => {
                write!(f, "chime:AssociatePhoneNumbersWithVoiceConnectorGroup")
            }
            ChimeActions::AssociateSigninDelegateGroupsWithAccount => {
                write!(f, "chime:AssociateSigninDelegateGroupsWithAccount")
            }
            ChimeActions::AssociateVoiceConnectorConnect => {
                write!(f, "chime:AssociateVoiceConnectorConnect")
            }
            ChimeActions::AuthorizeDirectory => write!(f, "chime:AuthorizeDirectory"),
            ChimeActions::BatchCreateAttendee => write!(f, "chime:BatchCreateAttendee"),
            ChimeActions::BatchCreateChannelMembership => {
                write!(f, "chime:BatchCreateChannelMembership")
            }
            ChimeActions::BatchCreateRoomMembership => write!(f, "chime:BatchCreateRoomMembership"),
            ChimeActions::BatchDeletePhoneNumber => write!(f, "chime:BatchDeletePhoneNumber"),
            ChimeActions::BatchSuspendUser => write!(f, "chime:BatchSuspendUser"),
            ChimeActions::BatchUnsuspendUser => write!(f, "chime:BatchUnsuspendUser"),
            ChimeActions::BatchUpdateAttendeeCapabilitiesExcept => {
                write!(f, "chime:BatchUpdateAttendeeCapabilitiesExcept")
            }
            ChimeActions::BatchUpdatePhoneNumber => write!(f, "chime:BatchUpdatePhoneNumber"),
            ChimeActions::BatchUpdateUser => write!(f, "chime:BatchUpdateUser"),
            ChimeActions::ChannelFlowCallback => write!(f, "chime:ChannelFlowCallback"),
            ChimeActions::Connect => write!(f, "chime:Connect"),
            ChimeActions::ConnectDirectory => write!(f, "chime:ConnectDirectory"),
            ChimeActions::CreateAccount => write!(f, "chime:CreateAccount"),
            ChimeActions::CreateApiKey => write!(f, "chime:CreateApiKey"),
            ChimeActions::CreateAppInstance => write!(f, "chime:CreateAppInstance"),
            ChimeActions::CreateAppInstanceAdmin => write!(f, "chime:CreateAppInstanceAdmin"),
            ChimeActions::CreateAppInstanceBot => write!(f, "chime:CreateAppInstanceBot"),
            ChimeActions::CreateAppInstanceUser => write!(f, "chime:CreateAppInstanceUser"),
            ChimeActions::CreateAttendee => write!(f, "chime:CreateAttendee"),
            ChimeActions::CreateBot => write!(f, "chime:CreateBot"),
            ChimeActions::CreateCdrBucket => write!(f, "chime:CreateCDRBucket"),
            ChimeActions::CreateChannel => write!(f, "chime:CreateChannel"),
            ChimeActions::CreateChannelBan => write!(f, "chime:CreateChannelBan"),
            ChimeActions::CreateChannelFlow => write!(f, "chime:CreateChannelFlow"),
            ChimeActions::CreateChannelMembership => write!(f, "chime:CreateChannelMembership"),
            ChimeActions::CreateChannelModerator => write!(f, "chime:CreateChannelModerator"),
            ChimeActions::CreateConnectAnalyticsConnector => {
                write!(f, "chime:CreateConnectAnalyticsConnector")
            }
            ChimeActions::CreateConnectCallTransferConnector => {
                write!(f, "chime:CreateConnectCallTransferConnector")
            }
            ChimeActions::CreateMediaCapturePipeline => {
                write!(f, "chime:CreateMediaCapturePipeline")
            }
            ChimeActions::CreateMediaConcatenationPipeline => {
                write!(f, "chime:CreateMediaConcatenationPipeline")
            }
            ChimeActions::CreateMediaInsightsPipeline => {
                write!(f, "chime:CreateMediaInsightsPipeline")
            }
            ChimeActions::CreateMediaInsightsPipelineConfiguration => {
                write!(f, "chime:CreateMediaInsightsPipelineConfiguration")
            }
            ChimeActions::CreateMediaLiveConnectorPipeline => {
                write!(f, "chime:CreateMediaLiveConnectorPipeline")
            }
            ChimeActions::CreateMediaPipelineKinesisVideoStreamPool => {
                write!(f, "chime:CreateMediaPipelineKinesisVideoStreamPool")
            }
            ChimeActions::CreateMediaStreamPipeline => write!(f, "chime:CreateMediaStreamPipeline"),
            ChimeActions::CreateMeeting => write!(f, "chime:CreateMeeting"),
            ChimeActions::CreateMeetingDialOut => write!(f, "chime:CreateMeetingDialOut"),
            ChimeActions::CreateMeetingWithAttendees => {
                write!(f, "chime:CreateMeetingWithAttendees")
            }
            ChimeActions::CreatePhoneNumberOrder => write!(f, "chime:CreatePhoneNumberOrder"),
            ChimeActions::CreateProxySession => write!(f, "chime:CreateProxySession"),
            ChimeActions::CreateRoom => write!(f, "chime:CreateRoom"),
            ChimeActions::CreateRoomMembership => write!(f, "chime:CreateRoomMembership"),
            ChimeActions::CreateSipMediaApplication => write!(f, "chime:CreateSipMediaApplication"),
            ChimeActions::CreateSipMediaApplicationCall => {
                write!(f, "chime:CreateSipMediaApplicationCall")
            }
            ChimeActions::CreateSipRule => write!(f, "chime:CreateSipRule"),
            ChimeActions::CreateUser => write!(f, "chime:CreateUser"),
            ChimeActions::CreateVoiceConnector => write!(f, "chime:CreateVoiceConnector"),
            ChimeActions::CreateVoiceConnectorGroup => write!(f, "chime:CreateVoiceConnectorGroup"),
            ChimeActions::CreateVoiceProfile => write!(f, "chime:CreateVoiceProfile"),
            ChimeActions::CreateVoiceProfileDomain => write!(f, "chime:CreateVoiceProfileDomain"),
            ChimeActions::DeleteAccount => write!(f, "chime:DeleteAccount"),
            ChimeActions::DeleteAccountOpenIdConfig => write!(f, "chime:DeleteAccountOpenIdConfig"),
            ChimeActions::DeleteApiKey => write!(f, "chime:DeleteApiKey"),
            ChimeActions::DeleteAppInstance => write!(f, "chime:DeleteAppInstance"),
            ChimeActions::DeleteAppInstanceAdmin => write!(f, "chime:DeleteAppInstanceAdmin"),
            ChimeActions::DeleteAppInstanceBot => write!(f, "chime:DeleteAppInstanceBot"),
            ChimeActions::DeleteAppInstanceStreamingConfigurations => {
                write!(f, "chime:DeleteAppInstanceStreamingConfigurations")
            }
            ChimeActions::DeleteAppInstanceUser => write!(f, "chime:DeleteAppInstanceUser"),
            ChimeActions::DeleteAttendee => write!(f, "chime:DeleteAttendee"),
            ChimeActions::DeleteCdrBucket => write!(f, "chime:DeleteCDRBucket"),
            ChimeActions::DeleteChannel => write!(f, "chime:DeleteChannel"),
            ChimeActions::DeleteChannelBan => write!(f, "chime:DeleteChannelBan"),
            ChimeActions::DeleteChannelFlow => write!(f, "chime:DeleteChannelFlow"),
            ChimeActions::DeleteChannelMembership => write!(f, "chime:DeleteChannelMembership"),
            ChimeActions::DeleteChannelMessage => write!(f, "chime:DeleteChannelMessage"),
            ChimeActions::DeleteChannelModerator => write!(f, "chime:DeleteChannelModerator"),
            ChimeActions::DeleteDelegate => write!(f, "chime:DeleteDelegate"),
            ChimeActions::DeleteDomain => write!(f, "chime:DeleteDomain"),
            ChimeActions::DeleteEventsConfiguration => write!(f, "chime:DeleteEventsConfiguration"),
            ChimeActions::DeleteGroups => write!(f, "chime:DeleteGroups"),
            ChimeActions::DeleteMediaCapturePipeline => {
                write!(f, "chime:DeleteMediaCapturePipeline")
            }
            ChimeActions::DeleteMediaInsightsPipelineConfiguration => {
                write!(f, "chime:DeleteMediaInsightsPipelineConfiguration")
            }
            ChimeActions::DeleteMediaPipeline => write!(f, "chime:DeleteMediaPipeline"),
            ChimeActions::DeleteMediaPipelineKinesisVideoStreamPool => {
                write!(f, "chime:DeleteMediaPipelineKinesisVideoStreamPool")
            }
            ChimeActions::DeleteMeeting => write!(f, "chime:DeleteMeeting"),
            ChimeActions::DeleteMessagingStreamingConfigurations => {
                write!(f, "chime:DeleteMessagingStreamingConfigurations")
            }
            ChimeActions::DeletePhoneNumber => write!(f, "chime:DeletePhoneNumber"),
            ChimeActions::DeleteProxySession => write!(f, "chime:DeleteProxySession"),
            ChimeActions::DeleteRoom => write!(f, "chime:DeleteRoom"),
            ChimeActions::DeleteRoomMembership => write!(f, "chime:DeleteRoomMembership"),
            ChimeActions::DeleteSipMediaApplication => write!(f, "chime:DeleteSipMediaApplication"),
            ChimeActions::DeleteSipRule => write!(f, "chime:DeleteSipRule"),
            ChimeActions::DeleteVoiceConnector => write!(f, "chime:DeleteVoiceConnector"),
            ChimeActions::DeleteVoiceConnectorEmergencyCallingConfiguration => {
                write!(f, "chime:DeleteVoiceConnectorEmergencyCallingConfiguration")
            }
            ChimeActions::DeleteVoiceConnectorExternalSystemsConfiguration => {
                write!(f, "chime:DeleteVoiceConnectorExternalSystemsConfiguration")
            }
            ChimeActions::DeleteVoiceConnectorGroup => write!(f, "chime:DeleteVoiceConnectorGroup"),
            ChimeActions::DeleteVoiceConnectorOrigination => {
                write!(f, "chime:DeleteVoiceConnectorOrigination")
            }
            ChimeActions::DeleteVoiceConnectorProxy => write!(f, "chime:DeleteVoiceConnectorProxy"),
            ChimeActions::DeleteVoiceConnectorStreamingConfiguration => {
                write!(f, "chime:DeleteVoiceConnectorStreamingConfiguration")
            }
            ChimeActions::DeleteVoiceConnectorTermination => {
                write!(f, "chime:DeleteVoiceConnectorTermination")
            }
            ChimeActions::DeleteVoiceConnectorTerminationCredentials => {
                write!(f, "chime:DeleteVoiceConnectorTerminationCredentials")
            }
            ChimeActions::DeleteVoiceProfile => write!(f, "chime:DeleteVoiceProfile"),
            ChimeActions::DeleteVoiceProfileDomain => write!(f, "chime:DeleteVoiceProfileDomain"),
            ChimeActions::DeregisterAppInstanceUserEndpoint => {
                write!(f, "chime:DeregisterAppInstanceUserEndpoint")
            }
            ChimeActions::DescribeAppInstance => write!(f, "chime:DescribeAppInstance"),
            ChimeActions::DescribeAppInstanceAdmin => write!(f, "chime:DescribeAppInstanceAdmin"),
            ChimeActions::DescribeAppInstanceBot => write!(f, "chime:DescribeAppInstanceBot"),
            ChimeActions::DescribeAppInstanceUser => write!(f, "chime:DescribeAppInstanceUser"),
            ChimeActions::DescribeAppInstanceUserEndpoint => {
                write!(f, "chime:DescribeAppInstanceUserEndpoint")
            }
            ChimeActions::DescribeChannel => write!(f, "chime:DescribeChannel"),
            ChimeActions::DescribeChannelBan => write!(f, "chime:DescribeChannelBan"),
            ChimeActions::DescribeChannelFlow => write!(f, "chime:DescribeChannelFlow"),
            ChimeActions::DescribeChannelMembership => write!(f, "chime:DescribeChannelMembership"),
            ChimeActions::DescribeChannelMembershipForAppInstanceUser => {
                write!(f, "chime:DescribeChannelMembershipForAppInstanceUser")
            }
            ChimeActions::DescribeChannelModeratedByAppInstanceUser => {
                write!(f, "chime:DescribeChannelModeratedByAppInstanceUser")
            }
            ChimeActions::DescribeChannelModerator => write!(f, "chime:DescribeChannelModerator"),
            ChimeActions::DisassociateChannelFlow => write!(f, "chime:DisassociateChannelFlow"),
            ChimeActions::DisassociatePhoneNumberFromUser => {
                write!(f, "chime:DisassociatePhoneNumberFromUser")
            }
            ChimeActions::DisassociatePhoneNumbersFromVoiceConnector => {
                write!(f, "chime:DisassociatePhoneNumbersFromVoiceConnector")
            }
            ChimeActions::DisassociatePhoneNumbersFromVoiceConnectorGroup => {
                write!(f, "chime:DisassociatePhoneNumbersFromVoiceConnectorGroup")
            }
            ChimeActions::DisassociateSigninDelegateGroupsFromAccount => {
                write!(f, "chime:DisassociateSigninDelegateGroupsFromAccount")
            }
            ChimeActions::DisassociateVoiceConnectorConnect => {
                write!(f, "chime:DisassociateVoiceConnectorConnect")
            }
            ChimeActions::DisconnectDirectory => write!(f, "chime:DisconnectDirectory"),
            ChimeActions::GetAccount => write!(f, "chime:GetAccount"),
            ChimeActions::GetAccountResource => write!(f, "chime:GetAccountResource"),
            ChimeActions::GetAccountSettings => write!(f, "chime:GetAccountSettings"),
            ChimeActions::GetAccountWithOpenIdConfig => {
                write!(f, "chime:GetAccountWithOpenIdConfig")
            }
            ChimeActions::GetAppInstanceRetentionSettings => {
                write!(f, "chime:GetAppInstanceRetentionSettings")
            }
            ChimeActions::GetAppInstanceStreamingConfigurations => {
                write!(f, "chime:GetAppInstanceStreamingConfigurations")
            }
            ChimeActions::GetAttendee => write!(f, "chime:GetAttendee"),
            ChimeActions::GetBot => write!(f, "chime:GetBot"),
            ChimeActions::GetCdrBucket => write!(f, "chime:GetCDRBucket"),
            ChimeActions::GetChannelMembershipPreferences => {
                write!(f, "chime:GetChannelMembershipPreferences")
            }
            ChimeActions::GetChannelMessage => write!(f, "chime:GetChannelMessage"),
            ChimeActions::GetChannelMessageStatus => write!(f, "chime:GetChannelMessageStatus"),
            ChimeActions::GetDomain => write!(f, "chime:GetDomain"),
            ChimeActions::GetEventsConfiguration => write!(f, "chime:GetEventsConfiguration"),
            ChimeActions::GetGlobalSettings => write!(f, "chime:GetGlobalSettings"),
            ChimeActions::GetMediaCapturePipeline => write!(f, "chime:GetMediaCapturePipeline"),
            ChimeActions::GetMediaInsightsPipelineConfiguration => {
                write!(f, "chime:GetMediaInsightsPipelineConfiguration")
            }
            ChimeActions::GetMediaPipeline => write!(f, "chime:GetMediaPipeline"),
            ChimeActions::GetMediaPipelineKinesisVideoStreamPool => {
                write!(f, "chime:GetMediaPipelineKinesisVideoStreamPool")
            }
            ChimeActions::GetMeeting => write!(f, "chime:GetMeeting"),
            ChimeActions::GetMeetingDetail => write!(f, "chime:GetMeetingDetail"),
            ChimeActions::GetMessagingSessionEndpoint => {
                write!(f, "chime:GetMessagingSessionEndpoint")
            }
            ChimeActions::GetMessagingStreamingConfigurations => {
                write!(f, "chime:GetMessagingStreamingConfigurations")
            }
            ChimeActions::GetPhoneNumber => write!(f, "chime:GetPhoneNumber"),
            ChimeActions::GetPhoneNumberOrder => write!(f, "chime:GetPhoneNumberOrder"),
            ChimeActions::GetPhoneNumberSettings => write!(f, "chime:GetPhoneNumberSettings"),
            ChimeActions::GetProxySession => write!(f, "chime:GetProxySession"),
            ChimeActions::GetRetentionSettings => write!(f, "chime:GetRetentionSettings"),
            ChimeActions::GetRoom => write!(f, "chime:GetRoom"),
            ChimeActions::GetSipMediaApplication => write!(f, "chime:GetSipMediaApplication"),
            ChimeActions::GetSipMediaApplicationAlexaSkillConfiguration => {
                write!(f, "chime:GetSipMediaApplicationAlexaSkillConfiguration")
            }
            ChimeActions::GetSipMediaApplicationLoggingConfiguration => {
                write!(f, "chime:GetSipMediaApplicationLoggingConfiguration")
            }
            ChimeActions::GetSipRule => write!(f, "chime:GetSipRule"),
            ChimeActions::GetSpeakerSearchTask => write!(f, "chime:GetSpeakerSearchTask"),
            ChimeActions::GetTelephonyLimits => write!(f, "chime:GetTelephonyLimits"),
            ChimeActions::GetUser => write!(f, "chime:GetUser"),
            ChimeActions::GetUserActivityReportData => write!(f, "chime:GetUserActivityReportData"),
            ChimeActions::GetUserByEmail => write!(f, "chime:GetUserByEmail"),
            ChimeActions::GetUserSettings => write!(f, "chime:GetUserSettings"),
            ChimeActions::GetVoiceConnector => write!(f, "chime:GetVoiceConnector"),
            ChimeActions::GetVoiceConnectorEmergencyCallingConfiguration => {
                write!(f, "chime:GetVoiceConnectorEmergencyCallingConfiguration")
            }
            ChimeActions::GetVoiceConnectorExternalSystemsConfiguration => {
                write!(f, "chime:GetVoiceConnectorExternalSystemsConfiguration")
            }
            ChimeActions::GetVoiceConnectorGroup => write!(f, "chime:GetVoiceConnectorGroup"),
            ChimeActions::GetVoiceConnectorLoggingConfiguration => {
                write!(f, "chime:GetVoiceConnectorLoggingConfiguration")
            }
            ChimeActions::GetVoiceConnectorOrigination => {
                write!(f, "chime:GetVoiceConnectorOrigination")
            }
            ChimeActions::GetVoiceConnectorProxy => write!(f, "chime:GetVoiceConnectorProxy"),
            ChimeActions::GetVoiceConnectorStreamingConfiguration => {
                write!(f, "chime:GetVoiceConnectorStreamingConfiguration")
            }
            ChimeActions::GetVoiceConnectorTermination => {
                write!(f, "chime:GetVoiceConnectorTermination")
            }
            ChimeActions::GetVoiceConnectorTerminationHealth => {
                write!(f, "chime:GetVoiceConnectorTerminationHealth")
            }
            ChimeActions::GetVoiceProfile => write!(f, "chime:GetVoiceProfile"),
            ChimeActions::GetVoiceProfileDomain => write!(f, "chime:GetVoiceProfileDomain"),
            ChimeActions::GetVoiceToneAnalysisTask => write!(f, "chime:GetVoiceToneAnalysisTask"),
            ChimeActions::InviteDelegate => write!(f, "chime:InviteDelegate"),
            ChimeActions::InviteUsers => write!(f, "chime:InviteUsers"),
            ChimeActions::InviteUsersFromProvider => write!(f, "chime:InviteUsersFromProvider"),
            ChimeActions::ListAccountUsageReportData => {
                write!(f, "chime:ListAccountUsageReportData")
            }
            ChimeActions::ListAccounts => write!(f, "chime:ListAccounts"),
            ChimeActions::ListApiKeys => write!(f, "chime:ListApiKeys"),
            ChimeActions::ListAppInstanceAdmins => write!(f, "chime:ListAppInstanceAdmins"),
            ChimeActions::ListAppInstanceBots => write!(f, "chime:ListAppInstanceBots"),
            ChimeActions::ListAppInstanceUserEndpoints => {
                write!(f, "chime:ListAppInstanceUserEndpoints")
            }
            ChimeActions::ListAppInstanceUsers => write!(f, "chime:ListAppInstanceUsers"),
            ChimeActions::ListAppInstances => write!(f, "chime:ListAppInstances"),
            ChimeActions::ListAttendeeTags => write!(f, "chime:ListAttendeeTags"),
            ChimeActions::ListAttendees => write!(f, "chime:ListAttendees"),
            ChimeActions::ListAvailableVoiceConnectorRegions => {
                write!(f, "chime:ListAvailableVoiceConnectorRegions")
            }
            ChimeActions::ListBots => write!(f, "chime:ListBots"),
            ChimeActions::ListCallingRegions => write!(f, "chime:ListCallingRegions"),
            ChimeActions::ListCdrBucket => write!(f, "chime:ListCDRBucket"),
            ChimeActions::ListChannelBans => write!(f, "chime:ListChannelBans"),
            ChimeActions::ListChannelFlows => write!(f, "chime:ListChannelFlows"),
            ChimeActions::ListChannelMemberships => write!(f, "chime:ListChannelMemberships"),
            ChimeActions::ListChannelMembershipsForAppInstanceUser => {
                write!(f, "chime:ListChannelMembershipsForAppInstanceUser")
            }
            ChimeActions::ListChannelMessages => write!(f, "chime:ListChannelMessages"),
            ChimeActions::ListChannelModerators => write!(f, "chime:ListChannelModerators"),
            ChimeActions::ListChannels => write!(f, "chime:ListChannels"),
            ChimeActions::ListChannelsAssociatedWithChannelFlow => {
                write!(f, "chime:ListChannelsAssociatedWithChannelFlow")
            }
            ChimeActions::ListChannelsModeratedByAppInstanceUser => {
                write!(f, "chime:ListChannelsModeratedByAppInstanceUser")
            }
            ChimeActions::ListDelegates => write!(f, "chime:ListDelegates"),
            ChimeActions::ListDirectories => write!(f, "chime:ListDirectories"),
            ChimeActions::ListDomains => write!(f, "chime:ListDomains"),
            ChimeActions::ListGroups => write!(f, "chime:ListGroups"),
            ChimeActions::ListMediaCapturePipelines => write!(f, "chime:ListMediaCapturePipelines"),
            ChimeActions::ListMediaInsightsPipelineConfigurations => {
                write!(f, "chime:ListMediaInsightsPipelineConfigurations")
            }
            ChimeActions::ListMediaPipelineKinesisVideoStreamPools => {
                write!(f, "chime:ListMediaPipelineKinesisVideoStreamPools")
            }
            ChimeActions::ListMediaPipelines => write!(f, "chime:ListMediaPipelines"),
            ChimeActions::ListMeetingEvents => write!(f, "chime:ListMeetingEvents"),
            ChimeActions::ListMeetingTags => write!(f, "chime:ListMeetingTags"),
            ChimeActions::ListMeetings => write!(f, "chime:ListMeetings"),
            ChimeActions::ListMeetingsReportData => write!(f, "chime:ListMeetingsReportData"),
            ChimeActions::ListPhoneNumberOrders => write!(f, "chime:ListPhoneNumberOrders"),
            ChimeActions::ListPhoneNumbers => write!(f, "chime:ListPhoneNumbers"),
            ChimeActions::ListProxySessions => write!(f, "chime:ListProxySessions"),
            ChimeActions::ListRoomMemberships => write!(f, "chime:ListRoomMemberships"),
            ChimeActions::ListRooms => write!(f, "chime:ListRooms"),
            ChimeActions::ListSipMediaApplications => write!(f, "chime:ListSipMediaApplications"),
            ChimeActions::ListSipRules => write!(f, "chime:ListSipRules"),
            ChimeActions::ListSubChannels => write!(f, "chime:ListSubChannels"),
            ChimeActions::ListSupportedPhoneNumberCountries => {
                write!(f, "chime:ListSupportedPhoneNumberCountries")
            }
            ChimeActions::ListTagsForResource => write!(f, "chime:ListTagsForResource"),
            ChimeActions::ListUsers => write!(f, "chime:ListUsers"),
            ChimeActions::ListVoiceConnectorGroups => write!(f, "chime:ListVoiceConnectorGroups"),
            ChimeActions::ListVoiceConnectorTerminationCredentials => {
                write!(f, "chime:ListVoiceConnectorTerminationCredentials")
            }
            ChimeActions::ListVoiceConnectors => write!(f, "chime:ListVoiceConnectors"),
            ChimeActions::ListVoiceProfileDomains => write!(f, "chime:ListVoiceProfileDomains"),
            ChimeActions::ListVoiceProfiles => write!(f, "chime:ListVoiceProfiles"),
            ChimeActions::LogoutUser => write!(f, "chime:LogoutUser"),
            ChimeActions::PutAppInstanceRetentionSettings => {
                write!(f, "chime:PutAppInstanceRetentionSettings")
            }
            ChimeActions::PutAppInstanceStreamingConfigurations => {
                write!(f, "chime:PutAppInstanceStreamingConfigurations")
            }
            ChimeActions::PutAppInstanceUserExpirationSettings => {
                write!(f, "chime:PutAppInstanceUserExpirationSettings")
            }
            ChimeActions::PutChannelExpirationSettings => {
                write!(f, "chime:PutChannelExpirationSettings")
            }
            ChimeActions::PutChannelMembershipPreferences => {
                write!(f, "chime:PutChannelMembershipPreferences")
            }
            ChimeActions::PutEventsConfiguration => write!(f, "chime:PutEventsConfiguration"),
            ChimeActions::PutMessagingStreamingConfigurations => {
                write!(f, "chime:PutMessagingStreamingConfigurations")
            }
            ChimeActions::PutRetentionSettings => write!(f, "chime:PutRetentionSettings"),
            ChimeActions::PutSipMediaApplicationAlexaSkillConfiguration => {
                write!(f, "chime:PutSipMediaApplicationAlexaSkillConfiguration")
            }
            ChimeActions::PutSipMediaApplicationLoggingConfiguration => {
                write!(f, "chime:PutSipMediaApplicationLoggingConfiguration")
            }
            ChimeActions::PutVoiceConnectorEmergencyCallingConfiguration => {
                write!(f, "chime:PutVoiceConnectorEmergencyCallingConfiguration")
            }
            ChimeActions::PutVoiceConnectorExternalSystemsConfiguration => {
                write!(f, "chime:PutVoiceConnectorExternalSystemsConfiguration")
            }
            ChimeActions::PutVoiceConnectorLoggingConfiguration => {
                write!(f, "chime:PutVoiceConnectorLoggingConfiguration")
            }
            ChimeActions::PutVoiceConnectorOrigination => {
                write!(f, "chime:PutVoiceConnectorOrigination")
            }
            ChimeActions::PutVoiceConnectorProxy => write!(f, "chime:PutVoiceConnectorProxy"),
            ChimeActions::PutVoiceConnectorStreamingConfiguration => {
                write!(f, "chime:PutVoiceConnectorStreamingConfiguration")
            }
            ChimeActions::PutVoiceConnectorTermination => {
                write!(f, "chime:PutVoiceConnectorTermination")
            }
            ChimeActions::PutVoiceConnectorTerminationCredentials => {
                write!(f, "chime:PutVoiceConnectorTerminationCredentials")
            }
            ChimeActions::RedactChannelMessage => write!(f, "chime:RedactChannelMessage"),
            ChimeActions::RedactConversationMessage => write!(f, "chime:RedactConversationMessage"),
            ChimeActions::RedactRoomMessage => write!(f, "chime:RedactRoomMessage"),
            ChimeActions::RegenerateSecurityToken => write!(f, "chime:RegenerateSecurityToken"),
            ChimeActions::RegisterAppInstanceUserEndpoint => {
                write!(f, "chime:RegisterAppInstanceUserEndpoint")
            }
            ChimeActions::RenameAccount => write!(f, "chime:RenameAccount"),
            ChimeActions::RenewDelegate => write!(f, "chime:RenewDelegate"),
            ChimeActions::ResetAccountResource => write!(f, "chime:ResetAccountResource"),
            ChimeActions::ResetPersonalPin => write!(f, "chime:ResetPersonalPIN"),
            ChimeActions::RestorePhoneNumber => write!(f, "chime:RestorePhoneNumber"),
            ChimeActions::RetrieveDataExports => write!(f, "chime:RetrieveDataExports"),
            ChimeActions::SearchAvailablePhoneNumbers => {
                write!(f, "chime:SearchAvailablePhoneNumbers")
            }
            ChimeActions::SearchChannels => write!(f, "chime:SearchChannels"),
            ChimeActions::SendChannelMessage => write!(f, "chime:SendChannelMessage"),
            ChimeActions::StartDataExport => write!(f, "chime:StartDataExport"),
            ChimeActions::StartMeetingTranscription => write!(f, "chime:StartMeetingTranscription"),
            ChimeActions::StartSpeakerSearchTask => write!(f, "chime:StartSpeakerSearchTask"),
            ChimeActions::StartVoiceToneAnalysisTask => {
                write!(f, "chime:StartVoiceToneAnalysisTask")
            }
            ChimeActions::StopMeetingTranscription => write!(f, "chime:StopMeetingTranscription"),
            ChimeActions::StopSpeakerSearchTask => write!(f, "chime:StopSpeakerSearchTask"),
            ChimeActions::StopVoiceToneAnalysisTask => write!(f, "chime:StopVoiceToneAnalysisTask"),
            ChimeActions::SubmitSupportRequest => write!(f, "chime:SubmitSupportRequest"),
            ChimeActions::SuspendUsers => write!(f, "chime:SuspendUsers"),
            ChimeActions::TagAttendee => write!(f, "chime:TagAttendee"),
            ChimeActions::TagMeeting => write!(f, "chime:TagMeeting"),
            ChimeActions::TagResource => write!(f, "chime:TagResource"),
            ChimeActions::UnauthorizeDirectory => write!(f, "chime:UnauthorizeDirectory"),
            ChimeActions::UntagAttendee => write!(f, "chime:UntagAttendee"),
            ChimeActions::UntagMeeting => write!(f, "chime:UntagMeeting"),
            ChimeActions::UntagResource => write!(f, "chime:UntagResource"),
            ChimeActions::UpdateAccount => write!(f, "chime:UpdateAccount"),
            ChimeActions::UpdateAccountOpenIdConfig => write!(f, "chime:UpdateAccountOpenIdConfig"),
            ChimeActions::UpdateAccountResource => write!(f, "chime:UpdateAccountResource"),
            ChimeActions::UpdateAccountSettings => write!(f, "chime:UpdateAccountSettings"),
            ChimeActions::UpdateAppInstance => write!(f, "chime:UpdateAppInstance"),
            ChimeActions::UpdateAppInstanceBot => write!(f, "chime:UpdateAppInstanceBot"),
            ChimeActions::UpdateAppInstanceUser => write!(f, "chime:UpdateAppInstanceUser"),
            ChimeActions::UpdateAppInstanceUserEndpoint => {
                write!(f, "chime:UpdateAppInstanceUserEndpoint")
            }
            ChimeActions::UpdateAttendeeCapabilities => {
                write!(f, "chime:UpdateAttendeeCapabilities")
            }
            ChimeActions::UpdateBot => write!(f, "chime:UpdateBot"),
            ChimeActions::UpdateCdrSettings => write!(f, "chime:UpdateCDRSettings"),
            ChimeActions::UpdateChannel => write!(f, "chime:UpdateChannel"),
            ChimeActions::UpdateChannelFlow => write!(f, "chime:UpdateChannelFlow"),
            ChimeActions::UpdateChannelMessage => write!(f, "chime:UpdateChannelMessage"),
            ChimeActions::UpdateChannelReadMarker => write!(f, "chime:UpdateChannelReadMarker"),
            ChimeActions::UpdateGlobalSettings => write!(f, "chime:UpdateGlobalSettings"),
            ChimeActions::UpdateMediaInsightsPipelineConfiguration => {
                write!(f, "chime:UpdateMediaInsightsPipelineConfiguration")
            }
            ChimeActions::UpdateMediaInsightsPipelineStatus => {
                write!(f, "chime:UpdateMediaInsightsPipelineStatus")
            }
            ChimeActions::UpdateMediaPipelineKinesisVideoStreamPool => {
                write!(f, "chime:UpdateMediaPipelineKinesisVideoStreamPool")
            }
            ChimeActions::UpdatePhoneNumber => write!(f, "chime:UpdatePhoneNumber"),
            ChimeActions::UpdatePhoneNumberSettings => write!(f, "chime:UpdatePhoneNumberSettings"),
            ChimeActions::UpdateProxySession => write!(f, "chime:UpdateProxySession"),
            ChimeActions::UpdateRoom => write!(f, "chime:UpdateRoom"),
            ChimeActions::UpdateRoomMembership => write!(f, "chime:UpdateRoomMembership"),
            ChimeActions::UpdateSipMediaApplication => write!(f, "chime:UpdateSipMediaApplication"),
            ChimeActions::UpdateSipMediaApplicationCall => {
                write!(f, "chime:UpdateSipMediaApplicationCall")
            }
            ChimeActions::UpdateSipRule => write!(f, "chime:UpdateSipRule"),
            ChimeActions::UpdateSupportedLicenses => write!(f, "chime:UpdateSupportedLicenses"),
            ChimeActions::UpdateUser => write!(f, "chime:UpdateUser"),
            ChimeActions::UpdateUserLicenses => write!(f, "chime:UpdateUserLicenses"),
            ChimeActions::UpdateUserSettings => write!(f, "chime:UpdateUserSettings"),
            ChimeActions::UpdateVoiceConnector => write!(f, "chime:UpdateVoiceConnector"),
            ChimeActions::UpdateVoiceConnectorGroup => write!(f, "chime:UpdateVoiceConnectorGroup"),
            ChimeActions::UpdateVoiceProfile => write!(f, "chime:UpdateVoiceProfile"),
            ChimeActions::UpdateVoiceProfileDomain => write!(f, "chime:UpdateVoiceProfileDomain"),
            ChimeActions::ValidateAccountResource => write!(f, "chime:ValidateAccountResource"),
            ChimeActions::ValidateE911Address => write!(f, "chime:ValidateE911Address"),
        }
    }
}
