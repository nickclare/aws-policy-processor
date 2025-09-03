// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum AutoscalingPlansActions {
    CreateScalingPlan,
    DeleteScalingPlan,
    DescribeScalingPlanResources,
    DescribeScalingPlans,
    GetScalingPlanResourceForecastData,
    UpdateScalingPlan,
}
impl std::fmt::Display for AutoscalingPlansActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AutoscalingPlansActions::CreateScalingPlan => {
                write!(f, "autoscaling-plans:CreateScalingPlan")
            }
            AutoscalingPlansActions::DeleteScalingPlan => {
                write!(f, "autoscaling-plans:DeleteScalingPlan")
            }
            AutoscalingPlansActions::DescribeScalingPlanResources => {
                write!(f, "autoscaling-plans:DescribeScalingPlanResources")
            }
            AutoscalingPlansActions::DescribeScalingPlans => {
                write!(f, "autoscaling-plans:DescribeScalingPlans")
            }
            AutoscalingPlansActions::GetScalingPlanResourceForecastData => {
                write!(f, "autoscaling-plans:GetScalingPlanResourceForecastData")
            }
            AutoscalingPlansActions::UpdateScalingPlan => {
                write!(f, "autoscaling-plans:UpdateScalingPlan")
            }
        }
    }
}
