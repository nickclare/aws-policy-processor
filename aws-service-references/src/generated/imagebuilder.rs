// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum ImagebuilderActions {
    CancelImageCreation,
    CancelLifecycleExecution,
    CreateComponent,
    CreateContainerRecipe,
    CreateDistributionConfiguration,
    CreateImage,
    CreateImagePipeline,
    CreateImageRecipe,
    CreateInfrastructureConfiguration,
    CreateLifecyclePolicy,
    CreateWorkflow,
    DeleteComponent,
    DeleteContainerRecipe,
    DeleteDistributionConfiguration,
    DeleteImage,
    DeleteImagePipeline,
    DeleteImageRecipe,
    DeleteInfrastructureConfiguration,
    DeleteLifecyclePolicy,
    DeleteWorkflow,
    GetComponent,
    GetComponentPolicy,
    GetContainerRecipe,
    GetContainerRecipePolicy,
    GetDistributionConfiguration,
    GetImage,
    GetImagePipeline,
    GetImagePolicy,
    GetImageRecipe,
    GetImageRecipePolicy,
    GetInfrastructureConfiguration,
    GetLifecycleExecution,
    GetLifecyclePolicy,
    GetMarketplaceResource,
    GetWorkflow,
    GetWorkflowExecution,
    GetWorkflowStepExecution,
    ImportComponent,
    ImportDiskImage,
    ImportVmImage,
    ListComponentBuildVersions,
    ListComponents,
    ListContainerRecipes,
    ListDistributionConfigurations,
    ListImageBuildVersions,
    ListImagePackages,
    ListImagePipelineImages,
    ListImagePipelines,
    ListImageRecipes,
    ListImageScanFindingAggregations,
    ListImageScanFindings,
    ListImages,
    ListInfrastructureConfigurations,
    ListLifecycleExecutionResources,
    ListLifecycleExecutions,
    ListLifecyclePolicies,
    ListTagsForResource,
    ListWaitingWorkflowSteps,
    ListWorkflowBuildVersions,
    ListWorkflowExecutions,
    ListWorkflowStepExecutions,
    ListWorkflows,
    PutComponentPolicy,
    PutContainerRecipePolicy,
    PutImagePolicy,
    PutImageRecipePolicy,
    SendWorkflowStepAction,
    StartImagePipelineExecution,
    StartResourceStateUpdate,
    TagResource,
    UntagResource,
    UpdateDistributionConfiguration,
    UpdateImagePipeline,
    UpdateInfrastructureConfiguration,
    UpdateLifecyclePolicy,
}
impl std::fmt::Display for ImagebuilderActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImagebuilderActions::CancelImageCreation => {
                write!(f, "imagebuilder:CancelImageCreation")
            }
            ImagebuilderActions::CancelLifecycleExecution => {
                write!(f, "imagebuilder:CancelLifecycleExecution")
            }
            ImagebuilderActions::CreateComponent => write!(f, "imagebuilder:CreateComponent"),
            ImagebuilderActions::CreateContainerRecipe => {
                write!(f, "imagebuilder:CreateContainerRecipe")
            }
            ImagebuilderActions::CreateDistributionConfiguration => {
                write!(f, "imagebuilder:CreateDistributionConfiguration")
            }
            ImagebuilderActions::CreateImage => write!(f, "imagebuilder:CreateImage"),
            ImagebuilderActions::CreateImagePipeline => {
                write!(f, "imagebuilder:CreateImagePipeline")
            }
            ImagebuilderActions::CreateImageRecipe => write!(f, "imagebuilder:CreateImageRecipe"),
            ImagebuilderActions::CreateInfrastructureConfiguration => {
                write!(f, "imagebuilder:CreateInfrastructureConfiguration")
            }
            ImagebuilderActions::CreateLifecyclePolicy => {
                write!(f, "imagebuilder:CreateLifecyclePolicy")
            }
            ImagebuilderActions::CreateWorkflow => write!(f, "imagebuilder:CreateWorkflow"),
            ImagebuilderActions::DeleteComponent => write!(f, "imagebuilder:DeleteComponent"),
            ImagebuilderActions::DeleteContainerRecipe => {
                write!(f, "imagebuilder:DeleteContainerRecipe")
            }
            ImagebuilderActions::DeleteDistributionConfiguration => {
                write!(f, "imagebuilder:DeleteDistributionConfiguration")
            }
            ImagebuilderActions::DeleteImage => write!(f, "imagebuilder:DeleteImage"),
            ImagebuilderActions::DeleteImagePipeline => {
                write!(f, "imagebuilder:DeleteImagePipeline")
            }
            ImagebuilderActions::DeleteImageRecipe => write!(f, "imagebuilder:DeleteImageRecipe"),
            ImagebuilderActions::DeleteInfrastructureConfiguration => {
                write!(f, "imagebuilder:DeleteInfrastructureConfiguration")
            }
            ImagebuilderActions::DeleteLifecyclePolicy => {
                write!(f, "imagebuilder:DeleteLifecyclePolicy")
            }
            ImagebuilderActions::DeleteWorkflow => write!(f, "imagebuilder:DeleteWorkflow"),
            ImagebuilderActions::GetComponent => write!(f, "imagebuilder:GetComponent"),
            ImagebuilderActions::GetComponentPolicy => write!(f, "imagebuilder:GetComponentPolicy"),
            ImagebuilderActions::GetContainerRecipe => write!(f, "imagebuilder:GetContainerRecipe"),
            ImagebuilderActions::GetContainerRecipePolicy => {
                write!(f, "imagebuilder:GetContainerRecipePolicy")
            }
            ImagebuilderActions::GetDistributionConfiguration => {
                write!(f, "imagebuilder:GetDistributionConfiguration")
            }
            ImagebuilderActions::GetImage => write!(f, "imagebuilder:GetImage"),
            ImagebuilderActions::GetImagePipeline => write!(f, "imagebuilder:GetImagePipeline"),
            ImagebuilderActions::GetImagePolicy => write!(f, "imagebuilder:GetImagePolicy"),
            ImagebuilderActions::GetImageRecipe => write!(f, "imagebuilder:GetImageRecipe"),
            ImagebuilderActions::GetImageRecipePolicy => {
                write!(f, "imagebuilder:GetImageRecipePolicy")
            }
            ImagebuilderActions::GetInfrastructureConfiguration => {
                write!(f, "imagebuilder:GetInfrastructureConfiguration")
            }
            ImagebuilderActions::GetLifecycleExecution => {
                write!(f, "imagebuilder:GetLifecycleExecution")
            }
            ImagebuilderActions::GetLifecyclePolicy => write!(f, "imagebuilder:GetLifecyclePolicy"),
            ImagebuilderActions::GetMarketplaceResource => {
                write!(f, "imagebuilder:GetMarketplaceResource")
            }
            ImagebuilderActions::GetWorkflow => write!(f, "imagebuilder:GetWorkflow"),
            ImagebuilderActions::GetWorkflowExecution => {
                write!(f, "imagebuilder:GetWorkflowExecution")
            }
            ImagebuilderActions::GetWorkflowStepExecution => {
                write!(f, "imagebuilder:GetWorkflowStepExecution")
            }
            ImagebuilderActions::ImportComponent => write!(f, "imagebuilder:ImportComponent"),
            ImagebuilderActions::ImportDiskImage => write!(f, "imagebuilder:ImportDiskImage"),
            ImagebuilderActions::ImportVmImage => write!(f, "imagebuilder:ImportVmImage"),
            ImagebuilderActions::ListComponentBuildVersions => {
                write!(f, "imagebuilder:ListComponentBuildVersions")
            }
            ImagebuilderActions::ListComponents => write!(f, "imagebuilder:ListComponents"),
            ImagebuilderActions::ListContainerRecipes => {
                write!(f, "imagebuilder:ListContainerRecipes")
            }
            ImagebuilderActions::ListDistributionConfigurations => {
                write!(f, "imagebuilder:ListDistributionConfigurations")
            }
            ImagebuilderActions::ListImageBuildVersions => {
                write!(f, "imagebuilder:ListImageBuildVersions")
            }
            ImagebuilderActions::ListImagePackages => write!(f, "imagebuilder:ListImagePackages"),
            ImagebuilderActions::ListImagePipelineImages => {
                write!(f, "imagebuilder:ListImagePipelineImages")
            }
            ImagebuilderActions::ListImagePipelines => write!(f, "imagebuilder:ListImagePipelines"),
            ImagebuilderActions::ListImageRecipes => write!(f, "imagebuilder:ListImageRecipes"),
            ImagebuilderActions::ListImageScanFindingAggregations => {
                write!(f, "imagebuilder:ListImageScanFindingAggregations")
            }
            ImagebuilderActions::ListImageScanFindings => {
                write!(f, "imagebuilder:ListImageScanFindings")
            }
            ImagebuilderActions::ListImages => write!(f, "imagebuilder:ListImages"),
            ImagebuilderActions::ListInfrastructureConfigurations => {
                write!(f, "imagebuilder:ListInfrastructureConfigurations")
            }
            ImagebuilderActions::ListLifecycleExecutionResources => {
                write!(f, "imagebuilder:ListLifecycleExecutionResources")
            }
            ImagebuilderActions::ListLifecycleExecutions => {
                write!(f, "imagebuilder:ListLifecycleExecutions")
            }
            ImagebuilderActions::ListLifecyclePolicies => {
                write!(f, "imagebuilder:ListLifecyclePolicies")
            }
            ImagebuilderActions::ListTagsForResource => {
                write!(f, "imagebuilder:ListTagsForResource")
            }
            ImagebuilderActions::ListWaitingWorkflowSteps => {
                write!(f, "imagebuilder:ListWaitingWorkflowSteps")
            }
            ImagebuilderActions::ListWorkflowBuildVersions => {
                write!(f, "imagebuilder:ListWorkflowBuildVersions")
            }
            ImagebuilderActions::ListWorkflowExecutions => {
                write!(f, "imagebuilder:ListWorkflowExecutions")
            }
            ImagebuilderActions::ListWorkflowStepExecutions => {
                write!(f, "imagebuilder:ListWorkflowStepExecutions")
            }
            ImagebuilderActions::ListWorkflows => write!(f, "imagebuilder:ListWorkflows"),
            ImagebuilderActions::PutComponentPolicy => write!(f, "imagebuilder:PutComponentPolicy"),
            ImagebuilderActions::PutContainerRecipePolicy => {
                write!(f, "imagebuilder:PutContainerRecipePolicy")
            }
            ImagebuilderActions::PutImagePolicy => write!(f, "imagebuilder:PutImagePolicy"),
            ImagebuilderActions::PutImageRecipePolicy => {
                write!(f, "imagebuilder:PutImageRecipePolicy")
            }
            ImagebuilderActions::SendWorkflowStepAction => {
                write!(f, "imagebuilder:SendWorkflowStepAction")
            }
            ImagebuilderActions::StartImagePipelineExecution => {
                write!(f, "imagebuilder:StartImagePipelineExecution")
            }
            ImagebuilderActions::StartResourceStateUpdate => {
                write!(f, "imagebuilder:StartResourceStateUpdate")
            }
            ImagebuilderActions::TagResource => write!(f, "imagebuilder:TagResource"),
            ImagebuilderActions::UntagResource => write!(f, "imagebuilder:UntagResource"),
            ImagebuilderActions::UpdateDistributionConfiguration => {
                write!(f, "imagebuilder:UpdateDistributionConfiguration")
            }
            ImagebuilderActions::UpdateImagePipeline => {
                write!(f, "imagebuilder:UpdateImagePipeline")
            }
            ImagebuilderActions::UpdateInfrastructureConfiguration => {
                write!(f, "imagebuilder:UpdateInfrastructureConfiguration")
            }
            ImagebuilderActions::UpdateLifecyclePolicy => {
                write!(f, "imagebuilder:UpdateLifecyclePolicy")
            }
        }
    }
}
