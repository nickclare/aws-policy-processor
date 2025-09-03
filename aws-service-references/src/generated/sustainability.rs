// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum SustainabilityActions {
    GetCarbonFootprintSummary,
}
impl std::fmt::Display for SustainabilityActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SustainabilityActions::GetCarbonFootprintSummary => {
                write!(f, "sustainability:GetCarbonFootprintSummary")
            }
        }
    }
}
