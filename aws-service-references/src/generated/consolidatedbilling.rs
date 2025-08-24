// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum ConsolidatedbillingActions {
    GetAccountBillingRole,
    ListLinkedAccounts,
}
impl std::fmt::Display for ConsolidatedbillingActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConsolidatedbillingActions::GetAccountBillingRole => {
                write!(f, "consolidatedbilling:GetAccountBillingRole")
            }
            ConsolidatedbillingActions::ListLinkedAccounts => {
                write!(f, "consolidatedbilling:ListLinkedAccounts")
            }
        }
    }
}
