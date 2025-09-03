// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum ApplicationAutoscalingActions {
    DeleteScalingPolicy,
    DeleteScheduledAction,
    DeregisterScalableTarget,
    DescribeScalableTargets,
    DescribeScalingActivities,
    DescribeScalingPolicies,
    DescribeScheduledActions,
    GetPredictiveScalingForecast,
    ListTagsForResource,
    PutScalingPolicy,
    PutScheduledAction,
    RegisterScalableTarget,
    TagResource,
    UntagResource,
}
impl std::fmt::Display for ApplicationAutoscalingActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApplicationAutoscalingActions::DeleteScalingPolicy => {
                write!(f, "application-autoscaling:DeleteScalingPolicy")
            }
            ApplicationAutoscalingActions::DeleteScheduledAction => {
                write!(f, "application-autoscaling:DeleteScheduledAction")
            }
            ApplicationAutoscalingActions::DeregisterScalableTarget => {
                write!(f, "application-autoscaling:DeregisterScalableTarget")
            }
            ApplicationAutoscalingActions::DescribeScalableTargets => {
                write!(f, "application-autoscaling:DescribeScalableTargets")
            }
            ApplicationAutoscalingActions::DescribeScalingActivities => {
                write!(f, "application-autoscaling:DescribeScalingActivities")
            }
            ApplicationAutoscalingActions::DescribeScalingPolicies => {
                write!(f, "application-autoscaling:DescribeScalingPolicies")
            }
            ApplicationAutoscalingActions::DescribeScheduledActions => {
                write!(f, "application-autoscaling:DescribeScheduledActions")
            }
            ApplicationAutoscalingActions::GetPredictiveScalingForecast => {
                write!(f, "application-autoscaling:GetPredictiveScalingForecast")
            }
            ApplicationAutoscalingActions::ListTagsForResource => {
                write!(f, "application-autoscaling:ListTagsForResource")
            }
            ApplicationAutoscalingActions::PutScalingPolicy => {
                write!(f, "application-autoscaling:PutScalingPolicy")
            }
            ApplicationAutoscalingActions::PutScheduledAction => {
                write!(f, "application-autoscaling:PutScheduledAction")
            }
            ApplicationAutoscalingActions::RegisterScalableTarget => {
                write!(f, "application-autoscaling:RegisterScalableTarget")
            }
            ApplicationAutoscalingActions::TagResource => {
                write!(f, "application-autoscaling:TagResource")
            }
            ApplicationAutoscalingActions::UntagResource => {
                write!(f, "application-autoscaling:UntagResource")
            }
        }
    }
}
