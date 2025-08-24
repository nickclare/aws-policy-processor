// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum ExecuteApiActions {
    InvalidateCache,
    Invoke,
    ManageConnections,
}
impl std::fmt::Display for ExecuteApiActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExecuteApiActions::InvalidateCache => write!(f, "execute-api:InvalidateCache"),
            ExecuteApiActions::Invoke => write!(f, "execute-api:Invoke"),
            ExecuteApiActions::ManageConnections => write!(f, "execute-api:ManageConnections"),
        }
    }
}
