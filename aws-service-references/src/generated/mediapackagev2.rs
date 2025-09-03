// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum Mediapackagev2Actions {
    CancelHarvestJob,
    CreateChannel,
    CreateChannelGroup,
    CreateHarvestJob,
    CreateOriginEndpoint,
    DeleteChannel,
    DeleteChannelGroup,
    DeleteChannelPolicy,
    DeleteOriginEndpoint,
    DeleteOriginEndpointPolicy,
    GetChannel,
    GetChannelGroup,
    GetChannelPolicy,
    GetHarvestJob,
    GetHeadObject,
    GetObject,
    GetOriginEndpoint,
    GetOriginEndpointPolicy,
    HarvestObject,
    ListChannelGroups,
    ListChannels,
    ListHarvestJobs,
    ListOriginEndpoints,
    ListTagsForResource,
    PutChannelPolicy,
    PutObject,
    PutOriginEndpointPolicy,
    ResetChannelState,
    ResetOriginEndpointState,
    TagResource,
    UntagResource,
    UpdateChannel,
    UpdateChannelGroup,
    UpdateOriginEndpoint,
}
impl std::fmt::Display for Mediapackagev2Actions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Mediapackagev2Actions::CancelHarvestJob => write!(f, "mediapackagev2:CancelHarvestJob"),
            Mediapackagev2Actions::CreateChannel => write!(f, "mediapackagev2:CreateChannel"),
            Mediapackagev2Actions::CreateChannelGroup => {
                write!(f, "mediapackagev2:CreateChannelGroup")
            }
            Mediapackagev2Actions::CreateHarvestJob => write!(f, "mediapackagev2:CreateHarvestJob"),
            Mediapackagev2Actions::CreateOriginEndpoint => {
                write!(f, "mediapackagev2:CreateOriginEndpoint")
            }
            Mediapackagev2Actions::DeleteChannel => write!(f, "mediapackagev2:DeleteChannel"),
            Mediapackagev2Actions::DeleteChannelGroup => {
                write!(f, "mediapackagev2:DeleteChannelGroup")
            }
            Mediapackagev2Actions::DeleteChannelPolicy => {
                write!(f, "mediapackagev2:DeleteChannelPolicy")
            }
            Mediapackagev2Actions::DeleteOriginEndpoint => {
                write!(f, "mediapackagev2:DeleteOriginEndpoint")
            }
            Mediapackagev2Actions::DeleteOriginEndpointPolicy => {
                write!(f, "mediapackagev2:DeleteOriginEndpointPolicy")
            }
            Mediapackagev2Actions::GetChannel => write!(f, "mediapackagev2:GetChannel"),
            Mediapackagev2Actions::GetChannelGroup => write!(f, "mediapackagev2:GetChannelGroup"),
            Mediapackagev2Actions::GetChannelPolicy => write!(f, "mediapackagev2:GetChannelPolicy"),
            Mediapackagev2Actions::GetHarvestJob => write!(f, "mediapackagev2:GetHarvestJob"),
            Mediapackagev2Actions::GetHeadObject => write!(f, "mediapackagev2:GetHeadObject"),
            Mediapackagev2Actions::GetObject => write!(f, "mediapackagev2:GetObject"),
            Mediapackagev2Actions::GetOriginEndpoint => {
                write!(f, "mediapackagev2:GetOriginEndpoint")
            }
            Mediapackagev2Actions::GetOriginEndpointPolicy => {
                write!(f, "mediapackagev2:GetOriginEndpointPolicy")
            }
            Mediapackagev2Actions::HarvestObject => write!(f, "mediapackagev2:HarvestObject"),
            Mediapackagev2Actions::ListChannelGroups => {
                write!(f, "mediapackagev2:ListChannelGroups")
            }
            Mediapackagev2Actions::ListChannels => write!(f, "mediapackagev2:ListChannels"),
            Mediapackagev2Actions::ListHarvestJobs => write!(f, "mediapackagev2:ListHarvestJobs"),
            Mediapackagev2Actions::ListOriginEndpoints => {
                write!(f, "mediapackagev2:ListOriginEndpoints")
            }
            Mediapackagev2Actions::ListTagsForResource => {
                write!(f, "mediapackagev2:ListTagsForResource")
            }
            Mediapackagev2Actions::PutChannelPolicy => write!(f, "mediapackagev2:PutChannelPolicy"),
            Mediapackagev2Actions::PutObject => write!(f, "mediapackagev2:PutObject"),
            Mediapackagev2Actions::PutOriginEndpointPolicy => {
                write!(f, "mediapackagev2:PutOriginEndpointPolicy")
            }
            Mediapackagev2Actions::ResetChannelState => {
                write!(f, "mediapackagev2:ResetChannelState")
            }
            Mediapackagev2Actions::ResetOriginEndpointState => {
                write!(f, "mediapackagev2:ResetOriginEndpointState")
            }
            Mediapackagev2Actions::TagResource => write!(f, "mediapackagev2:TagResource"),
            Mediapackagev2Actions::UntagResource => write!(f, "mediapackagev2:UntagResource"),
            Mediapackagev2Actions::UpdateChannel => write!(f, "mediapackagev2:UpdateChannel"),
            Mediapackagev2Actions::UpdateChannelGroup => {
                write!(f, "mediapackagev2:UpdateChannelGroup")
            }
            Mediapackagev2Actions::UpdateOriginEndpoint => {
                write!(f, "mediapackagev2:UpdateOriginEndpoint")
            }
        }
    }
}
