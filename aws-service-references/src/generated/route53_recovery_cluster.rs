// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum Route53RecoveryClusterActions {
    GetRoutingControlState,
    ListRoutingControls,
    UpdateRoutingControlState,
    UpdateRoutingControlStates,
}
impl std::fmt::Display for Route53RecoveryClusterActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Route53RecoveryClusterActions::GetRoutingControlState => {
                write!(f, "route53-recovery-cluster:GetRoutingControlState")
            }
            Route53RecoveryClusterActions::ListRoutingControls => {
                write!(f, "route53-recovery-cluster:ListRoutingControls")
            }
            Route53RecoveryClusterActions::UpdateRoutingControlState => {
                write!(f, "route53-recovery-cluster:UpdateRoutingControlState")
            }
            Route53RecoveryClusterActions::UpdateRoutingControlStates => {
                write!(f, "route53-recovery-cluster:UpdateRoutingControlStates")
            }
        }
    }
}
