// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum BatchActions {
    CancelJob,
    CreateComputeEnvironment,
    CreateConsumableResource,
    CreateJobQueue,
    CreateSchedulingPolicy,
    CreateServiceEnvironment,
    DeleteComputeEnvironment,
    DeleteConsumableResource,
    DeleteJobQueue,
    DeleteSchedulingPolicy,
    DeleteServiceEnvironment,
    DeregisterJobDefinition,
    DescribeComputeEnvironments,
    DescribeConsumableResource,
    DescribeJobDefinitions,
    DescribeJobQueues,
    DescribeJobs,
    DescribeSchedulingPolicies,
    DescribeServiceEnvironments,
    DescribeServiceJob,
    GetJobQueueSnapshot,
    ListConsumableResources,
    ListJobs,
    ListJobsByConsumableResource,
    ListSchedulingPolicies,
    ListServiceJobs,
    ListTagsForResource,
    RegisterJobDefinition,
    SubmitJob,
    SubmitServiceJob,
    TagResource,
    TerminateJob,
    TerminateServiceJob,
    UntagResource,
    UpdateComputeEnvironment,
    UpdateConsumableResource,
    UpdateJobQueue,
    UpdateSchedulingPolicy,
    UpdateServiceEnvironment,
}
impl std::fmt::Display for BatchActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BatchActions::CancelJob => write!(f, "batch:CancelJob"),
            BatchActions::CreateComputeEnvironment => write!(f, "batch:CreateComputeEnvironment"),
            BatchActions::CreateConsumableResource => write!(f, "batch:CreateConsumableResource"),
            BatchActions::CreateJobQueue => write!(f, "batch:CreateJobQueue"),
            BatchActions::CreateSchedulingPolicy => write!(f, "batch:CreateSchedulingPolicy"),
            BatchActions::CreateServiceEnvironment => write!(f, "batch:CreateServiceEnvironment"),
            BatchActions::DeleteComputeEnvironment => write!(f, "batch:DeleteComputeEnvironment"),
            BatchActions::DeleteConsumableResource => write!(f, "batch:DeleteConsumableResource"),
            BatchActions::DeleteJobQueue => write!(f, "batch:DeleteJobQueue"),
            BatchActions::DeleteSchedulingPolicy => write!(f, "batch:DeleteSchedulingPolicy"),
            BatchActions::DeleteServiceEnvironment => write!(f, "batch:DeleteServiceEnvironment"),
            BatchActions::DeregisterJobDefinition => write!(f, "batch:DeregisterJobDefinition"),
            BatchActions::DescribeComputeEnvironments => {
                write!(f, "batch:DescribeComputeEnvironments")
            }
            BatchActions::DescribeConsumableResource => {
                write!(f, "batch:DescribeConsumableResource")
            }
            BatchActions::DescribeJobDefinitions => write!(f, "batch:DescribeJobDefinitions"),
            BatchActions::DescribeJobQueues => write!(f, "batch:DescribeJobQueues"),
            BatchActions::DescribeJobs => write!(f, "batch:DescribeJobs"),
            BatchActions::DescribeSchedulingPolicies => {
                write!(f, "batch:DescribeSchedulingPolicies")
            }
            BatchActions::DescribeServiceEnvironments => {
                write!(f, "batch:DescribeServiceEnvironments")
            }
            BatchActions::DescribeServiceJob => write!(f, "batch:DescribeServiceJob"),
            BatchActions::GetJobQueueSnapshot => write!(f, "batch:GetJobQueueSnapshot"),
            BatchActions::ListConsumableResources => write!(f, "batch:ListConsumableResources"),
            BatchActions::ListJobs => write!(f, "batch:ListJobs"),
            BatchActions::ListJobsByConsumableResource => {
                write!(f, "batch:ListJobsByConsumableResource")
            }
            BatchActions::ListSchedulingPolicies => write!(f, "batch:ListSchedulingPolicies"),
            BatchActions::ListServiceJobs => write!(f, "batch:ListServiceJobs"),
            BatchActions::ListTagsForResource => write!(f, "batch:ListTagsForResource"),
            BatchActions::RegisterJobDefinition => write!(f, "batch:RegisterJobDefinition"),
            BatchActions::SubmitJob => write!(f, "batch:SubmitJob"),
            BatchActions::SubmitServiceJob => write!(f, "batch:SubmitServiceJob"),
            BatchActions::TagResource => write!(f, "batch:TagResource"),
            BatchActions::TerminateJob => write!(f, "batch:TerminateJob"),
            BatchActions::TerminateServiceJob => write!(f, "batch:TerminateServiceJob"),
            BatchActions::UntagResource => write!(f, "batch:UntagResource"),
            BatchActions::UpdateComputeEnvironment => write!(f, "batch:UpdateComputeEnvironment"),
            BatchActions::UpdateConsumableResource => write!(f, "batch:UpdateConsumableResource"),
            BatchActions::UpdateJobQueue => write!(f, "batch:UpdateJobQueue"),
            BatchActions::UpdateSchedulingPolicy => write!(f, "batch:UpdateSchedulingPolicy"),
            BatchActions::UpdateServiceEnvironment => write!(f, "batch:UpdateServiceEnvironment"),
        }
    }
}
