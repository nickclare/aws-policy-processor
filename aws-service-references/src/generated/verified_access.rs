// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum VerifiedAccessActions {
    AllowVerifiedAccess,
}
impl std::fmt::Display for VerifiedAccessActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VerifiedAccessActions::AllowVerifiedAccess => {
                write!(f, "verified-access:AllowVerifiedAccess")
            }
        }
    }
}
