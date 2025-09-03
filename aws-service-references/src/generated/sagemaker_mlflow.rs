// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum SagemakerMlflowActions {
    AccessUi,
    CreateExperiment,
    CreateModelVersion,
    CreateRegisteredModel,
    CreateRun,
    DeleteExperiment,
    DeleteLoggedModel,
    DeleteLoggedModelTag,
    DeleteModelVersion,
    DeleteModelVersionTag,
    DeleteRegisteredModel,
    DeleteRegisteredModelAlias,
    DeleteRegisteredModelTag,
    DeleteRun,
    DeleteTag,
    DeleteTraceTag,
    DeleteTraces,
    EndTrace,
    FinalizeLoggedModel,
    GetDownloadUriForModelVersionArtifacts,
    GetExperiment,
    GetExperimentByName,
    GetLatestModelVersions,
    GetLoggedModel,
    GetMetricHistory,
    GetModelVersion,
    GetModelVersionByAlias,
    GetRegisteredModel,
    GetRun,
    GetTraceInfo,
    ListArtifacts,
    ListLoggedModelArtifacts,
    LogBatch,
    LogInputs,
    LogLoggedModelParams,
    LogMetric,
    LogModel,
    LogOutputs,
    LogParam,
    RenameRegisteredModel,
    RestoreExperiment,
    RestoreRun,
    SearchExperiments,
    SearchLoggedModels,
    SearchModelVersions,
    SearchRegisteredModels,
    SearchRuns,
    SearchTraces,
    SetExperimentTag,
    SetLoggedModelTags,
    SetModelVersionTag,
    SetRegisteredModelAlias,
    SetRegisteredModelTag,
    SetTag,
    SetTraceTag,
    StartTrace,
    TransitionModelVersionStage,
    UpdateExperiment,
    UpdateModelVersion,
    UpdateRegisteredModel,
    UpdateRun,
}
impl std::fmt::Display for SagemakerMlflowActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SagemakerMlflowActions::AccessUi => write!(f, "sagemaker-mlflow:AccessUI"),
            SagemakerMlflowActions::CreateExperiment => {
                write!(f, "sagemaker-mlflow:CreateExperiment")
            }
            SagemakerMlflowActions::CreateModelVersion => {
                write!(f, "sagemaker-mlflow:CreateModelVersion")
            }
            SagemakerMlflowActions::CreateRegisteredModel => {
                write!(f, "sagemaker-mlflow:CreateRegisteredModel")
            }
            SagemakerMlflowActions::CreateRun => write!(f, "sagemaker-mlflow:CreateRun"),
            SagemakerMlflowActions::DeleteExperiment => {
                write!(f, "sagemaker-mlflow:DeleteExperiment")
            }
            SagemakerMlflowActions::DeleteLoggedModel => {
                write!(f, "sagemaker-mlflow:DeleteLoggedModel")
            }
            SagemakerMlflowActions::DeleteLoggedModelTag => {
                write!(f, "sagemaker-mlflow:DeleteLoggedModelTag")
            }
            SagemakerMlflowActions::DeleteModelVersion => {
                write!(f, "sagemaker-mlflow:DeleteModelVersion")
            }
            SagemakerMlflowActions::DeleteModelVersionTag => {
                write!(f, "sagemaker-mlflow:DeleteModelVersionTag")
            }
            SagemakerMlflowActions::DeleteRegisteredModel => {
                write!(f, "sagemaker-mlflow:DeleteRegisteredModel")
            }
            SagemakerMlflowActions::DeleteRegisteredModelAlias => {
                write!(f, "sagemaker-mlflow:DeleteRegisteredModelAlias")
            }
            SagemakerMlflowActions::DeleteRegisteredModelTag => {
                write!(f, "sagemaker-mlflow:DeleteRegisteredModelTag")
            }
            SagemakerMlflowActions::DeleteRun => write!(f, "sagemaker-mlflow:DeleteRun"),
            SagemakerMlflowActions::DeleteTag => write!(f, "sagemaker-mlflow:DeleteTag"),
            SagemakerMlflowActions::DeleteTraceTag => write!(f, "sagemaker-mlflow:DeleteTraceTag"),
            SagemakerMlflowActions::DeleteTraces => write!(f, "sagemaker-mlflow:DeleteTraces"),
            SagemakerMlflowActions::EndTrace => write!(f, "sagemaker-mlflow:EndTrace"),
            SagemakerMlflowActions::FinalizeLoggedModel => {
                write!(f, "sagemaker-mlflow:FinalizeLoggedModel")
            }
            SagemakerMlflowActions::GetDownloadUriForModelVersionArtifacts => {
                write!(f, "sagemaker-mlflow:GetDownloadURIForModelVersionArtifacts")
            }
            SagemakerMlflowActions::GetExperiment => write!(f, "sagemaker-mlflow:GetExperiment"),
            SagemakerMlflowActions::GetExperimentByName => {
                write!(f, "sagemaker-mlflow:GetExperimentByName")
            }
            SagemakerMlflowActions::GetLatestModelVersions => {
                write!(f, "sagemaker-mlflow:GetLatestModelVersions")
            }
            SagemakerMlflowActions::GetLoggedModel => write!(f, "sagemaker-mlflow:GetLoggedModel"),
            SagemakerMlflowActions::GetMetricHistory => {
                write!(f, "sagemaker-mlflow:GetMetricHistory")
            }
            SagemakerMlflowActions::GetModelVersion => {
                write!(f, "sagemaker-mlflow:GetModelVersion")
            }
            SagemakerMlflowActions::GetModelVersionByAlias => {
                write!(f, "sagemaker-mlflow:GetModelVersionByAlias")
            }
            SagemakerMlflowActions::GetRegisteredModel => {
                write!(f, "sagemaker-mlflow:GetRegisteredModel")
            }
            SagemakerMlflowActions::GetRun => write!(f, "sagemaker-mlflow:GetRun"),
            SagemakerMlflowActions::GetTraceInfo => write!(f, "sagemaker-mlflow:GetTraceInfo"),
            SagemakerMlflowActions::ListArtifacts => write!(f, "sagemaker-mlflow:ListArtifacts"),
            SagemakerMlflowActions::ListLoggedModelArtifacts => {
                write!(f, "sagemaker-mlflow:ListLoggedModelArtifacts")
            }
            SagemakerMlflowActions::LogBatch => write!(f, "sagemaker-mlflow:LogBatch"),
            SagemakerMlflowActions::LogInputs => write!(f, "sagemaker-mlflow:LogInputs"),
            SagemakerMlflowActions::LogLoggedModelParams => {
                write!(f, "sagemaker-mlflow:LogLoggedModelParams")
            }
            SagemakerMlflowActions::LogMetric => write!(f, "sagemaker-mlflow:LogMetric"),
            SagemakerMlflowActions::LogModel => write!(f, "sagemaker-mlflow:LogModel"),
            SagemakerMlflowActions::LogOutputs => write!(f, "sagemaker-mlflow:LogOutputs"),
            SagemakerMlflowActions::LogParam => write!(f, "sagemaker-mlflow:LogParam"),
            SagemakerMlflowActions::RenameRegisteredModel => {
                write!(f, "sagemaker-mlflow:RenameRegisteredModel")
            }
            SagemakerMlflowActions::RestoreExperiment => {
                write!(f, "sagemaker-mlflow:RestoreExperiment")
            }
            SagemakerMlflowActions::RestoreRun => write!(f, "sagemaker-mlflow:RestoreRun"),
            SagemakerMlflowActions::SearchExperiments => {
                write!(f, "sagemaker-mlflow:SearchExperiments")
            }
            SagemakerMlflowActions::SearchLoggedModels => {
                write!(f, "sagemaker-mlflow:SearchLoggedModels")
            }
            SagemakerMlflowActions::SearchModelVersions => {
                write!(f, "sagemaker-mlflow:SearchModelVersions")
            }
            SagemakerMlflowActions::SearchRegisteredModels => {
                write!(f, "sagemaker-mlflow:SearchRegisteredModels")
            }
            SagemakerMlflowActions::SearchRuns => write!(f, "sagemaker-mlflow:SearchRuns"),
            SagemakerMlflowActions::SearchTraces => write!(f, "sagemaker-mlflow:SearchTraces"),
            SagemakerMlflowActions::SetExperimentTag => {
                write!(f, "sagemaker-mlflow:SetExperimentTag")
            }
            SagemakerMlflowActions::SetLoggedModelTags => {
                write!(f, "sagemaker-mlflow:SetLoggedModelTags")
            }
            SagemakerMlflowActions::SetModelVersionTag => {
                write!(f, "sagemaker-mlflow:SetModelVersionTag")
            }
            SagemakerMlflowActions::SetRegisteredModelAlias => {
                write!(f, "sagemaker-mlflow:SetRegisteredModelAlias")
            }
            SagemakerMlflowActions::SetRegisteredModelTag => {
                write!(f, "sagemaker-mlflow:SetRegisteredModelTag")
            }
            SagemakerMlflowActions::SetTag => write!(f, "sagemaker-mlflow:SetTag"),
            SagemakerMlflowActions::SetTraceTag => write!(f, "sagemaker-mlflow:SetTraceTag"),
            SagemakerMlflowActions::StartTrace => write!(f, "sagemaker-mlflow:StartTrace"),
            SagemakerMlflowActions::TransitionModelVersionStage => {
                write!(f, "sagemaker-mlflow:TransitionModelVersionStage")
            }
            SagemakerMlflowActions::UpdateExperiment => {
                write!(f, "sagemaker-mlflow:UpdateExperiment")
            }
            SagemakerMlflowActions::UpdateModelVersion => {
                write!(f, "sagemaker-mlflow:UpdateModelVersion")
            }
            SagemakerMlflowActions::UpdateRegisteredModel => {
                write!(f, "sagemaker-mlflow:UpdateRegisteredModel")
            }
            SagemakerMlflowActions::UpdateRun => write!(f, "sagemaker-mlflow:UpdateRun"),
        }
    }
}
