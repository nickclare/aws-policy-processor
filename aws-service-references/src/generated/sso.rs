// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum SsoActions {
    AssociateDirectory,
    AssociateProfile,
    AttachCustomerManagedPolicyReferenceToPermissionSet,
    AttachManagedPolicyToPermissionSet,
    CreateAccountAssignment,
    CreateApplication,
    CreateApplicationAssignment,
    CreateApplicationInstance,
    CreateApplicationInstanceCertificate,
    CreateInstance,
    CreateInstanceAccessControlAttributeConfiguration,
    CreateManagedApplicationInstance,
    CreatePermissionSet,
    CreateProfile,
    CreateTrust,
    CreateTrustedTokenIssuer,
    DeleteAccountAssignment,
    DeleteApplication,
    DeleteApplicationAccessScope,
    DeleteApplicationAssignment,
    DeleteApplicationAuthenticationMethod,
    DeleteApplicationGrant,
    DeleteApplicationInstance,
    DeleteApplicationInstanceCertificate,
    DeleteInlinePolicyFromPermissionSet,
    DeleteInstance,
    DeleteInstanceAccessControlAttributeConfiguration,
    DeleteManagedApplicationInstance,
    DeletePermissionSet,
    DeletePermissionsBoundaryFromPermissionSet,
    DeleteProfile,
    DeleteTrustedTokenIssuer,
    DescribeAccountAssignmentCreationStatus,
    DescribeAccountAssignmentDeletionStatus,
    DescribeApplication,
    DescribeApplicationAssignment,
    DescribeApplicationProvider,
    DescribeInstance,
    DescribeInstanceAccessControlAttributeConfiguration,
    DescribePermissionSet,
    DescribePermissionSetProvisioningStatus,
    DescribeRegisteredRegions,
    DescribeTrustedTokenIssuer,
    DetachCustomerManagedPolicyReferenceFromPermissionSet,
    DetachManagedPolicyFromPermissionSet,
    DisassociateDirectory,
    DisassociateProfile,
    GetApplicationAccessScope,
    GetApplicationAssignmentConfiguration,
    GetApplicationAuthenticationMethod,
    GetApplicationGrant,
    GetApplicationInstance,
    GetApplicationSessionConfiguration,
    GetApplicationTemplate,
    GetInlinePolicyForPermissionSet,
    GetManagedApplicationInstance,
    GetMfaDeviceManagementForDirectory,
    GetPermissionSet,
    GetPermissionsBoundaryForPermissionSet,
    GetProfile,
    GetSharedSsoConfiguration,
    GetSsoConfiguration,
    GetSsoStatus,
    GetTrust,
    ImportApplicationInstanceServiceProviderMetadata,
    ListAccountAssignmentCreationStatus,
    ListAccountAssignmentDeletionStatus,
    ListAccountAssignments,
    ListAccountAssignmentsForPrincipal,
    ListAccountsForProvisionedPermissionSet,
    ListApplicationAccessScopes,
    ListApplicationAssignments,
    ListApplicationAssignmentsForPrincipal,
    ListApplicationAuthenticationMethods,
    ListApplicationGrants,
    ListApplicationInstanceCertificates,
    ListApplicationInstances,
    ListApplicationProviders,
    ListApplicationTemplates,
    ListApplications,
    ListCustomerManagedPolicyReferencesInPermissionSet,
    ListDirectoryAssociations,
    ListInstances,
    ListManagedPoliciesInPermissionSet,
    ListPermissionSetProvisioningStatus,
    ListPermissionSets,
    ListPermissionSetsProvisionedToAccount,
    ListProfileAssociations,
    ListProfiles,
    ListTagsForResource,
    ListTrustedTokenIssuers,
    ProvisionPermissionSet,
    PutApplicationAccessScope,
    PutApplicationAssignmentConfiguration,
    PutApplicationAuthenticationMethod,
    PutApplicationGrant,
    PutApplicationSessionConfiguration,
    PutInlinePolicyToPermissionSet,
    PutMfaDeviceManagementForDirectory,
    PutPermissionsBoundaryToPermissionSet,
    PutPermissionsPolicy,
    SearchGroups,
    SearchUsers,
    StartSso,
    TagResource,
    UntagResource,
    UpdateApplication,
    UpdateApplicationInstanceActiveCertificate,
    UpdateApplicationInstanceDisplayData,
    UpdateApplicationInstanceResponseConfiguration,
    UpdateApplicationInstanceResponseSchemaConfiguration,
    UpdateApplicationInstanceSecurityConfiguration,
    UpdateApplicationInstanceServiceProviderConfiguration,
    UpdateApplicationInstanceStatus,
    UpdateInstance,
    UpdateInstanceAccessControlAttributeConfiguration,
    UpdateManagedApplicationInstanceStatus,
    UpdatePermissionSet,
    UpdateProfile,
    UpdateSsoConfiguration,
    UpdateTrust,
    UpdateTrustedTokenIssuer,
}
impl std::fmt::Display for SsoActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SsoActions::AssociateDirectory => write!(f, "sso:AssociateDirectory"),
            SsoActions::AssociateProfile => write!(f, "sso:AssociateProfile"),
            SsoActions::AttachCustomerManagedPolicyReferenceToPermissionSet => {
                write!(f, "sso:AttachCustomerManagedPolicyReferenceToPermissionSet")
            }
            SsoActions::AttachManagedPolicyToPermissionSet => {
                write!(f, "sso:AttachManagedPolicyToPermissionSet")
            }
            SsoActions::CreateAccountAssignment => write!(f, "sso:CreateAccountAssignment"),
            SsoActions::CreateApplication => write!(f, "sso:CreateApplication"),
            SsoActions::CreateApplicationAssignment => write!(f, "sso:CreateApplicationAssignment"),
            SsoActions::CreateApplicationInstance => write!(f, "sso:CreateApplicationInstance"),
            SsoActions::CreateApplicationInstanceCertificate => {
                write!(f, "sso:CreateApplicationInstanceCertificate")
            }
            SsoActions::CreateInstance => write!(f, "sso:CreateInstance"),
            SsoActions::CreateInstanceAccessControlAttributeConfiguration => {
                write!(f, "sso:CreateInstanceAccessControlAttributeConfiguration")
            }
            SsoActions::CreateManagedApplicationInstance => {
                write!(f, "sso:CreateManagedApplicationInstance")
            }
            SsoActions::CreatePermissionSet => write!(f, "sso:CreatePermissionSet"),
            SsoActions::CreateProfile => write!(f, "sso:CreateProfile"),
            SsoActions::CreateTrust => write!(f, "sso:CreateTrust"),
            SsoActions::CreateTrustedTokenIssuer => write!(f, "sso:CreateTrustedTokenIssuer"),
            SsoActions::DeleteAccountAssignment => write!(f, "sso:DeleteAccountAssignment"),
            SsoActions::DeleteApplication => write!(f, "sso:DeleteApplication"),
            SsoActions::DeleteApplicationAccessScope => {
                write!(f, "sso:DeleteApplicationAccessScope")
            }
            SsoActions::DeleteApplicationAssignment => write!(f, "sso:DeleteApplicationAssignment"),
            SsoActions::DeleteApplicationAuthenticationMethod => {
                write!(f, "sso:DeleteApplicationAuthenticationMethod")
            }
            SsoActions::DeleteApplicationGrant => write!(f, "sso:DeleteApplicationGrant"),
            SsoActions::DeleteApplicationInstance => write!(f, "sso:DeleteApplicationInstance"),
            SsoActions::DeleteApplicationInstanceCertificate => {
                write!(f, "sso:DeleteApplicationInstanceCertificate")
            }
            SsoActions::DeleteInlinePolicyFromPermissionSet => {
                write!(f, "sso:DeleteInlinePolicyFromPermissionSet")
            }
            SsoActions::DeleteInstance => write!(f, "sso:DeleteInstance"),
            SsoActions::DeleteInstanceAccessControlAttributeConfiguration => {
                write!(f, "sso:DeleteInstanceAccessControlAttributeConfiguration")
            }
            SsoActions::DeleteManagedApplicationInstance => {
                write!(f, "sso:DeleteManagedApplicationInstance")
            }
            SsoActions::DeletePermissionSet => write!(f, "sso:DeletePermissionSet"),
            SsoActions::DeletePermissionsBoundaryFromPermissionSet => {
                write!(f, "sso:DeletePermissionsBoundaryFromPermissionSet")
            }
            SsoActions::DeleteProfile => write!(f, "sso:DeleteProfile"),
            SsoActions::DeleteTrustedTokenIssuer => write!(f, "sso:DeleteTrustedTokenIssuer"),
            SsoActions::DescribeAccountAssignmentCreationStatus => {
                write!(f, "sso:DescribeAccountAssignmentCreationStatus")
            }
            SsoActions::DescribeAccountAssignmentDeletionStatus => {
                write!(f, "sso:DescribeAccountAssignmentDeletionStatus")
            }
            SsoActions::DescribeApplication => write!(f, "sso:DescribeApplication"),
            SsoActions::DescribeApplicationAssignment => {
                write!(f, "sso:DescribeApplicationAssignment")
            }
            SsoActions::DescribeApplicationProvider => write!(f, "sso:DescribeApplicationProvider"),
            SsoActions::DescribeInstance => write!(f, "sso:DescribeInstance"),
            SsoActions::DescribeInstanceAccessControlAttributeConfiguration => {
                write!(f, "sso:DescribeInstanceAccessControlAttributeConfiguration")
            }
            SsoActions::DescribePermissionSet => write!(f, "sso:DescribePermissionSet"),
            SsoActions::DescribePermissionSetProvisioningStatus => {
                write!(f, "sso:DescribePermissionSetProvisioningStatus")
            }
            SsoActions::DescribeRegisteredRegions => write!(f, "sso:DescribeRegisteredRegions"),
            SsoActions::DescribeTrustedTokenIssuer => write!(f, "sso:DescribeTrustedTokenIssuer"),
            SsoActions::DetachCustomerManagedPolicyReferenceFromPermissionSet => write!(
                f,
                "sso:DetachCustomerManagedPolicyReferenceFromPermissionSet"
            ),
            SsoActions::DetachManagedPolicyFromPermissionSet => {
                write!(f, "sso:DetachManagedPolicyFromPermissionSet")
            }
            SsoActions::DisassociateDirectory => write!(f, "sso:DisassociateDirectory"),
            SsoActions::DisassociateProfile => write!(f, "sso:DisassociateProfile"),
            SsoActions::GetApplicationAccessScope => write!(f, "sso:GetApplicationAccessScope"),
            SsoActions::GetApplicationAssignmentConfiguration => {
                write!(f, "sso:GetApplicationAssignmentConfiguration")
            }
            SsoActions::GetApplicationAuthenticationMethod => {
                write!(f, "sso:GetApplicationAuthenticationMethod")
            }
            SsoActions::GetApplicationGrant => write!(f, "sso:GetApplicationGrant"),
            SsoActions::GetApplicationInstance => write!(f, "sso:GetApplicationInstance"),
            SsoActions::GetApplicationSessionConfiguration => {
                write!(f, "sso:GetApplicationSessionConfiguration")
            }
            SsoActions::GetApplicationTemplate => write!(f, "sso:GetApplicationTemplate"),
            SsoActions::GetInlinePolicyForPermissionSet => {
                write!(f, "sso:GetInlinePolicyForPermissionSet")
            }
            SsoActions::GetManagedApplicationInstance => {
                write!(f, "sso:GetManagedApplicationInstance")
            }
            SsoActions::GetMfaDeviceManagementForDirectory => {
                write!(f, "sso:GetMfaDeviceManagementForDirectory")
            }
            SsoActions::GetPermissionSet => write!(f, "sso:GetPermissionSet"),
            SsoActions::GetPermissionsBoundaryForPermissionSet => {
                write!(f, "sso:GetPermissionsBoundaryForPermissionSet")
            }
            SsoActions::GetProfile => write!(f, "sso:GetProfile"),
            SsoActions::GetSharedSsoConfiguration => write!(f, "sso:GetSharedSsoConfiguration"),
            SsoActions::GetSsoConfiguration => write!(f, "sso:GetSsoConfiguration"),
            SsoActions::GetSsoStatus => write!(f, "sso:GetSSOStatus"),
            SsoActions::GetTrust => write!(f, "sso:GetTrust"),
            SsoActions::ImportApplicationInstanceServiceProviderMetadata => {
                write!(f, "sso:ImportApplicationInstanceServiceProviderMetadata")
            }
            SsoActions::ListAccountAssignmentCreationStatus => {
                write!(f, "sso:ListAccountAssignmentCreationStatus")
            }
            SsoActions::ListAccountAssignmentDeletionStatus => {
                write!(f, "sso:ListAccountAssignmentDeletionStatus")
            }
            SsoActions::ListAccountAssignments => write!(f, "sso:ListAccountAssignments"),
            SsoActions::ListAccountAssignmentsForPrincipal => {
                write!(f, "sso:ListAccountAssignmentsForPrincipal")
            }
            SsoActions::ListAccountsForProvisionedPermissionSet => {
                write!(f, "sso:ListAccountsForProvisionedPermissionSet")
            }
            SsoActions::ListApplicationAccessScopes => write!(f, "sso:ListApplicationAccessScopes"),
            SsoActions::ListApplicationAssignments => write!(f, "sso:ListApplicationAssignments"),
            SsoActions::ListApplicationAssignmentsForPrincipal => {
                write!(f, "sso:ListApplicationAssignmentsForPrincipal")
            }
            SsoActions::ListApplicationAuthenticationMethods => {
                write!(f, "sso:ListApplicationAuthenticationMethods")
            }
            SsoActions::ListApplicationGrants => write!(f, "sso:ListApplicationGrants"),
            SsoActions::ListApplicationInstanceCertificates => {
                write!(f, "sso:ListApplicationInstanceCertificates")
            }
            SsoActions::ListApplicationInstances => write!(f, "sso:ListApplicationInstances"),
            SsoActions::ListApplicationProviders => write!(f, "sso:ListApplicationProviders"),
            SsoActions::ListApplicationTemplates => write!(f, "sso:ListApplicationTemplates"),
            SsoActions::ListApplications => write!(f, "sso:ListApplications"),
            SsoActions::ListCustomerManagedPolicyReferencesInPermissionSet => {
                write!(f, "sso:ListCustomerManagedPolicyReferencesInPermissionSet")
            }
            SsoActions::ListDirectoryAssociations => write!(f, "sso:ListDirectoryAssociations"),
            SsoActions::ListInstances => write!(f, "sso:ListInstances"),
            SsoActions::ListManagedPoliciesInPermissionSet => {
                write!(f, "sso:ListManagedPoliciesInPermissionSet")
            }
            SsoActions::ListPermissionSetProvisioningStatus => {
                write!(f, "sso:ListPermissionSetProvisioningStatus")
            }
            SsoActions::ListPermissionSets => write!(f, "sso:ListPermissionSets"),
            SsoActions::ListPermissionSetsProvisionedToAccount => {
                write!(f, "sso:ListPermissionSetsProvisionedToAccount")
            }
            SsoActions::ListProfileAssociations => write!(f, "sso:ListProfileAssociations"),
            SsoActions::ListProfiles => write!(f, "sso:ListProfiles"),
            SsoActions::ListTagsForResource => write!(f, "sso:ListTagsForResource"),
            SsoActions::ListTrustedTokenIssuers => write!(f, "sso:ListTrustedTokenIssuers"),
            SsoActions::ProvisionPermissionSet => write!(f, "sso:ProvisionPermissionSet"),
            SsoActions::PutApplicationAccessScope => write!(f, "sso:PutApplicationAccessScope"),
            SsoActions::PutApplicationAssignmentConfiguration => {
                write!(f, "sso:PutApplicationAssignmentConfiguration")
            }
            SsoActions::PutApplicationAuthenticationMethod => {
                write!(f, "sso:PutApplicationAuthenticationMethod")
            }
            SsoActions::PutApplicationGrant => write!(f, "sso:PutApplicationGrant"),
            SsoActions::PutApplicationSessionConfiguration => {
                write!(f, "sso:PutApplicationSessionConfiguration")
            }
            SsoActions::PutInlinePolicyToPermissionSet => {
                write!(f, "sso:PutInlinePolicyToPermissionSet")
            }
            SsoActions::PutMfaDeviceManagementForDirectory => {
                write!(f, "sso:PutMfaDeviceManagementForDirectory")
            }
            SsoActions::PutPermissionsBoundaryToPermissionSet => {
                write!(f, "sso:PutPermissionsBoundaryToPermissionSet")
            }
            SsoActions::PutPermissionsPolicy => write!(f, "sso:PutPermissionsPolicy"),
            SsoActions::SearchGroups => write!(f, "sso:SearchGroups"),
            SsoActions::SearchUsers => write!(f, "sso:SearchUsers"),
            SsoActions::StartSso => write!(f, "sso:StartSSO"),
            SsoActions::TagResource => write!(f, "sso:TagResource"),
            SsoActions::UntagResource => write!(f, "sso:UntagResource"),
            SsoActions::UpdateApplication => write!(f, "sso:UpdateApplication"),
            SsoActions::UpdateApplicationInstanceActiveCertificate => {
                write!(f, "sso:UpdateApplicationInstanceActiveCertificate")
            }
            SsoActions::UpdateApplicationInstanceDisplayData => {
                write!(f, "sso:UpdateApplicationInstanceDisplayData")
            }
            SsoActions::UpdateApplicationInstanceResponseConfiguration => {
                write!(f, "sso:UpdateApplicationInstanceResponseConfiguration")
            }
            SsoActions::UpdateApplicationInstanceResponseSchemaConfiguration => write!(
                f,
                "sso:UpdateApplicationInstanceResponseSchemaConfiguration"
            ),
            SsoActions::UpdateApplicationInstanceSecurityConfiguration => {
                write!(f, "sso:UpdateApplicationInstanceSecurityConfiguration")
            }
            SsoActions::UpdateApplicationInstanceServiceProviderConfiguration => write!(
                f,
                "sso:UpdateApplicationInstanceServiceProviderConfiguration"
            ),
            SsoActions::UpdateApplicationInstanceStatus => {
                write!(f, "sso:UpdateApplicationInstanceStatus")
            }
            SsoActions::UpdateInstance => write!(f, "sso:UpdateInstance"),
            SsoActions::UpdateInstanceAccessControlAttributeConfiguration => {
                write!(f, "sso:UpdateInstanceAccessControlAttributeConfiguration")
            }
            SsoActions::UpdateManagedApplicationInstanceStatus => {
                write!(f, "sso:UpdateManagedApplicationInstanceStatus")
            }
            SsoActions::UpdatePermissionSet => write!(f, "sso:UpdatePermissionSet"),
            SsoActions::UpdateProfile => write!(f, "sso:UpdateProfile"),
            SsoActions::UpdateSsoConfiguration => write!(f, "sso:UpdateSSOConfiguration"),
            SsoActions::UpdateTrust => write!(f, "sso:UpdateTrust"),
            SsoActions::UpdateTrustedTokenIssuer => write!(f, "sso:UpdateTrustedTokenIssuer"),
        }
    }
}
