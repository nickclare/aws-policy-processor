// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum MapcreditsActions {
    ListAssociatedPrograms,
    ListQuarterCredits,
    ListQuarterSpend,
}
impl std::fmt::Display for MapcreditsActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MapcreditsActions::ListAssociatedPrograms => {
                write!(f, "mapcredits:ListAssociatedPrograms")
            }
            MapcreditsActions::ListQuarterCredits => write!(f, "mapcredits:ListQuarterCredits"),
            MapcreditsActions::ListQuarterSpend => write!(f, "mapcredits:ListQuarterSpend"),
        }
    }
}
