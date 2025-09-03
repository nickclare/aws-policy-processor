// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum FmsActions {
    AssociateAdminAccount,
    AssociateThirdPartyFirewall,
    BatchAssociateResource,
    BatchDisassociateResource,
    DeleteAppsList,
    DeleteNotificationChannel,
    DeletePolicy,
    DeleteProtocolsList,
    DeleteResourceSet,
    DisassociateAdminAccount,
    DisassociateThirdPartyFirewall,
    GetAdminAccount,
    GetAdminScope,
    GetAppsList,
    GetComplianceDetail,
    GetNotificationChannel,
    GetPolicy,
    GetProtectionStatus,
    GetProtocolsList,
    GetResourceSet,
    GetThirdPartyFirewallAssociationStatus,
    GetViolationDetails,
    ListAdminAccountsForOrganization,
    ListAdminsManagingAccount,
    ListAppsLists,
    ListComplianceStatus,
    ListDiscoveredResources,
    ListMemberAccounts,
    ListPolicies,
    ListProtocolsLists,
    ListResourceSetResources,
    ListResourceSets,
    ListTagsForResource,
    ListThirdPartyFirewallFirewallPolicies,
    PutAdminAccount,
    PutAppsList,
    PutNotificationChannel,
    PutPolicy,
    PutProtocolsList,
    PutResourceSet,
    TagResource,
    UntagResource,
}
impl std::fmt::Display for FmsActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FmsActions::AssociateAdminAccount => write!(f, "fms:AssociateAdminAccount"),
            FmsActions::AssociateThirdPartyFirewall => write!(f, "fms:AssociateThirdPartyFirewall"),
            FmsActions::BatchAssociateResource => write!(f, "fms:BatchAssociateResource"),
            FmsActions::BatchDisassociateResource => write!(f, "fms:BatchDisassociateResource"),
            FmsActions::DeleteAppsList => write!(f, "fms:DeleteAppsList"),
            FmsActions::DeleteNotificationChannel => write!(f, "fms:DeleteNotificationChannel"),
            FmsActions::DeletePolicy => write!(f, "fms:DeletePolicy"),
            FmsActions::DeleteProtocolsList => write!(f, "fms:DeleteProtocolsList"),
            FmsActions::DeleteResourceSet => write!(f, "fms:DeleteResourceSet"),
            FmsActions::DisassociateAdminAccount => write!(f, "fms:DisassociateAdminAccount"),
            FmsActions::DisassociateThirdPartyFirewall => {
                write!(f, "fms:DisassociateThirdPartyFirewall")
            }
            FmsActions::GetAdminAccount => write!(f, "fms:GetAdminAccount"),
            FmsActions::GetAdminScope => write!(f, "fms:GetAdminScope"),
            FmsActions::GetAppsList => write!(f, "fms:GetAppsList"),
            FmsActions::GetComplianceDetail => write!(f, "fms:GetComplianceDetail"),
            FmsActions::GetNotificationChannel => write!(f, "fms:GetNotificationChannel"),
            FmsActions::GetPolicy => write!(f, "fms:GetPolicy"),
            FmsActions::GetProtectionStatus => write!(f, "fms:GetProtectionStatus"),
            FmsActions::GetProtocolsList => write!(f, "fms:GetProtocolsList"),
            FmsActions::GetResourceSet => write!(f, "fms:GetResourceSet"),
            FmsActions::GetThirdPartyFirewallAssociationStatus => {
                write!(f, "fms:GetThirdPartyFirewallAssociationStatus")
            }
            FmsActions::GetViolationDetails => write!(f, "fms:GetViolationDetails"),
            FmsActions::ListAdminAccountsForOrganization => {
                write!(f, "fms:ListAdminAccountsForOrganization")
            }
            FmsActions::ListAdminsManagingAccount => write!(f, "fms:ListAdminsManagingAccount"),
            FmsActions::ListAppsLists => write!(f, "fms:ListAppsLists"),
            FmsActions::ListComplianceStatus => write!(f, "fms:ListComplianceStatus"),
            FmsActions::ListDiscoveredResources => write!(f, "fms:ListDiscoveredResources"),
            FmsActions::ListMemberAccounts => write!(f, "fms:ListMemberAccounts"),
            FmsActions::ListPolicies => write!(f, "fms:ListPolicies"),
            FmsActions::ListProtocolsLists => write!(f, "fms:ListProtocolsLists"),
            FmsActions::ListResourceSetResources => write!(f, "fms:ListResourceSetResources"),
            FmsActions::ListResourceSets => write!(f, "fms:ListResourceSets"),
            FmsActions::ListTagsForResource => write!(f, "fms:ListTagsForResource"),
            FmsActions::ListThirdPartyFirewallFirewallPolicies => {
                write!(f, "fms:ListThirdPartyFirewallFirewallPolicies")
            }
            FmsActions::PutAdminAccount => write!(f, "fms:PutAdminAccount"),
            FmsActions::PutAppsList => write!(f, "fms:PutAppsList"),
            FmsActions::PutNotificationChannel => write!(f, "fms:PutNotificationChannel"),
            FmsActions::PutPolicy => write!(f, "fms:PutPolicy"),
            FmsActions::PutProtocolsList => write!(f, "fms:PutProtocolsList"),
            FmsActions::PutResourceSet => write!(f, "fms:PutResourceSet"),
            FmsActions::TagResource => write!(f, "fms:TagResource"),
            FmsActions::UntagResource => write!(f, "fms:UntagResource"),
        }
    }
}
