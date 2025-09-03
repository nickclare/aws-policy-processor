// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum GeoPlacesActions {
    Autocomplete,
    Geocode,
    GetPlace,
    ReverseGeocode,
    SearchNearby,
    SearchText,
    Suggest,
}
impl std::fmt::Display for GeoPlacesActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GeoPlacesActions::Autocomplete => write!(f, "geo-places:Autocomplete"),
            GeoPlacesActions::Geocode => write!(f, "geo-places:Geocode"),
            GeoPlacesActions::GetPlace => write!(f, "geo-places:GetPlace"),
            GeoPlacesActions::ReverseGeocode => write!(f, "geo-places:ReverseGeocode"),
            GeoPlacesActions::SearchNearby => write!(f, "geo-places:SearchNearby"),
            GeoPlacesActions::SearchText => write!(f, "geo-places:SearchText"),
            GeoPlacesActions::Suggest => write!(f, "geo-places:Suggest"),
        }
    }
}
