// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum ArsenalActions {
    RegisterOnPremisesAgent,
}
impl std::fmt::Display for ArsenalActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArsenalActions::RegisterOnPremisesAgent => write!(f, "arsenal:RegisterOnPremisesAgent"),
        }
    }
}
