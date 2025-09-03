// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum CognitoIdentityActions {
    CreateIdentityPool,
    DeleteIdentities,
    DeleteIdentityPool,
    DescribeIdentity,
    DescribeIdentityPool,
    GetCredentialsForIdentity,
    GetId,
    GetIdentityPoolAnalytics,
    GetIdentityPoolDailyAnalytics,
    GetIdentityPoolRoles,
    GetIdentityProviderDailyAnalytics,
    GetOpenIdToken,
    GetOpenIdTokenForDeveloperIdentity,
    GetPrincipalTagAttributeMap,
    ListIdentities,
    ListIdentityPools,
    ListTagsForResource,
    LookupDeveloperIdentity,
    MergeDeveloperIdentities,
    SetIdentityPoolRoles,
    SetPrincipalTagAttributeMap,
    TagResource,
    UnlinkDeveloperIdentity,
    UnlinkIdentity,
    UntagResource,
    UpdateIdentityPool,
}
impl std::fmt::Display for CognitoIdentityActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CognitoIdentityActions::CreateIdentityPool => {
                write!(f, "cognito-identity:CreateIdentityPool")
            }
            CognitoIdentityActions::DeleteIdentities => {
                write!(f, "cognito-identity:DeleteIdentities")
            }
            CognitoIdentityActions::DeleteIdentityPool => {
                write!(f, "cognito-identity:DeleteIdentityPool")
            }
            CognitoIdentityActions::DescribeIdentity => {
                write!(f, "cognito-identity:DescribeIdentity")
            }
            CognitoIdentityActions::DescribeIdentityPool => {
                write!(f, "cognito-identity:DescribeIdentityPool")
            }
            CognitoIdentityActions::GetCredentialsForIdentity => {
                write!(f, "cognito-identity:GetCredentialsForIdentity")
            }
            CognitoIdentityActions::GetId => write!(f, "cognito-identity:GetId"),
            CognitoIdentityActions::GetIdentityPoolAnalytics => {
                write!(f, "cognito-identity:GetIdentityPoolAnalytics")
            }
            CognitoIdentityActions::GetIdentityPoolDailyAnalytics => {
                write!(f, "cognito-identity:GetIdentityPoolDailyAnalytics")
            }
            CognitoIdentityActions::GetIdentityPoolRoles => {
                write!(f, "cognito-identity:GetIdentityPoolRoles")
            }
            CognitoIdentityActions::GetIdentityProviderDailyAnalytics => {
                write!(f, "cognito-identity:GetIdentityProviderDailyAnalytics")
            }
            CognitoIdentityActions::GetOpenIdToken => write!(f, "cognito-identity:GetOpenIdToken"),
            CognitoIdentityActions::GetOpenIdTokenForDeveloperIdentity => {
                write!(f, "cognito-identity:GetOpenIdTokenForDeveloperIdentity")
            }
            CognitoIdentityActions::GetPrincipalTagAttributeMap => {
                write!(f, "cognito-identity:GetPrincipalTagAttributeMap")
            }
            CognitoIdentityActions::ListIdentities => write!(f, "cognito-identity:ListIdentities"),
            CognitoIdentityActions::ListIdentityPools => {
                write!(f, "cognito-identity:ListIdentityPools")
            }
            CognitoIdentityActions::ListTagsForResource => {
                write!(f, "cognito-identity:ListTagsForResource")
            }
            CognitoIdentityActions::LookupDeveloperIdentity => {
                write!(f, "cognito-identity:LookupDeveloperIdentity")
            }
            CognitoIdentityActions::MergeDeveloperIdentities => {
                write!(f, "cognito-identity:MergeDeveloperIdentities")
            }
            CognitoIdentityActions::SetIdentityPoolRoles => {
                write!(f, "cognito-identity:SetIdentityPoolRoles")
            }
            CognitoIdentityActions::SetPrincipalTagAttributeMap => {
                write!(f, "cognito-identity:SetPrincipalTagAttributeMap")
            }
            CognitoIdentityActions::TagResource => write!(f, "cognito-identity:TagResource"),
            CognitoIdentityActions::UnlinkDeveloperIdentity => {
                write!(f, "cognito-identity:UnlinkDeveloperIdentity")
            }
            CognitoIdentityActions::UnlinkIdentity => write!(f, "cognito-identity:UnlinkIdentity"),
            CognitoIdentityActions::UntagResource => write!(f, "cognito-identity:UntagResource"),
            CognitoIdentityActions::UpdateIdentityPool => {
                write!(f, "cognito-identity:UpdateIdentityPool")
            }
        }
    }
}
