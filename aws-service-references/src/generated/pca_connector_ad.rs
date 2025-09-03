// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum PcaConnectorAdActions {
    CreateConnector,
    CreateDirectoryRegistration,
    CreateServicePrincipalName,
    CreateTemplate,
    CreateTemplateGroupAccessControlEntry,
    DeleteConnector,
    DeleteDirectoryRegistration,
    DeleteServicePrincipalName,
    DeleteTemplate,
    DeleteTemplateGroupAccessControlEntry,
    GetConnector,
    GetDirectoryRegistration,
    GetServicePrincipalName,
    GetTemplate,
    GetTemplateGroupAccessControlEntry,
    ListConnectors,
    ListDirectoryRegistrations,
    ListServicePrincipalNames,
    ListTagsForResource,
    ListTemplateGroupAccessControlEntries,
    ListTemplates,
    TagResource,
    UntagResource,
    UpdateTemplate,
    UpdateTemplateGroupAccessControlEntry,
}
impl std::fmt::Display for PcaConnectorAdActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PcaConnectorAdActions::CreateConnector => write!(f, "pca-connector-ad:CreateConnector"),
            PcaConnectorAdActions::CreateDirectoryRegistration => {
                write!(f, "pca-connector-ad:CreateDirectoryRegistration")
            }
            PcaConnectorAdActions::CreateServicePrincipalName => {
                write!(f, "pca-connector-ad:CreateServicePrincipalName")
            }
            PcaConnectorAdActions::CreateTemplate => write!(f, "pca-connector-ad:CreateTemplate"),
            PcaConnectorAdActions::CreateTemplateGroupAccessControlEntry => {
                write!(f, "pca-connector-ad:CreateTemplateGroupAccessControlEntry")
            }
            PcaConnectorAdActions::DeleteConnector => write!(f, "pca-connector-ad:DeleteConnector"),
            PcaConnectorAdActions::DeleteDirectoryRegistration => {
                write!(f, "pca-connector-ad:DeleteDirectoryRegistration")
            }
            PcaConnectorAdActions::DeleteServicePrincipalName => {
                write!(f, "pca-connector-ad:DeleteServicePrincipalName")
            }
            PcaConnectorAdActions::DeleteTemplate => write!(f, "pca-connector-ad:DeleteTemplate"),
            PcaConnectorAdActions::DeleteTemplateGroupAccessControlEntry => {
                write!(f, "pca-connector-ad:DeleteTemplateGroupAccessControlEntry")
            }
            PcaConnectorAdActions::GetConnector => write!(f, "pca-connector-ad:GetConnector"),
            PcaConnectorAdActions::GetDirectoryRegistration => {
                write!(f, "pca-connector-ad:GetDirectoryRegistration")
            }
            PcaConnectorAdActions::GetServicePrincipalName => {
                write!(f, "pca-connector-ad:GetServicePrincipalName")
            }
            PcaConnectorAdActions::GetTemplate => write!(f, "pca-connector-ad:GetTemplate"),
            PcaConnectorAdActions::GetTemplateGroupAccessControlEntry => {
                write!(f, "pca-connector-ad:GetTemplateGroupAccessControlEntry")
            }
            PcaConnectorAdActions::ListConnectors => write!(f, "pca-connector-ad:ListConnectors"),
            PcaConnectorAdActions::ListDirectoryRegistrations => {
                write!(f, "pca-connector-ad:ListDirectoryRegistrations")
            }
            PcaConnectorAdActions::ListServicePrincipalNames => {
                write!(f, "pca-connector-ad:ListServicePrincipalNames")
            }
            PcaConnectorAdActions::ListTagsForResource => {
                write!(f, "pca-connector-ad:ListTagsForResource")
            }
            PcaConnectorAdActions::ListTemplateGroupAccessControlEntries => {
                write!(f, "pca-connector-ad:ListTemplateGroupAccessControlEntries")
            }
            PcaConnectorAdActions::ListTemplates => write!(f, "pca-connector-ad:ListTemplates"),
            PcaConnectorAdActions::TagResource => write!(f, "pca-connector-ad:TagResource"),
            PcaConnectorAdActions::UntagResource => write!(f, "pca-connector-ad:UntagResource"),
            PcaConnectorAdActions::UpdateTemplate => write!(f, "pca-connector-ad:UpdateTemplate"),
            PcaConnectorAdActions::UpdateTemplateGroupAccessControlEntry => {
                write!(f, "pca-connector-ad:UpdateTemplateGroupAccessControlEntry")
            }
        }
    }
}
