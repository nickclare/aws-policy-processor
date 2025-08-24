// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum ActivateActions {
    CreateForm,
    GetAccountContact,
    GetContentInfo,
    GetCosts,
    GetCredits,
    GetMemberInfo,
    GetProgram,
    PutMemberInfo,
}
impl std::fmt::Display for ActivateActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ActivateActions::CreateForm => write!(f, "activate:CreateForm"),
            ActivateActions::GetAccountContact => write!(f, "activate:GetAccountContact"),
            ActivateActions::GetContentInfo => write!(f, "activate:GetContentInfo"),
            ActivateActions::GetCosts => write!(f, "activate:GetCosts"),
            ActivateActions::GetCredits => write!(f, "activate:GetCredits"),
            ActivateActions::GetMemberInfo => write!(f, "activate:GetMemberInfo"),
            ActivateActions::GetProgram => write!(f, "activate:GetProgram"),
            ActivateActions::PutMemberInfo => write!(f, "activate:PutMemberInfo"),
        }
    }
}
