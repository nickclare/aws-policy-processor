// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum WorkspacesWebActions {
    AssociateBrowserSettings,
    AssociateDataProtectionSettings,
    AssociateIpAccessSettings,
    AssociateNetworkSettings,
    AssociateSessionLogger,
    AssociateTrustStore,
    AssociateUserAccessLoggingSettings,
    AssociateUserSettings,
    CreateBrowserSettings,
    CreateDataProtectionSettings,
    CreateIdentityProvider,
    CreateIpAccessSettings,
    CreateNetworkSettings,
    CreatePortal,
    CreateSessionLogger,
    CreateTrustStore,
    CreateUserAccessLoggingSettings,
    CreateUserSettings,
    DeleteBrowserSettings,
    DeleteDataProtectionSettings,
    DeleteIdentityProvider,
    DeleteIpAccessSettings,
    DeleteNetworkSettings,
    DeletePortal,
    DeleteSessionLogger,
    DeleteTrustStore,
    DeleteUserAccessLoggingSettings,
    DeleteUserSettings,
    DisassociateBrowserSettings,
    DisassociateDataProtectionSettings,
    DisassociateIpAccessSettings,
    DisassociateNetworkSettings,
    DisassociateSessionLogger,
    DisassociateTrustStore,
    DisassociateUserAccessLoggingSettings,
    DisassociateUserSettings,
    ExpireSession,
    GetBrowserSettings,
    GetDataProtectionSettings,
    GetIdentityProvider,
    GetIpAccessSettings,
    GetNetworkSettings,
    GetPortal,
    GetPortalServiceProviderMetadata,
    GetSession,
    GetSessionLogger,
    GetTrustStore,
    GetTrustStoreCertificate,
    GetUserAccessLoggingSettings,
    GetUserSettings,
    ListBrowserSettings,
    ListDataProtectionSettings,
    ListIdentityProviders,
    ListIpAccessSettings,
    ListNetworkSettings,
    ListPortals,
    ListSessionLoggers,
    ListSessions,
    ListTagsForResource,
    ListTrustStoreCertificates,
    ListTrustStores,
    ListUserAccessLoggingSettings,
    ListUserSettings,
    TagResource,
    UntagResource,
    UpdateBrowserSettings,
    UpdateDataProtectionSettings,
    UpdateIdentityProvider,
    UpdateIpAccessSettings,
    UpdateNetworkSettings,
    UpdatePortal,
    UpdateSessionLogger,
    UpdateTrustStore,
    UpdateUserAccessLoggingSettings,
    UpdateUserSettings,
}
impl std::fmt::Display for WorkspacesWebActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WorkspacesWebActions::AssociateBrowserSettings => {
                write!(f, "workspaces-web:AssociateBrowserSettings")
            }
            WorkspacesWebActions::AssociateDataProtectionSettings => {
                write!(f, "workspaces-web:AssociateDataProtectionSettings")
            }
            WorkspacesWebActions::AssociateIpAccessSettings => {
                write!(f, "workspaces-web:AssociateIpAccessSettings")
            }
            WorkspacesWebActions::AssociateNetworkSettings => {
                write!(f, "workspaces-web:AssociateNetworkSettings")
            }
            WorkspacesWebActions::AssociateSessionLogger => {
                write!(f, "workspaces-web:AssociateSessionLogger")
            }
            WorkspacesWebActions::AssociateTrustStore => {
                write!(f, "workspaces-web:AssociateTrustStore")
            }
            WorkspacesWebActions::AssociateUserAccessLoggingSettings => {
                write!(f, "workspaces-web:AssociateUserAccessLoggingSettings")
            }
            WorkspacesWebActions::AssociateUserSettings => {
                write!(f, "workspaces-web:AssociateUserSettings")
            }
            WorkspacesWebActions::CreateBrowserSettings => {
                write!(f, "workspaces-web:CreateBrowserSettings")
            }
            WorkspacesWebActions::CreateDataProtectionSettings => {
                write!(f, "workspaces-web:CreateDataProtectionSettings")
            }
            WorkspacesWebActions::CreateIdentityProvider => {
                write!(f, "workspaces-web:CreateIdentityProvider")
            }
            WorkspacesWebActions::CreateIpAccessSettings => {
                write!(f, "workspaces-web:CreateIpAccessSettings")
            }
            WorkspacesWebActions::CreateNetworkSettings => {
                write!(f, "workspaces-web:CreateNetworkSettings")
            }
            WorkspacesWebActions::CreatePortal => write!(f, "workspaces-web:CreatePortal"),
            WorkspacesWebActions::CreateSessionLogger => {
                write!(f, "workspaces-web:CreateSessionLogger")
            }
            WorkspacesWebActions::CreateTrustStore => write!(f, "workspaces-web:CreateTrustStore"),
            WorkspacesWebActions::CreateUserAccessLoggingSettings => {
                write!(f, "workspaces-web:CreateUserAccessLoggingSettings")
            }
            WorkspacesWebActions::CreateUserSettings => {
                write!(f, "workspaces-web:CreateUserSettings")
            }
            WorkspacesWebActions::DeleteBrowserSettings => {
                write!(f, "workspaces-web:DeleteBrowserSettings")
            }
            WorkspacesWebActions::DeleteDataProtectionSettings => {
                write!(f, "workspaces-web:DeleteDataProtectionSettings")
            }
            WorkspacesWebActions::DeleteIdentityProvider => {
                write!(f, "workspaces-web:DeleteIdentityProvider")
            }
            WorkspacesWebActions::DeleteIpAccessSettings => {
                write!(f, "workspaces-web:DeleteIpAccessSettings")
            }
            WorkspacesWebActions::DeleteNetworkSettings => {
                write!(f, "workspaces-web:DeleteNetworkSettings")
            }
            WorkspacesWebActions::DeletePortal => write!(f, "workspaces-web:DeletePortal"),
            WorkspacesWebActions::DeleteSessionLogger => {
                write!(f, "workspaces-web:DeleteSessionLogger")
            }
            WorkspacesWebActions::DeleteTrustStore => write!(f, "workspaces-web:DeleteTrustStore"),
            WorkspacesWebActions::DeleteUserAccessLoggingSettings => {
                write!(f, "workspaces-web:DeleteUserAccessLoggingSettings")
            }
            WorkspacesWebActions::DeleteUserSettings => {
                write!(f, "workspaces-web:DeleteUserSettings")
            }
            WorkspacesWebActions::DisassociateBrowserSettings => {
                write!(f, "workspaces-web:DisassociateBrowserSettings")
            }
            WorkspacesWebActions::DisassociateDataProtectionSettings => {
                write!(f, "workspaces-web:DisassociateDataProtectionSettings")
            }
            WorkspacesWebActions::DisassociateIpAccessSettings => {
                write!(f, "workspaces-web:DisassociateIpAccessSettings")
            }
            WorkspacesWebActions::DisassociateNetworkSettings => {
                write!(f, "workspaces-web:DisassociateNetworkSettings")
            }
            WorkspacesWebActions::DisassociateSessionLogger => {
                write!(f, "workspaces-web:DisassociateSessionLogger")
            }
            WorkspacesWebActions::DisassociateTrustStore => {
                write!(f, "workspaces-web:DisassociateTrustStore")
            }
            WorkspacesWebActions::DisassociateUserAccessLoggingSettings => {
                write!(f, "workspaces-web:DisassociateUserAccessLoggingSettings")
            }
            WorkspacesWebActions::DisassociateUserSettings => {
                write!(f, "workspaces-web:DisassociateUserSettings")
            }
            WorkspacesWebActions::ExpireSession => write!(f, "workspaces-web:ExpireSession"),
            WorkspacesWebActions::GetBrowserSettings => {
                write!(f, "workspaces-web:GetBrowserSettings")
            }
            WorkspacesWebActions::GetDataProtectionSettings => {
                write!(f, "workspaces-web:GetDataProtectionSettings")
            }
            WorkspacesWebActions::GetIdentityProvider => {
                write!(f, "workspaces-web:GetIdentityProvider")
            }
            WorkspacesWebActions::GetIpAccessSettings => {
                write!(f, "workspaces-web:GetIpAccessSettings")
            }
            WorkspacesWebActions::GetNetworkSettings => {
                write!(f, "workspaces-web:GetNetworkSettings")
            }
            WorkspacesWebActions::GetPortal => write!(f, "workspaces-web:GetPortal"),
            WorkspacesWebActions::GetPortalServiceProviderMetadata => {
                write!(f, "workspaces-web:GetPortalServiceProviderMetadata")
            }
            WorkspacesWebActions::GetSession => write!(f, "workspaces-web:GetSession"),
            WorkspacesWebActions::GetSessionLogger => write!(f, "workspaces-web:GetSessionLogger"),
            WorkspacesWebActions::GetTrustStore => write!(f, "workspaces-web:GetTrustStore"),
            WorkspacesWebActions::GetTrustStoreCertificate => {
                write!(f, "workspaces-web:GetTrustStoreCertificate")
            }
            WorkspacesWebActions::GetUserAccessLoggingSettings => {
                write!(f, "workspaces-web:GetUserAccessLoggingSettings")
            }
            WorkspacesWebActions::GetUserSettings => write!(f, "workspaces-web:GetUserSettings"),
            WorkspacesWebActions::ListBrowserSettings => {
                write!(f, "workspaces-web:ListBrowserSettings")
            }
            WorkspacesWebActions::ListDataProtectionSettings => {
                write!(f, "workspaces-web:ListDataProtectionSettings")
            }
            WorkspacesWebActions::ListIdentityProviders => {
                write!(f, "workspaces-web:ListIdentityProviders")
            }
            WorkspacesWebActions::ListIpAccessSettings => {
                write!(f, "workspaces-web:ListIpAccessSettings")
            }
            WorkspacesWebActions::ListNetworkSettings => {
                write!(f, "workspaces-web:ListNetworkSettings")
            }
            WorkspacesWebActions::ListPortals => write!(f, "workspaces-web:ListPortals"),
            WorkspacesWebActions::ListSessionLoggers => {
                write!(f, "workspaces-web:ListSessionLoggers")
            }
            WorkspacesWebActions::ListSessions => write!(f, "workspaces-web:ListSessions"),
            WorkspacesWebActions::ListTagsForResource => {
                write!(f, "workspaces-web:ListTagsForResource")
            }
            WorkspacesWebActions::ListTrustStoreCertificates => {
                write!(f, "workspaces-web:ListTrustStoreCertificates")
            }
            WorkspacesWebActions::ListTrustStores => write!(f, "workspaces-web:ListTrustStores"),
            WorkspacesWebActions::ListUserAccessLoggingSettings => {
                write!(f, "workspaces-web:ListUserAccessLoggingSettings")
            }
            WorkspacesWebActions::ListUserSettings => write!(f, "workspaces-web:ListUserSettings"),
            WorkspacesWebActions::TagResource => write!(f, "workspaces-web:TagResource"),
            WorkspacesWebActions::UntagResource => write!(f, "workspaces-web:UntagResource"),
            WorkspacesWebActions::UpdateBrowserSettings => {
                write!(f, "workspaces-web:UpdateBrowserSettings")
            }
            WorkspacesWebActions::UpdateDataProtectionSettings => {
                write!(f, "workspaces-web:UpdateDataProtectionSettings")
            }
            WorkspacesWebActions::UpdateIdentityProvider => {
                write!(f, "workspaces-web:UpdateIdentityProvider")
            }
            WorkspacesWebActions::UpdateIpAccessSettings => {
                write!(f, "workspaces-web:UpdateIpAccessSettings")
            }
            WorkspacesWebActions::UpdateNetworkSettings => {
                write!(f, "workspaces-web:UpdateNetworkSettings")
            }
            WorkspacesWebActions::UpdatePortal => write!(f, "workspaces-web:UpdatePortal"),
            WorkspacesWebActions::UpdateSessionLogger => {
                write!(f, "workspaces-web:UpdateSessionLogger")
            }
            WorkspacesWebActions::UpdateTrustStore => write!(f, "workspaces-web:UpdateTrustStore"),
            WorkspacesWebActions::UpdateUserAccessLoggingSettings => {
                write!(f, "workspaces-web:UpdateUserAccessLoggingSettings")
            }
            WorkspacesWebActions::UpdateUserSettings => {
                write!(f, "workspaces-web:UpdateUserSettings")
            }
        }
    }
}
