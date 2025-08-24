// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum OpensearchActions {
    ApplicationAccessAll,
    CancelDirectQuery,
    GetDirectQuery,
    GetDirectQueryResult,
    StartDirectQuery,
}
impl std::fmt::Display for OpensearchActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OpensearchActions::ApplicationAccessAll => write!(f, "opensearch:ApplicationAccessAll"),
            OpensearchActions::CancelDirectQuery => write!(f, "opensearch:CancelDirectQuery"),
            OpensearchActions::GetDirectQuery => write!(f, "opensearch:GetDirectQuery"),
            OpensearchActions::GetDirectQueryResult => write!(f, "opensearch:GetDirectQueryResult"),
            OpensearchActions::StartDirectQuery => write!(f, "opensearch:StartDirectQuery"),
        }
    }
}
