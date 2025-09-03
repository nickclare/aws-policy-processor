// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum NeptuneDbActions {
    CancelLoaderJob,
    CancelMlDataProcessingJob,
    CancelMlModelTrainingJob,
    CancelMlModelTransformJob,
    CancelQuery,
    Connect,
    CreateMlEndpoint,
    DeleteDataViaQuery,
    DeleteMlEndpoint,
    DeleteStatistics,
    GetEngineStatus,
    GetGraphSummary,
    GetLoaderJobStatus,
    GetMlDataProcessingJobStatus,
    GetMlEndpointStatus,
    GetMlModelTrainingJobStatus,
    GetMlModelTransformJobStatus,
    GetQueryStatus,
    GetStatisticsStatus,
    GetStreamRecords,
    ListLoaderJobs,
    ListMlDataProcessingJobs,
    ListMlEndpoints,
    ListMlModelTrainingJobs,
    ListMlModelTransformJobs,
    ManageStatistics,
    ReadDataViaQuery,
    ResetDatabase,
    StartLoaderJob,
    StartMlDataProcessingJob,
    StartMlModelTrainingJob,
    StartMlModelTransformJob,
    WriteDataViaQuery,
}
impl std::fmt::Display for NeptuneDbActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NeptuneDbActions::CancelLoaderJob => write!(f, "neptune-db:CancelLoaderJob"),
            NeptuneDbActions::CancelMlDataProcessingJob => {
                write!(f, "neptune-db:CancelMLDataProcessingJob")
            }
            NeptuneDbActions::CancelMlModelTrainingJob => {
                write!(f, "neptune-db:CancelMLModelTrainingJob")
            }
            NeptuneDbActions::CancelMlModelTransformJob => {
                write!(f, "neptune-db:CancelMLModelTransformJob")
            }
            NeptuneDbActions::CancelQuery => write!(f, "neptune-db:CancelQuery"),
            NeptuneDbActions::Connect => write!(f, "neptune-db:connect"),
            NeptuneDbActions::CreateMlEndpoint => write!(f, "neptune-db:CreateMLEndpoint"),
            NeptuneDbActions::DeleteDataViaQuery => write!(f, "neptune-db:DeleteDataViaQuery"),
            NeptuneDbActions::DeleteMlEndpoint => write!(f, "neptune-db:DeleteMLEndpoint"),
            NeptuneDbActions::DeleteStatistics => write!(f, "neptune-db:DeleteStatistics"),
            NeptuneDbActions::GetEngineStatus => write!(f, "neptune-db:GetEngineStatus"),
            NeptuneDbActions::GetGraphSummary => write!(f, "neptune-db:GetGraphSummary"),
            NeptuneDbActions::GetLoaderJobStatus => write!(f, "neptune-db:GetLoaderJobStatus"),
            NeptuneDbActions::GetMlDataProcessingJobStatus => {
                write!(f, "neptune-db:GetMLDataProcessingJobStatus")
            }
            NeptuneDbActions::GetMlEndpointStatus => write!(f, "neptune-db:GetMLEndpointStatus"),
            NeptuneDbActions::GetMlModelTrainingJobStatus => {
                write!(f, "neptune-db:GetMLModelTrainingJobStatus")
            }
            NeptuneDbActions::GetMlModelTransformJobStatus => {
                write!(f, "neptune-db:GetMLModelTransformJobStatus")
            }
            NeptuneDbActions::GetQueryStatus => write!(f, "neptune-db:GetQueryStatus"),
            NeptuneDbActions::GetStatisticsStatus => write!(f, "neptune-db:GetStatisticsStatus"),
            NeptuneDbActions::GetStreamRecords => write!(f, "neptune-db:GetStreamRecords"),
            NeptuneDbActions::ListLoaderJobs => write!(f, "neptune-db:ListLoaderJobs"),
            NeptuneDbActions::ListMlDataProcessingJobs => {
                write!(f, "neptune-db:ListMLDataProcessingJobs")
            }
            NeptuneDbActions::ListMlEndpoints => write!(f, "neptune-db:ListMLEndpoints"),
            NeptuneDbActions::ListMlModelTrainingJobs => {
                write!(f, "neptune-db:ListMLModelTrainingJobs")
            }
            NeptuneDbActions::ListMlModelTransformJobs => {
                write!(f, "neptune-db:ListMLModelTransformJobs")
            }
            NeptuneDbActions::ManageStatistics => write!(f, "neptune-db:ManageStatistics"),
            NeptuneDbActions::ReadDataViaQuery => write!(f, "neptune-db:ReadDataViaQuery"),
            NeptuneDbActions::ResetDatabase => write!(f, "neptune-db:ResetDatabase"),
            NeptuneDbActions::StartLoaderJob => write!(f, "neptune-db:StartLoaderJob"),
            NeptuneDbActions::StartMlDataProcessingJob => {
                write!(f, "neptune-db:StartMLDataProcessingJob")
            }
            NeptuneDbActions::StartMlModelTrainingJob => {
                write!(f, "neptune-db:StartMLModelTrainingJob")
            }
            NeptuneDbActions::StartMlModelTransformJob => {
                write!(f, "neptune-db:StartMLModelTransformJob")
            }
            NeptuneDbActions::WriteDataViaQuery => write!(f, "neptune-db:WriteDataViaQuery"),
        }
    }
}
