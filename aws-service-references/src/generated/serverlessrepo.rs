// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum ServerlessrepoActions {
    CreateApplication,
    CreateApplicationVersion,
    CreateCloudFormationChangeSet,
    CreateCloudFormationTemplate,
    DeleteApplication,
    GetApplication,
    GetApplicationPolicy,
    GetCloudFormationTemplate,
    ListApplicationDependencies,
    ListApplicationVersions,
    ListApplications,
    PutApplicationPolicy,
    SearchApplications,
    UnshareApplication,
    UpdateApplication,
}
impl std::fmt::Display for ServerlessrepoActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServerlessrepoActions::CreateApplication => {
                write!(f, "serverlessrepo:CreateApplication")
            }
            ServerlessrepoActions::CreateApplicationVersion => {
                write!(f, "serverlessrepo:CreateApplicationVersion")
            }
            ServerlessrepoActions::CreateCloudFormationChangeSet => {
                write!(f, "serverlessrepo:CreateCloudFormationChangeSet")
            }
            ServerlessrepoActions::CreateCloudFormationTemplate => {
                write!(f, "serverlessrepo:CreateCloudFormationTemplate")
            }
            ServerlessrepoActions::DeleteApplication => {
                write!(f, "serverlessrepo:DeleteApplication")
            }
            ServerlessrepoActions::GetApplication => write!(f, "serverlessrepo:GetApplication"),
            ServerlessrepoActions::GetApplicationPolicy => {
                write!(f, "serverlessrepo:GetApplicationPolicy")
            }
            ServerlessrepoActions::GetCloudFormationTemplate => {
                write!(f, "serverlessrepo:GetCloudFormationTemplate")
            }
            ServerlessrepoActions::ListApplicationDependencies => {
                write!(f, "serverlessrepo:ListApplicationDependencies")
            }
            ServerlessrepoActions::ListApplicationVersions => {
                write!(f, "serverlessrepo:ListApplicationVersions")
            }
            ServerlessrepoActions::ListApplications => write!(f, "serverlessrepo:ListApplications"),
            ServerlessrepoActions::PutApplicationPolicy => {
                write!(f, "serverlessrepo:PutApplicationPolicy")
            }
            ServerlessrepoActions::SearchApplications => {
                write!(f, "serverlessrepo:SearchApplications")
            }
            ServerlessrepoActions::UnshareApplication => {
                write!(f, "serverlessrepo:UnshareApplication")
            }
            ServerlessrepoActions::UpdateApplication => {
                write!(f, "serverlessrepo:UpdateApplication")
            }
        }
    }
}
