// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum MediapackageVodActions {
    ConfigureLogs,
    CreateAsset,
    CreatePackagingConfiguration,
    CreatePackagingGroup,
    DeleteAsset,
    DeletePackagingConfiguration,
    DeletePackagingGroup,
    DescribeAsset,
    DescribePackagingConfiguration,
    DescribePackagingGroup,
    ListAssets,
    ListPackagingConfigurations,
    ListPackagingGroups,
    ListTagsForResource,
    TagResource,
    UntagResource,
    UpdatePackagingGroup,
}
impl std::fmt::Display for MediapackageVodActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MediapackageVodActions::ConfigureLogs => write!(f, "mediapackage-vod:ConfigureLogs"),
            MediapackageVodActions::CreateAsset => write!(f, "mediapackage-vod:CreateAsset"),
            MediapackageVodActions::CreatePackagingConfiguration => {
                write!(f, "mediapackage-vod:CreatePackagingConfiguration")
            }
            MediapackageVodActions::CreatePackagingGroup => {
                write!(f, "mediapackage-vod:CreatePackagingGroup")
            }
            MediapackageVodActions::DeleteAsset => write!(f, "mediapackage-vod:DeleteAsset"),
            MediapackageVodActions::DeletePackagingConfiguration => {
                write!(f, "mediapackage-vod:DeletePackagingConfiguration")
            }
            MediapackageVodActions::DeletePackagingGroup => {
                write!(f, "mediapackage-vod:DeletePackagingGroup")
            }
            MediapackageVodActions::DescribeAsset => write!(f, "mediapackage-vod:DescribeAsset"),
            MediapackageVodActions::DescribePackagingConfiguration => {
                write!(f, "mediapackage-vod:DescribePackagingConfiguration")
            }
            MediapackageVodActions::DescribePackagingGroup => {
                write!(f, "mediapackage-vod:DescribePackagingGroup")
            }
            MediapackageVodActions::ListAssets => write!(f, "mediapackage-vod:ListAssets"),
            MediapackageVodActions::ListPackagingConfigurations => {
                write!(f, "mediapackage-vod:ListPackagingConfigurations")
            }
            MediapackageVodActions::ListPackagingGroups => {
                write!(f, "mediapackage-vod:ListPackagingGroups")
            }
            MediapackageVodActions::ListTagsForResource => {
                write!(f, "mediapackage-vod:ListTagsForResource")
            }
            MediapackageVodActions::TagResource => write!(f, "mediapackage-vod:TagResource"),
            MediapackageVodActions::UntagResource => write!(f, "mediapackage-vod:UntagResource"),
            MediapackageVodActions::UpdatePackagingGroup => {
                write!(f, "mediapackage-vod:UpdatePackagingGroup")
            }
        }
    }
}
