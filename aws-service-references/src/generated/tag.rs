// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum TagActions {
    DescribeReportCreation,
    GetComplianceSummary,
    GetResources,
    GetTagKeys,
    GetTagValues,
    StartReportCreation,
    TagResources,
    UntagResources,
}
impl std::fmt::Display for TagActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TagActions::DescribeReportCreation => write!(f, "tag:DescribeReportCreation"),
            TagActions::GetComplianceSummary => write!(f, "tag:GetComplianceSummary"),
            TagActions::GetResources => write!(f, "tag:GetResources"),
            TagActions::GetTagKeys => write!(f, "tag:GetTagKeys"),
            TagActions::GetTagValues => write!(f, "tag:GetTagValues"),
            TagActions::StartReportCreation => write!(f, "tag:StartReportCreation"),
            TagActions::TagResources => write!(f, "tag:TagResources"),
            TagActions::UntagResources => write!(f, "tag:UntagResources"),
        }
    }
}
