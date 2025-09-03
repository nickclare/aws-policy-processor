// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum GeoMapsActions {
    GetStaticMap,
    GetTile,
}
impl std::fmt::Display for GeoMapsActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GeoMapsActions::GetStaticMap => write!(f, "geo-maps:GetStaticMap"),
            GeoMapsActions::GetTile => write!(f, "geo-maps:GetTile"),
        }
    }
}
