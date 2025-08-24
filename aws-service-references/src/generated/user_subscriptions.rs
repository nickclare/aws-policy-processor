// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum UserSubscriptionsActions {
    CreateClaim,
    DeleteClaim,
    ListApplicationClaims,
    ListClaims,
    ListUserSubscriptions,
    UpdateClaim,
}
impl std::fmt::Display for UserSubscriptionsActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserSubscriptionsActions::CreateClaim => write!(f, "user-subscriptions:CreateClaim"),
            UserSubscriptionsActions::DeleteClaim => write!(f, "user-subscriptions:DeleteClaim"),
            UserSubscriptionsActions::ListApplicationClaims => {
                write!(f, "user-subscriptions:ListApplicationClaims")
            }
            UserSubscriptionsActions::ListClaims => write!(f, "user-subscriptions:ListClaims"),
            UserSubscriptionsActions::ListUserSubscriptions => {
                write!(f, "user-subscriptions:ListUserSubscriptions")
            }
            UserSubscriptionsActions::UpdateClaim => write!(f, "user-subscriptions:UpdateClaim"),
        }
    }
}
