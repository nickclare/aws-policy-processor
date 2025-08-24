// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum NeptuneGraphActions {
    CancelExportTask,
    CancelImportTask,
    CancelQuery,
    CreateGraph,
    CreateGraphSnapshot,
    CreateGraphUsingImportTask,
    CreatePrivateGraphEndpoint,
    DeleteDataViaQuery,
    DeleteGraph,
    DeleteGraphSnapshot,
    DeletePrivateGraphEndpoint,
    GetEngineStatus,
    GetExportTask,
    GetGraph,
    GetGraphSnapshot,
    GetGraphSummary,
    GetImportTask,
    GetPrivateGraphEndpoint,
    GetQueryStatus,
    GetStatisticsStatus,
    ListExportTasks,
    ListGraphSnapshots,
    ListGraphs,
    ListImportTasks,
    ListPrivateGraphEndpoints,
    ListQueries,
    ListTagsForResource,
    ReadDataViaQuery,
    ResetGraph,
    RestoreGraphFromSnapshot,
    StartExportTask,
    StartImportTask,
    TagResource,
    UntagResource,
    UpdateGraph,
    WriteDataViaQuery,
}
impl std::fmt::Display for NeptuneGraphActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NeptuneGraphActions::CancelExportTask => write!(f, "neptune-graph:CancelExportTask"),
            NeptuneGraphActions::CancelImportTask => write!(f, "neptune-graph:CancelImportTask"),
            NeptuneGraphActions::CancelQuery => write!(f, "neptune-graph:CancelQuery"),
            NeptuneGraphActions::CreateGraph => write!(f, "neptune-graph:CreateGraph"),
            NeptuneGraphActions::CreateGraphSnapshot => {
                write!(f, "neptune-graph:CreateGraphSnapshot")
            }
            NeptuneGraphActions::CreateGraphUsingImportTask => {
                write!(f, "neptune-graph:CreateGraphUsingImportTask")
            }
            NeptuneGraphActions::CreatePrivateGraphEndpoint => {
                write!(f, "neptune-graph:CreatePrivateGraphEndpoint")
            }
            NeptuneGraphActions::DeleteDataViaQuery => {
                write!(f, "neptune-graph:DeleteDataViaQuery")
            }
            NeptuneGraphActions::DeleteGraph => write!(f, "neptune-graph:DeleteGraph"),
            NeptuneGraphActions::DeleteGraphSnapshot => {
                write!(f, "neptune-graph:DeleteGraphSnapshot")
            }
            NeptuneGraphActions::DeletePrivateGraphEndpoint => {
                write!(f, "neptune-graph:DeletePrivateGraphEndpoint")
            }
            NeptuneGraphActions::GetEngineStatus => write!(f, "neptune-graph:GetEngineStatus"),
            NeptuneGraphActions::GetExportTask => write!(f, "neptune-graph:GetExportTask"),
            NeptuneGraphActions::GetGraph => write!(f, "neptune-graph:GetGraph"),
            NeptuneGraphActions::GetGraphSnapshot => write!(f, "neptune-graph:GetGraphSnapshot"),
            NeptuneGraphActions::GetGraphSummary => write!(f, "neptune-graph:GetGraphSummary"),
            NeptuneGraphActions::GetImportTask => write!(f, "neptune-graph:GetImportTask"),
            NeptuneGraphActions::GetPrivateGraphEndpoint => {
                write!(f, "neptune-graph:GetPrivateGraphEndpoint")
            }
            NeptuneGraphActions::GetQueryStatus => write!(f, "neptune-graph:GetQueryStatus"),
            NeptuneGraphActions::GetStatisticsStatus => {
                write!(f, "neptune-graph:GetStatisticsStatus")
            }
            NeptuneGraphActions::ListExportTasks => write!(f, "neptune-graph:ListExportTasks"),
            NeptuneGraphActions::ListGraphSnapshots => {
                write!(f, "neptune-graph:ListGraphSnapshots")
            }
            NeptuneGraphActions::ListGraphs => write!(f, "neptune-graph:ListGraphs"),
            NeptuneGraphActions::ListImportTasks => write!(f, "neptune-graph:ListImportTasks"),
            NeptuneGraphActions::ListPrivateGraphEndpoints => {
                write!(f, "neptune-graph:ListPrivateGraphEndpoints")
            }
            NeptuneGraphActions::ListQueries => write!(f, "neptune-graph:ListQueries"),
            NeptuneGraphActions::ListTagsForResource => {
                write!(f, "neptune-graph:ListTagsForResource")
            }
            NeptuneGraphActions::ReadDataViaQuery => write!(f, "neptune-graph:ReadDataViaQuery"),
            NeptuneGraphActions::ResetGraph => write!(f, "neptune-graph:ResetGraph"),
            NeptuneGraphActions::RestoreGraphFromSnapshot => {
                write!(f, "neptune-graph:RestoreGraphFromSnapshot")
            }
            NeptuneGraphActions::StartExportTask => write!(f, "neptune-graph:StartExportTask"),
            NeptuneGraphActions::StartImportTask => write!(f, "neptune-graph:StartImportTask"),
            NeptuneGraphActions::TagResource => write!(f, "neptune-graph:TagResource"),
            NeptuneGraphActions::UntagResource => write!(f, "neptune-graph:UntagResource"),
            NeptuneGraphActions::UpdateGraph => write!(f, "neptune-graph:UpdateGraph"),
            NeptuneGraphActions::WriteDataViaQuery => write!(f, "neptune-graph:WriteDataViaQuery"),
        }
    }
}
