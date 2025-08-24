// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum UxcActions {
    DeleteAccountColor,
    GetAccountColor,
    PutAccountColor,
}
impl std::fmt::Display for UxcActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UxcActions::DeleteAccountColor => write!(f, "uxc:DeleteAccountColor"),
            UxcActions::GetAccountColor => write!(f, "uxc:GetAccountColor"),
            UxcActions::PutAccountColor => write!(f, "uxc:PutAccountColor"),
        }
    }
}
