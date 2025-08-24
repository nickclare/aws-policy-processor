// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum CurActions {
    DeleteReportDefinition,
    DescribeReportDefinitions,
    GetClassicReport,
    GetClassicReportPreferences,
    GetUsageReport,
    ListTagsForResource,
    ModifyReportDefinition,
    PutClassicReportPreferences,
    PutReportDefinition,
    TagResource,
    UntagResource,
    ValidateReportDestination,
}
impl std::fmt::Display for CurActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CurActions::DeleteReportDefinition => write!(f, "cur:DeleteReportDefinition"),
            CurActions::DescribeReportDefinitions => write!(f, "cur:DescribeReportDefinitions"),
            CurActions::GetClassicReport => write!(f, "cur:GetClassicReport"),
            CurActions::GetClassicReportPreferences => write!(f, "cur:GetClassicReportPreferences"),
            CurActions::GetUsageReport => write!(f, "cur:GetUsageReport"),
            CurActions::ListTagsForResource => write!(f, "cur:ListTagsForResource"),
            CurActions::ModifyReportDefinition => write!(f, "cur:ModifyReportDefinition"),
            CurActions::PutClassicReportPreferences => write!(f, "cur:PutClassicReportPreferences"),
            CurActions::PutReportDefinition => write!(f, "cur:PutReportDefinition"),
            CurActions::TagResource => write!(f, "cur:TagResource"),
            CurActions::UntagResource => write!(f, "cur:UntagResource"),
            CurActions::ValidateReportDestination => write!(f, "cur:ValidateReportDestination"),
        }
    }
}
