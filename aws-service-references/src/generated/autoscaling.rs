// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum AutoscalingActions {
    AttachInstances,
    AttachLoadBalancerTargetGroups,
    AttachLoadBalancers,
    AttachTrafficSources,
    BatchDeleteScheduledAction,
    BatchPutScheduledUpdateGroupAction,
    CancelInstanceRefresh,
    CompleteLifecycleAction,
    CreateAutoScalingGroup,
    CreateLaunchConfiguration,
    CreateOrUpdateTags,
    DeleteAutoScalingGroup,
    DeleteLaunchConfiguration,
    DeleteLifecycleHook,
    DeleteNotificationConfiguration,
    DeletePolicy,
    DeleteScheduledAction,
    DeleteTags,
    DeleteWarmPool,
    DescribeAccountLimits,
    DescribeAdjustmentTypes,
    DescribeAutoScalingGroups,
    DescribeAutoScalingInstances,
    DescribeAutoScalingNotificationTypes,
    DescribeInstanceRefreshes,
    DescribeLaunchConfigurations,
    DescribeLifecycleHookTypes,
    DescribeLifecycleHooks,
    DescribeLoadBalancerTargetGroups,
    DescribeLoadBalancers,
    DescribeMetricCollectionTypes,
    DescribeNotificationConfigurations,
    DescribePolicies,
    DescribeScalingActivities,
    DescribeScalingProcessTypes,
    DescribeScheduledActions,
    DescribeTags,
    DescribeTerminationPolicyTypes,
    DescribeTrafficSources,
    DescribeWarmPool,
    DetachInstances,
    DetachLoadBalancerTargetGroups,
    DetachLoadBalancers,
    DetachTrafficSources,
    DisableMetricsCollection,
    EnableMetricsCollection,
    EnterStandby,
    ExecutePolicy,
    ExitStandby,
    GetPredictiveScalingForecast,
    PutLifecycleHook,
    PutNotificationConfiguration,
    PutScalingPolicy,
    PutScheduledUpdateGroupAction,
    PutWarmPool,
    RecordLifecycleActionHeartbeat,
    ResumeProcesses,
    RollbackInstanceRefresh,
    SetDesiredCapacity,
    SetInstanceHealth,
    SetInstanceProtection,
    StartInstanceRefresh,
    SuspendProcesses,
    TerminateInstanceInAutoScalingGroup,
    UpdateAutoScalingGroup,
}
impl std::fmt::Display for AutoscalingActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AutoscalingActions::AttachInstances => write!(f, "autoscaling:AttachInstances"),
            AutoscalingActions::AttachLoadBalancerTargetGroups => {
                write!(f, "autoscaling:AttachLoadBalancerTargetGroups")
            }
            AutoscalingActions::AttachLoadBalancers => write!(f, "autoscaling:AttachLoadBalancers"),
            AutoscalingActions::AttachTrafficSources => {
                write!(f, "autoscaling:AttachTrafficSources")
            }
            AutoscalingActions::BatchDeleteScheduledAction => {
                write!(f, "autoscaling:BatchDeleteScheduledAction")
            }
            AutoscalingActions::BatchPutScheduledUpdateGroupAction => {
                write!(f, "autoscaling:BatchPutScheduledUpdateGroupAction")
            }
            AutoscalingActions::CancelInstanceRefresh => {
                write!(f, "autoscaling:CancelInstanceRefresh")
            }
            AutoscalingActions::CompleteLifecycleAction => {
                write!(f, "autoscaling:CompleteLifecycleAction")
            }
            AutoscalingActions::CreateAutoScalingGroup => {
                write!(f, "autoscaling:CreateAutoScalingGroup")
            }
            AutoscalingActions::CreateLaunchConfiguration => {
                write!(f, "autoscaling:CreateLaunchConfiguration")
            }
            AutoscalingActions::CreateOrUpdateTags => write!(f, "autoscaling:CreateOrUpdateTags"),
            AutoscalingActions::DeleteAutoScalingGroup => {
                write!(f, "autoscaling:DeleteAutoScalingGroup")
            }
            AutoscalingActions::DeleteLaunchConfiguration => {
                write!(f, "autoscaling:DeleteLaunchConfiguration")
            }
            AutoscalingActions::DeleteLifecycleHook => write!(f, "autoscaling:DeleteLifecycleHook"),
            AutoscalingActions::DeleteNotificationConfiguration => {
                write!(f, "autoscaling:DeleteNotificationConfiguration")
            }
            AutoscalingActions::DeletePolicy => write!(f, "autoscaling:DeletePolicy"),
            AutoscalingActions::DeleteScheduledAction => {
                write!(f, "autoscaling:DeleteScheduledAction")
            }
            AutoscalingActions::DeleteTags => write!(f, "autoscaling:DeleteTags"),
            AutoscalingActions::DeleteWarmPool => write!(f, "autoscaling:DeleteWarmPool"),
            AutoscalingActions::DescribeAccountLimits => {
                write!(f, "autoscaling:DescribeAccountLimits")
            }
            AutoscalingActions::DescribeAdjustmentTypes => {
                write!(f, "autoscaling:DescribeAdjustmentTypes")
            }
            AutoscalingActions::DescribeAutoScalingGroups => {
                write!(f, "autoscaling:DescribeAutoScalingGroups")
            }
            AutoscalingActions::DescribeAutoScalingInstances => {
                write!(f, "autoscaling:DescribeAutoScalingInstances")
            }
            AutoscalingActions::DescribeAutoScalingNotificationTypes => {
                write!(f, "autoscaling:DescribeAutoScalingNotificationTypes")
            }
            AutoscalingActions::DescribeInstanceRefreshes => {
                write!(f, "autoscaling:DescribeInstanceRefreshes")
            }
            AutoscalingActions::DescribeLaunchConfigurations => {
                write!(f, "autoscaling:DescribeLaunchConfigurations")
            }
            AutoscalingActions::DescribeLifecycleHookTypes => {
                write!(f, "autoscaling:DescribeLifecycleHookTypes")
            }
            AutoscalingActions::DescribeLifecycleHooks => {
                write!(f, "autoscaling:DescribeLifecycleHooks")
            }
            AutoscalingActions::DescribeLoadBalancerTargetGroups => {
                write!(f, "autoscaling:DescribeLoadBalancerTargetGroups")
            }
            AutoscalingActions::DescribeLoadBalancers => {
                write!(f, "autoscaling:DescribeLoadBalancers")
            }
            AutoscalingActions::DescribeMetricCollectionTypes => {
                write!(f, "autoscaling:DescribeMetricCollectionTypes")
            }
            AutoscalingActions::DescribeNotificationConfigurations => {
                write!(f, "autoscaling:DescribeNotificationConfigurations")
            }
            AutoscalingActions::DescribePolicies => write!(f, "autoscaling:DescribePolicies"),
            AutoscalingActions::DescribeScalingActivities => {
                write!(f, "autoscaling:DescribeScalingActivities")
            }
            AutoscalingActions::DescribeScalingProcessTypes => {
                write!(f, "autoscaling:DescribeScalingProcessTypes")
            }
            AutoscalingActions::DescribeScheduledActions => {
                write!(f, "autoscaling:DescribeScheduledActions")
            }
            AutoscalingActions::DescribeTags => write!(f, "autoscaling:DescribeTags"),
            AutoscalingActions::DescribeTerminationPolicyTypes => {
                write!(f, "autoscaling:DescribeTerminationPolicyTypes")
            }
            AutoscalingActions::DescribeTrafficSources => {
                write!(f, "autoscaling:DescribeTrafficSources")
            }
            AutoscalingActions::DescribeWarmPool => write!(f, "autoscaling:DescribeWarmPool"),
            AutoscalingActions::DetachInstances => write!(f, "autoscaling:DetachInstances"),
            AutoscalingActions::DetachLoadBalancerTargetGroups => {
                write!(f, "autoscaling:DetachLoadBalancerTargetGroups")
            }
            AutoscalingActions::DetachLoadBalancers => write!(f, "autoscaling:DetachLoadBalancers"),
            AutoscalingActions::DetachTrafficSources => {
                write!(f, "autoscaling:DetachTrafficSources")
            }
            AutoscalingActions::DisableMetricsCollection => {
                write!(f, "autoscaling:DisableMetricsCollection")
            }
            AutoscalingActions::EnableMetricsCollection => {
                write!(f, "autoscaling:EnableMetricsCollection")
            }
            AutoscalingActions::EnterStandby => write!(f, "autoscaling:EnterStandby"),
            AutoscalingActions::ExecutePolicy => write!(f, "autoscaling:ExecutePolicy"),
            AutoscalingActions::ExitStandby => write!(f, "autoscaling:ExitStandby"),
            AutoscalingActions::GetPredictiveScalingForecast => {
                write!(f, "autoscaling:GetPredictiveScalingForecast")
            }
            AutoscalingActions::PutLifecycleHook => write!(f, "autoscaling:PutLifecycleHook"),
            AutoscalingActions::PutNotificationConfiguration => {
                write!(f, "autoscaling:PutNotificationConfiguration")
            }
            AutoscalingActions::PutScalingPolicy => write!(f, "autoscaling:PutScalingPolicy"),
            AutoscalingActions::PutScheduledUpdateGroupAction => {
                write!(f, "autoscaling:PutScheduledUpdateGroupAction")
            }
            AutoscalingActions::PutWarmPool => write!(f, "autoscaling:PutWarmPool"),
            AutoscalingActions::RecordLifecycleActionHeartbeat => {
                write!(f, "autoscaling:RecordLifecycleActionHeartbeat")
            }
            AutoscalingActions::ResumeProcesses => write!(f, "autoscaling:ResumeProcesses"),
            AutoscalingActions::RollbackInstanceRefresh => {
                write!(f, "autoscaling:RollbackInstanceRefresh")
            }
            AutoscalingActions::SetDesiredCapacity => write!(f, "autoscaling:SetDesiredCapacity"),
            AutoscalingActions::SetInstanceHealth => write!(f, "autoscaling:SetInstanceHealth"),
            AutoscalingActions::SetInstanceProtection => {
                write!(f, "autoscaling:SetInstanceProtection")
            }
            AutoscalingActions::StartInstanceRefresh => {
                write!(f, "autoscaling:StartInstanceRefresh")
            }
            AutoscalingActions::SuspendProcesses => write!(f, "autoscaling:SuspendProcesses"),
            AutoscalingActions::TerminateInstanceInAutoScalingGroup => {
                write!(f, "autoscaling:TerminateInstanceInAutoScalingGroup")
            }
            AutoscalingActions::UpdateAutoScalingGroup => {
                write!(f, "autoscaling:UpdateAutoScalingGroup")
            }
        }
    }
}
