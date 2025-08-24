// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum MigrationhubOrchestratorActions {
    CreateTemplate,
    CreateWorkflow,
    CreateWorkflowStep,
    CreateWorkflowStepGroup,
    DeleteTemplate,
    DeleteWorkflow,
    DeleteWorkflowStep,
    DeleteWorkflowStepGroup,
    GetMessage,
    GetTemplate,
    GetTemplateStep,
    GetTemplateStepGroup,
    GetWorkflow,
    GetWorkflowStep,
    GetWorkflowStepGroup,
    ListPlugins,
    ListTagsForResource,
    ListTemplateStepGroups,
    ListTemplateSteps,
    ListTemplates,
    ListWorkflowStepGroups,
    ListWorkflowSteps,
    ListWorkflows,
    RegisterPlugin,
    RetryWorkflowStep,
    SendMessage,
    StartWorkflow,
    StopWorkflow,
    TagResource,
    UntagResource,
    UpdateTemplate,
    UpdateWorkflow,
    UpdateWorkflowStep,
    UpdateWorkflowStepGroup,
}
impl std::fmt::Display for MigrationhubOrchestratorActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MigrationhubOrchestratorActions::CreateTemplate => {
                write!(f, "migrationhub-orchestrator:CreateTemplate")
            }
            MigrationhubOrchestratorActions::CreateWorkflow => {
                write!(f, "migrationhub-orchestrator:CreateWorkflow")
            }
            MigrationhubOrchestratorActions::CreateWorkflowStep => {
                write!(f, "migrationhub-orchestrator:CreateWorkflowStep")
            }
            MigrationhubOrchestratorActions::CreateWorkflowStepGroup => {
                write!(f, "migrationhub-orchestrator:CreateWorkflowStepGroup")
            }
            MigrationhubOrchestratorActions::DeleteTemplate => {
                write!(f, "migrationhub-orchestrator:DeleteTemplate")
            }
            MigrationhubOrchestratorActions::DeleteWorkflow => {
                write!(f, "migrationhub-orchestrator:DeleteWorkflow")
            }
            MigrationhubOrchestratorActions::DeleteWorkflowStep => {
                write!(f, "migrationhub-orchestrator:DeleteWorkflowStep")
            }
            MigrationhubOrchestratorActions::DeleteWorkflowStepGroup => {
                write!(f, "migrationhub-orchestrator:DeleteWorkflowStepGroup")
            }
            MigrationhubOrchestratorActions::GetMessage => {
                write!(f, "migrationhub-orchestrator:GetMessage")
            }
            MigrationhubOrchestratorActions::GetTemplate => {
                write!(f, "migrationhub-orchestrator:GetTemplate")
            }
            MigrationhubOrchestratorActions::GetTemplateStep => {
                write!(f, "migrationhub-orchestrator:GetTemplateStep")
            }
            MigrationhubOrchestratorActions::GetTemplateStepGroup => {
                write!(f, "migrationhub-orchestrator:GetTemplateStepGroup")
            }
            MigrationhubOrchestratorActions::GetWorkflow => {
                write!(f, "migrationhub-orchestrator:GetWorkflow")
            }
            MigrationhubOrchestratorActions::GetWorkflowStep => {
                write!(f, "migrationhub-orchestrator:GetWorkflowStep")
            }
            MigrationhubOrchestratorActions::GetWorkflowStepGroup => {
                write!(f, "migrationhub-orchestrator:GetWorkflowStepGroup")
            }
            MigrationhubOrchestratorActions::ListPlugins => {
                write!(f, "migrationhub-orchestrator:ListPlugins")
            }
            MigrationhubOrchestratorActions::ListTagsForResource => {
                write!(f, "migrationhub-orchestrator:ListTagsForResource")
            }
            MigrationhubOrchestratorActions::ListTemplateStepGroups => {
                write!(f, "migrationhub-orchestrator:ListTemplateStepGroups")
            }
            MigrationhubOrchestratorActions::ListTemplateSteps => {
                write!(f, "migrationhub-orchestrator:ListTemplateSteps")
            }
            MigrationhubOrchestratorActions::ListTemplates => {
                write!(f, "migrationhub-orchestrator:ListTemplates")
            }
            MigrationhubOrchestratorActions::ListWorkflowStepGroups => {
                write!(f, "migrationhub-orchestrator:ListWorkflowStepGroups")
            }
            MigrationhubOrchestratorActions::ListWorkflowSteps => {
                write!(f, "migrationhub-orchestrator:ListWorkflowSteps")
            }
            MigrationhubOrchestratorActions::ListWorkflows => {
                write!(f, "migrationhub-orchestrator:ListWorkflows")
            }
            MigrationhubOrchestratorActions::RegisterPlugin => {
                write!(f, "migrationhub-orchestrator:RegisterPlugin")
            }
            MigrationhubOrchestratorActions::RetryWorkflowStep => {
                write!(f, "migrationhub-orchestrator:RetryWorkflowStep")
            }
            MigrationhubOrchestratorActions::SendMessage => {
                write!(f, "migrationhub-orchestrator:SendMessage")
            }
            MigrationhubOrchestratorActions::StartWorkflow => {
                write!(f, "migrationhub-orchestrator:StartWorkflow")
            }
            MigrationhubOrchestratorActions::StopWorkflow => {
                write!(f, "migrationhub-orchestrator:StopWorkflow")
            }
            MigrationhubOrchestratorActions::TagResource => {
                write!(f, "migrationhub-orchestrator:TagResource")
            }
            MigrationhubOrchestratorActions::UntagResource => {
                write!(f, "migrationhub-orchestrator:UntagResource")
            }
            MigrationhubOrchestratorActions::UpdateTemplate => {
                write!(f, "migrationhub-orchestrator:UpdateTemplate")
            }
            MigrationhubOrchestratorActions::UpdateWorkflow => {
                write!(f, "migrationhub-orchestrator:UpdateWorkflow")
            }
            MigrationhubOrchestratorActions::UpdateWorkflowStep => {
                write!(f, "migrationhub-orchestrator:UpdateWorkflowStep")
            }
            MigrationhubOrchestratorActions::UpdateWorkflowStepGroup => {
                write!(f, "migrationhub-orchestrator:UpdateWorkflowStepGroup")
            }
        }
    }
}
