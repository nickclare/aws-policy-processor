// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum SigninActions {
    CreateTrustedIdentityPropagationApplicationForConsole,
    ListTrustedIdentityPropagationApplicationsForConsole,
}
impl std::fmt::Display for SigninActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SigninActions::CreateTrustedIdentityPropagationApplicationForConsole => write!(
                f,
                "signin:CreateTrustedIdentityPropagationApplicationForConsole"
            ),
            SigninActions::ListTrustedIdentityPropagationApplicationsForConsole => write!(
                f,
                "signin:ListTrustedIdentityPropagationApplicationsForConsole"
            ),
        }
    }
}
