// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum MediatailorActions {
    ConfigureLogsForChannel,
    ConfigureLogsForPlaybackConfiguration,
    CreateChannel,
    CreateLiveSource,
    CreatePrefetchSchedule,
    CreateProgram,
    CreateSourceLocation,
    CreateVodSource,
    DeleteChannel,
    DeleteChannelPolicy,
    DeleteLiveSource,
    DeletePlaybackConfiguration,
    DeletePrefetchSchedule,
    DeleteProgram,
    DeleteSourceLocation,
    DeleteVodSource,
    DescribeChannel,
    DescribeLiveSource,
    DescribeProgram,
    DescribeSourceLocation,
    DescribeVodSource,
    GetChannelPolicy,
    GetChannelSchedule,
    GetPlaybackConfiguration,
    GetPrefetchSchedule,
    ListAlerts,
    ListChannels,
    ListLiveSources,
    ListPlaybackConfigurations,
    ListPrefetchSchedules,
    ListSourceLocations,
    ListTagsForResource,
    ListVodSources,
    PutChannelPolicy,
    PutPlaybackConfiguration,
    StartChannel,
    StopChannel,
    TagResource,
    UntagResource,
    UpdateChannel,
    UpdateLiveSource,
    UpdateProgram,
    UpdateSourceLocation,
    UpdateVodSource,
}
impl std::fmt::Display for MediatailorActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MediatailorActions::ConfigureLogsForChannel => {
                write!(f, "mediatailor:ConfigureLogsForChannel")
            }
            MediatailorActions::ConfigureLogsForPlaybackConfiguration => {
                write!(f, "mediatailor:ConfigureLogsForPlaybackConfiguration")
            }
            MediatailorActions::CreateChannel => write!(f, "mediatailor:CreateChannel"),
            MediatailorActions::CreateLiveSource => write!(f, "mediatailor:CreateLiveSource"),
            MediatailorActions::CreatePrefetchSchedule => {
                write!(f, "mediatailor:CreatePrefetchSchedule")
            }
            MediatailorActions::CreateProgram => write!(f, "mediatailor:CreateProgram"),
            MediatailorActions::CreateSourceLocation => {
                write!(f, "mediatailor:CreateSourceLocation")
            }
            MediatailorActions::CreateVodSource => write!(f, "mediatailor:CreateVodSource"),
            MediatailorActions::DeleteChannel => write!(f, "mediatailor:DeleteChannel"),
            MediatailorActions::DeleteChannelPolicy => write!(f, "mediatailor:DeleteChannelPolicy"),
            MediatailorActions::DeleteLiveSource => write!(f, "mediatailor:DeleteLiveSource"),
            MediatailorActions::DeletePlaybackConfiguration => {
                write!(f, "mediatailor:DeletePlaybackConfiguration")
            }
            MediatailorActions::DeletePrefetchSchedule => {
                write!(f, "mediatailor:DeletePrefetchSchedule")
            }
            MediatailorActions::DeleteProgram => write!(f, "mediatailor:DeleteProgram"),
            MediatailorActions::DeleteSourceLocation => {
                write!(f, "mediatailor:DeleteSourceLocation")
            }
            MediatailorActions::DeleteVodSource => write!(f, "mediatailor:DeleteVodSource"),
            MediatailorActions::DescribeChannel => write!(f, "mediatailor:DescribeChannel"),
            MediatailorActions::DescribeLiveSource => write!(f, "mediatailor:DescribeLiveSource"),
            MediatailorActions::DescribeProgram => write!(f, "mediatailor:DescribeProgram"),
            MediatailorActions::DescribeSourceLocation => {
                write!(f, "mediatailor:DescribeSourceLocation")
            }
            MediatailorActions::DescribeVodSource => write!(f, "mediatailor:DescribeVodSource"),
            MediatailorActions::GetChannelPolicy => write!(f, "mediatailor:GetChannelPolicy"),
            MediatailorActions::GetChannelSchedule => write!(f, "mediatailor:GetChannelSchedule"),
            MediatailorActions::GetPlaybackConfiguration => {
                write!(f, "mediatailor:GetPlaybackConfiguration")
            }
            MediatailorActions::GetPrefetchSchedule => write!(f, "mediatailor:GetPrefetchSchedule"),
            MediatailorActions::ListAlerts => write!(f, "mediatailor:ListAlerts"),
            MediatailorActions::ListChannels => write!(f, "mediatailor:ListChannels"),
            MediatailorActions::ListLiveSources => write!(f, "mediatailor:ListLiveSources"),
            MediatailorActions::ListPlaybackConfigurations => {
                write!(f, "mediatailor:ListPlaybackConfigurations")
            }
            MediatailorActions::ListPrefetchSchedules => {
                write!(f, "mediatailor:ListPrefetchSchedules")
            }
            MediatailorActions::ListSourceLocations => write!(f, "mediatailor:ListSourceLocations"),
            MediatailorActions::ListTagsForResource => write!(f, "mediatailor:ListTagsForResource"),
            MediatailorActions::ListVodSources => write!(f, "mediatailor:ListVodSources"),
            MediatailorActions::PutChannelPolicy => write!(f, "mediatailor:PutChannelPolicy"),
            MediatailorActions::PutPlaybackConfiguration => {
                write!(f, "mediatailor:PutPlaybackConfiguration")
            }
            MediatailorActions::StartChannel => write!(f, "mediatailor:StartChannel"),
            MediatailorActions::StopChannel => write!(f, "mediatailor:StopChannel"),
            MediatailorActions::TagResource => write!(f, "mediatailor:TagResource"),
            MediatailorActions::UntagResource => write!(f, "mediatailor:UntagResource"),
            MediatailorActions::UpdateChannel => write!(f, "mediatailor:UpdateChannel"),
            MediatailorActions::UpdateLiveSource => write!(f, "mediatailor:UpdateLiveSource"),
            MediatailorActions::UpdateProgram => write!(f, "mediatailor:UpdateProgram"),
            MediatailorActions::UpdateSourceLocation => {
                write!(f, "mediatailor:UpdateSourceLocation")
            }
            MediatailorActions::UpdateVodSource => write!(f, "mediatailor:UpdateVodSource"),
        }
    }
}
