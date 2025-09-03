// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum CodepipelineActions {
    AcknowledgeJob,
    AcknowledgeThirdPartyJob,
    CreateCustomActionType,
    CreatePipeline,
    DeleteCustomActionType,
    DeletePipeline,
    DeleteWebhook,
    DeregisterWebhookWithThirdParty,
    DisableStageTransition,
    EnableStageTransition,
    GetActionType,
    GetJobDetails,
    GetPipeline,
    GetPipelineExecution,
    GetPipelineState,
    GetThirdPartyJobDetails,
    ListActionExecutions,
    ListActionTypes,
    ListDeployActionExecutionTargets,
    ListPipelineExecutions,
    ListPipelines,
    ListRuleExecutions,
    ListRuleTypes,
    ListTagsForResource,
    ListWebhooks,
    OverrideStageCondition,
    PollForJobs,
    PollForThirdPartyJobs,
    PutActionRevision,
    PutApprovalResult,
    PutJobFailureResult,
    PutJobSuccessResult,
    PutThirdPartyJobFailureResult,
    PutThirdPartyJobSuccessResult,
    PutWebhook,
    RegisterWebhookWithThirdParty,
    RetryStageExecution,
    RollbackStage,
    StartPipelineExecution,
    StopPipelineExecution,
    TagResource,
    UntagResource,
    UpdateActionType,
    UpdatePipeline,
}
impl std::fmt::Display for CodepipelineActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CodepipelineActions::AcknowledgeJob => write!(f, "codepipeline:AcknowledgeJob"),
            CodepipelineActions::AcknowledgeThirdPartyJob => {
                write!(f, "codepipeline:AcknowledgeThirdPartyJob")
            }
            CodepipelineActions::CreateCustomActionType => {
                write!(f, "codepipeline:CreateCustomActionType")
            }
            CodepipelineActions::CreatePipeline => write!(f, "codepipeline:CreatePipeline"),
            CodepipelineActions::DeleteCustomActionType => {
                write!(f, "codepipeline:DeleteCustomActionType")
            }
            CodepipelineActions::DeletePipeline => write!(f, "codepipeline:DeletePipeline"),
            CodepipelineActions::DeleteWebhook => write!(f, "codepipeline:DeleteWebhook"),
            CodepipelineActions::DeregisterWebhookWithThirdParty => {
                write!(f, "codepipeline:DeregisterWebhookWithThirdParty")
            }
            CodepipelineActions::DisableStageTransition => {
                write!(f, "codepipeline:DisableStageTransition")
            }
            CodepipelineActions::EnableStageTransition => {
                write!(f, "codepipeline:EnableStageTransition")
            }
            CodepipelineActions::GetActionType => write!(f, "codepipeline:GetActionType"),
            CodepipelineActions::GetJobDetails => write!(f, "codepipeline:GetJobDetails"),
            CodepipelineActions::GetPipeline => write!(f, "codepipeline:GetPipeline"),
            CodepipelineActions::GetPipelineExecution => {
                write!(f, "codepipeline:GetPipelineExecution")
            }
            CodepipelineActions::GetPipelineState => write!(f, "codepipeline:GetPipelineState"),
            CodepipelineActions::GetThirdPartyJobDetails => {
                write!(f, "codepipeline:GetThirdPartyJobDetails")
            }
            CodepipelineActions::ListActionExecutions => {
                write!(f, "codepipeline:ListActionExecutions")
            }
            CodepipelineActions::ListActionTypes => write!(f, "codepipeline:ListActionTypes"),
            CodepipelineActions::ListDeployActionExecutionTargets => {
                write!(f, "codepipeline:ListDeployActionExecutionTargets")
            }
            CodepipelineActions::ListPipelineExecutions => {
                write!(f, "codepipeline:ListPipelineExecutions")
            }
            CodepipelineActions::ListPipelines => write!(f, "codepipeline:ListPipelines"),
            CodepipelineActions::ListRuleExecutions => write!(f, "codepipeline:ListRuleExecutions"),
            CodepipelineActions::ListRuleTypes => write!(f, "codepipeline:ListRuleTypes"),
            CodepipelineActions::ListTagsForResource => {
                write!(f, "codepipeline:ListTagsForResource")
            }
            CodepipelineActions::ListWebhooks => write!(f, "codepipeline:ListWebhooks"),
            CodepipelineActions::OverrideStageCondition => {
                write!(f, "codepipeline:OverrideStageCondition")
            }
            CodepipelineActions::PollForJobs => write!(f, "codepipeline:PollForJobs"),
            CodepipelineActions::PollForThirdPartyJobs => {
                write!(f, "codepipeline:PollForThirdPartyJobs")
            }
            CodepipelineActions::PutActionRevision => write!(f, "codepipeline:PutActionRevision"),
            CodepipelineActions::PutApprovalResult => write!(f, "codepipeline:PutApprovalResult"),
            CodepipelineActions::PutJobFailureResult => {
                write!(f, "codepipeline:PutJobFailureResult")
            }
            CodepipelineActions::PutJobSuccessResult => {
                write!(f, "codepipeline:PutJobSuccessResult")
            }
            CodepipelineActions::PutThirdPartyJobFailureResult => {
                write!(f, "codepipeline:PutThirdPartyJobFailureResult")
            }
            CodepipelineActions::PutThirdPartyJobSuccessResult => {
                write!(f, "codepipeline:PutThirdPartyJobSuccessResult")
            }
            CodepipelineActions::PutWebhook => write!(f, "codepipeline:PutWebhook"),
            CodepipelineActions::RegisterWebhookWithThirdParty => {
                write!(f, "codepipeline:RegisterWebhookWithThirdParty")
            }
            CodepipelineActions::RetryStageExecution => {
                write!(f, "codepipeline:RetryStageExecution")
            }
            CodepipelineActions::RollbackStage => write!(f, "codepipeline:RollbackStage"),
            CodepipelineActions::StartPipelineExecution => {
                write!(f, "codepipeline:StartPipelineExecution")
            }
            CodepipelineActions::StopPipelineExecution => {
                write!(f, "codepipeline:StopPipelineExecution")
            }
            CodepipelineActions::TagResource => write!(f, "codepipeline:TagResource"),
            CodepipelineActions::UntagResource => write!(f, "codepipeline:UntagResource"),
            CodepipelineActions::UpdateActionType => write!(f, "codepipeline:UpdateActionType"),
            CodepipelineActions::UpdatePipeline => write!(f, "codepipeline:UpdatePipeline"),
        }
    }
}
