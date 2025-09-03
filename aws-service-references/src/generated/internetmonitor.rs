// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum InternetmonitorActions {
    CreateMonitor,
    DeleteMonitor,
    GetHealthEvent,
    GetInternetEvent,
    GetMonitor,
    GetQueryResults,
    GetQueryStatus,
    Link,
    ListHealthEvents,
    ListInternetEvents,
    ListMonitors,
    ListTagsForResource,
    StartQuery,
    StopQuery,
    TagResource,
    UntagResource,
    UpdateMonitor,
}
impl std::fmt::Display for InternetmonitorActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InternetmonitorActions::CreateMonitor => write!(f, "internetmonitor:CreateMonitor"),
            InternetmonitorActions::DeleteMonitor => write!(f, "internetmonitor:DeleteMonitor"),
            InternetmonitorActions::GetHealthEvent => write!(f, "internetmonitor:GetHealthEvent"),
            InternetmonitorActions::GetInternetEvent => {
                write!(f, "internetmonitor:GetInternetEvent")
            }
            InternetmonitorActions::GetMonitor => write!(f, "internetmonitor:GetMonitor"),
            InternetmonitorActions::GetQueryResults => write!(f, "internetmonitor:GetQueryResults"),
            InternetmonitorActions::GetQueryStatus => write!(f, "internetmonitor:GetQueryStatus"),
            InternetmonitorActions::Link => write!(f, "internetmonitor:Link"),
            InternetmonitorActions::ListHealthEvents => {
                write!(f, "internetmonitor:ListHealthEvents")
            }
            InternetmonitorActions::ListInternetEvents => {
                write!(f, "internetmonitor:ListInternetEvents")
            }
            InternetmonitorActions::ListMonitors => write!(f, "internetmonitor:ListMonitors"),
            InternetmonitorActions::ListTagsForResource => {
                write!(f, "internetmonitor:ListTagsForResource")
            }
            InternetmonitorActions::StartQuery => write!(f, "internetmonitor:StartQuery"),
            InternetmonitorActions::StopQuery => write!(f, "internetmonitor:StopQuery"),
            InternetmonitorActions::TagResource => write!(f, "internetmonitor:TagResource"),
            InternetmonitorActions::UntagResource => write!(f, "internetmonitor:UntagResource"),
            InternetmonitorActions::UpdateMonitor => write!(f, "internetmonitor:UpdateMonitor"),
        }
    }
}
