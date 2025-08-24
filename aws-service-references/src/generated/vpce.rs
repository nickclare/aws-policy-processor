// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum VpceActions {
    AllowMultiRegion,
}
impl std::fmt::Display for VpceActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VpceActions::AllowMultiRegion => write!(f, "vpce:AllowMultiRegion"),
        }
    }
}
