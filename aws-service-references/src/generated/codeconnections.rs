// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum CodeconnectionsActions {
    CreateConnection,
    CreateHost,
    CreateRepositoryLink,
    CreateSyncConfiguration,
    DeleteConnection,
    DeleteHost,
    DeleteRepositoryLink,
    DeleteSyncConfiguration,
    GetConnection,
    GetConnectionToken,
    GetHost,
    GetIndividualAccessToken,
    GetInstallationUrl,
    GetRepositoryLink,
    GetRepositorySyncStatus,
    GetResourceSyncStatus,
    GetSyncBlockerSummary,
    GetSyncConfiguration,
    ListConnections,
    ListHosts,
    ListInstallationTargets,
    ListRepositoryLinks,
    ListRepositorySyncDefinitions,
    ListSyncConfigurations,
    ListTagsForResource,
    PassConnection,
    PassRepository,
    RegisterAppCode,
    StartAppRegistrationHandshake,
    StartOAuthHandshake,
    TagResource,
    UntagResource,
    UpdateConnectionInstallation,
    UpdateHost,
    UpdateRepositoryLink,
    UpdateSyncBlocker,
    UpdateSyncConfiguration,
    UseConnection,
}
impl std::fmt::Display for CodeconnectionsActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CodeconnectionsActions::CreateConnection => {
                write!(f, "codeconnections:CreateConnection")
            }
            CodeconnectionsActions::CreateHost => write!(f, "codeconnections:CreateHost"),
            CodeconnectionsActions::CreateRepositoryLink => {
                write!(f, "codeconnections:CreateRepositoryLink")
            }
            CodeconnectionsActions::CreateSyncConfiguration => {
                write!(f, "codeconnections:CreateSyncConfiguration")
            }
            CodeconnectionsActions::DeleteConnection => {
                write!(f, "codeconnections:DeleteConnection")
            }
            CodeconnectionsActions::DeleteHost => write!(f, "codeconnections:DeleteHost"),
            CodeconnectionsActions::DeleteRepositoryLink => {
                write!(f, "codeconnections:DeleteRepositoryLink")
            }
            CodeconnectionsActions::DeleteSyncConfiguration => {
                write!(f, "codeconnections:DeleteSyncConfiguration")
            }
            CodeconnectionsActions::GetConnection => write!(f, "codeconnections:GetConnection"),
            CodeconnectionsActions::GetConnectionToken => {
                write!(f, "codeconnections:GetConnectionToken")
            }
            CodeconnectionsActions::GetHost => write!(f, "codeconnections:GetHost"),
            CodeconnectionsActions::GetIndividualAccessToken => {
                write!(f, "codeconnections:GetIndividualAccessToken")
            }
            CodeconnectionsActions::GetInstallationUrl => {
                write!(f, "codeconnections:GetInstallationUrl")
            }
            CodeconnectionsActions::GetRepositoryLink => {
                write!(f, "codeconnections:GetRepositoryLink")
            }
            CodeconnectionsActions::GetRepositorySyncStatus => {
                write!(f, "codeconnections:GetRepositorySyncStatus")
            }
            CodeconnectionsActions::GetResourceSyncStatus => {
                write!(f, "codeconnections:GetResourceSyncStatus")
            }
            CodeconnectionsActions::GetSyncBlockerSummary => {
                write!(f, "codeconnections:GetSyncBlockerSummary")
            }
            CodeconnectionsActions::GetSyncConfiguration => {
                write!(f, "codeconnections:GetSyncConfiguration")
            }
            CodeconnectionsActions::ListConnections => write!(f, "codeconnections:ListConnections"),
            CodeconnectionsActions::ListHosts => write!(f, "codeconnections:ListHosts"),
            CodeconnectionsActions::ListInstallationTargets => {
                write!(f, "codeconnections:ListInstallationTargets")
            }
            CodeconnectionsActions::ListRepositoryLinks => {
                write!(f, "codeconnections:ListRepositoryLinks")
            }
            CodeconnectionsActions::ListRepositorySyncDefinitions => {
                write!(f, "codeconnections:ListRepositorySyncDefinitions")
            }
            CodeconnectionsActions::ListSyncConfigurations => {
                write!(f, "codeconnections:ListSyncConfigurations")
            }
            CodeconnectionsActions::ListTagsForResource => {
                write!(f, "codeconnections:ListTagsForResource")
            }
            CodeconnectionsActions::PassConnection => write!(f, "codeconnections:PassConnection"),
            CodeconnectionsActions::PassRepository => write!(f, "codeconnections:PassRepository"),
            CodeconnectionsActions::RegisterAppCode => write!(f, "codeconnections:RegisterAppCode"),
            CodeconnectionsActions::StartAppRegistrationHandshake => {
                write!(f, "codeconnections:StartAppRegistrationHandshake")
            }
            CodeconnectionsActions::StartOAuthHandshake => {
                write!(f, "codeconnections:StartOAuthHandshake")
            }
            CodeconnectionsActions::TagResource => write!(f, "codeconnections:TagResource"),
            CodeconnectionsActions::UntagResource => write!(f, "codeconnections:UntagResource"),
            CodeconnectionsActions::UpdateConnectionInstallation => {
                write!(f, "codeconnections:UpdateConnectionInstallation")
            }
            CodeconnectionsActions::UpdateHost => write!(f, "codeconnections:UpdateHost"),
            CodeconnectionsActions::UpdateRepositoryLink => {
                write!(f, "codeconnections:UpdateRepositoryLink")
            }
            CodeconnectionsActions::UpdateSyncBlocker => {
                write!(f, "codeconnections:UpdateSyncBlocker")
            }
            CodeconnectionsActions::UpdateSyncConfiguration => {
                write!(f, "codeconnections:UpdateSyncConfiguration")
            }
            CodeconnectionsActions::UseConnection => write!(f, "codeconnections:UseConnection"),
        }
    }
}
