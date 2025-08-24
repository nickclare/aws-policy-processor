// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum MobileanalyticsActions {
    GetFinancialReports,
    GetReports,
    PutEvents,
}
impl std::fmt::Display for MobileanalyticsActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MobileanalyticsActions::GetFinancialReports => {
                write!(f, "mobileanalytics:GetFinancialReports")
            }
            MobileanalyticsActions::GetReports => write!(f, "mobileanalytics:GetReports"),
            MobileanalyticsActions::PutEvents => write!(f, "mobileanalytics:PutEvents"),
        }
    }
}
