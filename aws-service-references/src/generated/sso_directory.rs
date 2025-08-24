// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum SsoDirectoryActions {
    AddMemberToGroup,
    CompleteVirtualMfaDeviceRegistration,
    CompleteWebAuthnDeviceRegistration,
    CreateAlias,
    CreateBearerToken,
    CreateExternalIdPConfigurationForDirectory,
    CreateGroup,
    CreateProvisioningTenant,
    CreateUser,
    DeleteBearerToken,
    DeleteExternalIdPCertificate,
    DeleteExternalIdPConfigurationForDirectory,
    DeleteGroup,
    DeleteMfaDeviceForUser,
    DeleteProvisioningTenant,
    DeleteUser,
    DescribeDirectory,
    DescribeGroup,
    DescribeGroups,
    DescribeProvisioningTenant,
    DescribeUser,
    DescribeUserByUniqueAttribute,
    DescribeUsers,
    DisableExternalIdPConfigurationForDirectory,
    DisableUser,
    EnableExternalIdPConfigurationForDirectory,
    EnableUser,
    GetAwsspConfigurationForDirectory,
    GetGroupId,
    GetUserId,
    GetUserPoolInfo,
    ImportExternalIdPCertificate,
    IsMemberInGroup,
    IsMemberInGroups,
    ListBearerTokens,
    ListExternalIdPCertificates,
    ListExternalIdPConfigurationsForDirectory,
    ListGroups,
    ListGroupsForMember,
    ListGroupsForUser,
    ListMembersInGroup,
    ListMfaDevicesForUser,
    ListProvisioningTenants,
    ListUsers,
    RemoveMemberFromGroup,
    SearchGroups,
    SearchUsers,
    StartVirtualMfaDeviceRegistration,
    StartWebAuthnDeviceRegistration,
    UpdateExternalIdPConfigurationForDirectory,
    UpdateGroup,
    UpdateGroupDisplayName,
    UpdateMfaDeviceForUser,
    UpdatePassword,
    UpdateUser,
    UpdateUserName,
    VerifyEmail,
}
impl std::fmt::Display for SsoDirectoryActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SsoDirectoryActions::AddMemberToGroup => write!(f, "sso-directory:AddMemberToGroup"),
            SsoDirectoryActions::CompleteVirtualMfaDeviceRegistration => {
                write!(f, "sso-directory:CompleteVirtualMfaDeviceRegistration")
            }
            SsoDirectoryActions::CompleteWebAuthnDeviceRegistration => {
                write!(f, "sso-directory:CompleteWebAuthnDeviceRegistration")
            }
            SsoDirectoryActions::CreateAlias => write!(f, "sso-directory:CreateAlias"),
            SsoDirectoryActions::CreateBearerToken => write!(f, "sso-directory:CreateBearerToken"),
            SsoDirectoryActions::CreateExternalIdPConfigurationForDirectory => write!(
                f,
                "sso-directory:CreateExternalIdPConfigurationForDirectory"
            ),
            SsoDirectoryActions::CreateGroup => write!(f, "sso-directory:CreateGroup"),
            SsoDirectoryActions::CreateProvisioningTenant => {
                write!(f, "sso-directory:CreateProvisioningTenant")
            }
            SsoDirectoryActions::CreateUser => write!(f, "sso-directory:CreateUser"),
            SsoDirectoryActions::DeleteBearerToken => write!(f, "sso-directory:DeleteBearerToken"),
            SsoDirectoryActions::DeleteExternalIdPCertificate => {
                write!(f, "sso-directory:DeleteExternalIdPCertificate")
            }
            SsoDirectoryActions::DeleteExternalIdPConfigurationForDirectory => write!(
                f,
                "sso-directory:DeleteExternalIdPConfigurationForDirectory"
            ),
            SsoDirectoryActions::DeleteGroup => write!(f, "sso-directory:DeleteGroup"),
            SsoDirectoryActions::DeleteMfaDeviceForUser => {
                write!(f, "sso-directory:DeleteMfaDeviceForUser")
            }
            SsoDirectoryActions::DeleteProvisioningTenant => {
                write!(f, "sso-directory:DeleteProvisioningTenant")
            }
            SsoDirectoryActions::DeleteUser => write!(f, "sso-directory:DeleteUser"),
            SsoDirectoryActions::DescribeDirectory => write!(f, "sso-directory:DescribeDirectory"),
            SsoDirectoryActions::DescribeGroup => write!(f, "sso-directory:DescribeGroup"),
            SsoDirectoryActions::DescribeGroups => write!(f, "sso-directory:DescribeGroups"),
            SsoDirectoryActions::DescribeProvisioningTenant => {
                write!(f, "sso-directory:DescribeProvisioningTenant")
            }
            SsoDirectoryActions::DescribeUser => write!(f, "sso-directory:DescribeUser"),
            SsoDirectoryActions::DescribeUserByUniqueAttribute => {
                write!(f, "sso-directory:DescribeUserByUniqueAttribute")
            }
            SsoDirectoryActions::DescribeUsers => write!(f, "sso-directory:DescribeUsers"),
            SsoDirectoryActions::DisableExternalIdPConfigurationForDirectory => write!(
                f,
                "sso-directory:DisableExternalIdPConfigurationForDirectory"
            ),
            SsoDirectoryActions::DisableUser => write!(f, "sso-directory:DisableUser"),
            SsoDirectoryActions::EnableExternalIdPConfigurationForDirectory => write!(
                f,
                "sso-directory:EnableExternalIdPConfigurationForDirectory"
            ),
            SsoDirectoryActions::EnableUser => write!(f, "sso-directory:EnableUser"),
            SsoDirectoryActions::GetAwsspConfigurationForDirectory => {
                write!(f, "sso-directory:GetAWSSPConfigurationForDirectory")
            }
            SsoDirectoryActions::GetGroupId => write!(f, "sso-directory:GetGroupId"),
            SsoDirectoryActions::GetUserId => write!(f, "sso-directory:GetUserId"),
            SsoDirectoryActions::GetUserPoolInfo => write!(f, "sso-directory:GetUserPoolInfo"),
            SsoDirectoryActions::ImportExternalIdPCertificate => {
                write!(f, "sso-directory:ImportExternalIdPCertificate")
            }
            SsoDirectoryActions::IsMemberInGroup => write!(f, "sso-directory:IsMemberInGroup"),
            SsoDirectoryActions::IsMemberInGroups => write!(f, "sso-directory:IsMemberInGroups"),
            SsoDirectoryActions::ListBearerTokens => write!(f, "sso-directory:ListBearerTokens"),
            SsoDirectoryActions::ListExternalIdPCertificates => {
                write!(f, "sso-directory:ListExternalIdPCertificates")
            }
            SsoDirectoryActions::ListExternalIdPConfigurationsForDirectory => {
                write!(f, "sso-directory:ListExternalIdPConfigurationsForDirectory")
            }
            SsoDirectoryActions::ListGroups => write!(f, "sso-directory:ListGroups"),
            SsoDirectoryActions::ListGroupsForMember => {
                write!(f, "sso-directory:ListGroupsForMember")
            }
            SsoDirectoryActions::ListGroupsForUser => write!(f, "sso-directory:ListGroupsForUser"),
            SsoDirectoryActions::ListMembersInGroup => {
                write!(f, "sso-directory:ListMembersInGroup")
            }
            SsoDirectoryActions::ListMfaDevicesForUser => {
                write!(f, "sso-directory:ListMfaDevicesForUser")
            }
            SsoDirectoryActions::ListProvisioningTenants => {
                write!(f, "sso-directory:ListProvisioningTenants")
            }
            SsoDirectoryActions::ListUsers => write!(f, "sso-directory:ListUsers"),
            SsoDirectoryActions::RemoveMemberFromGroup => {
                write!(f, "sso-directory:RemoveMemberFromGroup")
            }
            SsoDirectoryActions::SearchGroups => write!(f, "sso-directory:SearchGroups"),
            SsoDirectoryActions::SearchUsers => write!(f, "sso-directory:SearchUsers"),
            SsoDirectoryActions::StartVirtualMfaDeviceRegistration => {
                write!(f, "sso-directory:StartVirtualMfaDeviceRegistration")
            }
            SsoDirectoryActions::StartWebAuthnDeviceRegistration => {
                write!(f, "sso-directory:StartWebAuthnDeviceRegistration")
            }
            SsoDirectoryActions::UpdateExternalIdPConfigurationForDirectory => write!(
                f,
                "sso-directory:UpdateExternalIdPConfigurationForDirectory"
            ),
            SsoDirectoryActions::UpdateGroup => write!(f, "sso-directory:UpdateGroup"),
            SsoDirectoryActions::UpdateGroupDisplayName => {
                write!(f, "sso-directory:UpdateGroupDisplayName")
            }
            SsoDirectoryActions::UpdateMfaDeviceForUser => {
                write!(f, "sso-directory:UpdateMfaDeviceForUser")
            }
            SsoDirectoryActions::UpdatePassword => write!(f, "sso-directory:UpdatePassword"),
            SsoDirectoryActions::UpdateUser => write!(f, "sso-directory:UpdateUser"),
            SsoDirectoryActions::UpdateUserName => write!(f, "sso-directory:UpdateUserName"),
            SsoDirectoryActions::VerifyEmail => write!(f, "sso-directory:VerifyEmail"),
        }
    }
}
