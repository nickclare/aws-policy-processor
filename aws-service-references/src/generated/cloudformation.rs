// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum CloudformationActions {
    ActivateOrganizationsAccess,
    ActivateType,
    BatchDescribeTypeConfigurations,
    CancelResourceRequest,
    CancelUpdateStack,
    ContinueUpdateRollback,
    CreateChangeSet,
    CreateGeneratedTemplate,
    CreateResource,
    CreateStack,
    CreateStackInstances,
    CreateStackRefactor,
    CreateStackSet,
    CreateUploadBucket,
    DeactivateOrganizationsAccess,
    DeactivateType,
    DeleteChangeSet,
    DeleteGeneratedTemplate,
    DeleteResource,
    DeleteStack,
    DeleteStackInstances,
    DeleteStackSet,
    DeregisterType,
    DescribeAccountLimits,
    DescribeChangeSet,
    DescribeChangeSetHooks,
    DescribeGeneratedTemplate,
    DescribeOrganizationsAccess,
    DescribePublisher,
    DescribeResourceScan,
    DescribeStackDriftDetectionStatus,
    DescribeStackEvents,
    DescribeStackInstance,
    DescribeStackRefactor,
    DescribeStackResource,
    DescribeStackResourceDrifts,
    DescribeStackResources,
    DescribeStackSet,
    DescribeStackSetOperation,
    DescribeStacks,
    DescribeType,
    DescribeTypeRegistration,
    DetectStackDrift,
    DetectStackResourceDrift,
    DetectStackSetDrift,
    EstimateTemplateCost,
    ExecuteChangeSet,
    ExecuteStackRefactor,
    GetGeneratedTemplate,
    GetResource,
    GetResourceRequestStatus,
    GetStackPolicy,
    GetTemplate,
    GetTemplateSummary,
    ImportStacksToStackSet,
    ListChangeSets,
    ListExports,
    ListGeneratedTemplates,
    ListHookResults,
    ListImports,
    ListResourceRequests,
    ListResourceScanRelatedResources,
    ListResourceScanResources,
    ListResourceScans,
    ListResources,
    ListStackInstanceResourceDrifts,
    ListStackInstances,
    ListStackRefactorActions,
    ListStackRefactors,
    ListStackResources,
    ListStackSetAutoDeploymentTargets,
    ListStackSetOperationResults,
    ListStackSetOperations,
    ListStackSets,
    ListStacks,
    ListTypeRegistrations,
    ListTypeVersions,
    ListTypes,
    PublishType,
    RecordHandlerProgress,
    RegisterPublisher,
    RegisterType,
    RollbackStack,
    SetStackPolicy,
    SetTypeConfiguration,
    SetTypeDefaultVersion,
    SignalResource,
    StartResourceScan,
    StopStackSetOperation,
    TagResource,
    TestType,
    UntagResource,
    UpdateGeneratedTemplate,
    UpdateResource,
    UpdateStack,
    UpdateStackInstances,
    UpdateStackSet,
    UpdateTerminationProtection,
    ValidateTemplate,
}
impl std::fmt::Display for CloudformationActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CloudformationActions::ActivateOrganizationsAccess => {
                write!(f, "cloudformation:ActivateOrganizationsAccess")
            }
            CloudformationActions::ActivateType => write!(f, "cloudformation:ActivateType"),
            CloudformationActions::BatchDescribeTypeConfigurations => {
                write!(f, "cloudformation:BatchDescribeTypeConfigurations")
            }
            CloudformationActions::CancelResourceRequest => {
                write!(f, "cloudformation:CancelResourceRequest")
            }
            CloudformationActions::CancelUpdateStack => {
                write!(f, "cloudformation:CancelUpdateStack")
            }
            CloudformationActions::ContinueUpdateRollback => {
                write!(f, "cloudformation:ContinueUpdateRollback")
            }
            CloudformationActions::CreateChangeSet => write!(f, "cloudformation:CreateChangeSet"),
            CloudformationActions::CreateGeneratedTemplate => {
                write!(f, "cloudformation:CreateGeneratedTemplate")
            }
            CloudformationActions::CreateResource => write!(f, "cloudformation:CreateResource"),
            CloudformationActions::CreateStack => write!(f, "cloudformation:CreateStack"),
            CloudformationActions::CreateStackInstances => {
                write!(f, "cloudformation:CreateStackInstances")
            }
            CloudformationActions::CreateStackRefactor => {
                write!(f, "cloudformation:CreateStackRefactor")
            }
            CloudformationActions::CreateStackSet => write!(f, "cloudformation:CreateStackSet"),
            CloudformationActions::CreateUploadBucket => {
                write!(f, "cloudformation:CreateUploadBucket")
            }
            CloudformationActions::DeactivateOrganizationsAccess => {
                write!(f, "cloudformation:DeactivateOrganizationsAccess")
            }
            CloudformationActions::DeactivateType => write!(f, "cloudformation:DeactivateType"),
            CloudformationActions::DeleteChangeSet => write!(f, "cloudformation:DeleteChangeSet"),
            CloudformationActions::DeleteGeneratedTemplate => {
                write!(f, "cloudformation:DeleteGeneratedTemplate")
            }
            CloudformationActions::DeleteResource => write!(f, "cloudformation:DeleteResource"),
            CloudformationActions::DeleteStack => write!(f, "cloudformation:DeleteStack"),
            CloudformationActions::DeleteStackInstances => {
                write!(f, "cloudformation:DeleteStackInstances")
            }
            CloudformationActions::DeleteStackSet => write!(f, "cloudformation:DeleteStackSet"),
            CloudformationActions::DeregisterType => write!(f, "cloudformation:DeregisterType"),
            CloudformationActions::DescribeAccountLimits => {
                write!(f, "cloudformation:DescribeAccountLimits")
            }
            CloudformationActions::DescribeChangeSet => {
                write!(f, "cloudformation:DescribeChangeSet")
            }
            CloudformationActions::DescribeChangeSetHooks => {
                write!(f, "cloudformation:DescribeChangeSetHooks")
            }
            CloudformationActions::DescribeGeneratedTemplate => {
                write!(f, "cloudformation:DescribeGeneratedTemplate")
            }
            CloudformationActions::DescribeOrganizationsAccess => {
                write!(f, "cloudformation:DescribeOrganizationsAccess")
            }
            CloudformationActions::DescribePublisher => {
                write!(f, "cloudformation:DescribePublisher")
            }
            CloudformationActions::DescribeResourceScan => {
                write!(f, "cloudformation:DescribeResourceScan")
            }
            CloudformationActions::DescribeStackDriftDetectionStatus => {
                write!(f, "cloudformation:DescribeStackDriftDetectionStatus")
            }
            CloudformationActions::DescribeStackEvents => {
                write!(f, "cloudformation:DescribeStackEvents")
            }
            CloudformationActions::DescribeStackInstance => {
                write!(f, "cloudformation:DescribeStackInstance")
            }
            CloudformationActions::DescribeStackRefactor => {
                write!(f, "cloudformation:DescribeStackRefactor")
            }
            CloudformationActions::DescribeStackResource => {
                write!(f, "cloudformation:DescribeStackResource")
            }
            CloudformationActions::DescribeStackResourceDrifts => {
                write!(f, "cloudformation:DescribeStackResourceDrifts")
            }
            CloudformationActions::DescribeStackResources => {
                write!(f, "cloudformation:DescribeStackResources")
            }
            CloudformationActions::DescribeStackSet => write!(f, "cloudformation:DescribeStackSet"),
            CloudformationActions::DescribeStackSetOperation => {
                write!(f, "cloudformation:DescribeStackSetOperation")
            }
            CloudformationActions::DescribeStacks => write!(f, "cloudformation:DescribeStacks"),
            CloudformationActions::DescribeType => write!(f, "cloudformation:DescribeType"),
            CloudformationActions::DescribeTypeRegistration => {
                write!(f, "cloudformation:DescribeTypeRegistration")
            }
            CloudformationActions::DetectStackDrift => write!(f, "cloudformation:DetectStackDrift"),
            CloudformationActions::DetectStackResourceDrift => {
                write!(f, "cloudformation:DetectStackResourceDrift")
            }
            CloudformationActions::DetectStackSetDrift => {
                write!(f, "cloudformation:DetectStackSetDrift")
            }
            CloudformationActions::EstimateTemplateCost => {
                write!(f, "cloudformation:EstimateTemplateCost")
            }
            CloudformationActions::ExecuteChangeSet => write!(f, "cloudformation:ExecuteChangeSet"),
            CloudformationActions::ExecuteStackRefactor => {
                write!(f, "cloudformation:ExecuteStackRefactor")
            }
            CloudformationActions::GetGeneratedTemplate => {
                write!(f, "cloudformation:GetGeneratedTemplate")
            }
            CloudformationActions::GetResource => write!(f, "cloudformation:GetResource"),
            CloudformationActions::GetResourceRequestStatus => {
                write!(f, "cloudformation:GetResourceRequestStatus")
            }
            CloudformationActions::GetStackPolicy => write!(f, "cloudformation:GetStackPolicy"),
            CloudformationActions::GetTemplate => write!(f, "cloudformation:GetTemplate"),
            CloudformationActions::GetTemplateSummary => {
                write!(f, "cloudformation:GetTemplateSummary")
            }
            CloudformationActions::ImportStacksToStackSet => {
                write!(f, "cloudformation:ImportStacksToStackSet")
            }
            CloudformationActions::ListChangeSets => write!(f, "cloudformation:ListChangeSets"),
            CloudformationActions::ListExports => write!(f, "cloudformation:ListExports"),
            CloudformationActions::ListGeneratedTemplates => {
                write!(f, "cloudformation:ListGeneratedTemplates")
            }
            CloudformationActions::ListHookResults => write!(f, "cloudformation:ListHookResults"),
            CloudformationActions::ListImports => write!(f, "cloudformation:ListImports"),
            CloudformationActions::ListResourceRequests => {
                write!(f, "cloudformation:ListResourceRequests")
            }
            CloudformationActions::ListResourceScanRelatedResources => {
                write!(f, "cloudformation:ListResourceScanRelatedResources")
            }
            CloudformationActions::ListResourceScanResources => {
                write!(f, "cloudformation:ListResourceScanResources")
            }
            CloudformationActions::ListResourceScans => {
                write!(f, "cloudformation:ListResourceScans")
            }
            CloudformationActions::ListResources => write!(f, "cloudformation:ListResources"),
            CloudformationActions::ListStackInstanceResourceDrifts => {
                write!(f, "cloudformation:ListStackInstanceResourceDrifts")
            }
            CloudformationActions::ListStackInstances => {
                write!(f, "cloudformation:ListStackInstances")
            }
            CloudformationActions::ListStackRefactorActions => {
                write!(f, "cloudformation:ListStackRefactorActions")
            }
            CloudformationActions::ListStackRefactors => {
                write!(f, "cloudformation:ListStackRefactors")
            }
            CloudformationActions::ListStackResources => {
                write!(f, "cloudformation:ListStackResources")
            }
            CloudformationActions::ListStackSetAutoDeploymentTargets => {
                write!(f, "cloudformation:ListStackSetAutoDeploymentTargets")
            }
            CloudformationActions::ListStackSetOperationResults => {
                write!(f, "cloudformation:ListStackSetOperationResults")
            }
            CloudformationActions::ListStackSetOperations => {
                write!(f, "cloudformation:ListStackSetOperations")
            }
            CloudformationActions::ListStackSets => write!(f, "cloudformation:ListStackSets"),
            CloudformationActions::ListStacks => write!(f, "cloudformation:ListStacks"),
            CloudformationActions::ListTypeRegistrations => {
                write!(f, "cloudformation:ListTypeRegistrations")
            }
            CloudformationActions::ListTypeVersions => write!(f, "cloudformation:ListTypeVersions"),
            CloudformationActions::ListTypes => write!(f, "cloudformation:ListTypes"),
            CloudformationActions::PublishType => write!(f, "cloudformation:PublishType"),
            CloudformationActions::RecordHandlerProgress => {
                write!(f, "cloudformation:RecordHandlerProgress")
            }
            CloudformationActions::RegisterPublisher => {
                write!(f, "cloudformation:RegisterPublisher")
            }
            CloudformationActions::RegisterType => write!(f, "cloudformation:RegisterType"),
            CloudformationActions::RollbackStack => write!(f, "cloudformation:RollbackStack"),
            CloudformationActions::SetStackPolicy => write!(f, "cloudformation:SetStackPolicy"),
            CloudformationActions::SetTypeConfiguration => {
                write!(f, "cloudformation:SetTypeConfiguration")
            }
            CloudformationActions::SetTypeDefaultVersion => {
                write!(f, "cloudformation:SetTypeDefaultVersion")
            }
            CloudformationActions::SignalResource => write!(f, "cloudformation:SignalResource"),
            CloudformationActions::StartResourceScan => {
                write!(f, "cloudformation:StartResourceScan")
            }
            CloudformationActions::StopStackSetOperation => {
                write!(f, "cloudformation:StopStackSetOperation")
            }
            CloudformationActions::TagResource => write!(f, "cloudformation:TagResource"),
            CloudformationActions::TestType => write!(f, "cloudformation:TestType"),
            CloudformationActions::UntagResource => write!(f, "cloudformation:UntagResource"),
            CloudformationActions::UpdateGeneratedTemplate => {
                write!(f, "cloudformation:UpdateGeneratedTemplate")
            }
            CloudformationActions::UpdateResource => write!(f, "cloudformation:UpdateResource"),
            CloudformationActions::UpdateStack => write!(f, "cloudformation:UpdateStack"),
            CloudformationActions::UpdateStackInstances => {
                write!(f, "cloudformation:UpdateStackInstances")
            }
            CloudformationActions::UpdateStackSet => write!(f, "cloudformation:UpdateStackSet"),
            CloudformationActions::UpdateTerminationProtection => {
                write!(f, "cloudformation:UpdateTerminationProtection")
            }
            CloudformationActions::ValidateTemplate => write!(f, "cloudformation:ValidateTemplate"),
        }
    }
}
