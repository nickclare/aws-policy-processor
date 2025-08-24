// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum ConnectCampaignsActions {
    CreateCampaign,
    DeleteCampaign,
    DeleteCampaignChannelSubtypeConfig,
    DeleteCampaignCommunicationLimits,
    DeleteCampaignCommunicationTime,
    DeleteConnectInstanceConfig,
    DeleteConnectInstanceIntegration,
    DeleteInstanceOnboardingJob,
    DescribeCampaign,
    GetCampaignState,
    GetCampaignStateBatch,
    GetConnectInstanceConfig,
    GetInstanceCommunicationLimits,
    GetInstanceOnboardingJobStatus,
    ListCampaigns,
    ListConnectInstanceIntegrations,
    ListTagsForResource,
    PauseCampaign,
    PutConnectInstanceIntegration,
    PutDialRequestBatch,
    PutInstanceCommunicationLimits,
    PutOutboundRequestBatch,
    PutProfileOutboundRequestBatch,
    ResumeCampaign,
    StartCampaign,
    StartInstanceOnboardingJob,
    StopCampaign,
    TagResource,
    UntagResource,
    UpdateCampaignChannelSubtypeConfig,
    UpdateCampaignCommunicationLimits,
    UpdateCampaignCommunicationTime,
    UpdateCampaignDialerConfig,
    UpdateCampaignFlowAssociation,
    UpdateCampaignName,
    UpdateCampaignOutboundCallConfig,
    UpdateCampaignSchedule,
    UpdateCampaignSource,
}
impl std::fmt::Display for ConnectCampaignsActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConnectCampaignsActions::CreateCampaign => {
                write!(f, "connect-campaigns:CreateCampaign")
            }
            ConnectCampaignsActions::DeleteCampaign => {
                write!(f, "connect-campaigns:DeleteCampaign")
            }
            ConnectCampaignsActions::DeleteCampaignChannelSubtypeConfig => {
                write!(f, "connect-campaigns:DeleteCampaignChannelSubtypeConfig")
            }
            ConnectCampaignsActions::DeleteCampaignCommunicationLimits => {
                write!(f, "connect-campaigns:DeleteCampaignCommunicationLimits")
            }
            ConnectCampaignsActions::DeleteCampaignCommunicationTime => {
                write!(f, "connect-campaigns:DeleteCampaignCommunicationTime")
            }
            ConnectCampaignsActions::DeleteConnectInstanceConfig => {
                write!(f, "connect-campaigns:DeleteConnectInstanceConfig")
            }
            ConnectCampaignsActions::DeleteConnectInstanceIntegration => {
                write!(f, "connect-campaigns:DeleteConnectInstanceIntegration")
            }
            ConnectCampaignsActions::DeleteInstanceOnboardingJob => {
                write!(f, "connect-campaigns:DeleteInstanceOnboardingJob")
            }
            ConnectCampaignsActions::DescribeCampaign => {
                write!(f, "connect-campaigns:DescribeCampaign")
            }
            ConnectCampaignsActions::GetCampaignState => {
                write!(f, "connect-campaigns:GetCampaignState")
            }
            ConnectCampaignsActions::GetCampaignStateBatch => {
                write!(f, "connect-campaigns:GetCampaignStateBatch")
            }
            ConnectCampaignsActions::GetConnectInstanceConfig => {
                write!(f, "connect-campaigns:GetConnectInstanceConfig")
            }
            ConnectCampaignsActions::GetInstanceCommunicationLimits => {
                write!(f, "connect-campaigns:GetInstanceCommunicationLimits")
            }
            ConnectCampaignsActions::GetInstanceOnboardingJobStatus => {
                write!(f, "connect-campaigns:GetInstanceOnboardingJobStatus")
            }
            ConnectCampaignsActions::ListCampaigns => write!(f, "connect-campaigns:ListCampaigns"),
            ConnectCampaignsActions::ListConnectInstanceIntegrations => {
                write!(f, "connect-campaigns:ListConnectInstanceIntegrations")
            }
            ConnectCampaignsActions::ListTagsForResource => {
                write!(f, "connect-campaigns:ListTagsForResource")
            }
            ConnectCampaignsActions::PauseCampaign => write!(f, "connect-campaigns:PauseCampaign"),
            ConnectCampaignsActions::PutConnectInstanceIntegration => {
                write!(f, "connect-campaigns:PutConnectInstanceIntegration")
            }
            ConnectCampaignsActions::PutDialRequestBatch => {
                write!(f, "connect-campaigns:PutDialRequestBatch")
            }
            ConnectCampaignsActions::PutInstanceCommunicationLimits => {
                write!(f, "connect-campaigns:PutInstanceCommunicationLimits")
            }
            ConnectCampaignsActions::PutOutboundRequestBatch => {
                write!(f, "connect-campaigns:PutOutboundRequestBatch")
            }
            ConnectCampaignsActions::PutProfileOutboundRequestBatch => {
                write!(f, "connect-campaigns:PutProfileOutboundRequestBatch")
            }
            ConnectCampaignsActions::ResumeCampaign => {
                write!(f, "connect-campaigns:ResumeCampaign")
            }
            ConnectCampaignsActions::StartCampaign => write!(f, "connect-campaigns:StartCampaign"),
            ConnectCampaignsActions::StartInstanceOnboardingJob => {
                write!(f, "connect-campaigns:StartInstanceOnboardingJob")
            }
            ConnectCampaignsActions::StopCampaign => write!(f, "connect-campaigns:StopCampaign"),
            ConnectCampaignsActions::TagResource => write!(f, "connect-campaigns:TagResource"),
            ConnectCampaignsActions::UntagResource => write!(f, "connect-campaigns:UntagResource"),
            ConnectCampaignsActions::UpdateCampaignChannelSubtypeConfig => {
                write!(f, "connect-campaigns:UpdateCampaignChannelSubtypeConfig")
            }
            ConnectCampaignsActions::UpdateCampaignCommunicationLimits => {
                write!(f, "connect-campaigns:UpdateCampaignCommunicationLimits")
            }
            ConnectCampaignsActions::UpdateCampaignCommunicationTime => {
                write!(f, "connect-campaigns:UpdateCampaignCommunicationTime")
            }
            ConnectCampaignsActions::UpdateCampaignDialerConfig => {
                write!(f, "connect-campaigns:UpdateCampaignDialerConfig")
            }
            ConnectCampaignsActions::UpdateCampaignFlowAssociation => {
                write!(f, "connect-campaigns:UpdateCampaignFlowAssociation")
            }
            ConnectCampaignsActions::UpdateCampaignName => {
                write!(f, "connect-campaigns:UpdateCampaignName")
            }
            ConnectCampaignsActions::UpdateCampaignOutboundCallConfig => {
                write!(f, "connect-campaigns:UpdateCampaignOutboundCallConfig")
            }
            ConnectCampaignsActions::UpdateCampaignSchedule => {
                write!(f, "connect-campaigns:UpdateCampaignSchedule")
            }
            ConnectCampaignsActions::UpdateCampaignSource => {
                write!(f, "connect-campaigns:UpdateCampaignSource")
            }
        }
    }
}
