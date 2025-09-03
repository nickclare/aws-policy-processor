// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum XrayActions {
    BatchGetTraceSummaryById,
    BatchGetTraces,
    CancelTraceRetrieval,
    CreateGroup,
    CreateSamplingRule,
    DeleteGroup,
    DeleteResourcePolicy,
    DeleteSamplingRule,
    GetDistinctTraceGraphs,
    GetEncryptionConfig,
    GetGroup,
    GetGroups,
    GetIndexingRules,
    GetInsight,
    GetInsightEvents,
    GetInsightImpactGraph,
    GetInsightSummaries,
    GetRetrievedTracesGraph,
    GetSamplingRules,
    GetSamplingStatisticSummaries,
    GetSamplingTargets,
    GetServiceGraph,
    GetTimeSeriesServiceStatistics,
    GetTraceGraph,
    GetTraceSegmentDestination,
    GetTraceSummaries,
    Link,
    ListResourcePolicies,
    ListRetrievedTraces,
    ListTagsForResource,
    PutEncryptionConfig,
    PutResourcePolicy,
    PutSpans,
    PutSpansForIndexing,
    PutTelemetryRecords,
    PutTraceSegments,
    StartTraceRetrieval,
    TagResource,
    UntagResource,
    UpdateGroup,
    UpdateIndexingRule,
    UpdateSamplingRule,
    UpdateTraceSegmentDestination,
}
impl std::fmt::Display for XrayActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            XrayActions::BatchGetTraceSummaryById => write!(f, "xray:BatchGetTraceSummaryById"),
            XrayActions::BatchGetTraces => write!(f, "xray:BatchGetTraces"),
            XrayActions::CancelTraceRetrieval => write!(f, "xray:CancelTraceRetrieval"),
            XrayActions::CreateGroup => write!(f, "xray:CreateGroup"),
            XrayActions::CreateSamplingRule => write!(f, "xray:CreateSamplingRule"),
            XrayActions::DeleteGroup => write!(f, "xray:DeleteGroup"),
            XrayActions::DeleteResourcePolicy => write!(f, "xray:DeleteResourcePolicy"),
            XrayActions::DeleteSamplingRule => write!(f, "xray:DeleteSamplingRule"),
            XrayActions::GetDistinctTraceGraphs => write!(f, "xray:GetDistinctTraceGraphs"),
            XrayActions::GetEncryptionConfig => write!(f, "xray:GetEncryptionConfig"),
            XrayActions::GetGroup => write!(f, "xray:GetGroup"),
            XrayActions::GetGroups => write!(f, "xray:GetGroups"),
            XrayActions::GetIndexingRules => write!(f, "xray:GetIndexingRules"),
            XrayActions::GetInsight => write!(f, "xray:GetInsight"),
            XrayActions::GetInsightEvents => write!(f, "xray:GetInsightEvents"),
            XrayActions::GetInsightImpactGraph => write!(f, "xray:GetInsightImpactGraph"),
            XrayActions::GetInsightSummaries => write!(f, "xray:GetInsightSummaries"),
            XrayActions::GetRetrievedTracesGraph => write!(f, "xray:GetRetrievedTracesGraph"),
            XrayActions::GetSamplingRules => write!(f, "xray:GetSamplingRules"),
            XrayActions::GetSamplingStatisticSummaries => {
                write!(f, "xray:GetSamplingStatisticSummaries")
            }
            XrayActions::GetSamplingTargets => write!(f, "xray:GetSamplingTargets"),
            XrayActions::GetServiceGraph => write!(f, "xray:GetServiceGraph"),
            XrayActions::GetTimeSeriesServiceStatistics => {
                write!(f, "xray:GetTimeSeriesServiceStatistics")
            }
            XrayActions::GetTraceGraph => write!(f, "xray:GetTraceGraph"),
            XrayActions::GetTraceSegmentDestination => write!(f, "xray:GetTraceSegmentDestination"),
            XrayActions::GetTraceSummaries => write!(f, "xray:GetTraceSummaries"),
            XrayActions::Link => write!(f, "xray:Link"),
            XrayActions::ListResourcePolicies => write!(f, "xray:ListResourcePolicies"),
            XrayActions::ListRetrievedTraces => write!(f, "xray:ListRetrievedTraces"),
            XrayActions::ListTagsForResource => write!(f, "xray:ListTagsForResource"),
            XrayActions::PutEncryptionConfig => write!(f, "xray:PutEncryptionConfig"),
            XrayActions::PutResourcePolicy => write!(f, "xray:PutResourcePolicy"),
            XrayActions::PutSpans => write!(f, "xray:PutSpans"),
            XrayActions::PutSpansForIndexing => write!(f, "xray:PutSpansForIndexing"),
            XrayActions::PutTelemetryRecords => write!(f, "xray:PutTelemetryRecords"),
            XrayActions::PutTraceSegments => write!(f, "xray:PutTraceSegments"),
            XrayActions::StartTraceRetrieval => write!(f, "xray:StartTraceRetrieval"),
            XrayActions::TagResource => write!(f, "xray:TagResource"),
            XrayActions::UntagResource => write!(f, "xray:UntagResource"),
            XrayActions::UpdateGroup => write!(f, "xray:UpdateGroup"),
            XrayActions::UpdateIndexingRule => write!(f, "xray:UpdateIndexingRule"),
            XrayActions::UpdateSamplingRule => write!(f, "xray:UpdateSamplingRule"),
            XrayActions::UpdateTraceSegmentDestination => {
                write!(f, "xray:UpdateTraceSegmentDestination")
            }
        }
    }
}
