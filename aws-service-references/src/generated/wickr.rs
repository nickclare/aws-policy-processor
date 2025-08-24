// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum WickrActions {
    CreateAdminSession,
    CreateNetwork,
    DeleteNetwork,
    ListNetworks,
    ListTagsForResource,
    TagResource,
    UntagResource,
    UpdateNetworkDetails,
}
impl std::fmt::Display for WickrActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WickrActions::CreateAdminSession => write!(f, "wickr:CreateAdminSession"),
            WickrActions::CreateNetwork => write!(f, "wickr:CreateNetwork"),
            WickrActions::DeleteNetwork => write!(f, "wickr:DeleteNetwork"),
            WickrActions::ListNetworks => write!(f, "wickr:ListNetworks"),
            WickrActions::ListTagsForResource => write!(f, "wickr:ListTagsForResource"),
            WickrActions::TagResource => write!(f, "wickr:TagResource"),
            WickrActions::UntagResource => write!(f, "wickr:UntagResource"),
            WickrActions::UpdateNetworkDetails => write!(f, "wickr:UpdateNetworkDetails"),
        }
    }
}
