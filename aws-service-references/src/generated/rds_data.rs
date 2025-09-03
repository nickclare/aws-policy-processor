// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum RdsDataActions {
    BatchExecuteStatement,
    BeginTransaction,
    CommitTransaction,
    ExecuteSql,
    ExecuteStatement,
    RollbackTransaction,
}
impl std::fmt::Display for RdsDataActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RdsDataActions::BatchExecuteStatement => write!(f, "rds-data:BatchExecuteStatement"),
            RdsDataActions::BeginTransaction => write!(f, "rds-data:BeginTransaction"),
            RdsDataActions::CommitTransaction => write!(f, "rds-data:CommitTransaction"),
            RdsDataActions::ExecuteSql => write!(f, "rds-data:ExecuteSql"),
            RdsDataActions::ExecuteStatement => write!(f, "rds-data:ExecuteStatement"),
            RdsDataActions::RollbackTransaction => write!(f, "rds-data:RollbackTransaction"),
        }
    }
}
