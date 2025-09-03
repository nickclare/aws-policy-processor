// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum CognitoIdpActions {
    AddCustomAttributes,
    AdminAddUserToGroup,
    AdminConfirmSignUp,
    AdminCreateUser,
    AdminDeleteUser,
    AdminDeleteUserAttributes,
    AdminDisableProviderForUser,
    AdminDisableUser,
    AdminEnableUser,
    AdminForgetDevice,
    AdminGetDevice,
    AdminGetUser,
    AdminInitiateAuth,
    AdminLinkProviderForUser,
    AdminListDevices,
    AdminListGroupsForUser,
    AdminListUserAuthEvents,
    AdminRemoveUserFromGroup,
    AdminResetUserPassword,
    AdminRespondToAuthChallenge,
    AdminSetUserMfaPreference,
    AdminSetUserPassword,
    AdminSetUserSettings,
    AdminUpdateAuthEventFeedback,
    AdminUpdateDeviceStatus,
    AdminUpdateUserAttributes,
    AdminUserGlobalSignOut,
    AssociateSoftwareToken,
    AssociateWebAcl,
    ChangePassword,
    ConfirmDevice,
    ConfirmForgotPassword,
    ConfirmSignUp,
    CreateGroup,
    CreateIdentityProvider,
    CreateManagedLoginBranding,
    CreateResourceServer,
    CreateUserImportJob,
    CreateUserPool,
    CreateUserPoolClient,
    CreateUserPoolDomain,
    DeleteGroup,
    DeleteIdentityProvider,
    DeleteManagedLoginBranding,
    DeleteResourceServer,
    DeleteUser,
    DeleteUserAttributes,
    DeleteUserPool,
    DeleteUserPoolClient,
    DeleteUserPoolDomain,
    DescribeIdentityProvider,
    DescribeManagedLoginBranding,
    DescribeManagedLoginBrandingByClient,
    DescribeResourceServer,
    DescribeRiskConfiguration,
    DescribeUserImportJob,
    DescribeUserPool,
    DescribeUserPoolClient,
    DescribeUserPoolDomain,
    DisassociateWebAcl,
    ForgetDevice,
    ForgotPassword,
    GetCsvHeader,
    GetDevice,
    GetGroup,
    GetIdentityProviderByIdentifier,
    GetLogDeliveryConfiguration,
    GetSigningCertificate,
    GetTokensFromRefreshToken,
    GetUiCustomization,
    GetUser,
    GetUserAttributeVerificationCode,
    GetUserPoolMfaConfig,
    GetWebAclForResource,
    GlobalSignOut,
    InitiateAuth,
    ListDevices,
    ListGroups,
    ListIdentityProviders,
    ListResourceServers,
    ListResourcesForWebAcl,
    ListTagsForResource,
    ListUserImportJobs,
    ListUserPoolClients,
    ListUserPools,
    ListUsers,
    ListUsersInGroup,
    ResendConfirmationCode,
    RespondToAuthChallenge,
    RevokeToken,
    SetLogDeliveryConfiguration,
    SetRiskConfiguration,
    SetUiCustomization,
    SetUserMfaPreference,
    SetUserPoolMfaConfig,
    SetUserSettings,
    SignUp,
    StartUserImportJob,
    StopUserImportJob,
    TagResource,
    UntagResource,
    UpdateAuthEventFeedback,
    UpdateDeviceStatus,
    UpdateGroup,
    UpdateIdentityProvider,
    UpdateManagedLoginBranding,
    UpdateResourceServer,
    UpdateUserAttributes,
    UpdateUserPool,
    UpdateUserPoolClient,
    UpdateUserPoolDomain,
    VerifySoftwareToken,
    VerifyUserAttribute,
}
impl std::fmt::Display for CognitoIdpActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CognitoIdpActions::AddCustomAttributes => write!(f, "cognito-idp:AddCustomAttributes"),
            CognitoIdpActions::AdminAddUserToGroup => write!(f, "cognito-idp:AdminAddUserToGroup"),
            CognitoIdpActions::AdminConfirmSignUp => write!(f, "cognito-idp:AdminConfirmSignUp"),
            CognitoIdpActions::AdminCreateUser => write!(f, "cognito-idp:AdminCreateUser"),
            CognitoIdpActions::AdminDeleteUser => write!(f, "cognito-idp:AdminDeleteUser"),
            CognitoIdpActions::AdminDeleteUserAttributes => {
                write!(f, "cognito-idp:AdminDeleteUserAttributes")
            }
            CognitoIdpActions::AdminDisableProviderForUser => {
                write!(f, "cognito-idp:AdminDisableProviderForUser")
            }
            CognitoIdpActions::AdminDisableUser => write!(f, "cognito-idp:AdminDisableUser"),
            CognitoIdpActions::AdminEnableUser => write!(f, "cognito-idp:AdminEnableUser"),
            CognitoIdpActions::AdminForgetDevice => write!(f, "cognito-idp:AdminForgetDevice"),
            CognitoIdpActions::AdminGetDevice => write!(f, "cognito-idp:AdminGetDevice"),
            CognitoIdpActions::AdminGetUser => write!(f, "cognito-idp:AdminGetUser"),
            CognitoIdpActions::AdminInitiateAuth => write!(f, "cognito-idp:AdminInitiateAuth"),
            CognitoIdpActions::AdminLinkProviderForUser => {
                write!(f, "cognito-idp:AdminLinkProviderForUser")
            }
            CognitoIdpActions::AdminListDevices => write!(f, "cognito-idp:AdminListDevices"),
            CognitoIdpActions::AdminListGroupsForUser => {
                write!(f, "cognito-idp:AdminListGroupsForUser")
            }
            CognitoIdpActions::AdminListUserAuthEvents => {
                write!(f, "cognito-idp:AdminListUserAuthEvents")
            }
            CognitoIdpActions::AdminRemoveUserFromGroup => {
                write!(f, "cognito-idp:AdminRemoveUserFromGroup")
            }
            CognitoIdpActions::AdminResetUserPassword => {
                write!(f, "cognito-idp:AdminResetUserPassword")
            }
            CognitoIdpActions::AdminRespondToAuthChallenge => {
                write!(f, "cognito-idp:AdminRespondToAuthChallenge")
            }
            CognitoIdpActions::AdminSetUserMfaPreference => {
                write!(f, "cognito-idp:AdminSetUserMFAPreference")
            }
            CognitoIdpActions::AdminSetUserPassword => {
                write!(f, "cognito-idp:AdminSetUserPassword")
            }
            CognitoIdpActions::AdminSetUserSettings => {
                write!(f, "cognito-idp:AdminSetUserSettings")
            }
            CognitoIdpActions::AdminUpdateAuthEventFeedback => {
                write!(f, "cognito-idp:AdminUpdateAuthEventFeedback")
            }
            CognitoIdpActions::AdminUpdateDeviceStatus => {
                write!(f, "cognito-idp:AdminUpdateDeviceStatus")
            }
            CognitoIdpActions::AdminUpdateUserAttributes => {
                write!(f, "cognito-idp:AdminUpdateUserAttributes")
            }
            CognitoIdpActions::AdminUserGlobalSignOut => {
                write!(f, "cognito-idp:AdminUserGlobalSignOut")
            }
            CognitoIdpActions::AssociateSoftwareToken => {
                write!(f, "cognito-idp:AssociateSoftwareToken")
            }
            CognitoIdpActions::AssociateWebAcl => write!(f, "cognito-idp:AssociateWebACL"),
            CognitoIdpActions::ChangePassword => write!(f, "cognito-idp:ChangePassword"),
            CognitoIdpActions::ConfirmDevice => write!(f, "cognito-idp:ConfirmDevice"),
            CognitoIdpActions::ConfirmForgotPassword => {
                write!(f, "cognito-idp:ConfirmForgotPassword")
            }
            CognitoIdpActions::ConfirmSignUp => write!(f, "cognito-idp:ConfirmSignUp"),
            CognitoIdpActions::CreateGroup => write!(f, "cognito-idp:CreateGroup"),
            CognitoIdpActions::CreateIdentityProvider => {
                write!(f, "cognito-idp:CreateIdentityProvider")
            }
            CognitoIdpActions::CreateManagedLoginBranding => {
                write!(f, "cognito-idp:CreateManagedLoginBranding")
            }
            CognitoIdpActions::CreateResourceServer => {
                write!(f, "cognito-idp:CreateResourceServer")
            }
            CognitoIdpActions::CreateUserImportJob => write!(f, "cognito-idp:CreateUserImportJob"),
            CognitoIdpActions::CreateUserPool => write!(f, "cognito-idp:CreateUserPool"),
            CognitoIdpActions::CreateUserPoolClient => {
                write!(f, "cognito-idp:CreateUserPoolClient")
            }
            CognitoIdpActions::CreateUserPoolDomain => {
                write!(f, "cognito-idp:CreateUserPoolDomain")
            }
            CognitoIdpActions::DeleteGroup => write!(f, "cognito-idp:DeleteGroup"),
            CognitoIdpActions::DeleteIdentityProvider => {
                write!(f, "cognito-idp:DeleteIdentityProvider")
            }
            CognitoIdpActions::DeleteManagedLoginBranding => {
                write!(f, "cognito-idp:DeleteManagedLoginBranding")
            }
            CognitoIdpActions::DeleteResourceServer => {
                write!(f, "cognito-idp:DeleteResourceServer")
            }
            CognitoIdpActions::DeleteUser => write!(f, "cognito-idp:DeleteUser"),
            CognitoIdpActions::DeleteUserAttributes => {
                write!(f, "cognito-idp:DeleteUserAttributes")
            }
            CognitoIdpActions::DeleteUserPool => write!(f, "cognito-idp:DeleteUserPool"),
            CognitoIdpActions::DeleteUserPoolClient => {
                write!(f, "cognito-idp:DeleteUserPoolClient")
            }
            CognitoIdpActions::DeleteUserPoolDomain => {
                write!(f, "cognito-idp:DeleteUserPoolDomain")
            }
            CognitoIdpActions::DescribeIdentityProvider => {
                write!(f, "cognito-idp:DescribeIdentityProvider")
            }
            CognitoIdpActions::DescribeManagedLoginBranding => {
                write!(f, "cognito-idp:DescribeManagedLoginBranding")
            }
            CognitoIdpActions::DescribeManagedLoginBrandingByClient => {
                write!(f, "cognito-idp:DescribeManagedLoginBrandingByClient")
            }
            CognitoIdpActions::DescribeResourceServer => {
                write!(f, "cognito-idp:DescribeResourceServer")
            }
            CognitoIdpActions::DescribeRiskConfiguration => {
                write!(f, "cognito-idp:DescribeRiskConfiguration")
            }
            CognitoIdpActions::DescribeUserImportJob => {
                write!(f, "cognito-idp:DescribeUserImportJob")
            }
            CognitoIdpActions::DescribeUserPool => write!(f, "cognito-idp:DescribeUserPool"),
            CognitoIdpActions::DescribeUserPoolClient => {
                write!(f, "cognito-idp:DescribeUserPoolClient")
            }
            CognitoIdpActions::DescribeUserPoolDomain => {
                write!(f, "cognito-idp:DescribeUserPoolDomain")
            }
            CognitoIdpActions::DisassociateWebAcl => write!(f, "cognito-idp:DisassociateWebACL"),
            CognitoIdpActions::ForgetDevice => write!(f, "cognito-idp:ForgetDevice"),
            CognitoIdpActions::ForgotPassword => write!(f, "cognito-idp:ForgotPassword"),
            CognitoIdpActions::GetCsvHeader => write!(f, "cognito-idp:GetCSVHeader"),
            CognitoIdpActions::GetDevice => write!(f, "cognito-idp:GetDevice"),
            CognitoIdpActions::GetGroup => write!(f, "cognito-idp:GetGroup"),
            CognitoIdpActions::GetIdentityProviderByIdentifier => {
                write!(f, "cognito-idp:GetIdentityProviderByIdentifier")
            }
            CognitoIdpActions::GetLogDeliveryConfiguration => {
                write!(f, "cognito-idp:GetLogDeliveryConfiguration")
            }
            CognitoIdpActions::GetSigningCertificate => {
                write!(f, "cognito-idp:GetSigningCertificate")
            }
            CognitoIdpActions::GetTokensFromRefreshToken => {
                write!(f, "cognito-idp:GetTokensFromRefreshToken")
            }
            CognitoIdpActions::GetUiCustomization => write!(f, "cognito-idp:GetUICustomization"),
            CognitoIdpActions::GetUser => write!(f, "cognito-idp:GetUser"),
            CognitoIdpActions::GetUserAttributeVerificationCode => {
                write!(f, "cognito-idp:GetUserAttributeVerificationCode")
            }
            CognitoIdpActions::GetUserPoolMfaConfig => {
                write!(f, "cognito-idp:GetUserPoolMfaConfig")
            }
            CognitoIdpActions::GetWebAclForResource => {
                write!(f, "cognito-idp:GetWebACLForResource")
            }
            CognitoIdpActions::GlobalSignOut => write!(f, "cognito-idp:GlobalSignOut"),
            CognitoIdpActions::InitiateAuth => write!(f, "cognito-idp:InitiateAuth"),
            CognitoIdpActions::ListDevices => write!(f, "cognito-idp:ListDevices"),
            CognitoIdpActions::ListGroups => write!(f, "cognito-idp:ListGroups"),
            CognitoIdpActions::ListIdentityProviders => {
                write!(f, "cognito-idp:ListIdentityProviders")
            }
            CognitoIdpActions::ListResourceServers => write!(f, "cognito-idp:ListResourceServers"),
            CognitoIdpActions::ListResourcesForWebAcl => {
                write!(f, "cognito-idp:ListResourcesForWebACL")
            }
            CognitoIdpActions::ListTagsForResource => write!(f, "cognito-idp:ListTagsForResource"),
            CognitoIdpActions::ListUserImportJobs => write!(f, "cognito-idp:ListUserImportJobs"),
            CognitoIdpActions::ListUserPoolClients => write!(f, "cognito-idp:ListUserPoolClients"),
            CognitoIdpActions::ListUserPools => write!(f, "cognito-idp:ListUserPools"),
            CognitoIdpActions::ListUsers => write!(f, "cognito-idp:ListUsers"),
            CognitoIdpActions::ListUsersInGroup => write!(f, "cognito-idp:ListUsersInGroup"),
            CognitoIdpActions::ResendConfirmationCode => {
                write!(f, "cognito-idp:ResendConfirmationCode")
            }
            CognitoIdpActions::RespondToAuthChallenge => {
                write!(f, "cognito-idp:RespondToAuthChallenge")
            }
            CognitoIdpActions::RevokeToken => write!(f, "cognito-idp:RevokeToken"),
            CognitoIdpActions::SetLogDeliveryConfiguration => {
                write!(f, "cognito-idp:SetLogDeliveryConfiguration")
            }
            CognitoIdpActions::SetRiskConfiguration => {
                write!(f, "cognito-idp:SetRiskConfiguration")
            }
            CognitoIdpActions::SetUiCustomization => write!(f, "cognito-idp:SetUICustomization"),
            CognitoIdpActions::SetUserMfaPreference => {
                write!(f, "cognito-idp:SetUserMFAPreference")
            }
            CognitoIdpActions::SetUserPoolMfaConfig => {
                write!(f, "cognito-idp:SetUserPoolMfaConfig")
            }
            CognitoIdpActions::SetUserSettings => write!(f, "cognito-idp:SetUserSettings"),
            CognitoIdpActions::SignUp => write!(f, "cognito-idp:SignUp"),
            CognitoIdpActions::StartUserImportJob => write!(f, "cognito-idp:StartUserImportJob"),
            CognitoIdpActions::StopUserImportJob => write!(f, "cognito-idp:StopUserImportJob"),
            CognitoIdpActions::TagResource => write!(f, "cognito-idp:TagResource"),
            CognitoIdpActions::UntagResource => write!(f, "cognito-idp:UntagResource"),
            CognitoIdpActions::UpdateAuthEventFeedback => {
                write!(f, "cognito-idp:UpdateAuthEventFeedback")
            }
            CognitoIdpActions::UpdateDeviceStatus => write!(f, "cognito-idp:UpdateDeviceStatus"),
            CognitoIdpActions::UpdateGroup => write!(f, "cognito-idp:UpdateGroup"),
            CognitoIdpActions::UpdateIdentityProvider => {
                write!(f, "cognito-idp:UpdateIdentityProvider")
            }
            CognitoIdpActions::UpdateManagedLoginBranding => {
                write!(f, "cognito-idp:UpdateManagedLoginBranding")
            }
            CognitoIdpActions::UpdateResourceServer => {
                write!(f, "cognito-idp:UpdateResourceServer")
            }
            CognitoIdpActions::UpdateUserAttributes => {
                write!(f, "cognito-idp:UpdateUserAttributes")
            }
            CognitoIdpActions::UpdateUserPool => write!(f, "cognito-idp:UpdateUserPool"),
            CognitoIdpActions::UpdateUserPoolClient => {
                write!(f, "cognito-idp:UpdateUserPoolClient")
            }
            CognitoIdpActions::UpdateUserPoolDomain => {
                write!(f, "cognito-idp:UpdateUserPoolDomain")
            }
            CognitoIdpActions::VerifySoftwareToken => write!(f, "cognito-idp:VerifySoftwareToken"),
            CognitoIdpActions::VerifyUserAttribute => write!(f, "cognito-idp:VerifyUserAttribute"),
        }
    }
}
