// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum CodewhispererActions {
    AllowVendedLogDeliveryForResource,
    AssociateCustomizationPermission,
    CreateCustomization,
    CreateProfile,
    DeleteCustomization,
    DeleteProfile,
    DisassociateCustomizationPermission,
    GenerateRecommendations,
    GetCustomization,
    ListCustomizationPermissions,
    ListCustomizationVersions,
    ListCustomizations,
    ListProfiles,
    ListTagsForResource,
    TagResource,
    UntagResource,
    UpdateCustomization,
    UpdateProfile,
}
impl std::fmt::Display for CodewhispererActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CodewhispererActions::AllowVendedLogDeliveryForResource => {
                write!(f, "codewhisperer:AllowVendedLogDeliveryForResource")
            }
            CodewhispererActions::AssociateCustomizationPermission => {
                write!(f, "codewhisperer:AssociateCustomizationPermission")
            }
            CodewhispererActions::CreateCustomization => {
                write!(f, "codewhisperer:CreateCustomization")
            }
            CodewhispererActions::CreateProfile => write!(f, "codewhisperer:CreateProfile"),
            CodewhispererActions::DeleteCustomization => {
                write!(f, "codewhisperer:DeleteCustomization")
            }
            CodewhispererActions::DeleteProfile => write!(f, "codewhisperer:DeleteProfile"),
            CodewhispererActions::DisassociateCustomizationPermission => {
                write!(f, "codewhisperer:DisassociateCustomizationPermission")
            }
            CodewhispererActions::GenerateRecommendations => {
                write!(f, "codewhisperer:GenerateRecommendations")
            }
            CodewhispererActions::GetCustomization => write!(f, "codewhisperer:GetCustomization"),
            CodewhispererActions::ListCustomizationPermissions => {
                write!(f, "codewhisperer:ListCustomizationPermissions")
            }
            CodewhispererActions::ListCustomizationVersions => {
                write!(f, "codewhisperer:ListCustomizationVersions")
            }
            CodewhispererActions::ListCustomizations => {
                write!(f, "codewhisperer:ListCustomizations")
            }
            CodewhispererActions::ListProfiles => write!(f, "codewhisperer:ListProfiles"),
            CodewhispererActions::ListTagsForResource => {
                write!(f, "codewhisperer:ListTagsForResource")
            }
            CodewhispererActions::TagResource => write!(f, "codewhisperer:TagResource"),
            CodewhispererActions::UntagResource => write!(f, "codewhisperer:UntagResource"),
            CodewhispererActions::UpdateCustomization => {
                write!(f, "codewhisperer:UpdateCustomization")
            }
            CodewhispererActions::UpdateProfile => write!(f, "codewhisperer:UpdateProfile"),
        }
    }
}
