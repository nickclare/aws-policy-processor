// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum CodeguruActions {
    GetCodeGuruFreeTrialSummary,
}
impl std::fmt::Display for CodeguruActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CodeguruActions::GetCodeGuruFreeTrialSummary => {
                write!(f, "codeguru:GetCodeGuruFreeTrialSummary")
            }
        }
    }
}
