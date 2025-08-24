// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum ElementalActivationsActions {
    CompleteAccountRegistration,
    CompleteFileUpload,
    ConfirmAccount,
    DownloadKickstart,
    DownloadSoftware,
    GenerateLicense,
    GenerateLicenses,
    GetArtifactGroupSoftwareVersions,
    GetAsset,
    GetAssets,
    GetProductAdvisories,
    GetSoftwareVersions,
    StartFileUpload,
}
impl std::fmt::Display for ElementalActivationsActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ElementalActivationsActions::CompleteAccountRegistration => {
                write!(f, "elemental-activations:CompleteAccountRegistration")
            }
            ElementalActivationsActions::CompleteFileUpload => {
                write!(f, "elemental-activations:CompleteFileUpload")
            }
            ElementalActivationsActions::ConfirmAccount => {
                write!(f, "elemental-activations:ConfirmAccount")
            }
            ElementalActivationsActions::DownloadKickstart => {
                write!(f, "elemental-activations:DownloadKickstart")
            }
            ElementalActivationsActions::DownloadSoftware => {
                write!(f, "elemental-activations:DownloadSoftware")
            }
            ElementalActivationsActions::GenerateLicense => {
                write!(f, "elemental-activations:GenerateLicense")
            }
            ElementalActivationsActions::GenerateLicenses => {
                write!(f, "elemental-activations:GenerateLicenses")
            }
            ElementalActivationsActions::GetArtifactGroupSoftwareVersions => {
                write!(f, "elemental-activations:GetArtifactGroupSoftwareVersions")
            }
            ElementalActivationsActions::GetAsset => write!(f, "elemental-activations:GetAsset"),
            ElementalActivationsActions::GetAssets => write!(f, "elemental-activations:GetAssets"),
            ElementalActivationsActions::GetProductAdvisories => {
                write!(f, "elemental-activations:GetProductAdvisories")
            }
            ElementalActivationsActions::GetSoftwareVersions => {
                write!(f, "elemental-activations:GetSoftwareVersions")
            }
            ElementalActivationsActions::StartFileUpload => {
                write!(f, "elemental-activations:StartFileUpload")
            }
        }
    }
}
