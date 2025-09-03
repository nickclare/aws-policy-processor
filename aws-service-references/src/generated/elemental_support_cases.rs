// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum ElementalSupportCasesActions {
    AddCaseComment,
    CheckCasePermission,
    CompleteMultipartUpload,
    CreateCase,
    CreateS3DownloadUrl,
    CreateS3cliUploadCommand,
    GetCase,
    GetCasePermission,
    GetCases,
    GetUiCache,
    ListTagsForCase,
    StartMultipartUpload,
    TagCase,
    UntagCase,
    UpdateCase,
    UpdateCaseStatus,
    UpdateMultipartUpload,
}
impl std::fmt::Display for ElementalSupportCasesActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ElementalSupportCasesActions::AddCaseComment => {
                write!(f, "elemental-support-cases:AddCaseComment")
            }
            ElementalSupportCasesActions::CheckCasePermission => {
                write!(f, "elemental-support-cases:CheckCasePermission")
            }
            ElementalSupportCasesActions::CompleteMultipartUpload => {
                write!(f, "elemental-support-cases:CompleteMultipartUpload")
            }
            ElementalSupportCasesActions::CreateCase => {
                write!(f, "elemental-support-cases:CreateCase")
            }
            ElementalSupportCasesActions::CreateS3DownloadUrl => {
                write!(f, "elemental-support-cases:CreateS3DownloadUrl")
            }
            ElementalSupportCasesActions::CreateS3cliUploadCommand => {
                write!(f, "elemental-support-cases:CreateS3CLIUploadCommand")
            }
            ElementalSupportCasesActions::GetCase => write!(f, "elemental-support-cases:GetCase"),
            ElementalSupportCasesActions::GetCasePermission => {
                write!(f, "elemental-support-cases:GetCasePermission")
            }
            ElementalSupportCasesActions::GetCases => write!(f, "elemental-support-cases:GetCases"),
            ElementalSupportCasesActions::GetUiCache => {
                write!(f, "elemental-support-cases:GetUICache")
            }
            ElementalSupportCasesActions::ListTagsForCase => {
                write!(f, "elemental-support-cases:ListTagsForCase")
            }
            ElementalSupportCasesActions::StartMultipartUpload => {
                write!(f, "elemental-support-cases:StartMultipartUpload")
            }
            ElementalSupportCasesActions::TagCase => write!(f, "elemental-support-cases:TagCase"),
            ElementalSupportCasesActions::UntagCase => {
                write!(f, "elemental-support-cases:UntagCase")
            }
            ElementalSupportCasesActions::UpdateCase => {
                write!(f, "elemental-support-cases:UpdateCase")
            }
            ElementalSupportCasesActions::UpdateCaseStatus => {
                write!(f, "elemental-support-cases:UpdateCaseStatus")
            }
            ElementalSupportCasesActions::UpdateMultipartUpload => {
                write!(f, "elemental-support-cases:UpdateMultipartUpload")
            }
        }
    }
}
