// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum IdentitystoreAuthActions {
    BatchDeleteSession,
    BatchGetSession,
    ListSessions,
}
impl std::fmt::Display for IdentitystoreAuthActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IdentitystoreAuthActions::BatchDeleteSession => {
                write!(f, "identitystore-auth:BatchDeleteSession")
            }
            IdentitystoreAuthActions::BatchGetSession => {
                write!(f, "identitystore-auth:BatchGetSession")
            }
            IdentitystoreAuthActions::ListSessions => write!(f, "identitystore-auth:ListSessions"),
        }
    }
}
