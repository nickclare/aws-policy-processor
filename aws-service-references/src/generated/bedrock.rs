// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum BedrockActions {
    AllowVendedLogDeliveryForResource,
    ApplyGuardrail,
    AssociateAgentCollaborator,
    AssociateAgentKnowledgeBase,
    AssociateThirdPartyKnowledgeBase,
    BatchDeleteEvaluationJob,
    CallWithBearerToken,
    CancelAutomatedReasoningPolicyBuildWorkflow,
    CreateAgent,
    CreateAgentActionGroup,
    CreateAgentAlias,
    CreateAutomatedReasoningPolicy,
    CreateAutomatedReasoningPolicyTestCase,
    CreateAutomatedReasoningPolicyVersion,
    CreateBlueprint,
    CreateBlueprintVersion,
    CreateCustomModel,
    CreateCustomModelDeployment,
    CreateDataAutomationProject,
    CreateDataSource,
    CreateEvaluationJob,
    CreateFlow,
    CreateFlowAlias,
    CreateFlowVersion,
    CreateFoundationModelAgreement,
    CreateGuardrail,
    CreateGuardrailVersion,
    CreateInferenceProfile,
    CreateInvocation,
    CreateKnowledgeBase,
    CreateMarketplaceModelEndpoint,
    CreateModelCopyJob,
    CreateModelCustomizationJob,
    CreateModelEvaluationJob,
    CreateModelImportJob,
    CreateModelInvocationJob,
    CreatePrompt,
    CreatePromptRouter,
    CreatePromptVersion,
    CreateProvisionedModelThroughput,
    CreateSession,
    DeleteAgent,
    DeleteAgentActionGroup,
    DeleteAgentAlias,
    DeleteAgentMemory,
    DeleteAgentVersion,
    DeleteAutomatedReasoningPolicy,
    DeleteAutomatedReasoningPolicyBuildWorkflow,
    DeleteAutomatedReasoningPolicyTestCase,
    DeleteBlueprint,
    DeleteCustomModel,
    DeleteCustomModelDeployment,
    DeleteDataAutomationProject,
    DeleteDataSource,
    DeleteFlow,
    DeleteFlowAlias,
    DeleteFlowVersion,
    DeleteFoundationModelAgreement,
    DeleteGuardrail,
    DeleteImportedModel,
    DeleteInferenceProfile,
    DeleteKnowledgeBase,
    DeleteKnowledgeBaseDocuments,
    DeleteMarketplaceModelAgreement,
    DeleteMarketplaceModelEndpoint,
    DeleteModelInvocationLoggingConfiguration,
    DeletePrompt,
    DeletePromptRouter,
    DeleteProvisionedModelThroughput,
    DeleteResourcePolicy,
    DeleteSession,
    DeregisterMarketplaceModelEndpoint,
    DetectGeneratedContent,
    DisassociateAgentCollaborator,
    DisassociateAgentKnowledgeBase,
    EndSession,
    ExportAutomatedReasoningPolicyVersion,
    GenerateQuery,
    GetAgent,
    GetAgentActionGroup,
    GetAgentAlias,
    GetAgentCollaborator,
    GetAgentKnowledgeBase,
    GetAgentMemory,
    GetAgentVersion,
    GetAsyncInvoke,
    GetAutomatedReasoningPolicy,
    GetAutomatedReasoningPolicyAnnotations,
    GetAutomatedReasoningPolicyBuildWorkflow,
    GetAutomatedReasoningPolicyBuildWorkflowResultAssets,
    GetAutomatedReasoningPolicyNextScenario,
    GetAutomatedReasoningPolicyTestCase,
    GetAutomatedReasoningPolicyTestResult,
    GetBlueprint,
    GetBlueprintRecommendation,
    GetCustomModel,
    GetCustomModelDeployment,
    GetDataAutomationProject,
    GetDataAutomationStatus,
    GetDataSource,
    GetEvaluationJob,
    GetExecutionFlowSnapshot,
    GetFlow,
    GetFlowAlias,
    GetFlowExecution,
    GetFlowVersion,
    GetFoundationModel,
    GetFoundationModelAvailability,
    GetGuardrail,
    GetImportedModel,
    GetInferenceProfile,
    GetIngestionJob,
    GetInvocationStep,
    GetKnowledgeBase,
    GetKnowledgeBaseDocuments,
    GetMarketplaceModelEndpoint,
    GetModelCopyJob,
    GetModelCustomizationJob,
    GetModelEvaluationJob,
    GetModelImportJob,
    GetModelInvocationJob,
    GetModelInvocationLoggingConfiguration,
    GetPrompt,
    GetPromptRouter,
    GetProvisionedModelThroughput,
    GetResourcePolicy,
    GetSession,
    GetUseCaseForModelAccess,
    IngestKnowledgeBaseDocuments,
    InvokeAgent,
    InvokeAutomatedReasoningPolicy,
    InvokeBlueprintRecommendationAsync,
    InvokeBuilder,
    InvokeDataAutomationAsync,
    InvokeFlow,
    InvokeInlineAgent,
    InvokeModel,
    InvokeModelWithResponseStream,
    ListAgentActionGroups,
    ListAgentAliases,
    ListAgentCollaborators,
    ListAgentKnowledgeBases,
    ListAgentVersions,
    ListAgents,
    ListAsyncInvokes,
    ListAutomatedReasoningPolicies,
    ListAutomatedReasoningPolicyBuildWorkflows,
    ListAutomatedReasoningPolicyTestCases,
    ListAutomatedReasoningPolicyTestResults,
    ListBlueprints,
    ListCustomModelDeployments,
    ListCustomModels,
    ListDataAutomationProjects,
    ListDataSources,
    ListEvaluationJobs,
    ListFlowAliases,
    ListFlowExecutionEvents,
    ListFlowExecutions,
    ListFlowVersions,
    ListFlows,
    ListFoundationModelAgreementOffers,
    ListFoundationModels,
    ListGuardrails,
    ListImportedModels,
    ListInferenceProfiles,
    ListIngestionJobs,
    ListInvocationSteps,
    ListInvocations,
    ListKnowledgeBaseDocuments,
    ListKnowledgeBases,
    ListMarketplaceModelEndpoints,
    ListModelCopyJobs,
    ListModelCustomizationJobs,
    ListModelEvaluationJobs,
    ListModelImportJobs,
    ListModelInvocationJobs,
    ListPromptRouters,
    ListPrompts,
    ListProvisionedModelThroughputs,
    ListSessions,
    ListTagsForResource,
    OptimizePrompt,
    PrepareAgent,
    PrepareFlow,
    PutFoundationModelEntitlement,
    PutInvocationStep,
    PutModelInvocationLoggingConfiguration,
    PutResourcePolicy,
    PutUseCaseForModelAccess,
    RegisterMarketplaceModelEndpoint,
    RenderPrompt,
    Rerank,
    Retrieve,
    RetrieveAndGenerate,
    StartAutomatedReasoningPolicyBuildWorkflow,
    StartAutomatedReasoningPolicyTestWorkflow,
    StartFlowExecution,
    StartIngestionJob,
    StopEvaluationJob,
    StopFlowExecution,
    StopIngestionJob,
    StopModelCustomizationJob,
    StopModelInvocationJob,
    TagResource,
    UntagResource,
    UpdateAgent,
    UpdateAgentActionGroup,
    UpdateAgentAlias,
    UpdateAgentCollaborator,
    UpdateAgentKnowledgeBase,
    UpdateAutomatedReasoningPolicy,
    UpdateAutomatedReasoningPolicyAnnotations,
    UpdateAutomatedReasoningPolicyTestCase,
    UpdateBlueprint,
    UpdateDataAutomationProject,
    UpdateDataSource,
    UpdateFlow,
    UpdateFlowAlias,
    UpdateGuardrail,
    UpdateKnowledgeBase,
    UpdateMarketplaceModelEndpoint,
    UpdatePrompt,
    UpdateProvisionedModelThroughput,
    UpdateSession,
    ValidateFlowDefinition,
}
impl std::fmt::Display for BedrockActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BedrockActions::AllowVendedLogDeliveryForResource => {
                write!(f, "bedrock:AllowVendedLogDeliveryForResource")
            }
            BedrockActions::ApplyGuardrail => write!(f, "bedrock:ApplyGuardrail"),
            BedrockActions::AssociateAgentCollaborator => {
                write!(f, "bedrock:AssociateAgentCollaborator")
            }
            BedrockActions::AssociateAgentKnowledgeBase => {
                write!(f, "bedrock:AssociateAgentKnowledgeBase")
            }
            BedrockActions::AssociateThirdPartyKnowledgeBase => {
                write!(f, "bedrock:AssociateThirdPartyKnowledgeBase")
            }
            BedrockActions::BatchDeleteEvaluationJob => {
                write!(f, "bedrock:BatchDeleteEvaluationJob")
            }
            BedrockActions::CallWithBearerToken => write!(f, "bedrock:CallWithBearerToken"),
            BedrockActions::CancelAutomatedReasoningPolicyBuildWorkflow => {
                write!(f, "bedrock:CancelAutomatedReasoningPolicyBuildWorkflow")
            }
            BedrockActions::CreateAgent => write!(f, "bedrock:CreateAgent"),
            BedrockActions::CreateAgentActionGroup => write!(f, "bedrock:CreateAgentActionGroup"),
            BedrockActions::CreateAgentAlias => write!(f, "bedrock:CreateAgentAlias"),
            BedrockActions::CreateAutomatedReasoningPolicy => {
                write!(f, "bedrock:CreateAutomatedReasoningPolicy")
            }
            BedrockActions::CreateAutomatedReasoningPolicyTestCase => {
                write!(f, "bedrock:CreateAutomatedReasoningPolicyTestCase")
            }
            BedrockActions::CreateAutomatedReasoningPolicyVersion => {
                write!(f, "bedrock:CreateAutomatedReasoningPolicyVersion")
            }
            BedrockActions::CreateBlueprint => write!(f, "bedrock:CreateBlueprint"),
            BedrockActions::CreateBlueprintVersion => write!(f, "bedrock:CreateBlueprintVersion"),
            BedrockActions::CreateCustomModel => write!(f, "bedrock:CreateCustomModel"),
            BedrockActions::CreateCustomModelDeployment => {
                write!(f, "bedrock:CreateCustomModelDeployment")
            }
            BedrockActions::CreateDataAutomationProject => {
                write!(f, "bedrock:CreateDataAutomationProject")
            }
            BedrockActions::CreateDataSource => write!(f, "bedrock:CreateDataSource"),
            BedrockActions::CreateEvaluationJob => write!(f, "bedrock:CreateEvaluationJob"),
            BedrockActions::CreateFlow => write!(f, "bedrock:CreateFlow"),
            BedrockActions::CreateFlowAlias => write!(f, "bedrock:CreateFlowAlias"),
            BedrockActions::CreateFlowVersion => write!(f, "bedrock:CreateFlowVersion"),
            BedrockActions::CreateFoundationModelAgreement => {
                write!(f, "bedrock:CreateFoundationModelAgreement")
            }
            BedrockActions::CreateGuardrail => write!(f, "bedrock:CreateGuardrail"),
            BedrockActions::CreateGuardrailVersion => write!(f, "bedrock:CreateGuardrailVersion"),
            BedrockActions::CreateInferenceProfile => write!(f, "bedrock:CreateInferenceProfile"),
            BedrockActions::CreateInvocation => write!(f, "bedrock:CreateInvocation"),
            BedrockActions::CreateKnowledgeBase => write!(f, "bedrock:CreateKnowledgeBase"),
            BedrockActions::CreateMarketplaceModelEndpoint => {
                write!(f, "bedrock:CreateMarketplaceModelEndpoint")
            }
            BedrockActions::CreateModelCopyJob => write!(f, "bedrock:CreateModelCopyJob"),
            BedrockActions::CreateModelCustomizationJob => {
                write!(f, "bedrock:CreateModelCustomizationJob")
            }
            BedrockActions::CreateModelEvaluationJob => {
                write!(f, "bedrock:CreateModelEvaluationJob")
            }
            BedrockActions::CreateModelImportJob => write!(f, "bedrock:CreateModelImportJob"),
            BedrockActions::CreateModelInvocationJob => {
                write!(f, "bedrock:CreateModelInvocationJob")
            }
            BedrockActions::CreatePrompt => write!(f, "bedrock:CreatePrompt"),
            BedrockActions::CreatePromptRouter => write!(f, "bedrock:CreatePromptRouter"),
            BedrockActions::CreatePromptVersion => write!(f, "bedrock:CreatePromptVersion"),
            BedrockActions::CreateProvisionedModelThroughput => {
                write!(f, "bedrock:CreateProvisionedModelThroughput")
            }
            BedrockActions::CreateSession => write!(f, "bedrock:CreateSession"),
            BedrockActions::DeleteAgent => write!(f, "bedrock:DeleteAgent"),
            BedrockActions::DeleteAgentActionGroup => write!(f, "bedrock:DeleteAgentActionGroup"),
            BedrockActions::DeleteAgentAlias => write!(f, "bedrock:DeleteAgentAlias"),
            BedrockActions::DeleteAgentMemory => write!(f, "bedrock:DeleteAgentMemory"),
            BedrockActions::DeleteAgentVersion => write!(f, "bedrock:DeleteAgentVersion"),
            BedrockActions::DeleteAutomatedReasoningPolicy => {
                write!(f, "bedrock:DeleteAutomatedReasoningPolicy")
            }
            BedrockActions::DeleteAutomatedReasoningPolicyBuildWorkflow => {
                write!(f, "bedrock:DeleteAutomatedReasoningPolicyBuildWorkflow")
            }
            BedrockActions::DeleteAutomatedReasoningPolicyTestCase => {
                write!(f, "bedrock:DeleteAutomatedReasoningPolicyTestCase")
            }
            BedrockActions::DeleteBlueprint => write!(f, "bedrock:DeleteBlueprint"),
            BedrockActions::DeleteCustomModel => write!(f, "bedrock:DeleteCustomModel"),
            BedrockActions::DeleteCustomModelDeployment => {
                write!(f, "bedrock:DeleteCustomModelDeployment")
            }
            BedrockActions::DeleteDataAutomationProject => {
                write!(f, "bedrock:DeleteDataAutomationProject")
            }
            BedrockActions::DeleteDataSource => write!(f, "bedrock:DeleteDataSource"),
            BedrockActions::DeleteFlow => write!(f, "bedrock:DeleteFlow"),
            BedrockActions::DeleteFlowAlias => write!(f, "bedrock:DeleteFlowAlias"),
            BedrockActions::DeleteFlowVersion => write!(f, "bedrock:DeleteFlowVersion"),
            BedrockActions::DeleteFoundationModelAgreement => {
                write!(f, "bedrock:DeleteFoundationModelAgreement")
            }
            BedrockActions::DeleteGuardrail => write!(f, "bedrock:DeleteGuardrail"),
            BedrockActions::DeleteImportedModel => write!(f, "bedrock:DeleteImportedModel"),
            BedrockActions::DeleteInferenceProfile => write!(f, "bedrock:DeleteInferenceProfile"),
            BedrockActions::DeleteKnowledgeBase => write!(f, "bedrock:DeleteKnowledgeBase"),
            BedrockActions::DeleteKnowledgeBaseDocuments => {
                write!(f, "bedrock:DeleteKnowledgeBaseDocuments")
            }
            BedrockActions::DeleteMarketplaceModelAgreement => {
                write!(f, "bedrock:DeleteMarketplaceModelAgreement")
            }
            BedrockActions::DeleteMarketplaceModelEndpoint => {
                write!(f, "bedrock:DeleteMarketplaceModelEndpoint")
            }
            BedrockActions::DeleteModelInvocationLoggingConfiguration => {
                write!(f, "bedrock:DeleteModelInvocationLoggingConfiguration")
            }
            BedrockActions::DeletePrompt => write!(f, "bedrock:DeletePrompt"),
            BedrockActions::DeletePromptRouter => write!(f, "bedrock:DeletePromptRouter"),
            BedrockActions::DeleteProvisionedModelThroughput => {
                write!(f, "bedrock:DeleteProvisionedModelThroughput")
            }
            BedrockActions::DeleteResourcePolicy => write!(f, "bedrock:DeleteResourcePolicy"),
            BedrockActions::DeleteSession => write!(f, "bedrock:DeleteSession"),
            BedrockActions::DeregisterMarketplaceModelEndpoint => {
                write!(f, "bedrock:DeregisterMarketplaceModelEndpoint")
            }
            BedrockActions::DetectGeneratedContent => write!(f, "bedrock:DetectGeneratedContent"),
            BedrockActions::DisassociateAgentCollaborator => {
                write!(f, "bedrock:DisassociateAgentCollaborator")
            }
            BedrockActions::DisassociateAgentKnowledgeBase => {
                write!(f, "bedrock:DisassociateAgentKnowledgeBase")
            }
            BedrockActions::EndSession => write!(f, "bedrock:EndSession"),
            BedrockActions::ExportAutomatedReasoningPolicyVersion => {
                write!(f, "bedrock:ExportAutomatedReasoningPolicyVersion")
            }
            BedrockActions::GenerateQuery => write!(f, "bedrock:GenerateQuery"),
            BedrockActions::GetAgent => write!(f, "bedrock:GetAgent"),
            BedrockActions::GetAgentActionGroup => write!(f, "bedrock:GetAgentActionGroup"),
            BedrockActions::GetAgentAlias => write!(f, "bedrock:GetAgentAlias"),
            BedrockActions::GetAgentCollaborator => write!(f, "bedrock:GetAgentCollaborator"),
            BedrockActions::GetAgentKnowledgeBase => write!(f, "bedrock:GetAgentKnowledgeBase"),
            BedrockActions::GetAgentMemory => write!(f, "bedrock:GetAgentMemory"),
            BedrockActions::GetAgentVersion => write!(f, "bedrock:GetAgentVersion"),
            BedrockActions::GetAsyncInvoke => write!(f, "bedrock:GetAsyncInvoke"),
            BedrockActions::GetAutomatedReasoningPolicy => {
                write!(f, "bedrock:GetAutomatedReasoningPolicy")
            }
            BedrockActions::GetAutomatedReasoningPolicyAnnotations => {
                write!(f, "bedrock:GetAutomatedReasoningPolicyAnnotations")
            }
            BedrockActions::GetAutomatedReasoningPolicyBuildWorkflow => {
                write!(f, "bedrock:GetAutomatedReasoningPolicyBuildWorkflow")
            }
            BedrockActions::GetAutomatedReasoningPolicyBuildWorkflowResultAssets => write!(
                f,
                "bedrock:GetAutomatedReasoningPolicyBuildWorkflowResultAssets"
            ),
            BedrockActions::GetAutomatedReasoningPolicyNextScenario => {
                write!(f, "bedrock:GetAutomatedReasoningPolicyNextScenario")
            }
            BedrockActions::GetAutomatedReasoningPolicyTestCase => {
                write!(f, "bedrock:GetAutomatedReasoningPolicyTestCase")
            }
            BedrockActions::GetAutomatedReasoningPolicyTestResult => {
                write!(f, "bedrock:GetAutomatedReasoningPolicyTestResult")
            }
            BedrockActions::GetBlueprint => write!(f, "bedrock:GetBlueprint"),
            BedrockActions::GetBlueprintRecommendation => {
                write!(f, "bedrock:GetBlueprintRecommendation")
            }
            BedrockActions::GetCustomModel => write!(f, "bedrock:GetCustomModel"),
            BedrockActions::GetCustomModelDeployment => {
                write!(f, "bedrock:GetCustomModelDeployment")
            }
            BedrockActions::GetDataAutomationProject => {
                write!(f, "bedrock:GetDataAutomationProject")
            }
            BedrockActions::GetDataAutomationStatus => write!(f, "bedrock:GetDataAutomationStatus"),
            BedrockActions::GetDataSource => write!(f, "bedrock:GetDataSource"),
            BedrockActions::GetEvaluationJob => write!(f, "bedrock:GetEvaluationJob"),
            BedrockActions::GetExecutionFlowSnapshot => {
                write!(f, "bedrock:GetExecutionFlowSnapshot")
            }
            BedrockActions::GetFlow => write!(f, "bedrock:GetFlow"),
            BedrockActions::GetFlowAlias => write!(f, "bedrock:GetFlowAlias"),
            BedrockActions::GetFlowExecution => write!(f, "bedrock:GetFlowExecution"),
            BedrockActions::GetFlowVersion => write!(f, "bedrock:GetFlowVersion"),
            BedrockActions::GetFoundationModel => write!(f, "bedrock:GetFoundationModel"),
            BedrockActions::GetFoundationModelAvailability => {
                write!(f, "bedrock:GetFoundationModelAvailability")
            }
            BedrockActions::GetGuardrail => write!(f, "bedrock:GetGuardrail"),
            BedrockActions::GetImportedModel => write!(f, "bedrock:GetImportedModel"),
            BedrockActions::GetInferenceProfile => write!(f, "bedrock:GetInferenceProfile"),
            BedrockActions::GetIngestionJob => write!(f, "bedrock:GetIngestionJob"),
            BedrockActions::GetInvocationStep => write!(f, "bedrock:GetInvocationStep"),
            BedrockActions::GetKnowledgeBase => write!(f, "bedrock:GetKnowledgeBase"),
            BedrockActions::GetKnowledgeBaseDocuments => {
                write!(f, "bedrock:GetKnowledgeBaseDocuments")
            }
            BedrockActions::GetMarketplaceModelEndpoint => {
                write!(f, "bedrock:GetMarketplaceModelEndpoint")
            }
            BedrockActions::GetModelCopyJob => write!(f, "bedrock:GetModelCopyJob"),
            BedrockActions::GetModelCustomizationJob => {
                write!(f, "bedrock:GetModelCustomizationJob")
            }
            BedrockActions::GetModelEvaluationJob => write!(f, "bedrock:GetModelEvaluationJob"),
            BedrockActions::GetModelImportJob => write!(f, "bedrock:GetModelImportJob"),
            BedrockActions::GetModelInvocationJob => write!(f, "bedrock:GetModelInvocationJob"),
            BedrockActions::GetModelInvocationLoggingConfiguration => {
                write!(f, "bedrock:GetModelInvocationLoggingConfiguration")
            }
            BedrockActions::GetPrompt => write!(f, "bedrock:GetPrompt"),
            BedrockActions::GetPromptRouter => write!(f, "bedrock:GetPromptRouter"),
            BedrockActions::GetProvisionedModelThroughput => {
                write!(f, "bedrock:GetProvisionedModelThroughput")
            }
            BedrockActions::GetResourcePolicy => write!(f, "bedrock:GetResourcePolicy"),
            BedrockActions::GetSession => write!(f, "bedrock:GetSession"),
            BedrockActions::GetUseCaseForModelAccess => {
                write!(f, "bedrock:GetUseCaseForModelAccess")
            }
            BedrockActions::IngestKnowledgeBaseDocuments => {
                write!(f, "bedrock:IngestKnowledgeBaseDocuments")
            }
            BedrockActions::InvokeAgent => write!(f, "bedrock:InvokeAgent"),
            BedrockActions::InvokeAutomatedReasoningPolicy => {
                write!(f, "bedrock:InvokeAutomatedReasoningPolicy")
            }
            BedrockActions::InvokeBlueprintRecommendationAsync => {
                write!(f, "bedrock:InvokeBlueprintRecommendationAsync")
            }
            BedrockActions::InvokeBuilder => write!(f, "bedrock:InvokeBuilder"),
            BedrockActions::InvokeDataAutomationAsync => {
                write!(f, "bedrock:InvokeDataAutomationAsync")
            }
            BedrockActions::InvokeFlow => write!(f, "bedrock:InvokeFlow"),
            BedrockActions::InvokeInlineAgent => write!(f, "bedrock:InvokeInlineAgent"),
            BedrockActions::InvokeModel => write!(f, "bedrock:InvokeModel"),
            BedrockActions::InvokeModelWithResponseStream => {
                write!(f, "bedrock:InvokeModelWithResponseStream")
            }
            BedrockActions::ListAgentActionGroups => write!(f, "bedrock:ListAgentActionGroups"),
            BedrockActions::ListAgentAliases => write!(f, "bedrock:ListAgentAliases"),
            BedrockActions::ListAgentCollaborators => write!(f, "bedrock:ListAgentCollaborators"),
            BedrockActions::ListAgentKnowledgeBases => write!(f, "bedrock:ListAgentKnowledgeBases"),
            BedrockActions::ListAgentVersions => write!(f, "bedrock:ListAgentVersions"),
            BedrockActions::ListAgents => write!(f, "bedrock:ListAgents"),
            BedrockActions::ListAsyncInvokes => write!(f, "bedrock:ListAsyncInvokes"),
            BedrockActions::ListAutomatedReasoningPolicies => {
                write!(f, "bedrock:ListAutomatedReasoningPolicies")
            }
            BedrockActions::ListAutomatedReasoningPolicyBuildWorkflows => {
                write!(f, "bedrock:ListAutomatedReasoningPolicyBuildWorkflows")
            }
            BedrockActions::ListAutomatedReasoningPolicyTestCases => {
                write!(f, "bedrock:ListAutomatedReasoningPolicyTestCases")
            }
            BedrockActions::ListAutomatedReasoningPolicyTestResults => {
                write!(f, "bedrock:ListAutomatedReasoningPolicyTestResults")
            }
            BedrockActions::ListBlueprints => write!(f, "bedrock:ListBlueprints"),
            BedrockActions::ListCustomModelDeployments => {
                write!(f, "bedrock:ListCustomModelDeployments")
            }
            BedrockActions::ListCustomModels => write!(f, "bedrock:ListCustomModels"),
            BedrockActions::ListDataAutomationProjects => {
                write!(f, "bedrock:ListDataAutomationProjects")
            }
            BedrockActions::ListDataSources => write!(f, "bedrock:ListDataSources"),
            BedrockActions::ListEvaluationJobs => write!(f, "bedrock:ListEvaluationJobs"),
            BedrockActions::ListFlowAliases => write!(f, "bedrock:ListFlowAliases"),
            BedrockActions::ListFlowExecutionEvents => write!(f, "bedrock:ListFlowExecutionEvents"),
            BedrockActions::ListFlowExecutions => write!(f, "bedrock:ListFlowExecutions"),
            BedrockActions::ListFlowVersions => write!(f, "bedrock:ListFlowVersions"),
            BedrockActions::ListFlows => write!(f, "bedrock:ListFlows"),
            BedrockActions::ListFoundationModelAgreementOffers => {
                write!(f, "bedrock:ListFoundationModelAgreementOffers")
            }
            BedrockActions::ListFoundationModels => write!(f, "bedrock:ListFoundationModels"),
            BedrockActions::ListGuardrails => write!(f, "bedrock:ListGuardrails"),
            BedrockActions::ListImportedModels => write!(f, "bedrock:ListImportedModels"),
            BedrockActions::ListInferenceProfiles => write!(f, "bedrock:ListInferenceProfiles"),
            BedrockActions::ListIngestionJobs => write!(f, "bedrock:ListIngestionJobs"),
            BedrockActions::ListInvocationSteps => write!(f, "bedrock:ListInvocationSteps"),
            BedrockActions::ListInvocations => write!(f, "bedrock:ListInvocations"),
            BedrockActions::ListKnowledgeBaseDocuments => {
                write!(f, "bedrock:ListKnowledgeBaseDocuments")
            }
            BedrockActions::ListKnowledgeBases => write!(f, "bedrock:ListKnowledgeBases"),
            BedrockActions::ListMarketplaceModelEndpoints => {
                write!(f, "bedrock:ListMarketplaceModelEndpoints")
            }
            BedrockActions::ListModelCopyJobs => write!(f, "bedrock:ListModelCopyJobs"),
            BedrockActions::ListModelCustomizationJobs => {
                write!(f, "bedrock:ListModelCustomizationJobs")
            }
            BedrockActions::ListModelEvaluationJobs => write!(f, "bedrock:ListModelEvaluationJobs"),
            BedrockActions::ListModelImportJobs => write!(f, "bedrock:ListModelImportJobs"),
            BedrockActions::ListModelInvocationJobs => write!(f, "bedrock:ListModelInvocationJobs"),
            BedrockActions::ListPromptRouters => write!(f, "bedrock:ListPromptRouters"),
            BedrockActions::ListPrompts => write!(f, "bedrock:ListPrompts"),
            BedrockActions::ListProvisionedModelThroughputs => {
                write!(f, "bedrock:ListProvisionedModelThroughputs")
            }
            BedrockActions::ListSessions => write!(f, "bedrock:ListSessions"),
            BedrockActions::ListTagsForResource => write!(f, "bedrock:ListTagsForResource"),
            BedrockActions::OptimizePrompt => write!(f, "bedrock:OptimizePrompt"),
            BedrockActions::PrepareAgent => write!(f, "bedrock:PrepareAgent"),
            BedrockActions::PrepareFlow => write!(f, "bedrock:PrepareFlow"),
            BedrockActions::PutFoundationModelEntitlement => {
                write!(f, "bedrock:PutFoundationModelEntitlement")
            }
            BedrockActions::PutInvocationStep => write!(f, "bedrock:PutInvocationStep"),
            BedrockActions::PutModelInvocationLoggingConfiguration => {
                write!(f, "bedrock:PutModelInvocationLoggingConfiguration")
            }
            BedrockActions::PutResourcePolicy => write!(f, "bedrock:PutResourcePolicy"),
            BedrockActions::PutUseCaseForModelAccess => {
                write!(f, "bedrock:PutUseCaseForModelAccess")
            }
            BedrockActions::RegisterMarketplaceModelEndpoint => {
                write!(f, "bedrock:RegisterMarketplaceModelEndpoint")
            }
            BedrockActions::RenderPrompt => write!(f, "bedrock:RenderPrompt"),
            BedrockActions::Rerank => write!(f, "bedrock:Rerank"),
            BedrockActions::Retrieve => write!(f, "bedrock:Retrieve"),
            BedrockActions::RetrieveAndGenerate => write!(f, "bedrock:RetrieveAndGenerate"),
            BedrockActions::StartAutomatedReasoningPolicyBuildWorkflow => {
                write!(f, "bedrock:StartAutomatedReasoningPolicyBuildWorkflow")
            }
            BedrockActions::StartAutomatedReasoningPolicyTestWorkflow => {
                write!(f, "bedrock:StartAutomatedReasoningPolicyTestWorkflow")
            }
            BedrockActions::StartFlowExecution => write!(f, "bedrock:StartFlowExecution"),
            BedrockActions::StartIngestionJob => write!(f, "bedrock:StartIngestionJob"),
            BedrockActions::StopEvaluationJob => write!(f, "bedrock:StopEvaluationJob"),
            BedrockActions::StopFlowExecution => write!(f, "bedrock:StopFlowExecution"),
            BedrockActions::StopIngestionJob => write!(f, "bedrock:StopIngestionJob"),
            BedrockActions::StopModelCustomizationJob => {
                write!(f, "bedrock:StopModelCustomizationJob")
            }
            BedrockActions::StopModelInvocationJob => write!(f, "bedrock:StopModelInvocationJob"),
            BedrockActions::TagResource => write!(f, "bedrock:TagResource"),
            BedrockActions::UntagResource => write!(f, "bedrock:UntagResource"),
            BedrockActions::UpdateAgent => write!(f, "bedrock:UpdateAgent"),
            BedrockActions::UpdateAgentActionGroup => write!(f, "bedrock:UpdateAgentActionGroup"),
            BedrockActions::UpdateAgentAlias => write!(f, "bedrock:UpdateAgentAlias"),
            BedrockActions::UpdateAgentCollaborator => write!(f, "bedrock:UpdateAgentCollaborator"),
            BedrockActions::UpdateAgentKnowledgeBase => {
                write!(f, "bedrock:UpdateAgentKnowledgeBase")
            }
            BedrockActions::UpdateAutomatedReasoningPolicy => {
                write!(f, "bedrock:UpdateAutomatedReasoningPolicy")
            }
            BedrockActions::UpdateAutomatedReasoningPolicyAnnotations => {
                write!(f, "bedrock:UpdateAutomatedReasoningPolicyAnnotations")
            }
            BedrockActions::UpdateAutomatedReasoningPolicyTestCase => {
                write!(f, "bedrock:UpdateAutomatedReasoningPolicyTestCase")
            }
            BedrockActions::UpdateBlueprint => write!(f, "bedrock:UpdateBlueprint"),
            BedrockActions::UpdateDataAutomationProject => {
                write!(f, "bedrock:UpdateDataAutomationProject")
            }
            BedrockActions::UpdateDataSource => write!(f, "bedrock:UpdateDataSource"),
            BedrockActions::UpdateFlow => write!(f, "bedrock:UpdateFlow"),
            BedrockActions::UpdateFlowAlias => write!(f, "bedrock:UpdateFlowAlias"),
            BedrockActions::UpdateGuardrail => write!(f, "bedrock:UpdateGuardrail"),
            BedrockActions::UpdateKnowledgeBase => write!(f, "bedrock:UpdateKnowledgeBase"),
            BedrockActions::UpdateMarketplaceModelEndpoint => {
                write!(f, "bedrock:UpdateMarketplaceModelEndpoint")
            }
            BedrockActions::UpdatePrompt => write!(f, "bedrock:UpdatePrompt"),
            BedrockActions::UpdateProvisionedModelThroughput => {
                write!(f, "bedrock:UpdateProvisionedModelThroughput")
            }
            BedrockActions::UpdateSession => write!(f, "bedrock:UpdateSession"),
            BedrockActions::ValidateFlowDefinition => write!(f, "bedrock:ValidateFlowDefinition"),
        }
    }
}
