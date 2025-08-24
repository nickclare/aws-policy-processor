// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum SchedulerActions {
    CreateSchedule,
    CreateScheduleGroup,
    DeleteSchedule,
    DeleteScheduleGroup,
    GetSchedule,
    GetScheduleGroup,
    ListScheduleGroups,
    ListSchedules,
    ListTagsForResource,
    TagResource,
    UntagResource,
    UpdateSchedule,
}
impl std::fmt::Display for SchedulerActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SchedulerActions::CreateSchedule => write!(f, "scheduler:CreateSchedule"),
            SchedulerActions::CreateScheduleGroup => write!(f, "scheduler:CreateScheduleGroup"),
            SchedulerActions::DeleteSchedule => write!(f, "scheduler:DeleteSchedule"),
            SchedulerActions::DeleteScheduleGroup => write!(f, "scheduler:DeleteScheduleGroup"),
            SchedulerActions::GetSchedule => write!(f, "scheduler:GetSchedule"),
            SchedulerActions::GetScheduleGroup => write!(f, "scheduler:GetScheduleGroup"),
            SchedulerActions::ListScheduleGroups => write!(f, "scheduler:ListScheduleGroups"),
            SchedulerActions::ListSchedules => write!(f, "scheduler:ListSchedules"),
            SchedulerActions::ListTagsForResource => write!(f, "scheduler:ListTagsForResource"),
            SchedulerActions::TagResource => write!(f, "scheduler:TagResource"),
            SchedulerActions::UntagResource => write!(f, "scheduler:UntagResource"),
            SchedulerActions::UpdateSchedule => write!(f, "scheduler:UpdateSchedule"),
        }
    }
}
