// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum SecurityIrActions {
    BatchGetMemberAccountDetails,
    CancelMembership,
    CloseCase,
    CreateCase,
    CreateCaseComment,
    CreateMembership,
    GetCase,
    GetCaseAttachmentDownloadUrl,
    GetCaseAttachmentUploadUrl,
    GetMembership,
    ListCaseEdits,
    ListCases,
    ListComments,
    ListMemberships,
    ListTagsForResource,
    TagResource,
    UntagResource,
    UpdateCase,
    UpdateCaseComment,
    UpdateCaseStatus,
    UpdateMembership,
    UpdateResolverType,
}
impl std::fmt::Display for SecurityIrActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SecurityIrActions::BatchGetMemberAccountDetails => {
                write!(f, "security-ir:BatchGetMemberAccountDetails")
            }
            SecurityIrActions::CancelMembership => write!(f, "security-ir:CancelMembership"),
            SecurityIrActions::CloseCase => write!(f, "security-ir:CloseCase"),
            SecurityIrActions::CreateCase => write!(f, "security-ir:CreateCase"),
            SecurityIrActions::CreateCaseComment => write!(f, "security-ir:CreateCaseComment"),
            SecurityIrActions::CreateMembership => write!(f, "security-ir:CreateMembership"),
            SecurityIrActions::GetCase => write!(f, "security-ir:GetCase"),
            SecurityIrActions::GetCaseAttachmentDownloadUrl => {
                write!(f, "security-ir:GetCaseAttachmentDownloadUrl")
            }
            SecurityIrActions::GetCaseAttachmentUploadUrl => {
                write!(f, "security-ir:GetCaseAttachmentUploadUrl")
            }
            SecurityIrActions::GetMembership => write!(f, "security-ir:GetMembership"),
            SecurityIrActions::ListCaseEdits => write!(f, "security-ir:ListCaseEdits"),
            SecurityIrActions::ListCases => write!(f, "security-ir:ListCases"),
            SecurityIrActions::ListComments => write!(f, "security-ir:ListComments"),
            SecurityIrActions::ListMemberships => write!(f, "security-ir:ListMemberships"),
            SecurityIrActions::ListTagsForResource => write!(f, "security-ir:ListTagsForResource"),
            SecurityIrActions::TagResource => write!(f, "security-ir:TagResource"),
            SecurityIrActions::UntagResource => write!(f, "security-ir:UntagResource"),
            SecurityIrActions::UpdateCase => write!(f, "security-ir:UpdateCase"),
            SecurityIrActions::UpdateCaseComment => write!(f, "security-ir:UpdateCaseComment"),
            SecurityIrActions::UpdateCaseStatus => write!(f, "security-ir:UpdateCaseStatus"),
            SecurityIrActions::UpdateMembership => write!(f, "security-ir:UpdateMembership"),
            SecurityIrActions::UpdateResolverType => write!(f, "security-ir:UpdateResolverType"),
        }
    }
}
