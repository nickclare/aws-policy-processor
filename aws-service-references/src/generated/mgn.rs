// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum MgnActions {
    ArchiveApplication,
    ArchiveWave,
    AssociateApplications,
    AssociateSourceServers,
    BatchCreateVolumeSnapshotGroupForMgn,
    BatchDeleteSnapshotRequestForMgn,
    ChangeServerLifeCycleState,
    CreateApplication,
    CreateConnector,
    CreateLaunchConfigurationTemplate,
    CreateNetworkMigrationDefinition,
    CreateReplicationConfigurationTemplate,
    CreateVcenterClientForMgn,
    CreateWave,
    DeleteApplication,
    DeleteConnector,
    DeleteJob,
    DeleteLaunchConfigurationTemplate,
    DeleteNetworkMigrationDefinition,
    DeleteReplicationConfigurationTemplate,
    DeleteSourceServer,
    DeleteVcenterClient,
    DeleteWave,
    DescribeJobLogItems,
    DescribeJobs,
    DescribeLaunchConfigurationTemplates,
    DescribeReplicationConfigurationTemplates,
    DescribeReplicationServerAssociationsForMgn,
    DescribeSnapshotRequestsForMgn,
    DescribeSourceServers,
    DescribeVcenterClients,
    DisassociateApplications,
    DisassociateSourceServers,
    DisconnectFromService,
    FinalizeCutover,
    GetAgentCommandForMgn,
    GetAgentConfirmedResumeInfoForMgn,
    GetAgentInstallationAssetsForMgn,
    GetAgentReplicationInfoForMgn,
    GetAgentRuntimeConfigurationForMgn,
    GetAgentSnapshotCreditsForMgn,
    GetChannelCommandsForMgn,
    GetLaunchConfiguration,
    GetNetworkMigrationDefinition,
    GetNetworkMigrationMapperSegmentConstruct,
    GetReplicationConfiguration,
    GetVcenterClientCommandsForMgn,
    InitializeService,
    IssueClientCertificateForMgn,
    ListApplications,
    ListConnectors,
    ListExportErrors,
    ListExports,
    ListImportErrors,
    ListImports,
    ListManagedAccounts,
    ListNetworkMigrationAnalyses,
    ListNetworkMigrationAnalysisResults,
    ListNetworkMigrationCodeGenerationSegments,
    ListNetworkMigrationCodeGenerations,
    ListNetworkMigrationDefinitions,
    ListNetworkMigrationDeployedStacks,
    ListNetworkMigrationDeployedStacksDeletions,
    ListNetworkMigrationDeployments,
    ListNetworkMigrationExecutions,
    ListNetworkMigrationMapperSegmentConstructs,
    ListNetworkMigrationMapperSegments,
    ListNetworkMigrationMappings,
    ListSourceServerActions,
    ListTagsForResource,
    ListTemplateActions,
    ListWaves,
    MarkAsArchived,
    NotifyAgentAuthenticationForMgn,
    NotifyAgentConnectedForMgn,
    NotifyAgentDisconnectedForMgn,
    NotifyAgentReplicationProgressForMgn,
    NotifyVcenterClientStartedForMgn,
    PauseReplication,
    PutSourceServerAction,
    PutTemplateAction,
    RegisterAgentForMgn,
    RemoveSourceServerAction,
    RemoveTemplateAction,
    ResumeReplication,
    RetryDataReplication,
    SendAgentLogsForMgn,
    SendAgentMetricsForMgn,
    SendChannelCommandResultForMgn,
    SendClientLogsForMgn,
    SendClientMetricsForMgn,
    SendVcenterClientCommandResultForMgn,
    SendVcenterClientLogsForMgn,
    SendVcenterClientMetricsForMgn,
    StartCutover,
    StartExport,
    StartImport,
    StartNetworkMigrationAnalysis,
    StartNetworkMigrationCodeGeneration,
    StartNetworkMigrationDeployedStacksDeletion,
    StartNetworkMigrationDeployment,
    StartNetworkMigrationMapping,
    StartReplication,
    StartTest,
    StopReplication,
    TagResource,
    TerminateTargetInstances,
    UnarchiveApplication,
    UnarchiveWave,
    UntagResource,
    UpdateAgentBacklogForMgn,
    UpdateAgentConversionInfoForMgn,
    UpdateAgentReplicationInfoForMgn,
    UpdateAgentReplicationProcessStateForMgn,
    UpdateAgentSourcePropertiesForMgn,
    UpdateApplication,
    UpdateConnector,
    UpdateLaunchConfiguration,
    UpdateLaunchConfigurationTemplate,
    UpdateNetworkMigrationDefinition,
    UpdateNetworkMigrationMapperSegment,
    UpdateNetworkMigrationMapperSegmentConstruct,
    UpdateReplicationConfiguration,
    UpdateReplicationConfigurationTemplate,
    UpdateSourceServer,
    UpdateSourceServerReplicationType,
    UpdateWave,
    VerifyClientRoleForMgn,
}
impl std::fmt::Display for MgnActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MgnActions::ArchiveApplication => write!(f, "mgn:ArchiveApplication"),
            MgnActions::ArchiveWave => write!(f, "mgn:ArchiveWave"),
            MgnActions::AssociateApplications => write!(f, "mgn:AssociateApplications"),
            MgnActions::AssociateSourceServers => write!(f, "mgn:AssociateSourceServers"),
            MgnActions::BatchCreateVolumeSnapshotGroupForMgn => {
                write!(f, "mgn:BatchCreateVolumeSnapshotGroupForMgn")
            }
            MgnActions::BatchDeleteSnapshotRequestForMgn => {
                write!(f, "mgn:BatchDeleteSnapshotRequestForMgn")
            }
            MgnActions::ChangeServerLifeCycleState => write!(f, "mgn:ChangeServerLifeCycleState"),
            MgnActions::CreateApplication => write!(f, "mgn:CreateApplication"),
            MgnActions::CreateConnector => write!(f, "mgn:CreateConnector"),
            MgnActions::CreateLaunchConfigurationTemplate => {
                write!(f, "mgn:CreateLaunchConfigurationTemplate")
            }
            MgnActions::CreateNetworkMigrationDefinition => {
                write!(f, "mgn:CreateNetworkMigrationDefinition")
            }
            MgnActions::CreateReplicationConfigurationTemplate => {
                write!(f, "mgn:CreateReplicationConfigurationTemplate")
            }
            MgnActions::CreateVcenterClientForMgn => write!(f, "mgn:CreateVcenterClientForMgn"),
            MgnActions::CreateWave => write!(f, "mgn:CreateWave"),
            MgnActions::DeleteApplication => write!(f, "mgn:DeleteApplication"),
            MgnActions::DeleteConnector => write!(f, "mgn:DeleteConnector"),
            MgnActions::DeleteJob => write!(f, "mgn:DeleteJob"),
            MgnActions::DeleteLaunchConfigurationTemplate => {
                write!(f, "mgn:DeleteLaunchConfigurationTemplate")
            }
            MgnActions::DeleteNetworkMigrationDefinition => {
                write!(f, "mgn:DeleteNetworkMigrationDefinition")
            }
            MgnActions::DeleteReplicationConfigurationTemplate => {
                write!(f, "mgn:DeleteReplicationConfigurationTemplate")
            }
            MgnActions::DeleteSourceServer => write!(f, "mgn:DeleteSourceServer"),
            MgnActions::DeleteVcenterClient => write!(f, "mgn:DeleteVcenterClient"),
            MgnActions::DeleteWave => write!(f, "mgn:DeleteWave"),
            MgnActions::DescribeJobLogItems => write!(f, "mgn:DescribeJobLogItems"),
            MgnActions::DescribeJobs => write!(f, "mgn:DescribeJobs"),
            MgnActions::DescribeLaunchConfigurationTemplates => {
                write!(f, "mgn:DescribeLaunchConfigurationTemplates")
            }
            MgnActions::DescribeReplicationConfigurationTemplates => {
                write!(f, "mgn:DescribeReplicationConfigurationTemplates")
            }
            MgnActions::DescribeReplicationServerAssociationsForMgn => {
                write!(f, "mgn:DescribeReplicationServerAssociationsForMgn")
            }
            MgnActions::DescribeSnapshotRequestsForMgn => {
                write!(f, "mgn:DescribeSnapshotRequestsForMgn")
            }
            MgnActions::DescribeSourceServers => write!(f, "mgn:DescribeSourceServers"),
            MgnActions::DescribeVcenterClients => write!(f, "mgn:DescribeVcenterClients"),
            MgnActions::DisassociateApplications => write!(f, "mgn:DisassociateApplications"),
            MgnActions::DisassociateSourceServers => write!(f, "mgn:DisassociateSourceServers"),
            MgnActions::DisconnectFromService => write!(f, "mgn:DisconnectFromService"),
            MgnActions::FinalizeCutover => write!(f, "mgn:FinalizeCutover"),
            MgnActions::GetAgentCommandForMgn => write!(f, "mgn:GetAgentCommandForMgn"),
            MgnActions::GetAgentConfirmedResumeInfoForMgn => {
                write!(f, "mgn:GetAgentConfirmedResumeInfoForMgn")
            }
            MgnActions::GetAgentInstallationAssetsForMgn => {
                write!(f, "mgn:GetAgentInstallationAssetsForMgn")
            }
            MgnActions::GetAgentReplicationInfoForMgn => {
                write!(f, "mgn:GetAgentReplicationInfoForMgn")
            }
            MgnActions::GetAgentRuntimeConfigurationForMgn => {
                write!(f, "mgn:GetAgentRuntimeConfigurationForMgn")
            }
            MgnActions::GetAgentSnapshotCreditsForMgn => {
                write!(f, "mgn:GetAgentSnapshotCreditsForMgn")
            }
            MgnActions::GetChannelCommandsForMgn => write!(f, "mgn:GetChannelCommandsForMgn"),
            MgnActions::GetLaunchConfiguration => write!(f, "mgn:GetLaunchConfiguration"),
            MgnActions::GetNetworkMigrationDefinition => {
                write!(f, "mgn:GetNetworkMigrationDefinition")
            }
            MgnActions::GetNetworkMigrationMapperSegmentConstruct => {
                write!(f, "mgn:GetNetworkMigrationMapperSegmentConstruct")
            }
            MgnActions::GetReplicationConfiguration => write!(f, "mgn:GetReplicationConfiguration"),
            MgnActions::GetVcenterClientCommandsForMgn => {
                write!(f, "mgn:GetVcenterClientCommandsForMgn")
            }
            MgnActions::InitializeService => write!(f, "mgn:InitializeService"),
            MgnActions::IssueClientCertificateForMgn => {
                write!(f, "mgn:IssueClientCertificateForMgn")
            }
            MgnActions::ListApplications => write!(f, "mgn:ListApplications"),
            MgnActions::ListConnectors => write!(f, "mgn:ListConnectors"),
            MgnActions::ListExportErrors => write!(f, "mgn:ListExportErrors"),
            MgnActions::ListExports => write!(f, "mgn:ListExports"),
            MgnActions::ListImportErrors => write!(f, "mgn:ListImportErrors"),
            MgnActions::ListImports => write!(f, "mgn:ListImports"),
            MgnActions::ListManagedAccounts => write!(f, "mgn:ListManagedAccounts"),
            MgnActions::ListNetworkMigrationAnalyses => {
                write!(f, "mgn:ListNetworkMigrationAnalyses")
            }
            MgnActions::ListNetworkMigrationAnalysisResults => {
                write!(f, "mgn:ListNetworkMigrationAnalysisResults")
            }
            MgnActions::ListNetworkMigrationCodeGenerationSegments => {
                write!(f, "mgn:ListNetworkMigrationCodeGenerationSegments")
            }
            MgnActions::ListNetworkMigrationCodeGenerations => {
                write!(f, "mgn:ListNetworkMigrationCodeGenerations")
            }
            MgnActions::ListNetworkMigrationDefinitions => {
                write!(f, "mgn:ListNetworkMigrationDefinitions")
            }
            MgnActions::ListNetworkMigrationDeployedStacks => {
                write!(f, "mgn:ListNetworkMigrationDeployedStacks")
            }
            MgnActions::ListNetworkMigrationDeployedStacksDeletions => {
                write!(f, "mgn:ListNetworkMigrationDeployedStacksDeletions")
            }
            MgnActions::ListNetworkMigrationDeployments => {
                write!(f, "mgn:ListNetworkMigrationDeployments")
            }
            MgnActions::ListNetworkMigrationExecutions => {
                write!(f, "mgn:ListNetworkMigrationExecutions")
            }
            MgnActions::ListNetworkMigrationMapperSegmentConstructs => {
                write!(f, "mgn:ListNetworkMigrationMapperSegmentConstructs")
            }
            MgnActions::ListNetworkMigrationMapperSegments => {
                write!(f, "mgn:ListNetworkMigrationMapperSegments")
            }
            MgnActions::ListNetworkMigrationMappings => {
                write!(f, "mgn:ListNetworkMigrationMappings")
            }
            MgnActions::ListSourceServerActions => write!(f, "mgn:ListSourceServerActions"),
            MgnActions::ListTagsForResource => write!(f, "mgn:ListTagsForResource"),
            MgnActions::ListTemplateActions => write!(f, "mgn:ListTemplateActions"),
            MgnActions::ListWaves => write!(f, "mgn:ListWaves"),
            MgnActions::MarkAsArchived => write!(f, "mgn:MarkAsArchived"),
            MgnActions::NotifyAgentAuthenticationForMgn => {
                write!(f, "mgn:NotifyAgentAuthenticationForMgn")
            }
            MgnActions::NotifyAgentConnectedForMgn => write!(f, "mgn:NotifyAgentConnectedForMgn"),
            MgnActions::NotifyAgentDisconnectedForMgn => {
                write!(f, "mgn:NotifyAgentDisconnectedForMgn")
            }
            MgnActions::NotifyAgentReplicationProgressForMgn => {
                write!(f, "mgn:NotifyAgentReplicationProgressForMgn")
            }
            MgnActions::NotifyVcenterClientStartedForMgn => {
                write!(f, "mgn:NotifyVcenterClientStartedForMgn")
            }
            MgnActions::PauseReplication => write!(f, "mgn:PauseReplication"),
            MgnActions::PutSourceServerAction => write!(f, "mgn:PutSourceServerAction"),
            MgnActions::PutTemplateAction => write!(f, "mgn:PutTemplateAction"),
            MgnActions::RegisterAgentForMgn => write!(f, "mgn:RegisterAgentForMgn"),
            MgnActions::RemoveSourceServerAction => write!(f, "mgn:RemoveSourceServerAction"),
            MgnActions::RemoveTemplateAction => write!(f, "mgn:RemoveTemplateAction"),
            MgnActions::ResumeReplication => write!(f, "mgn:ResumeReplication"),
            MgnActions::RetryDataReplication => write!(f, "mgn:RetryDataReplication"),
            MgnActions::SendAgentLogsForMgn => write!(f, "mgn:SendAgentLogsForMgn"),
            MgnActions::SendAgentMetricsForMgn => write!(f, "mgn:SendAgentMetricsForMgn"),
            MgnActions::SendChannelCommandResultForMgn => {
                write!(f, "mgn:SendChannelCommandResultForMgn")
            }
            MgnActions::SendClientLogsForMgn => write!(f, "mgn:SendClientLogsForMgn"),
            MgnActions::SendClientMetricsForMgn => write!(f, "mgn:SendClientMetricsForMgn"),
            MgnActions::SendVcenterClientCommandResultForMgn => {
                write!(f, "mgn:SendVcenterClientCommandResultForMgn")
            }
            MgnActions::SendVcenterClientLogsForMgn => write!(f, "mgn:SendVcenterClientLogsForMgn"),
            MgnActions::SendVcenterClientMetricsForMgn => {
                write!(f, "mgn:SendVcenterClientMetricsForMgn")
            }
            MgnActions::StartCutover => write!(f, "mgn:StartCutover"),
            MgnActions::StartExport => write!(f, "mgn:StartExport"),
            MgnActions::StartImport => write!(f, "mgn:StartImport"),
            MgnActions::StartNetworkMigrationAnalysis => {
                write!(f, "mgn:StartNetworkMigrationAnalysis")
            }
            MgnActions::StartNetworkMigrationCodeGeneration => {
                write!(f, "mgn:StartNetworkMigrationCodeGeneration")
            }
            MgnActions::StartNetworkMigrationDeployedStacksDeletion => {
                write!(f, "mgn:StartNetworkMigrationDeployedStacksDeletion")
            }
            MgnActions::StartNetworkMigrationDeployment => {
                write!(f, "mgn:StartNetworkMigrationDeployment")
            }
            MgnActions::StartNetworkMigrationMapping => {
                write!(f, "mgn:StartNetworkMigrationMapping")
            }
            MgnActions::StartReplication => write!(f, "mgn:StartReplication"),
            MgnActions::StartTest => write!(f, "mgn:StartTest"),
            MgnActions::StopReplication => write!(f, "mgn:StopReplication"),
            MgnActions::TagResource => write!(f, "mgn:TagResource"),
            MgnActions::TerminateTargetInstances => write!(f, "mgn:TerminateTargetInstances"),
            MgnActions::UnarchiveApplication => write!(f, "mgn:UnarchiveApplication"),
            MgnActions::UnarchiveWave => write!(f, "mgn:UnarchiveWave"),
            MgnActions::UntagResource => write!(f, "mgn:UntagResource"),
            MgnActions::UpdateAgentBacklogForMgn => write!(f, "mgn:UpdateAgentBacklogForMgn"),
            MgnActions::UpdateAgentConversionInfoForMgn => {
                write!(f, "mgn:UpdateAgentConversionInfoForMgn")
            }
            MgnActions::UpdateAgentReplicationInfoForMgn => {
                write!(f, "mgn:UpdateAgentReplicationInfoForMgn")
            }
            MgnActions::UpdateAgentReplicationProcessStateForMgn => {
                write!(f, "mgn:UpdateAgentReplicationProcessStateForMgn")
            }
            MgnActions::UpdateAgentSourcePropertiesForMgn => {
                write!(f, "mgn:UpdateAgentSourcePropertiesForMgn")
            }
            MgnActions::UpdateApplication => write!(f, "mgn:UpdateApplication"),
            MgnActions::UpdateConnector => write!(f, "mgn:UpdateConnector"),
            MgnActions::UpdateLaunchConfiguration => write!(f, "mgn:UpdateLaunchConfiguration"),
            MgnActions::UpdateLaunchConfigurationTemplate => {
                write!(f, "mgn:UpdateLaunchConfigurationTemplate")
            }
            MgnActions::UpdateNetworkMigrationDefinition => {
                write!(f, "mgn:UpdateNetworkMigrationDefinition")
            }
            MgnActions::UpdateNetworkMigrationMapperSegment => {
                write!(f, "mgn:UpdateNetworkMigrationMapperSegment")
            }
            MgnActions::UpdateNetworkMigrationMapperSegmentConstruct => {
                write!(f, "mgn:UpdateNetworkMigrationMapperSegmentConstruct")
            }
            MgnActions::UpdateReplicationConfiguration => {
                write!(f, "mgn:UpdateReplicationConfiguration")
            }
            MgnActions::UpdateReplicationConfigurationTemplate => {
                write!(f, "mgn:UpdateReplicationConfigurationTemplate")
            }
            MgnActions::UpdateSourceServer => write!(f, "mgn:UpdateSourceServer"),
            MgnActions::UpdateSourceServerReplicationType => {
                write!(f, "mgn:UpdateSourceServerReplicationType")
            }
            MgnActions::UpdateWave => write!(f, "mgn:UpdateWave"),
            MgnActions::VerifyClientRoleForMgn => write!(f, "mgn:VerifyClientRoleForMgn"),
        }
    }
}
