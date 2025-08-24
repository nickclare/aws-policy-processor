// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum RolesanywhereActions {
    CreateProfile,
    CreateTrustAnchor,
    DeleteAttributeMapping,
    DeleteCrl,
    DeleteProfile,
    DeleteTrustAnchor,
    DisableCrl,
    DisableProfile,
    DisableTrustAnchor,
    EnableCrl,
    EnableProfile,
    EnableTrustAnchor,
    GetCrl,
    GetProfile,
    GetSubject,
    GetTrustAnchor,
    ImportCrl,
    ListCrls,
    ListProfiles,
    ListSubjects,
    ListTagsForResource,
    ListTrustAnchors,
    PutAttributeMapping,
    PutNotificationSettings,
    ResetNotificationSettings,
    TagResource,
    UntagResource,
    UpdateCrl,
    UpdateProfile,
    UpdateTrustAnchor,
}
impl std::fmt::Display for RolesanywhereActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RolesanywhereActions::CreateProfile => write!(f, "rolesanywhere:CreateProfile"),
            RolesanywhereActions::CreateTrustAnchor => write!(f, "rolesanywhere:CreateTrustAnchor"),
            RolesanywhereActions::DeleteAttributeMapping => {
                write!(f, "rolesanywhere:DeleteAttributeMapping")
            }
            RolesanywhereActions::DeleteCrl => write!(f, "rolesanywhere:DeleteCrl"),
            RolesanywhereActions::DeleteProfile => write!(f, "rolesanywhere:DeleteProfile"),
            RolesanywhereActions::DeleteTrustAnchor => write!(f, "rolesanywhere:DeleteTrustAnchor"),
            RolesanywhereActions::DisableCrl => write!(f, "rolesanywhere:DisableCrl"),
            RolesanywhereActions::DisableProfile => write!(f, "rolesanywhere:DisableProfile"),
            RolesanywhereActions::DisableTrustAnchor => {
                write!(f, "rolesanywhere:DisableTrustAnchor")
            }
            RolesanywhereActions::EnableCrl => write!(f, "rolesanywhere:EnableCrl"),
            RolesanywhereActions::EnableProfile => write!(f, "rolesanywhere:EnableProfile"),
            RolesanywhereActions::EnableTrustAnchor => write!(f, "rolesanywhere:EnableTrustAnchor"),
            RolesanywhereActions::GetCrl => write!(f, "rolesanywhere:GetCrl"),
            RolesanywhereActions::GetProfile => write!(f, "rolesanywhere:GetProfile"),
            RolesanywhereActions::GetSubject => write!(f, "rolesanywhere:GetSubject"),
            RolesanywhereActions::GetTrustAnchor => write!(f, "rolesanywhere:GetTrustAnchor"),
            RolesanywhereActions::ImportCrl => write!(f, "rolesanywhere:ImportCrl"),
            RolesanywhereActions::ListCrls => write!(f, "rolesanywhere:ListCrls"),
            RolesanywhereActions::ListProfiles => write!(f, "rolesanywhere:ListProfiles"),
            RolesanywhereActions::ListSubjects => write!(f, "rolesanywhere:ListSubjects"),
            RolesanywhereActions::ListTagsForResource => {
                write!(f, "rolesanywhere:ListTagsForResource")
            }
            RolesanywhereActions::ListTrustAnchors => write!(f, "rolesanywhere:ListTrustAnchors"),
            RolesanywhereActions::PutAttributeMapping => {
                write!(f, "rolesanywhere:PutAttributeMapping")
            }
            RolesanywhereActions::PutNotificationSettings => {
                write!(f, "rolesanywhere:PutNotificationSettings")
            }
            RolesanywhereActions::ResetNotificationSettings => {
                write!(f, "rolesanywhere:ResetNotificationSettings")
            }
            RolesanywhereActions::TagResource => write!(f, "rolesanywhere:TagResource"),
            RolesanywhereActions::UntagResource => write!(f, "rolesanywhere:UntagResource"),
            RolesanywhereActions::UpdateCrl => write!(f, "rolesanywhere:UpdateCrl"),
            RolesanywhereActions::UpdateProfile => write!(f, "rolesanywhere:UpdateProfile"),
            RolesanywhereActions::UpdateTrustAnchor => write!(f, "rolesanywhere:UpdateTrustAnchor"),
        }
    }
}
