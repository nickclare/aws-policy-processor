// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum TirosActions {
    CreateQuery,
    ExtendQuery,
    GetQueryAnswer,
    GetQueryExplanation,
    GetQueryExtensionAccounts,
}
impl std::fmt::Display for TirosActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TirosActions::CreateQuery => write!(f, "tiros:CreateQuery"),
            TirosActions::ExtendQuery => write!(f, "tiros:ExtendQuery"),
            TirosActions::GetQueryAnswer => write!(f, "tiros:GetQueryAnswer"),
            TirosActions::GetQueryExplanation => write!(f, "tiros:GetQueryExplanation"),
            TirosActions::GetQueryExtensionAccounts => write!(f, "tiros:GetQueryExtensionAccounts"),
        }
    }
}
