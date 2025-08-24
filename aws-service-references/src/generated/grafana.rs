// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum GrafanaActions {
    AssociateLicense,
    CreateWorkspace,
    CreateWorkspaceApiKey,
    CreateWorkspaceServiceAccount,
    CreateWorkspaceServiceAccountToken,
    DeleteWorkspace,
    DeleteWorkspaceApiKey,
    DeleteWorkspaceServiceAccount,
    DeleteWorkspaceServiceAccountToken,
    DescribeWorkspace,
    DescribeWorkspaceAuthentication,
    DescribeWorkspaceConfiguration,
    DisassociateLicense,
    ListPermissions,
    ListTagsForResource,
    ListVersions,
    ListWorkspaceServiceAccountTokens,
    ListWorkspaceServiceAccounts,
    ListWorkspaces,
    TagResource,
    UntagResource,
    UpdatePermissions,
    UpdateWorkspace,
    UpdateWorkspaceAuthentication,
    UpdateWorkspaceConfiguration,
}
impl std::fmt::Display for GrafanaActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GrafanaActions::AssociateLicense => write!(f, "grafana:AssociateLicense"),
            GrafanaActions::CreateWorkspace => write!(f, "grafana:CreateWorkspace"),
            GrafanaActions::CreateWorkspaceApiKey => write!(f, "grafana:CreateWorkspaceApiKey"),
            GrafanaActions::CreateWorkspaceServiceAccount => {
                write!(f, "grafana:CreateWorkspaceServiceAccount")
            }
            GrafanaActions::CreateWorkspaceServiceAccountToken => {
                write!(f, "grafana:CreateWorkspaceServiceAccountToken")
            }
            GrafanaActions::DeleteWorkspace => write!(f, "grafana:DeleteWorkspace"),
            GrafanaActions::DeleteWorkspaceApiKey => write!(f, "grafana:DeleteWorkspaceApiKey"),
            GrafanaActions::DeleteWorkspaceServiceAccount => {
                write!(f, "grafana:DeleteWorkspaceServiceAccount")
            }
            GrafanaActions::DeleteWorkspaceServiceAccountToken => {
                write!(f, "grafana:DeleteWorkspaceServiceAccountToken")
            }
            GrafanaActions::DescribeWorkspace => write!(f, "grafana:DescribeWorkspace"),
            GrafanaActions::DescribeWorkspaceAuthentication => {
                write!(f, "grafana:DescribeWorkspaceAuthentication")
            }
            GrafanaActions::DescribeWorkspaceConfiguration => {
                write!(f, "grafana:DescribeWorkspaceConfiguration")
            }
            GrafanaActions::DisassociateLicense => write!(f, "grafana:DisassociateLicense"),
            GrafanaActions::ListPermissions => write!(f, "grafana:ListPermissions"),
            GrafanaActions::ListTagsForResource => write!(f, "grafana:ListTagsForResource"),
            GrafanaActions::ListVersions => write!(f, "grafana:ListVersions"),
            GrafanaActions::ListWorkspaceServiceAccountTokens => {
                write!(f, "grafana:ListWorkspaceServiceAccountTokens")
            }
            GrafanaActions::ListWorkspaceServiceAccounts => {
                write!(f, "grafana:ListWorkspaceServiceAccounts")
            }
            GrafanaActions::ListWorkspaces => write!(f, "grafana:ListWorkspaces"),
            GrafanaActions::TagResource => write!(f, "grafana:TagResource"),
            GrafanaActions::UntagResource => write!(f, "grafana:UntagResource"),
            GrafanaActions::UpdatePermissions => write!(f, "grafana:UpdatePermissions"),
            GrafanaActions::UpdateWorkspace => write!(f, "grafana:UpdateWorkspace"),
            GrafanaActions::UpdateWorkspaceAuthentication => {
                write!(f, "grafana:UpdateWorkspaceAuthentication")
            }
            GrafanaActions::UpdateWorkspaceConfiguration => {
                write!(f, "grafana:UpdateWorkspaceConfiguration")
            }
        }
    }
}
