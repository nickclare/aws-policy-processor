// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum ServiceextractActions {
    GetConfig,
}
impl std::fmt::Display for ServiceextractActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServiceextractActions::GetConfig => write!(f, "serviceextract:GetConfig"),
        }
    }
}
