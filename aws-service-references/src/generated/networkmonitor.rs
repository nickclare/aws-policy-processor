// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum NetworkmonitorActions {
    CreateMonitor,
    CreateProbe,
    DeleteMonitor,
    DeleteProbe,
    GetMonitor,
    GetProbe,
    ListMonitors,
    ListTagsForResource,
    TagResource,
    UntagResource,
    UpdateMonitor,
    UpdateProbe,
}
impl std::fmt::Display for NetworkmonitorActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NetworkmonitorActions::CreateMonitor => write!(f, "networkmonitor:CreateMonitor"),
            NetworkmonitorActions::CreateProbe => write!(f, "networkmonitor:CreateProbe"),
            NetworkmonitorActions::DeleteMonitor => write!(f, "networkmonitor:DeleteMonitor"),
            NetworkmonitorActions::DeleteProbe => write!(f, "networkmonitor:DeleteProbe"),
            NetworkmonitorActions::GetMonitor => write!(f, "networkmonitor:GetMonitor"),
            NetworkmonitorActions::GetProbe => write!(f, "networkmonitor:GetProbe"),
            NetworkmonitorActions::ListMonitors => write!(f, "networkmonitor:ListMonitors"),
            NetworkmonitorActions::ListTagsForResource => {
                write!(f, "networkmonitor:ListTagsForResource")
            }
            NetworkmonitorActions::TagResource => write!(f, "networkmonitor:TagResource"),
            NetworkmonitorActions::UntagResource => write!(f, "networkmonitor:UntagResource"),
            NetworkmonitorActions::UpdateMonitor => write!(f, "networkmonitor:UpdateMonitor"),
            NetworkmonitorActions::UpdateProbe => write!(f, "networkmonitor:UpdateProbe"),
        }
    }
}
