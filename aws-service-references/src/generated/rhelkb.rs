// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum RhelkbActions {
    GetRhelUrl,
}
impl std::fmt::Display for RhelkbActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RhelkbActions::GetRhelUrl => write!(f, "rhelkb:GetRhelURL"),
        }
    }
}
