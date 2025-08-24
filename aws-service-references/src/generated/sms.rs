// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum SmsActions {
    CreateApp,
    CreateReplicationJob,
    DeleteApp,
    DeleteAppLaunchConfiguration,
    DeleteAppReplicationConfiguration,
    DeleteAppValidationConfiguration,
    DeleteReplicationJob,
    DeleteServerCatalog,
    DisassociateConnector,
    GenerateChangeSet,
    GenerateTemplate,
    GetApp,
    GetAppLaunchConfiguration,
    GetAppReplicationConfiguration,
    GetAppValidationConfiguration,
    GetAppValidationOutput,
    GetConnectors,
    GetMessages,
    GetReplicationJobs,
    GetReplicationRuns,
    GetServers,
    ImportAppCatalog,
    ImportServerCatalog,
    LaunchApp,
    ListApps,
    NotifyAppValidationOutput,
    PutAppLaunchConfiguration,
    PutAppReplicationConfiguration,
    PutAppValidationConfiguration,
    SendMessage,
    StartAppReplication,
    StartOnDemandAppReplication,
    StartOnDemandReplicationRun,
    StopAppReplication,
    TerminateApp,
    UpdateApp,
    UpdateReplicationJob,
}
impl std::fmt::Display for SmsActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SmsActions::CreateApp => write!(f, "sms:CreateApp"),
            SmsActions::CreateReplicationJob => write!(f, "sms:CreateReplicationJob"),
            SmsActions::DeleteApp => write!(f, "sms:DeleteApp"),
            SmsActions::DeleteAppLaunchConfiguration => {
                write!(f, "sms:DeleteAppLaunchConfiguration")
            }
            SmsActions::DeleteAppReplicationConfiguration => {
                write!(f, "sms:DeleteAppReplicationConfiguration")
            }
            SmsActions::DeleteAppValidationConfiguration => {
                write!(f, "sms:DeleteAppValidationConfiguration")
            }
            SmsActions::DeleteReplicationJob => write!(f, "sms:DeleteReplicationJob"),
            SmsActions::DeleteServerCatalog => write!(f, "sms:DeleteServerCatalog"),
            SmsActions::DisassociateConnector => write!(f, "sms:DisassociateConnector"),
            SmsActions::GenerateChangeSet => write!(f, "sms:GenerateChangeSet"),
            SmsActions::GenerateTemplate => write!(f, "sms:GenerateTemplate"),
            SmsActions::GetApp => write!(f, "sms:GetApp"),
            SmsActions::GetAppLaunchConfiguration => write!(f, "sms:GetAppLaunchConfiguration"),
            SmsActions::GetAppReplicationConfiguration => {
                write!(f, "sms:GetAppReplicationConfiguration")
            }
            SmsActions::GetAppValidationConfiguration => {
                write!(f, "sms:GetAppValidationConfiguration")
            }
            SmsActions::GetAppValidationOutput => write!(f, "sms:GetAppValidationOutput"),
            SmsActions::GetConnectors => write!(f, "sms:GetConnectors"),
            SmsActions::GetMessages => write!(f, "sms:GetMessages"),
            SmsActions::GetReplicationJobs => write!(f, "sms:GetReplicationJobs"),
            SmsActions::GetReplicationRuns => write!(f, "sms:GetReplicationRuns"),
            SmsActions::GetServers => write!(f, "sms:GetServers"),
            SmsActions::ImportAppCatalog => write!(f, "sms:ImportAppCatalog"),
            SmsActions::ImportServerCatalog => write!(f, "sms:ImportServerCatalog"),
            SmsActions::LaunchApp => write!(f, "sms:LaunchApp"),
            SmsActions::ListApps => write!(f, "sms:ListApps"),
            SmsActions::NotifyAppValidationOutput => write!(f, "sms:NotifyAppValidationOutput"),
            SmsActions::PutAppLaunchConfiguration => write!(f, "sms:PutAppLaunchConfiguration"),
            SmsActions::PutAppReplicationConfiguration => {
                write!(f, "sms:PutAppReplicationConfiguration")
            }
            SmsActions::PutAppValidationConfiguration => {
                write!(f, "sms:PutAppValidationConfiguration")
            }
            SmsActions::SendMessage => write!(f, "sms:SendMessage"),
            SmsActions::StartAppReplication => write!(f, "sms:StartAppReplication"),
            SmsActions::StartOnDemandAppReplication => write!(f, "sms:StartOnDemandAppReplication"),
            SmsActions::StartOnDemandReplicationRun => write!(f, "sms:StartOnDemandReplicationRun"),
            SmsActions::StopAppReplication => write!(f, "sms:StopAppReplication"),
            SmsActions::TerminateApp => write!(f, "sms:TerminateApp"),
            SmsActions::UpdateApp => write!(f, "sms:UpdateApp"),
            SmsActions::UpdateReplicationJob => write!(f, "sms:UpdateReplicationJob"),
        }
    }
}
