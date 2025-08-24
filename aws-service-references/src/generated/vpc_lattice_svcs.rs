// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum VpcLatticeSvcsActions {
    Connect,
    Invoke,
}
impl std::fmt::Display for VpcLatticeSvcsActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VpcLatticeSvcsActions::Connect => write!(f, "vpc-lattice-svcs:Connect"),
            VpcLatticeSvcsActions::Invoke => write!(f, "vpc-lattice-svcs:Invoke"),
        }
    }
}
