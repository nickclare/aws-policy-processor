// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum ApplicationCostProfilerActions {
    DeleteReportDefinition,
    GetReportDefinition,
    ImportApplicationUsage,
    ListReportDefinitions,
    PutReportDefinition,
    UpdateReportDefinition,
}
impl std::fmt::Display for ApplicationCostProfilerActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApplicationCostProfilerActions::DeleteReportDefinition => {
                write!(f, "application-cost-profiler:DeleteReportDefinition")
            }
            ApplicationCostProfilerActions::GetReportDefinition => {
                write!(f, "application-cost-profiler:GetReportDefinition")
            }
            ApplicationCostProfilerActions::ImportApplicationUsage => {
                write!(f, "application-cost-profiler:ImportApplicationUsage")
            }
            ApplicationCostProfilerActions::ListReportDefinitions => {
                write!(f, "application-cost-profiler:ListReportDefinitions")
            }
            ApplicationCostProfilerActions::PutReportDefinition => {
                write!(f, "application-cost-profiler:PutReportDefinition")
            }
            ApplicationCostProfilerActions::UpdateReportDefinition => {
                write!(f, "application-cost-profiler:UpdateReportDefinition")
            }
        }
    }
}
