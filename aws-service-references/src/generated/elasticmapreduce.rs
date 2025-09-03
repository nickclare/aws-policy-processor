// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum ElasticmapreduceActions {
    AccessAllEventLogs,
    AddInstanceFleet,
    AddInstanceGroups,
    AddJobFlowSteps,
    AddTags,
    AttachEditor,
    CancelSteps,
    CreateEditor,
    CreatePersistentAppUi,
    CreateRepository,
    CreateSecurityConfiguration,
    CreateStudio,
    CreateStudioPresignedUrl,
    CreateStudioSessionMapping,
    DeleteEditor,
    DeleteRepository,
    DeleteSecurityConfiguration,
    DeleteStudio,
    DeleteStudioSessionMapping,
    DeleteWorkspaceAccess,
    DescribeCluster,
    DescribeEditor,
    DescribeJobFlows,
    DescribeNotebookExecution,
    DescribePersistentAppUi,
    DescribeReleaseLabel,
    DescribeRepository,
    DescribeSecurityConfiguration,
    DescribeStep,
    DescribeStudio,
    DetachEditor,
    GetAutoTerminationPolicy,
    GetBlockPublicAccessConfiguration,
    GetClusterSessionCredentials,
    GetManagedScalingPolicy,
    GetOnClusterAppUiPresignedUrl,
    GetPersistentAppUiPresignedUrl,
    GetStudioSessionMapping,
    LinkRepository,
    ListBootstrapActions,
    ListClusters,
    ListEditors,
    ListInstanceFleets,
    ListInstanceGroups,
    ListInstances,
    ListNotebookExecutions,
    ListReleaseLabels,
    ListRepositories,
    ListSecurityConfigurations,
    ListSteps,
    ListStudioSessionMappings,
    ListStudios,
    ListSupportedInstanceTypes,
    ListWorkspaceAccessIdentities,
    ModifyCluster,
    ModifyInstanceFleet,
    ModifyInstanceGroups,
    OpenEditorInConsole,
    PutAutoScalingPolicy,
    PutAutoTerminationPolicy,
    PutBlockPublicAccessConfiguration,
    PutManagedScalingPolicy,
    PutWorkspaceAccess,
    RemoveAutoScalingPolicy,
    RemoveAutoTerminationPolicy,
    RemoveManagedScalingPolicy,
    RemoveTags,
    RunJobFlow,
    SetKeepJobFlowAliveWhenNoSteps,
    SetTerminationProtection,
    SetUnhealthyNodeReplacement,
    SetVisibleToAllUsers,
    StartEditor,
    StartNotebookExecution,
    StopEditor,
    StopNotebookExecution,
    TerminateJobFlows,
    UnlinkRepository,
    UpdateEditor,
    UpdateRepository,
    UpdateStudio,
    UpdateStudioSessionMapping,
    ViewEventsFromAllClustersInConsole,
}
impl std::fmt::Display for ElasticmapreduceActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ElasticmapreduceActions::AccessAllEventLogs => {
                write!(f, "elasticmapreduce:AccessAllEventLogs")
            }
            ElasticmapreduceActions::AddInstanceFleet => {
                write!(f, "elasticmapreduce:AddInstanceFleet")
            }
            ElasticmapreduceActions::AddInstanceGroups => {
                write!(f, "elasticmapreduce:AddInstanceGroups")
            }
            ElasticmapreduceActions::AddJobFlowSteps => {
                write!(f, "elasticmapreduce:AddJobFlowSteps")
            }
            ElasticmapreduceActions::AddTags => write!(f, "elasticmapreduce:AddTags"),
            ElasticmapreduceActions::AttachEditor => write!(f, "elasticmapreduce:AttachEditor"),
            ElasticmapreduceActions::CancelSteps => write!(f, "elasticmapreduce:CancelSteps"),
            ElasticmapreduceActions::CreateEditor => write!(f, "elasticmapreduce:CreateEditor"),
            ElasticmapreduceActions::CreatePersistentAppUi => {
                write!(f, "elasticmapreduce:CreatePersistentAppUI")
            }
            ElasticmapreduceActions::CreateRepository => {
                write!(f, "elasticmapreduce:CreateRepository")
            }
            ElasticmapreduceActions::CreateSecurityConfiguration => {
                write!(f, "elasticmapreduce:CreateSecurityConfiguration")
            }
            ElasticmapreduceActions::CreateStudio => write!(f, "elasticmapreduce:CreateStudio"),
            ElasticmapreduceActions::CreateStudioPresignedUrl => {
                write!(f, "elasticmapreduce:CreateStudioPresignedUrl")
            }
            ElasticmapreduceActions::CreateStudioSessionMapping => {
                write!(f, "elasticmapreduce:CreateStudioSessionMapping")
            }
            ElasticmapreduceActions::DeleteEditor => write!(f, "elasticmapreduce:DeleteEditor"),
            ElasticmapreduceActions::DeleteRepository => {
                write!(f, "elasticmapreduce:DeleteRepository")
            }
            ElasticmapreduceActions::DeleteSecurityConfiguration => {
                write!(f, "elasticmapreduce:DeleteSecurityConfiguration")
            }
            ElasticmapreduceActions::DeleteStudio => write!(f, "elasticmapreduce:DeleteStudio"),
            ElasticmapreduceActions::DeleteStudioSessionMapping => {
                write!(f, "elasticmapreduce:DeleteStudioSessionMapping")
            }
            ElasticmapreduceActions::DeleteWorkspaceAccess => {
                write!(f, "elasticmapreduce:DeleteWorkspaceAccess")
            }
            ElasticmapreduceActions::DescribeCluster => {
                write!(f, "elasticmapreduce:DescribeCluster")
            }
            ElasticmapreduceActions::DescribeEditor => write!(f, "elasticmapreduce:DescribeEditor"),
            ElasticmapreduceActions::DescribeJobFlows => {
                write!(f, "elasticmapreduce:DescribeJobFlows")
            }
            ElasticmapreduceActions::DescribeNotebookExecution => {
                write!(f, "elasticmapreduce:DescribeNotebookExecution")
            }
            ElasticmapreduceActions::DescribePersistentAppUi => {
                write!(f, "elasticmapreduce:DescribePersistentAppUI")
            }
            ElasticmapreduceActions::DescribeReleaseLabel => {
                write!(f, "elasticmapreduce:DescribeReleaseLabel")
            }
            ElasticmapreduceActions::DescribeRepository => {
                write!(f, "elasticmapreduce:DescribeRepository")
            }
            ElasticmapreduceActions::DescribeSecurityConfiguration => {
                write!(f, "elasticmapreduce:DescribeSecurityConfiguration")
            }
            ElasticmapreduceActions::DescribeStep => write!(f, "elasticmapreduce:DescribeStep"),
            ElasticmapreduceActions::DescribeStudio => write!(f, "elasticmapreduce:DescribeStudio"),
            ElasticmapreduceActions::DetachEditor => write!(f, "elasticmapreduce:DetachEditor"),
            ElasticmapreduceActions::GetAutoTerminationPolicy => {
                write!(f, "elasticmapreduce:GetAutoTerminationPolicy")
            }
            ElasticmapreduceActions::GetBlockPublicAccessConfiguration => {
                write!(f, "elasticmapreduce:GetBlockPublicAccessConfiguration")
            }
            ElasticmapreduceActions::GetClusterSessionCredentials => {
                write!(f, "elasticmapreduce:GetClusterSessionCredentials")
            }
            ElasticmapreduceActions::GetManagedScalingPolicy => {
                write!(f, "elasticmapreduce:GetManagedScalingPolicy")
            }
            ElasticmapreduceActions::GetOnClusterAppUiPresignedUrl => {
                write!(f, "elasticmapreduce:GetOnClusterAppUIPresignedURL")
            }
            ElasticmapreduceActions::GetPersistentAppUiPresignedUrl => {
                write!(f, "elasticmapreduce:GetPersistentAppUIPresignedURL")
            }
            ElasticmapreduceActions::GetStudioSessionMapping => {
                write!(f, "elasticmapreduce:GetStudioSessionMapping")
            }
            ElasticmapreduceActions::LinkRepository => write!(f, "elasticmapreduce:LinkRepository"),
            ElasticmapreduceActions::ListBootstrapActions => {
                write!(f, "elasticmapreduce:ListBootstrapActions")
            }
            ElasticmapreduceActions::ListClusters => write!(f, "elasticmapreduce:ListClusters"),
            ElasticmapreduceActions::ListEditors => write!(f, "elasticmapreduce:ListEditors"),
            ElasticmapreduceActions::ListInstanceFleets => {
                write!(f, "elasticmapreduce:ListInstanceFleets")
            }
            ElasticmapreduceActions::ListInstanceGroups => {
                write!(f, "elasticmapreduce:ListInstanceGroups")
            }
            ElasticmapreduceActions::ListInstances => write!(f, "elasticmapreduce:ListInstances"),
            ElasticmapreduceActions::ListNotebookExecutions => {
                write!(f, "elasticmapreduce:ListNotebookExecutions")
            }
            ElasticmapreduceActions::ListReleaseLabels => {
                write!(f, "elasticmapreduce:ListReleaseLabels")
            }
            ElasticmapreduceActions::ListRepositories => {
                write!(f, "elasticmapreduce:ListRepositories")
            }
            ElasticmapreduceActions::ListSecurityConfigurations => {
                write!(f, "elasticmapreduce:ListSecurityConfigurations")
            }
            ElasticmapreduceActions::ListSteps => write!(f, "elasticmapreduce:ListSteps"),
            ElasticmapreduceActions::ListStudioSessionMappings => {
                write!(f, "elasticmapreduce:ListStudioSessionMappings")
            }
            ElasticmapreduceActions::ListStudios => write!(f, "elasticmapreduce:ListStudios"),
            ElasticmapreduceActions::ListSupportedInstanceTypes => {
                write!(f, "elasticmapreduce:ListSupportedInstanceTypes")
            }
            ElasticmapreduceActions::ListWorkspaceAccessIdentities => {
                write!(f, "elasticmapreduce:ListWorkspaceAccessIdentities")
            }
            ElasticmapreduceActions::ModifyCluster => write!(f, "elasticmapreduce:ModifyCluster"),
            ElasticmapreduceActions::ModifyInstanceFleet => {
                write!(f, "elasticmapreduce:ModifyInstanceFleet")
            }
            ElasticmapreduceActions::ModifyInstanceGroups => {
                write!(f, "elasticmapreduce:ModifyInstanceGroups")
            }
            ElasticmapreduceActions::OpenEditorInConsole => {
                write!(f, "elasticmapreduce:OpenEditorInConsole")
            }
            ElasticmapreduceActions::PutAutoScalingPolicy => {
                write!(f, "elasticmapreduce:PutAutoScalingPolicy")
            }
            ElasticmapreduceActions::PutAutoTerminationPolicy => {
                write!(f, "elasticmapreduce:PutAutoTerminationPolicy")
            }
            ElasticmapreduceActions::PutBlockPublicAccessConfiguration => {
                write!(f, "elasticmapreduce:PutBlockPublicAccessConfiguration")
            }
            ElasticmapreduceActions::PutManagedScalingPolicy => {
                write!(f, "elasticmapreduce:PutManagedScalingPolicy")
            }
            ElasticmapreduceActions::PutWorkspaceAccess => {
                write!(f, "elasticmapreduce:PutWorkspaceAccess")
            }
            ElasticmapreduceActions::RemoveAutoScalingPolicy => {
                write!(f, "elasticmapreduce:RemoveAutoScalingPolicy")
            }
            ElasticmapreduceActions::RemoveAutoTerminationPolicy => {
                write!(f, "elasticmapreduce:RemoveAutoTerminationPolicy")
            }
            ElasticmapreduceActions::RemoveManagedScalingPolicy => {
                write!(f, "elasticmapreduce:RemoveManagedScalingPolicy")
            }
            ElasticmapreduceActions::RemoveTags => write!(f, "elasticmapreduce:RemoveTags"),
            ElasticmapreduceActions::RunJobFlow => write!(f, "elasticmapreduce:RunJobFlow"),
            ElasticmapreduceActions::SetKeepJobFlowAliveWhenNoSteps => {
                write!(f, "elasticmapreduce:SetKeepJobFlowAliveWhenNoSteps")
            }
            ElasticmapreduceActions::SetTerminationProtection => {
                write!(f, "elasticmapreduce:SetTerminationProtection")
            }
            ElasticmapreduceActions::SetUnhealthyNodeReplacement => {
                write!(f, "elasticmapreduce:SetUnhealthyNodeReplacement")
            }
            ElasticmapreduceActions::SetVisibleToAllUsers => {
                write!(f, "elasticmapreduce:SetVisibleToAllUsers")
            }
            ElasticmapreduceActions::StartEditor => write!(f, "elasticmapreduce:StartEditor"),
            ElasticmapreduceActions::StartNotebookExecution => {
                write!(f, "elasticmapreduce:StartNotebookExecution")
            }
            ElasticmapreduceActions::StopEditor => write!(f, "elasticmapreduce:StopEditor"),
            ElasticmapreduceActions::StopNotebookExecution => {
                write!(f, "elasticmapreduce:StopNotebookExecution")
            }
            ElasticmapreduceActions::TerminateJobFlows => {
                write!(f, "elasticmapreduce:TerminateJobFlows")
            }
            ElasticmapreduceActions::UnlinkRepository => {
                write!(f, "elasticmapreduce:UnlinkRepository")
            }
            ElasticmapreduceActions::UpdateEditor => write!(f, "elasticmapreduce:UpdateEditor"),
            ElasticmapreduceActions::UpdateRepository => {
                write!(f, "elasticmapreduce:UpdateRepository")
            }
            ElasticmapreduceActions::UpdateStudio => write!(f, "elasticmapreduce:UpdateStudio"),
            ElasticmapreduceActions::UpdateStudioSessionMapping => {
                write!(f, "elasticmapreduce:UpdateStudioSessionMapping")
            }
            ElasticmapreduceActions::ViewEventsFromAllClustersInConsole => {
                write!(f, "elasticmapreduce:ViewEventsFromAllClustersInConsole")
            }
        }
    }
}
