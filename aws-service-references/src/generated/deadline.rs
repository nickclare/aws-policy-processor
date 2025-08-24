// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum DeadlineActions {
    AssociateMemberToFarm,
    AssociateMemberToFleet,
    AssociateMemberToJob,
    AssociateMemberToQueue,
    AssumeFleetRoleForRead,
    AssumeFleetRoleForWorker,
    AssumeQueueRoleForRead,
    AssumeQueueRoleForUser,
    AssumeQueueRoleForWorker,
    BatchGetJobEntity,
    CopyJobTemplate,
    CreateBudget,
    CreateFarm,
    CreateFleet,
    CreateJob,
    CreateLicenseEndpoint,
    CreateLimit,
    CreateMonitor,
    CreateQueue,
    CreateQueueEnvironment,
    CreateQueueFleetAssociation,
    CreateQueueLimitAssociation,
    CreateStorageProfile,
    CreateWorker,
    DeleteBudget,
    DeleteFarm,
    DeleteFleet,
    DeleteLicenseEndpoint,
    DeleteLimit,
    DeleteMeteredProduct,
    DeleteMonitor,
    DeleteQueue,
    DeleteQueueEnvironment,
    DeleteQueueFleetAssociation,
    DeleteQueueLimitAssociation,
    DeleteStorageProfile,
    DeleteWorker,
    DisassociateMemberFromFarm,
    DisassociateMemberFromFleet,
    DisassociateMemberFromJob,
    DisassociateMemberFromQueue,
    GetApplicationVersion,
    GetBudget,
    GetFarm,
    GetFleet,
    GetJob,
    GetJobTemplate,
    GetLicenseEndpoint,
    GetLimit,
    GetMonitor,
    GetQueue,
    GetQueueEnvironment,
    GetQueueFleetAssociation,
    GetQueueLimitAssociation,
    GetSession,
    GetSessionAction,
    GetSessionsStatisticsAggregation,
    GetStep,
    GetStorageProfile,
    GetStorageProfileForQueue,
    GetTask,
    GetWorker,
    ListAvailableMeteredProducts,
    ListBudgets,
    ListFarmMembers,
    ListFarms,
    ListFleetMembers,
    ListFleets,
    ListJobMembers,
    ListJobParameterDefinitions,
    ListJobs,
    ListLicenseEndpoints,
    ListLimits,
    ListMeteredProducts,
    ListMonitors,
    ListQueueEnvironments,
    ListQueueFleetAssociations,
    ListQueueLimitAssociations,
    ListQueueMembers,
    ListQueues,
    ListSessionActions,
    ListSessions,
    ListSessionsForWorker,
    ListStepConsumers,
    ListStepDependencies,
    ListSteps,
    ListStorageProfiles,
    ListStorageProfilesForQueue,
    ListTagsForResource,
    ListTasks,
    ListWorkers,
    PutMeteredProduct,
    SearchJobs,
    SearchSteps,
    SearchTasks,
    SearchWorkers,
    StartSessionsStatisticsAggregation,
    TagResource,
    UntagResource,
    UpdateBudget,
    UpdateFarm,
    UpdateFleet,
    UpdateJob,
    UpdateLimit,
    UpdateMonitor,
    UpdateQueue,
    UpdateQueueEnvironment,
    UpdateQueueFleetAssociation,
    UpdateQueueLimitAssociation,
    UpdateSession,
    UpdateStep,
    UpdateStorageProfile,
    UpdateTask,
    UpdateWorker,
    UpdateWorkerSchedule,
}
impl std::fmt::Display for DeadlineActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeadlineActions::AssociateMemberToFarm => write!(f, "deadline:AssociateMemberToFarm"),
            DeadlineActions::AssociateMemberToFleet => write!(f, "deadline:AssociateMemberToFleet"),
            DeadlineActions::AssociateMemberToJob => write!(f, "deadline:AssociateMemberToJob"),
            DeadlineActions::AssociateMemberToQueue => write!(f, "deadline:AssociateMemberToQueue"),
            DeadlineActions::AssumeFleetRoleForRead => write!(f, "deadline:AssumeFleetRoleForRead"),
            DeadlineActions::AssumeFleetRoleForWorker => {
                write!(f, "deadline:AssumeFleetRoleForWorker")
            }
            DeadlineActions::AssumeQueueRoleForRead => write!(f, "deadline:AssumeQueueRoleForRead"),
            DeadlineActions::AssumeQueueRoleForUser => write!(f, "deadline:AssumeQueueRoleForUser"),
            DeadlineActions::AssumeQueueRoleForWorker => {
                write!(f, "deadline:AssumeQueueRoleForWorker")
            }
            DeadlineActions::BatchGetJobEntity => write!(f, "deadline:BatchGetJobEntity"),
            DeadlineActions::CopyJobTemplate => write!(f, "deadline:CopyJobTemplate"),
            DeadlineActions::CreateBudget => write!(f, "deadline:CreateBudget"),
            DeadlineActions::CreateFarm => write!(f, "deadline:CreateFarm"),
            DeadlineActions::CreateFleet => write!(f, "deadline:CreateFleet"),
            DeadlineActions::CreateJob => write!(f, "deadline:CreateJob"),
            DeadlineActions::CreateLicenseEndpoint => write!(f, "deadline:CreateLicenseEndpoint"),
            DeadlineActions::CreateLimit => write!(f, "deadline:CreateLimit"),
            DeadlineActions::CreateMonitor => write!(f, "deadline:CreateMonitor"),
            DeadlineActions::CreateQueue => write!(f, "deadline:CreateQueue"),
            DeadlineActions::CreateQueueEnvironment => write!(f, "deadline:CreateQueueEnvironment"),
            DeadlineActions::CreateQueueFleetAssociation => {
                write!(f, "deadline:CreateQueueFleetAssociation")
            }
            DeadlineActions::CreateQueueLimitAssociation => {
                write!(f, "deadline:CreateQueueLimitAssociation")
            }
            DeadlineActions::CreateStorageProfile => write!(f, "deadline:CreateStorageProfile"),
            DeadlineActions::CreateWorker => write!(f, "deadline:CreateWorker"),
            DeadlineActions::DeleteBudget => write!(f, "deadline:DeleteBudget"),
            DeadlineActions::DeleteFarm => write!(f, "deadline:DeleteFarm"),
            DeadlineActions::DeleteFleet => write!(f, "deadline:DeleteFleet"),
            DeadlineActions::DeleteLicenseEndpoint => write!(f, "deadline:DeleteLicenseEndpoint"),
            DeadlineActions::DeleteLimit => write!(f, "deadline:DeleteLimit"),
            DeadlineActions::DeleteMeteredProduct => write!(f, "deadline:DeleteMeteredProduct"),
            DeadlineActions::DeleteMonitor => write!(f, "deadline:DeleteMonitor"),
            DeadlineActions::DeleteQueue => write!(f, "deadline:DeleteQueue"),
            DeadlineActions::DeleteQueueEnvironment => write!(f, "deadline:DeleteQueueEnvironment"),
            DeadlineActions::DeleteQueueFleetAssociation => {
                write!(f, "deadline:DeleteQueueFleetAssociation")
            }
            DeadlineActions::DeleteQueueLimitAssociation => {
                write!(f, "deadline:DeleteQueueLimitAssociation")
            }
            DeadlineActions::DeleteStorageProfile => write!(f, "deadline:DeleteStorageProfile"),
            DeadlineActions::DeleteWorker => write!(f, "deadline:DeleteWorker"),
            DeadlineActions::DisassociateMemberFromFarm => {
                write!(f, "deadline:DisassociateMemberFromFarm")
            }
            DeadlineActions::DisassociateMemberFromFleet => {
                write!(f, "deadline:DisassociateMemberFromFleet")
            }
            DeadlineActions::DisassociateMemberFromJob => {
                write!(f, "deadline:DisassociateMemberFromJob")
            }
            DeadlineActions::DisassociateMemberFromQueue => {
                write!(f, "deadline:DisassociateMemberFromQueue")
            }
            DeadlineActions::GetApplicationVersion => write!(f, "deadline:GetApplicationVersion"),
            DeadlineActions::GetBudget => write!(f, "deadline:GetBudget"),
            DeadlineActions::GetFarm => write!(f, "deadline:GetFarm"),
            DeadlineActions::GetFleet => write!(f, "deadline:GetFleet"),
            DeadlineActions::GetJob => write!(f, "deadline:GetJob"),
            DeadlineActions::GetJobTemplate => write!(f, "deadline:GetJobTemplate"),
            DeadlineActions::GetLicenseEndpoint => write!(f, "deadline:GetLicenseEndpoint"),
            DeadlineActions::GetLimit => write!(f, "deadline:GetLimit"),
            DeadlineActions::GetMonitor => write!(f, "deadline:GetMonitor"),
            DeadlineActions::GetQueue => write!(f, "deadline:GetQueue"),
            DeadlineActions::GetQueueEnvironment => write!(f, "deadline:GetQueueEnvironment"),
            DeadlineActions::GetQueueFleetAssociation => {
                write!(f, "deadline:GetQueueFleetAssociation")
            }
            DeadlineActions::GetQueueLimitAssociation => {
                write!(f, "deadline:GetQueueLimitAssociation")
            }
            DeadlineActions::GetSession => write!(f, "deadline:GetSession"),
            DeadlineActions::GetSessionAction => write!(f, "deadline:GetSessionAction"),
            DeadlineActions::GetSessionsStatisticsAggregation => {
                write!(f, "deadline:GetSessionsStatisticsAggregation")
            }
            DeadlineActions::GetStep => write!(f, "deadline:GetStep"),
            DeadlineActions::GetStorageProfile => write!(f, "deadline:GetStorageProfile"),
            DeadlineActions::GetStorageProfileForQueue => {
                write!(f, "deadline:GetStorageProfileForQueue")
            }
            DeadlineActions::GetTask => write!(f, "deadline:GetTask"),
            DeadlineActions::GetWorker => write!(f, "deadline:GetWorker"),
            DeadlineActions::ListAvailableMeteredProducts => {
                write!(f, "deadline:ListAvailableMeteredProducts")
            }
            DeadlineActions::ListBudgets => write!(f, "deadline:ListBudgets"),
            DeadlineActions::ListFarmMembers => write!(f, "deadline:ListFarmMembers"),
            DeadlineActions::ListFarms => write!(f, "deadline:ListFarms"),
            DeadlineActions::ListFleetMembers => write!(f, "deadline:ListFleetMembers"),
            DeadlineActions::ListFleets => write!(f, "deadline:ListFleets"),
            DeadlineActions::ListJobMembers => write!(f, "deadline:ListJobMembers"),
            DeadlineActions::ListJobParameterDefinitions => {
                write!(f, "deadline:ListJobParameterDefinitions")
            }
            DeadlineActions::ListJobs => write!(f, "deadline:ListJobs"),
            DeadlineActions::ListLicenseEndpoints => write!(f, "deadline:ListLicenseEndpoints"),
            DeadlineActions::ListLimits => write!(f, "deadline:ListLimits"),
            DeadlineActions::ListMeteredProducts => write!(f, "deadline:ListMeteredProducts"),
            DeadlineActions::ListMonitors => write!(f, "deadline:ListMonitors"),
            DeadlineActions::ListQueueEnvironments => write!(f, "deadline:ListQueueEnvironments"),
            DeadlineActions::ListQueueFleetAssociations => {
                write!(f, "deadline:ListQueueFleetAssociations")
            }
            DeadlineActions::ListQueueLimitAssociations => {
                write!(f, "deadline:ListQueueLimitAssociations")
            }
            DeadlineActions::ListQueueMembers => write!(f, "deadline:ListQueueMembers"),
            DeadlineActions::ListQueues => write!(f, "deadline:ListQueues"),
            DeadlineActions::ListSessionActions => write!(f, "deadline:ListSessionActions"),
            DeadlineActions::ListSessions => write!(f, "deadline:ListSessions"),
            DeadlineActions::ListSessionsForWorker => write!(f, "deadline:ListSessionsForWorker"),
            DeadlineActions::ListStepConsumers => write!(f, "deadline:ListStepConsumers"),
            DeadlineActions::ListStepDependencies => write!(f, "deadline:ListStepDependencies"),
            DeadlineActions::ListSteps => write!(f, "deadline:ListSteps"),
            DeadlineActions::ListStorageProfiles => write!(f, "deadline:ListStorageProfiles"),
            DeadlineActions::ListStorageProfilesForQueue => {
                write!(f, "deadline:ListStorageProfilesForQueue")
            }
            DeadlineActions::ListTagsForResource => write!(f, "deadline:ListTagsForResource"),
            DeadlineActions::ListTasks => write!(f, "deadline:ListTasks"),
            DeadlineActions::ListWorkers => write!(f, "deadline:ListWorkers"),
            DeadlineActions::PutMeteredProduct => write!(f, "deadline:PutMeteredProduct"),
            DeadlineActions::SearchJobs => write!(f, "deadline:SearchJobs"),
            DeadlineActions::SearchSteps => write!(f, "deadline:SearchSteps"),
            DeadlineActions::SearchTasks => write!(f, "deadline:SearchTasks"),
            DeadlineActions::SearchWorkers => write!(f, "deadline:SearchWorkers"),
            DeadlineActions::StartSessionsStatisticsAggregation => {
                write!(f, "deadline:StartSessionsStatisticsAggregation")
            }
            DeadlineActions::TagResource => write!(f, "deadline:TagResource"),
            DeadlineActions::UntagResource => write!(f, "deadline:UntagResource"),
            DeadlineActions::UpdateBudget => write!(f, "deadline:UpdateBudget"),
            DeadlineActions::UpdateFarm => write!(f, "deadline:UpdateFarm"),
            DeadlineActions::UpdateFleet => write!(f, "deadline:UpdateFleet"),
            DeadlineActions::UpdateJob => write!(f, "deadline:UpdateJob"),
            DeadlineActions::UpdateLimit => write!(f, "deadline:UpdateLimit"),
            DeadlineActions::UpdateMonitor => write!(f, "deadline:UpdateMonitor"),
            DeadlineActions::UpdateQueue => write!(f, "deadline:UpdateQueue"),
            DeadlineActions::UpdateQueueEnvironment => write!(f, "deadline:UpdateQueueEnvironment"),
            DeadlineActions::UpdateQueueFleetAssociation => {
                write!(f, "deadline:UpdateQueueFleetAssociation")
            }
            DeadlineActions::UpdateQueueLimitAssociation => {
                write!(f, "deadline:UpdateQueueLimitAssociation")
            }
            DeadlineActions::UpdateSession => write!(f, "deadline:UpdateSession"),
            DeadlineActions::UpdateStep => write!(f, "deadline:UpdateStep"),
            DeadlineActions::UpdateStorageProfile => write!(f, "deadline:UpdateStorageProfile"),
            DeadlineActions::UpdateTask => write!(f, "deadline:UpdateTask"),
            DeadlineActions::UpdateWorker => write!(f, "deadline:UpdateWorker"),
            DeadlineActions::UpdateWorkerSchedule => write!(f, "deadline:UpdateWorkerSchedule"),
        }
    }
}
