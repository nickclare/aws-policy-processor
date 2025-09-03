// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum DmsActions {
    AddTagsToResource,
    ApplyPendingMaintenanceAction,
    AssociateExtensionPack,
    BatchStartRecommendations,
    CancelReplicationTaskAssessmentRun,
    CreateDataMigration,
    CreateDataProvider,
    CreateEndpoint,
    CreateEventSubscription,
    CreateFleetAdvisorCollector,
    CreateInstanceProfile,
    CreateMigrationProject,
    CreateReplicationConfig,
    CreateReplicationInstance,
    CreateReplicationSubnetGroup,
    CreateReplicationTask,
    DeleteCertificate,
    DeleteConnection,
    DeleteDataMigration,
    DeleteDataProvider,
    DeleteEndpoint,
    DeleteEventSubscription,
    DeleteFleetAdvisorCollector,
    DeleteFleetAdvisorDatabases,
    DeleteInstanceProfile,
    DeleteMigrationProject,
    DeleteReplicationConfig,
    DeleteReplicationInstance,
    DeleteReplicationSubnetGroup,
    DeleteReplicationTask,
    DeleteReplicationTaskAssessmentRun,
    DescribeAccountAttributes,
    DescribeApplicableIndividualAssessments,
    DescribeCertificates,
    DescribeConnections,
    DescribeConversionConfiguration,
    DescribeDataMigrations,
    DescribeEndpointSettings,
    DescribeEndpointTypes,
    DescribeEndpoints,
    DescribeEngineVersions,
    DescribeEventCategories,
    DescribeEventSubscriptions,
    DescribeEvents,
    DescribeFleetAdvisorCollectors,
    DescribeFleetAdvisorDatabases,
    DescribeFleetAdvisorLsaAnalysis,
    DescribeFleetAdvisorSchemaObjectSummary,
    DescribeFleetAdvisorSchemas,
    DescribeMetadataModelImports,
    DescribeOrderableReplicationInstances,
    DescribePendingMaintenanceActions,
    DescribeRecommendationLimitations,
    DescribeRecommendations,
    DescribeRefreshSchemasStatus,
    DescribeReplicationConfigs,
    DescribeReplicationInstanceTaskLogs,
    DescribeReplicationInstances,
    DescribeReplicationSubnetGroups,
    DescribeReplicationTableStatistics,
    DescribeReplicationTaskAssessmentResults,
    DescribeReplicationTaskAssessmentRuns,
    DescribeReplicationTaskIndividualAssessments,
    DescribeReplicationTasks,
    DescribeReplications,
    DescribeSchemas,
    DescribeTableStatistics,
    ExportMetadataModelAssessment,
    ImportCertificate,
    ListDataProviders,
    ListExtensionPacks,
    ListInstanceProfiles,
    ListMetadataModelAssessmentActionItems,
    ListMetadataModelAssessments,
    ListMetadataModelConversions,
    ListMetadataModelExports,
    ListMigrationProjects,
    ListTagsForResource,
    ModifyDataMigration,
    ModifyEndpoint,
    ModifyEventSubscription,
    ModifyFleetAdvisorCollector,
    ModifyFleetAdvisorCollectorStatuses,
    ModifyReplicationConfig,
    ModifyReplicationInstance,
    ModifyReplicationSubnetGroup,
    ModifyReplicationTask,
    MoveReplicationTask,
    RebootReplicationInstance,
    RefreshSchemas,
    ReloadReplicationTables,
    ReloadTables,
    RemoveTagsFromResource,
    RunFleetAdvisorLsaAnalysis,
    StartDataMigration,
    StartMetadataModelAssessment,
    StartMetadataModelConversion,
    StartMetadataModelExportAsScripts,
    StartMetadataModelExportToTarget,
    StartMetadataModelImport,
    StartRecommendations,
    StartReplication,
    StartReplicationTask,
    StartReplicationTaskAssessment,
    StartReplicationTaskAssessmentRun,
    StopDataMigration,
    StopReplication,
    StopReplicationTask,
    TestConnection,
    UpdateConversionConfiguration,
    UpdateDataProvider,
    UpdateInstanceProfile,
    UpdateMigrationProject,
    UpdateSubscriptionsToEventBridge,
    UploadFileMetadataList,
}
impl std::fmt::Display for DmsActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DmsActions::AddTagsToResource => write!(f, "dms:AddTagsToResource"),
            DmsActions::ApplyPendingMaintenanceAction => {
                write!(f, "dms:ApplyPendingMaintenanceAction")
            }
            DmsActions::AssociateExtensionPack => write!(f, "dms:AssociateExtensionPack"),
            DmsActions::BatchStartRecommendations => write!(f, "dms:BatchStartRecommendations"),
            DmsActions::CancelReplicationTaskAssessmentRun => {
                write!(f, "dms:CancelReplicationTaskAssessmentRun")
            }
            DmsActions::CreateDataMigration => write!(f, "dms:CreateDataMigration"),
            DmsActions::CreateDataProvider => write!(f, "dms:CreateDataProvider"),
            DmsActions::CreateEndpoint => write!(f, "dms:CreateEndpoint"),
            DmsActions::CreateEventSubscription => write!(f, "dms:CreateEventSubscription"),
            DmsActions::CreateFleetAdvisorCollector => write!(f, "dms:CreateFleetAdvisorCollector"),
            DmsActions::CreateInstanceProfile => write!(f, "dms:CreateInstanceProfile"),
            DmsActions::CreateMigrationProject => write!(f, "dms:CreateMigrationProject"),
            DmsActions::CreateReplicationConfig => write!(f, "dms:CreateReplicationConfig"),
            DmsActions::CreateReplicationInstance => write!(f, "dms:CreateReplicationInstance"),
            DmsActions::CreateReplicationSubnetGroup => {
                write!(f, "dms:CreateReplicationSubnetGroup")
            }
            DmsActions::CreateReplicationTask => write!(f, "dms:CreateReplicationTask"),
            DmsActions::DeleteCertificate => write!(f, "dms:DeleteCertificate"),
            DmsActions::DeleteConnection => write!(f, "dms:DeleteConnection"),
            DmsActions::DeleteDataMigration => write!(f, "dms:DeleteDataMigration"),
            DmsActions::DeleteDataProvider => write!(f, "dms:DeleteDataProvider"),
            DmsActions::DeleteEndpoint => write!(f, "dms:DeleteEndpoint"),
            DmsActions::DeleteEventSubscription => write!(f, "dms:DeleteEventSubscription"),
            DmsActions::DeleteFleetAdvisorCollector => write!(f, "dms:DeleteFleetAdvisorCollector"),
            DmsActions::DeleteFleetAdvisorDatabases => write!(f, "dms:DeleteFleetAdvisorDatabases"),
            DmsActions::DeleteInstanceProfile => write!(f, "dms:DeleteInstanceProfile"),
            DmsActions::DeleteMigrationProject => write!(f, "dms:DeleteMigrationProject"),
            DmsActions::DeleteReplicationConfig => write!(f, "dms:DeleteReplicationConfig"),
            DmsActions::DeleteReplicationInstance => write!(f, "dms:DeleteReplicationInstance"),
            DmsActions::DeleteReplicationSubnetGroup => {
                write!(f, "dms:DeleteReplicationSubnetGroup")
            }
            DmsActions::DeleteReplicationTask => write!(f, "dms:DeleteReplicationTask"),
            DmsActions::DeleteReplicationTaskAssessmentRun => {
                write!(f, "dms:DeleteReplicationTaskAssessmentRun")
            }
            DmsActions::DescribeAccountAttributes => write!(f, "dms:DescribeAccountAttributes"),
            DmsActions::DescribeApplicableIndividualAssessments => {
                write!(f, "dms:DescribeApplicableIndividualAssessments")
            }
            DmsActions::DescribeCertificates => write!(f, "dms:DescribeCertificates"),
            DmsActions::DescribeConnections => write!(f, "dms:DescribeConnections"),
            DmsActions::DescribeConversionConfiguration => {
                write!(f, "dms:DescribeConversionConfiguration")
            }
            DmsActions::DescribeDataMigrations => write!(f, "dms:DescribeDataMigrations"),
            DmsActions::DescribeEndpointSettings => write!(f, "dms:DescribeEndpointSettings"),
            DmsActions::DescribeEndpointTypes => write!(f, "dms:DescribeEndpointTypes"),
            DmsActions::DescribeEndpoints => write!(f, "dms:DescribeEndpoints"),
            DmsActions::DescribeEngineVersions => write!(f, "dms:DescribeEngineVersions"),
            DmsActions::DescribeEventCategories => write!(f, "dms:DescribeEventCategories"),
            DmsActions::DescribeEventSubscriptions => write!(f, "dms:DescribeEventSubscriptions"),
            DmsActions::DescribeEvents => write!(f, "dms:DescribeEvents"),
            DmsActions::DescribeFleetAdvisorCollectors => {
                write!(f, "dms:DescribeFleetAdvisorCollectors")
            }
            DmsActions::DescribeFleetAdvisorDatabases => {
                write!(f, "dms:DescribeFleetAdvisorDatabases")
            }
            DmsActions::DescribeFleetAdvisorLsaAnalysis => {
                write!(f, "dms:DescribeFleetAdvisorLsaAnalysis")
            }
            DmsActions::DescribeFleetAdvisorSchemaObjectSummary => {
                write!(f, "dms:DescribeFleetAdvisorSchemaObjectSummary")
            }
            DmsActions::DescribeFleetAdvisorSchemas => write!(f, "dms:DescribeFleetAdvisorSchemas"),
            DmsActions::DescribeMetadataModelImports => {
                write!(f, "dms:DescribeMetadataModelImports")
            }
            DmsActions::DescribeOrderableReplicationInstances => {
                write!(f, "dms:DescribeOrderableReplicationInstances")
            }
            DmsActions::DescribePendingMaintenanceActions => {
                write!(f, "dms:DescribePendingMaintenanceActions")
            }
            DmsActions::DescribeRecommendationLimitations => {
                write!(f, "dms:DescribeRecommendationLimitations")
            }
            DmsActions::DescribeRecommendations => write!(f, "dms:DescribeRecommendations"),
            DmsActions::DescribeRefreshSchemasStatus => {
                write!(f, "dms:DescribeRefreshSchemasStatus")
            }
            DmsActions::DescribeReplicationConfigs => write!(f, "dms:DescribeReplicationConfigs"),
            DmsActions::DescribeReplicationInstanceTaskLogs => {
                write!(f, "dms:DescribeReplicationInstanceTaskLogs")
            }
            DmsActions::DescribeReplicationInstances => {
                write!(f, "dms:DescribeReplicationInstances")
            }
            DmsActions::DescribeReplicationSubnetGroups => {
                write!(f, "dms:DescribeReplicationSubnetGroups")
            }
            DmsActions::DescribeReplicationTableStatistics => {
                write!(f, "dms:DescribeReplicationTableStatistics")
            }
            DmsActions::DescribeReplicationTaskAssessmentResults => {
                write!(f, "dms:DescribeReplicationTaskAssessmentResults")
            }
            DmsActions::DescribeReplicationTaskAssessmentRuns => {
                write!(f, "dms:DescribeReplicationTaskAssessmentRuns")
            }
            DmsActions::DescribeReplicationTaskIndividualAssessments => {
                write!(f, "dms:DescribeReplicationTaskIndividualAssessments")
            }
            DmsActions::DescribeReplicationTasks => write!(f, "dms:DescribeReplicationTasks"),
            DmsActions::DescribeReplications => write!(f, "dms:DescribeReplications"),
            DmsActions::DescribeSchemas => write!(f, "dms:DescribeSchemas"),
            DmsActions::DescribeTableStatistics => write!(f, "dms:DescribeTableStatistics"),
            DmsActions::ExportMetadataModelAssessment => {
                write!(f, "dms:ExportMetadataModelAssessment")
            }
            DmsActions::ImportCertificate => write!(f, "dms:ImportCertificate"),
            DmsActions::ListDataProviders => write!(f, "dms:ListDataProviders"),
            DmsActions::ListExtensionPacks => write!(f, "dms:ListExtensionPacks"),
            DmsActions::ListInstanceProfiles => write!(f, "dms:ListInstanceProfiles"),
            DmsActions::ListMetadataModelAssessmentActionItems => {
                write!(f, "dms:ListMetadataModelAssessmentActionItems")
            }
            DmsActions::ListMetadataModelAssessments => {
                write!(f, "dms:ListMetadataModelAssessments")
            }
            DmsActions::ListMetadataModelConversions => {
                write!(f, "dms:ListMetadataModelConversions")
            }
            DmsActions::ListMetadataModelExports => write!(f, "dms:ListMetadataModelExports"),
            DmsActions::ListMigrationProjects => write!(f, "dms:ListMigrationProjects"),
            DmsActions::ListTagsForResource => write!(f, "dms:ListTagsForResource"),
            DmsActions::ModifyDataMigration => write!(f, "dms:ModifyDataMigration"),
            DmsActions::ModifyEndpoint => write!(f, "dms:ModifyEndpoint"),
            DmsActions::ModifyEventSubscription => write!(f, "dms:ModifyEventSubscription"),
            DmsActions::ModifyFleetAdvisorCollector => write!(f, "dms:ModifyFleetAdvisorCollector"),
            DmsActions::ModifyFleetAdvisorCollectorStatuses => {
                write!(f, "dms:ModifyFleetAdvisorCollectorStatuses")
            }
            DmsActions::ModifyReplicationConfig => write!(f, "dms:ModifyReplicationConfig"),
            DmsActions::ModifyReplicationInstance => write!(f, "dms:ModifyReplicationInstance"),
            DmsActions::ModifyReplicationSubnetGroup => {
                write!(f, "dms:ModifyReplicationSubnetGroup")
            }
            DmsActions::ModifyReplicationTask => write!(f, "dms:ModifyReplicationTask"),
            DmsActions::MoveReplicationTask => write!(f, "dms:MoveReplicationTask"),
            DmsActions::RebootReplicationInstance => write!(f, "dms:RebootReplicationInstance"),
            DmsActions::RefreshSchemas => write!(f, "dms:RefreshSchemas"),
            DmsActions::ReloadReplicationTables => write!(f, "dms:ReloadReplicationTables"),
            DmsActions::ReloadTables => write!(f, "dms:ReloadTables"),
            DmsActions::RemoveTagsFromResource => write!(f, "dms:RemoveTagsFromResource"),
            DmsActions::RunFleetAdvisorLsaAnalysis => write!(f, "dms:RunFleetAdvisorLsaAnalysis"),
            DmsActions::StartDataMigration => write!(f, "dms:StartDataMigration"),
            DmsActions::StartMetadataModelAssessment => {
                write!(f, "dms:StartMetadataModelAssessment")
            }
            DmsActions::StartMetadataModelConversion => {
                write!(f, "dms:StartMetadataModelConversion")
            }
            DmsActions::StartMetadataModelExportAsScripts => {
                write!(f, "dms:StartMetadataModelExportAsScripts")
            }
            DmsActions::StartMetadataModelExportToTarget => {
                write!(f, "dms:StartMetadataModelExportToTarget")
            }
            DmsActions::StartMetadataModelImport => write!(f, "dms:StartMetadataModelImport"),
            DmsActions::StartRecommendations => write!(f, "dms:StartRecommendations"),
            DmsActions::StartReplication => write!(f, "dms:StartReplication"),
            DmsActions::StartReplicationTask => write!(f, "dms:StartReplicationTask"),
            DmsActions::StartReplicationTaskAssessment => {
                write!(f, "dms:StartReplicationTaskAssessment")
            }
            DmsActions::StartReplicationTaskAssessmentRun => {
                write!(f, "dms:StartReplicationTaskAssessmentRun")
            }
            DmsActions::StopDataMigration => write!(f, "dms:StopDataMigration"),
            DmsActions::StopReplication => write!(f, "dms:StopReplication"),
            DmsActions::StopReplicationTask => write!(f, "dms:StopReplicationTask"),
            DmsActions::TestConnection => write!(f, "dms:TestConnection"),
            DmsActions::UpdateConversionConfiguration => {
                write!(f, "dms:UpdateConversionConfiguration")
            }
            DmsActions::UpdateDataProvider => write!(f, "dms:UpdateDataProvider"),
            DmsActions::UpdateInstanceProfile => write!(f, "dms:UpdateInstanceProfile"),
            DmsActions::UpdateMigrationProject => write!(f, "dms:UpdateMigrationProject"),
            DmsActions::UpdateSubscriptionsToEventBridge => {
                write!(f, "dms:UpdateSubscriptionsToEventBridge")
            }
            DmsActions::UploadFileMetadataList => write!(f, "dms:UploadFileMetadataList"),
        }
    }
}
