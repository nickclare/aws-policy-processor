// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum PiActions {
    CreatePerformanceAnalysisReport,
    DeletePerformanceAnalysisReport,
    DescribeDimensionKeys,
    GetDimensionKeyDetails,
    GetPerformanceAnalysisReport,
    GetResourceMetadata,
    GetResourceMetrics,
    ListAvailableResourceDimensions,
    ListAvailableResourceMetrics,
    ListPerformanceAnalysisReports,
    ListTagsForResource,
    TagResource,
    UntagResource,
}
impl std::fmt::Display for PiActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PiActions::CreatePerformanceAnalysisReport => {
                write!(f, "pi:CreatePerformanceAnalysisReport")
            }
            PiActions::DeletePerformanceAnalysisReport => {
                write!(f, "pi:DeletePerformanceAnalysisReport")
            }
            PiActions::DescribeDimensionKeys => write!(f, "pi:DescribeDimensionKeys"),
            PiActions::GetDimensionKeyDetails => write!(f, "pi:GetDimensionKeyDetails"),
            PiActions::GetPerformanceAnalysisReport => write!(f, "pi:GetPerformanceAnalysisReport"),
            PiActions::GetResourceMetadata => write!(f, "pi:GetResourceMetadata"),
            PiActions::GetResourceMetrics => write!(f, "pi:GetResourceMetrics"),
            PiActions::ListAvailableResourceDimensions => {
                write!(f, "pi:ListAvailableResourceDimensions")
            }
            PiActions::ListAvailableResourceMetrics => write!(f, "pi:ListAvailableResourceMetrics"),
            PiActions::ListPerformanceAnalysisReports => {
                write!(f, "pi:ListPerformanceAnalysisReports")
            }
            PiActions::ListTagsForResource => write!(f, "pi:ListTagsForResource"),
            PiActions::TagResource => write!(f, "pi:TagResource"),
            PiActions::UntagResource => write!(f, "pi:UntagResource"),
        }
    }
}
