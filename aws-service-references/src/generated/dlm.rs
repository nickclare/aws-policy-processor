// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum DlmActions {
    CreateLifecyclePolicy,
    DeleteLifecyclePolicy,
    GetLifecyclePolicies,
    GetLifecyclePolicy,
    ListTagsForResource,
    TagResource,
    UntagResource,
    UpdateLifecyclePolicy,
}
impl std::fmt::Display for DlmActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DlmActions::CreateLifecyclePolicy => write!(f, "dlm:CreateLifecyclePolicy"),
            DlmActions::DeleteLifecyclePolicy => write!(f, "dlm:DeleteLifecyclePolicy"),
            DlmActions::GetLifecyclePolicies => write!(f, "dlm:GetLifecyclePolicies"),
            DlmActions::GetLifecyclePolicy => write!(f, "dlm:GetLifecyclePolicy"),
            DlmActions::ListTagsForResource => write!(f, "dlm:ListTagsForResource"),
            DlmActions::TagResource => write!(f, "dlm:TagResource"),
            DlmActions::UntagResource => write!(f, "dlm:UntagResource"),
            DlmActions::UpdateLifecyclePolicy => write!(f, "dlm:UpdateLifecyclePolicy"),
        }
    }
}
