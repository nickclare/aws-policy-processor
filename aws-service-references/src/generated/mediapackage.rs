// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum MediapackageActions {
    ConfigureLogs,
    CreateChannel,
    CreateHarvestJob,
    CreateOriginEndpoint,
    DeleteChannel,
    DeleteOriginEndpoint,
    DescribeChannel,
    DescribeHarvestJob,
    DescribeOriginEndpoint,
    ListChannels,
    ListHarvestJobs,
    ListOriginEndpoints,
    ListTagsForResource,
    RotateChannelCredentials,
    RotateIngestEndpointCredentials,
    TagResource,
    UntagResource,
    UpdateChannel,
    UpdateOriginEndpoint,
}
impl std::fmt::Display for MediapackageActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MediapackageActions::ConfigureLogs => write!(f, "mediapackage:ConfigureLogs"),
            MediapackageActions::CreateChannel => write!(f, "mediapackage:CreateChannel"),
            MediapackageActions::CreateHarvestJob => write!(f, "mediapackage:CreateHarvestJob"),
            MediapackageActions::CreateOriginEndpoint => {
                write!(f, "mediapackage:CreateOriginEndpoint")
            }
            MediapackageActions::DeleteChannel => write!(f, "mediapackage:DeleteChannel"),
            MediapackageActions::DeleteOriginEndpoint => {
                write!(f, "mediapackage:DeleteOriginEndpoint")
            }
            MediapackageActions::DescribeChannel => write!(f, "mediapackage:DescribeChannel"),
            MediapackageActions::DescribeHarvestJob => write!(f, "mediapackage:DescribeHarvestJob"),
            MediapackageActions::DescribeOriginEndpoint => {
                write!(f, "mediapackage:DescribeOriginEndpoint")
            }
            MediapackageActions::ListChannels => write!(f, "mediapackage:ListChannels"),
            MediapackageActions::ListHarvestJobs => write!(f, "mediapackage:ListHarvestJobs"),
            MediapackageActions::ListOriginEndpoints => {
                write!(f, "mediapackage:ListOriginEndpoints")
            }
            MediapackageActions::ListTagsForResource => {
                write!(f, "mediapackage:ListTagsForResource")
            }
            MediapackageActions::RotateChannelCredentials => {
                write!(f, "mediapackage:RotateChannelCredentials")
            }
            MediapackageActions::RotateIngestEndpointCredentials => {
                write!(f, "mediapackage:RotateIngestEndpointCredentials")
            }
            MediapackageActions::TagResource => write!(f, "mediapackage:TagResource"),
            MediapackageActions::UntagResource => write!(f, "mediapackage:UntagResource"),
            MediapackageActions::UpdateChannel => write!(f, "mediapackage:UpdateChannel"),
            MediapackageActions::UpdateOriginEndpoint => {
                write!(f, "mediapackage:UpdateOriginEndpoint")
            }
        }
    }
}
