// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum CodedeployActions {
    AddTagsToOnPremisesInstances,
    BatchGetApplicationRevisions,
    BatchGetApplications,
    BatchGetDeploymentGroups,
    BatchGetDeploymentInstances,
    BatchGetDeploymentTargets,
    BatchGetDeployments,
    BatchGetOnPremisesInstances,
    ContinueDeployment,
    CreateApplication,
    CreateCloudFormationDeployment,
    CreateDeployment,
    CreateDeploymentConfig,
    CreateDeploymentGroup,
    DeleteApplication,
    DeleteDeploymentConfig,
    DeleteDeploymentGroup,
    DeleteGitHubAccountToken,
    DeleteResourcesByExternalId,
    DeregisterOnPremisesInstance,
    GetApplication,
    GetApplicationRevision,
    GetDeployment,
    GetDeploymentConfig,
    GetDeploymentGroup,
    GetDeploymentInstance,
    GetDeploymentTarget,
    GetOnPremisesInstance,
    ListApplicationRevisions,
    ListApplications,
    ListDeploymentConfigs,
    ListDeploymentGroups,
    ListDeploymentInstances,
    ListDeploymentTargets,
    ListDeployments,
    ListGitHubAccountTokenNames,
    ListOnPremisesInstances,
    ListTagsForResource,
    PutLifecycleEventHookExecutionStatus,
    RegisterApplicationRevision,
    RegisterOnPremisesInstance,
    RemoveTagsFromOnPremisesInstances,
    SkipWaitTimeForInstanceTermination,
    StopDeployment,
    TagResource,
    UntagResource,
    UpdateApplication,
    UpdateDeploymentGroup,
}
impl std::fmt::Display for CodedeployActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CodedeployActions::AddTagsToOnPremisesInstances => {
                write!(f, "codedeploy:AddTagsToOnPremisesInstances")
            }
            CodedeployActions::BatchGetApplicationRevisions => {
                write!(f, "codedeploy:BatchGetApplicationRevisions")
            }
            CodedeployActions::BatchGetApplications => write!(f, "codedeploy:BatchGetApplications"),
            CodedeployActions::BatchGetDeploymentGroups => {
                write!(f, "codedeploy:BatchGetDeploymentGroups")
            }
            CodedeployActions::BatchGetDeploymentInstances => {
                write!(f, "codedeploy:BatchGetDeploymentInstances")
            }
            CodedeployActions::BatchGetDeploymentTargets => {
                write!(f, "codedeploy:BatchGetDeploymentTargets")
            }
            CodedeployActions::BatchGetDeployments => write!(f, "codedeploy:BatchGetDeployments"),
            CodedeployActions::BatchGetOnPremisesInstances => {
                write!(f, "codedeploy:BatchGetOnPremisesInstances")
            }
            CodedeployActions::ContinueDeployment => write!(f, "codedeploy:ContinueDeployment"),
            CodedeployActions::CreateApplication => write!(f, "codedeploy:CreateApplication"),
            CodedeployActions::CreateCloudFormationDeployment => {
                write!(f, "codedeploy:CreateCloudFormationDeployment")
            }
            CodedeployActions::CreateDeployment => write!(f, "codedeploy:CreateDeployment"),
            CodedeployActions::CreateDeploymentConfig => {
                write!(f, "codedeploy:CreateDeploymentConfig")
            }
            CodedeployActions::CreateDeploymentGroup => {
                write!(f, "codedeploy:CreateDeploymentGroup")
            }
            CodedeployActions::DeleteApplication => write!(f, "codedeploy:DeleteApplication"),
            CodedeployActions::DeleteDeploymentConfig => {
                write!(f, "codedeploy:DeleteDeploymentConfig")
            }
            CodedeployActions::DeleteDeploymentGroup => {
                write!(f, "codedeploy:DeleteDeploymentGroup")
            }
            CodedeployActions::DeleteGitHubAccountToken => {
                write!(f, "codedeploy:DeleteGitHubAccountToken")
            }
            CodedeployActions::DeleteResourcesByExternalId => {
                write!(f, "codedeploy:DeleteResourcesByExternalId")
            }
            CodedeployActions::DeregisterOnPremisesInstance => {
                write!(f, "codedeploy:DeregisterOnPremisesInstance")
            }
            CodedeployActions::GetApplication => write!(f, "codedeploy:GetApplication"),
            CodedeployActions::GetApplicationRevision => {
                write!(f, "codedeploy:GetApplicationRevision")
            }
            CodedeployActions::GetDeployment => write!(f, "codedeploy:GetDeployment"),
            CodedeployActions::GetDeploymentConfig => write!(f, "codedeploy:GetDeploymentConfig"),
            CodedeployActions::GetDeploymentGroup => write!(f, "codedeploy:GetDeploymentGroup"),
            CodedeployActions::GetDeploymentInstance => {
                write!(f, "codedeploy:GetDeploymentInstance")
            }
            CodedeployActions::GetDeploymentTarget => write!(f, "codedeploy:GetDeploymentTarget"),
            CodedeployActions::GetOnPremisesInstance => {
                write!(f, "codedeploy:GetOnPremisesInstance")
            }
            CodedeployActions::ListApplicationRevisions => {
                write!(f, "codedeploy:ListApplicationRevisions")
            }
            CodedeployActions::ListApplications => write!(f, "codedeploy:ListApplications"),
            CodedeployActions::ListDeploymentConfigs => {
                write!(f, "codedeploy:ListDeploymentConfigs")
            }
            CodedeployActions::ListDeploymentGroups => write!(f, "codedeploy:ListDeploymentGroups"),
            CodedeployActions::ListDeploymentInstances => {
                write!(f, "codedeploy:ListDeploymentInstances")
            }
            CodedeployActions::ListDeploymentTargets => {
                write!(f, "codedeploy:ListDeploymentTargets")
            }
            CodedeployActions::ListDeployments => write!(f, "codedeploy:ListDeployments"),
            CodedeployActions::ListGitHubAccountTokenNames => {
                write!(f, "codedeploy:ListGitHubAccountTokenNames")
            }
            CodedeployActions::ListOnPremisesInstances => {
                write!(f, "codedeploy:ListOnPremisesInstances")
            }
            CodedeployActions::ListTagsForResource => write!(f, "codedeploy:ListTagsForResource"),
            CodedeployActions::PutLifecycleEventHookExecutionStatus => {
                write!(f, "codedeploy:PutLifecycleEventHookExecutionStatus")
            }
            CodedeployActions::RegisterApplicationRevision => {
                write!(f, "codedeploy:RegisterApplicationRevision")
            }
            CodedeployActions::RegisterOnPremisesInstance => {
                write!(f, "codedeploy:RegisterOnPremisesInstance")
            }
            CodedeployActions::RemoveTagsFromOnPremisesInstances => {
                write!(f, "codedeploy:RemoveTagsFromOnPremisesInstances")
            }
            CodedeployActions::SkipWaitTimeForInstanceTermination => {
                write!(f, "codedeploy:SkipWaitTimeForInstanceTermination")
            }
            CodedeployActions::StopDeployment => write!(f, "codedeploy:StopDeployment"),
            CodedeployActions::TagResource => write!(f, "codedeploy:TagResource"),
            CodedeployActions::UntagResource => write!(f, "codedeploy:UntagResource"),
            CodedeployActions::UpdateApplication => write!(f, "codedeploy:UpdateApplication"),
            CodedeployActions::UpdateDeploymentGroup => {
                write!(f, "codedeploy:UpdateDeploymentGroup")
            }
        }
    }
}
