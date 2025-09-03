// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum CodebuildActions {
    BatchDeleteBuilds,
    BatchGetBuildBatches,
    BatchGetBuilds,
    BatchGetCommandExecutions,
    BatchGetFleets,
    BatchGetProjects,
    BatchGetReportGroups,
    BatchGetReports,
    BatchGetSandboxes,
    BatchPutCodeCoverages,
    BatchPutTestCases,
    CreateFleet,
    CreateProject,
    CreateReport,
    CreateReportGroup,
    CreateWebhook,
    DeleteBuildBatch,
    DeleteFleet,
    DeleteOAuthToken,
    DeleteProject,
    DeleteReport,
    DeleteReportGroup,
    DeleteResourcePolicy,
    DeleteSourceCredentials,
    DeleteWebhook,
    DescribeCodeCoverages,
    DescribeTestCases,
    GetReportGroupTrend,
    GetResourcePolicy,
    ImportSourceCredentials,
    InvalidateProjectCache,
    ListBuildBatches,
    ListBuildBatchesForProject,
    ListBuilds,
    ListBuildsForProject,
    ListCommandExecutionsForSandbox,
    ListConnectedOAuthAccounts,
    ListCuratedEnvironmentImages,
    ListFleets,
    ListProjects,
    ListReportGroups,
    ListReports,
    ListReportsForReportGroup,
    ListRepositories,
    ListSandboxes,
    ListSandboxesForProject,
    ListSharedProjects,
    ListSharedReportGroups,
    ListSourceCredentials,
    PersistOAuthToken,
    PutResourcePolicy,
    RetryBuild,
    RetryBuildBatch,
    StartBuild,
    StartBuildBatch,
    StartCommandExecution,
    StartSandbox,
    StartSandboxConnection,
    StopBuild,
    StopBuildBatch,
    StopSandbox,
    UpdateFleet,
    UpdateProject,
    UpdateProjectVisibility,
    UpdateReport,
    UpdateReportGroup,
    UpdateWebhook,
}
impl std::fmt::Display for CodebuildActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CodebuildActions::BatchDeleteBuilds => write!(f, "codebuild:BatchDeleteBuilds"),
            CodebuildActions::BatchGetBuildBatches => write!(f, "codebuild:BatchGetBuildBatches"),
            CodebuildActions::BatchGetBuilds => write!(f, "codebuild:BatchGetBuilds"),
            CodebuildActions::BatchGetCommandExecutions => {
                write!(f, "codebuild:BatchGetCommandExecutions")
            }
            CodebuildActions::BatchGetFleets => write!(f, "codebuild:BatchGetFleets"),
            CodebuildActions::BatchGetProjects => write!(f, "codebuild:BatchGetProjects"),
            CodebuildActions::BatchGetReportGroups => write!(f, "codebuild:BatchGetReportGroups"),
            CodebuildActions::BatchGetReports => write!(f, "codebuild:BatchGetReports"),
            CodebuildActions::BatchGetSandboxes => write!(f, "codebuild:BatchGetSandboxes"),
            CodebuildActions::BatchPutCodeCoverages => write!(f, "codebuild:BatchPutCodeCoverages"),
            CodebuildActions::BatchPutTestCases => write!(f, "codebuild:BatchPutTestCases"),
            CodebuildActions::CreateFleet => write!(f, "codebuild:CreateFleet"),
            CodebuildActions::CreateProject => write!(f, "codebuild:CreateProject"),
            CodebuildActions::CreateReport => write!(f, "codebuild:CreateReport"),
            CodebuildActions::CreateReportGroup => write!(f, "codebuild:CreateReportGroup"),
            CodebuildActions::CreateWebhook => write!(f, "codebuild:CreateWebhook"),
            CodebuildActions::DeleteBuildBatch => write!(f, "codebuild:DeleteBuildBatch"),
            CodebuildActions::DeleteFleet => write!(f, "codebuild:DeleteFleet"),
            CodebuildActions::DeleteOAuthToken => write!(f, "codebuild:DeleteOAuthToken"),
            CodebuildActions::DeleteProject => write!(f, "codebuild:DeleteProject"),
            CodebuildActions::DeleteReport => write!(f, "codebuild:DeleteReport"),
            CodebuildActions::DeleteReportGroup => write!(f, "codebuild:DeleteReportGroup"),
            CodebuildActions::DeleteResourcePolicy => write!(f, "codebuild:DeleteResourcePolicy"),
            CodebuildActions::DeleteSourceCredentials => {
                write!(f, "codebuild:DeleteSourceCredentials")
            }
            CodebuildActions::DeleteWebhook => write!(f, "codebuild:DeleteWebhook"),
            CodebuildActions::DescribeCodeCoverages => write!(f, "codebuild:DescribeCodeCoverages"),
            CodebuildActions::DescribeTestCases => write!(f, "codebuild:DescribeTestCases"),
            CodebuildActions::GetReportGroupTrend => write!(f, "codebuild:GetReportGroupTrend"),
            CodebuildActions::GetResourcePolicy => write!(f, "codebuild:GetResourcePolicy"),
            CodebuildActions::ImportSourceCredentials => {
                write!(f, "codebuild:ImportSourceCredentials")
            }
            CodebuildActions::InvalidateProjectCache => {
                write!(f, "codebuild:InvalidateProjectCache")
            }
            CodebuildActions::ListBuildBatches => write!(f, "codebuild:ListBuildBatches"),
            CodebuildActions::ListBuildBatchesForProject => {
                write!(f, "codebuild:ListBuildBatchesForProject")
            }
            CodebuildActions::ListBuilds => write!(f, "codebuild:ListBuilds"),
            CodebuildActions::ListBuildsForProject => write!(f, "codebuild:ListBuildsForProject"),
            CodebuildActions::ListCommandExecutionsForSandbox => {
                write!(f, "codebuild:ListCommandExecutionsForSandbox")
            }
            CodebuildActions::ListConnectedOAuthAccounts => {
                write!(f, "codebuild:ListConnectedOAuthAccounts")
            }
            CodebuildActions::ListCuratedEnvironmentImages => {
                write!(f, "codebuild:ListCuratedEnvironmentImages")
            }
            CodebuildActions::ListFleets => write!(f, "codebuild:ListFleets"),
            CodebuildActions::ListProjects => write!(f, "codebuild:ListProjects"),
            CodebuildActions::ListReportGroups => write!(f, "codebuild:ListReportGroups"),
            CodebuildActions::ListReports => write!(f, "codebuild:ListReports"),
            CodebuildActions::ListReportsForReportGroup => {
                write!(f, "codebuild:ListReportsForReportGroup")
            }
            CodebuildActions::ListRepositories => write!(f, "codebuild:ListRepositories"),
            CodebuildActions::ListSandboxes => write!(f, "codebuild:ListSandboxes"),
            CodebuildActions::ListSandboxesForProject => {
                write!(f, "codebuild:ListSandboxesForProject")
            }
            CodebuildActions::ListSharedProjects => write!(f, "codebuild:ListSharedProjects"),
            CodebuildActions::ListSharedReportGroups => {
                write!(f, "codebuild:ListSharedReportGroups")
            }
            CodebuildActions::ListSourceCredentials => write!(f, "codebuild:ListSourceCredentials"),
            CodebuildActions::PersistOAuthToken => write!(f, "codebuild:PersistOAuthToken"),
            CodebuildActions::PutResourcePolicy => write!(f, "codebuild:PutResourcePolicy"),
            CodebuildActions::RetryBuild => write!(f, "codebuild:RetryBuild"),
            CodebuildActions::RetryBuildBatch => write!(f, "codebuild:RetryBuildBatch"),
            CodebuildActions::StartBuild => write!(f, "codebuild:StartBuild"),
            CodebuildActions::StartBuildBatch => write!(f, "codebuild:StartBuildBatch"),
            CodebuildActions::StartCommandExecution => write!(f, "codebuild:StartCommandExecution"),
            CodebuildActions::StartSandbox => write!(f, "codebuild:StartSandbox"),
            CodebuildActions::StartSandboxConnection => {
                write!(f, "codebuild:StartSandboxConnection")
            }
            CodebuildActions::StopBuild => write!(f, "codebuild:StopBuild"),
            CodebuildActions::StopBuildBatch => write!(f, "codebuild:StopBuildBatch"),
            CodebuildActions::StopSandbox => write!(f, "codebuild:StopSandbox"),
            CodebuildActions::UpdateFleet => write!(f, "codebuild:UpdateFleet"),
            CodebuildActions::UpdateProject => write!(f, "codebuild:UpdateProject"),
            CodebuildActions::UpdateProjectVisibility => {
                write!(f, "codebuild:UpdateProjectVisibility")
            }
            CodebuildActions::UpdateReport => write!(f, "codebuild:UpdateReport"),
            CodebuildActions::UpdateReportGroup => write!(f, "codebuild:UpdateReportGroup"),
            CodebuildActions::UpdateWebhook => write!(f, "codebuild:UpdateWebhook"),
        }
    }
}
