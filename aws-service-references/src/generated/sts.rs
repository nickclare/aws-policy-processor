// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum StsActions {
    AssumeRole,
    AssumeRoleWithSaml,
    AssumeRoleWithWebIdentity,
    AssumeRoot,
    DecodeAuthorizationMessage,
    GetAccessKeyInfo,
    GetCallerIdentity,
    GetFederationToken,
    GetServiceBearerToken,
    GetSessionToken,
    SetContext,
    SetSourceIdentity,
    TagSession,
}
impl std::fmt::Display for StsActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StsActions::AssumeRole => write!(f, "sts:AssumeRole"),
            StsActions::AssumeRoleWithSaml => write!(f, "sts:AssumeRoleWithSAML"),
            StsActions::AssumeRoleWithWebIdentity => write!(f, "sts:AssumeRoleWithWebIdentity"),
            StsActions::AssumeRoot => write!(f, "sts:AssumeRoot"),
            StsActions::DecodeAuthorizationMessage => write!(f, "sts:DecodeAuthorizationMessage"),
            StsActions::GetAccessKeyInfo => write!(f, "sts:GetAccessKeyInfo"),
            StsActions::GetCallerIdentity => write!(f, "sts:GetCallerIdentity"),
            StsActions::GetFederationToken => write!(f, "sts:GetFederationToken"),
            StsActions::GetServiceBearerToken => write!(f, "sts:GetServiceBearerToken"),
            StsActions::GetSessionToken => write!(f, "sts:GetSessionToken"),
            StsActions::SetContext => write!(f, "sts:SetContext"),
            StsActions::SetSourceIdentity => write!(f, "sts:SetSourceIdentity"),
            StsActions::TagSession => write!(f, "sts:TagSession"),
        }
    }
}
