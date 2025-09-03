// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum RumActions {
    BatchCreateRumMetricDefinitions,
    BatchDeleteRumMetricDefinitions,
    BatchGetRumMetricDefinitions,
    CreateAppMonitor,
    DeleteAppMonitor,
    DeleteResourcePolicy,
    DeleteRumMetricsDestination,
    GetAppMonitor,
    GetAppMonitorData,
    GetResourcePolicy,
    ListAppMonitors,
    ListRumMetricsDestinations,
    ListTagsForResource,
    PutResourcePolicy,
    PutRumEvents,
    PutRumMetricsDestination,
    TagResource,
    UntagResource,
    UpdateAppMonitor,
    UpdateRumMetricDefinition,
}
impl std::fmt::Display for RumActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RumActions::BatchCreateRumMetricDefinitions => {
                write!(f, "rum:BatchCreateRumMetricDefinitions")
            }
            RumActions::BatchDeleteRumMetricDefinitions => {
                write!(f, "rum:BatchDeleteRumMetricDefinitions")
            }
            RumActions::BatchGetRumMetricDefinitions => {
                write!(f, "rum:BatchGetRumMetricDefinitions")
            }
            RumActions::CreateAppMonitor => write!(f, "rum:CreateAppMonitor"),
            RumActions::DeleteAppMonitor => write!(f, "rum:DeleteAppMonitor"),
            RumActions::DeleteResourcePolicy => write!(f, "rum:DeleteResourcePolicy"),
            RumActions::DeleteRumMetricsDestination => write!(f, "rum:DeleteRumMetricsDestination"),
            RumActions::GetAppMonitor => write!(f, "rum:GetAppMonitor"),
            RumActions::GetAppMonitorData => write!(f, "rum:GetAppMonitorData"),
            RumActions::GetResourcePolicy => write!(f, "rum:GetResourcePolicy"),
            RumActions::ListAppMonitors => write!(f, "rum:ListAppMonitors"),
            RumActions::ListRumMetricsDestinations => write!(f, "rum:ListRumMetricsDestinations"),
            RumActions::ListTagsForResource => write!(f, "rum:ListTagsForResource"),
            RumActions::PutResourcePolicy => write!(f, "rum:PutResourcePolicy"),
            RumActions::PutRumEvents => write!(f, "rum:PutRumEvents"),
            RumActions::PutRumMetricsDestination => write!(f, "rum:PutRumMetricsDestination"),
            RumActions::TagResource => write!(f, "rum:TagResource"),
            RumActions::UntagResource => write!(f, "rum:UntagResource"),
            RumActions::UpdateAppMonitor => write!(f, "rum:UpdateAppMonitor"),
            RumActions::UpdateRumMetricDefinition => write!(f, "rum:UpdateRumMetricDefinition"),
        }
    }
}
