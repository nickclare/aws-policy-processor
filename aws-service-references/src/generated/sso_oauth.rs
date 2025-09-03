// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum SsoOauthActions {
    CreateTokenWithIam,
}
impl std::fmt::Display for SsoOauthActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SsoOauthActions::CreateTokenWithIam => write!(f, "sso-oauth:CreateTokenWithIAM"),
        }
    }
}
