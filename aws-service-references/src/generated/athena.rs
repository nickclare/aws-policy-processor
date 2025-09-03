// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum AthenaActions {
    BatchGetNamedQuery,
    BatchGetPreparedStatement,
    BatchGetQueryExecution,
    CancelCapacityReservation,
    CancelQueryExecution,
    CreateCapacityReservation,
    CreateDataCatalog,
    CreateNamedQuery,
    CreateNotebook,
    CreatePreparedStatement,
    CreatePresignedNotebookUrl,
    CreateWorkGroup,
    DeleteCapacityReservation,
    DeleteDataCatalog,
    DeleteNamedQuery,
    DeleteNotebook,
    DeletePreparedStatement,
    DeleteWorkGroup,
    ExportNotebook,
    GetCalculationExecution,
    GetCalculationExecutionCode,
    GetCalculationExecutionStatus,
    GetCapacityAssignmentConfiguration,
    GetCapacityReservation,
    GetCatalogs,
    GetDataCatalog,
    GetDatabase,
    GetExecutionEngine,
    GetExecutionEngines,
    GetNamedQuery,
    GetNamespace,
    GetNamespaces,
    GetNotebookMetadata,
    GetPreparedStatement,
    GetQueryExecution,
    GetQueryExecutions,
    GetQueryResults,
    GetQueryResultsStream,
    GetQueryRuntimeStatistics,
    GetSession,
    GetSessionStatus,
    GetTable,
    GetTableMetadata,
    GetTables,
    GetWorkGroup,
    ImportNotebook,
    ListApplicationDpuSizes,
    ListCalculationExecutions,
    ListCapacityReservations,
    ListDataCatalogs,
    ListDatabases,
    ListEngineVersions,
    ListExecutors,
    ListNamedQueries,
    ListNotebookMetadata,
    ListNotebookSessions,
    ListPreparedStatements,
    ListQueryExecutions,
    ListSessions,
    ListTableMetadata,
    ListTagsForResource,
    ListWorkGroups,
    PutCapacityAssignmentConfiguration,
    RunQuery,
    StartCalculationExecution,
    StartQueryExecution,
    StartSession,
    StopCalculationExecution,
    StopQueryExecution,
    TagResource,
    TerminateSession,
    UntagResource,
    UpdateCapacityReservation,
    UpdateDataCatalog,
    UpdateNamedQuery,
    UpdateNotebook,
    UpdateNotebookMetadata,
    UpdatePreparedStatement,
    UpdateWorkGroup,
}
impl std::fmt::Display for AthenaActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AthenaActions::BatchGetNamedQuery => write!(f, "athena:BatchGetNamedQuery"),
            AthenaActions::BatchGetPreparedStatement => {
                write!(f, "athena:BatchGetPreparedStatement")
            }
            AthenaActions::BatchGetQueryExecution => write!(f, "athena:BatchGetQueryExecution"),
            AthenaActions::CancelCapacityReservation => {
                write!(f, "athena:CancelCapacityReservation")
            }
            AthenaActions::CancelQueryExecution => write!(f, "athena:CancelQueryExecution"),
            AthenaActions::CreateCapacityReservation => {
                write!(f, "athena:CreateCapacityReservation")
            }
            AthenaActions::CreateDataCatalog => write!(f, "athena:CreateDataCatalog"),
            AthenaActions::CreateNamedQuery => write!(f, "athena:CreateNamedQuery"),
            AthenaActions::CreateNotebook => write!(f, "athena:CreateNotebook"),
            AthenaActions::CreatePreparedStatement => write!(f, "athena:CreatePreparedStatement"),
            AthenaActions::CreatePresignedNotebookUrl => {
                write!(f, "athena:CreatePresignedNotebookUrl")
            }
            AthenaActions::CreateWorkGroup => write!(f, "athena:CreateWorkGroup"),
            AthenaActions::DeleteCapacityReservation => {
                write!(f, "athena:DeleteCapacityReservation")
            }
            AthenaActions::DeleteDataCatalog => write!(f, "athena:DeleteDataCatalog"),
            AthenaActions::DeleteNamedQuery => write!(f, "athena:DeleteNamedQuery"),
            AthenaActions::DeleteNotebook => write!(f, "athena:DeleteNotebook"),
            AthenaActions::DeletePreparedStatement => write!(f, "athena:DeletePreparedStatement"),
            AthenaActions::DeleteWorkGroup => write!(f, "athena:DeleteWorkGroup"),
            AthenaActions::ExportNotebook => write!(f, "athena:ExportNotebook"),
            AthenaActions::GetCalculationExecution => write!(f, "athena:GetCalculationExecution"),
            AthenaActions::GetCalculationExecutionCode => {
                write!(f, "athena:GetCalculationExecutionCode")
            }
            AthenaActions::GetCalculationExecutionStatus => {
                write!(f, "athena:GetCalculationExecutionStatus")
            }
            AthenaActions::GetCapacityAssignmentConfiguration => {
                write!(f, "athena:GetCapacityAssignmentConfiguration")
            }
            AthenaActions::GetCapacityReservation => write!(f, "athena:GetCapacityReservation"),
            AthenaActions::GetCatalogs => write!(f, "athena:GetCatalogs"),
            AthenaActions::GetDataCatalog => write!(f, "athena:GetDataCatalog"),
            AthenaActions::GetDatabase => write!(f, "athena:GetDatabase"),
            AthenaActions::GetExecutionEngine => write!(f, "athena:GetExecutionEngine"),
            AthenaActions::GetExecutionEngines => write!(f, "athena:GetExecutionEngines"),
            AthenaActions::GetNamedQuery => write!(f, "athena:GetNamedQuery"),
            AthenaActions::GetNamespace => write!(f, "athena:GetNamespace"),
            AthenaActions::GetNamespaces => write!(f, "athena:GetNamespaces"),
            AthenaActions::GetNotebookMetadata => write!(f, "athena:GetNotebookMetadata"),
            AthenaActions::GetPreparedStatement => write!(f, "athena:GetPreparedStatement"),
            AthenaActions::GetQueryExecution => write!(f, "athena:GetQueryExecution"),
            AthenaActions::GetQueryExecutions => write!(f, "athena:GetQueryExecutions"),
            AthenaActions::GetQueryResults => write!(f, "athena:GetQueryResults"),
            AthenaActions::GetQueryResultsStream => write!(f, "athena:GetQueryResultsStream"),
            AthenaActions::GetQueryRuntimeStatistics => {
                write!(f, "athena:GetQueryRuntimeStatistics")
            }
            AthenaActions::GetSession => write!(f, "athena:GetSession"),
            AthenaActions::GetSessionStatus => write!(f, "athena:GetSessionStatus"),
            AthenaActions::GetTable => write!(f, "athena:GetTable"),
            AthenaActions::GetTableMetadata => write!(f, "athena:GetTableMetadata"),
            AthenaActions::GetTables => write!(f, "athena:GetTables"),
            AthenaActions::GetWorkGroup => write!(f, "athena:GetWorkGroup"),
            AthenaActions::ImportNotebook => write!(f, "athena:ImportNotebook"),
            AthenaActions::ListApplicationDpuSizes => write!(f, "athena:ListApplicationDPUSizes"),
            AthenaActions::ListCalculationExecutions => {
                write!(f, "athena:ListCalculationExecutions")
            }
            AthenaActions::ListCapacityReservations => write!(f, "athena:ListCapacityReservations"),
            AthenaActions::ListDataCatalogs => write!(f, "athena:ListDataCatalogs"),
            AthenaActions::ListDatabases => write!(f, "athena:ListDatabases"),
            AthenaActions::ListEngineVersions => write!(f, "athena:ListEngineVersions"),
            AthenaActions::ListExecutors => write!(f, "athena:ListExecutors"),
            AthenaActions::ListNamedQueries => write!(f, "athena:ListNamedQueries"),
            AthenaActions::ListNotebookMetadata => write!(f, "athena:ListNotebookMetadata"),
            AthenaActions::ListNotebookSessions => write!(f, "athena:ListNotebookSessions"),
            AthenaActions::ListPreparedStatements => write!(f, "athena:ListPreparedStatements"),
            AthenaActions::ListQueryExecutions => write!(f, "athena:ListQueryExecutions"),
            AthenaActions::ListSessions => write!(f, "athena:ListSessions"),
            AthenaActions::ListTableMetadata => write!(f, "athena:ListTableMetadata"),
            AthenaActions::ListTagsForResource => write!(f, "athena:ListTagsForResource"),
            AthenaActions::ListWorkGroups => write!(f, "athena:ListWorkGroups"),
            AthenaActions::PutCapacityAssignmentConfiguration => {
                write!(f, "athena:PutCapacityAssignmentConfiguration")
            }
            AthenaActions::RunQuery => write!(f, "athena:RunQuery"),
            AthenaActions::StartCalculationExecution => {
                write!(f, "athena:StartCalculationExecution")
            }
            AthenaActions::StartQueryExecution => write!(f, "athena:StartQueryExecution"),
            AthenaActions::StartSession => write!(f, "athena:StartSession"),
            AthenaActions::StopCalculationExecution => write!(f, "athena:StopCalculationExecution"),
            AthenaActions::StopQueryExecution => write!(f, "athena:StopQueryExecution"),
            AthenaActions::TagResource => write!(f, "athena:TagResource"),
            AthenaActions::TerminateSession => write!(f, "athena:TerminateSession"),
            AthenaActions::UntagResource => write!(f, "athena:UntagResource"),
            AthenaActions::UpdateCapacityReservation => {
                write!(f, "athena:UpdateCapacityReservation")
            }
            AthenaActions::UpdateDataCatalog => write!(f, "athena:UpdateDataCatalog"),
            AthenaActions::UpdateNamedQuery => write!(f, "athena:UpdateNamedQuery"),
            AthenaActions::UpdateNotebook => write!(f, "athena:UpdateNotebook"),
            AthenaActions::UpdateNotebookMetadata => write!(f, "athena:UpdateNotebookMetadata"),
            AthenaActions::UpdatePreparedStatement => write!(f, "athena:UpdatePreparedStatement"),
            AthenaActions::UpdateWorkGroup => write!(f, "athena:UpdateWorkGroup"),
        }
    }
}
