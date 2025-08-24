// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum SsmActions {
    AddTagsToResource,
    AssociateOpsItemRelatedItem,
    CancelCommand,
    CancelMaintenanceWindowExecution,
    CreateActivation,
    CreateAssociation,
    CreateAssociationBatch,
    CreateDocument,
    CreateMaintenanceWindow,
    CreateOpsItem,
    CreateOpsMetadata,
    CreatePatchBaseline,
    CreateResourceDataSync,
    DeleteActivation,
    DeleteAssociation,
    DeleteDocument,
    DeleteInventory,
    DeleteMaintenanceWindow,
    DeleteOpsItem,
    DeleteOpsMetadata,
    DeleteParameter,
    DeleteParameters,
    DeletePatchBaseline,
    DeleteResourceDataSync,
    DeleteResourcePolicy,
    DeregisterManagedInstance,
    DeregisterPatchBaselineForPatchGroup,
    DeregisterTargetFromMaintenanceWindow,
    DeregisterTaskFromMaintenanceWindow,
    DescribeActivations,
    DescribeAssociation,
    DescribeAssociationExecutionTargets,
    DescribeAssociationExecutions,
    DescribeAutomationExecutions,
    DescribeAutomationStepExecutions,
    DescribeAvailablePatches,
    DescribeDocument,
    DescribeDocumentParameters,
    DescribeDocumentPermission,
    DescribeEffectiveInstanceAssociations,
    DescribeEffectivePatchesForPatchBaseline,
    DescribeInstanceAssociationsStatus,
    DescribeInstanceInformation,
    DescribeInstancePatchStates,
    DescribeInstancePatchStatesForPatchGroup,
    DescribeInstancePatches,
    DescribeInstanceProperties,
    DescribeInventoryDeletions,
    DescribeMaintenanceWindowExecutionTaskInvocations,
    DescribeMaintenanceWindowExecutionTasks,
    DescribeMaintenanceWindowExecutions,
    DescribeMaintenanceWindowSchedule,
    DescribeMaintenanceWindowTargets,
    DescribeMaintenanceWindowTasks,
    DescribeMaintenanceWindows,
    DescribeMaintenanceWindowsForTarget,
    DescribeOpsItems,
    DescribeParameters,
    DescribePatchBaselines,
    DescribePatchGroupState,
    DescribePatchGroups,
    DescribePatchProperties,
    DescribeSessions,
    DisassociateOpsItemRelatedItem,
    ExecuteApi,
    GetAccessToken,
    GetAutomationExecution,
    GetCalendar,
    GetCalendarState,
    GetCommandInvocation,
    GetConnectionStatus,
    GetDefaultPatchBaseline,
    GetDeployablePatchSnapshotForInstance,
    GetDocument,
    GetExecutionPreview,
    GetInventory,
    GetInventorySchema,
    GetMaintenanceWindow,
    GetMaintenanceWindowExecution,
    GetMaintenanceWindowExecutionTask,
    GetMaintenanceWindowExecutionTaskInvocation,
    GetMaintenanceWindowTask,
    GetManifest,
    GetOpsItem,
    GetOpsMetadata,
    GetOpsSummary,
    GetParameter,
    GetParameterHistory,
    GetParameters,
    GetParametersByPath,
    GetPatchBaseline,
    GetPatchBaselineForPatchGroup,
    GetResourcePolicies,
    GetServiceSetting,
    LabelParameterVersion,
    ListAssociationVersions,
    ListAssociations,
    ListCommandInvocations,
    ListCommands,
    ListComplianceItems,
    ListComplianceSummaries,
    ListDocumentMetadataHistory,
    ListDocumentVersions,
    ListDocuments,
    ListInstanceAssociations,
    ListInventoryEntries,
    ListNodes,
    ListNodesSummary,
    ListOpsItemEvents,
    ListOpsItemRelatedItems,
    ListOpsMetadata,
    ListResourceComplianceSummaries,
    ListResourceDataSync,
    ListTagsForResource,
    ModifyDocumentPermission,
    PutCalendar,
    PutComplianceItems,
    PutConfigurePackageResult,
    PutInventory,
    PutParameter,
    PutResourcePolicy,
    RegisterDefaultPatchBaseline,
    RegisterManagedInstance,
    RegisterPatchBaselineForPatchGroup,
    RegisterTargetWithMaintenanceWindow,
    RegisterTaskWithMaintenanceWindow,
    RemoveTagsFromResource,
    ResetServiceSetting,
    ResumeSession,
    SendAutomationSignal,
    SendCommand,
    StartAccessRequest,
    StartAssociationsOnce,
    StartAutomationExecution,
    StartChangeRequestExecution,
    StartExecutionPreview,
    StartSession,
    StopAutomationExecution,
    TerminateSession,
    UnlabelParameterVersion,
    UpdateAssociation,
    UpdateAssociationStatus,
    UpdateDocument,
    UpdateDocumentDefaultVersion,
    UpdateDocumentMetadata,
    UpdateInstanceAssociationStatus,
    UpdateInstanceInformation,
    UpdateMaintenanceWindow,
    UpdateMaintenanceWindowTarget,
    UpdateMaintenanceWindowTask,
    UpdateManagedInstanceRole,
    UpdateOpsItem,
    UpdateOpsMetadata,
    UpdatePatchBaseline,
    UpdateResourceDataSync,
    UpdateServiceSetting,
}
impl std::fmt::Display for SsmActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SsmActions::AddTagsToResource => write!(f, "ssm:AddTagsToResource"),
            SsmActions::AssociateOpsItemRelatedItem => write!(f, "ssm:AssociateOpsItemRelatedItem"),
            SsmActions::CancelCommand => write!(f, "ssm:CancelCommand"),
            SsmActions::CancelMaintenanceWindowExecution => {
                write!(f, "ssm:CancelMaintenanceWindowExecution")
            }
            SsmActions::CreateActivation => write!(f, "ssm:CreateActivation"),
            SsmActions::CreateAssociation => write!(f, "ssm:CreateAssociation"),
            SsmActions::CreateAssociationBatch => write!(f, "ssm:CreateAssociationBatch"),
            SsmActions::CreateDocument => write!(f, "ssm:CreateDocument"),
            SsmActions::CreateMaintenanceWindow => write!(f, "ssm:CreateMaintenanceWindow"),
            SsmActions::CreateOpsItem => write!(f, "ssm:CreateOpsItem"),
            SsmActions::CreateOpsMetadata => write!(f, "ssm:CreateOpsMetadata"),
            SsmActions::CreatePatchBaseline => write!(f, "ssm:CreatePatchBaseline"),
            SsmActions::CreateResourceDataSync => write!(f, "ssm:CreateResourceDataSync"),
            SsmActions::DeleteActivation => write!(f, "ssm:DeleteActivation"),
            SsmActions::DeleteAssociation => write!(f, "ssm:DeleteAssociation"),
            SsmActions::DeleteDocument => write!(f, "ssm:DeleteDocument"),
            SsmActions::DeleteInventory => write!(f, "ssm:DeleteInventory"),
            SsmActions::DeleteMaintenanceWindow => write!(f, "ssm:DeleteMaintenanceWindow"),
            SsmActions::DeleteOpsItem => write!(f, "ssm:DeleteOpsItem"),
            SsmActions::DeleteOpsMetadata => write!(f, "ssm:DeleteOpsMetadata"),
            SsmActions::DeleteParameter => write!(f, "ssm:DeleteParameter"),
            SsmActions::DeleteParameters => write!(f, "ssm:DeleteParameters"),
            SsmActions::DeletePatchBaseline => write!(f, "ssm:DeletePatchBaseline"),
            SsmActions::DeleteResourceDataSync => write!(f, "ssm:DeleteResourceDataSync"),
            SsmActions::DeleteResourcePolicy => write!(f, "ssm:DeleteResourcePolicy"),
            SsmActions::DeregisterManagedInstance => write!(f, "ssm:DeregisterManagedInstance"),
            SsmActions::DeregisterPatchBaselineForPatchGroup => {
                write!(f, "ssm:DeregisterPatchBaselineForPatchGroup")
            }
            SsmActions::DeregisterTargetFromMaintenanceWindow => {
                write!(f, "ssm:DeregisterTargetFromMaintenanceWindow")
            }
            SsmActions::DeregisterTaskFromMaintenanceWindow => {
                write!(f, "ssm:DeregisterTaskFromMaintenanceWindow")
            }
            SsmActions::DescribeActivations => write!(f, "ssm:DescribeActivations"),
            SsmActions::DescribeAssociation => write!(f, "ssm:DescribeAssociation"),
            SsmActions::DescribeAssociationExecutionTargets => {
                write!(f, "ssm:DescribeAssociationExecutionTargets")
            }
            SsmActions::DescribeAssociationExecutions => {
                write!(f, "ssm:DescribeAssociationExecutions")
            }
            SsmActions::DescribeAutomationExecutions => {
                write!(f, "ssm:DescribeAutomationExecutions")
            }
            SsmActions::DescribeAutomationStepExecutions => {
                write!(f, "ssm:DescribeAutomationStepExecutions")
            }
            SsmActions::DescribeAvailablePatches => write!(f, "ssm:DescribeAvailablePatches"),
            SsmActions::DescribeDocument => write!(f, "ssm:DescribeDocument"),
            SsmActions::DescribeDocumentParameters => write!(f, "ssm:DescribeDocumentParameters"),
            SsmActions::DescribeDocumentPermission => write!(f, "ssm:DescribeDocumentPermission"),
            SsmActions::DescribeEffectiveInstanceAssociations => {
                write!(f, "ssm:DescribeEffectiveInstanceAssociations")
            }
            SsmActions::DescribeEffectivePatchesForPatchBaseline => {
                write!(f, "ssm:DescribeEffectivePatchesForPatchBaseline")
            }
            SsmActions::DescribeInstanceAssociationsStatus => {
                write!(f, "ssm:DescribeInstanceAssociationsStatus")
            }
            SsmActions::DescribeInstanceInformation => write!(f, "ssm:DescribeInstanceInformation"),
            SsmActions::DescribeInstancePatchStates => write!(f, "ssm:DescribeInstancePatchStates"),
            SsmActions::DescribeInstancePatchStatesForPatchGroup => {
                write!(f, "ssm:DescribeInstancePatchStatesForPatchGroup")
            }
            SsmActions::DescribeInstancePatches => write!(f, "ssm:DescribeInstancePatches"),
            SsmActions::DescribeInstanceProperties => write!(f, "ssm:DescribeInstanceProperties"),
            SsmActions::DescribeInventoryDeletions => write!(f, "ssm:DescribeInventoryDeletions"),
            SsmActions::DescribeMaintenanceWindowExecutionTaskInvocations => {
                write!(f, "ssm:DescribeMaintenanceWindowExecutionTaskInvocations")
            }
            SsmActions::DescribeMaintenanceWindowExecutionTasks => {
                write!(f, "ssm:DescribeMaintenanceWindowExecutionTasks")
            }
            SsmActions::DescribeMaintenanceWindowExecutions => {
                write!(f, "ssm:DescribeMaintenanceWindowExecutions")
            }
            SsmActions::DescribeMaintenanceWindowSchedule => {
                write!(f, "ssm:DescribeMaintenanceWindowSchedule")
            }
            SsmActions::DescribeMaintenanceWindowTargets => {
                write!(f, "ssm:DescribeMaintenanceWindowTargets")
            }
            SsmActions::DescribeMaintenanceWindowTasks => {
                write!(f, "ssm:DescribeMaintenanceWindowTasks")
            }
            SsmActions::DescribeMaintenanceWindows => write!(f, "ssm:DescribeMaintenanceWindows"),
            SsmActions::DescribeMaintenanceWindowsForTarget => {
                write!(f, "ssm:DescribeMaintenanceWindowsForTarget")
            }
            SsmActions::DescribeOpsItems => write!(f, "ssm:DescribeOpsItems"),
            SsmActions::DescribeParameters => write!(f, "ssm:DescribeParameters"),
            SsmActions::DescribePatchBaselines => write!(f, "ssm:DescribePatchBaselines"),
            SsmActions::DescribePatchGroupState => write!(f, "ssm:DescribePatchGroupState"),
            SsmActions::DescribePatchGroups => write!(f, "ssm:DescribePatchGroups"),
            SsmActions::DescribePatchProperties => write!(f, "ssm:DescribePatchProperties"),
            SsmActions::DescribeSessions => write!(f, "ssm:DescribeSessions"),
            SsmActions::DisassociateOpsItemRelatedItem => {
                write!(f, "ssm:DisassociateOpsItemRelatedItem")
            }
            SsmActions::ExecuteApi => write!(f, "ssm:ExecuteAPI"),
            SsmActions::GetAccessToken => write!(f, "ssm:GetAccessToken"),
            SsmActions::GetAutomationExecution => write!(f, "ssm:GetAutomationExecution"),
            SsmActions::GetCalendar => write!(f, "ssm:GetCalendar"),
            SsmActions::GetCalendarState => write!(f, "ssm:GetCalendarState"),
            SsmActions::GetCommandInvocation => write!(f, "ssm:GetCommandInvocation"),
            SsmActions::GetConnectionStatus => write!(f, "ssm:GetConnectionStatus"),
            SsmActions::GetDefaultPatchBaseline => write!(f, "ssm:GetDefaultPatchBaseline"),
            SsmActions::GetDeployablePatchSnapshotForInstance => {
                write!(f, "ssm:GetDeployablePatchSnapshotForInstance")
            }
            SsmActions::GetDocument => write!(f, "ssm:GetDocument"),
            SsmActions::GetExecutionPreview => write!(f, "ssm:GetExecutionPreview"),
            SsmActions::GetInventory => write!(f, "ssm:GetInventory"),
            SsmActions::GetInventorySchema => write!(f, "ssm:GetInventorySchema"),
            SsmActions::GetMaintenanceWindow => write!(f, "ssm:GetMaintenanceWindow"),
            SsmActions::GetMaintenanceWindowExecution => {
                write!(f, "ssm:GetMaintenanceWindowExecution")
            }
            SsmActions::GetMaintenanceWindowExecutionTask => {
                write!(f, "ssm:GetMaintenanceWindowExecutionTask")
            }
            SsmActions::GetMaintenanceWindowExecutionTaskInvocation => {
                write!(f, "ssm:GetMaintenanceWindowExecutionTaskInvocation")
            }
            SsmActions::GetMaintenanceWindowTask => write!(f, "ssm:GetMaintenanceWindowTask"),
            SsmActions::GetManifest => write!(f, "ssm:GetManifest"),
            SsmActions::GetOpsItem => write!(f, "ssm:GetOpsItem"),
            SsmActions::GetOpsMetadata => write!(f, "ssm:GetOpsMetadata"),
            SsmActions::GetOpsSummary => write!(f, "ssm:GetOpsSummary"),
            SsmActions::GetParameter => write!(f, "ssm:GetParameter"),
            SsmActions::GetParameterHistory => write!(f, "ssm:GetParameterHistory"),
            SsmActions::GetParameters => write!(f, "ssm:GetParameters"),
            SsmActions::GetParametersByPath => write!(f, "ssm:GetParametersByPath"),
            SsmActions::GetPatchBaseline => write!(f, "ssm:GetPatchBaseline"),
            SsmActions::GetPatchBaselineForPatchGroup => {
                write!(f, "ssm:GetPatchBaselineForPatchGroup")
            }
            SsmActions::GetResourcePolicies => write!(f, "ssm:GetResourcePolicies"),
            SsmActions::GetServiceSetting => write!(f, "ssm:GetServiceSetting"),
            SsmActions::LabelParameterVersion => write!(f, "ssm:LabelParameterVersion"),
            SsmActions::ListAssociationVersions => write!(f, "ssm:ListAssociationVersions"),
            SsmActions::ListAssociations => write!(f, "ssm:ListAssociations"),
            SsmActions::ListCommandInvocations => write!(f, "ssm:ListCommandInvocations"),
            SsmActions::ListCommands => write!(f, "ssm:ListCommands"),
            SsmActions::ListComplianceItems => write!(f, "ssm:ListComplianceItems"),
            SsmActions::ListComplianceSummaries => write!(f, "ssm:ListComplianceSummaries"),
            SsmActions::ListDocumentMetadataHistory => write!(f, "ssm:ListDocumentMetadataHistory"),
            SsmActions::ListDocumentVersions => write!(f, "ssm:ListDocumentVersions"),
            SsmActions::ListDocuments => write!(f, "ssm:ListDocuments"),
            SsmActions::ListInstanceAssociations => write!(f, "ssm:ListInstanceAssociations"),
            SsmActions::ListInventoryEntries => write!(f, "ssm:ListInventoryEntries"),
            SsmActions::ListNodes => write!(f, "ssm:ListNodes"),
            SsmActions::ListNodesSummary => write!(f, "ssm:ListNodesSummary"),
            SsmActions::ListOpsItemEvents => write!(f, "ssm:ListOpsItemEvents"),
            SsmActions::ListOpsItemRelatedItems => write!(f, "ssm:ListOpsItemRelatedItems"),
            SsmActions::ListOpsMetadata => write!(f, "ssm:ListOpsMetadata"),
            SsmActions::ListResourceComplianceSummaries => {
                write!(f, "ssm:ListResourceComplianceSummaries")
            }
            SsmActions::ListResourceDataSync => write!(f, "ssm:ListResourceDataSync"),
            SsmActions::ListTagsForResource => write!(f, "ssm:ListTagsForResource"),
            SsmActions::ModifyDocumentPermission => write!(f, "ssm:ModifyDocumentPermission"),
            SsmActions::PutCalendar => write!(f, "ssm:PutCalendar"),
            SsmActions::PutComplianceItems => write!(f, "ssm:PutComplianceItems"),
            SsmActions::PutConfigurePackageResult => write!(f, "ssm:PutConfigurePackageResult"),
            SsmActions::PutInventory => write!(f, "ssm:PutInventory"),
            SsmActions::PutParameter => write!(f, "ssm:PutParameter"),
            SsmActions::PutResourcePolicy => write!(f, "ssm:PutResourcePolicy"),
            SsmActions::RegisterDefaultPatchBaseline => {
                write!(f, "ssm:RegisterDefaultPatchBaseline")
            }
            SsmActions::RegisterManagedInstance => write!(f, "ssm:RegisterManagedInstance"),
            SsmActions::RegisterPatchBaselineForPatchGroup => {
                write!(f, "ssm:RegisterPatchBaselineForPatchGroup")
            }
            SsmActions::RegisterTargetWithMaintenanceWindow => {
                write!(f, "ssm:RegisterTargetWithMaintenanceWindow")
            }
            SsmActions::RegisterTaskWithMaintenanceWindow => {
                write!(f, "ssm:RegisterTaskWithMaintenanceWindow")
            }
            SsmActions::RemoveTagsFromResource => write!(f, "ssm:RemoveTagsFromResource"),
            SsmActions::ResetServiceSetting => write!(f, "ssm:ResetServiceSetting"),
            SsmActions::ResumeSession => write!(f, "ssm:ResumeSession"),
            SsmActions::SendAutomationSignal => write!(f, "ssm:SendAutomationSignal"),
            SsmActions::SendCommand => write!(f, "ssm:SendCommand"),
            SsmActions::StartAccessRequest => write!(f, "ssm:StartAccessRequest"),
            SsmActions::StartAssociationsOnce => write!(f, "ssm:StartAssociationsOnce"),
            SsmActions::StartAutomationExecution => write!(f, "ssm:StartAutomationExecution"),
            SsmActions::StartChangeRequestExecution => write!(f, "ssm:StartChangeRequestExecution"),
            SsmActions::StartExecutionPreview => write!(f, "ssm:StartExecutionPreview"),
            SsmActions::StartSession => write!(f, "ssm:StartSession"),
            SsmActions::StopAutomationExecution => write!(f, "ssm:StopAutomationExecution"),
            SsmActions::TerminateSession => write!(f, "ssm:TerminateSession"),
            SsmActions::UnlabelParameterVersion => write!(f, "ssm:UnlabelParameterVersion"),
            SsmActions::UpdateAssociation => write!(f, "ssm:UpdateAssociation"),
            SsmActions::UpdateAssociationStatus => write!(f, "ssm:UpdateAssociationStatus"),
            SsmActions::UpdateDocument => write!(f, "ssm:UpdateDocument"),
            SsmActions::UpdateDocumentDefaultVersion => {
                write!(f, "ssm:UpdateDocumentDefaultVersion")
            }
            SsmActions::UpdateDocumentMetadata => write!(f, "ssm:UpdateDocumentMetadata"),
            SsmActions::UpdateInstanceAssociationStatus => {
                write!(f, "ssm:UpdateInstanceAssociationStatus")
            }
            SsmActions::UpdateInstanceInformation => write!(f, "ssm:UpdateInstanceInformation"),
            SsmActions::UpdateMaintenanceWindow => write!(f, "ssm:UpdateMaintenanceWindow"),
            SsmActions::UpdateMaintenanceWindowTarget => {
                write!(f, "ssm:UpdateMaintenanceWindowTarget")
            }
            SsmActions::UpdateMaintenanceWindowTask => write!(f, "ssm:UpdateMaintenanceWindowTask"),
            SsmActions::UpdateManagedInstanceRole => write!(f, "ssm:UpdateManagedInstanceRole"),
            SsmActions::UpdateOpsItem => write!(f, "ssm:UpdateOpsItem"),
            SsmActions::UpdateOpsMetadata => write!(f, "ssm:UpdateOpsMetadata"),
            SsmActions::UpdatePatchBaseline => write!(f, "ssm:UpdatePatchBaseline"),
            SsmActions::UpdateResourceDataSync => write!(f, "ssm:UpdateResourceDataSync"),
            SsmActions::UpdateServiceSetting => write!(f, "ssm:UpdateServiceSetting"),
        }
    }
}
