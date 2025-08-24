// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum CloudwatchActions {
    BatchGetServiceLevelIndicatorReport,
    BatchGetServiceLevelObjectiveBudgetReport,
    CreateServiceLevelObjective,
    DeleteAlarms,
    DeleteAnomalyDetector,
    DeleteDashboards,
    DeleteInsightRules,
    DeleteMetricStream,
    DeleteServiceLevelObjective,
    DescribeAlarmHistory,
    DescribeAlarms,
    DescribeAlarmsForMetric,
    DescribeAnomalyDetectors,
    DescribeInsightRules,
    DisableAlarmActions,
    DisableInsightRules,
    EnableAlarmActions,
    EnableInsightRules,
    EnableTopologyDiscovery,
    GenerateQuery,
    GenerateQueryResultsSummary,
    GetDashboard,
    GetInsightRuleReport,
    GetMetricData,
    GetMetricStatistics,
    GetMetricStream,
    GetMetricWidgetImage,
    GetService,
    GetServiceData,
    GetServiceLevelObjective,
    GetTopologyDiscoveryStatus,
    GetTopologyMap,
    Link,
    ListDashboards,
    ListEntitiesForMetric,
    ListManagedInsightRules,
    ListMetricStreams,
    ListMetrics,
    ListServiceLevelObjectives,
    ListServices,
    ListTagsForResource,
    PutAnomalyDetector,
    PutCompositeAlarm,
    PutDashboard,
    PutInsightRule,
    PutManagedInsightRules,
    PutMetricAlarm,
    PutMetricData,
    PutMetricStream,
    SetAlarmState,
    StartMetricStreams,
    StopMetricStreams,
    TagResource,
    UntagResource,
    UpdateServiceLevelObjective,
}
impl std::fmt::Display for CloudwatchActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CloudwatchActions::BatchGetServiceLevelIndicatorReport => {
                write!(f, "cloudwatch:BatchGetServiceLevelIndicatorReport")
            }
            CloudwatchActions::BatchGetServiceLevelObjectiveBudgetReport => {
                write!(f, "cloudwatch:BatchGetServiceLevelObjectiveBudgetReport")
            }
            CloudwatchActions::CreateServiceLevelObjective => {
                write!(f, "cloudwatch:CreateServiceLevelObjective")
            }
            CloudwatchActions::DeleteAlarms => write!(f, "cloudwatch:DeleteAlarms"),
            CloudwatchActions::DeleteAnomalyDetector => {
                write!(f, "cloudwatch:DeleteAnomalyDetector")
            }
            CloudwatchActions::DeleteDashboards => write!(f, "cloudwatch:DeleteDashboards"),
            CloudwatchActions::DeleteInsightRules => write!(f, "cloudwatch:DeleteInsightRules"),
            CloudwatchActions::DeleteMetricStream => write!(f, "cloudwatch:DeleteMetricStream"),
            CloudwatchActions::DeleteServiceLevelObjective => {
                write!(f, "cloudwatch:DeleteServiceLevelObjective")
            }
            CloudwatchActions::DescribeAlarmHistory => write!(f, "cloudwatch:DescribeAlarmHistory"),
            CloudwatchActions::DescribeAlarms => write!(f, "cloudwatch:DescribeAlarms"),
            CloudwatchActions::DescribeAlarmsForMetric => {
                write!(f, "cloudwatch:DescribeAlarmsForMetric")
            }
            CloudwatchActions::DescribeAnomalyDetectors => {
                write!(f, "cloudwatch:DescribeAnomalyDetectors")
            }
            CloudwatchActions::DescribeInsightRules => write!(f, "cloudwatch:DescribeInsightRules"),
            CloudwatchActions::DisableAlarmActions => write!(f, "cloudwatch:DisableAlarmActions"),
            CloudwatchActions::DisableInsightRules => write!(f, "cloudwatch:DisableInsightRules"),
            CloudwatchActions::EnableAlarmActions => write!(f, "cloudwatch:EnableAlarmActions"),
            CloudwatchActions::EnableInsightRules => write!(f, "cloudwatch:EnableInsightRules"),
            CloudwatchActions::EnableTopologyDiscovery => {
                write!(f, "cloudwatch:EnableTopologyDiscovery")
            }
            CloudwatchActions::GenerateQuery => write!(f, "cloudwatch:GenerateQuery"),
            CloudwatchActions::GenerateQueryResultsSummary => {
                write!(f, "cloudwatch:GenerateQueryResultsSummary")
            }
            CloudwatchActions::GetDashboard => write!(f, "cloudwatch:GetDashboard"),
            CloudwatchActions::GetInsightRuleReport => write!(f, "cloudwatch:GetInsightRuleReport"),
            CloudwatchActions::GetMetricData => write!(f, "cloudwatch:GetMetricData"),
            CloudwatchActions::GetMetricStatistics => write!(f, "cloudwatch:GetMetricStatistics"),
            CloudwatchActions::GetMetricStream => write!(f, "cloudwatch:GetMetricStream"),
            CloudwatchActions::GetMetricWidgetImage => write!(f, "cloudwatch:GetMetricWidgetImage"),
            CloudwatchActions::GetService => write!(f, "cloudwatch:GetService"),
            CloudwatchActions::GetServiceData => write!(f, "cloudwatch:GetServiceData"),
            CloudwatchActions::GetServiceLevelObjective => {
                write!(f, "cloudwatch:GetServiceLevelObjective")
            }
            CloudwatchActions::GetTopologyDiscoveryStatus => {
                write!(f, "cloudwatch:GetTopologyDiscoveryStatus")
            }
            CloudwatchActions::GetTopologyMap => write!(f, "cloudwatch:GetTopologyMap"),
            CloudwatchActions::Link => write!(f, "cloudwatch:Link"),
            CloudwatchActions::ListDashboards => write!(f, "cloudwatch:ListDashboards"),
            CloudwatchActions::ListEntitiesForMetric => {
                write!(f, "cloudwatch:ListEntitiesForMetric")
            }
            CloudwatchActions::ListManagedInsightRules => {
                write!(f, "cloudwatch:ListManagedInsightRules")
            }
            CloudwatchActions::ListMetricStreams => write!(f, "cloudwatch:ListMetricStreams"),
            CloudwatchActions::ListMetrics => write!(f, "cloudwatch:ListMetrics"),
            CloudwatchActions::ListServiceLevelObjectives => {
                write!(f, "cloudwatch:ListServiceLevelObjectives")
            }
            CloudwatchActions::ListServices => write!(f, "cloudwatch:ListServices"),
            CloudwatchActions::ListTagsForResource => write!(f, "cloudwatch:ListTagsForResource"),
            CloudwatchActions::PutAnomalyDetector => write!(f, "cloudwatch:PutAnomalyDetector"),
            CloudwatchActions::PutCompositeAlarm => write!(f, "cloudwatch:PutCompositeAlarm"),
            CloudwatchActions::PutDashboard => write!(f, "cloudwatch:PutDashboard"),
            CloudwatchActions::PutInsightRule => write!(f, "cloudwatch:PutInsightRule"),
            CloudwatchActions::PutManagedInsightRules => {
                write!(f, "cloudwatch:PutManagedInsightRules")
            }
            CloudwatchActions::PutMetricAlarm => write!(f, "cloudwatch:PutMetricAlarm"),
            CloudwatchActions::PutMetricData => write!(f, "cloudwatch:PutMetricData"),
            CloudwatchActions::PutMetricStream => write!(f, "cloudwatch:PutMetricStream"),
            CloudwatchActions::SetAlarmState => write!(f, "cloudwatch:SetAlarmState"),
            CloudwatchActions::StartMetricStreams => write!(f, "cloudwatch:StartMetricStreams"),
            CloudwatchActions::StopMetricStreams => write!(f, "cloudwatch:StopMetricStreams"),
            CloudwatchActions::TagResource => write!(f, "cloudwatch:TagResource"),
            CloudwatchActions::UntagResource => write!(f, "cloudwatch:UntagResource"),
            CloudwatchActions::UpdateServiceLevelObjective => {
                write!(f, "cloudwatch:UpdateServiceLevelObjective")
            }
        }
    }
}
