// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum SupportplansActions {
    CreateSupportPlanSchedule,
    GetSupportPlan,
    GetSupportPlanUpdateStatus,
    ListSupportPlanModifiers,
    StartSupportPlanUpdate,
}
impl std::fmt::Display for SupportplansActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SupportplansActions::CreateSupportPlanSchedule => {
                write!(f, "supportplans:CreateSupportPlanSchedule")
            }
            SupportplansActions::GetSupportPlan => write!(f, "supportplans:GetSupportPlan"),
            SupportplansActions::GetSupportPlanUpdateStatus => {
                write!(f, "supportplans:GetSupportPlanUpdateStatus")
            }
            SupportplansActions::ListSupportPlanModifiers => {
                write!(f, "supportplans:ListSupportPlanModifiers")
            }
            SupportplansActions::StartSupportPlanUpdate => {
                write!(f, "supportplans:StartSupportPlanUpdate")
            }
        }
    }
}
