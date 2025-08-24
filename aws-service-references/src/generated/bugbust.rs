// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum BugbustActions {
    CreateEvent,
    EvaluateProfilingGroups,
    GetEvent,
    GetJoinEventStatus,
    JoinEvent,
    ListBugs,
    ListEventParticipants,
    ListEventScores,
    ListEvents,
    ListProfilingGroups,
    ListPullRequests,
    ListTagsForResource,
    TagResource,
    UntagResource,
    UpdateEvent,
    UpdateWorkItem,
    UpdateWorkItemAdmin,
}
impl std::fmt::Display for BugbustActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BugbustActions::CreateEvent => write!(f, "bugbust:CreateEvent"),
            BugbustActions::EvaluateProfilingGroups => write!(f, "bugbust:EvaluateProfilingGroups"),
            BugbustActions::GetEvent => write!(f, "bugbust:GetEvent"),
            BugbustActions::GetJoinEventStatus => write!(f, "bugbust:GetJoinEventStatus"),
            BugbustActions::JoinEvent => write!(f, "bugbust:JoinEvent"),
            BugbustActions::ListBugs => write!(f, "bugbust:ListBugs"),
            BugbustActions::ListEventParticipants => write!(f, "bugbust:ListEventParticipants"),
            BugbustActions::ListEventScores => write!(f, "bugbust:ListEventScores"),
            BugbustActions::ListEvents => write!(f, "bugbust:ListEvents"),
            BugbustActions::ListProfilingGroups => write!(f, "bugbust:ListProfilingGroups"),
            BugbustActions::ListPullRequests => write!(f, "bugbust:ListPullRequests"),
            BugbustActions::ListTagsForResource => write!(f, "bugbust:ListTagsForResource"),
            BugbustActions::TagResource => write!(f, "bugbust:TagResource"),
            BugbustActions::UntagResource => write!(f, "bugbust:UntagResource"),
            BugbustActions::UpdateEvent => write!(f, "bugbust:UpdateEvent"),
            BugbustActions::UpdateWorkItem => write!(f, "bugbust:UpdateWorkItem"),
            BugbustActions::UpdateWorkItemAdmin => write!(f, "bugbust:UpdateWorkItemAdmin"),
        }
    }
}
