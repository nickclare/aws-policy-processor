// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum ArcRegionSwitchActions {
    ApprovePlanExecutionStep,
    CancelPlanExecution,
    CreatePlan,
    DeletePlan,
    DeleteResourcePolicy,
    GetPlan,
    GetPlanEvaluationStatus,
    GetPlanExecution,
    GetPlanInRegion,
    GetResourcePolicy,
    ListPlanExecutionEvents,
    ListPlanExecutions,
    ListPlans,
    ListPlansInRegion,
    ListRoute53HealthChecks,
    ListTagsForResource,
    PutResourcePolicy,
    StartPlanExecution,
    TagResource,
    UntagResource,
    UpdatePlan,
    UpdatePlanExecution,
    UpdatePlanExecutionStep,
}
impl std::fmt::Display for ArcRegionSwitchActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArcRegionSwitchActions::ApprovePlanExecutionStep => {
                write!(f, "arc-region-switch:ApprovePlanExecutionStep")
            }
            ArcRegionSwitchActions::CancelPlanExecution => {
                write!(f, "arc-region-switch:CancelPlanExecution")
            }
            ArcRegionSwitchActions::CreatePlan => write!(f, "arc-region-switch:CreatePlan"),
            ArcRegionSwitchActions::DeletePlan => write!(f, "arc-region-switch:DeletePlan"),
            ArcRegionSwitchActions::DeleteResourcePolicy => {
                write!(f, "arc-region-switch:DeleteResourcePolicy")
            }
            ArcRegionSwitchActions::GetPlan => write!(f, "arc-region-switch:GetPlan"),
            ArcRegionSwitchActions::GetPlanEvaluationStatus => {
                write!(f, "arc-region-switch:GetPlanEvaluationStatus")
            }
            ArcRegionSwitchActions::GetPlanExecution => {
                write!(f, "arc-region-switch:GetPlanExecution")
            }
            ArcRegionSwitchActions::GetPlanInRegion => {
                write!(f, "arc-region-switch:GetPlanInRegion")
            }
            ArcRegionSwitchActions::GetResourcePolicy => {
                write!(f, "arc-region-switch:GetResourcePolicy")
            }
            ArcRegionSwitchActions::ListPlanExecutionEvents => {
                write!(f, "arc-region-switch:ListPlanExecutionEvents")
            }
            ArcRegionSwitchActions::ListPlanExecutions => {
                write!(f, "arc-region-switch:ListPlanExecutions")
            }
            ArcRegionSwitchActions::ListPlans => write!(f, "arc-region-switch:ListPlans"),
            ArcRegionSwitchActions::ListPlansInRegion => {
                write!(f, "arc-region-switch:ListPlansInRegion")
            }
            ArcRegionSwitchActions::ListRoute53HealthChecks => {
                write!(f, "arc-region-switch:ListRoute53HealthChecks")
            }
            ArcRegionSwitchActions::ListTagsForResource => {
                write!(f, "arc-region-switch:ListTagsForResource")
            }
            ArcRegionSwitchActions::PutResourcePolicy => {
                write!(f, "arc-region-switch:PutResourcePolicy")
            }
            ArcRegionSwitchActions::StartPlanExecution => {
                write!(f, "arc-region-switch:StartPlanExecution")
            }
            ArcRegionSwitchActions::TagResource => write!(f, "arc-region-switch:TagResource"),
            ArcRegionSwitchActions::UntagResource => write!(f, "arc-region-switch:UntagResource"),
            ArcRegionSwitchActions::UpdatePlan => write!(f, "arc-region-switch:UpdatePlan"),
            ArcRegionSwitchActions::UpdatePlanExecution => {
                write!(f, "arc-region-switch:UpdatePlanExecution")
            }
            ArcRegionSwitchActions::UpdatePlanExecutionStep => {
                write!(f, "arc-region-switch:UpdatePlanExecutionStep")
            }
        }
    }
}
