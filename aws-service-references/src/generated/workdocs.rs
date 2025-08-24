// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum WorkdocsActions {
    AbortDocumentVersionUpload,
    ActivateUser,
    AddNotificationPermissions,
    AddResourcePermissions,
    AddUserToGroup,
    CheckAlias,
    CreateComment,
    CreateCustomMetadata,
    CreateFolder,
    CreateInstance,
    CreateLabels,
    CreateNotificationSubscription,
    CreateUser,
    DeactivateUser,
    DeleteComment,
    DeleteCustomMetadata,
    DeleteDocument,
    DeleteDocumentVersion,
    DeleteFolder,
    DeleteFolderContents,
    DeleteInstance,
    DeleteLabels,
    DeleteNotificationPermissions,
    DeleteNotificationSubscription,
    DeleteUser,
    DeregisterDirectory,
    DescribeActivities,
    DescribeAvailableDirectories,
    DescribeComments,
    DescribeDocumentVersions,
    DescribeFolderContents,
    DescribeGroups,
    DescribeInstanceExports,
    DescribeInstances,
    DescribeNotificationPermissions,
    DescribeNotificationSubscriptions,
    DescribeResourcePermissions,
    DescribeRootFolders,
    DescribeUsers,
    DownloadDocumentVersion,
    GetCurrentUser,
    GetDocument,
    GetDocumentPath,
    GetDocumentVersion,
    GetFolder,
    GetFolderPath,
    GetGroup,
    GetResources,
    InitiateDocumentVersionUpload,
    RegisterDirectory,
    RemoveAllResourcePermissions,
    RemoveResourcePermission,
    RestoreDocumentVersions,
    SearchResources,
    StartInstanceExport,
    UpdateDocument,
    UpdateDocumentVersion,
    UpdateFolder,
    UpdateInstanceAlias,
    UpdateUser,
    UpdateUserAdministrativeSettings,
}
impl std::fmt::Display for WorkdocsActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WorkdocsActions::AbortDocumentVersionUpload => {
                write!(f, "workdocs:AbortDocumentVersionUpload")
            }
            WorkdocsActions::ActivateUser => write!(f, "workdocs:ActivateUser"),
            WorkdocsActions::AddNotificationPermissions => {
                write!(f, "workdocs:AddNotificationPermissions")
            }
            WorkdocsActions::AddResourcePermissions => write!(f, "workdocs:AddResourcePermissions"),
            WorkdocsActions::AddUserToGroup => write!(f, "workdocs:AddUserToGroup"),
            WorkdocsActions::CheckAlias => write!(f, "workdocs:CheckAlias"),
            WorkdocsActions::CreateComment => write!(f, "workdocs:CreateComment"),
            WorkdocsActions::CreateCustomMetadata => write!(f, "workdocs:CreateCustomMetadata"),
            WorkdocsActions::CreateFolder => write!(f, "workdocs:CreateFolder"),
            WorkdocsActions::CreateInstance => write!(f, "workdocs:CreateInstance"),
            WorkdocsActions::CreateLabels => write!(f, "workdocs:CreateLabels"),
            WorkdocsActions::CreateNotificationSubscription => {
                write!(f, "workdocs:CreateNotificationSubscription")
            }
            WorkdocsActions::CreateUser => write!(f, "workdocs:CreateUser"),
            WorkdocsActions::DeactivateUser => write!(f, "workdocs:DeactivateUser"),
            WorkdocsActions::DeleteComment => write!(f, "workdocs:DeleteComment"),
            WorkdocsActions::DeleteCustomMetadata => write!(f, "workdocs:DeleteCustomMetadata"),
            WorkdocsActions::DeleteDocument => write!(f, "workdocs:DeleteDocument"),
            WorkdocsActions::DeleteDocumentVersion => write!(f, "workdocs:DeleteDocumentVersion"),
            WorkdocsActions::DeleteFolder => write!(f, "workdocs:DeleteFolder"),
            WorkdocsActions::DeleteFolderContents => write!(f, "workdocs:DeleteFolderContents"),
            WorkdocsActions::DeleteInstance => write!(f, "workdocs:DeleteInstance"),
            WorkdocsActions::DeleteLabels => write!(f, "workdocs:DeleteLabels"),
            WorkdocsActions::DeleteNotificationPermissions => {
                write!(f, "workdocs:DeleteNotificationPermissions")
            }
            WorkdocsActions::DeleteNotificationSubscription => {
                write!(f, "workdocs:DeleteNotificationSubscription")
            }
            WorkdocsActions::DeleteUser => write!(f, "workdocs:DeleteUser"),
            WorkdocsActions::DeregisterDirectory => write!(f, "workdocs:DeregisterDirectory"),
            WorkdocsActions::DescribeActivities => write!(f, "workdocs:DescribeActivities"),
            WorkdocsActions::DescribeAvailableDirectories => {
                write!(f, "workdocs:DescribeAvailableDirectories")
            }
            WorkdocsActions::DescribeComments => write!(f, "workdocs:DescribeComments"),
            WorkdocsActions::DescribeDocumentVersions => {
                write!(f, "workdocs:DescribeDocumentVersions")
            }
            WorkdocsActions::DescribeFolderContents => write!(f, "workdocs:DescribeFolderContents"),
            WorkdocsActions::DescribeGroups => write!(f, "workdocs:DescribeGroups"),
            WorkdocsActions::DescribeInstanceExports => {
                write!(f, "workdocs:DescribeInstanceExports")
            }
            WorkdocsActions::DescribeInstances => write!(f, "workdocs:DescribeInstances"),
            WorkdocsActions::DescribeNotificationPermissions => {
                write!(f, "workdocs:DescribeNotificationPermissions")
            }
            WorkdocsActions::DescribeNotificationSubscriptions => {
                write!(f, "workdocs:DescribeNotificationSubscriptions")
            }
            WorkdocsActions::DescribeResourcePermissions => {
                write!(f, "workdocs:DescribeResourcePermissions")
            }
            WorkdocsActions::DescribeRootFolders => write!(f, "workdocs:DescribeRootFolders"),
            WorkdocsActions::DescribeUsers => write!(f, "workdocs:DescribeUsers"),
            WorkdocsActions::DownloadDocumentVersion => {
                write!(f, "workdocs:DownloadDocumentVersion")
            }
            WorkdocsActions::GetCurrentUser => write!(f, "workdocs:GetCurrentUser"),
            WorkdocsActions::GetDocument => write!(f, "workdocs:GetDocument"),
            WorkdocsActions::GetDocumentPath => write!(f, "workdocs:GetDocumentPath"),
            WorkdocsActions::GetDocumentVersion => write!(f, "workdocs:GetDocumentVersion"),
            WorkdocsActions::GetFolder => write!(f, "workdocs:GetFolder"),
            WorkdocsActions::GetFolderPath => write!(f, "workdocs:GetFolderPath"),
            WorkdocsActions::GetGroup => write!(f, "workdocs:GetGroup"),
            WorkdocsActions::GetResources => write!(f, "workdocs:GetResources"),
            WorkdocsActions::InitiateDocumentVersionUpload => {
                write!(f, "workdocs:InitiateDocumentVersionUpload")
            }
            WorkdocsActions::RegisterDirectory => write!(f, "workdocs:RegisterDirectory"),
            WorkdocsActions::RemoveAllResourcePermissions => {
                write!(f, "workdocs:RemoveAllResourcePermissions")
            }
            WorkdocsActions::RemoveResourcePermission => {
                write!(f, "workdocs:RemoveResourcePermission")
            }
            WorkdocsActions::RestoreDocumentVersions => {
                write!(f, "workdocs:RestoreDocumentVersions")
            }
            WorkdocsActions::SearchResources => write!(f, "workdocs:SearchResources"),
            WorkdocsActions::StartInstanceExport => write!(f, "workdocs:StartInstanceExport"),
            WorkdocsActions::UpdateDocument => write!(f, "workdocs:UpdateDocument"),
            WorkdocsActions::UpdateDocumentVersion => write!(f, "workdocs:UpdateDocumentVersion"),
            WorkdocsActions::UpdateFolder => write!(f, "workdocs:UpdateFolder"),
            WorkdocsActions::UpdateInstanceAlias => write!(f, "workdocs:UpdateInstanceAlias"),
            WorkdocsActions::UpdateUser => write!(f, "workdocs:UpdateUser"),
            WorkdocsActions::UpdateUserAdministrativeSettings => {
                write!(f, "workdocs:UpdateUserAdministrativeSettings")
            }
        }
    }
}
