// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum TransformActions {
    AssociateConnectorResource,
    CreateProfile,
    DeleteConnector,
    DeleteProfile,
    GetConnector,
    ListConnectors,
    ListProfiles,
    ListTagsForResource,
    RejectConnector,
    TagResource,
    UntagResource,
    UpdateProfile,
}
impl std::fmt::Display for TransformActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransformActions::AssociateConnectorResource => {
                write!(f, "transform:AssociateConnectorResource")
            }
            TransformActions::CreateProfile => write!(f, "transform:CreateProfile"),
            TransformActions::DeleteConnector => write!(f, "transform:DeleteConnector"),
            TransformActions::DeleteProfile => write!(f, "transform:DeleteProfile"),
            TransformActions::GetConnector => write!(f, "transform:GetConnector"),
            TransformActions::ListConnectors => write!(f, "transform:ListConnectors"),
            TransformActions::ListProfiles => write!(f, "transform:ListProfiles"),
            TransformActions::ListTagsForResource => write!(f, "transform:ListTagsForResource"),
            TransformActions::RejectConnector => write!(f, "transform:RejectConnector"),
            TransformActions::TagResource => write!(f, "transform:TagResource"),
            TransformActions::UntagResource => write!(f, "transform:UntagResource"),
            TransformActions::UpdateProfile => write!(f, "transform:UpdateProfile"),
        }
    }
}
