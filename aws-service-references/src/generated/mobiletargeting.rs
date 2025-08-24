// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum MobiletargetingActions {
    CreateApp,
    CreateCampaign,
    CreateEmailTemplate,
    CreateExportJob,
    CreateImportJob,
    CreateInAppTemplate,
    CreateJourney,
    CreatePushTemplate,
    CreateRecommenderConfiguration,
    CreateSegment,
    CreateSmsTemplate,
    CreateVoiceTemplate,
    DeleteAdmChannel,
    DeleteApnsChannel,
    DeleteApnsSandboxChannel,
    DeleteApnsVoipChannel,
    DeleteApnsVoipSandboxChannel,
    DeleteApp,
    DeleteBaiduChannel,
    DeleteCampaign,
    DeleteEmailChannel,
    DeleteEmailTemplate,
    DeleteEndpoint,
    DeleteEventStream,
    DeleteGcmChannel,
    DeleteInAppTemplate,
    DeleteJourney,
    DeletePushTemplate,
    DeleteRecommenderConfiguration,
    DeleteSegment,
    DeleteSmsChannel,
    DeleteSmsTemplate,
    DeleteUserEndpoints,
    DeleteVoiceChannel,
    DeleteVoiceTemplate,
    GetAdmChannel,
    GetApnsChannel,
    GetApnsSandboxChannel,
    GetApnsVoipChannel,
    GetApnsVoipSandboxChannel,
    GetApp,
    GetApplicationDateRangeKpi,
    GetApplicationSettings,
    GetApps,
    GetBaiduChannel,
    GetCampaign,
    GetCampaignActivities,
    GetCampaignDateRangeKpi,
    GetCampaignVersion,
    GetCampaignVersions,
    GetCampaigns,
    GetChannels,
    GetEmailChannel,
    GetEmailTemplate,
    GetEndpoint,
    GetEventStream,
    GetExportJob,
    GetExportJobs,
    GetGcmChannel,
    GetImportJob,
    GetImportJobs,
    GetInAppMessages,
    GetInAppTemplate,
    GetJourney,
    GetJourneyDateRangeKpi,
    GetJourneyExecutionActivityMetrics,
    GetJourneyExecutionMetrics,
    GetJourneyRunExecutionActivityMetrics,
    GetJourneyRunExecutionMetrics,
    GetJourneyRuns,
    GetPushTemplate,
    GetRecommenderConfiguration,
    GetRecommenderConfigurations,
    GetReports,
    GetSegment,
    GetSegmentExportJobs,
    GetSegmentImportJobs,
    GetSegmentVersion,
    GetSegmentVersions,
    GetSegments,
    GetSmsChannel,
    GetSmsTemplate,
    GetUserEndpoints,
    GetVoiceChannel,
    GetVoiceTemplate,
    ListJourneys,
    ListTagsForResource,
    ListTemplateVersions,
    ListTemplates,
    PhoneNumberValidate,
    PutEventStream,
    PutEvents,
    RemoveAttributes,
    SendMessages,
    SendOtpMessage,
    SendUsersMessages,
    TagResource,
    UntagResource,
    UpdateAdmChannel,
    UpdateApnsChannel,
    UpdateApnsSandboxChannel,
    UpdateApnsVoipChannel,
    UpdateApnsVoipSandboxChannel,
    UpdateApplicationSettings,
    UpdateBaiduChannel,
    UpdateCampaign,
    UpdateEmailChannel,
    UpdateEmailTemplate,
    UpdateEndpoint,
    UpdateEndpointsBatch,
    UpdateGcmChannel,
    UpdateInAppTemplate,
    UpdateJourney,
    UpdateJourneyState,
    UpdatePushTemplate,
    UpdateRecommenderConfiguration,
    UpdateSegment,
    UpdateSmsChannel,
    UpdateSmsTemplate,
    UpdateTemplateActiveVersion,
    UpdateVoiceChannel,
    UpdateVoiceTemplate,
    VerifyOtpMessage,
}
impl std::fmt::Display for MobiletargetingActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MobiletargetingActions::CreateApp => write!(f, "mobiletargeting:CreateApp"),
            MobiletargetingActions::CreateCampaign => write!(f, "mobiletargeting:CreateCampaign"),
            MobiletargetingActions::CreateEmailTemplate => {
                write!(f, "mobiletargeting:CreateEmailTemplate")
            }
            MobiletargetingActions::CreateExportJob => write!(f, "mobiletargeting:CreateExportJob"),
            MobiletargetingActions::CreateImportJob => write!(f, "mobiletargeting:CreateImportJob"),
            MobiletargetingActions::CreateInAppTemplate => {
                write!(f, "mobiletargeting:CreateInAppTemplate")
            }
            MobiletargetingActions::CreateJourney => write!(f, "mobiletargeting:CreateJourney"),
            MobiletargetingActions::CreatePushTemplate => {
                write!(f, "mobiletargeting:CreatePushTemplate")
            }
            MobiletargetingActions::CreateRecommenderConfiguration => {
                write!(f, "mobiletargeting:CreateRecommenderConfiguration")
            }
            MobiletargetingActions::CreateSegment => write!(f, "mobiletargeting:CreateSegment"),
            MobiletargetingActions::CreateSmsTemplate => {
                write!(f, "mobiletargeting:CreateSmsTemplate")
            }
            MobiletargetingActions::CreateVoiceTemplate => {
                write!(f, "mobiletargeting:CreateVoiceTemplate")
            }
            MobiletargetingActions::DeleteAdmChannel => {
                write!(f, "mobiletargeting:DeleteAdmChannel")
            }
            MobiletargetingActions::DeleteApnsChannel => {
                write!(f, "mobiletargeting:DeleteApnsChannel")
            }
            MobiletargetingActions::DeleteApnsSandboxChannel => {
                write!(f, "mobiletargeting:DeleteApnsSandboxChannel")
            }
            MobiletargetingActions::DeleteApnsVoipChannel => {
                write!(f, "mobiletargeting:DeleteApnsVoipChannel")
            }
            MobiletargetingActions::DeleteApnsVoipSandboxChannel => {
                write!(f, "mobiletargeting:DeleteApnsVoipSandboxChannel")
            }
            MobiletargetingActions::DeleteApp => write!(f, "mobiletargeting:DeleteApp"),
            MobiletargetingActions::DeleteBaiduChannel => {
                write!(f, "mobiletargeting:DeleteBaiduChannel")
            }
            MobiletargetingActions::DeleteCampaign => write!(f, "mobiletargeting:DeleteCampaign"),
            MobiletargetingActions::DeleteEmailChannel => {
                write!(f, "mobiletargeting:DeleteEmailChannel")
            }
            MobiletargetingActions::DeleteEmailTemplate => {
                write!(f, "mobiletargeting:DeleteEmailTemplate")
            }
            MobiletargetingActions::DeleteEndpoint => write!(f, "mobiletargeting:DeleteEndpoint"),
            MobiletargetingActions::DeleteEventStream => {
                write!(f, "mobiletargeting:DeleteEventStream")
            }
            MobiletargetingActions::DeleteGcmChannel => {
                write!(f, "mobiletargeting:DeleteGcmChannel")
            }
            MobiletargetingActions::DeleteInAppTemplate => {
                write!(f, "mobiletargeting:DeleteInAppTemplate")
            }
            MobiletargetingActions::DeleteJourney => write!(f, "mobiletargeting:DeleteJourney"),
            MobiletargetingActions::DeletePushTemplate => {
                write!(f, "mobiletargeting:DeletePushTemplate")
            }
            MobiletargetingActions::DeleteRecommenderConfiguration => {
                write!(f, "mobiletargeting:DeleteRecommenderConfiguration")
            }
            MobiletargetingActions::DeleteSegment => write!(f, "mobiletargeting:DeleteSegment"),
            MobiletargetingActions::DeleteSmsChannel => {
                write!(f, "mobiletargeting:DeleteSmsChannel")
            }
            MobiletargetingActions::DeleteSmsTemplate => {
                write!(f, "mobiletargeting:DeleteSmsTemplate")
            }
            MobiletargetingActions::DeleteUserEndpoints => {
                write!(f, "mobiletargeting:DeleteUserEndpoints")
            }
            MobiletargetingActions::DeleteVoiceChannel => {
                write!(f, "mobiletargeting:DeleteVoiceChannel")
            }
            MobiletargetingActions::DeleteVoiceTemplate => {
                write!(f, "mobiletargeting:DeleteVoiceTemplate")
            }
            MobiletargetingActions::GetAdmChannel => write!(f, "mobiletargeting:GetAdmChannel"),
            MobiletargetingActions::GetApnsChannel => write!(f, "mobiletargeting:GetApnsChannel"),
            MobiletargetingActions::GetApnsSandboxChannel => {
                write!(f, "mobiletargeting:GetApnsSandboxChannel")
            }
            MobiletargetingActions::GetApnsVoipChannel => {
                write!(f, "mobiletargeting:GetApnsVoipChannel")
            }
            MobiletargetingActions::GetApnsVoipSandboxChannel => {
                write!(f, "mobiletargeting:GetApnsVoipSandboxChannel")
            }
            MobiletargetingActions::GetApp => write!(f, "mobiletargeting:GetApp"),
            MobiletargetingActions::GetApplicationDateRangeKpi => {
                write!(f, "mobiletargeting:GetApplicationDateRangeKpi")
            }
            MobiletargetingActions::GetApplicationSettings => {
                write!(f, "mobiletargeting:GetApplicationSettings")
            }
            MobiletargetingActions::GetApps => write!(f, "mobiletargeting:GetApps"),
            MobiletargetingActions::GetBaiduChannel => write!(f, "mobiletargeting:GetBaiduChannel"),
            MobiletargetingActions::GetCampaign => write!(f, "mobiletargeting:GetCampaign"),
            MobiletargetingActions::GetCampaignActivities => {
                write!(f, "mobiletargeting:GetCampaignActivities")
            }
            MobiletargetingActions::GetCampaignDateRangeKpi => {
                write!(f, "mobiletargeting:GetCampaignDateRangeKpi")
            }
            MobiletargetingActions::GetCampaignVersion => {
                write!(f, "mobiletargeting:GetCampaignVersion")
            }
            MobiletargetingActions::GetCampaignVersions => {
                write!(f, "mobiletargeting:GetCampaignVersions")
            }
            MobiletargetingActions::GetCampaigns => write!(f, "mobiletargeting:GetCampaigns"),
            MobiletargetingActions::GetChannels => write!(f, "mobiletargeting:GetChannels"),
            MobiletargetingActions::GetEmailChannel => write!(f, "mobiletargeting:GetEmailChannel"),
            MobiletargetingActions::GetEmailTemplate => {
                write!(f, "mobiletargeting:GetEmailTemplate")
            }
            MobiletargetingActions::GetEndpoint => write!(f, "mobiletargeting:GetEndpoint"),
            MobiletargetingActions::GetEventStream => write!(f, "mobiletargeting:GetEventStream"),
            MobiletargetingActions::GetExportJob => write!(f, "mobiletargeting:GetExportJob"),
            MobiletargetingActions::GetExportJobs => write!(f, "mobiletargeting:GetExportJobs"),
            MobiletargetingActions::GetGcmChannel => write!(f, "mobiletargeting:GetGcmChannel"),
            MobiletargetingActions::GetImportJob => write!(f, "mobiletargeting:GetImportJob"),
            MobiletargetingActions::GetImportJobs => write!(f, "mobiletargeting:GetImportJobs"),
            MobiletargetingActions::GetInAppMessages => {
                write!(f, "mobiletargeting:GetInAppMessages")
            }
            MobiletargetingActions::GetInAppTemplate => {
                write!(f, "mobiletargeting:GetInAppTemplate")
            }
            MobiletargetingActions::GetJourney => write!(f, "mobiletargeting:GetJourney"),
            MobiletargetingActions::GetJourneyDateRangeKpi => {
                write!(f, "mobiletargeting:GetJourneyDateRangeKpi")
            }
            MobiletargetingActions::GetJourneyExecutionActivityMetrics => {
                write!(f, "mobiletargeting:GetJourneyExecutionActivityMetrics")
            }
            MobiletargetingActions::GetJourneyExecutionMetrics => {
                write!(f, "mobiletargeting:GetJourneyExecutionMetrics")
            }
            MobiletargetingActions::GetJourneyRunExecutionActivityMetrics => {
                write!(f, "mobiletargeting:GetJourneyRunExecutionActivityMetrics")
            }
            MobiletargetingActions::GetJourneyRunExecutionMetrics => {
                write!(f, "mobiletargeting:GetJourneyRunExecutionMetrics")
            }
            MobiletargetingActions::GetJourneyRuns => write!(f, "mobiletargeting:GetJourneyRuns"),
            MobiletargetingActions::GetPushTemplate => write!(f, "mobiletargeting:GetPushTemplate"),
            MobiletargetingActions::GetRecommenderConfiguration => {
                write!(f, "mobiletargeting:GetRecommenderConfiguration")
            }
            MobiletargetingActions::GetRecommenderConfigurations => {
                write!(f, "mobiletargeting:GetRecommenderConfigurations")
            }
            MobiletargetingActions::GetReports => write!(f, "mobiletargeting:GetReports"),
            MobiletargetingActions::GetSegment => write!(f, "mobiletargeting:GetSegment"),
            MobiletargetingActions::GetSegmentExportJobs => {
                write!(f, "mobiletargeting:GetSegmentExportJobs")
            }
            MobiletargetingActions::GetSegmentImportJobs => {
                write!(f, "mobiletargeting:GetSegmentImportJobs")
            }
            MobiletargetingActions::GetSegmentVersion => {
                write!(f, "mobiletargeting:GetSegmentVersion")
            }
            MobiletargetingActions::GetSegmentVersions => {
                write!(f, "mobiletargeting:GetSegmentVersions")
            }
            MobiletargetingActions::GetSegments => write!(f, "mobiletargeting:GetSegments"),
            MobiletargetingActions::GetSmsChannel => write!(f, "mobiletargeting:GetSmsChannel"),
            MobiletargetingActions::GetSmsTemplate => write!(f, "mobiletargeting:GetSmsTemplate"),
            MobiletargetingActions::GetUserEndpoints => {
                write!(f, "mobiletargeting:GetUserEndpoints")
            }
            MobiletargetingActions::GetVoiceChannel => write!(f, "mobiletargeting:GetVoiceChannel"),
            MobiletargetingActions::GetVoiceTemplate => {
                write!(f, "mobiletargeting:GetVoiceTemplate")
            }
            MobiletargetingActions::ListJourneys => write!(f, "mobiletargeting:ListJourneys"),
            MobiletargetingActions::ListTagsForResource => {
                write!(f, "mobiletargeting:ListTagsForResource")
            }
            MobiletargetingActions::ListTemplateVersions => {
                write!(f, "mobiletargeting:ListTemplateVersions")
            }
            MobiletargetingActions::ListTemplates => write!(f, "mobiletargeting:ListTemplates"),
            MobiletargetingActions::PhoneNumberValidate => {
                write!(f, "mobiletargeting:PhoneNumberValidate")
            }
            MobiletargetingActions::PutEventStream => write!(f, "mobiletargeting:PutEventStream"),
            MobiletargetingActions::PutEvents => write!(f, "mobiletargeting:PutEvents"),
            MobiletargetingActions::RemoveAttributes => {
                write!(f, "mobiletargeting:RemoveAttributes")
            }
            MobiletargetingActions::SendMessages => write!(f, "mobiletargeting:SendMessages"),
            MobiletargetingActions::SendOtpMessage => write!(f, "mobiletargeting:SendOTPMessage"),
            MobiletargetingActions::SendUsersMessages => {
                write!(f, "mobiletargeting:SendUsersMessages")
            }
            MobiletargetingActions::TagResource => write!(f, "mobiletargeting:TagResource"),
            MobiletargetingActions::UntagResource => write!(f, "mobiletargeting:UntagResource"),
            MobiletargetingActions::UpdateAdmChannel => {
                write!(f, "mobiletargeting:UpdateAdmChannel")
            }
            MobiletargetingActions::UpdateApnsChannel => {
                write!(f, "mobiletargeting:UpdateApnsChannel")
            }
            MobiletargetingActions::UpdateApnsSandboxChannel => {
                write!(f, "mobiletargeting:UpdateApnsSandboxChannel")
            }
            MobiletargetingActions::UpdateApnsVoipChannel => {
                write!(f, "mobiletargeting:UpdateApnsVoipChannel")
            }
            MobiletargetingActions::UpdateApnsVoipSandboxChannel => {
                write!(f, "mobiletargeting:UpdateApnsVoipSandboxChannel")
            }
            MobiletargetingActions::UpdateApplicationSettings => {
                write!(f, "mobiletargeting:UpdateApplicationSettings")
            }
            MobiletargetingActions::UpdateBaiduChannel => {
                write!(f, "mobiletargeting:UpdateBaiduChannel")
            }
            MobiletargetingActions::UpdateCampaign => write!(f, "mobiletargeting:UpdateCampaign"),
            MobiletargetingActions::UpdateEmailChannel => {
                write!(f, "mobiletargeting:UpdateEmailChannel")
            }
            MobiletargetingActions::UpdateEmailTemplate => {
                write!(f, "mobiletargeting:UpdateEmailTemplate")
            }
            MobiletargetingActions::UpdateEndpoint => write!(f, "mobiletargeting:UpdateEndpoint"),
            MobiletargetingActions::UpdateEndpointsBatch => {
                write!(f, "mobiletargeting:UpdateEndpointsBatch")
            }
            MobiletargetingActions::UpdateGcmChannel => {
                write!(f, "mobiletargeting:UpdateGcmChannel")
            }
            MobiletargetingActions::UpdateInAppTemplate => {
                write!(f, "mobiletargeting:UpdateInAppTemplate")
            }
            MobiletargetingActions::UpdateJourney => write!(f, "mobiletargeting:UpdateJourney"),
            MobiletargetingActions::UpdateJourneyState => {
                write!(f, "mobiletargeting:UpdateJourneyState")
            }
            MobiletargetingActions::UpdatePushTemplate => {
                write!(f, "mobiletargeting:UpdatePushTemplate")
            }
            MobiletargetingActions::UpdateRecommenderConfiguration => {
                write!(f, "mobiletargeting:UpdateRecommenderConfiguration")
            }
            MobiletargetingActions::UpdateSegment => write!(f, "mobiletargeting:UpdateSegment"),
            MobiletargetingActions::UpdateSmsChannel => {
                write!(f, "mobiletargeting:UpdateSmsChannel")
            }
            MobiletargetingActions::UpdateSmsTemplate => {
                write!(f, "mobiletargeting:UpdateSmsTemplate")
            }
            MobiletargetingActions::UpdateTemplateActiveVersion => {
                write!(f, "mobiletargeting:UpdateTemplateActiveVersion")
            }
            MobiletargetingActions::UpdateVoiceChannel => {
                write!(f, "mobiletargeting:UpdateVoiceChannel")
            }
            MobiletargetingActions::UpdateVoiceTemplate => {
                write!(f, "mobiletargeting:UpdateVoiceTemplate")
            }
            MobiletargetingActions::VerifyOtpMessage => {
                write!(f, "mobiletargeting:VerifyOTPMessage")
            }
        }
    }
}
