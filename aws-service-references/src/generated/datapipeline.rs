// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum DatapipelineActions {
    ActivatePipeline,
    AddTags,
    CreatePipeline,
    DeactivatePipeline,
    DeletePipeline,
    DescribeObjects,
    DescribePipelines,
    EvaluateExpression,
    GetAccountLimits,
    GetPipelineDefinition,
    ListPipelines,
    PollForTask,
    PutAccountLimits,
    PutPipelineDefinition,
    QueryObjects,
    RemoveTags,
    ReportTaskProgress,
    ReportTaskRunnerHeartbeat,
    SetStatus,
    SetTaskStatus,
    ValidatePipelineDefinition,
}
impl std::fmt::Display for DatapipelineActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DatapipelineActions::ActivatePipeline => write!(f, "datapipeline:ActivatePipeline"),
            DatapipelineActions::AddTags => write!(f, "datapipeline:AddTags"),
            DatapipelineActions::CreatePipeline => write!(f, "datapipeline:CreatePipeline"),
            DatapipelineActions::DeactivatePipeline => write!(f, "datapipeline:DeactivatePipeline"),
            DatapipelineActions::DeletePipeline => write!(f, "datapipeline:DeletePipeline"),
            DatapipelineActions::DescribeObjects => write!(f, "datapipeline:DescribeObjects"),
            DatapipelineActions::DescribePipelines => write!(f, "datapipeline:DescribePipelines"),
            DatapipelineActions::EvaluateExpression => write!(f, "datapipeline:EvaluateExpression"),
            DatapipelineActions::GetAccountLimits => write!(f, "datapipeline:GetAccountLimits"),
            DatapipelineActions::GetPipelineDefinition => {
                write!(f, "datapipeline:GetPipelineDefinition")
            }
            DatapipelineActions::ListPipelines => write!(f, "datapipeline:ListPipelines"),
            DatapipelineActions::PollForTask => write!(f, "datapipeline:PollForTask"),
            DatapipelineActions::PutAccountLimits => write!(f, "datapipeline:PutAccountLimits"),
            DatapipelineActions::PutPipelineDefinition => {
                write!(f, "datapipeline:PutPipelineDefinition")
            }
            DatapipelineActions::QueryObjects => write!(f, "datapipeline:QueryObjects"),
            DatapipelineActions::RemoveTags => write!(f, "datapipeline:RemoveTags"),
            DatapipelineActions::ReportTaskProgress => write!(f, "datapipeline:ReportTaskProgress"),
            DatapipelineActions::ReportTaskRunnerHeartbeat => {
                write!(f, "datapipeline:ReportTaskRunnerHeartbeat")
            }
            DatapipelineActions::SetStatus => write!(f, "datapipeline:SetStatus"),
            DatapipelineActions::SetTaskStatus => write!(f, "datapipeline:SetTaskStatus"),
            DatapipelineActions::ValidatePipelineDefinition => {
                write!(f, "datapipeline:ValidatePipelineDefinition")
            }
        }
    }
}
