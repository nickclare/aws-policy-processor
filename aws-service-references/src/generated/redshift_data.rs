// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum RedshiftDataActions {
    BatchExecuteStatement,
    CancelStatement,
    DescribeStatement,
    DescribeTable,
    ExecuteStatement,
    GetStagingBucketLocation,
    GetStatementResult,
    ListDatabases,
    ListSchemas,
    ListStatements,
    ListTables,
}
impl std::fmt::Display for RedshiftDataActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RedshiftDataActions::BatchExecuteStatement => {
                write!(f, "redshift-data:BatchExecuteStatement")
            }
            RedshiftDataActions::CancelStatement => write!(f, "redshift-data:CancelStatement"),
            RedshiftDataActions::DescribeStatement => write!(f, "redshift-data:DescribeStatement"),
            RedshiftDataActions::DescribeTable => write!(f, "redshift-data:DescribeTable"),
            RedshiftDataActions::ExecuteStatement => write!(f, "redshift-data:ExecuteStatement"),
            RedshiftDataActions::GetStagingBucketLocation => {
                write!(f, "redshift-data:GetStagingBucketLocation")
            }
            RedshiftDataActions::GetStatementResult => {
                write!(f, "redshift-data:GetStatementResult")
            }
            RedshiftDataActions::ListDatabases => write!(f, "redshift-data:ListDatabases"),
            RedshiftDataActions::ListSchemas => write!(f, "redshift-data:ListSchemas"),
            RedshiftDataActions::ListStatements => write!(f, "redshift-data:ListStatements"),
            RedshiftDataActions::ListTables => write!(f, "redshift-data:ListTables"),
        }
    }
}
