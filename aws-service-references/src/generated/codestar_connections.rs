// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum CodestarConnectionsActions {
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
impl std::fmt::Display for CodestarConnectionsActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CodestarConnectionsActions::CreateConnection => {
                write!(f, "codestar-connections:CreateConnection")
            }
            CodestarConnectionsActions::CreateHost => write!(f, "codestar-connections:CreateHost"),
            CodestarConnectionsActions::CreateRepositoryLink => {
                write!(f, "codestar-connections:CreateRepositoryLink")
            }
            CodestarConnectionsActions::CreateSyncConfiguration => {
                write!(f, "codestar-connections:CreateSyncConfiguration")
            }
            CodestarConnectionsActions::DeleteConnection => {
                write!(f, "codestar-connections:DeleteConnection")
            }
            CodestarConnectionsActions::DeleteHost => write!(f, "codestar-connections:DeleteHost"),
            CodestarConnectionsActions::DeleteRepositoryLink => {
                write!(f, "codestar-connections:DeleteRepositoryLink")
            }
            CodestarConnectionsActions::DeleteSyncConfiguration => {
                write!(f, "codestar-connections:DeleteSyncConfiguration")
            }
            CodestarConnectionsActions::GetConnection => {
                write!(f, "codestar-connections:GetConnection")
            }
            CodestarConnectionsActions::GetConnectionToken => {
                write!(f, "codestar-connections:GetConnectionToken")
            }
            CodestarConnectionsActions::GetHost => write!(f, "codestar-connections:GetHost"),
            CodestarConnectionsActions::GetIndividualAccessToken => {
                write!(f, "codestar-connections:GetIndividualAccessToken")
            }
            CodestarConnectionsActions::GetInstallationUrl => {
                write!(f, "codestar-connections:GetInstallationUrl")
            }
            CodestarConnectionsActions::GetRepositoryLink => {
                write!(f, "codestar-connections:GetRepositoryLink")
            }
            CodestarConnectionsActions::GetRepositorySyncStatus => {
                write!(f, "codestar-connections:GetRepositorySyncStatus")
            }
            CodestarConnectionsActions::GetResourceSyncStatus => {
                write!(f, "codestar-connections:GetResourceSyncStatus")
            }
            CodestarConnectionsActions::GetSyncBlockerSummary => {
                write!(f, "codestar-connections:GetSyncBlockerSummary")
            }
            CodestarConnectionsActions::GetSyncConfiguration => {
                write!(f, "codestar-connections:GetSyncConfiguration")
            }
            CodestarConnectionsActions::ListConnections => {
                write!(f, "codestar-connections:ListConnections")
            }
            CodestarConnectionsActions::ListHosts => write!(f, "codestar-connections:ListHosts"),
            CodestarConnectionsActions::ListInstallationTargets => {
                write!(f, "codestar-connections:ListInstallationTargets")
            }
            CodestarConnectionsActions::ListRepositoryLinks => {
                write!(f, "codestar-connections:ListRepositoryLinks")
            }
            CodestarConnectionsActions::ListRepositorySyncDefinitions => {
                write!(f, "codestar-connections:ListRepositorySyncDefinitions")
            }
            CodestarConnectionsActions::ListSyncConfigurations => {
                write!(f, "codestar-connections:ListSyncConfigurations")
            }
            CodestarConnectionsActions::ListTagsForResource => {
                write!(f, "codestar-connections:ListTagsForResource")
            }
            CodestarConnectionsActions::PassConnection => {
                write!(f, "codestar-connections:PassConnection")
            }
            CodestarConnectionsActions::PassRepository => {
                write!(f, "codestar-connections:PassRepository")
            }
            CodestarConnectionsActions::RegisterAppCode => {
                write!(f, "codestar-connections:RegisterAppCode")
            }
            CodestarConnectionsActions::StartAppRegistrationHandshake => {
                write!(f, "codestar-connections:StartAppRegistrationHandshake")
            }
            CodestarConnectionsActions::StartOAuthHandshake => {
                write!(f, "codestar-connections:StartOAuthHandshake")
            }
            CodestarConnectionsActions::TagResource => {
                write!(f, "codestar-connections:TagResource")
            }
            CodestarConnectionsActions::UntagResource => {
                write!(f, "codestar-connections:UntagResource")
            }
            CodestarConnectionsActions::UpdateConnectionInstallation => {
                write!(f, "codestar-connections:UpdateConnectionInstallation")
            }
            CodestarConnectionsActions::UpdateHost => write!(f, "codestar-connections:UpdateHost"),
            CodestarConnectionsActions::UpdateRepositoryLink => {
                write!(f, "codestar-connections:UpdateRepositoryLink")
            }
            CodestarConnectionsActions::UpdateSyncBlocker => {
                write!(f, "codestar-connections:UpdateSyncBlocker")
            }
            CodestarConnectionsActions::UpdateSyncConfiguration => {
                write!(f, "codestar-connections:UpdateSyncConfiguration")
            }
            CodestarConnectionsActions::UseConnection => {
                write!(f, "codestar-connections:UseConnection")
            }
        }
    }
}
