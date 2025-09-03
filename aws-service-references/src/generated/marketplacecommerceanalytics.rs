// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum MarketplacecommerceanalyticsActions {
    GenerateDataSet,
    StartSupportDataExport,
}
impl std::fmt::Display for MarketplacecommerceanalyticsActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MarketplacecommerceanalyticsActions::GenerateDataSet => {
                write!(f, "marketplacecommerceanalytics:GenerateDataSet")
            }
            MarketplacecommerceanalyticsActions::StartSupportDataExport => {
                write!(f, "marketplacecommerceanalytics:StartSupportDataExport")
            }
        }
    }
}
