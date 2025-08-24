// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum ManagedblockchainQueryActions {
    BatchGetTokenBalance,
    GetAssetContract,
    GetTokenBalance,
    GetTransaction,
    ListAssetContracts,
    ListFilteredTransactionEvents,
    ListTokenBalances,
    ListTransactionEvents,
    ListTransactions,
}
impl std::fmt::Display for ManagedblockchainQueryActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ManagedblockchainQueryActions::BatchGetTokenBalance => {
                write!(f, "managedblockchain-query:BatchGetTokenBalance")
            }
            ManagedblockchainQueryActions::GetAssetContract => {
                write!(f, "managedblockchain-query:GetAssetContract")
            }
            ManagedblockchainQueryActions::GetTokenBalance => {
                write!(f, "managedblockchain-query:GetTokenBalance")
            }
            ManagedblockchainQueryActions::GetTransaction => {
                write!(f, "managedblockchain-query:GetTransaction")
            }
            ManagedblockchainQueryActions::ListAssetContracts => {
                write!(f, "managedblockchain-query:ListAssetContracts")
            }
            ManagedblockchainQueryActions::ListFilteredTransactionEvents => {
                write!(f, "managedblockchain-query:ListFilteredTransactionEvents")
            }
            ManagedblockchainQueryActions::ListTokenBalances => {
                write!(f, "managedblockchain-query:ListTokenBalances")
            }
            ManagedblockchainQueryActions::ListTransactionEvents => {
                write!(f, "managedblockchain-query:ListTransactionEvents")
            }
            ManagedblockchainQueryActions::ListTransactions => {
                write!(f, "managedblockchain-query:ListTransactions")
            }
        }
    }
}
