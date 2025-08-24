// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum LookoutmetricsActions {
    ActivateAnomalyDetector,
    BackTestAnomalyDetector,
    CreateAlert,
    CreateAnomalyDetector,
    CreateMetricSet,
    DeactivateAnomalyDetector,
    DeleteAlert,
    DeleteAnomalyDetector,
    DescribeAlert,
    DescribeAnomalyDetectionExecutions,
    DescribeAnomalyDetector,
    DescribeMetricSet,
    DetectMetricSetConfig,
    GetAnomalyGroup,
    GetDataQualityMetrics,
    GetFeedback,
    GetSampleData,
    ListAlerts,
    ListAnomalyDetectors,
    ListAnomalyGroupRelatedMetrics,
    ListAnomalyGroupSummaries,
    ListAnomalyGroupTimeSeries,
    ListMetricSets,
    ListTagsForResource,
    PutFeedback,
    TagResource,
    UntagResource,
    UpdateAlert,
    UpdateAnomalyDetector,
    UpdateMetricSet,
}
impl std::fmt::Display for LookoutmetricsActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LookoutmetricsActions::ActivateAnomalyDetector => {
                write!(f, "lookoutmetrics:ActivateAnomalyDetector")
            }
            LookoutmetricsActions::BackTestAnomalyDetector => {
                write!(f, "lookoutmetrics:BackTestAnomalyDetector")
            }
            LookoutmetricsActions::CreateAlert => write!(f, "lookoutmetrics:CreateAlert"),
            LookoutmetricsActions::CreateAnomalyDetector => {
                write!(f, "lookoutmetrics:CreateAnomalyDetector")
            }
            LookoutmetricsActions::CreateMetricSet => write!(f, "lookoutmetrics:CreateMetricSet"),
            LookoutmetricsActions::DeactivateAnomalyDetector => {
                write!(f, "lookoutmetrics:DeactivateAnomalyDetector")
            }
            LookoutmetricsActions::DeleteAlert => write!(f, "lookoutmetrics:DeleteAlert"),
            LookoutmetricsActions::DeleteAnomalyDetector => {
                write!(f, "lookoutmetrics:DeleteAnomalyDetector")
            }
            LookoutmetricsActions::DescribeAlert => write!(f, "lookoutmetrics:DescribeAlert"),
            LookoutmetricsActions::DescribeAnomalyDetectionExecutions => {
                write!(f, "lookoutmetrics:DescribeAnomalyDetectionExecutions")
            }
            LookoutmetricsActions::DescribeAnomalyDetector => {
                write!(f, "lookoutmetrics:DescribeAnomalyDetector")
            }
            LookoutmetricsActions::DescribeMetricSet => {
                write!(f, "lookoutmetrics:DescribeMetricSet")
            }
            LookoutmetricsActions::DetectMetricSetConfig => {
                write!(f, "lookoutmetrics:DetectMetricSetConfig")
            }
            LookoutmetricsActions::GetAnomalyGroup => write!(f, "lookoutmetrics:GetAnomalyGroup"),
            LookoutmetricsActions::GetDataQualityMetrics => {
                write!(f, "lookoutmetrics:GetDataQualityMetrics")
            }
            LookoutmetricsActions::GetFeedback => write!(f, "lookoutmetrics:GetFeedback"),
            LookoutmetricsActions::GetSampleData => write!(f, "lookoutmetrics:GetSampleData"),
            LookoutmetricsActions::ListAlerts => write!(f, "lookoutmetrics:ListAlerts"),
            LookoutmetricsActions::ListAnomalyDetectors => {
                write!(f, "lookoutmetrics:ListAnomalyDetectors")
            }
            LookoutmetricsActions::ListAnomalyGroupRelatedMetrics => {
                write!(f, "lookoutmetrics:ListAnomalyGroupRelatedMetrics")
            }
            LookoutmetricsActions::ListAnomalyGroupSummaries => {
                write!(f, "lookoutmetrics:ListAnomalyGroupSummaries")
            }
            LookoutmetricsActions::ListAnomalyGroupTimeSeries => {
                write!(f, "lookoutmetrics:ListAnomalyGroupTimeSeries")
            }
            LookoutmetricsActions::ListMetricSets => write!(f, "lookoutmetrics:ListMetricSets"),
            LookoutmetricsActions::ListTagsForResource => {
                write!(f, "lookoutmetrics:ListTagsForResource")
            }
            LookoutmetricsActions::PutFeedback => write!(f, "lookoutmetrics:PutFeedback"),
            LookoutmetricsActions::TagResource => write!(f, "lookoutmetrics:TagResource"),
            LookoutmetricsActions::UntagResource => write!(f, "lookoutmetrics:UntagResource"),
            LookoutmetricsActions::UpdateAlert => write!(f, "lookoutmetrics:UpdateAlert"),
            LookoutmetricsActions::UpdateAnomalyDetector => {
                write!(f, "lookoutmetrics:UpdateAnomalyDetector")
            }
            LookoutmetricsActions::UpdateMetricSet => write!(f, "lookoutmetrics:UpdateMetricSet"),
        }
    }
}
