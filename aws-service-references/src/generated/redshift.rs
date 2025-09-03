// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum RedshiftActions {
    AcceptReservedNodeExchange,
    AddPartner,
    AssociateDataShareConsumer,
    AuthorizeClusterSecurityGroupIngress,
    AuthorizeDataShare,
    AuthorizeEndpointAccess,
    AuthorizeInboundIntegration,
    AuthorizeSnapshotAccess,
    BatchDeleteClusterSnapshots,
    BatchModifyClusterSnapshots,
    CancelQuery,
    CancelQuerySession,
    CancelResize,
    CopyClusterSnapshot,
    CreateAuthenticationProfile,
    CreateCluster,
    CreateClusterParameterGroup,
    CreateClusterSecurityGroup,
    CreateClusterSnapshot,
    CreateClusterSubnetGroup,
    CreateClusterUser,
    CreateCustomDomainAssociation,
    CreateEndpointAccess,
    CreateEventSubscription,
    CreateHsmClientCertificate,
    CreateHsmConfiguration,
    CreateInboundIntegration,
    CreateIntegration,
    CreateQev2IdcApplication,
    CreateRedshiftIdcApplication,
    CreateSavedQuery,
    CreateScheduledAction,
    CreateSnapshotCopyGrant,
    CreateSnapshotSchedule,
    CreateTags,
    CreateUsageLimit,
    DeauthorizeDataShare,
    DeleteAuthenticationProfile,
    DeleteCluster,
    DeleteClusterParameterGroup,
    DeleteClusterSecurityGroup,
    DeleteClusterSnapshot,
    DeleteClusterSubnetGroup,
    DeleteCustomDomainAssociation,
    DeleteEndpointAccess,
    DeleteEventSubscription,
    DeleteHsmClientCertificate,
    DeleteHsmConfiguration,
    DeleteIntegration,
    DeletePartner,
    DeleteQev2IdcApplication,
    DeleteRedshiftIdcApplication,
    DeleteResourcePolicy,
    DeleteSavedQueries,
    DeleteScheduledAction,
    DeleteSnapshotCopyGrant,
    DeleteSnapshotSchedule,
    DeleteTags,
    DeleteUsageLimit,
    DeregisterNamespace,
    DescribeAccountAttributes,
    DescribeAuthenticationProfiles,
    DescribeClusterDbRevisions,
    DescribeClusterParameterGroups,
    DescribeClusterParameters,
    DescribeClusterSecurityGroups,
    DescribeClusterSnapshots,
    DescribeClusterSubnetGroups,
    DescribeClusterTracks,
    DescribeClusterVersions,
    DescribeClusters,
    DescribeCustomDomainAssociations,
    DescribeDataShares,
    DescribeDataSharesForConsumer,
    DescribeDataSharesForProducer,
    DescribeDefaultClusterParameters,
    DescribeEndpointAccess,
    DescribeEndpointAuthorization,
    DescribeEventCategories,
    DescribeEventSubscriptions,
    DescribeEvents,
    DescribeHsmClientCertificates,
    DescribeHsmConfigurations,
    DescribeInboundIntegrations,
    DescribeIntegrations,
    DescribeLoggingStatus,
    DescribeNodeConfigurationOptions,
    DescribeOrderableClusterOptions,
    DescribePartners,
    DescribeQev2IdcApplications,
    DescribeQuery,
    DescribeRedshiftIdcApplications,
    DescribeReservedNodeExchangeStatus,
    DescribeReservedNodeOfferings,
    DescribeReservedNodes,
    DescribeResize,
    DescribeSavedQueries,
    DescribeScheduledActions,
    DescribeSnapshotCopyGrants,
    DescribeSnapshotSchedules,
    DescribeStorage,
    DescribeTable,
    DescribeTableRestoreStatus,
    DescribeTags,
    DescribeUsageLimits,
    DisableLogging,
    DisableSnapshotCopy,
    DisassociateDataShareConsumer,
    EnableLogging,
    EnableSnapshotCopy,
    ExecuteQuery,
    FailoverPrimaryCompute,
    FetchResults,
    GetClusterCredentials,
    GetClusterCredentialsWithIam,
    GetReservedNodeExchangeConfigurationOptions,
    GetReservedNodeExchangeOfferings,
    GetResourcePolicy,
    JoinGroup,
    ListDatabases,
    ListRecommendations,
    ListSavedQueries,
    ListSchemas,
    ListTables,
    ModifyAquaConfiguration,
    ModifyAuthenticationProfile,
    ModifyCluster,
    ModifyClusterDbRevision,
    ModifyClusterIamRoles,
    ModifyClusterMaintenance,
    ModifyClusterParameterGroup,
    ModifyClusterSnapshot,
    ModifyClusterSnapshotSchedule,
    ModifyClusterSubnetGroup,
    ModifyCustomDomainAssociation,
    ModifyEndpointAccess,
    ModifyEventSubscription,
    ModifyIntegration,
    ModifyQev2IdcApplication,
    ModifyRedshiftIdcApplication,
    ModifySavedQuery,
    ModifyScheduledAction,
    ModifySnapshotCopyRetentionPeriod,
    ModifySnapshotSchedule,
    ModifyUsageLimit,
    PauseCluster,
    PurchaseReservedNodeOffering,
    PutResourcePolicy,
    RebootCluster,
    RegisterNamespace,
    RejectDataShare,
    ResetClusterParameterGroup,
    ResizeCluster,
    RestoreFromClusterSnapshot,
    RestoreTableFromClusterSnapshot,
    ResumeCluster,
    RevokeClusterSecurityGroupIngress,
    RevokeEndpointAccess,
    RevokeSnapshotAccess,
    RotateEncryptionKey,
    UpdatePartnerStatus,
    ViewQueriesFromConsole,
    ViewQueriesInConsole,
}
impl std::fmt::Display for RedshiftActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RedshiftActions::AcceptReservedNodeExchange => {
                write!(f, "redshift:AcceptReservedNodeExchange")
            }
            RedshiftActions::AddPartner => write!(f, "redshift:AddPartner"),
            RedshiftActions::AssociateDataShareConsumer => {
                write!(f, "redshift:AssociateDataShareConsumer")
            }
            RedshiftActions::AuthorizeClusterSecurityGroupIngress => {
                write!(f, "redshift:AuthorizeClusterSecurityGroupIngress")
            }
            RedshiftActions::AuthorizeDataShare => write!(f, "redshift:AuthorizeDataShare"),
            RedshiftActions::AuthorizeEndpointAccess => {
                write!(f, "redshift:AuthorizeEndpointAccess")
            }
            RedshiftActions::AuthorizeInboundIntegration => {
                write!(f, "redshift:AuthorizeInboundIntegration")
            }
            RedshiftActions::AuthorizeSnapshotAccess => {
                write!(f, "redshift:AuthorizeSnapshotAccess")
            }
            RedshiftActions::BatchDeleteClusterSnapshots => {
                write!(f, "redshift:BatchDeleteClusterSnapshots")
            }
            RedshiftActions::BatchModifyClusterSnapshots => {
                write!(f, "redshift:BatchModifyClusterSnapshots")
            }
            RedshiftActions::CancelQuery => write!(f, "redshift:CancelQuery"),
            RedshiftActions::CancelQuerySession => write!(f, "redshift:CancelQuerySession"),
            RedshiftActions::CancelResize => write!(f, "redshift:CancelResize"),
            RedshiftActions::CopyClusterSnapshot => write!(f, "redshift:CopyClusterSnapshot"),
            RedshiftActions::CreateAuthenticationProfile => {
                write!(f, "redshift:CreateAuthenticationProfile")
            }
            RedshiftActions::CreateCluster => write!(f, "redshift:CreateCluster"),
            RedshiftActions::CreateClusterParameterGroup => {
                write!(f, "redshift:CreateClusterParameterGroup")
            }
            RedshiftActions::CreateClusterSecurityGroup => {
                write!(f, "redshift:CreateClusterSecurityGroup")
            }
            RedshiftActions::CreateClusterSnapshot => write!(f, "redshift:CreateClusterSnapshot"),
            RedshiftActions::CreateClusterSubnetGroup => {
                write!(f, "redshift:CreateClusterSubnetGroup")
            }
            RedshiftActions::CreateClusterUser => write!(f, "redshift:CreateClusterUser"),
            RedshiftActions::CreateCustomDomainAssociation => {
                write!(f, "redshift:CreateCustomDomainAssociation")
            }
            RedshiftActions::CreateEndpointAccess => write!(f, "redshift:CreateEndpointAccess"),
            RedshiftActions::CreateEventSubscription => {
                write!(f, "redshift:CreateEventSubscription")
            }
            RedshiftActions::CreateHsmClientCertificate => {
                write!(f, "redshift:CreateHsmClientCertificate")
            }
            RedshiftActions::CreateHsmConfiguration => write!(f, "redshift:CreateHsmConfiguration"),
            RedshiftActions::CreateInboundIntegration => {
                write!(f, "redshift:CreateInboundIntegration")
            }
            RedshiftActions::CreateIntegration => write!(f, "redshift:CreateIntegration"),
            RedshiftActions::CreateQev2IdcApplication => {
                write!(f, "redshift:CreateQev2IdcApplication")
            }
            RedshiftActions::CreateRedshiftIdcApplication => {
                write!(f, "redshift:CreateRedshiftIdcApplication")
            }
            RedshiftActions::CreateSavedQuery => write!(f, "redshift:CreateSavedQuery"),
            RedshiftActions::CreateScheduledAction => write!(f, "redshift:CreateScheduledAction"),
            RedshiftActions::CreateSnapshotCopyGrant => {
                write!(f, "redshift:CreateSnapshotCopyGrant")
            }
            RedshiftActions::CreateSnapshotSchedule => write!(f, "redshift:CreateSnapshotSchedule"),
            RedshiftActions::CreateTags => write!(f, "redshift:CreateTags"),
            RedshiftActions::CreateUsageLimit => write!(f, "redshift:CreateUsageLimit"),
            RedshiftActions::DeauthorizeDataShare => write!(f, "redshift:DeauthorizeDataShare"),
            RedshiftActions::DeleteAuthenticationProfile => {
                write!(f, "redshift:DeleteAuthenticationProfile")
            }
            RedshiftActions::DeleteCluster => write!(f, "redshift:DeleteCluster"),
            RedshiftActions::DeleteClusterParameterGroup => {
                write!(f, "redshift:DeleteClusterParameterGroup")
            }
            RedshiftActions::DeleteClusterSecurityGroup => {
                write!(f, "redshift:DeleteClusterSecurityGroup")
            }
            RedshiftActions::DeleteClusterSnapshot => write!(f, "redshift:DeleteClusterSnapshot"),
            RedshiftActions::DeleteClusterSubnetGroup => {
                write!(f, "redshift:DeleteClusterSubnetGroup")
            }
            RedshiftActions::DeleteCustomDomainAssociation => {
                write!(f, "redshift:DeleteCustomDomainAssociation")
            }
            RedshiftActions::DeleteEndpointAccess => write!(f, "redshift:DeleteEndpointAccess"),
            RedshiftActions::DeleteEventSubscription => {
                write!(f, "redshift:DeleteEventSubscription")
            }
            RedshiftActions::DeleteHsmClientCertificate => {
                write!(f, "redshift:DeleteHsmClientCertificate")
            }
            RedshiftActions::DeleteHsmConfiguration => write!(f, "redshift:DeleteHsmConfiguration"),
            RedshiftActions::DeleteIntegration => write!(f, "redshift:DeleteIntegration"),
            RedshiftActions::DeletePartner => write!(f, "redshift:DeletePartner"),
            RedshiftActions::DeleteQev2IdcApplication => {
                write!(f, "redshift:DeleteQev2IdcApplication")
            }
            RedshiftActions::DeleteRedshiftIdcApplication => {
                write!(f, "redshift:DeleteRedshiftIdcApplication")
            }
            RedshiftActions::DeleteResourcePolicy => write!(f, "redshift:DeleteResourcePolicy"),
            RedshiftActions::DeleteSavedQueries => write!(f, "redshift:DeleteSavedQueries"),
            RedshiftActions::DeleteScheduledAction => write!(f, "redshift:DeleteScheduledAction"),
            RedshiftActions::DeleteSnapshotCopyGrant => {
                write!(f, "redshift:DeleteSnapshotCopyGrant")
            }
            RedshiftActions::DeleteSnapshotSchedule => write!(f, "redshift:DeleteSnapshotSchedule"),
            RedshiftActions::DeleteTags => write!(f, "redshift:DeleteTags"),
            RedshiftActions::DeleteUsageLimit => write!(f, "redshift:DeleteUsageLimit"),
            RedshiftActions::DeregisterNamespace => write!(f, "redshift:DeregisterNamespace"),
            RedshiftActions::DescribeAccountAttributes => {
                write!(f, "redshift:DescribeAccountAttributes")
            }
            RedshiftActions::DescribeAuthenticationProfiles => {
                write!(f, "redshift:DescribeAuthenticationProfiles")
            }
            RedshiftActions::DescribeClusterDbRevisions => {
                write!(f, "redshift:DescribeClusterDbRevisions")
            }
            RedshiftActions::DescribeClusterParameterGroups => {
                write!(f, "redshift:DescribeClusterParameterGroups")
            }
            RedshiftActions::DescribeClusterParameters => {
                write!(f, "redshift:DescribeClusterParameters")
            }
            RedshiftActions::DescribeClusterSecurityGroups => {
                write!(f, "redshift:DescribeClusterSecurityGroups")
            }
            RedshiftActions::DescribeClusterSnapshots => {
                write!(f, "redshift:DescribeClusterSnapshots")
            }
            RedshiftActions::DescribeClusterSubnetGroups => {
                write!(f, "redshift:DescribeClusterSubnetGroups")
            }
            RedshiftActions::DescribeClusterTracks => write!(f, "redshift:DescribeClusterTracks"),
            RedshiftActions::DescribeClusterVersions => {
                write!(f, "redshift:DescribeClusterVersions")
            }
            RedshiftActions::DescribeClusters => write!(f, "redshift:DescribeClusters"),
            RedshiftActions::DescribeCustomDomainAssociations => {
                write!(f, "redshift:DescribeCustomDomainAssociations")
            }
            RedshiftActions::DescribeDataShares => write!(f, "redshift:DescribeDataShares"),
            RedshiftActions::DescribeDataSharesForConsumer => {
                write!(f, "redshift:DescribeDataSharesForConsumer")
            }
            RedshiftActions::DescribeDataSharesForProducer => {
                write!(f, "redshift:DescribeDataSharesForProducer")
            }
            RedshiftActions::DescribeDefaultClusterParameters => {
                write!(f, "redshift:DescribeDefaultClusterParameters")
            }
            RedshiftActions::DescribeEndpointAccess => write!(f, "redshift:DescribeEndpointAccess"),
            RedshiftActions::DescribeEndpointAuthorization => {
                write!(f, "redshift:DescribeEndpointAuthorization")
            }
            RedshiftActions::DescribeEventCategories => {
                write!(f, "redshift:DescribeEventCategories")
            }
            RedshiftActions::DescribeEventSubscriptions => {
                write!(f, "redshift:DescribeEventSubscriptions")
            }
            RedshiftActions::DescribeEvents => write!(f, "redshift:DescribeEvents"),
            RedshiftActions::DescribeHsmClientCertificates => {
                write!(f, "redshift:DescribeHsmClientCertificates")
            }
            RedshiftActions::DescribeHsmConfigurations => {
                write!(f, "redshift:DescribeHsmConfigurations")
            }
            RedshiftActions::DescribeInboundIntegrations => {
                write!(f, "redshift:DescribeInboundIntegrations")
            }
            RedshiftActions::DescribeIntegrations => write!(f, "redshift:DescribeIntegrations"),
            RedshiftActions::DescribeLoggingStatus => write!(f, "redshift:DescribeLoggingStatus"),
            RedshiftActions::DescribeNodeConfigurationOptions => {
                write!(f, "redshift:DescribeNodeConfigurationOptions")
            }
            RedshiftActions::DescribeOrderableClusterOptions => {
                write!(f, "redshift:DescribeOrderableClusterOptions")
            }
            RedshiftActions::DescribePartners => write!(f, "redshift:DescribePartners"),
            RedshiftActions::DescribeQev2IdcApplications => {
                write!(f, "redshift:DescribeQev2IdcApplications")
            }
            RedshiftActions::DescribeQuery => write!(f, "redshift:DescribeQuery"),
            RedshiftActions::DescribeRedshiftIdcApplications => {
                write!(f, "redshift:DescribeRedshiftIdcApplications")
            }
            RedshiftActions::DescribeReservedNodeExchangeStatus => {
                write!(f, "redshift:DescribeReservedNodeExchangeStatus")
            }
            RedshiftActions::DescribeReservedNodeOfferings => {
                write!(f, "redshift:DescribeReservedNodeOfferings")
            }
            RedshiftActions::DescribeReservedNodes => write!(f, "redshift:DescribeReservedNodes"),
            RedshiftActions::DescribeResize => write!(f, "redshift:DescribeResize"),
            RedshiftActions::DescribeSavedQueries => write!(f, "redshift:DescribeSavedQueries"),
            RedshiftActions::DescribeScheduledActions => {
                write!(f, "redshift:DescribeScheduledActions")
            }
            RedshiftActions::DescribeSnapshotCopyGrants => {
                write!(f, "redshift:DescribeSnapshotCopyGrants")
            }
            RedshiftActions::DescribeSnapshotSchedules => {
                write!(f, "redshift:DescribeSnapshotSchedules")
            }
            RedshiftActions::DescribeStorage => write!(f, "redshift:DescribeStorage"),
            RedshiftActions::DescribeTable => write!(f, "redshift:DescribeTable"),
            RedshiftActions::DescribeTableRestoreStatus => {
                write!(f, "redshift:DescribeTableRestoreStatus")
            }
            RedshiftActions::DescribeTags => write!(f, "redshift:DescribeTags"),
            RedshiftActions::DescribeUsageLimits => write!(f, "redshift:DescribeUsageLimits"),
            RedshiftActions::DisableLogging => write!(f, "redshift:DisableLogging"),
            RedshiftActions::DisableSnapshotCopy => write!(f, "redshift:DisableSnapshotCopy"),
            RedshiftActions::DisassociateDataShareConsumer => {
                write!(f, "redshift:DisassociateDataShareConsumer")
            }
            RedshiftActions::EnableLogging => write!(f, "redshift:EnableLogging"),
            RedshiftActions::EnableSnapshotCopy => write!(f, "redshift:EnableSnapshotCopy"),
            RedshiftActions::ExecuteQuery => write!(f, "redshift:ExecuteQuery"),
            RedshiftActions::FailoverPrimaryCompute => write!(f, "redshift:FailoverPrimaryCompute"),
            RedshiftActions::FetchResults => write!(f, "redshift:FetchResults"),
            RedshiftActions::GetClusterCredentials => write!(f, "redshift:GetClusterCredentials"),
            RedshiftActions::GetClusterCredentialsWithIam => {
                write!(f, "redshift:GetClusterCredentialsWithIAM")
            }
            RedshiftActions::GetReservedNodeExchangeConfigurationOptions => {
                write!(f, "redshift:GetReservedNodeExchangeConfigurationOptions")
            }
            RedshiftActions::GetReservedNodeExchangeOfferings => {
                write!(f, "redshift:GetReservedNodeExchangeOfferings")
            }
            RedshiftActions::GetResourcePolicy => write!(f, "redshift:GetResourcePolicy"),
            RedshiftActions::JoinGroup => write!(f, "redshift:JoinGroup"),
            RedshiftActions::ListDatabases => write!(f, "redshift:ListDatabases"),
            RedshiftActions::ListRecommendations => write!(f, "redshift:ListRecommendations"),
            RedshiftActions::ListSavedQueries => write!(f, "redshift:ListSavedQueries"),
            RedshiftActions::ListSchemas => write!(f, "redshift:ListSchemas"),
            RedshiftActions::ListTables => write!(f, "redshift:ListTables"),
            RedshiftActions::ModifyAquaConfiguration => {
                write!(f, "redshift:ModifyAquaConfiguration")
            }
            RedshiftActions::ModifyAuthenticationProfile => {
                write!(f, "redshift:ModifyAuthenticationProfile")
            }
            RedshiftActions::ModifyCluster => write!(f, "redshift:ModifyCluster"),
            RedshiftActions::ModifyClusterDbRevision => {
                write!(f, "redshift:ModifyClusterDbRevision")
            }
            RedshiftActions::ModifyClusterIamRoles => write!(f, "redshift:ModifyClusterIamRoles"),
            RedshiftActions::ModifyClusterMaintenance => {
                write!(f, "redshift:ModifyClusterMaintenance")
            }
            RedshiftActions::ModifyClusterParameterGroup => {
                write!(f, "redshift:ModifyClusterParameterGroup")
            }
            RedshiftActions::ModifyClusterSnapshot => write!(f, "redshift:ModifyClusterSnapshot"),
            RedshiftActions::ModifyClusterSnapshotSchedule => {
                write!(f, "redshift:ModifyClusterSnapshotSchedule")
            }
            RedshiftActions::ModifyClusterSubnetGroup => {
                write!(f, "redshift:ModifyClusterSubnetGroup")
            }
            RedshiftActions::ModifyCustomDomainAssociation => {
                write!(f, "redshift:ModifyCustomDomainAssociation")
            }
            RedshiftActions::ModifyEndpointAccess => write!(f, "redshift:ModifyEndpointAccess"),
            RedshiftActions::ModifyEventSubscription => {
                write!(f, "redshift:ModifyEventSubscription")
            }
            RedshiftActions::ModifyIntegration => write!(f, "redshift:ModifyIntegration"),
            RedshiftActions::ModifyQev2IdcApplication => {
                write!(f, "redshift:ModifyQev2IdcApplication")
            }
            RedshiftActions::ModifyRedshiftIdcApplication => {
                write!(f, "redshift:ModifyRedshiftIdcApplication")
            }
            RedshiftActions::ModifySavedQuery => write!(f, "redshift:ModifySavedQuery"),
            RedshiftActions::ModifyScheduledAction => write!(f, "redshift:ModifyScheduledAction"),
            RedshiftActions::ModifySnapshotCopyRetentionPeriod => {
                write!(f, "redshift:ModifySnapshotCopyRetentionPeriod")
            }
            RedshiftActions::ModifySnapshotSchedule => write!(f, "redshift:ModifySnapshotSchedule"),
            RedshiftActions::ModifyUsageLimit => write!(f, "redshift:ModifyUsageLimit"),
            RedshiftActions::PauseCluster => write!(f, "redshift:PauseCluster"),
            RedshiftActions::PurchaseReservedNodeOffering => {
                write!(f, "redshift:PurchaseReservedNodeOffering")
            }
            RedshiftActions::PutResourcePolicy => write!(f, "redshift:PutResourcePolicy"),
            RedshiftActions::RebootCluster => write!(f, "redshift:RebootCluster"),
            RedshiftActions::RegisterNamespace => write!(f, "redshift:RegisterNamespace"),
            RedshiftActions::RejectDataShare => write!(f, "redshift:RejectDataShare"),
            RedshiftActions::ResetClusterParameterGroup => {
                write!(f, "redshift:ResetClusterParameterGroup")
            }
            RedshiftActions::ResizeCluster => write!(f, "redshift:ResizeCluster"),
            RedshiftActions::RestoreFromClusterSnapshot => {
                write!(f, "redshift:RestoreFromClusterSnapshot")
            }
            RedshiftActions::RestoreTableFromClusterSnapshot => {
                write!(f, "redshift:RestoreTableFromClusterSnapshot")
            }
            RedshiftActions::ResumeCluster => write!(f, "redshift:ResumeCluster"),
            RedshiftActions::RevokeClusterSecurityGroupIngress => {
                write!(f, "redshift:RevokeClusterSecurityGroupIngress")
            }
            RedshiftActions::RevokeEndpointAccess => write!(f, "redshift:RevokeEndpointAccess"),
            RedshiftActions::RevokeSnapshotAccess => write!(f, "redshift:RevokeSnapshotAccess"),
            RedshiftActions::RotateEncryptionKey => write!(f, "redshift:RotateEncryptionKey"),
            RedshiftActions::UpdatePartnerStatus => write!(f, "redshift:UpdatePartnerStatus"),
            RedshiftActions::ViewQueriesFromConsole => write!(f, "redshift:ViewQueriesFromConsole"),
            RedshiftActions::ViewQueriesInConsole => write!(f, "redshift:ViewQueriesInConsole"),
        }
    }
}
