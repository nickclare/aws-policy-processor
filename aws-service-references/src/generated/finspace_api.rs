// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum FinspaceApiActions {
    GetProgrammaticAccessCredentials,
}
impl std::fmt::Display for FinspaceApiActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FinspaceApiActions::GetProgrammaticAccessCredentials => {
                write!(f, "finspace-api:GetProgrammaticAccessCredentials")
            }
        }
    }
}
