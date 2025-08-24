// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum SqlworkbenchActions {
    AssociateConnectionWithChart,
    AssociateConnectionWithTab,
    AssociateNotebookWithTab,
    AssociateQueryWithTab,
    BatchDeleteFolder,
    BatchGetNotebookCell,
    CreateAccount,
    CreateChart,
    CreateConnection,
    CreateFolder,
    CreateNotebook,
    CreateNotebookCell,
    CreateNotebookFromVersion,
    CreateNotebookVersion,
    CreateSavedQuery,
    DeleteChart,
    DeleteConnection,
    DeleteNotebook,
    DeleteNotebookCell,
    DeleteNotebookVersion,
    DeleteQCustomContext,
    DeleteSavedQuery,
    DeleteSqlGenerationContext,
    DeleteTab,
    DriverExecute,
    DuplicateNotebook,
    ExportNotebook,
    GenerateSession,
    GetAccountInfo,
    GetAccountSettings,
    GetAutocompletionMetadata,
    GetAutocompletionResource,
    GetChart,
    GetConnection,
    GetNotebook,
    GetNotebookVersion,
    GetQCustomContext,
    GetQSqlPromptQuotas,
    GetQSqlRecommendations,
    GetQueryExecutionHistory,
    GetSavedQuery,
    GetSchemaInference,
    GetSqlGenerationContext,
    GetSqlRecommendations,
    GetUserInfo,
    GetUserWorkspaceSettings,
    ImportNotebook,
    ListConnections,
    ListDatabases,
    ListFiles,
    ListNotebookVersions,
    ListNotebooks,
    ListQueryExecutionHistory,
    ListRedshiftClusters,
    ListSampleDatabases,
    ListSavedQueryVersions,
    ListTabs,
    ListTaggedResources,
    ListTagsForResource,
    PassAccountSettings,
    PutQCustomContext,
    PutSqlGenerationContext,
    PutTab,
    PutUserWorkspaceSettings,
    RestoreNotebookVersion,
    TagResource,
    UntagResource,
    UpdateAccountConnectionSettings,
    UpdateAccountExportSettings,
    UpdateAccountGeneralSettings,
    UpdateAccountQSqlSettings,
    UpdateChart,
    UpdateConnection,
    UpdateFileFolder,
    UpdateFolder,
    UpdateNotebook,
    UpdateNotebookCellContent,
    UpdateNotebookCellLayout,
    UpdateSavedQuery,
}
impl std::fmt::Display for SqlworkbenchActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SqlworkbenchActions::AssociateConnectionWithChart => {
                write!(f, "sqlworkbench:AssociateConnectionWithChart")
            }
            SqlworkbenchActions::AssociateConnectionWithTab => {
                write!(f, "sqlworkbench:AssociateConnectionWithTab")
            }
            SqlworkbenchActions::AssociateNotebookWithTab => {
                write!(f, "sqlworkbench:AssociateNotebookWithTab")
            }
            SqlworkbenchActions::AssociateQueryWithTab => {
                write!(f, "sqlworkbench:AssociateQueryWithTab")
            }
            SqlworkbenchActions::BatchDeleteFolder => write!(f, "sqlworkbench:BatchDeleteFolder"),
            SqlworkbenchActions::BatchGetNotebookCell => {
                write!(f, "sqlworkbench:BatchGetNotebookCell")
            }
            SqlworkbenchActions::CreateAccount => write!(f, "sqlworkbench:CreateAccount"),
            SqlworkbenchActions::CreateChart => write!(f, "sqlworkbench:CreateChart"),
            SqlworkbenchActions::CreateConnection => write!(f, "sqlworkbench:CreateConnection"),
            SqlworkbenchActions::CreateFolder => write!(f, "sqlworkbench:CreateFolder"),
            SqlworkbenchActions::CreateNotebook => write!(f, "sqlworkbench:CreateNotebook"),
            SqlworkbenchActions::CreateNotebookCell => write!(f, "sqlworkbench:CreateNotebookCell"),
            SqlworkbenchActions::CreateNotebookFromVersion => {
                write!(f, "sqlworkbench:CreateNotebookFromVersion")
            }
            SqlworkbenchActions::CreateNotebookVersion => {
                write!(f, "sqlworkbench:CreateNotebookVersion")
            }
            SqlworkbenchActions::CreateSavedQuery => write!(f, "sqlworkbench:CreateSavedQuery"),
            SqlworkbenchActions::DeleteChart => write!(f, "sqlworkbench:DeleteChart"),
            SqlworkbenchActions::DeleteConnection => write!(f, "sqlworkbench:DeleteConnection"),
            SqlworkbenchActions::DeleteNotebook => write!(f, "sqlworkbench:DeleteNotebook"),
            SqlworkbenchActions::DeleteNotebookCell => write!(f, "sqlworkbench:DeleteNotebookCell"),
            SqlworkbenchActions::DeleteNotebookVersion => {
                write!(f, "sqlworkbench:DeleteNotebookVersion")
            }
            SqlworkbenchActions::DeleteQCustomContext => {
                write!(f, "sqlworkbench:DeleteQCustomContext")
            }
            SqlworkbenchActions::DeleteSavedQuery => write!(f, "sqlworkbench:DeleteSavedQuery"),
            SqlworkbenchActions::DeleteSqlGenerationContext => {
                write!(f, "sqlworkbench:DeleteSqlGenerationContext")
            }
            SqlworkbenchActions::DeleteTab => write!(f, "sqlworkbench:DeleteTab"),
            SqlworkbenchActions::DriverExecute => write!(f, "sqlworkbench:DriverExecute"),
            SqlworkbenchActions::DuplicateNotebook => write!(f, "sqlworkbench:DuplicateNotebook"),
            SqlworkbenchActions::ExportNotebook => write!(f, "sqlworkbench:ExportNotebook"),
            SqlworkbenchActions::GenerateSession => write!(f, "sqlworkbench:GenerateSession"),
            SqlworkbenchActions::GetAccountInfo => write!(f, "sqlworkbench:GetAccountInfo"),
            SqlworkbenchActions::GetAccountSettings => write!(f, "sqlworkbench:GetAccountSettings"),
            SqlworkbenchActions::GetAutocompletionMetadata => {
                write!(f, "sqlworkbench:GetAutocompletionMetadata")
            }
            SqlworkbenchActions::GetAutocompletionResource => {
                write!(f, "sqlworkbench:GetAutocompletionResource")
            }
            SqlworkbenchActions::GetChart => write!(f, "sqlworkbench:GetChart"),
            SqlworkbenchActions::GetConnection => write!(f, "sqlworkbench:GetConnection"),
            SqlworkbenchActions::GetNotebook => write!(f, "sqlworkbench:GetNotebook"),
            SqlworkbenchActions::GetNotebookVersion => write!(f, "sqlworkbench:GetNotebookVersion"),
            SqlworkbenchActions::GetQCustomContext => write!(f, "sqlworkbench:GetQCustomContext"),
            SqlworkbenchActions::GetQSqlPromptQuotas => {
                write!(f, "sqlworkbench:GetQSqlPromptQuotas")
            }
            SqlworkbenchActions::GetQSqlRecommendations => {
                write!(f, "sqlworkbench:GetQSqlRecommendations")
            }
            SqlworkbenchActions::GetQueryExecutionHistory => {
                write!(f, "sqlworkbench:GetQueryExecutionHistory")
            }
            SqlworkbenchActions::GetSavedQuery => write!(f, "sqlworkbench:GetSavedQuery"),
            SqlworkbenchActions::GetSchemaInference => write!(f, "sqlworkbench:GetSchemaInference"),
            SqlworkbenchActions::GetSqlGenerationContext => {
                write!(f, "sqlworkbench:GetSqlGenerationContext")
            }
            SqlworkbenchActions::GetSqlRecommendations => {
                write!(f, "sqlworkbench:GetSqlRecommendations")
            }
            SqlworkbenchActions::GetUserInfo => write!(f, "sqlworkbench:GetUserInfo"),
            SqlworkbenchActions::GetUserWorkspaceSettings => {
                write!(f, "sqlworkbench:GetUserWorkspaceSettings")
            }
            SqlworkbenchActions::ImportNotebook => write!(f, "sqlworkbench:ImportNotebook"),
            SqlworkbenchActions::ListConnections => write!(f, "sqlworkbench:ListConnections"),
            SqlworkbenchActions::ListDatabases => write!(f, "sqlworkbench:ListDatabases"),
            SqlworkbenchActions::ListFiles => write!(f, "sqlworkbench:ListFiles"),
            SqlworkbenchActions::ListNotebookVersions => {
                write!(f, "sqlworkbench:ListNotebookVersions")
            }
            SqlworkbenchActions::ListNotebooks => write!(f, "sqlworkbench:ListNotebooks"),
            SqlworkbenchActions::ListQueryExecutionHistory => {
                write!(f, "sqlworkbench:ListQueryExecutionHistory")
            }
            SqlworkbenchActions::ListRedshiftClusters => {
                write!(f, "sqlworkbench:ListRedshiftClusters")
            }
            SqlworkbenchActions::ListSampleDatabases => {
                write!(f, "sqlworkbench:ListSampleDatabases")
            }
            SqlworkbenchActions::ListSavedQueryVersions => {
                write!(f, "sqlworkbench:ListSavedQueryVersions")
            }
            SqlworkbenchActions::ListTabs => write!(f, "sqlworkbench:ListTabs"),
            SqlworkbenchActions::ListTaggedResources => {
                write!(f, "sqlworkbench:ListTaggedResources")
            }
            SqlworkbenchActions::ListTagsForResource => {
                write!(f, "sqlworkbench:ListTagsForResource")
            }
            SqlworkbenchActions::PassAccountSettings => {
                write!(f, "sqlworkbench:PassAccountSettings")
            }
            SqlworkbenchActions::PutQCustomContext => write!(f, "sqlworkbench:PutQCustomContext"),
            SqlworkbenchActions::PutSqlGenerationContext => {
                write!(f, "sqlworkbench:PutSqlGenerationContext")
            }
            SqlworkbenchActions::PutTab => write!(f, "sqlworkbench:PutTab"),
            SqlworkbenchActions::PutUserWorkspaceSettings => {
                write!(f, "sqlworkbench:PutUserWorkspaceSettings")
            }
            SqlworkbenchActions::RestoreNotebookVersion => {
                write!(f, "sqlworkbench:RestoreNotebookVersion")
            }
            SqlworkbenchActions::TagResource => write!(f, "sqlworkbench:TagResource"),
            SqlworkbenchActions::UntagResource => write!(f, "sqlworkbench:UntagResource"),
            SqlworkbenchActions::UpdateAccountConnectionSettings => {
                write!(f, "sqlworkbench:UpdateAccountConnectionSettings")
            }
            SqlworkbenchActions::UpdateAccountExportSettings => {
                write!(f, "sqlworkbench:UpdateAccountExportSettings")
            }
            SqlworkbenchActions::UpdateAccountGeneralSettings => {
                write!(f, "sqlworkbench:UpdateAccountGeneralSettings")
            }
            SqlworkbenchActions::UpdateAccountQSqlSettings => {
                write!(f, "sqlworkbench:UpdateAccountQSqlSettings")
            }
            SqlworkbenchActions::UpdateChart => write!(f, "sqlworkbench:UpdateChart"),
            SqlworkbenchActions::UpdateConnection => write!(f, "sqlworkbench:UpdateConnection"),
            SqlworkbenchActions::UpdateFileFolder => write!(f, "sqlworkbench:UpdateFileFolder"),
            SqlworkbenchActions::UpdateFolder => write!(f, "sqlworkbench:UpdateFolder"),
            SqlworkbenchActions::UpdateNotebook => write!(f, "sqlworkbench:UpdateNotebook"),
            SqlworkbenchActions::UpdateNotebookCellContent => {
                write!(f, "sqlworkbench:UpdateNotebookCellContent")
            }
            SqlworkbenchActions::UpdateNotebookCellLayout => {
                write!(f, "sqlworkbench:UpdateNotebookCellLayout")
            }
            SqlworkbenchActions::UpdateSavedQuery => write!(f, "sqlworkbench:UpdateSavedQuery"),
        }
    }
}
