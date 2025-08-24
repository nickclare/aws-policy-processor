// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum GlueActions {
    AuthorizeInboundIntegration,
    BatchCreatePartition,
    BatchDeleteConnection,
    BatchDeletePartition,
    BatchDeleteTable,
    BatchDeleteTableVersion,
    BatchGetBlueprints,
    BatchGetCrawlers,
    BatchGetCustomEntityTypes,
    BatchGetDevEndpoints,
    BatchGetJobs,
    BatchGetPartition,
    BatchGetStageFiles,
    BatchGetTableOptimizer,
    BatchGetTriggers,
    BatchGetWorkflows,
    BatchStopJobRun,
    BatchUpdatePartition,
    CancelDataQualityRuleRecommendationRun,
    CancelDataQualityRulesetEvaluationRun,
    CancelMlTaskRun,
    CancelStatement,
    CheckSchemaVersionValidity,
    CreateBlueprint,
    CreateCatalog,
    CreateClassifier,
    CreateColumnStatisticsTaskSettings,
    CreateConnection,
    CreateCrawler,
    CreateCustomEntityType,
    CreateDataQualityRuleset,
    CreateDatabase,
    CreateDevEndpoint,
    CreateGlueIdentityCenterConfiguration,
    CreateInboundIntegration,
    CreateIntegration,
    CreateIntegrationResourceProperty,
    CreateIntegrationTableProperties,
    CreateJob,
    CreateMlTransform,
    CreatePartition,
    CreatePartitionIndex,
    CreateRegistry,
    CreateSchema,
    CreateScript,
    CreateSecurityConfiguration,
    CreateSession,
    CreateTable,
    CreateTableOptimizer,
    CreateTrigger,
    CreateUsageProfile,
    CreateUserDefinedFunction,
    CreateWorkflow,
    DeleteBlueprint,
    DeleteCatalog,
    DeleteClassifier,
    DeleteColumnStatisticsForPartition,
    DeleteColumnStatisticsForTable,
    DeleteColumnStatisticsTaskSettings,
    DeleteConnection,
    DeleteCrawler,
    DeleteCustomEntityType,
    DeleteDataQualityRuleset,
    DeleteDatabase,
    DeleteDevEndpoint,
    DeleteGlueIdentityCenterConfiguration,
    DeleteIntegration,
    DeleteIntegrationTableProperties,
    DeleteJob,
    DeleteMlTransform,
    DeletePartition,
    DeletePartitionIndex,
    DeleteRegistry,
    DeleteResourcePolicy,
    DeleteSchema,
    DeleteSchemaVersions,
    DeleteSecurityConfiguration,
    DeleteSession,
    DeleteTable,
    DeleteTableOptimizer,
    DeleteTableVersion,
    DeleteTrigger,
    DeleteUsageProfile,
    DeleteUserDefinedFunction,
    DeleteWorkflow,
    DeregisterDataPreview,
    DescribeConnectionType,
    DescribeEntity,
    DescribeInboundIntegrations,
    DescribeIntegrations,
    GetBlueprint,
    GetBlueprintRun,
    GetBlueprintRuns,
    GetCatalog,
    GetCatalogImportStatus,
    GetCatalogs,
    GetClassifier,
    GetClassifiers,
    GetColumnStatisticsForPartition,
    GetColumnStatisticsForTable,
    GetColumnStatisticsTaskRun,
    GetColumnStatisticsTaskRuns,
    GetColumnStatisticsTaskSettings,
    GetCompletion,
    GetConnection,
    GetConnections,
    GetCrawler,
    GetCrawlerMetrics,
    GetCrawlers,
    GetCustomEntityType,
    GetDashboardUrl,
    GetDataCatalogEncryptionSettings,
    GetDataPreviewStatement,
    GetDataQualityModel,
    GetDataQualityModelResult,
    GetDataQualityResult,
    GetDataQualityRuleRecommendationRun,
    GetDataQualityRuleset,
    GetDataQualityRulesetEvaluationRun,
    GetDatabase,
    GetDatabases,
    GetDataflowGraph,
    GetDevEndpoint,
    GetDevEndpoints,
    GetEntityRecords,
    GetEnvironment,
    GetExecutors,
    GetExecutorsThreads,
    GetGeneratedCode,
    GetGlueIdentityCenterConfiguration,
    GetIntegrationResourceProperty,
    GetIntegrationTableProperties,
    GetJob,
    GetJobBookmark,
    GetJobRun,
    GetJobRuns,
    GetJobUpgradeAnalysis,
    GetJobs,
    GetLogParsingStatus,
    GetMapping,
    GetMlTaskRun,
    GetMlTaskRuns,
    GetMlTransform,
    GetMlTransforms,
    GetNotebookInstanceStatus,
    GetPartition,
    GetPartitionIndexes,
    GetPartitions,
    GetPlan,
    GetQueries,
    GetQuery,
    GetRecipeAction,
    GetRegistry,
    GetResourcePolicies,
    GetResourcePolicy,
    GetSchema,
    GetSchemaByDefinition,
    GetSchemaVersion,
    GetSchemaVersionsDiff,
    GetSecurityConfiguration,
    GetSecurityConfigurations,
    GetSession,
    GetStage,
    GetStageAttempt,
    GetStageAttemptTaskList,
    GetStageAttemptTaskSummary,
    GetStageFiles,
    GetStages,
    GetStatement,
    GetStorage,
    GetStorageUnit,
    GetTable,
    GetTableOptimizer,
    GetTableVersion,
    GetTableVersions,
    GetTables,
    GetTags,
    GetTrigger,
    GetTriggers,
    GetUsageProfile,
    GetUserDefinedFunction,
    GetUserDefinedFunctions,
    GetWorkflow,
    GetWorkflowRun,
    GetWorkflowRunProperties,
    GetWorkflowRuns,
    GlueNotebookAuthorize,
    GlueNotebookRefreshCredentials,
    ImportCatalogToGlue,
    ListBlueprints,
    ListColumnStatisticsTaskRuns,
    ListConnectionTypes,
    ListCrawlers,
    ListCrawls,
    ListCustomEntityTypes,
    ListDataQualityResults,
    ListDataQualityRuleRecommendationRuns,
    ListDataQualityRulesetEvaluationRuns,
    ListDataQualityRulesets,
    ListDevEndpoints,
    ListEntities,
    ListJobUpgradeAnalyses,
    ListJobs,
    ListMlTransforms,
    ListRegistries,
    ListSchemaVersions,
    ListSchemas,
    ListSessions,
    ListStatements,
    ListTableOptimizerRuns,
    ListTriggers,
    ListUsageProfiles,
    ListWorkflows,
    ModifyIntegration,
    NotifyEvent,
    PassConnection,
    PublishDataQuality,
    PutDataCatalogEncryptionSettings,
    PutDataQualityProfileAnnotation,
    PutDataQualityStatisticAnnotation,
    PutResourcePolicy,
    PutSchemaVersionMetadata,
    PutWorkflowRunProperties,
    QuerySchemaVersionMetadata,
    RefreshOAuth2Tokens,
    RegisterSchemaVersion,
    RemoveSchemaVersionMetadata,
    RequestLogParsing,
    ResetJobBookmark,
    ResumeWorkflowRun,
    RunDataPreviewStatement,
    RunStatement,
    SearchTables,
    SendFeedback,
    SendRecipeAction,
    StartBlueprintRun,
    StartColumnStatisticsTaskRun,
    StartColumnStatisticsTaskRunSchedule,
    StartCompletion,
    StartCrawler,
    StartCrawlerSchedule,
    StartDataQualityRuleRecommendationRun,
    StartDataQualityRulesetEvaluationRun,
    StartExportLabelsTaskRun,
    StartImportLabelsTaskRun,
    StartJobRun,
    StartJobUpgradeAnalysis,
    StartMlEvaluationTaskRun,
    StartMlLabelingSetGenerationTaskRun,
    StartNotebook,
    StartTrigger,
    StartWorkflowRun,
    StopColumnStatisticsTaskRun,
    StopColumnStatisticsTaskRunSchedule,
    StopCrawler,
    StopCrawlerSchedule,
    StopJobUpgradeAnalysis,
    StopSession,
    StopTrigger,
    StopWorkflowRun,
    TagResource,
    TerminateNotebook,
    TestConnection,
    UntagResource,
    UpdateBlueprint,
    UpdateCatalog,
    UpdateClassifier,
    UpdateColumnStatisticsForPartition,
    UpdateColumnStatisticsForTable,
    UpdateColumnStatisticsTaskSettings,
    UpdateConnection,
    UpdateCrawler,
    UpdateCrawlerSchedule,
    UpdateDataQualityRuleset,
    UpdateDatabase,
    UpdateDevEndpoint,
    UpdateGlueIdentityCenterConfiguration,
    UpdateIntegrationResourceProperty,
    UpdateIntegrationTableProperties,
    UpdateJob,
    UpdateJobFromSourceControl,
    UpdateMlTransform,
    UpdatePartition,
    UpdateRegistry,
    UpdateSchema,
    UpdateSourceControlFromJob,
    UpdateTable,
    UpdateTableOptimizer,
    UpdateTrigger,
    UpdateUsageProfile,
    UpdateUserDefinedFunction,
    UpdateWorkflow,
    UpgradeJob,
    UseGlueStudio,
    UseMlTransforms,
}
impl std::fmt::Display for GlueActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GlueActions::AuthorizeInboundIntegration => {
                write!(f, "glue:AuthorizeInboundIntegration")
            }
            GlueActions::BatchCreatePartition => write!(f, "glue:BatchCreatePartition"),
            GlueActions::BatchDeleteConnection => write!(f, "glue:BatchDeleteConnection"),
            GlueActions::BatchDeletePartition => write!(f, "glue:BatchDeletePartition"),
            GlueActions::BatchDeleteTable => write!(f, "glue:BatchDeleteTable"),
            GlueActions::BatchDeleteTableVersion => write!(f, "glue:BatchDeleteTableVersion"),
            GlueActions::BatchGetBlueprints => write!(f, "glue:BatchGetBlueprints"),
            GlueActions::BatchGetCrawlers => write!(f, "glue:BatchGetCrawlers"),
            GlueActions::BatchGetCustomEntityTypes => write!(f, "glue:BatchGetCustomEntityTypes"),
            GlueActions::BatchGetDevEndpoints => write!(f, "glue:BatchGetDevEndpoints"),
            GlueActions::BatchGetJobs => write!(f, "glue:BatchGetJobs"),
            GlueActions::BatchGetPartition => write!(f, "glue:BatchGetPartition"),
            GlueActions::BatchGetStageFiles => write!(f, "glue:BatchGetStageFiles"),
            GlueActions::BatchGetTableOptimizer => write!(f, "glue:BatchGetTableOptimizer"),
            GlueActions::BatchGetTriggers => write!(f, "glue:BatchGetTriggers"),
            GlueActions::BatchGetWorkflows => write!(f, "glue:BatchGetWorkflows"),
            GlueActions::BatchStopJobRun => write!(f, "glue:BatchStopJobRun"),
            GlueActions::BatchUpdatePartition => write!(f, "glue:BatchUpdatePartition"),
            GlueActions::CancelDataQualityRuleRecommendationRun => {
                write!(f, "glue:CancelDataQualityRuleRecommendationRun")
            }
            GlueActions::CancelDataQualityRulesetEvaluationRun => {
                write!(f, "glue:CancelDataQualityRulesetEvaluationRun")
            }
            GlueActions::CancelMlTaskRun => write!(f, "glue:CancelMLTaskRun"),
            GlueActions::CancelStatement => write!(f, "glue:CancelStatement"),
            GlueActions::CheckSchemaVersionValidity => write!(f, "glue:CheckSchemaVersionValidity"),
            GlueActions::CreateBlueprint => write!(f, "glue:CreateBlueprint"),
            GlueActions::CreateCatalog => write!(f, "glue:CreateCatalog"),
            GlueActions::CreateClassifier => write!(f, "glue:CreateClassifier"),
            GlueActions::CreateColumnStatisticsTaskSettings => {
                write!(f, "glue:CreateColumnStatisticsTaskSettings")
            }
            GlueActions::CreateConnection => write!(f, "glue:CreateConnection"),
            GlueActions::CreateCrawler => write!(f, "glue:CreateCrawler"),
            GlueActions::CreateCustomEntityType => write!(f, "glue:CreateCustomEntityType"),
            GlueActions::CreateDataQualityRuleset => write!(f, "glue:CreateDataQualityRuleset"),
            GlueActions::CreateDatabase => write!(f, "glue:CreateDatabase"),
            GlueActions::CreateDevEndpoint => write!(f, "glue:CreateDevEndpoint"),
            GlueActions::CreateGlueIdentityCenterConfiguration => {
                write!(f, "glue:CreateGlueIdentityCenterConfiguration")
            }
            GlueActions::CreateInboundIntegration => write!(f, "glue:CreateInboundIntegration"),
            GlueActions::CreateIntegration => write!(f, "glue:CreateIntegration"),
            GlueActions::CreateIntegrationResourceProperty => {
                write!(f, "glue:CreateIntegrationResourceProperty")
            }
            GlueActions::CreateIntegrationTableProperties => {
                write!(f, "glue:CreateIntegrationTableProperties")
            }
            GlueActions::CreateJob => write!(f, "glue:CreateJob"),
            GlueActions::CreateMlTransform => write!(f, "glue:CreateMLTransform"),
            GlueActions::CreatePartition => write!(f, "glue:CreatePartition"),
            GlueActions::CreatePartitionIndex => write!(f, "glue:CreatePartitionIndex"),
            GlueActions::CreateRegistry => write!(f, "glue:CreateRegistry"),
            GlueActions::CreateSchema => write!(f, "glue:CreateSchema"),
            GlueActions::CreateScript => write!(f, "glue:CreateScript"),
            GlueActions::CreateSecurityConfiguration => {
                write!(f, "glue:CreateSecurityConfiguration")
            }
            GlueActions::CreateSession => write!(f, "glue:CreateSession"),
            GlueActions::CreateTable => write!(f, "glue:CreateTable"),
            GlueActions::CreateTableOptimizer => write!(f, "glue:CreateTableOptimizer"),
            GlueActions::CreateTrigger => write!(f, "glue:CreateTrigger"),
            GlueActions::CreateUsageProfile => write!(f, "glue:CreateUsageProfile"),
            GlueActions::CreateUserDefinedFunction => write!(f, "glue:CreateUserDefinedFunction"),
            GlueActions::CreateWorkflow => write!(f, "glue:CreateWorkflow"),
            GlueActions::DeleteBlueprint => write!(f, "glue:DeleteBlueprint"),
            GlueActions::DeleteCatalog => write!(f, "glue:DeleteCatalog"),
            GlueActions::DeleteClassifier => write!(f, "glue:DeleteClassifier"),
            GlueActions::DeleteColumnStatisticsForPartition => {
                write!(f, "glue:DeleteColumnStatisticsForPartition")
            }
            GlueActions::DeleteColumnStatisticsForTable => {
                write!(f, "glue:DeleteColumnStatisticsForTable")
            }
            GlueActions::DeleteColumnStatisticsTaskSettings => {
                write!(f, "glue:DeleteColumnStatisticsTaskSettings")
            }
            GlueActions::DeleteConnection => write!(f, "glue:DeleteConnection"),
            GlueActions::DeleteCrawler => write!(f, "glue:DeleteCrawler"),
            GlueActions::DeleteCustomEntityType => write!(f, "glue:DeleteCustomEntityType"),
            GlueActions::DeleteDataQualityRuleset => write!(f, "glue:DeleteDataQualityRuleset"),
            GlueActions::DeleteDatabase => write!(f, "glue:DeleteDatabase"),
            GlueActions::DeleteDevEndpoint => write!(f, "glue:DeleteDevEndpoint"),
            GlueActions::DeleteGlueIdentityCenterConfiguration => {
                write!(f, "glue:DeleteGlueIdentityCenterConfiguration")
            }
            GlueActions::DeleteIntegration => write!(f, "glue:DeleteIntegration"),
            GlueActions::DeleteIntegrationTableProperties => {
                write!(f, "glue:DeleteIntegrationTableProperties")
            }
            GlueActions::DeleteJob => write!(f, "glue:DeleteJob"),
            GlueActions::DeleteMlTransform => write!(f, "glue:DeleteMLTransform"),
            GlueActions::DeletePartition => write!(f, "glue:DeletePartition"),
            GlueActions::DeletePartitionIndex => write!(f, "glue:DeletePartitionIndex"),
            GlueActions::DeleteRegistry => write!(f, "glue:DeleteRegistry"),
            GlueActions::DeleteResourcePolicy => write!(f, "glue:DeleteResourcePolicy"),
            GlueActions::DeleteSchema => write!(f, "glue:DeleteSchema"),
            GlueActions::DeleteSchemaVersions => write!(f, "glue:DeleteSchemaVersions"),
            GlueActions::DeleteSecurityConfiguration => {
                write!(f, "glue:DeleteSecurityConfiguration")
            }
            GlueActions::DeleteSession => write!(f, "glue:DeleteSession"),
            GlueActions::DeleteTable => write!(f, "glue:DeleteTable"),
            GlueActions::DeleteTableOptimizer => write!(f, "glue:DeleteTableOptimizer"),
            GlueActions::DeleteTableVersion => write!(f, "glue:DeleteTableVersion"),
            GlueActions::DeleteTrigger => write!(f, "glue:DeleteTrigger"),
            GlueActions::DeleteUsageProfile => write!(f, "glue:DeleteUsageProfile"),
            GlueActions::DeleteUserDefinedFunction => write!(f, "glue:DeleteUserDefinedFunction"),
            GlueActions::DeleteWorkflow => write!(f, "glue:DeleteWorkflow"),
            GlueActions::DeregisterDataPreview => write!(f, "glue:DeregisterDataPreview"),
            GlueActions::DescribeConnectionType => write!(f, "glue:DescribeConnectionType"),
            GlueActions::DescribeEntity => write!(f, "glue:DescribeEntity"),
            GlueActions::DescribeInboundIntegrations => {
                write!(f, "glue:DescribeInboundIntegrations")
            }
            GlueActions::DescribeIntegrations => write!(f, "glue:DescribeIntegrations"),
            GlueActions::GetBlueprint => write!(f, "glue:GetBlueprint"),
            GlueActions::GetBlueprintRun => write!(f, "glue:GetBlueprintRun"),
            GlueActions::GetBlueprintRuns => write!(f, "glue:GetBlueprintRuns"),
            GlueActions::GetCatalog => write!(f, "glue:GetCatalog"),
            GlueActions::GetCatalogImportStatus => write!(f, "glue:GetCatalogImportStatus"),
            GlueActions::GetCatalogs => write!(f, "glue:GetCatalogs"),
            GlueActions::GetClassifier => write!(f, "glue:GetClassifier"),
            GlueActions::GetClassifiers => write!(f, "glue:GetClassifiers"),
            GlueActions::GetColumnStatisticsForPartition => {
                write!(f, "glue:GetColumnStatisticsForPartition")
            }
            GlueActions::GetColumnStatisticsForTable => {
                write!(f, "glue:GetColumnStatisticsForTable")
            }
            GlueActions::GetColumnStatisticsTaskRun => write!(f, "glue:GetColumnStatisticsTaskRun"),
            GlueActions::GetColumnStatisticsTaskRuns => {
                write!(f, "glue:GetColumnStatisticsTaskRuns")
            }
            GlueActions::GetColumnStatisticsTaskSettings => {
                write!(f, "glue:GetColumnStatisticsTaskSettings")
            }
            GlueActions::GetCompletion => write!(f, "glue:GetCompletion"),
            GlueActions::GetConnection => write!(f, "glue:GetConnection"),
            GlueActions::GetConnections => write!(f, "glue:GetConnections"),
            GlueActions::GetCrawler => write!(f, "glue:GetCrawler"),
            GlueActions::GetCrawlerMetrics => write!(f, "glue:GetCrawlerMetrics"),
            GlueActions::GetCrawlers => write!(f, "glue:GetCrawlers"),
            GlueActions::GetCustomEntityType => write!(f, "glue:GetCustomEntityType"),
            GlueActions::GetDashboardUrl => write!(f, "glue:GetDashboardUrl"),
            GlueActions::GetDataCatalogEncryptionSettings => {
                write!(f, "glue:GetDataCatalogEncryptionSettings")
            }
            GlueActions::GetDataPreviewStatement => write!(f, "glue:GetDataPreviewStatement"),
            GlueActions::GetDataQualityModel => write!(f, "glue:GetDataQualityModel"),
            GlueActions::GetDataQualityModelResult => write!(f, "glue:GetDataQualityModelResult"),
            GlueActions::GetDataQualityResult => write!(f, "glue:GetDataQualityResult"),
            GlueActions::GetDataQualityRuleRecommendationRun => {
                write!(f, "glue:GetDataQualityRuleRecommendationRun")
            }
            GlueActions::GetDataQualityRuleset => write!(f, "glue:GetDataQualityRuleset"),
            GlueActions::GetDataQualityRulesetEvaluationRun => {
                write!(f, "glue:GetDataQualityRulesetEvaluationRun")
            }
            GlueActions::GetDatabase => write!(f, "glue:GetDatabase"),
            GlueActions::GetDatabases => write!(f, "glue:GetDatabases"),
            GlueActions::GetDataflowGraph => write!(f, "glue:GetDataflowGraph"),
            GlueActions::GetDevEndpoint => write!(f, "glue:GetDevEndpoint"),
            GlueActions::GetDevEndpoints => write!(f, "glue:GetDevEndpoints"),
            GlueActions::GetEntityRecords => write!(f, "glue:GetEntityRecords"),
            GlueActions::GetEnvironment => write!(f, "glue:GetEnvironment"),
            GlueActions::GetExecutors => write!(f, "glue:GetExecutors"),
            GlueActions::GetExecutorsThreads => write!(f, "glue:GetExecutorsThreads"),
            GlueActions::GetGeneratedCode => write!(f, "glue:GetGeneratedCode"),
            GlueActions::GetGlueIdentityCenterConfiguration => {
                write!(f, "glue:GetGlueIdentityCenterConfiguration")
            }
            GlueActions::GetIntegrationResourceProperty => {
                write!(f, "glue:GetIntegrationResourceProperty")
            }
            GlueActions::GetIntegrationTableProperties => {
                write!(f, "glue:GetIntegrationTableProperties")
            }
            GlueActions::GetJob => write!(f, "glue:GetJob"),
            GlueActions::GetJobBookmark => write!(f, "glue:GetJobBookmark"),
            GlueActions::GetJobRun => write!(f, "glue:GetJobRun"),
            GlueActions::GetJobRuns => write!(f, "glue:GetJobRuns"),
            GlueActions::GetJobUpgradeAnalysis => write!(f, "glue:GetJobUpgradeAnalysis"),
            GlueActions::GetJobs => write!(f, "glue:GetJobs"),
            GlueActions::GetLogParsingStatus => write!(f, "glue:GetLogParsingStatus"),
            GlueActions::GetMapping => write!(f, "glue:GetMapping"),
            GlueActions::GetMlTaskRun => write!(f, "glue:GetMLTaskRun"),
            GlueActions::GetMlTaskRuns => write!(f, "glue:GetMLTaskRuns"),
            GlueActions::GetMlTransform => write!(f, "glue:GetMLTransform"),
            GlueActions::GetMlTransforms => write!(f, "glue:GetMLTransforms"),
            GlueActions::GetNotebookInstanceStatus => write!(f, "glue:GetNotebookInstanceStatus"),
            GlueActions::GetPartition => write!(f, "glue:GetPartition"),
            GlueActions::GetPartitionIndexes => write!(f, "glue:GetPartitionIndexes"),
            GlueActions::GetPartitions => write!(f, "glue:GetPartitions"),
            GlueActions::GetPlan => write!(f, "glue:GetPlan"),
            GlueActions::GetQueries => write!(f, "glue:GetQueries"),
            GlueActions::GetQuery => write!(f, "glue:GetQuery"),
            GlueActions::GetRecipeAction => write!(f, "glue:GetRecipeAction"),
            GlueActions::GetRegistry => write!(f, "glue:GetRegistry"),
            GlueActions::GetResourcePolicies => write!(f, "glue:GetResourcePolicies"),
            GlueActions::GetResourcePolicy => write!(f, "glue:GetResourcePolicy"),
            GlueActions::GetSchema => write!(f, "glue:GetSchema"),
            GlueActions::GetSchemaByDefinition => write!(f, "glue:GetSchemaByDefinition"),
            GlueActions::GetSchemaVersion => write!(f, "glue:GetSchemaVersion"),
            GlueActions::GetSchemaVersionsDiff => write!(f, "glue:GetSchemaVersionsDiff"),
            GlueActions::GetSecurityConfiguration => write!(f, "glue:GetSecurityConfiguration"),
            GlueActions::GetSecurityConfigurations => write!(f, "glue:GetSecurityConfigurations"),
            GlueActions::GetSession => write!(f, "glue:GetSession"),
            GlueActions::GetStage => write!(f, "glue:GetStage"),
            GlueActions::GetStageAttempt => write!(f, "glue:GetStageAttempt"),
            GlueActions::GetStageAttemptTaskList => write!(f, "glue:GetStageAttemptTaskList"),
            GlueActions::GetStageAttemptTaskSummary => write!(f, "glue:GetStageAttemptTaskSummary"),
            GlueActions::GetStageFiles => write!(f, "glue:GetStageFiles"),
            GlueActions::GetStages => write!(f, "glue:GetStages"),
            GlueActions::GetStatement => write!(f, "glue:GetStatement"),
            GlueActions::GetStorage => write!(f, "glue:GetStorage"),
            GlueActions::GetStorageUnit => write!(f, "glue:GetStorageUnit"),
            GlueActions::GetTable => write!(f, "glue:GetTable"),
            GlueActions::GetTableOptimizer => write!(f, "glue:GetTableOptimizer"),
            GlueActions::GetTableVersion => write!(f, "glue:GetTableVersion"),
            GlueActions::GetTableVersions => write!(f, "glue:GetTableVersions"),
            GlueActions::GetTables => write!(f, "glue:GetTables"),
            GlueActions::GetTags => write!(f, "glue:GetTags"),
            GlueActions::GetTrigger => write!(f, "glue:GetTrigger"),
            GlueActions::GetTriggers => write!(f, "glue:GetTriggers"),
            GlueActions::GetUsageProfile => write!(f, "glue:GetUsageProfile"),
            GlueActions::GetUserDefinedFunction => write!(f, "glue:GetUserDefinedFunction"),
            GlueActions::GetUserDefinedFunctions => write!(f, "glue:GetUserDefinedFunctions"),
            GlueActions::GetWorkflow => write!(f, "glue:GetWorkflow"),
            GlueActions::GetWorkflowRun => write!(f, "glue:GetWorkflowRun"),
            GlueActions::GetWorkflowRunProperties => write!(f, "glue:GetWorkflowRunProperties"),
            GlueActions::GetWorkflowRuns => write!(f, "glue:GetWorkflowRuns"),
            GlueActions::GlueNotebookAuthorize => write!(f, "glue:GlueNotebookAuthorize"),
            GlueActions::GlueNotebookRefreshCredentials => {
                write!(f, "glue:GlueNotebookRefreshCredentials")
            }
            GlueActions::ImportCatalogToGlue => write!(f, "glue:ImportCatalogToGlue"),
            GlueActions::ListBlueprints => write!(f, "glue:ListBlueprints"),
            GlueActions::ListColumnStatisticsTaskRuns => {
                write!(f, "glue:ListColumnStatisticsTaskRuns")
            }
            GlueActions::ListConnectionTypes => write!(f, "glue:ListConnectionTypes"),
            GlueActions::ListCrawlers => write!(f, "glue:ListCrawlers"),
            GlueActions::ListCrawls => write!(f, "glue:ListCrawls"),
            GlueActions::ListCustomEntityTypes => write!(f, "glue:ListCustomEntityTypes"),
            GlueActions::ListDataQualityResults => write!(f, "glue:ListDataQualityResults"),
            GlueActions::ListDataQualityRuleRecommendationRuns => {
                write!(f, "glue:ListDataQualityRuleRecommendationRuns")
            }
            GlueActions::ListDataQualityRulesetEvaluationRuns => {
                write!(f, "glue:ListDataQualityRulesetEvaluationRuns")
            }
            GlueActions::ListDataQualityRulesets => write!(f, "glue:ListDataQualityRulesets"),
            GlueActions::ListDevEndpoints => write!(f, "glue:ListDevEndpoints"),
            GlueActions::ListEntities => write!(f, "glue:ListEntities"),
            GlueActions::ListJobUpgradeAnalyses => write!(f, "glue:ListJobUpgradeAnalyses"),
            GlueActions::ListJobs => write!(f, "glue:ListJobs"),
            GlueActions::ListMlTransforms => write!(f, "glue:ListMLTransforms"),
            GlueActions::ListRegistries => write!(f, "glue:ListRegistries"),
            GlueActions::ListSchemaVersions => write!(f, "glue:ListSchemaVersions"),
            GlueActions::ListSchemas => write!(f, "glue:ListSchemas"),
            GlueActions::ListSessions => write!(f, "glue:ListSessions"),
            GlueActions::ListStatements => write!(f, "glue:ListStatements"),
            GlueActions::ListTableOptimizerRuns => write!(f, "glue:ListTableOptimizerRuns"),
            GlueActions::ListTriggers => write!(f, "glue:ListTriggers"),
            GlueActions::ListUsageProfiles => write!(f, "glue:ListUsageProfiles"),
            GlueActions::ListWorkflows => write!(f, "glue:ListWorkflows"),
            GlueActions::ModifyIntegration => write!(f, "glue:ModifyIntegration"),
            GlueActions::NotifyEvent => write!(f, "glue:NotifyEvent"),
            GlueActions::PassConnection => write!(f, "glue:PassConnection"),
            GlueActions::PublishDataQuality => write!(f, "glue:PublishDataQuality"),
            GlueActions::PutDataCatalogEncryptionSettings => {
                write!(f, "glue:PutDataCatalogEncryptionSettings")
            }
            GlueActions::PutDataQualityProfileAnnotation => {
                write!(f, "glue:PutDataQualityProfileAnnotation")
            }
            GlueActions::PutDataQualityStatisticAnnotation => {
                write!(f, "glue:PutDataQualityStatisticAnnotation")
            }
            GlueActions::PutResourcePolicy => write!(f, "glue:PutResourcePolicy"),
            GlueActions::PutSchemaVersionMetadata => write!(f, "glue:PutSchemaVersionMetadata"),
            GlueActions::PutWorkflowRunProperties => write!(f, "glue:PutWorkflowRunProperties"),
            GlueActions::QuerySchemaVersionMetadata => write!(f, "glue:QuerySchemaVersionMetadata"),
            GlueActions::RefreshOAuth2Tokens => write!(f, "glue:RefreshOAuth2Tokens"),
            GlueActions::RegisterSchemaVersion => write!(f, "glue:RegisterSchemaVersion"),
            GlueActions::RemoveSchemaVersionMetadata => {
                write!(f, "glue:RemoveSchemaVersionMetadata")
            }
            GlueActions::RequestLogParsing => write!(f, "glue:RequestLogParsing"),
            GlueActions::ResetJobBookmark => write!(f, "glue:ResetJobBookmark"),
            GlueActions::ResumeWorkflowRun => write!(f, "glue:ResumeWorkflowRun"),
            GlueActions::RunDataPreviewStatement => write!(f, "glue:RunDataPreviewStatement"),
            GlueActions::RunStatement => write!(f, "glue:RunStatement"),
            GlueActions::SearchTables => write!(f, "glue:SearchTables"),
            GlueActions::SendFeedback => write!(f, "glue:SendFeedback"),
            GlueActions::SendRecipeAction => write!(f, "glue:SendRecipeAction"),
            GlueActions::StartBlueprintRun => write!(f, "glue:StartBlueprintRun"),
            GlueActions::StartColumnStatisticsTaskRun => {
                write!(f, "glue:StartColumnStatisticsTaskRun")
            }
            GlueActions::StartColumnStatisticsTaskRunSchedule => {
                write!(f, "glue:StartColumnStatisticsTaskRunSchedule")
            }
            GlueActions::StartCompletion => write!(f, "glue:StartCompletion"),
            GlueActions::StartCrawler => write!(f, "glue:StartCrawler"),
            GlueActions::StartCrawlerSchedule => write!(f, "glue:StartCrawlerSchedule"),
            GlueActions::StartDataQualityRuleRecommendationRun => {
                write!(f, "glue:StartDataQualityRuleRecommendationRun")
            }
            GlueActions::StartDataQualityRulesetEvaluationRun => {
                write!(f, "glue:StartDataQualityRulesetEvaluationRun")
            }
            GlueActions::StartExportLabelsTaskRun => write!(f, "glue:StartExportLabelsTaskRun"),
            GlueActions::StartImportLabelsTaskRun => write!(f, "glue:StartImportLabelsTaskRun"),
            GlueActions::StartJobRun => write!(f, "glue:StartJobRun"),
            GlueActions::StartJobUpgradeAnalysis => write!(f, "glue:StartJobUpgradeAnalysis"),
            GlueActions::StartMlEvaluationTaskRun => write!(f, "glue:StartMLEvaluationTaskRun"),
            GlueActions::StartMlLabelingSetGenerationTaskRun => {
                write!(f, "glue:StartMLLabelingSetGenerationTaskRun")
            }
            GlueActions::StartNotebook => write!(f, "glue:StartNotebook"),
            GlueActions::StartTrigger => write!(f, "glue:StartTrigger"),
            GlueActions::StartWorkflowRun => write!(f, "glue:StartWorkflowRun"),
            GlueActions::StopColumnStatisticsTaskRun => {
                write!(f, "glue:StopColumnStatisticsTaskRun")
            }
            GlueActions::StopColumnStatisticsTaskRunSchedule => {
                write!(f, "glue:StopColumnStatisticsTaskRunSchedule")
            }
            GlueActions::StopCrawler => write!(f, "glue:StopCrawler"),
            GlueActions::StopCrawlerSchedule => write!(f, "glue:StopCrawlerSchedule"),
            GlueActions::StopJobUpgradeAnalysis => write!(f, "glue:StopJobUpgradeAnalysis"),
            GlueActions::StopSession => write!(f, "glue:StopSession"),
            GlueActions::StopTrigger => write!(f, "glue:StopTrigger"),
            GlueActions::StopWorkflowRun => write!(f, "glue:StopWorkflowRun"),
            GlueActions::TagResource => write!(f, "glue:TagResource"),
            GlueActions::TerminateNotebook => write!(f, "glue:TerminateNotebook"),
            GlueActions::TestConnection => write!(f, "glue:TestConnection"),
            GlueActions::UntagResource => write!(f, "glue:UntagResource"),
            GlueActions::UpdateBlueprint => write!(f, "glue:UpdateBlueprint"),
            GlueActions::UpdateCatalog => write!(f, "glue:UpdateCatalog"),
            GlueActions::UpdateClassifier => write!(f, "glue:UpdateClassifier"),
            GlueActions::UpdateColumnStatisticsForPartition => {
                write!(f, "glue:UpdateColumnStatisticsForPartition")
            }
            GlueActions::UpdateColumnStatisticsForTable => {
                write!(f, "glue:UpdateColumnStatisticsForTable")
            }
            GlueActions::UpdateColumnStatisticsTaskSettings => {
                write!(f, "glue:UpdateColumnStatisticsTaskSettings")
            }
            GlueActions::UpdateConnection => write!(f, "glue:UpdateConnection"),
            GlueActions::UpdateCrawler => write!(f, "glue:UpdateCrawler"),
            GlueActions::UpdateCrawlerSchedule => write!(f, "glue:UpdateCrawlerSchedule"),
            GlueActions::UpdateDataQualityRuleset => write!(f, "glue:UpdateDataQualityRuleset"),
            GlueActions::UpdateDatabase => write!(f, "glue:UpdateDatabase"),
            GlueActions::UpdateDevEndpoint => write!(f, "glue:UpdateDevEndpoint"),
            GlueActions::UpdateGlueIdentityCenterConfiguration => {
                write!(f, "glue:UpdateGlueIdentityCenterConfiguration")
            }
            GlueActions::UpdateIntegrationResourceProperty => {
                write!(f, "glue:UpdateIntegrationResourceProperty")
            }
            GlueActions::UpdateIntegrationTableProperties => {
                write!(f, "glue:UpdateIntegrationTableProperties")
            }
            GlueActions::UpdateJob => write!(f, "glue:UpdateJob"),
            GlueActions::UpdateJobFromSourceControl => write!(f, "glue:UpdateJobFromSourceControl"),
            GlueActions::UpdateMlTransform => write!(f, "glue:UpdateMLTransform"),
            GlueActions::UpdatePartition => write!(f, "glue:UpdatePartition"),
            GlueActions::UpdateRegistry => write!(f, "glue:UpdateRegistry"),
            GlueActions::UpdateSchema => write!(f, "glue:UpdateSchema"),
            GlueActions::UpdateSourceControlFromJob => write!(f, "glue:UpdateSourceControlFromJob"),
            GlueActions::UpdateTable => write!(f, "glue:UpdateTable"),
            GlueActions::UpdateTableOptimizer => write!(f, "glue:UpdateTableOptimizer"),
            GlueActions::UpdateTrigger => write!(f, "glue:UpdateTrigger"),
            GlueActions::UpdateUsageProfile => write!(f, "glue:UpdateUsageProfile"),
            GlueActions::UpdateUserDefinedFunction => write!(f, "glue:UpdateUserDefinedFunction"),
            GlueActions::UpdateWorkflow => write!(f, "glue:UpdateWorkflow"),
            GlueActions::UpgradeJob => write!(f, "glue:UpgradeJob"),
            GlueActions::UseGlueStudio => write!(f, "glue:UseGlueStudio"),
            GlueActions::UseMlTransforms => write!(f, "glue:UseMLTransforms"),
        }
    }
}
