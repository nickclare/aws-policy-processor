// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum Route53RecoveryControlConfigActions {
    CreateCluster,
    CreateControlPanel,
    CreateRoutingControl,
    CreateSafetyRule,
    DeleteCluster,
    DeleteControlPanel,
    DeleteResourcePolicy,
    DeleteRoutingControl,
    DeleteSafetyRule,
    DescribeCluster,
    DescribeControlPanel,
    DescribeRoutingControl,
    DescribeRoutingControlByName,
    DescribeSafetyRule,
    GetResourcePolicy,
    ListAssociatedRoute53HealthChecks,
    ListClusters,
    ListControlPanels,
    ListRoutingControls,
    ListSafetyRules,
    ListTagsForResource,
    PutResourcePolicy,
    TagResource,
    UntagResource,
    UpdateCluster,
    UpdateControlPanel,
    UpdateRoutingControl,
    UpdateSafetyRule,
}
impl std::fmt::Display for Route53RecoveryControlConfigActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Route53RecoveryControlConfigActions::CreateCluster => {
                write!(f, "route53-recovery-control-config:CreateCluster")
            }
            Route53RecoveryControlConfigActions::CreateControlPanel => {
                write!(f, "route53-recovery-control-config:CreateControlPanel")
            }
            Route53RecoveryControlConfigActions::CreateRoutingControl => {
                write!(f, "route53-recovery-control-config:CreateRoutingControl")
            }
            Route53RecoveryControlConfigActions::CreateSafetyRule => {
                write!(f, "route53-recovery-control-config:CreateSafetyRule")
            }
            Route53RecoveryControlConfigActions::DeleteCluster => {
                write!(f, "route53-recovery-control-config:DeleteCluster")
            }
            Route53RecoveryControlConfigActions::DeleteControlPanel => {
                write!(f, "route53-recovery-control-config:DeleteControlPanel")
            }
            Route53RecoveryControlConfigActions::DeleteResourcePolicy => {
                write!(f, "route53-recovery-control-config:DeleteResourcePolicy")
            }
            Route53RecoveryControlConfigActions::DeleteRoutingControl => {
                write!(f, "route53-recovery-control-config:DeleteRoutingControl")
            }
            Route53RecoveryControlConfigActions::DeleteSafetyRule => {
                write!(f, "route53-recovery-control-config:DeleteSafetyRule")
            }
            Route53RecoveryControlConfigActions::DescribeCluster => {
                write!(f, "route53-recovery-control-config:DescribeCluster")
            }
            Route53RecoveryControlConfigActions::DescribeControlPanel => {
                write!(f, "route53-recovery-control-config:DescribeControlPanel")
            }
            Route53RecoveryControlConfigActions::DescribeRoutingControl => {
                write!(f, "route53-recovery-control-config:DescribeRoutingControl")
            }
            Route53RecoveryControlConfigActions::DescribeRoutingControlByName => write!(
                f,
                "route53-recovery-control-config:DescribeRoutingControlByName"
            ),
            Route53RecoveryControlConfigActions::DescribeSafetyRule => {
                write!(f, "route53-recovery-control-config:DescribeSafetyRule")
            }
            Route53RecoveryControlConfigActions::GetResourcePolicy => {
                write!(f, "route53-recovery-control-config:GetResourcePolicy")
            }
            Route53RecoveryControlConfigActions::ListAssociatedRoute53HealthChecks => write!(
                f,
                "route53-recovery-control-config:ListAssociatedRoute53HealthChecks"
            ),
            Route53RecoveryControlConfigActions::ListClusters => {
                write!(f, "route53-recovery-control-config:ListClusters")
            }
            Route53RecoveryControlConfigActions::ListControlPanels => {
                write!(f, "route53-recovery-control-config:ListControlPanels")
            }
            Route53RecoveryControlConfigActions::ListRoutingControls => {
                write!(f, "route53-recovery-control-config:ListRoutingControls")
            }
            Route53RecoveryControlConfigActions::ListSafetyRules => {
                write!(f, "route53-recovery-control-config:ListSafetyRules")
            }
            Route53RecoveryControlConfigActions::ListTagsForResource => {
                write!(f, "route53-recovery-control-config:ListTagsForResource")
            }
            Route53RecoveryControlConfigActions::PutResourcePolicy => {
                write!(f, "route53-recovery-control-config:PutResourcePolicy")
            }
            Route53RecoveryControlConfigActions::TagResource => {
                write!(f, "route53-recovery-control-config:TagResource")
            }
            Route53RecoveryControlConfigActions::UntagResource => {
                write!(f, "route53-recovery-control-config:UntagResource")
            }
            Route53RecoveryControlConfigActions::UpdateCluster => {
                write!(f, "route53-recovery-control-config:UpdateCluster")
            }
            Route53RecoveryControlConfigActions::UpdateControlPanel => {
                write!(f, "route53-recovery-control-config:UpdateControlPanel")
            }
            Route53RecoveryControlConfigActions::UpdateRoutingControl => {
                write!(f, "route53-recovery-control-config:UpdateRoutingControl")
            }
            Route53RecoveryControlConfigActions::UpdateSafetyRule => {
                write!(f, "route53-recovery-control-config:UpdateSafetyRule")
            }
        }
    }
}
