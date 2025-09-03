// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum RamActions {
    AcceptResourceShareInvitation,
    AssociateResourceShare,
    AssociateResourceSharePermission,
    CreatePermission,
    CreatePermissionVersion,
    CreateResourceShare,
    DeletePermission,
    DeletePermissionVersion,
    DeleteResourceShare,
    DisassociateResourceShare,
    DisassociateResourceSharePermission,
    EnableSharingWithAwsOrganization,
    GetPermission,
    GetResourcePolicies,
    GetResourceShareAssociations,
    GetResourceShareInvitations,
    GetResourceShares,
    ListPendingInvitationResources,
    ListPermissionAssociations,
    ListPermissionVersions,
    ListPermissions,
    ListPrincipals,
    ListReplacePermissionAssociationsWork,
    ListResourceSharePermissions,
    ListResourceTypes,
    ListResources,
    PromotePermissionCreatedFromPolicy,
    PromoteResourceShareCreatedFromPolicy,
    RejectResourceShareInvitation,
    ReplacePermissionAssociations,
    SetDefaultPermissionVersion,
    TagResource,
    UntagResource,
    UpdateResourceShare,
}
impl std::fmt::Display for RamActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RamActions::AcceptResourceShareInvitation => {
                write!(f, "ram:AcceptResourceShareInvitation")
            }
            RamActions::AssociateResourceShare => write!(f, "ram:AssociateResourceShare"),
            RamActions::AssociateResourceSharePermission => {
                write!(f, "ram:AssociateResourceSharePermission")
            }
            RamActions::CreatePermission => write!(f, "ram:CreatePermission"),
            RamActions::CreatePermissionVersion => write!(f, "ram:CreatePermissionVersion"),
            RamActions::CreateResourceShare => write!(f, "ram:CreateResourceShare"),
            RamActions::DeletePermission => write!(f, "ram:DeletePermission"),
            RamActions::DeletePermissionVersion => write!(f, "ram:DeletePermissionVersion"),
            RamActions::DeleteResourceShare => write!(f, "ram:DeleteResourceShare"),
            RamActions::DisassociateResourceShare => write!(f, "ram:DisassociateResourceShare"),
            RamActions::DisassociateResourceSharePermission => {
                write!(f, "ram:DisassociateResourceSharePermission")
            }
            RamActions::EnableSharingWithAwsOrganization => {
                write!(f, "ram:EnableSharingWithAwsOrganization")
            }
            RamActions::GetPermission => write!(f, "ram:GetPermission"),
            RamActions::GetResourcePolicies => write!(f, "ram:GetResourcePolicies"),
            RamActions::GetResourceShareAssociations => {
                write!(f, "ram:GetResourceShareAssociations")
            }
            RamActions::GetResourceShareInvitations => write!(f, "ram:GetResourceShareInvitations"),
            RamActions::GetResourceShares => write!(f, "ram:GetResourceShares"),
            RamActions::ListPendingInvitationResources => {
                write!(f, "ram:ListPendingInvitationResources")
            }
            RamActions::ListPermissionAssociations => write!(f, "ram:ListPermissionAssociations"),
            RamActions::ListPermissionVersions => write!(f, "ram:ListPermissionVersions"),
            RamActions::ListPermissions => write!(f, "ram:ListPermissions"),
            RamActions::ListPrincipals => write!(f, "ram:ListPrincipals"),
            RamActions::ListReplacePermissionAssociationsWork => {
                write!(f, "ram:ListReplacePermissionAssociationsWork")
            }
            RamActions::ListResourceSharePermissions => {
                write!(f, "ram:ListResourceSharePermissions")
            }
            RamActions::ListResourceTypes => write!(f, "ram:ListResourceTypes"),
            RamActions::ListResources => write!(f, "ram:ListResources"),
            RamActions::PromotePermissionCreatedFromPolicy => {
                write!(f, "ram:PromotePermissionCreatedFromPolicy")
            }
            RamActions::PromoteResourceShareCreatedFromPolicy => {
                write!(f, "ram:PromoteResourceShareCreatedFromPolicy")
            }
            RamActions::RejectResourceShareInvitation => {
                write!(f, "ram:RejectResourceShareInvitation")
            }
            RamActions::ReplacePermissionAssociations => {
                write!(f, "ram:ReplacePermissionAssociations")
            }
            RamActions::SetDefaultPermissionVersion => write!(f, "ram:SetDefaultPermissionVersion"),
            RamActions::TagResource => write!(f, "ram:TagResource"),
            RamActions::UntagResource => write!(f, "ram:UntagResource"),
            RamActions::UpdateResourceShare => write!(f, "ram:UpdateResourceShare"),
        }
    }
}
