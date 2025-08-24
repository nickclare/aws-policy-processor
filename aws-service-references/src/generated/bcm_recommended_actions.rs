// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum BcmRecommendedActionsActions {
    ListRecommendedActions,
}
impl std::fmt::Display for BcmRecommendedActionsActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BcmRecommendedActionsActions::ListRecommendedActions => {
                write!(f, "bcm-recommended-actions:ListRecommendedActions")
            }
        }
    }
}
