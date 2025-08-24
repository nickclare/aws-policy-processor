// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum SsmSapActions {
    BackupDatabase,
    DeleteResourcePermission,
    DeregisterApplication,
    GetApplication,
    GetComponent,
    GetDatabase,
    GetOperation,
    GetResourcePermission,
    ListApplications,
    ListComponents,
    ListDatabases,
    ListOperationEvents,
    ListOperations,
    ListTagsForResource,
    PutResourcePermission,
    RegisterApplication,
    RestoreDatabase,
    StartApplication,
    StartApplicationRefresh,
    StopApplication,
    TagResource,
    UntagResource,
    UpdateApplicationSettings,
    UpdateHanaBackupSettings,
}
impl std::fmt::Display for SsmSapActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SsmSapActions::BackupDatabase => write!(f, "ssm-sap:BackupDatabase"),
            SsmSapActions::DeleteResourcePermission => {
                write!(f, "ssm-sap:DeleteResourcePermission")
            }
            SsmSapActions::DeregisterApplication => write!(f, "ssm-sap:DeregisterApplication"),
            SsmSapActions::GetApplication => write!(f, "ssm-sap:GetApplication"),
            SsmSapActions::GetComponent => write!(f, "ssm-sap:GetComponent"),
            SsmSapActions::GetDatabase => write!(f, "ssm-sap:GetDatabase"),
            SsmSapActions::GetOperation => write!(f, "ssm-sap:GetOperation"),
            SsmSapActions::GetResourcePermission => write!(f, "ssm-sap:GetResourcePermission"),
            SsmSapActions::ListApplications => write!(f, "ssm-sap:ListApplications"),
            SsmSapActions::ListComponents => write!(f, "ssm-sap:ListComponents"),
            SsmSapActions::ListDatabases => write!(f, "ssm-sap:ListDatabases"),
            SsmSapActions::ListOperationEvents => write!(f, "ssm-sap:ListOperationEvents"),
            SsmSapActions::ListOperations => write!(f, "ssm-sap:ListOperations"),
            SsmSapActions::ListTagsForResource => write!(f, "ssm-sap:ListTagsForResource"),
            SsmSapActions::PutResourcePermission => write!(f, "ssm-sap:PutResourcePermission"),
            SsmSapActions::RegisterApplication => write!(f, "ssm-sap:RegisterApplication"),
            SsmSapActions::RestoreDatabase => write!(f, "ssm-sap:RestoreDatabase"),
            SsmSapActions::StartApplication => write!(f, "ssm-sap:StartApplication"),
            SsmSapActions::StartApplicationRefresh => write!(f, "ssm-sap:StartApplicationRefresh"),
            SsmSapActions::StopApplication => write!(f, "ssm-sap:StopApplication"),
            SsmSapActions::TagResource => write!(f, "ssm-sap:TagResource"),
            SsmSapActions::UntagResource => write!(f, "ssm-sap:UntagResource"),
            SsmSapActions::UpdateApplicationSettings => {
                write!(f, "ssm-sap:UpdateApplicationSettings")
            }
            SsmSapActions::UpdateHanaBackupSettings => {
                write!(f, "ssm-sap:UpdateHANABackupSettings")
            }
        }
    }
}
