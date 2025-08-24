// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum MachinelearningActions {
    AddTags,
    CreateBatchPrediction,
    CreateDataSourceFromRds,
    CreateDataSourceFromRedshift,
    CreateDataSourceFromS3,
    CreateEvaluation,
    CreateMlModel,
    CreateRealtimeEndpoint,
    DeleteBatchPrediction,
    DeleteDataSource,
    DeleteEvaluation,
    DeleteMlModel,
    DeleteRealtimeEndpoint,
    DeleteTags,
    DescribeBatchPredictions,
    DescribeDataSources,
    DescribeEvaluations,
    DescribeMlModels,
    DescribeTags,
    GetBatchPrediction,
    GetDataSource,
    GetEvaluation,
    GetMlModel,
    Predict,
    UpdateBatchPrediction,
    UpdateDataSource,
    UpdateEvaluation,
    UpdateMlModel,
}
impl std::fmt::Display for MachinelearningActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MachinelearningActions::AddTags => write!(f, "machinelearning:AddTags"),
            MachinelearningActions::CreateBatchPrediction => {
                write!(f, "machinelearning:CreateBatchPrediction")
            }
            MachinelearningActions::CreateDataSourceFromRds => {
                write!(f, "machinelearning:CreateDataSourceFromRDS")
            }
            MachinelearningActions::CreateDataSourceFromRedshift => {
                write!(f, "machinelearning:CreateDataSourceFromRedshift")
            }
            MachinelearningActions::CreateDataSourceFromS3 => {
                write!(f, "machinelearning:CreateDataSourceFromS3")
            }
            MachinelearningActions::CreateEvaluation => {
                write!(f, "machinelearning:CreateEvaluation")
            }
            MachinelearningActions::CreateMlModel => write!(f, "machinelearning:CreateMLModel"),
            MachinelearningActions::CreateRealtimeEndpoint => {
                write!(f, "machinelearning:CreateRealtimeEndpoint")
            }
            MachinelearningActions::DeleteBatchPrediction => {
                write!(f, "machinelearning:DeleteBatchPrediction")
            }
            MachinelearningActions::DeleteDataSource => {
                write!(f, "machinelearning:DeleteDataSource")
            }
            MachinelearningActions::DeleteEvaluation => {
                write!(f, "machinelearning:DeleteEvaluation")
            }
            MachinelearningActions::DeleteMlModel => write!(f, "machinelearning:DeleteMLModel"),
            MachinelearningActions::DeleteRealtimeEndpoint => {
                write!(f, "machinelearning:DeleteRealtimeEndpoint")
            }
            MachinelearningActions::DeleteTags => write!(f, "machinelearning:DeleteTags"),
            MachinelearningActions::DescribeBatchPredictions => {
                write!(f, "machinelearning:DescribeBatchPredictions")
            }
            MachinelearningActions::DescribeDataSources => {
                write!(f, "machinelearning:DescribeDataSources")
            }
            MachinelearningActions::DescribeEvaluations => {
                write!(f, "machinelearning:DescribeEvaluations")
            }
            MachinelearningActions::DescribeMlModels => {
                write!(f, "machinelearning:DescribeMLModels")
            }
            MachinelearningActions::DescribeTags => write!(f, "machinelearning:DescribeTags"),
            MachinelearningActions::GetBatchPrediction => {
                write!(f, "machinelearning:GetBatchPrediction")
            }
            MachinelearningActions::GetDataSource => write!(f, "machinelearning:GetDataSource"),
            MachinelearningActions::GetEvaluation => write!(f, "machinelearning:GetEvaluation"),
            MachinelearningActions::GetMlModel => write!(f, "machinelearning:GetMLModel"),
            MachinelearningActions::Predict => write!(f, "machinelearning:Predict"),
            MachinelearningActions::UpdateBatchPrediction => {
                write!(f, "machinelearning:UpdateBatchPrediction")
            }
            MachinelearningActions::UpdateDataSource => {
                write!(f, "machinelearning:UpdateDataSource")
            }
            MachinelearningActions::UpdateEvaluation => {
                write!(f, "machinelearning:UpdateEvaluation")
            }
            MachinelearningActions::UpdateMlModel => write!(f, "machinelearning:UpdateMLModel"),
        }
    }
}
