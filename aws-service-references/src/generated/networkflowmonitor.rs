// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum NetworkflowmonitorActions {
    CreateMonitor,
    CreateScope,
    DeleteMonitor,
    DeleteScope,
    GetMonitor,
    GetQueryResultsMonitorTopContributors,
    GetQueryResultsWorkloadInsightsTopContributors,
    GetQueryResultsWorkloadInsightsTopContributorsData,
    GetQueryStatusMonitorTopContributors,
    GetQueryStatusWorkloadInsightsTopContributors,
    GetQueryStatusWorkloadInsightsTopContributorsData,
    GetScope,
    ListMonitors,
    ListScopes,
    ListTagsForResource,
    Publish,
    StartQueryMonitorTopContributors,
    StartQueryWorkloadInsightsTopContributors,
    StartQueryWorkloadInsightsTopContributorsData,
    StopQueryMonitorTopContributors,
    StopQueryWorkloadInsightsTopContributors,
    StopQueryWorkloadInsightsTopContributorsData,
    TagResource,
    UntagResource,
    UpdateMonitor,
    UpdateScope,
}
impl std::fmt::Display for NetworkflowmonitorActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NetworkflowmonitorActions::CreateMonitor => {
                write!(f, "networkflowmonitor:CreateMonitor")
            }
            NetworkflowmonitorActions::CreateScope => write!(f, "networkflowmonitor:CreateScope"),
            NetworkflowmonitorActions::DeleteMonitor => {
                write!(f, "networkflowmonitor:DeleteMonitor")
            }
            NetworkflowmonitorActions::DeleteScope => write!(f, "networkflowmonitor:DeleteScope"),
            NetworkflowmonitorActions::GetMonitor => write!(f, "networkflowmonitor:GetMonitor"),
            NetworkflowmonitorActions::GetQueryResultsMonitorTopContributors => write!(
                f,
                "networkflowmonitor:GetQueryResultsMonitorTopContributors"
            ),
            NetworkflowmonitorActions::GetQueryResultsWorkloadInsightsTopContributors => write!(
                f,
                "networkflowmonitor:GetQueryResultsWorkloadInsightsTopContributors"
            ),
            NetworkflowmonitorActions::GetQueryResultsWorkloadInsightsTopContributorsData => {
                write!(
                    f,
                    "networkflowmonitor:GetQueryResultsWorkloadInsightsTopContributorsData"
                )
            }
            NetworkflowmonitorActions::GetQueryStatusMonitorTopContributors => {
                write!(f, "networkflowmonitor:GetQueryStatusMonitorTopContributors")
            }
            NetworkflowmonitorActions::GetQueryStatusWorkloadInsightsTopContributors => write!(
                f,
                "networkflowmonitor:GetQueryStatusWorkloadInsightsTopContributors"
            ),
            NetworkflowmonitorActions::GetQueryStatusWorkloadInsightsTopContributorsData => write!(
                f,
                "networkflowmonitor:GetQueryStatusWorkloadInsightsTopContributorsData"
            ),
            NetworkflowmonitorActions::GetScope => write!(f, "networkflowmonitor:GetScope"),
            NetworkflowmonitorActions::ListMonitors => write!(f, "networkflowmonitor:ListMonitors"),
            NetworkflowmonitorActions::ListScopes => write!(f, "networkflowmonitor:ListScopes"),
            NetworkflowmonitorActions::ListTagsForResource => {
                write!(f, "networkflowmonitor:ListTagsForResource")
            }
            NetworkflowmonitorActions::Publish => write!(f, "networkflowmonitor:Publish"),
            NetworkflowmonitorActions::StartQueryMonitorTopContributors => {
                write!(f, "networkflowmonitor:StartQueryMonitorTopContributors")
            }
            NetworkflowmonitorActions::StartQueryWorkloadInsightsTopContributors => write!(
                f,
                "networkflowmonitor:StartQueryWorkloadInsightsTopContributors"
            ),
            NetworkflowmonitorActions::StartQueryWorkloadInsightsTopContributorsData => write!(
                f,
                "networkflowmonitor:StartQueryWorkloadInsightsTopContributorsData"
            ),
            NetworkflowmonitorActions::StopQueryMonitorTopContributors => {
                write!(f, "networkflowmonitor:StopQueryMonitorTopContributors")
            }
            NetworkflowmonitorActions::StopQueryWorkloadInsightsTopContributors => write!(
                f,
                "networkflowmonitor:StopQueryWorkloadInsightsTopContributors"
            ),
            NetworkflowmonitorActions::StopQueryWorkloadInsightsTopContributorsData => write!(
                f,
                "networkflowmonitor:StopQueryWorkloadInsightsTopContributorsData"
            ),
            NetworkflowmonitorActions::TagResource => write!(f, "networkflowmonitor:TagResource"),
            NetworkflowmonitorActions::UntagResource => {
                write!(f, "networkflowmonitor:UntagResource")
            }
            NetworkflowmonitorActions::UpdateMonitor => {
                write!(f, "networkflowmonitor:UpdateMonitor")
            }
            NetworkflowmonitorActions::UpdateScope => write!(f, "networkflowmonitor:UpdateScope"),
        }
    }
}
