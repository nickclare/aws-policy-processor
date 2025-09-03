// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum CloudshellActions {
    ApproveCommand,
    CreateEnvironment,
    CreateSession,
    DeleteEnvironment,
    DescribeEnvironments,
    GetEnvironmentStatus,
    GetFileDownloadUrls,
    GetFileUploadUrls,
    PutCredentials,
    StartEnvironment,
    StopEnvironment,
}
impl std::fmt::Display for CloudshellActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CloudshellActions::ApproveCommand => write!(f, "cloudshell:ApproveCommand"),
            CloudshellActions::CreateEnvironment => write!(f, "cloudshell:CreateEnvironment"),
            CloudshellActions::CreateSession => write!(f, "cloudshell:CreateSession"),
            CloudshellActions::DeleteEnvironment => write!(f, "cloudshell:DeleteEnvironment"),
            CloudshellActions::DescribeEnvironments => write!(f, "cloudshell:DescribeEnvironments"),
            CloudshellActions::GetEnvironmentStatus => write!(f, "cloudshell:GetEnvironmentStatus"),
            CloudshellActions::GetFileDownloadUrls => write!(f, "cloudshell:GetFileDownloadUrls"),
            CloudshellActions::GetFileUploadUrls => write!(f, "cloudshell:GetFileUploadUrls"),
            CloudshellActions::PutCredentials => write!(f, "cloudshell:PutCredentials"),
            CloudshellActions::StartEnvironment => write!(f, "cloudshell:StartEnvironment"),
            CloudshellActions::StopEnvironment => write!(f, "cloudshell:StopEnvironment"),
        }
    }
}
