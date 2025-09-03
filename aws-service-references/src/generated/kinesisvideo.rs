// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum KinesisvideoActions {
    ConnectAsMaster,
    ConnectAsViewer,
    CreateSignalingChannel,
    CreateStream,
    DeleteEdgeConfiguration,
    DeleteSignalingChannel,
    DeleteStream,
    DescribeEdgeConfiguration,
    DescribeImageGenerationConfiguration,
    DescribeMappedResourceConfiguration,
    DescribeMediaStorageConfiguration,
    DescribeNotificationConfiguration,
    DescribeSignalingChannel,
    DescribeStream,
    GetClip,
    GetDashStreamingSessionUrl,
    GetDataEndpoint,
    GetHlsStreamingSessionUrl,
    GetIceServerConfig,
    GetImages,
    GetMedia,
    GetMediaForFragmentList,
    GetSignalingChannelEndpoint,
    JoinStorageSession,
    JoinStorageSessionAsViewer,
    ListEdgeAgentConfigurations,
    ListFragments,
    ListSignalingChannels,
    ListStreams,
    ListTagsForResource,
    ListTagsForStream,
    PutMedia,
    SendAlexaOfferToMaster,
    StartEdgeConfigurationUpdate,
    TagResource,
    TagStream,
    UntagResource,
    UntagStream,
    UpdateDataRetention,
    UpdateImageGenerationConfiguration,
    UpdateMediaStorageConfiguration,
    UpdateNotificationConfiguration,
    UpdateSignalingChannel,
    UpdateStream,
}
impl std::fmt::Display for KinesisvideoActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KinesisvideoActions::ConnectAsMaster => write!(f, "kinesisvideo:ConnectAsMaster"),
            KinesisvideoActions::ConnectAsViewer => write!(f, "kinesisvideo:ConnectAsViewer"),
            KinesisvideoActions::CreateSignalingChannel => {
                write!(f, "kinesisvideo:CreateSignalingChannel")
            }
            KinesisvideoActions::CreateStream => write!(f, "kinesisvideo:CreateStream"),
            KinesisvideoActions::DeleteEdgeConfiguration => {
                write!(f, "kinesisvideo:DeleteEdgeConfiguration")
            }
            KinesisvideoActions::DeleteSignalingChannel => {
                write!(f, "kinesisvideo:DeleteSignalingChannel")
            }
            KinesisvideoActions::DeleteStream => write!(f, "kinesisvideo:DeleteStream"),
            KinesisvideoActions::DescribeEdgeConfiguration => {
                write!(f, "kinesisvideo:DescribeEdgeConfiguration")
            }
            KinesisvideoActions::DescribeImageGenerationConfiguration => {
                write!(f, "kinesisvideo:DescribeImageGenerationConfiguration")
            }
            KinesisvideoActions::DescribeMappedResourceConfiguration => {
                write!(f, "kinesisvideo:DescribeMappedResourceConfiguration")
            }
            KinesisvideoActions::DescribeMediaStorageConfiguration => {
                write!(f, "kinesisvideo:DescribeMediaStorageConfiguration")
            }
            KinesisvideoActions::DescribeNotificationConfiguration => {
                write!(f, "kinesisvideo:DescribeNotificationConfiguration")
            }
            KinesisvideoActions::DescribeSignalingChannel => {
                write!(f, "kinesisvideo:DescribeSignalingChannel")
            }
            KinesisvideoActions::DescribeStream => write!(f, "kinesisvideo:DescribeStream"),
            KinesisvideoActions::GetClip => write!(f, "kinesisvideo:GetClip"),
            KinesisvideoActions::GetDashStreamingSessionUrl => {
                write!(f, "kinesisvideo:GetDASHStreamingSessionURL")
            }
            KinesisvideoActions::GetDataEndpoint => write!(f, "kinesisvideo:GetDataEndpoint"),
            KinesisvideoActions::GetHlsStreamingSessionUrl => {
                write!(f, "kinesisvideo:GetHLSStreamingSessionURL")
            }
            KinesisvideoActions::GetIceServerConfig => write!(f, "kinesisvideo:GetIceServerConfig"),
            KinesisvideoActions::GetImages => write!(f, "kinesisvideo:GetImages"),
            KinesisvideoActions::GetMedia => write!(f, "kinesisvideo:GetMedia"),
            KinesisvideoActions::GetMediaForFragmentList => {
                write!(f, "kinesisvideo:GetMediaForFragmentList")
            }
            KinesisvideoActions::GetSignalingChannelEndpoint => {
                write!(f, "kinesisvideo:GetSignalingChannelEndpoint")
            }
            KinesisvideoActions::JoinStorageSession => write!(f, "kinesisvideo:JoinStorageSession"),
            KinesisvideoActions::JoinStorageSessionAsViewer => {
                write!(f, "kinesisvideo:JoinStorageSessionAsViewer")
            }
            KinesisvideoActions::ListEdgeAgentConfigurations => {
                write!(f, "kinesisvideo:ListEdgeAgentConfigurations")
            }
            KinesisvideoActions::ListFragments => write!(f, "kinesisvideo:ListFragments"),
            KinesisvideoActions::ListSignalingChannels => {
                write!(f, "kinesisvideo:ListSignalingChannels")
            }
            KinesisvideoActions::ListStreams => write!(f, "kinesisvideo:ListStreams"),
            KinesisvideoActions::ListTagsForResource => {
                write!(f, "kinesisvideo:ListTagsForResource")
            }
            KinesisvideoActions::ListTagsForStream => write!(f, "kinesisvideo:ListTagsForStream"),
            KinesisvideoActions::PutMedia => write!(f, "kinesisvideo:PutMedia"),
            KinesisvideoActions::SendAlexaOfferToMaster => {
                write!(f, "kinesisvideo:SendAlexaOfferToMaster")
            }
            KinesisvideoActions::StartEdgeConfigurationUpdate => {
                write!(f, "kinesisvideo:StartEdgeConfigurationUpdate")
            }
            KinesisvideoActions::TagResource => write!(f, "kinesisvideo:TagResource"),
            KinesisvideoActions::TagStream => write!(f, "kinesisvideo:TagStream"),
            KinesisvideoActions::UntagResource => write!(f, "kinesisvideo:UntagResource"),
            KinesisvideoActions::UntagStream => write!(f, "kinesisvideo:UntagStream"),
            KinesisvideoActions::UpdateDataRetention => {
                write!(f, "kinesisvideo:UpdateDataRetention")
            }
            KinesisvideoActions::UpdateImageGenerationConfiguration => {
                write!(f, "kinesisvideo:UpdateImageGenerationConfiguration")
            }
            KinesisvideoActions::UpdateMediaStorageConfiguration => {
                write!(f, "kinesisvideo:UpdateMediaStorageConfiguration")
            }
            KinesisvideoActions::UpdateNotificationConfiguration => {
                write!(f, "kinesisvideo:UpdateNotificationConfiguration")
            }
            KinesisvideoActions::UpdateSignalingChannel => {
                write!(f, "kinesisvideo:UpdateSignalingChannel")
            }
            KinesisvideoActions::UpdateStream => write!(f, "kinesisvideo:UpdateStream"),
        }
    }
}
