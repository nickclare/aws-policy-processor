// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum FreetierActions {
    GetAccountActivity,
    GetAccountPlanState,
    GetFreeTierAlertPreference,
    GetFreeTierUsage,
    ListAccountActivities,
    PutFreeTierAlertPreference,
    UpgradeAccountPlan,
}
impl std::fmt::Display for FreetierActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FreetierActions::GetAccountActivity => write!(f, "freetier:GetAccountActivity"),
            FreetierActions::GetAccountPlanState => write!(f, "freetier:GetAccountPlanState"),
            FreetierActions::GetFreeTierAlertPreference => {
                write!(f, "freetier:GetFreeTierAlertPreference")
            }
            FreetierActions::GetFreeTierUsage => write!(f, "freetier:GetFreeTierUsage"),
            FreetierActions::ListAccountActivities => write!(f, "freetier:ListAccountActivities"),
            FreetierActions::PutFreeTierAlertPreference => {
                write!(f, "freetier:PutFreeTierAlertPreference")
            }
            FreetierActions::UpgradeAccountPlan => write!(f, "freetier:UpgradeAccountPlan"),
        }
    }
}
