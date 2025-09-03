// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum AppstudioActions {
    GetAccountStatus,
    GetEnablementJobStatus,
    StartEnablementJob,
    StartRollbackEnablementJob,
    StartTeamDeployment,
}
impl std::fmt::Display for AppstudioActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppstudioActions::GetAccountStatus => write!(f, "appstudio:GetAccountStatus"),
            AppstudioActions::GetEnablementJobStatus => {
                write!(f, "appstudio:GetEnablementJobStatus")
            }
            AppstudioActions::StartEnablementJob => write!(f, "appstudio:StartEnablementJob"),
            AppstudioActions::StartRollbackEnablementJob => {
                write!(f, "appstudio:StartRollbackEnablementJob")
            }
            AppstudioActions::StartTeamDeployment => write!(f, "appstudio:StartTeamDeployment"),
        }
    }
}
