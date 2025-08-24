// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum GeoRoutesActions {
    CalculateIsolines,
    CalculateRouteMatrix,
    CalculateRoutes,
    OptimizeWaypoints,
    SnapToRoads,
}
impl std::fmt::Display for GeoRoutesActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GeoRoutesActions::CalculateIsolines => write!(f, "geo-routes:CalculateIsolines"),
            GeoRoutesActions::CalculateRouteMatrix => write!(f, "geo-routes:CalculateRouteMatrix"),
            GeoRoutesActions::CalculateRoutes => write!(f, "geo-routes:CalculateRoutes"),
            GeoRoutesActions::OptimizeWaypoints => write!(f, "geo-routes:OptimizeWaypoints"),
            GeoRoutesActions::SnapToRoads => write!(f, "geo-routes:SnapToRoads"),
        }
    }
}
