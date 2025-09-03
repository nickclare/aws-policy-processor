// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum CodeguruProfilerActions {
    AddNotificationChannels,
    BatchGetFrameMetricData,
    ConfigureAgent,
    CreateProfilingGroup,
    DeleteProfilingGroup,
    DescribeProfilingGroup,
    GetFindingsReportAccountSummary,
    GetNotificationConfiguration,
    GetPolicy,
    GetProfile,
    GetRecommendations,
    ListFindingsReports,
    ListProfileTimes,
    ListProfilingGroups,
    ListTagsForResource,
    PostAgentProfile,
    PutPermission,
    RemoveNotificationChannel,
    RemovePermission,
    SubmitFeedback,
    TagResource,
    UntagResource,
    UpdateProfilingGroup,
}
impl std::fmt::Display for CodeguruProfilerActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CodeguruProfilerActions::AddNotificationChannels => {
                write!(f, "codeguru-profiler:AddNotificationChannels")
            }
            CodeguruProfilerActions::BatchGetFrameMetricData => {
                write!(f, "codeguru-profiler:BatchGetFrameMetricData")
            }
            CodeguruProfilerActions::ConfigureAgent => {
                write!(f, "codeguru-profiler:ConfigureAgent")
            }
            CodeguruProfilerActions::CreateProfilingGroup => {
                write!(f, "codeguru-profiler:CreateProfilingGroup")
            }
            CodeguruProfilerActions::DeleteProfilingGroup => {
                write!(f, "codeguru-profiler:DeleteProfilingGroup")
            }
            CodeguruProfilerActions::DescribeProfilingGroup => {
                write!(f, "codeguru-profiler:DescribeProfilingGroup")
            }
            CodeguruProfilerActions::GetFindingsReportAccountSummary => {
                write!(f, "codeguru-profiler:GetFindingsReportAccountSummary")
            }
            CodeguruProfilerActions::GetNotificationConfiguration => {
                write!(f, "codeguru-profiler:GetNotificationConfiguration")
            }
            CodeguruProfilerActions::GetPolicy => write!(f, "codeguru-profiler:GetPolicy"),
            CodeguruProfilerActions::GetProfile => write!(f, "codeguru-profiler:GetProfile"),
            CodeguruProfilerActions::GetRecommendations => {
                write!(f, "codeguru-profiler:GetRecommendations")
            }
            CodeguruProfilerActions::ListFindingsReports => {
                write!(f, "codeguru-profiler:ListFindingsReports")
            }
            CodeguruProfilerActions::ListProfileTimes => {
                write!(f, "codeguru-profiler:ListProfileTimes")
            }
            CodeguruProfilerActions::ListProfilingGroups => {
                write!(f, "codeguru-profiler:ListProfilingGroups")
            }
            CodeguruProfilerActions::ListTagsForResource => {
                write!(f, "codeguru-profiler:ListTagsForResource")
            }
            CodeguruProfilerActions::PostAgentProfile => {
                write!(f, "codeguru-profiler:PostAgentProfile")
            }
            CodeguruProfilerActions::PutPermission => write!(f, "codeguru-profiler:PutPermission"),
            CodeguruProfilerActions::RemoveNotificationChannel => {
                write!(f, "codeguru-profiler:RemoveNotificationChannel")
            }
            CodeguruProfilerActions::RemovePermission => {
                write!(f, "codeguru-profiler:RemovePermission")
            }
            CodeguruProfilerActions::SubmitFeedback => {
                write!(f, "codeguru-profiler:SubmitFeedback")
            }
            CodeguruProfilerActions::TagResource => write!(f, "codeguru-profiler:TagResource"),
            CodeguruProfilerActions::UntagResource => write!(f, "codeguru-profiler:UntagResource"),
            CodeguruProfilerActions::UpdateProfilingGroup => {
                write!(f, "codeguru-profiler:UpdateProfilingGroup")
            }
        }
    }
}
