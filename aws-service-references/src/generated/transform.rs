// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum TransformActions {
    AssociateConnectorResource,
    CreateProfile,
    DeleteProfile,
    GetConnector,
    ListProfiles,
    RejectConnector,
    UpdateProfile,
}
impl std::fmt::Display for TransformActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransformActions::AssociateConnectorResource => {
                write!(f, "transform:AssociateConnectorResource")
            }
            TransformActions::CreateProfile => write!(f, "transform:CreateProfile"),
            TransformActions::DeleteProfile => write!(f, "transform:DeleteProfile"),
            TransformActions::GetConnector => write!(f, "transform:GetConnector"),
            TransformActions::ListProfiles => write!(f, "transform:ListProfiles"),
            TransformActions::RejectConnector => write!(f, "transform:RejectConnector"),
            TransformActions::UpdateProfile => write!(f, "transform:UpdateProfile"),
        }
    }
}
