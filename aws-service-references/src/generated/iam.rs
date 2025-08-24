// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum IamActions {
    AddClientIdToOpenIdConnectProvider,
    AddRoleToInstanceProfile,
    AddUserToGroup,
    AttachGroupPolicy,
    AttachRolePolicy,
    AttachUserPolicy,
    ChangePassword,
    CreateAccessKey,
    CreateAccountAlias,
    CreateGroup,
    CreateInstanceProfile,
    CreateLoginProfile,
    CreateOpenIdConnectProvider,
    CreatePolicy,
    CreatePolicyVersion,
    CreateRole,
    CreateSamlProvider,
    CreateServiceLinkedRole,
    CreateServiceSpecificCredential,
    CreateUser,
    CreateVirtualMfaDevice,
    DeactivateMfaDevice,
    DeleteAccessKey,
    DeleteAccountAlias,
    DeleteAccountPasswordPolicy,
    DeleteCloudFrontPublicKey,
    DeleteGroup,
    DeleteGroupPolicy,
    DeleteInstanceProfile,
    DeleteLoginProfile,
    DeleteOpenIdConnectProvider,
    DeletePolicy,
    DeletePolicyVersion,
    DeleteRole,
    DeleteRolePermissionsBoundary,
    DeleteRolePolicy,
    DeleteSamlProvider,
    DeleteServerCertificate,
    DeleteServiceLinkedRole,
    DeleteServiceSpecificCredential,
    DeleteSigningCertificate,
    DeleteSshPublicKey,
    DeleteUser,
    DeleteUserPermissionsBoundary,
    DeleteUserPolicy,
    DeleteVirtualMfaDevice,
    DetachGroupPolicy,
    DetachRolePolicy,
    DetachUserPolicy,
    DisableOrganizationsRootCredentialsManagement,
    DisableOrganizationsRootSessions,
    EnableMfaDevice,
    EnableOrganizationsRootCredentialsManagement,
    EnableOrganizationsRootSessions,
    GenerateCredentialReport,
    GenerateOrganizationsAccessReport,
    GenerateServiceLastAccessedDetails,
    GetAccessKeyLastUsed,
    GetAccountAuthorizationDetails,
    GetAccountEmailAddress,
    GetAccountName,
    GetAccountPasswordPolicy,
    GetAccountSummary,
    GetCloudFrontPublicKey,
    GetContextKeysForCustomPolicy,
    GetContextKeysForPrincipalPolicy,
    GetCredentialReport,
    GetGroup,
    GetGroupPolicy,
    GetInstanceProfile,
    GetLoginProfile,
    GetMfaDevice,
    GetOpenIdConnectProvider,
    GetOrganizationsAccessReport,
    GetPolicy,
    GetPolicyVersion,
    GetRole,
    GetRolePolicy,
    GetSamlProvider,
    GetServerCertificate,
    GetServiceLastAccessedDetails,
    GetServiceLastAccessedDetailsWithEntities,
    GetServiceLinkedRoleDeletionStatus,
    GetSshPublicKey,
    GetUser,
    GetUserPolicy,
    ListAccessKeys,
    ListAccountAliases,
    ListAttachedGroupPolicies,
    ListAttachedRolePolicies,
    ListAttachedUserPolicies,
    ListCloudFrontPublicKeys,
    ListEntitiesForPolicy,
    ListGroupPolicies,
    ListGroups,
    ListGroupsForUser,
    ListInstanceProfileTags,
    ListInstanceProfiles,
    ListInstanceProfilesForRole,
    ListMfaDeviceTags,
    ListMfaDevices,
    ListOpenIdConnectProviderTags,
    ListOpenIdConnectProviders,
    ListOrganizationsFeatures,
    ListPolicies,
    ListPoliciesGrantingServiceAccess,
    ListPolicyTags,
    ListPolicyVersions,
    ListRolePolicies,
    ListRoleTags,
    ListRoles,
    ListSamlProviderTags,
    ListSamlProviders,
    ListServerCertificateTags,
    ListServerCertificates,
    ListServiceSpecificCredentials,
    ListSigningCertificates,
    ListSshPublicKeys,
    ListStsRegionalEndpointsStatus,
    ListUserPolicies,
    ListUserTags,
    ListUsers,
    ListVirtualMfaDevices,
    PassRole,
    PutGroupPolicy,
    PutRolePermissionsBoundary,
    PutRolePolicy,
    PutUserPermissionsBoundary,
    PutUserPolicy,
    RemoveClientIdFromOpenIdConnectProvider,
    RemoveRoleFromInstanceProfile,
    RemoveUserFromGroup,
    ResetServiceSpecificCredential,
    ResyncMfaDevice,
    SetDefaultPolicyVersion,
    SetSecurityTokenServicePreferences,
    SetStsRegionalEndpointStatus,
    SimulateCustomPolicy,
    SimulatePrincipalPolicy,
    TagInstanceProfile,
    TagMfaDevice,
    TagOpenIdConnectProvider,
    TagPolicy,
    TagRole,
    TagSamlProvider,
    TagServerCertificate,
    TagUser,
    UntagInstanceProfile,
    UntagMfaDevice,
    UntagOpenIdConnectProvider,
    UntagPolicy,
    UntagRole,
    UntagSamlProvider,
    UntagServerCertificate,
    UntagUser,
    UpdateAccessKey,
    UpdateAccountEmailAddress,
    UpdateAccountName,
    UpdateAccountPasswordPolicy,
    UpdateAssumeRolePolicy,
    UpdateCloudFrontPublicKey,
    UpdateGroup,
    UpdateLoginProfile,
    UpdateOpenIdConnectProviderThumbprint,
    UpdateRole,
    UpdateRoleDescription,
    UpdateSamlProvider,
    UpdateServerCertificate,
    UpdateServiceSpecificCredential,
    UpdateSigningCertificate,
    UpdateSshPublicKey,
    UpdateUser,
    UploadCloudFrontPublicKey,
    UploadServerCertificate,
    UploadSigningCertificate,
    UploadSshPublicKey,
}
impl std::fmt::Display for IamActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IamActions::AddClientIdToOpenIdConnectProvider => {
                write!(f, "iam:AddClientIDToOpenIDConnectProvider")
            }
            IamActions::AddRoleToInstanceProfile => write!(f, "iam:AddRoleToInstanceProfile"),
            IamActions::AddUserToGroup => write!(f, "iam:AddUserToGroup"),
            IamActions::AttachGroupPolicy => write!(f, "iam:AttachGroupPolicy"),
            IamActions::AttachRolePolicy => write!(f, "iam:AttachRolePolicy"),
            IamActions::AttachUserPolicy => write!(f, "iam:AttachUserPolicy"),
            IamActions::ChangePassword => write!(f, "iam:ChangePassword"),
            IamActions::CreateAccessKey => write!(f, "iam:CreateAccessKey"),
            IamActions::CreateAccountAlias => write!(f, "iam:CreateAccountAlias"),
            IamActions::CreateGroup => write!(f, "iam:CreateGroup"),
            IamActions::CreateInstanceProfile => write!(f, "iam:CreateInstanceProfile"),
            IamActions::CreateLoginProfile => write!(f, "iam:CreateLoginProfile"),
            IamActions::CreateOpenIdConnectProvider => write!(f, "iam:CreateOpenIDConnectProvider"),
            IamActions::CreatePolicy => write!(f, "iam:CreatePolicy"),
            IamActions::CreatePolicyVersion => write!(f, "iam:CreatePolicyVersion"),
            IamActions::CreateRole => write!(f, "iam:CreateRole"),
            IamActions::CreateSamlProvider => write!(f, "iam:CreateSAMLProvider"),
            IamActions::CreateServiceLinkedRole => write!(f, "iam:CreateServiceLinkedRole"),
            IamActions::CreateServiceSpecificCredential => {
                write!(f, "iam:CreateServiceSpecificCredential")
            }
            IamActions::CreateUser => write!(f, "iam:CreateUser"),
            IamActions::CreateVirtualMfaDevice => write!(f, "iam:CreateVirtualMFADevice"),
            IamActions::DeactivateMfaDevice => write!(f, "iam:DeactivateMFADevice"),
            IamActions::DeleteAccessKey => write!(f, "iam:DeleteAccessKey"),
            IamActions::DeleteAccountAlias => write!(f, "iam:DeleteAccountAlias"),
            IamActions::DeleteAccountPasswordPolicy => write!(f, "iam:DeleteAccountPasswordPolicy"),
            IamActions::DeleteCloudFrontPublicKey => write!(f, "iam:DeleteCloudFrontPublicKey"),
            IamActions::DeleteGroup => write!(f, "iam:DeleteGroup"),
            IamActions::DeleteGroupPolicy => write!(f, "iam:DeleteGroupPolicy"),
            IamActions::DeleteInstanceProfile => write!(f, "iam:DeleteInstanceProfile"),
            IamActions::DeleteLoginProfile => write!(f, "iam:DeleteLoginProfile"),
            IamActions::DeleteOpenIdConnectProvider => write!(f, "iam:DeleteOpenIDConnectProvider"),
            IamActions::DeletePolicy => write!(f, "iam:DeletePolicy"),
            IamActions::DeletePolicyVersion => write!(f, "iam:DeletePolicyVersion"),
            IamActions::DeleteRole => write!(f, "iam:DeleteRole"),
            IamActions::DeleteRolePermissionsBoundary => {
                write!(f, "iam:DeleteRolePermissionsBoundary")
            }
            IamActions::DeleteRolePolicy => write!(f, "iam:DeleteRolePolicy"),
            IamActions::DeleteSamlProvider => write!(f, "iam:DeleteSAMLProvider"),
            IamActions::DeleteServerCertificate => write!(f, "iam:DeleteServerCertificate"),
            IamActions::DeleteServiceLinkedRole => write!(f, "iam:DeleteServiceLinkedRole"),
            IamActions::DeleteServiceSpecificCredential => {
                write!(f, "iam:DeleteServiceSpecificCredential")
            }
            IamActions::DeleteSigningCertificate => write!(f, "iam:DeleteSigningCertificate"),
            IamActions::DeleteSshPublicKey => write!(f, "iam:DeleteSSHPublicKey"),
            IamActions::DeleteUser => write!(f, "iam:DeleteUser"),
            IamActions::DeleteUserPermissionsBoundary => {
                write!(f, "iam:DeleteUserPermissionsBoundary")
            }
            IamActions::DeleteUserPolicy => write!(f, "iam:DeleteUserPolicy"),
            IamActions::DeleteVirtualMfaDevice => write!(f, "iam:DeleteVirtualMFADevice"),
            IamActions::DetachGroupPolicy => write!(f, "iam:DetachGroupPolicy"),
            IamActions::DetachRolePolicy => write!(f, "iam:DetachRolePolicy"),
            IamActions::DetachUserPolicy => write!(f, "iam:DetachUserPolicy"),
            IamActions::DisableOrganizationsRootCredentialsManagement => {
                write!(f, "iam:DisableOrganizationsRootCredentialsManagement")
            }
            IamActions::DisableOrganizationsRootSessions => {
                write!(f, "iam:DisableOrganizationsRootSessions")
            }
            IamActions::EnableMfaDevice => write!(f, "iam:EnableMFADevice"),
            IamActions::EnableOrganizationsRootCredentialsManagement => {
                write!(f, "iam:EnableOrganizationsRootCredentialsManagement")
            }
            IamActions::EnableOrganizationsRootSessions => {
                write!(f, "iam:EnableOrganizationsRootSessions")
            }
            IamActions::GenerateCredentialReport => write!(f, "iam:GenerateCredentialReport"),
            IamActions::GenerateOrganizationsAccessReport => {
                write!(f, "iam:GenerateOrganizationsAccessReport")
            }
            IamActions::GenerateServiceLastAccessedDetails => {
                write!(f, "iam:GenerateServiceLastAccessedDetails")
            }
            IamActions::GetAccessKeyLastUsed => write!(f, "iam:GetAccessKeyLastUsed"),
            IamActions::GetAccountAuthorizationDetails => {
                write!(f, "iam:GetAccountAuthorizationDetails")
            }
            IamActions::GetAccountEmailAddress => write!(f, "iam:GetAccountEmailAddress"),
            IamActions::GetAccountName => write!(f, "iam:GetAccountName"),
            IamActions::GetAccountPasswordPolicy => write!(f, "iam:GetAccountPasswordPolicy"),
            IamActions::GetAccountSummary => write!(f, "iam:GetAccountSummary"),
            IamActions::GetCloudFrontPublicKey => write!(f, "iam:GetCloudFrontPublicKey"),
            IamActions::GetContextKeysForCustomPolicy => {
                write!(f, "iam:GetContextKeysForCustomPolicy")
            }
            IamActions::GetContextKeysForPrincipalPolicy => {
                write!(f, "iam:GetContextKeysForPrincipalPolicy")
            }
            IamActions::GetCredentialReport => write!(f, "iam:GetCredentialReport"),
            IamActions::GetGroup => write!(f, "iam:GetGroup"),
            IamActions::GetGroupPolicy => write!(f, "iam:GetGroupPolicy"),
            IamActions::GetInstanceProfile => write!(f, "iam:GetInstanceProfile"),
            IamActions::GetLoginProfile => write!(f, "iam:GetLoginProfile"),
            IamActions::GetMfaDevice => write!(f, "iam:GetMFADevice"),
            IamActions::GetOpenIdConnectProvider => write!(f, "iam:GetOpenIDConnectProvider"),
            IamActions::GetOrganizationsAccessReport => {
                write!(f, "iam:GetOrganizationsAccessReport")
            }
            IamActions::GetPolicy => write!(f, "iam:GetPolicy"),
            IamActions::GetPolicyVersion => write!(f, "iam:GetPolicyVersion"),
            IamActions::GetRole => write!(f, "iam:GetRole"),
            IamActions::GetRolePolicy => write!(f, "iam:GetRolePolicy"),
            IamActions::GetSamlProvider => write!(f, "iam:GetSAMLProvider"),
            IamActions::GetServerCertificate => write!(f, "iam:GetServerCertificate"),
            IamActions::GetServiceLastAccessedDetails => {
                write!(f, "iam:GetServiceLastAccessedDetails")
            }
            IamActions::GetServiceLastAccessedDetailsWithEntities => {
                write!(f, "iam:GetServiceLastAccessedDetailsWithEntities")
            }
            IamActions::GetServiceLinkedRoleDeletionStatus => {
                write!(f, "iam:GetServiceLinkedRoleDeletionStatus")
            }
            IamActions::GetSshPublicKey => write!(f, "iam:GetSSHPublicKey"),
            IamActions::GetUser => write!(f, "iam:GetUser"),
            IamActions::GetUserPolicy => write!(f, "iam:GetUserPolicy"),
            IamActions::ListAccessKeys => write!(f, "iam:ListAccessKeys"),
            IamActions::ListAccountAliases => write!(f, "iam:ListAccountAliases"),
            IamActions::ListAttachedGroupPolicies => write!(f, "iam:ListAttachedGroupPolicies"),
            IamActions::ListAttachedRolePolicies => write!(f, "iam:ListAttachedRolePolicies"),
            IamActions::ListAttachedUserPolicies => write!(f, "iam:ListAttachedUserPolicies"),
            IamActions::ListCloudFrontPublicKeys => write!(f, "iam:ListCloudFrontPublicKeys"),
            IamActions::ListEntitiesForPolicy => write!(f, "iam:ListEntitiesForPolicy"),
            IamActions::ListGroupPolicies => write!(f, "iam:ListGroupPolicies"),
            IamActions::ListGroups => write!(f, "iam:ListGroups"),
            IamActions::ListGroupsForUser => write!(f, "iam:ListGroupsForUser"),
            IamActions::ListInstanceProfileTags => write!(f, "iam:ListInstanceProfileTags"),
            IamActions::ListInstanceProfiles => write!(f, "iam:ListInstanceProfiles"),
            IamActions::ListInstanceProfilesForRole => write!(f, "iam:ListInstanceProfilesForRole"),
            IamActions::ListMfaDeviceTags => write!(f, "iam:ListMFADeviceTags"),
            IamActions::ListMfaDevices => write!(f, "iam:ListMFADevices"),
            IamActions::ListOpenIdConnectProviderTags => {
                write!(f, "iam:ListOpenIDConnectProviderTags")
            }
            IamActions::ListOpenIdConnectProviders => write!(f, "iam:ListOpenIDConnectProviders"),
            IamActions::ListOrganizationsFeatures => write!(f, "iam:ListOrganizationsFeatures"),
            IamActions::ListPolicies => write!(f, "iam:ListPolicies"),
            IamActions::ListPoliciesGrantingServiceAccess => {
                write!(f, "iam:ListPoliciesGrantingServiceAccess")
            }
            IamActions::ListPolicyTags => write!(f, "iam:ListPolicyTags"),
            IamActions::ListPolicyVersions => write!(f, "iam:ListPolicyVersions"),
            IamActions::ListRolePolicies => write!(f, "iam:ListRolePolicies"),
            IamActions::ListRoleTags => write!(f, "iam:ListRoleTags"),
            IamActions::ListRoles => write!(f, "iam:ListRoles"),
            IamActions::ListSamlProviderTags => write!(f, "iam:ListSAMLProviderTags"),
            IamActions::ListSamlProviders => write!(f, "iam:ListSAMLProviders"),
            IamActions::ListServerCertificateTags => write!(f, "iam:ListServerCertificateTags"),
            IamActions::ListServerCertificates => write!(f, "iam:ListServerCertificates"),
            IamActions::ListServiceSpecificCredentials => {
                write!(f, "iam:ListServiceSpecificCredentials")
            }
            IamActions::ListSigningCertificates => write!(f, "iam:ListSigningCertificates"),
            IamActions::ListSshPublicKeys => write!(f, "iam:ListSSHPublicKeys"),
            IamActions::ListStsRegionalEndpointsStatus => {
                write!(f, "iam:ListSTSRegionalEndpointsStatus")
            }
            IamActions::ListUserPolicies => write!(f, "iam:ListUserPolicies"),
            IamActions::ListUserTags => write!(f, "iam:ListUserTags"),
            IamActions::ListUsers => write!(f, "iam:ListUsers"),
            IamActions::ListVirtualMfaDevices => write!(f, "iam:ListVirtualMFADevices"),
            IamActions::PassRole => write!(f, "iam:PassRole"),
            IamActions::PutGroupPolicy => write!(f, "iam:PutGroupPolicy"),
            IamActions::PutRolePermissionsBoundary => write!(f, "iam:PutRolePermissionsBoundary"),
            IamActions::PutRolePolicy => write!(f, "iam:PutRolePolicy"),
            IamActions::PutUserPermissionsBoundary => write!(f, "iam:PutUserPermissionsBoundary"),
            IamActions::PutUserPolicy => write!(f, "iam:PutUserPolicy"),
            IamActions::RemoveClientIdFromOpenIdConnectProvider => {
                write!(f, "iam:RemoveClientIDFromOpenIDConnectProvider")
            }
            IamActions::RemoveRoleFromInstanceProfile => {
                write!(f, "iam:RemoveRoleFromInstanceProfile")
            }
            IamActions::RemoveUserFromGroup => write!(f, "iam:RemoveUserFromGroup"),
            IamActions::ResetServiceSpecificCredential => {
                write!(f, "iam:ResetServiceSpecificCredential")
            }
            IamActions::ResyncMfaDevice => write!(f, "iam:ResyncMFADevice"),
            IamActions::SetDefaultPolicyVersion => write!(f, "iam:SetDefaultPolicyVersion"),
            IamActions::SetSecurityTokenServicePreferences => {
                write!(f, "iam:SetSecurityTokenServicePreferences")
            }
            IamActions::SetStsRegionalEndpointStatus => {
                write!(f, "iam:SetSTSRegionalEndpointStatus")
            }
            IamActions::SimulateCustomPolicy => write!(f, "iam:SimulateCustomPolicy"),
            IamActions::SimulatePrincipalPolicy => write!(f, "iam:SimulatePrincipalPolicy"),
            IamActions::TagInstanceProfile => write!(f, "iam:TagInstanceProfile"),
            IamActions::TagMfaDevice => write!(f, "iam:TagMFADevice"),
            IamActions::TagOpenIdConnectProvider => write!(f, "iam:TagOpenIDConnectProvider"),
            IamActions::TagPolicy => write!(f, "iam:TagPolicy"),
            IamActions::TagRole => write!(f, "iam:TagRole"),
            IamActions::TagSamlProvider => write!(f, "iam:TagSAMLProvider"),
            IamActions::TagServerCertificate => write!(f, "iam:TagServerCertificate"),
            IamActions::TagUser => write!(f, "iam:TagUser"),
            IamActions::UntagInstanceProfile => write!(f, "iam:UntagInstanceProfile"),
            IamActions::UntagMfaDevice => write!(f, "iam:UntagMFADevice"),
            IamActions::UntagOpenIdConnectProvider => write!(f, "iam:UntagOpenIDConnectProvider"),
            IamActions::UntagPolicy => write!(f, "iam:UntagPolicy"),
            IamActions::UntagRole => write!(f, "iam:UntagRole"),
            IamActions::UntagSamlProvider => write!(f, "iam:UntagSAMLProvider"),
            IamActions::UntagServerCertificate => write!(f, "iam:UntagServerCertificate"),
            IamActions::UntagUser => write!(f, "iam:UntagUser"),
            IamActions::UpdateAccessKey => write!(f, "iam:UpdateAccessKey"),
            IamActions::UpdateAccountEmailAddress => write!(f, "iam:UpdateAccountEmailAddress"),
            IamActions::UpdateAccountName => write!(f, "iam:UpdateAccountName"),
            IamActions::UpdateAccountPasswordPolicy => write!(f, "iam:UpdateAccountPasswordPolicy"),
            IamActions::UpdateAssumeRolePolicy => write!(f, "iam:UpdateAssumeRolePolicy"),
            IamActions::UpdateCloudFrontPublicKey => write!(f, "iam:UpdateCloudFrontPublicKey"),
            IamActions::UpdateGroup => write!(f, "iam:UpdateGroup"),
            IamActions::UpdateLoginProfile => write!(f, "iam:UpdateLoginProfile"),
            IamActions::UpdateOpenIdConnectProviderThumbprint => {
                write!(f, "iam:UpdateOpenIDConnectProviderThumbprint")
            }
            IamActions::UpdateRole => write!(f, "iam:UpdateRole"),
            IamActions::UpdateRoleDescription => write!(f, "iam:UpdateRoleDescription"),
            IamActions::UpdateSamlProvider => write!(f, "iam:UpdateSAMLProvider"),
            IamActions::UpdateServerCertificate => write!(f, "iam:UpdateServerCertificate"),
            IamActions::UpdateServiceSpecificCredential => {
                write!(f, "iam:UpdateServiceSpecificCredential")
            }
            IamActions::UpdateSigningCertificate => write!(f, "iam:UpdateSigningCertificate"),
            IamActions::UpdateSshPublicKey => write!(f, "iam:UpdateSSHPublicKey"),
            IamActions::UpdateUser => write!(f, "iam:UpdateUser"),
            IamActions::UploadCloudFrontPublicKey => write!(f, "iam:UploadCloudFrontPublicKey"),
            IamActions::UploadServerCertificate => write!(f, "iam:UploadServerCertificate"),
            IamActions::UploadSigningCertificate => write!(f, "iam:UploadSigningCertificate"),
            IamActions::UploadSshPublicKey => write!(f, "iam:UploadSSHPublicKey"),
        }
    }
}
