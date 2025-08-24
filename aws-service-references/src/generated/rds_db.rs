// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum RdsDbActions {
    Connect,
}
impl std::fmt::Display for RdsDbActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RdsDbActions::Connect => write!(f, "rds-db:connect"),
        }
    }
}
