// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum DrsActions {
    AssociateFailbackClientToRecoveryInstanceForDrs,
    AssociateSourceNetworkStack,
    BatchCreateVolumeSnapshotGroupForDrs,
    BatchDeleteSnapshotRequestForDrs,
    CreateConvertedSnapshotForDrs,
    CreateExtendedSourceServer,
    CreateLaunchConfigurationTemplate,
    CreateRecoveryInstanceForDrs,
    CreateReplicationConfigurationTemplate,
    CreateSourceNetwork,
    CreateSourceServerForDrs,
    DeleteJob,
    DeleteLaunchAction,
    DeleteLaunchConfigurationTemplate,
    DeleteRecoveryInstance,
    DeleteReplicationConfigurationTemplate,
    DeleteSourceNetwork,
    DeleteSourceServer,
    DescribeJobLogItems,
    DescribeJobs,
    DescribeLaunchConfigurationTemplates,
    DescribeRecoveryInstances,
    DescribeRecoverySnapshots,
    DescribeReplicationConfigurationTemplates,
    DescribeReplicationServerAssociationsForDrs,
    DescribeSnapshotRequestsForDrs,
    DescribeSourceNetworks,
    DescribeSourceServers,
    DisconnectRecoveryInstance,
    DisconnectSourceServer,
    ExportSourceNetworkCfnTemplate,
    GetAgentCommandForDrs,
    GetAgentConfirmedResumeInfoForDrs,
    GetAgentInstallationAssetsForDrs,
    GetAgentReplicationInfoForDrs,
    GetAgentRuntimeConfigurationForDrs,
    GetAgentSnapshotCreditsForDrs,
    GetChannelCommandsForDrs,
    GetFailbackCommandForDrs,
    GetFailbackLaunchRequestedForDrs,
    GetFailbackReplicationConfiguration,
    GetLaunchConfiguration,
    GetReplicationConfiguration,
    GetSuggestedFailbackClientDeviceMappingForDrs,
    InitializeService,
    IssueAgentCertificateForDrs,
    ListExtensibleSourceServers,
    ListLaunchActions,
    ListStagingAccounts,
    ListTagsForResource,
    NotifyAgentAuthenticationForDrs,
    NotifyAgentConnectedForDrs,
    NotifyAgentDisconnectedForDrs,
    NotifyAgentReplicationProgressForDrs,
    NotifyConsistencyAttainedForDrs,
    NotifyReplicationServerAuthenticationForDrs,
    NotifyVolumeEventForDrs,
    PutLaunchAction,
    RetryDataReplication,
    ReverseReplication,
    SendAgentLogsForDrs,
    SendAgentMetricsForDrs,
    SendChannelCommandResultForDrs,
    SendClientLogsForDrs,
    SendClientMetricsForDrs,
    SendVolumeStatsForDrs,
    StartFailbackLaunch,
    StartRecovery,
    StartReplication,
    StartSourceNetworkRecovery,
    StartSourceNetworkReplication,
    StopFailback,
    StopReplication,
    StopSourceNetworkReplication,
    TagResource,
    TerminateRecoveryInstances,
    UntagResource,
    UpdateAgentBacklogForDrs,
    UpdateAgentConversionInfoForDrs,
    UpdateAgentReplicationInfoForDrs,
    UpdateAgentReplicationProcessStateForDrs,
    UpdateAgentSourcePropertiesForDrs,
    UpdateFailbackClientDeviceMappingForDrs,
    UpdateFailbackClientLastSeenForDrs,
    UpdateFailbackReplicationConfiguration,
    UpdateLaunchConfiguration,
    UpdateLaunchConfigurationTemplate,
    UpdateReplicationCertificateForDrs,
    UpdateReplicationConfiguration,
    UpdateReplicationConfigurationTemplate,
}
impl std::fmt::Display for DrsActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DrsActions::AssociateFailbackClientToRecoveryInstanceForDrs => {
                write!(f, "drs:AssociateFailbackClientToRecoveryInstanceForDrs")
            }
            DrsActions::AssociateSourceNetworkStack => write!(f, "drs:AssociateSourceNetworkStack"),
            DrsActions::BatchCreateVolumeSnapshotGroupForDrs => {
                write!(f, "drs:BatchCreateVolumeSnapshotGroupForDrs")
            }
            DrsActions::BatchDeleteSnapshotRequestForDrs => {
                write!(f, "drs:BatchDeleteSnapshotRequestForDrs")
            }
            DrsActions::CreateConvertedSnapshotForDrs => {
                write!(f, "drs:CreateConvertedSnapshotForDrs")
            }
            DrsActions::CreateExtendedSourceServer => write!(f, "drs:CreateExtendedSourceServer"),
            DrsActions::CreateLaunchConfigurationTemplate => {
                write!(f, "drs:CreateLaunchConfigurationTemplate")
            }
            DrsActions::CreateRecoveryInstanceForDrs => {
                write!(f, "drs:CreateRecoveryInstanceForDrs")
            }
            DrsActions::CreateReplicationConfigurationTemplate => {
                write!(f, "drs:CreateReplicationConfigurationTemplate")
            }
            DrsActions::CreateSourceNetwork => write!(f, "drs:CreateSourceNetwork"),
            DrsActions::CreateSourceServerForDrs => write!(f, "drs:CreateSourceServerForDrs"),
            DrsActions::DeleteJob => write!(f, "drs:DeleteJob"),
            DrsActions::DeleteLaunchAction => write!(f, "drs:DeleteLaunchAction"),
            DrsActions::DeleteLaunchConfigurationTemplate => {
                write!(f, "drs:DeleteLaunchConfigurationTemplate")
            }
            DrsActions::DeleteRecoveryInstance => write!(f, "drs:DeleteRecoveryInstance"),
            DrsActions::DeleteReplicationConfigurationTemplate => {
                write!(f, "drs:DeleteReplicationConfigurationTemplate")
            }
            DrsActions::DeleteSourceNetwork => write!(f, "drs:DeleteSourceNetwork"),
            DrsActions::DeleteSourceServer => write!(f, "drs:DeleteSourceServer"),
            DrsActions::DescribeJobLogItems => write!(f, "drs:DescribeJobLogItems"),
            DrsActions::DescribeJobs => write!(f, "drs:DescribeJobs"),
            DrsActions::DescribeLaunchConfigurationTemplates => {
                write!(f, "drs:DescribeLaunchConfigurationTemplates")
            }
            DrsActions::DescribeRecoveryInstances => write!(f, "drs:DescribeRecoveryInstances"),
            DrsActions::DescribeRecoverySnapshots => write!(f, "drs:DescribeRecoverySnapshots"),
            DrsActions::DescribeReplicationConfigurationTemplates => {
                write!(f, "drs:DescribeReplicationConfigurationTemplates")
            }
            DrsActions::DescribeReplicationServerAssociationsForDrs => {
                write!(f, "drs:DescribeReplicationServerAssociationsForDrs")
            }
            DrsActions::DescribeSnapshotRequestsForDrs => {
                write!(f, "drs:DescribeSnapshotRequestsForDrs")
            }
            DrsActions::DescribeSourceNetworks => write!(f, "drs:DescribeSourceNetworks"),
            DrsActions::DescribeSourceServers => write!(f, "drs:DescribeSourceServers"),
            DrsActions::DisconnectRecoveryInstance => write!(f, "drs:DisconnectRecoveryInstance"),
            DrsActions::DisconnectSourceServer => write!(f, "drs:DisconnectSourceServer"),
            DrsActions::ExportSourceNetworkCfnTemplate => {
                write!(f, "drs:ExportSourceNetworkCfnTemplate")
            }
            DrsActions::GetAgentCommandForDrs => write!(f, "drs:GetAgentCommandForDrs"),
            DrsActions::GetAgentConfirmedResumeInfoForDrs => {
                write!(f, "drs:GetAgentConfirmedResumeInfoForDrs")
            }
            DrsActions::GetAgentInstallationAssetsForDrs => {
                write!(f, "drs:GetAgentInstallationAssetsForDrs")
            }
            DrsActions::GetAgentReplicationInfoForDrs => {
                write!(f, "drs:GetAgentReplicationInfoForDrs")
            }
            DrsActions::GetAgentRuntimeConfigurationForDrs => {
                write!(f, "drs:GetAgentRuntimeConfigurationForDrs")
            }
            DrsActions::GetAgentSnapshotCreditsForDrs => {
                write!(f, "drs:GetAgentSnapshotCreditsForDrs")
            }
            DrsActions::GetChannelCommandsForDrs => write!(f, "drs:GetChannelCommandsForDrs"),
            DrsActions::GetFailbackCommandForDrs => write!(f, "drs:GetFailbackCommandForDrs"),
            DrsActions::GetFailbackLaunchRequestedForDrs => {
                write!(f, "drs:GetFailbackLaunchRequestedForDrs")
            }
            DrsActions::GetFailbackReplicationConfiguration => {
                write!(f, "drs:GetFailbackReplicationConfiguration")
            }
            DrsActions::GetLaunchConfiguration => write!(f, "drs:GetLaunchConfiguration"),
            DrsActions::GetReplicationConfiguration => write!(f, "drs:GetReplicationConfiguration"),
            DrsActions::GetSuggestedFailbackClientDeviceMappingForDrs => {
                write!(f, "drs:GetSuggestedFailbackClientDeviceMappingForDrs")
            }
            DrsActions::InitializeService => write!(f, "drs:InitializeService"),
            DrsActions::IssueAgentCertificateForDrs => write!(f, "drs:IssueAgentCertificateForDrs"),
            DrsActions::ListExtensibleSourceServers => write!(f, "drs:ListExtensibleSourceServers"),
            DrsActions::ListLaunchActions => write!(f, "drs:ListLaunchActions"),
            DrsActions::ListStagingAccounts => write!(f, "drs:ListStagingAccounts"),
            DrsActions::ListTagsForResource => write!(f, "drs:ListTagsForResource"),
            DrsActions::NotifyAgentAuthenticationForDrs => {
                write!(f, "drs:NotifyAgentAuthenticationForDrs")
            }
            DrsActions::NotifyAgentConnectedForDrs => write!(f, "drs:NotifyAgentConnectedForDrs"),
            DrsActions::NotifyAgentDisconnectedForDrs => {
                write!(f, "drs:NotifyAgentDisconnectedForDrs")
            }
            DrsActions::NotifyAgentReplicationProgressForDrs => {
                write!(f, "drs:NotifyAgentReplicationProgressForDrs")
            }
            DrsActions::NotifyConsistencyAttainedForDrs => {
                write!(f, "drs:NotifyConsistencyAttainedForDrs")
            }
            DrsActions::NotifyReplicationServerAuthenticationForDrs => {
                write!(f, "drs:NotifyReplicationServerAuthenticationForDrs")
            }
            DrsActions::NotifyVolumeEventForDrs => write!(f, "drs:NotifyVolumeEventForDrs"),
            DrsActions::PutLaunchAction => write!(f, "drs:PutLaunchAction"),
            DrsActions::RetryDataReplication => write!(f, "drs:RetryDataReplication"),
            DrsActions::ReverseReplication => write!(f, "drs:ReverseReplication"),
            DrsActions::SendAgentLogsForDrs => write!(f, "drs:SendAgentLogsForDrs"),
            DrsActions::SendAgentMetricsForDrs => write!(f, "drs:SendAgentMetricsForDrs"),
            DrsActions::SendChannelCommandResultForDrs => {
                write!(f, "drs:SendChannelCommandResultForDrs")
            }
            DrsActions::SendClientLogsForDrs => write!(f, "drs:SendClientLogsForDrs"),
            DrsActions::SendClientMetricsForDrs => write!(f, "drs:SendClientMetricsForDrs"),
            DrsActions::SendVolumeStatsForDrs => write!(f, "drs:SendVolumeStatsForDrs"),
            DrsActions::StartFailbackLaunch => write!(f, "drs:StartFailbackLaunch"),
            DrsActions::StartRecovery => write!(f, "drs:StartRecovery"),
            DrsActions::StartReplication => write!(f, "drs:StartReplication"),
            DrsActions::StartSourceNetworkRecovery => write!(f, "drs:StartSourceNetworkRecovery"),
            DrsActions::StartSourceNetworkReplication => {
                write!(f, "drs:StartSourceNetworkReplication")
            }
            DrsActions::StopFailback => write!(f, "drs:StopFailback"),
            DrsActions::StopReplication => write!(f, "drs:StopReplication"),
            DrsActions::StopSourceNetworkReplication => {
                write!(f, "drs:StopSourceNetworkReplication")
            }
            DrsActions::TagResource => write!(f, "drs:TagResource"),
            DrsActions::TerminateRecoveryInstances => write!(f, "drs:TerminateRecoveryInstances"),
            DrsActions::UntagResource => write!(f, "drs:UntagResource"),
            DrsActions::UpdateAgentBacklogForDrs => write!(f, "drs:UpdateAgentBacklogForDrs"),
            DrsActions::UpdateAgentConversionInfoForDrs => {
                write!(f, "drs:UpdateAgentConversionInfoForDrs")
            }
            DrsActions::UpdateAgentReplicationInfoForDrs => {
                write!(f, "drs:UpdateAgentReplicationInfoForDrs")
            }
            DrsActions::UpdateAgentReplicationProcessStateForDrs => {
                write!(f, "drs:UpdateAgentReplicationProcessStateForDrs")
            }
            DrsActions::UpdateAgentSourcePropertiesForDrs => {
                write!(f, "drs:UpdateAgentSourcePropertiesForDrs")
            }
            DrsActions::UpdateFailbackClientDeviceMappingForDrs => {
                write!(f, "drs:UpdateFailbackClientDeviceMappingForDrs")
            }
            DrsActions::UpdateFailbackClientLastSeenForDrs => {
                write!(f, "drs:UpdateFailbackClientLastSeenForDrs")
            }
            DrsActions::UpdateFailbackReplicationConfiguration => {
                write!(f, "drs:UpdateFailbackReplicationConfiguration")
            }
            DrsActions::UpdateLaunchConfiguration => write!(f, "drs:UpdateLaunchConfiguration"),
            DrsActions::UpdateLaunchConfigurationTemplate => {
                write!(f, "drs:UpdateLaunchConfigurationTemplate")
            }
            DrsActions::UpdateReplicationCertificateForDrs => {
                write!(f, "drs:UpdateReplicationCertificateForDrs")
            }
            DrsActions::UpdateReplicationConfiguration => {
                write!(f, "drs:UpdateReplicationConfiguration")
            }
            DrsActions::UpdateReplicationConfigurationTemplate => {
                write!(f, "drs:UpdateReplicationConfigurationTemplate")
            }
        }
    }
}
