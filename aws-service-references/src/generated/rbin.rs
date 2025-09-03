// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum RbinActions {
    CreateRule,
    DeleteRule,
    GetRule,
    ListRules,
    ListTagsForResource,
    LockRule,
    TagResource,
    UnlockRule,
    UntagResource,
    UpdateRule,
}
impl std::fmt::Display for RbinActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RbinActions::CreateRule => write!(f, "rbin:CreateRule"),
            RbinActions::DeleteRule => write!(f, "rbin:DeleteRule"),
            RbinActions::GetRule => write!(f, "rbin:GetRule"),
            RbinActions::ListRules => write!(f, "rbin:ListRules"),
            RbinActions::ListTagsForResource => write!(f, "rbin:ListTagsForResource"),
            RbinActions::LockRule => write!(f, "rbin:LockRule"),
            RbinActions::TagResource => write!(f, "rbin:TagResource"),
            RbinActions::UnlockRule => write!(f, "rbin:UnlockRule"),
            RbinActions::UntagResource => write!(f, "rbin:UntagResource"),
            RbinActions::UpdateRule => write!(f, "rbin:UpdateRule"),
        }
    }
}
