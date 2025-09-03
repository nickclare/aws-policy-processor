// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum SagemakerActions {
    AddAssociation,
    AddTags,
    AssociateTrialComponent,
    AttachClusterNodeVolume,
    BatchAddClusterNodes,
    BatchDeleteClusterNodes,
    BatchDescribeModelPackage,
    BatchGetMetrics,
    BatchGetRecord,
    BatchPutMetrics,
    CallPartnerAppApi,
    CreateAction,
    CreateAlgorithm,
    CreateApp,
    CreateAppImageConfig,
    CreateArtifact,
    CreateAutoMlJob,
    CreateAutoMlJobV2,
    CreateCluster,
    CreateClusterSchedulerConfig,
    CreateCodeRepository,
    CreateCompilationJob,
    CreateComputeQuota,
    CreateContext,
    CreateDataQualityJobDefinition,
    CreateDeviceFleet,
    CreateDomain,
    CreateEdgeDeploymentPlan,
    CreateEdgeDeploymentStage,
    CreateEdgePackagingJob,
    CreateEndpoint,
    CreateEndpointConfig,
    CreateExperiment,
    CreateFeatureGroup,
    CreateFlowDefinition,
    CreateHub,
    CreateHubContentPresignedUrls,
    CreateHubContentReference,
    CreateHumanTaskUi,
    CreateHyperParameterTuningJob,
    CreateImage,
    CreateImageVersion,
    CreateInferenceComponent,
    CreateInferenceExperiment,
    CreateInferenceRecommendationsJob,
    CreateLabelingJob,
    CreateLineageGroupPolicy,
    CreateMlflowTrackingServer,
    CreateModel,
    CreateModelBiasJobDefinition,
    CreateModelCard,
    CreateModelCardExportJob,
    CreateModelExplainabilityJobDefinition,
    CreateModelPackage,
    CreateModelPackageGroup,
    CreateModelQualityJobDefinition,
    CreateMonitoringSchedule,
    CreateNotebookInstance,
    CreateNotebookInstanceLifecycleConfig,
    CreateOptimizationJob,
    CreatePartnerApp,
    CreatePartnerAppPresignedUrl,
    CreatePipeline,
    CreatePresignedDomainUrl,
    CreatePresignedMlflowTrackingServerUrl,
    CreatePresignedNotebookInstanceUrl,
    CreateProcessingJob,
    CreateProject,
    CreateReservedCapacity,
    CreateSharedModel,
    CreateSpace,
    CreateStudioLifecycleConfig,
    CreateTrainingJob,
    CreateTrainingPlan,
    CreateTransformJob,
    CreateTrial,
    CreateTrialComponent,
    CreateUserProfile,
    CreateWorkforce,
    CreateWorkteam,
    DeleteAction,
    DeleteAlgorithm,
    DeleteApp,
    DeleteAppImageConfig,
    DeleteArtifact,
    DeleteAssociation,
    DeleteCluster,
    DeleteClusterSchedulerConfig,
    DeleteCodeRepository,
    DeleteCompilationJob,
    DeleteComputeQuota,
    DeleteContext,
    DeleteDataQualityJobDefinition,
    DeleteDeviceFleet,
    DeleteDomain,
    DeleteEdgeDeploymentPlan,
    DeleteEdgeDeploymentStage,
    DeleteEndpoint,
    DeleteEndpointConfig,
    DeleteExperiment,
    DeleteFeatureGroup,
    DeleteFlowDefinition,
    DeleteHub,
    DeleteHubContent,
    DeleteHubContentReference,
    DeleteHumanLoop,
    DeleteHumanTaskUi,
    DeleteHyperParameterTuningJob,
    DeleteImage,
    DeleteImageVersion,
    DeleteInferenceComponent,
    DeleteInferenceExperiment,
    DeleteLineageGroupPolicy,
    DeleteMlflowTrackingServer,
    DeleteModel,
    DeleteModelBiasJobDefinition,
    DeleteModelCard,
    DeleteModelExplainabilityJobDefinition,
    DeleteModelPackage,
    DeleteModelPackageGroup,
    DeleteModelPackageGroupPolicy,
    DeleteModelQualityJobDefinition,
    DeleteMonitoringSchedule,
    DeleteNotebookInstance,
    DeleteNotebookInstanceLifecycleConfig,
    DeleteOptimizationJob,
    DeletePartnerApp,
    DeletePipeline,
    DeleteProject,
    DeleteRecord,
    DeleteResourcePolicy,
    DeleteSpace,
    DeleteStudioLifecycleConfig,
    DeleteTags,
    DeleteTrial,
    DeleteTrialComponent,
    DeleteUserProfile,
    DeleteWorkforce,
    DeleteWorkteam,
    DeployHubModel,
    DeregisterDevices,
    DescribeAction,
    DescribeAlgorithm,
    DescribeApp,
    DescribeAppImageConfig,
    DescribeArtifact,
    DescribeAutoMlJob,
    DescribeAutoMlJobV2,
    DescribeCluster,
    DescribeClusterEvent,
    DescribeClusterInference,
    DescribeClusterNode,
    DescribeClusterSchedulerConfig,
    DescribeCodeRepository,
    DescribeCompilationJob,
    DescribeComputeQuota,
    DescribeContext,
    DescribeDataQualityJobDefinition,
    DescribeDevice,
    DescribeDeviceFleet,
    DescribeDomain,
    DescribeEdgeDeploymentPlan,
    DescribeEdgePackagingJob,
    DescribeEndpoint,
    DescribeEndpointConfig,
    DescribeExperiment,
    DescribeFeatureGroup,
    DescribeFeatureMetadata,
    DescribeFlowDefinition,
    DescribeHub,
    DescribeHubContent,
    DescribeHumanLoop,
    DescribeHumanTaskUi,
    DescribeHyperParameterTuningJob,
    DescribeImage,
    DescribeImageVersion,
    DescribeInferenceComponent,
    DescribeInferenceExperiment,
    DescribeInferenceRecommendationsJob,
    DescribeLabelingJob,
    DescribeLineageGroup,
    DescribeMlflowTrackingServer,
    DescribeModel,
    DescribeModelBiasJobDefinition,
    DescribeModelCard,
    DescribeModelCardExportJob,
    DescribeModelExplainabilityJobDefinition,
    DescribeModelPackage,
    DescribeModelPackageGroup,
    DescribeModelQualityJobDefinition,
    DescribeMonitoringSchedule,
    DescribeNotebookInstance,
    DescribeNotebookInstanceLifecycleConfig,
    DescribeOptimizationJob,
    DescribePartnerApp,
    DescribePipeline,
    DescribePipelineDefinitionForExecution,
    DescribePipelineExecution,
    DescribeProcessingJob,
    DescribeProject,
    DescribeReservedCapacity,
    DescribeSharedModel,
    DescribeSpace,
    DescribeStudioLifecycleConfig,
    DescribeSubscribedWorkteam,
    DescribeTrainingJob,
    DescribeTrainingPlan,
    DescribeTransformJob,
    DescribeTrial,
    DescribeTrialComponent,
    DescribeUserProfile,
    DescribeWorkforce,
    DescribeWorkteam,
    DetachClusterNodeVolume,
    DisableSagemakerServicecatalogPortfolio,
    DisassociateTrialComponent,
    EnableSagemakerServicecatalogPortfolio,
    GetDeployments,
    GetDeviceFleetReport,
    GetDeviceRegistration,
    GetLineageGroupPolicy,
    GetModelPackageGroupPolicy,
    GetRecord,
    GetResourcePolicy,
    GetSagemakerServicecatalogPortfolioStatus,
    GetScalingConfigurationRecommendation,
    GetSearchSuggestions,
    ImportHubContent,
    InvokeEndpoint,
    InvokeEndpointAsync,
    InvokeEndpointWithResponseStream,
    ListActions,
    ListAlgorithms,
    ListAliases,
    ListAppImageConfigs,
    ListApps,
    ListArtifacts,
    ListAssociations,
    ListAutoMlJobs,
    ListCandidatesForAutoMlJob,
    ListClusterEvents,
    ListClusterNodes,
    ListClusterSchedulerConfigs,
    ListClusters,
    ListCodeRepositories,
    ListCompilationJobs,
    ListComputeQuotas,
    ListContexts,
    ListDataQualityJobDefinitions,
    ListDeviceFleets,
    ListDevices,
    ListDomains,
    ListEdgeDeploymentPlans,
    ListEdgePackagingJobs,
    ListEndpointConfigs,
    ListEndpoints,
    ListExperiments,
    ListFeatureGroups,
    ListFlowDefinitions,
    ListHubContentVersions,
    ListHubContents,
    ListHubs,
    ListHumanLoops,
    ListHumanTaskUis,
    ListHyperParameterTuningJobs,
    ListImageVersions,
    ListImages,
    ListInferenceComponents,
    ListInferenceExperiments,
    ListInferenceRecommendationsJobSteps,
    ListInferenceRecommendationsJobs,
    ListLabelingJobs,
    ListLabelingJobsForWorkteam,
    ListLineageGroups,
    ListMlflowTrackingServers,
    ListModelBiasJobDefinitions,
    ListModelCardExportJobs,
    ListModelCardVersions,
    ListModelCards,
    ListModelExplainabilityJobDefinitions,
    ListModelMetadata,
    ListModelPackageGroups,
    ListModelPackages,
    ListModelQualityJobDefinitions,
    ListModels,
    ListMonitoringAlertHistory,
    ListMonitoringAlerts,
    ListMonitoringExecutions,
    ListMonitoringSchedules,
    ListNotebookInstanceLifecycleConfigs,
    ListNotebookInstances,
    ListOptimizationJobs,
    ListPartnerApps,
    ListPipelineExecutionSteps,
    ListPipelineExecutions,
    ListPipelineParametersForExecution,
    ListPipelineVersions,
    ListPipelines,
    ListProcessingJobs,
    ListProjects,
    ListResourceCatalogs,
    ListSharedModelEvents,
    ListSharedModelVersions,
    ListSharedModels,
    ListSpaces,
    ListStageDevices,
    ListStudioLifecycleConfigs,
    ListSubscribedWorkteams,
    ListTags,
    ListTrainingJobs,
    ListTrainingJobsForHyperParameterTuningJob,
    ListTrainingPlans,
    ListTransformJobs,
    ListTrialComponents,
    ListTrials,
    ListUltraServersByReservedCapacity,
    ListUserProfiles,
    ListWorkforces,
    ListWorkteams,
    PutLineageGroupPolicy,
    PutModelPackageGroupPolicy,
    PutRecord,
    PutResourcePolicy,
    QueryLineage,
    RegisterDevices,
    RenderUiTemplate,
    RetryPipelineExecution,
    Search,
    SearchTrainingPlanOfferings,
    SendHeartbeat,
    SendPipelineExecutionStepFailure,
    SendPipelineExecutionStepSuccess,
    SendSharedModelEvent,
    StartEdgeDeploymentStage,
    StartHumanLoop,
    StartInferenceExperiment,
    StartMlflowTrackingServer,
    StartMonitoringSchedule,
    StartNotebookInstance,
    StartPipelineExecution,
    StartSession,
    StopAutoMlJob,
    StopCompilationJob,
    StopEdgeDeploymentStage,
    StopEdgePackagingJob,
    StopHumanLoop,
    StopHyperParameterTuningJob,
    StopInferenceExperiment,
    StopInferenceRecommendationsJob,
    StopLabelingJob,
    StopMlflowTrackingServer,
    StopMonitoringSchedule,
    StopNotebookInstance,
    StopOptimizationJob,
    StopPipelineExecution,
    StopProcessingJob,
    StopTrainingJob,
    StopTransformJob,
    TrainHubModel,
    UpdateAction,
    UpdateAppImageConfig,
    UpdateArtifact,
    UpdateCluster,
    UpdateClusterInference,
    UpdateClusterSchedulerConfig,
    UpdateClusterSoftware,
    UpdateCodeRepository,
    UpdateComputeQuota,
    UpdateContext,
    UpdateDeviceFleet,
    UpdateDevices,
    UpdateDomain,
    UpdateEndpoint,
    UpdateEndpointWeightsAndCapacities,
    UpdateExperiment,
    UpdateFeatureGroup,
    UpdateFeatureMetadata,
    UpdateHub,
    UpdateHubContent,
    UpdateHubContentReference,
    UpdateImage,
    UpdateImageVersion,
    UpdateInferenceComponent,
    UpdateInferenceComponentRuntimeConfig,
    UpdateInferenceExperiment,
    UpdateMlflowTrackingServer,
    UpdateModelCard,
    UpdateModelPackage,
    UpdateMonitoringAlert,
    UpdateMonitoringSchedule,
    UpdateNotebookInstance,
    UpdateNotebookInstanceLifecycleConfig,
    UpdatePartnerApp,
    UpdatePipeline,
    UpdatePipelineExecution,
    UpdatePipelineVersion,
    UpdateProject,
    UpdateSharedModel,
    UpdateSpace,
    UpdateTrainingJob,
    UpdateTrial,
    UpdateTrialComponent,
    UpdateUserProfile,
    UpdateWorkforce,
    UpdateWorkteam,
}
impl std::fmt::Display for SagemakerActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SagemakerActions::AddAssociation => write!(f, "sagemaker:AddAssociation"),
            SagemakerActions::AddTags => write!(f, "sagemaker:AddTags"),
            SagemakerActions::AssociateTrialComponent => {
                write!(f, "sagemaker:AssociateTrialComponent")
            }
            SagemakerActions::AttachClusterNodeVolume => {
                write!(f, "sagemaker:AttachClusterNodeVolume")
            }
            SagemakerActions::BatchAddClusterNodes => write!(f, "sagemaker:BatchAddClusterNodes"),
            SagemakerActions::BatchDeleteClusterNodes => {
                write!(f, "sagemaker:BatchDeleteClusterNodes")
            }
            SagemakerActions::BatchDescribeModelPackage => {
                write!(f, "sagemaker:BatchDescribeModelPackage")
            }
            SagemakerActions::BatchGetMetrics => write!(f, "sagemaker:BatchGetMetrics"),
            SagemakerActions::BatchGetRecord => write!(f, "sagemaker:BatchGetRecord"),
            SagemakerActions::BatchPutMetrics => write!(f, "sagemaker:BatchPutMetrics"),
            SagemakerActions::CallPartnerAppApi => write!(f, "sagemaker:CallPartnerAppApi"),
            SagemakerActions::CreateAction => write!(f, "sagemaker:CreateAction"),
            SagemakerActions::CreateAlgorithm => write!(f, "sagemaker:CreateAlgorithm"),
            SagemakerActions::CreateApp => write!(f, "sagemaker:CreateApp"),
            SagemakerActions::CreateAppImageConfig => write!(f, "sagemaker:CreateAppImageConfig"),
            SagemakerActions::CreateArtifact => write!(f, "sagemaker:CreateArtifact"),
            SagemakerActions::CreateAutoMlJob => write!(f, "sagemaker:CreateAutoMLJob"),
            SagemakerActions::CreateAutoMlJobV2 => write!(f, "sagemaker:CreateAutoMLJobV2"),
            SagemakerActions::CreateCluster => write!(f, "sagemaker:CreateCluster"),
            SagemakerActions::CreateClusterSchedulerConfig => {
                write!(f, "sagemaker:CreateClusterSchedulerConfig")
            }
            SagemakerActions::CreateCodeRepository => write!(f, "sagemaker:CreateCodeRepository"),
            SagemakerActions::CreateCompilationJob => write!(f, "sagemaker:CreateCompilationJob"),
            SagemakerActions::CreateComputeQuota => write!(f, "sagemaker:CreateComputeQuota"),
            SagemakerActions::CreateContext => write!(f, "sagemaker:CreateContext"),
            SagemakerActions::CreateDataQualityJobDefinition => {
                write!(f, "sagemaker:CreateDataQualityJobDefinition")
            }
            SagemakerActions::CreateDeviceFleet => write!(f, "sagemaker:CreateDeviceFleet"),
            SagemakerActions::CreateDomain => write!(f, "sagemaker:CreateDomain"),
            SagemakerActions::CreateEdgeDeploymentPlan => {
                write!(f, "sagemaker:CreateEdgeDeploymentPlan")
            }
            SagemakerActions::CreateEdgeDeploymentStage => {
                write!(f, "sagemaker:CreateEdgeDeploymentStage")
            }
            SagemakerActions::CreateEdgePackagingJob => {
                write!(f, "sagemaker:CreateEdgePackagingJob")
            }
            SagemakerActions::CreateEndpoint => write!(f, "sagemaker:CreateEndpoint"),
            SagemakerActions::CreateEndpointConfig => write!(f, "sagemaker:CreateEndpointConfig"),
            SagemakerActions::CreateExperiment => write!(f, "sagemaker:CreateExperiment"),
            SagemakerActions::CreateFeatureGroup => write!(f, "sagemaker:CreateFeatureGroup"),
            SagemakerActions::CreateFlowDefinition => write!(f, "sagemaker:CreateFlowDefinition"),
            SagemakerActions::CreateHub => write!(f, "sagemaker:CreateHub"),
            SagemakerActions::CreateHubContentPresignedUrls => {
                write!(f, "sagemaker:CreateHubContentPresignedUrls")
            }
            SagemakerActions::CreateHubContentReference => {
                write!(f, "sagemaker:CreateHubContentReference")
            }
            SagemakerActions::CreateHumanTaskUi => write!(f, "sagemaker:CreateHumanTaskUi"),
            SagemakerActions::CreateHyperParameterTuningJob => {
                write!(f, "sagemaker:CreateHyperParameterTuningJob")
            }
            SagemakerActions::CreateImage => write!(f, "sagemaker:CreateImage"),
            SagemakerActions::CreateImageVersion => write!(f, "sagemaker:CreateImageVersion"),
            SagemakerActions::CreateInferenceComponent => {
                write!(f, "sagemaker:CreateInferenceComponent")
            }
            SagemakerActions::CreateInferenceExperiment => {
                write!(f, "sagemaker:CreateInferenceExperiment")
            }
            SagemakerActions::CreateInferenceRecommendationsJob => {
                write!(f, "sagemaker:CreateInferenceRecommendationsJob")
            }
            SagemakerActions::CreateLabelingJob => write!(f, "sagemaker:CreateLabelingJob"),
            SagemakerActions::CreateLineageGroupPolicy => {
                write!(f, "sagemaker:CreateLineageGroupPolicy")
            }
            SagemakerActions::CreateMlflowTrackingServer => {
                write!(f, "sagemaker:CreateMlflowTrackingServer")
            }
            SagemakerActions::CreateModel => write!(f, "sagemaker:CreateModel"),
            SagemakerActions::CreateModelBiasJobDefinition => {
                write!(f, "sagemaker:CreateModelBiasJobDefinition")
            }
            SagemakerActions::CreateModelCard => write!(f, "sagemaker:CreateModelCard"),
            SagemakerActions::CreateModelCardExportJob => {
                write!(f, "sagemaker:CreateModelCardExportJob")
            }
            SagemakerActions::CreateModelExplainabilityJobDefinition => {
                write!(f, "sagemaker:CreateModelExplainabilityJobDefinition")
            }
            SagemakerActions::CreateModelPackage => write!(f, "sagemaker:CreateModelPackage"),
            SagemakerActions::CreateModelPackageGroup => {
                write!(f, "sagemaker:CreateModelPackageGroup")
            }
            SagemakerActions::CreateModelQualityJobDefinition => {
                write!(f, "sagemaker:CreateModelQualityJobDefinition")
            }
            SagemakerActions::CreateMonitoringSchedule => {
                write!(f, "sagemaker:CreateMonitoringSchedule")
            }
            SagemakerActions::CreateNotebookInstance => {
                write!(f, "sagemaker:CreateNotebookInstance")
            }
            SagemakerActions::CreateNotebookInstanceLifecycleConfig => {
                write!(f, "sagemaker:CreateNotebookInstanceLifecycleConfig")
            }
            SagemakerActions::CreateOptimizationJob => write!(f, "sagemaker:CreateOptimizationJob"),
            SagemakerActions::CreatePartnerApp => write!(f, "sagemaker:CreatePartnerApp"),
            SagemakerActions::CreatePartnerAppPresignedUrl => {
                write!(f, "sagemaker:CreatePartnerAppPresignedUrl")
            }
            SagemakerActions::CreatePipeline => write!(f, "sagemaker:CreatePipeline"),
            SagemakerActions::CreatePresignedDomainUrl => {
                write!(f, "sagemaker:CreatePresignedDomainUrl")
            }
            SagemakerActions::CreatePresignedMlflowTrackingServerUrl => {
                write!(f, "sagemaker:CreatePresignedMlflowTrackingServerUrl")
            }
            SagemakerActions::CreatePresignedNotebookInstanceUrl => {
                write!(f, "sagemaker:CreatePresignedNotebookInstanceUrl")
            }
            SagemakerActions::CreateProcessingJob => write!(f, "sagemaker:CreateProcessingJob"),
            SagemakerActions::CreateProject => write!(f, "sagemaker:CreateProject"),
            SagemakerActions::CreateReservedCapacity => {
                write!(f, "sagemaker:CreateReservedCapacity")
            }
            SagemakerActions::CreateSharedModel => write!(f, "sagemaker:CreateSharedModel"),
            SagemakerActions::CreateSpace => write!(f, "sagemaker:CreateSpace"),
            SagemakerActions::CreateStudioLifecycleConfig => {
                write!(f, "sagemaker:CreateStudioLifecycleConfig")
            }
            SagemakerActions::CreateTrainingJob => write!(f, "sagemaker:CreateTrainingJob"),
            SagemakerActions::CreateTrainingPlan => write!(f, "sagemaker:CreateTrainingPlan"),
            SagemakerActions::CreateTransformJob => write!(f, "sagemaker:CreateTransformJob"),
            SagemakerActions::CreateTrial => write!(f, "sagemaker:CreateTrial"),
            SagemakerActions::CreateTrialComponent => write!(f, "sagemaker:CreateTrialComponent"),
            SagemakerActions::CreateUserProfile => write!(f, "sagemaker:CreateUserProfile"),
            SagemakerActions::CreateWorkforce => write!(f, "sagemaker:CreateWorkforce"),
            SagemakerActions::CreateWorkteam => write!(f, "sagemaker:CreateWorkteam"),
            SagemakerActions::DeleteAction => write!(f, "sagemaker:DeleteAction"),
            SagemakerActions::DeleteAlgorithm => write!(f, "sagemaker:DeleteAlgorithm"),
            SagemakerActions::DeleteApp => write!(f, "sagemaker:DeleteApp"),
            SagemakerActions::DeleteAppImageConfig => write!(f, "sagemaker:DeleteAppImageConfig"),
            SagemakerActions::DeleteArtifact => write!(f, "sagemaker:DeleteArtifact"),
            SagemakerActions::DeleteAssociation => write!(f, "sagemaker:DeleteAssociation"),
            SagemakerActions::DeleteCluster => write!(f, "sagemaker:DeleteCluster"),
            SagemakerActions::DeleteClusterSchedulerConfig => {
                write!(f, "sagemaker:DeleteClusterSchedulerConfig")
            }
            SagemakerActions::DeleteCodeRepository => write!(f, "sagemaker:DeleteCodeRepository"),
            SagemakerActions::DeleteCompilationJob => write!(f, "sagemaker:DeleteCompilationJob"),
            SagemakerActions::DeleteComputeQuota => write!(f, "sagemaker:DeleteComputeQuota"),
            SagemakerActions::DeleteContext => write!(f, "sagemaker:DeleteContext"),
            SagemakerActions::DeleteDataQualityJobDefinition => {
                write!(f, "sagemaker:DeleteDataQualityJobDefinition")
            }
            SagemakerActions::DeleteDeviceFleet => write!(f, "sagemaker:DeleteDeviceFleet"),
            SagemakerActions::DeleteDomain => write!(f, "sagemaker:DeleteDomain"),
            SagemakerActions::DeleteEdgeDeploymentPlan => {
                write!(f, "sagemaker:DeleteEdgeDeploymentPlan")
            }
            SagemakerActions::DeleteEdgeDeploymentStage => {
                write!(f, "sagemaker:DeleteEdgeDeploymentStage")
            }
            SagemakerActions::DeleteEndpoint => write!(f, "sagemaker:DeleteEndpoint"),
            SagemakerActions::DeleteEndpointConfig => write!(f, "sagemaker:DeleteEndpointConfig"),
            SagemakerActions::DeleteExperiment => write!(f, "sagemaker:DeleteExperiment"),
            SagemakerActions::DeleteFeatureGroup => write!(f, "sagemaker:DeleteFeatureGroup"),
            SagemakerActions::DeleteFlowDefinition => write!(f, "sagemaker:DeleteFlowDefinition"),
            SagemakerActions::DeleteHub => write!(f, "sagemaker:DeleteHub"),
            SagemakerActions::DeleteHubContent => write!(f, "sagemaker:DeleteHubContent"),
            SagemakerActions::DeleteHubContentReference => {
                write!(f, "sagemaker:DeleteHubContentReference")
            }
            SagemakerActions::DeleteHumanLoop => write!(f, "sagemaker:DeleteHumanLoop"),
            SagemakerActions::DeleteHumanTaskUi => write!(f, "sagemaker:DeleteHumanTaskUi"),
            SagemakerActions::DeleteHyperParameterTuningJob => {
                write!(f, "sagemaker:DeleteHyperParameterTuningJob")
            }
            SagemakerActions::DeleteImage => write!(f, "sagemaker:DeleteImage"),
            SagemakerActions::DeleteImageVersion => write!(f, "sagemaker:DeleteImageVersion"),
            SagemakerActions::DeleteInferenceComponent => {
                write!(f, "sagemaker:DeleteInferenceComponent")
            }
            SagemakerActions::DeleteInferenceExperiment => {
                write!(f, "sagemaker:DeleteInferenceExperiment")
            }
            SagemakerActions::DeleteLineageGroupPolicy => {
                write!(f, "sagemaker:DeleteLineageGroupPolicy")
            }
            SagemakerActions::DeleteMlflowTrackingServer => {
                write!(f, "sagemaker:DeleteMlflowTrackingServer")
            }
            SagemakerActions::DeleteModel => write!(f, "sagemaker:DeleteModel"),
            SagemakerActions::DeleteModelBiasJobDefinition => {
                write!(f, "sagemaker:DeleteModelBiasJobDefinition")
            }
            SagemakerActions::DeleteModelCard => write!(f, "sagemaker:DeleteModelCard"),
            SagemakerActions::DeleteModelExplainabilityJobDefinition => {
                write!(f, "sagemaker:DeleteModelExplainabilityJobDefinition")
            }
            SagemakerActions::DeleteModelPackage => write!(f, "sagemaker:DeleteModelPackage"),
            SagemakerActions::DeleteModelPackageGroup => {
                write!(f, "sagemaker:DeleteModelPackageGroup")
            }
            SagemakerActions::DeleteModelPackageGroupPolicy => {
                write!(f, "sagemaker:DeleteModelPackageGroupPolicy")
            }
            SagemakerActions::DeleteModelQualityJobDefinition => {
                write!(f, "sagemaker:DeleteModelQualityJobDefinition")
            }
            SagemakerActions::DeleteMonitoringSchedule => {
                write!(f, "sagemaker:DeleteMonitoringSchedule")
            }
            SagemakerActions::DeleteNotebookInstance => {
                write!(f, "sagemaker:DeleteNotebookInstance")
            }
            SagemakerActions::DeleteNotebookInstanceLifecycleConfig => {
                write!(f, "sagemaker:DeleteNotebookInstanceLifecycleConfig")
            }
            SagemakerActions::DeleteOptimizationJob => write!(f, "sagemaker:DeleteOptimizationJob"),
            SagemakerActions::DeletePartnerApp => write!(f, "sagemaker:DeletePartnerApp"),
            SagemakerActions::DeletePipeline => write!(f, "sagemaker:DeletePipeline"),
            SagemakerActions::DeleteProject => write!(f, "sagemaker:DeleteProject"),
            SagemakerActions::DeleteRecord => write!(f, "sagemaker:DeleteRecord"),
            SagemakerActions::DeleteResourcePolicy => write!(f, "sagemaker:DeleteResourcePolicy"),
            SagemakerActions::DeleteSpace => write!(f, "sagemaker:DeleteSpace"),
            SagemakerActions::DeleteStudioLifecycleConfig => {
                write!(f, "sagemaker:DeleteStudioLifecycleConfig")
            }
            SagemakerActions::DeleteTags => write!(f, "sagemaker:DeleteTags"),
            SagemakerActions::DeleteTrial => write!(f, "sagemaker:DeleteTrial"),
            SagemakerActions::DeleteTrialComponent => write!(f, "sagemaker:DeleteTrialComponent"),
            SagemakerActions::DeleteUserProfile => write!(f, "sagemaker:DeleteUserProfile"),
            SagemakerActions::DeleteWorkforce => write!(f, "sagemaker:DeleteWorkforce"),
            SagemakerActions::DeleteWorkteam => write!(f, "sagemaker:DeleteWorkteam"),
            SagemakerActions::DeployHubModel => write!(f, "sagemaker:DeployHubModel"),
            SagemakerActions::DeregisterDevices => write!(f, "sagemaker:DeregisterDevices"),
            SagemakerActions::DescribeAction => write!(f, "sagemaker:DescribeAction"),
            SagemakerActions::DescribeAlgorithm => write!(f, "sagemaker:DescribeAlgorithm"),
            SagemakerActions::DescribeApp => write!(f, "sagemaker:DescribeApp"),
            SagemakerActions::DescribeAppImageConfig => {
                write!(f, "sagemaker:DescribeAppImageConfig")
            }
            SagemakerActions::DescribeArtifact => write!(f, "sagemaker:DescribeArtifact"),
            SagemakerActions::DescribeAutoMlJob => write!(f, "sagemaker:DescribeAutoMLJob"),
            SagemakerActions::DescribeAutoMlJobV2 => write!(f, "sagemaker:DescribeAutoMLJobV2"),
            SagemakerActions::DescribeCluster => write!(f, "sagemaker:DescribeCluster"),
            SagemakerActions::DescribeClusterEvent => write!(f, "sagemaker:DescribeClusterEvent"),
            SagemakerActions::DescribeClusterInference => {
                write!(f, "sagemaker:DescribeClusterInference")
            }
            SagemakerActions::DescribeClusterNode => write!(f, "sagemaker:DescribeClusterNode"),
            SagemakerActions::DescribeClusterSchedulerConfig => {
                write!(f, "sagemaker:DescribeClusterSchedulerConfig")
            }
            SagemakerActions::DescribeCodeRepository => {
                write!(f, "sagemaker:DescribeCodeRepository")
            }
            SagemakerActions::DescribeCompilationJob => {
                write!(f, "sagemaker:DescribeCompilationJob")
            }
            SagemakerActions::DescribeComputeQuota => write!(f, "sagemaker:DescribeComputeQuota"),
            SagemakerActions::DescribeContext => write!(f, "sagemaker:DescribeContext"),
            SagemakerActions::DescribeDataQualityJobDefinition => {
                write!(f, "sagemaker:DescribeDataQualityJobDefinition")
            }
            SagemakerActions::DescribeDevice => write!(f, "sagemaker:DescribeDevice"),
            SagemakerActions::DescribeDeviceFleet => write!(f, "sagemaker:DescribeDeviceFleet"),
            SagemakerActions::DescribeDomain => write!(f, "sagemaker:DescribeDomain"),
            SagemakerActions::DescribeEdgeDeploymentPlan => {
                write!(f, "sagemaker:DescribeEdgeDeploymentPlan")
            }
            SagemakerActions::DescribeEdgePackagingJob => {
                write!(f, "sagemaker:DescribeEdgePackagingJob")
            }
            SagemakerActions::DescribeEndpoint => write!(f, "sagemaker:DescribeEndpoint"),
            SagemakerActions::DescribeEndpointConfig => {
                write!(f, "sagemaker:DescribeEndpointConfig")
            }
            SagemakerActions::DescribeExperiment => write!(f, "sagemaker:DescribeExperiment"),
            SagemakerActions::DescribeFeatureGroup => write!(f, "sagemaker:DescribeFeatureGroup"),
            SagemakerActions::DescribeFeatureMetadata => {
                write!(f, "sagemaker:DescribeFeatureMetadata")
            }
            SagemakerActions::DescribeFlowDefinition => {
                write!(f, "sagemaker:DescribeFlowDefinition")
            }
            SagemakerActions::DescribeHub => write!(f, "sagemaker:DescribeHub"),
            SagemakerActions::DescribeHubContent => write!(f, "sagemaker:DescribeHubContent"),
            SagemakerActions::DescribeHumanLoop => write!(f, "sagemaker:DescribeHumanLoop"),
            SagemakerActions::DescribeHumanTaskUi => write!(f, "sagemaker:DescribeHumanTaskUi"),
            SagemakerActions::DescribeHyperParameterTuningJob => {
                write!(f, "sagemaker:DescribeHyperParameterTuningJob")
            }
            SagemakerActions::DescribeImage => write!(f, "sagemaker:DescribeImage"),
            SagemakerActions::DescribeImageVersion => write!(f, "sagemaker:DescribeImageVersion"),
            SagemakerActions::DescribeInferenceComponent => {
                write!(f, "sagemaker:DescribeInferenceComponent")
            }
            SagemakerActions::DescribeInferenceExperiment => {
                write!(f, "sagemaker:DescribeInferenceExperiment")
            }
            SagemakerActions::DescribeInferenceRecommendationsJob => {
                write!(f, "sagemaker:DescribeInferenceRecommendationsJob")
            }
            SagemakerActions::DescribeLabelingJob => write!(f, "sagemaker:DescribeLabelingJob"),
            SagemakerActions::DescribeLineageGroup => write!(f, "sagemaker:DescribeLineageGroup"),
            SagemakerActions::DescribeMlflowTrackingServer => {
                write!(f, "sagemaker:DescribeMlflowTrackingServer")
            }
            SagemakerActions::DescribeModel => write!(f, "sagemaker:DescribeModel"),
            SagemakerActions::DescribeModelBiasJobDefinition => {
                write!(f, "sagemaker:DescribeModelBiasJobDefinition")
            }
            SagemakerActions::DescribeModelCard => write!(f, "sagemaker:DescribeModelCard"),
            SagemakerActions::DescribeModelCardExportJob => {
                write!(f, "sagemaker:DescribeModelCardExportJob")
            }
            SagemakerActions::DescribeModelExplainabilityJobDefinition => {
                write!(f, "sagemaker:DescribeModelExplainabilityJobDefinition")
            }
            SagemakerActions::DescribeModelPackage => write!(f, "sagemaker:DescribeModelPackage"),
            SagemakerActions::DescribeModelPackageGroup => {
                write!(f, "sagemaker:DescribeModelPackageGroup")
            }
            SagemakerActions::DescribeModelQualityJobDefinition => {
                write!(f, "sagemaker:DescribeModelQualityJobDefinition")
            }
            SagemakerActions::DescribeMonitoringSchedule => {
                write!(f, "sagemaker:DescribeMonitoringSchedule")
            }
            SagemakerActions::DescribeNotebookInstance => {
                write!(f, "sagemaker:DescribeNotebookInstance")
            }
            SagemakerActions::DescribeNotebookInstanceLifecycleConfig => {
                write!(f, "sagemaker:DescribeNotebookInstanceLifecycleConfig")
            }
            SagemakerActions::DescribeOptimizationJob => {
                write!(f, "sagemaker:DescribeOptimizationJob")
            }
            SagemakerActions::DescribePartnerApp => write!(f, "sagemaker:DescribePartnerApp"),
            SagemakerActions::DescribePipeline => write!(f, "sagemaker:DescribePipeline"),
            SagemakerActions::DescribePipelineDefinitionForExecution => {
                write!(f, "sagemaker:DescribePipelineDefinitionForExecution")
            }
            SagemakerActions::DescribePipelineExecution => {
                write!(f, "sagemaker:DescribePipelineExecution")
            }
            SagemakerActions::DescribeProcessingJob => write!(f, "sagemaker:DescribeProcessingJob"),
            SagemakerActions::DescribeProject => write!(f, "sagemaker:DescribeProject"),
            SagemakerActions::DescribeReservedCapacity => {
                write!(f, "sagemaker:DescribeReservedCapacity")
            }
            SagemakerActions::DescribeSharedModel => write!(f, "sagemaker:DescribeSharedModel"),
            SagemakerActions::DescribeSpace => write!(f, "sagemaker:DescribeSpace"),
            SagemakerActions::DescribeStudioLifecycleConfig => {
                write!(f, "sagemaker:DescribeStudioLifecycleConfig")
            }
            SagemakerActions::DescribeSubscribedWorkteam => {
                write!(f, "sagemaker:DescribeSubscribedWorkteam")
            }
            SagemakerActions::DescribeTrainingJob => write!(f, "sagemaker:DescribeTrainingJob"),
            SagemakerActions::DescribeTrainingPlan => write!(f, "sagemaker:DescribeTrainingPlan"),
            SagemakerActions::DescribeTransformJob => write!(f, "sagemaker:DescribeTransformJob"),
            SagemakerActions::DescribeTrial => write!(f, "sagemaker:DescribeTrial"),
            SagemakerActions::DescribeTrialComponent => {
                write!(f, "sagemaker:DescribeTrialComponent")
            }
            SagemakerActions::DescribeUserProfile => write!(f, "sagemaker:DescribeUserProfile"),
            SagemakerActions::DescribeWorkforce => write!(f, "sagemaker:DescribeWorkforce"),
            SagemakerActions::DescribeWorkteam => write!(f, "sagemaker:DescribeWorkteam"),
            SagemakerActions::DetachClusterNodeVolume => {
                write!(f, "sagemaker:DetachClusterNodeVolume")
            }
            SagemakerActions::DisableSagemakerServicecatalogPortfolio => {
                write!(f, "sagemaker:DisableSagemakerServicecatalogPortfolio")
            }
            SagemakerActions::DisassociateTrialComponent => {
                write!(f, "sagemaker:DisassociateTrialComponent")
            }
            SagemakerActions::EnableSagemakerServicecatalogPortfolio => {
                write!(f, "sagemaker:EnableSagemakerServicecatalogPortfolio")
            }
            SagemakerActions::GetDeployments => write!(f, "sagemaker:GetDeployments"),
            SagemakerActions::GetDeviceFleetReport => write!(f, "sagemaker:GetDeviceFleetReport"),
            SagemakerActions::GetDeviceRegistration => write!(f, "sagemaker:GetDeviceRegistration"),
            SagemakerActions::GetLineageGroupPolicy => write!(f, "sagemaker:GetLineageGroupPolicy"),
            SagemakerActions::GetModelPackageGroupPolicy => {
                write!(f, "sagemaker:GetModelPackageGroupPolicy")
            }
            SagemakerActions::GetRecord => write!(f, "sagemaker:GetRecord"),
            SagemakerActions::GetResourcePolicy => write!(f, "sagemaker:GetResourcePolicy"),
            SagemakerActions::GetSagemakerServicecatalogPortfolioStatus => {
                write!(f, "sagemaker:GetSagemakerServicecatalogPortfolioStatus")
            }
            SagemakerActions::GetScalingConfigurationRecommendation => {
                write!(f, "sagemaker:GetScalingConfigurationRecommendation")
            }
            SagemakerActions::GetSearchSuggestions => write!(f, "sagemaker:GetSearchSuggestions"),
            SagemakerActions::ImportHubContent => write!(f, "sagemaker:ImportHubContent"),
            SagemakerActions::InvokeEndpoint => write!(f, "sagemaker:InvokeEndpoint"),
            SagemakerActions::InvokeEndpointAsync => write!(f, "sagemaker:InvokeEndpointAsync"),
            SagemakerActions::InvokeEndpointWithResponseStream => {
                write!(f, "sagemaker:InvokeEndpointWithResponseStream")
            }
            SagemakerActions::ListActions => write!(f, "sagemaker:ListActions"),
            SagemakerActions::ListAlgorithms => write!(f, "sagemaker:ListAlgorithms"),
            SagemakerActions::ListAliases => write!(f, "sagemaker:ListAliases"),
            SagemakerActions::ListAppImageConfigs => write!(f, "sagemaker:ListAppImageConfigs"),
            SagemakerActions::ListApps => write!(f, "sagemaker:ListApps"),
            SagemakerActions::ListArtifacts => write!(f, "sagemaker:ListArtifacts"),
            SagemakerActions::ListAssociations => write!(f, "sagemaker:ListAssociations"),
            SagemakerActions::ListAutoMlJobs => write!(f, "sagemaker:ListAutoMLJobs"),
            SagemakerActions::ListCandidatesForAutoMlJob => {
                write!(f, "sagemaker:ListCandidatesForAutoMLJob")
            }
            SagemakerActions::ListClusterEvents => write!(f, "sagemaker:ListClusterEvents"),
            SagemakerActions::ListClusterNodes => write!(f, "sagemaker:ListClusterNodes"),
            SagemakerActions::ListClusterSchedulerConfigs => {
                write!(f, "sagemaker:ListClusterSchedulerConfigs")
            }
            SagemakerActions::ListClusters => write!(f, "sagemaker:ListClusters"),
            SagemakerActions::ListCodeRepositories => write!(f, "sagemaker:ListCodeRepositories"),
            SagemakerActions::ListCompilationJobs => write!(f, "sagemaker:ListCompilationJobs"),
            SagemakerActions::ListComputeQuotas => write!(f, "sagemaker:ListComputeQuotas"),
            SagemakerActions::ListContexts => write!(f, "sagemaker:ListContexts"),
            SagemakerActions::ListDataQualityJobDefinitions => {
                write!(f, "sagemaker:ListDataQualityJobDefinitions")
            }
            SagemakerActions::ListDeviceFleets => write!(f, "sagemaker:ListDeviceFleets"),
            SagemakerActions::ListDevices => write!(f, "sagemaker:ListDevices"),
            SagemakerActions::ListDomains => write!(f, "sagemaker:ListDomains"),
            SagemakerActions::ListEdgeDeploymentPlans => {
                write!(f, "sagemaker:ListEdgeDeploymentPlans")
            }
            SagemakerActions::ListEdgePackagingJobs => write!(f, "sagemaker:ListEdgePackagingJobs"),
            SagemakerActions::ListEndpointConfigs => write!(f, "sagemaker:ListEndpointConfigs"),
            SagemakerActions::ListEndpoints => write!(f, "sagemaker:ListEndpoints"),
            SagemakerActions::ListExperiments => write!(f, "sagemaker:ListExperiments"),
            SagemakerActions::ListFeatureGroups => write!(f, "sagemaker:ListFeatureGroups"),
            SagemakerActions::ListFlowDefinitions => write!(f, "sagemaker:ListFlowDefinitions"),
            SagemakerActions::ListHubContentVersions => {
                write!(f, "sagemaker:ListHubContentVersions")
            }
            SagemakerActions::ListHubContents => write!(f, "sagemaker:ListHubContents"),
            SagemakerActions::ListHubs => write!(f, "sagemaker:ListHubs"),
            SagemakerActions::ListHumanLoops => write!(f, "sagemaker:ListHumanLoops"),
            SagemakerActions::ListHumanTaskUis => write!(f, "sagemaker:ListHumanTaskUis"),
            SagemakerActions::ListHyperParameterTuningJobs => {
                write!(f, "sagemaker:ListHyperParameterTuningJobs")
            }
            SagemakerActions::ListImageVersions => write!(f, "sagemaker:ListImageVersions"),
            SagemakerActions::ListImages => write!(f, "sagemaker:ListImages"),
            SagemakerActions::ListInferenceComponents => {
                write!(f, "sagemaker:ListInferenceComponents")
            }
            SagemakerActions::ListInferenceExperiments => {
                write!(f, "sagemaker:ListInferenceExperiments")
            }
            SagemakerActions::ListInferenceRecommendationsJobSteps => {
                write!(f, "sagemaker:ListInferenceRecommendationsJobSteps")
            }
            SagemakerActions::ListInferenceRecommendationsJobs => {
                write!(f, "sagemaker:ListInferenceRecommendationsJobs")
            }
            SagemakerActions::ListLabelingJobs => write!(f, "sagemaker:ListLabelingJobs"),
            SagemakerActions::ListLabelingJobsForWorkteam => {
                write!(f, "sagemaker:ListLabelingJobsForWorkteam")
            }
            SagemakerActions::ListLineageGroups => write!(f, "sagemaker:ListLineageGroups"),
            SagemakerActions::ListMlflowTrackingServers => {
                write!(f, "sagemaker:ListMlflowTrackingServers")
            }
            SagemakerActions::ListModelBiasJobDefinitions => {
                write!(f, "sagemaker:ListModelBiasJobDefinitions")
            }
            SagemakerActions::ListModelCardExportJobs => {
                write!(f, "sagemaker:ListModelCardExportJobs")
            }
            SagemakerActions::ListModelCardVersions => write!(f, "sagemaker:ListModelCardVersions"),
            SagemakerActions::ListModelCards => write!(f, "sagemaker:ListModelCards"),
            SagemakerActions::ListModelExplainabilityJobDefinitions => {
                write!(f, "sagemaker:ListModelExplainabilityJobDefinitions")
            }
            SagemakerActions::ListModelMetadata => write!(f, "sagemaker:ListModelMetadata"),
            SagemakerActions::ListModelPackageGroups => {
                write!(f, "sagemaker:ListModelPackageGroups")
            }
            SagemakerActions::ListModelPackages => write!(f, "sagemaker:ListModelPackages"),
            SagemakerActions::ListModelQualityJobDefinitions => {
                write!(f, "sagemaker:ListModelQualityJobDefinitions")
            }
            SagemakerActions::ListModels => write!(f, "sagemaker:ListModels"),
            SagemakerActions::ListMonitoringAlertHistory => {
                write!(f, "sagemaker:ListMonitoringAlertHistory")
            }
            SagemakerActions::ListMonitoringAlerts => write!(f, "sagemaker:ListMonitoringAlerts"),
            SagemakerActions::ListMonitoringExecutions => {
                write!(f, "sagemaker:ListMonitoringExecutions")
            }
            SagemakerActions::ListMonitoringSchedules => {
                write!(f, "sagemaker:ListMonitoringSchedules")
            }
            SagemakerActions::ListNotebookInstanceLifecycleConfigs => {
                write!(f, "sagemaker:ListNotebookInstanceLifecycleConfigs")
            }
            SagemakerActions::ListNotebookInstances => write!(f, "sagemaker:ListNotebookInstances"),
            SagemakerActions::ListOptimizationJobs => write!(f, "sagemaker:ListOptimizationJobs"),
            SagemakerActions::ListPartnerApps => write!(f, "sagemaker:ListPartnerApps"),
            SagemakerActions::ListPipelineExecutionSteps => {
                write!(f, "sagemaker:ListPipelineExecutionSteps")
            }
            SagemakerActions::ListPipelineExecutions => {
                write!(f, "sagemaker:ListPipelineExecutions")
            }
            SagemakerActions::ListPipelineParametersForExecution => {
                write!(f, "sagemaker:ListPipelineParametersForExecution")
            }
            SagemakerActions::ListPipelineVersions => write!(f, "sagemaker:ListPipelineVersions"),
            SagemakerActions::ListPipelines => write!(f, "sagemaker:ListPipelines"),
            SagemakerActions::ListProcessingJobs => write!(f, "sagemaker:ListProcessingJobs"),
            SagemakerActions::ListProjects => write!(f, "sagemaker:ListProjects"),
            SagemakerActions::ListResourceCatalogs => write!(f, "sagemaker:ListResourceCatalogs"),
            SagemakerActions::ListSharedModelEvents => write!(f, "sagemaker:ListSharedModelEvents"),
            SagemakerActions::ListSharedModelVersions => {
                write!(f, "sagemaker:ListSharedModelVersions")
            }
            SagemakerActions::ListSharedModels => write!(f, "sagemaker:ListSharedModels"),
            SagemakerActions::ListSpaces => write!(f, "sagemaker:ListSpaces"),
            SagemakerActions::ListStageDevices => write!(f, "sagemaker:ListStageDevices"),
            SagemakerActions::ListStudioLifecycleConfigs => {
                write!(f, "sagemaker:ListStudioLifecycleConfigs")
            }
            SagemakerActions::ListSubscribedWorkteams => {
                write!(f, "sagemaker:ListSubscribedWorkteams")
            }
            SagemakerActions::ListTags => write!(f, "sagemaker:ListTags"),
            SagemakerActions::ListTrainingJobs => write!(f, "sagemaker:ListTrainingJobs"),
            SagemakerActions::ListTrainingJobsForHyperParameterTuningJob => {
                write!(f, "sagemaker:ListTrainingJobsForHyperParameterTuningJob")
            }
            SagemakerActions::ListTrainingPlans => write!(f, "sagemaker:ListTrainingPlans"),
            SagemakerActions::ListTransformJobs => write!(f, "sagemaker:ListTransformJobs"),
            SagemakerActions::ListTrialComponents => write!(f, "sagemaker:ListTrialComponents"),
            SagemakerActions::ListTrials => write!(f, "sagemaker:ListTrials"),
            SagemakerActions::ListUltraServersByReservedCapacity => {
                write!(f, "sagemaker:ListUltraServersByReservedCapacity")
            }
            SagemakerActions::ListUserProfiles => write!(f, "sagemaker:ListUserProfiles"),
            SagemakerActions::ListWorkforces => write!(f, "sagemaker:ListWorkforces"),
            SagemakerActions::ListWorkteams => write!(f, "sagemaker:ListWorkteams"),
            SagemakerActions::PutLineageGroupPolicy => write!(f, "sagemaker:PutLineageGroupPolicy"),
            SagemakerActions::PutModelPackageGroupPolicy => {
                write!(f, "sagemaker:PutModelPackageGroupPolicy")
            }
            SagemakerActions::PutRecord => write!(f, "sagemaker:PutRecord"),
            SagemakerActions::PutResourcePolicy => write!(f, "sagemaker:PutResourcePolicy"),
            SagemakerActions::QueryLineage => write!(f, "sagemaker:QueryLineage"),
            SagemakerActions::RegisterDevices => write!(f, "sagemaker:RegisterDevices"),
            SagemakerActions::RenderUiTemplate => write!(f, "sagemaker:RenderUiTemplate"),
            SagemakerActions::RetryPipelineExecution => {
                write!(f, "sagemaker:RetryPipelineExecution")
            }
            SagemakerActions::Search => write!(f, "sagemaker:Search"),
            SagemakerActions::SearchTrainingPlanOfferings => {
                write!(f, "sagemaker:SearchTrainingPlanOfferings")
            }
            SagemakerActions::SendHeartbeat => write!(f, "sagemaker:SendHeartbeat"),
            SagemakerActions::SendPipelineExecutionStepFailure => {
                write!(f, "sagemaker:SendPipelineExecutionStepFailure")
            }
            SagemakerActions::SendPipelineExecutionStepSuccess => {
                write!(f, "sagemaker:SendPipelineExecutionStepSuccess")
            }
            SagemakerActions::SendSharedModelEvent => write!(f, "sagemaker:SendSharedModelEvent"),
            SagemakerActions::StartEdgeDeploymentStage => {
                write!(f, "sagemaker:StartEdgeDeploymentStage")
            }
            SagemakerActions::StartHumanLoop => write!(f, "sagemaker:StartHumanLoop"),
            SagemakerActions::StartInferenceExperiment => {
                write!(f, "sagemaker:StartInferenceExperiment")
            }
            SagemakerActions::StartMlflowTrackingServer => {
                write!(f, "sagemaker:StartMlflowTrackingServer")
            }
            SagemakerActions::StartMonitoringSchedule => {
                write!(f, "sagemaker:StartMonitoringSchedule")
            }
            SagemakerActions::StartNotebookInstance => write!(f, "sagemaker:StartNotebookInstance"),
            SagemakerActions::StartPipelineExecution => {
                write!(f, "sagemaker:StartPipelineExecution")
            }
            SagemakerActions::StartSession => write!(f, "sagemaker:StartSession"),
            SagemakerActions::StopAutoMlJob => write!(f, "sagemaker:StopAutoMLJob"),
            SagemakerActions::StopCompilationJob => write!(f, "sagemaker:StopCompilationJob"),
            SagemakerActions::StopEdgeDeploymentStage => {
                write!(f, "sagemaker:StopEdgeDeploymentStage")
            }
            SagemakerActions::StopEdgePackagingJob => write!(f, "sagemaker:StopEdgePackagingJob"),
            SagemakerActions::StopHumanLoop => write!(f, "sagemaker:StopHumanLoop"),
            SagemakerActions::StopHyperParameterTuningJob => {
                write!(f, "sagemaker:StopHyperParameterTuningJob")
            }
            SagemakerActions::StopInferenceExperiment => {
                write!(f, "sagemaker:StopInferenceExperiment")
            }
            SagemakerActions::StopInferenceRecommendationsJob => {
                write!(f, "sagemaker:StopInferenceRecommendationsJob")
            }
            SagemakerActions::StopLabelingJob => write!(f, "sagemaker:StopLabelingJob"),
            SagemakerActions::StopMlflowTrackingServer => {
                write!(f, "sagemaker:StopMlflowTrackingServer")
            }
            SagemakerActions::StopMonitoringSchedule => {
                write!(f, "sagemaker:StopMonitoringSchedule")
            }
            SagemakerActions::StopNotebookInstance => write!(f, "sagemaker:StopNotebookInstance"),
            SagemakerActions::StopOptimizationJob => write!(f, "sagemaker:StopOptimizationJob"),
            SagemakerActions::StopPipelineExecution => write!(f, "sagemaker:StopPipelineExecution"),
            SagemakerActions::StopProcessingJob => write!(f, "sagemaker:StopProcessingJob"),
            SagemakerActions::StopTrainingJob => write!(f, "sagemaker:StopTrainingJob"),
            SagemakerActions::StopTransformJob => write!(f, "sagemaker:StopTransformJob"),
            SagemakerActions::TrainHubModel => write!(f, "sagemaker:TrainHubModel"),
            SagemakerActions::UpdateAction => write!(f, "sagemaker:UpdateAction"),
            SagemakerActions::UpdateAppImageConfig => write!(f, "sagemaker:UpdateAppImageConfig"),
            SagemakerActions::UpdateArtifact => write!(f, "sagemaker:UpdateArtifact"),
            SagemakerActions::UpdateCluster => write!(f, "sagemaker:UpdateCluster"),
            SagemakerActions::UpdateClusterInference => {
                write!(f, "sagemaker:UpdateClusterInference")
            }
            SagemakerActions::UpdateClusterSchedulerConfig => {
                write!(f, "sagemaker:UpdateClusterSchedulerConfig")
            }
            SagemakerActions::UpdateClusterSoftware => write!(f, "sagemaker:UpdateClusterSoftware"),
            SagemakerActions::UpdateCodeRepository => write!(f, "sagemaker:UpdateCodeRepository"),
            SagemakerActions::UpdateComputeQuota => write!(f, "sagemaker:UpdateComputeQuota"),
            SagemakerActions::UpdateContext => write!(f, "sagemaker:UpdateContext"),
            SagemakerActions::UpdateDeviceFleet => write!(f, "sagemaker:UpdateDeviceFleet"),
            SagemakerActions::UpdateDevices => write!(f, "sagemaker:UpdateDevices"),
            SagemakerActions::UpdateDomain => write!(f, "sagemaker:UpdateDomain"),
            SagemakerActions::UpdateEndpoint => write!(f, "sagemaker:UpdateEndpoint"),
            SagemakerActions::UpdateEndpointWeightsAndCapacities => {
                write!(f, "sagemaker:UpdateEndpointWeightsAndCapacities")
            }
            SagemakerActions::UpdateExperiment => write!(f, "sagemaker:UpdateExperiment"),
            SagemakerActions::UpdateFeatureGroup => write!(f, "sagemaker:UpdateFeatureGroup"),
            SagemakerActions::UpdateFeatureMetadata => write!(f, "sagemaker:UpdateFeatureMetadata"),
            SagemakerActions::UpdateHub => write!(f, "sagemaker:UpdateHub"),
            SagemakerActions::UpdateHubContent => write!(f, "sagemaker:UpdateHubContent"),
            SagemakerActions::UpdateHubContentReference => {
                write!(f, "sagemaker:UpdateHubContentReference")
            }
            SagemakerActions::UpdateImage => write!(f, "sagemaker:UpdateImage"),
            SagemakerActions::UpdateImageVersion => write!(f, "sagemaker:UpdateImageVersion"),
            SagemakerActions::UpdateInferenceComponent => {
                write!(f, "sagemaker:UpdateInferenceComponent")
            }
            SagemakerActions::UpdateInferenceComponentRuntimeConfig => {
                write!(f, "sagemaker:UpdateInferenceComponentRuntimeConfig")
            }
            SagemakerActions::UpdateInferenceExperiment => {
                write!(f, "sagemaker:UpdateInferenceExperiment")
            }
            SagemakerActions::UpdateMlflowTrackingServer => {
                write!(f, "sagemaker:UpdateMlflowTrackingServer")
            }
            SagemakerActions::UpdateModelCard => write!(f, "sagemaker:UpdateModelCard"),
            SagemakerActions::UpdateModelPackage => write!(f, "sagemaker:UpdateModelPackage"),
            SagemakerActions::UpdateMonitoringAlert => write!(f, "sagemaker:UpdateMonitoringAlert"),
            SagemakerActions::UpdateMonitoringSchedule => {
                write!(f, "sagemaker:UpdateMonitoringSchedule")
            }
            SagemakerActions::UpdateNotebookInstance => {
                write!(f, "sagemaker:UpdateNotebookInstance")
            }
            SagemakerActions::UpdateNotebookInstanceLifecycleConfig => {
                write!(f, "sagemaker:UpdateNotebookInstanceLifecycleConfig")
            }
            SagemakerActions::UpdatePartnerApp => write!(f, "sagemaker:UpdatePartnerApp"),
            SagemakerActions::UpdatePipeline => write!(f, "sagemaker:UpdatePipeline"),
            SagemakerActions::UpdatePipelineExecution => {
                write!(f, "sagemaker:UpdatePipelineExecution")
            }
            SagemakerActions::UpdatePipelineVersion => write!(f, "sagemaker:UpdatePipelineVersion"),
            SagemakerActions::UpdateProject => write!(f, "sagemaker:UpdateProject"),
            SagemakerActions::UpdateSharedModel => write!(f, "sagemaker:UpdateSharedModel"),
            SagemakerActions::UpdateSpace => write!(f, "sagemaker:UpdateSpace"),
            SagemakerActions::UpdateTrainingJob => write!(f, "sagemaker:UpdateTrainingJob"),
            SagemakerActions::UpdateTrial => write!(f, "sagemaker:UpdateTrial"),
            SagemakerActions::UpdateTrialComponent => write!(f, "sagemaker:UpdateTrialComponent"),
            SagemakerActions::UpdateUserProfile => write!(f, "sagemaker:UpdateUserProfile"),
            SagemakerActions::UpdateWorkforce => write!(f, "sagemaker:UpdateWorkforce"),
            SagemakerActions::UpdateWorkteam => write!(f, "sagemaker:UpdateWorkteam"),
        }
    }
}
