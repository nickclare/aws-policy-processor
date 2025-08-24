// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum RdsActions {
    AddRoleToDbCluster,
    AddRoleToDbInstance,
    AddSourceIdentifierToSubscription,
    AddTagsToResource,
    ApplyPendingMaintenanceAction,
    AuthorizeDbSecurityGroupIngress,
    BacktrackDbCluster,
    CancelExportTask,
    CopyCustomDbEngineVersion,
    CopyDbClusterParameterGroup,
    CopyDbClusterSnapshot,
    CopyDbParameterGroup,
    CopyDbSnapshot,
    CopyOptionGroup,
    CreateBlueGreenDeployment,
    CreateCustomDbEngineVersion,
    CreateDbCluster,
    CreateDbClusterEndpoint,
    CreateDbClusterParameterGroup,
    CreateDbClusterSnapshot,
    CreateDbInstance,
    CreateDbInstanceReadReplica,
    CreateDbParameterGroup,
    CreateDbProxy,
    CreateDbProxyEndpoint,
    CreateDbSecurityGroup,
    CreateDbShardGroup,
    CreateDbSnapshot,
    CreateDbSubnetGroup,
    CreateEventSubscription,
    CreateGlobalCluster,
    CreateIntegration,
    CreateOptionGroup,
    CreateTenantDatabase,
    CrossRegionCommunication,
    DeleteBlueGreenDeployment,
    DeleteCustomDbEngineVersion,
    DeleteDbCluster,
    DeleteDbClusterAutomatedBackup,
    DeleteDbClusterEndpoint,
    DeleteDbClusterParameterGroup,
    DeleteDbClusterSnapshot,
    DeleteDbInstance,
    DeleteDbInstanceAutomatedBackup,
    DeleteDbParameterGroup,
    DeleteDbProxy,
    DeleteDbProxyEndpoint,
    DeleteDbSecurityGroup,
    DeleteDbShardGroup,
    DeleteDbSnapshot,
    DeleteDbSubnetGroup,
    DeleteEventSubscription,
    DeleteGlobalCluster,
    DeleteIntegration,
    DeleteOptionGroup,
    DeleteTenantDatabase,
    DeregisterDbProxyTargets,
    DescribeAccountAttributes,
    DescribeBlueGreenDeployments,
    DescribeCertificates,
    DescribeDbClusterAutomatedBackups,
    DescribeDbClusterBacktracks,
    DescribeDbClusterEndpoints,
    DescribeDbClusterParameterGroups,
    DescribeDbClusterParameters,
    DescribeDbClusterSnapshotAttributes,
    DescribeDbClusterSnapshots,
    DescribeDbClusters,
    DescribeDbEngineVersions,
    DescribeDbInstanceAutomatedBackups,
    DescribeDbInstances,
    DescribeDbLogFiles,
    DescribeDbMajorEngineVersions,
    DescribeDbParameterGroups,
    DescribeDbParameters,
    DescribeDbProxies,
    DescribeDbProxyEndpoints,
    DescribeDbProxyTargetGroups,
    DescribeDbProxyTargets,
    DescribeDbRecommendations,
    DescribeDbSecurityGroups,
    DescribeDbShardGroups,
    DescribeDbSnapshotAttributes,
    DescribeDbSnapshotTenantDatabases,
    DescribeDbSnapshots,
    DescribeDbSubnetGroups,
    DescribeEngineDefaultClusterParameters,
    DescribeEngineDefaultParameters,
    DescribeEventCategories,
    DescribeEventSubscriptions,
    DescribeEvents,
    DescribeExportTasks,
    DescribeGlobalClusters,
    DescribeIntegrations,
    DescribeOptionGroupOptions,
    DescribeOptionGroups,
    DescribeOrderableDbInstanceOptions,
    DescribePendingMaintenanceActions,
    DescribeRecommendationGroups,
    DescribeRecommendations,
    DescribeReservedDbInstances,
    DescribeReservedDbInstancesOfferings,
    DescribeSourceRegions,
    DescribeTenantDatabases,
    DescribeValidDbInstanceModifications,
    DisableHttpEndpoint,
    DownloadCompleteDbLogFile,
    DownloadDbLogFilePortion,
    EnableHttpEndpoint,
    FailoverDbCluster,
    FailoverGlobalCluster,
    ListTagsForResource,
    ModifyActivityStream,
    ModifyCertificates,
    ModifyCurrentDbClusterCapacity,
    ModifyCustomDbEngineVersion,
    ModifyDbCluster,
    ModifyDbClusterEndpoint,
    ModifyDbClusterParameterGroup,
    ModifyDbClusterSnapshotAttribute,
    ModifyDbInstance,
    ModifyDbParameterGroup,
    ModifyDbProxy,
    ModifyDbProxyEndpoint,
    ModifyDbProxyTargetGroup,
    ModifyDbRecommendation,
    ModifyDbShardGroup,
    ModifyDbSnapshot,
    ModifyDbSnapshotAttribute,
    ModifyDbSubnetGroup,
    ModifyEventSubscription,
    ModifyGlobalCluster,
    ModifyIntegration,
    ModifyOptionGroup,
    ModifyRecommendation,
    ModifyTenantDatabase,
    PromoteReadReplica,
    PromoteReadReplicaDbCluster,
    PurchaseReservedDbInstancesOffering,
    RebootDbCluster,
    RebootDbInstance,
    RebootDbShardGroup,
    RegisterDbProxyTargets,
    RemoveFromGlobalCluster,
    RemoveRoleFromDbCluster,
    RemoveRoleFromDbInstance,
    RemoveSourceIdentifierFromSubscription,
    RemoveTagsFromResource,
    ResetDbClusterParameterGroup,
    ResetDbParameterGroup,
    RestoreDbClusterFromS3,
    RestoreDbClusterFromSnapshot,
    RestoreDbClusterToPointInTime,
    RestoreDbInstanceFromDbSnapshot,
    RestoreDbInstanceFromS3,
    RestoreDbInstanceToPointInTime,
    RevokeDbSecurityGroupIngress,
    StartActivityStream,
    StartDbCluster,
    StartDbInstance,
    StartDbInstanceAutomatedBackupsReplication,
    StartExportTask,
    StopActivityStream,
    StopDbCluster,
    StopDbInstance,
    StopDbInstanceAutomatedBackupsReplication,
    SwitchoverBlueGreenDeployment,
    SwitchoverGlobalCluster,
    SwitchoverReadReplica,
}
impl std::fmt::Display for RdsActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RdsActions::AddRoleToDbCluster => write!(f, "rds:AddRoleToDBCluster"),
            RdsActions::AddRoleToDbInstance => write!(f, "rds:AddRoleToDBInstance"),
            RdsActions::AddSourceIdentifierToSubscription => {
                write!(f, "rds:AddSourceIdentifierToSubscription")
            }
            RdsActions::AddTagsToResource => write!(f, "rds:AddTagsToResource"),
            RdsActions::ApplyPendingMaintenanceAction => {
                write!(f, "rds:ApplyPendingMaintenanceAction")
            }
            RdsActions::AuthorizeDbSecurityGroupIngress => {
                write!(f, "rds:AuthorizeDBSecurityGroupIngress")
            }
            RdsActions::BacktrackDbCluster => write!(f, "rds:BacktrackDBCluster"),
            RdsActions::CancelExportTask => write!(f, "rds:CancelExportTask"),
            RdsActions::CopyCustomDbEngineVersion => write!(f, "rds:CopyCustomDBEngineVersion"),
            RdsActions::CopyDbClusterParameterGroup => write!(f, "rds:CopyDBClusterParameterGroup"),
            RdsActions::CopyDbClusterSnapshot => write!(f, "rds:CopyDBClusterSnapshot"),
            RdsActions::CopyDbParameterGroup => write!(f, "rds:CopyDBParameterGroup"),
            RdsActions::CopyDbSnapshot => write!(f, "rds:CopyDBSnapshot"),
            RdsActions::CopyOptionGroup => write!(f, "rds:CopyOptionGroup"),
            RdsActions::CreateBlueGreenDeployment => write!(f, "rds:CreateBlueGreenDeployment"),
            RdsActions::CreateCustomDbEngineVersion => write!(f, "rds:CreateCustomDBEngineVersion"),
            RdsActions::CreateDbCluster => write!(f, "rds:CreateDBCluster"),
            RdsActions::CreateDbClusterEndpoint => write!(f, "rds:CreateDBClusterEndpoint"),
            RdsActions::CreateDbClusterParameterGroup => {
                write!(f, "rds:CreateDBClusterParameterGroup")
            }
            RdsActions::CreateDbClusterSnapshot => write!(f, "rds:CreateDBClusterSnapshot"),
            RdsActions::CreateDbInstance => write!(f, "rds:CreateDBInstance"),
            RdsActions::CreateDbInstanceReadReplica => write!(f, "rds:CreateDBInstanceReadReplica"),
            RdsActions::CreateDbParameterGroup => write!(f, "rds:CreateDBParameterGroup"),
            RdsActions::CreateDbProxy => write!(f, "rds:CreateDBProxy"),
            RdsActions::CreateDbProxyEndpoint => write!(f, "rds:CreateDBProxyEndpoint"),
            RdsActions::CreateDbSecurityGroup => write!(f, "rds:CreateDBSecurityGroup"),
            RdsActions::CreateDbShardGroup => write!(f, "rds:CreateDBShardGroup"),
            RdsActions::CreateDbSnapshot => write!(f, "rds:CreateDBSnapshot"),
            RdsActions::CreateDbSubnetGroup => write!(f, "rds:CreateDBSubnetGroup"),
            RdsActions::CreateEventSubscription => write!(f, "rds:CreateEventSubscription"),
            RdsActions::CreateGlobalCluster => write!(f, "rds:CreateGlobalCluster"),
            RdsActions::CreateIntegration => write!(f, "rds:CreateIntegration"),
            RdsActions::CreateOptionGroup => write!(f, "rds:CreateOptionGroup"),
            RdsActions::CreateTenantDatabase => write!(f, "rds:CreateTenantDatabase"),
            RdsActions::CrossRegionCommunication => write!(f, "rds:CrossRegionCommunication"),
            RdsActions::DeleteBlueGreenDeployment => write!(f, "rds:DeleteBlueGreenDeployment"),
            RdsActions::DeleteCustomDbEngineVersion => write!(f, "rds:DeleteCustomDBEngineVersion"),
            RdsActions::DeleteDbCluster => write!(f, "rds:DeleteDBCluster"),
            RdsActions::DeleteDbClusterAutomatedBackup => {
                write!(f, "rds:DeleteDBClusterAutomatedBackup")
            }
            RdsActions::DeleteDbClusterEndpoint => write!(f, "rds:DeleteDBClusterEndpoint"),
            RdsActions::DeleteDbClusterParameterGroup => {
                write!(f, "rds:DeleteDBClusterParameterGroup")
            }
            RdsActions::DeleteDbClusterSnapshot => write!(f, "rds:DeleteDBClusterSnapshot"),
            RdsActions::DeleteDbInstance => write!(f, "rds:DeleteDBInstance"),
            RdsActions::DeleteDbInstanceAutomatedBackup => {
                write!(f, "rds:DeleteDBInstanceAutomatedBackup")
            }
            RdsActions::DeleteDbParameterGroup => write!(f, "rds:DeleteDBParameterGroup"),
            RdsActions::DeleteDbProxy => write!(f, "rds:DeleteDBProxy"),
            RdsActions::DeleteDbProxyEndpoint => write!(f, "rds:DeleteDBProxyEndpoint"),
            RdsActions::DeleteDbSecurityGroup => write!(f, "rds:DeleteDBSecurityGroup"),
            RdsActions::DeleteDbShardGroup => write!(f, "rds:DeleteDBShardGroup"),
            RdsActions::DeleteDbSnapshot => write!(f, "rds:DeleteDBSnapshot"),
            RdsActions::DeleteDbSubnetGroup => write!(f, "rds:DeleteDBSubnetGroup"),
            RdsActions::DeleteEventSubscription => write!(f, "rds:DeleteEventSubscription"),
            RdsActions::DeleteGlobalCluster => write!(f, "rds:DeleteGlobalCluster"),
            RdsActions::DeleteIntegration => write!(f, "rds:DeleteIntegration"),
            RdsActions::DeleteOptionGroup => write!(f, "rds:DeleteOptionGroup"),
            RdsActions::DeleteTenantDatabase => write!(f, "rds:DeleteTenantDatabase"),
            RdsActions::DeregisterDbProxyTargets => write!(f, "rds:DeregisterDBProxyTargets"),
            RdsActions::DescribeAccountAttributes => write!(f, "rds:DescribeAccountAttributes"),
            RdsActions::DescribeBlueGreenDeployments => {
                write!(f, "rds:DescribeBlueGreenDeployments")
            }
            RdsActions::DescribeCertificates => write!(f, "rds:DescribeCertificates"),
            RdsActions::DescribeDbClusterAutomatedBackups => {
                write!(f, "rds:DescribeDBClusterAutomatedBackups")
            }
            RdsActions::DescribeDbClusterBacktracks => write!(f, "rds:DescribeDBClusterBacktracks"),
            RdsActions::DescribeDbClusterEndpoints => write!(f, "rds:DescribeDBClusterEndpoints"),
            RdsActions::DescribeDbClusterParameterGroups => {
                write!(f, "rds:DescribeDBClusterParameterGroups")
            }
            RdsActions::DescribeDbClusterParameters => write!(f, "rds:DescribeDBClusterParameters"),
            RdsActions::DescribeDbClusterSnapshotAttributes => {
                write!(f, "rds:DescribeDBClusterSnapshotAttributes")
            }
            RdsActions::DescribeDbClusterSnapshots => write!(f, "rds:DescribeDBClusterSnapshots"),
            RdsActions::DescribeDbClusters => write!(f, "rds:DescribeDBClusters"),
            RdsActions::DescribeDbEngineVersions => write!(f, "rds:DescribeDBEngineVersions"),
            RdsActions::DescribeDbInstanceAutomatedBackups => {
                write!(f, "rds:DescribeDBInstanceAutomatedBackups")
            }
            RdsActions::DescribeDbInstances => write!(f, "rds:DescribeDBInstances"),
            RdsActions::DescribeDbLogFiles => write!(f, "rds:DescribeDBLogFiles"),
            RdsActions::DescribeDbMajorEngineVersions => {
                write!(f, "rds:DescribeDBMajorEngineVersions")
            }
            RdsActions::DescribeDbParameterGroups => write!(f, "rds:DescribeDBParameterGroups"),
            RdsActions::DescribeDbParameters => write!(f, "rds:DescribeDBParameters"),
            RdsActions::DescribeDbProxies => write!(f, "rds:DescribeDBProxies"),
            RdsActions::DescribeDbProxyEndpoints => write!(f, "rds:DescribeDBProxyEndpoints"),
            RdsActions::DescribeDbProxyTargetGroups => write!(f, "rds:DescribeDBProxyTargetGroups"),
            RdsActions::DescribeDbProxyTargets => write!(f, "rds:DescribeDBProxyTargets"),
            RdsActions::DescribeDbRecommendations => write!(f, "rds:DescribeDBRecommendations"),
            RdsActions::DescribeDbSecurityGroups => write!(f, "rds:DescribeDBSecurityGroups"),
            RdsActions::DescribeDbShardGroups => write!(f, "rds:DescribeDBShardGroups"),
            RdsActions::DescribeDbSnapshotAttributes => {
                write!(f, "rds:DescribeDBSnapshotAttributes")
            }
            RdsActions::DescribeDbSnapshotTenantDatabases => {
                write!(f, "rds:DescribeDBSnapshotTenantDatabases")
            }
            RdsActions::DescribeDbSnapshots => write!(f, "rds:DescribeDBSnapshots"),
            RdsActions::DescribeDbSubnetGroups => write!(f, "rds:DescribeDBSubnetGroups"),
            RdsActions::DescribeEngineDefaultClusterParameters => {
                write!(f, "rds:DescribeEngineDefaultClusterParameters")
            }
            RdsActions::DescribeEngineDefaultParameters => {
                write!(f, "rds:DescribeEngineDefaultParameters")
            }
            RdsActions::DescribeEventCategories => write!(f, "rds:DescribeEventCategories"),
            RdsActions::DescribeEventSubscriptions => write!(f, "rds:DescribeEventSubscriptions"),
            RdsActions::DescribeEvents => write!(f, "rds:DescribeEvents"),
            RdsActions::DescribeExportTasks => write!(f, "rds:DescribeExportTasks"),
            RdsActions::DescribeGlobalClusters => write!(f, "rds:DescribeGlobalClusters"),
            RdsActions::DescribeIntegrations => write!(f, "rds:DescribeIntegrations"),
            RdsActions::DescribeOptionGroupOptions => write!(f, "rds:DescribeOptionGroupOptions"),
            RdsActions::DescribeOptionGroups => write!(f, "rds:DescribeOptionGroups"),
            RdsActions::DescribeOrderableDbInstanceOptions => {
                write!(f, "rds:DescribeOrderableDBInstanceOptions")
            }
            RdsActions::DescribePendingMaintenanceActions => {
                write!(f, "rds:DescribePendingMaintenanceActions")
            }
            RdsActions::DescribeRecommendationGroups => {
                write!(f, "rds:DescribeRecommendationGroups")
            }
            RdsActions::DescribeRecommendations => write!(f, "rds:DescribeRecommendations"),
            RdsActions::DescribeReservedDbInstances => write!(f, "rds:DescribeReservedDBInstances"),
            RdsActions::DescribeReservedDbInstancesOfferings => {
                write!(f, "rds:DescribeReservedDBInstancesOfferings")
            }
            RdsActions::DescribeSourceRegions => write!(f, "rds:DescribeSourceRegions"),
            RdsActions::DescribeTenantDatabases => write!(f, "rds:DescribeTenantDatabases"),
            RdsActions::DescribeValidDbInstanceModifications => {
                write!(f, "rds:DescribeValidDBInstanceModifications")
            }
            RdsActions::DisableHttpEndpoint => write!(f, "rds:DisableHttpEndpoint"),
            RdsActions::DownloadCompleteDbLogFile => write!(f, "rds:DownloadCompleteDBLogFile"),
            RdsActions::DownloadDbLogFilePortion => write!(f, "rds:DownloadDBLogFilePortion"),
            RdsActions::EnableHttpEndpoint => write!(f, "rds:EnableHttpEndpoint"),
            RdsActions::FailoverDbCluster => write!(f, "rds:FailoverDBCluster"),
            RdsActions::FailoverGlobalCluster => write!(f, "rds:FailoverGlobalCluster"),
            RdsActions::ListTagsForResource => write!(f, "rds:ListTagsForResource"),
            RdsActions::ModifyActivityStream => write!(f, "rds:ModifyActivityStream"),
            RdsActions::ModifyCertificates => write!(f, "rds:ModifyCertificates"),
            RdsActions::ModifyCurrentDbClusterCapacity => {
                write!(f, "rds:ModifyCurrentDBClusterCapacity")
            }
            RdsActions::ModifyCustomDbEngineVersion => write!(f, "rds:ModifyCustomDBEngineVersion"),
            RdsActions::ModifyDbCluster => write!(f, "rds:ModifyDBCluster"),
            RdsActions::ModifyDbClusterEndpoint => write!(f, "rds:ModifyDBClusterEndpoint"),
            RdsActions::ModifyDbClusterParameterGroup => {
                write!(f, "rds:ModifyDBClusterParameterGroup")
            }
            RdsActions::ModifyDbClusterSnapshotAttribute => {
                write!(f, "rds:ModifyDBClusterSnapshotAttribute")
            }
            RdsActions::ModifyDbInstance => write!(f, "rds:ModifyDBInstance"),
            RdsActions::ModifyDbParameterGroup => write!(f, "rds:ModifyDBParameterGroup"),
            RdsActions::ModifyDbProxy => write!(f, "rds:ModifyDBProxy"),
            RdsActions::ModifyDbProxyEndpoint => write!(f, "rds:ModifyDBProxyEndpoint"),
            RdsActions::ModifyDbProxyTargetGroup => write!(f, "rds:ModifyDBProxyTargetGroup"),
            RdsActions::ModifyDbRecommendation => write!(f, "rds:ModifyDBRecommendation"),
            RdsActions::ModifyDbShardGroup => write!(f, "rds:ModifyDBShardGroup"),
            RdsActions::ModifyDbSnapshot => write!(f, "rds:ModifyDBSnapshot"),
            RdsActions::ModifyDbSnapshotAttribute => write!(f, "rds:ModifyDBSnapshotAttribute"),
            RdsActions::ModifyDbSubnetGroup => write!(f, "rds:ModifyDBSubnetGroup"),
            RdsActions::ModifyEventSubscription => write!(f, "rds:ModifyEventSubscription"),
            RdsActions::ModifyGlobalCluster => write!(f, "rds:ModifyGlobalCluster"),
            RdsActions::ModifyIntegration => write!(f, "rds:ModifyIntegration"),
            RdsActions::ModifyOptionGroup => write!(f, "rds:ModifyOptionGroup"),
            RdsActions::ModifyRecommendation => write!(f, "rds:ModifyRecommendation"),
            RdsActions::ModifyTenantDatabase => write!(f, "rds:ModifyTenantDatabase"),
            RdsActions::PromoteReadReplica => write!(f, "rds:PromoteReadReplica"),
            RdsActions::PromoteReadReplicaDbCluster => write!(f, "rds:PromoteReadReplicaDBCluster"),
            RdsActions::PurchaseReservedDbInstancesOffering => {
                write!(f, "rds:PurchaseReservedDBInstancesOffering")
            }
            RdsActions::RebootDbCluster => write!(f, "rds:RebootDBCluster"),
            RdsActions::RebootDbInstance => write!(f, "rds:RebootDBInstance"),
            RdsActions::RebootDbShardGroup => write!(f, "rds:RebootDBShardGroup"),
            RdsActions::RegisterDbProxyTargets => write!(f, "rds:RegisterDBProxyTargets"),
            RdsActions::RemoveFromGlobalCluster => write!(f, "rds:RemoveFromGlobalCluster"),
            RdsActions::RemoveRoleFromDbCluster => write!(f, "rds:RemoveRoleFromDBCluster"),
            RdsActions::RemoveRoleFromDbInstance => write!(f, "rds:RemoveRoleFromDBInstance"),
            RdsActions::RemoveSourceIdentifierFromSubscription => {
                write!(f, "rds:RemoveSourceIdentifierFromSubscription")
            }
            RdsActions::RemoveTagsFromResource => write!(f, "rds:RemoveTagsFromResource"),
            RdsActions::ResetDbClusterParameterGroup => {
                write!(f, "rds:ResetDBClusterParameterGroup")
            }
            RdsActions::ResetDbParameterGroup => write!(f, "rds:ResetDBParameterGroup"),
            RdsActions::RestoreDbClusterFromS3 => write!(f, "rds:RestoreDBClusterFromS3"),
            RdsActions::RestoreDbClusterFromSnapshot => {
                write!(f, "rds:RestoreDBClusterFromSnapshot")
            }
            RdsActions::RestoreDbClusterToPointInTime => {
                write!(f, "rds:RestoreDBClusterToPointInTime")
            }
            RdsActions::RestoreDbInstanceFromDbSnapshot => {
                write!(f, "rds:RestoreDBInstanceFromDBSnapshot")
            }
            RdsActions::RestoreDbInstanceFromS3 => write!(f, "rds:RestoreDBInstanceFromS3"),
            RdsActions::RestoreDbInstanceToPointInTime => {
                write!(f, "rds:RestoreDBInstanceToPointInTime")
            }
            RdsActions::RevokeDbSecurityGroupIngress => {
                write!(f, "rds:RevokeDBSecurityGroupIngress")
            }
            RdsActions::StartActivityStream => write!(f, "rds:StartActivityStream"),
            RdsActions::StartDbCluster => write!(f, "rds:StartDBCluster"),
            RdsActions::StartDbInstance => write!(f, "rds:StartDBInstance"),
            RdsActions::StartDbInstanceAutomatedBackupsReplication => {
                write!(f, "rds:StartDBInstanceAutomatedBackupsReplication")
            }
            RdsActions::StartExportTask => write!(f, "rds:StartExportTask"),
            RdsActions::StopActivityStream => write!(f, "rds:StopActivityStream"),
            RdsActions::StopDbCluster => write!(f, "rds:StopDBCluster"),
            RdsActions::StopDbInstance => write!(f, "rds:StopDBInstance"),
            RdsActions::StopDbInstanceAutomatedBackupsReplication => {
                write!(f, "rds:StopDBInstanceAutomatedBackupsReplication")
            }
            RdsActions::SwitchoverBlueGreenDeployment => {
                write!(f, "rds:SwitchoverBlueGreenDeployment")
            }
            RdsActions::SwitchoverGlobalCluster => write!(f, "rds:SwitchoverGlobalCluster"),
            RdsActions::SwitchoverReadReplica => write!(f, "rds:SwitchoverReadReplica"),
        }
    }
}
