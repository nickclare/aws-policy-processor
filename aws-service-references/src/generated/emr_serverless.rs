// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum EmrServerlessActions {
    AccessInteractiveEndpoints,
    AccessLivyEndpoints,
    AccessSystemProfileLogs,
    CancelJobRun,
    CreateApplication,
    DeleteApplication,
    GetApplication,
    GetDashboardForJobRun,
    GetJobRun,
    ListApplications,
    ListJobRunAttempts,
    ListJobRuns,
    ListTagsForResource,
    StartApplication,
    StartJobRun,
    StopApplication,
    TagResource,
    UntagResource,
    UpdateApplication,
}
impl std::fmt::Display for EmrServerlessActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EmrServerlessActions::AccessInteractiveEndpoints => {
                write!(f, "emr-serverless:AccessInteractiveEndpoints")
            }
            EmrServerlessActions::AccessLivyEndpoints => {
                write!(f, "emr-serverless:AccessLivyEndpoints")
            }
            EmrServerlessActions::AccessSystemProfileLogs => {
                write!(f, "emr-serverless:AccessSystemProfileLogs")
            }
            EmrServerlessActions::CancelJobRun => write!(f, "emr-serverless:CancelJobRun"),
            EmrServerlessActions::CreateApplication => {
                write!(f, "emr-serverless:CreateApplication")
            }
            EmrServerlessActions::DeleteApplication => {
                write!(f, "emr-serverless:DeleteApplication")
            }
            EmrServerlessActions::GetApplication => write!(f, "emr-serverless:GetApplication"),
            EmrServerlessActions::GetDashboardForJobRun => {
                write!(f, "emr-serverless:GetDashboardForJobRun")
            }
            EmrServerlessActions::GetJobRun => write!(f, "emr-serverless:GetJobRun"),
            EmrServerlessActions::ListApplications => write!(f, "emr-serverless:ListApplications"),
            EmrServerlessActions::ListJobRunAttempts => {
                write!(f, "emr-serverless:ListJobRunAttempts")
            }
            EmrServerlessActions::ListJobRuns => write!(f, "emr-serverless:ListJobRuns"),
            EmrServerlessActions::ListTagsForResource => {
                write!(f, "emr-serverless:ListTagsForResource")
            }
            EmrServerlessActions::StartApplication => write!(f, "emr-serverless:StartApplication"),
            EmrServerlessActions::StartJobRun => write!(f, "emr-serverless:StartJobRun"),
            EmrServerlessActions::StopApplication => write!(f, "emr-serverless:StopApplication"),
            EmrServerlessActions::TagResource => write!(f, "emr-serverless:TagResource"),
            EmrServerlessActions::UntagResource => write!(f, "emr-serverless:UntagResource"),
            EmrServerlessActions::UpdateApplication => {
                write!(f, "emr-serverless:UpdateApplication")
            }
        }
    }
}
