// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum EksAuthActions {
    AssumeRoleForPodIdentity,
}
impl std::fmt::Display for EksAuthActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EksAuthActions::AssumeRoleForPodIdentity => {
                write!(f, "eks-auth:AssumeRoleForPodIdentity")
            }
        }
    }
}
