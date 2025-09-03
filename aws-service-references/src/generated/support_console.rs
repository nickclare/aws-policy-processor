// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum SupportConsoleActions {
    CheckSubscription,
    CreateCaseDraft,
    CreateContact,
    DeleteCaseDraft,
    DescribeDynamicHelp,
    GetAccountGovCloudEnabled,
    GetAccountState,
    GetBanner,
    GetCaseDraft,
    GetQuestionnaire,
    SaveFeedback,
}
impl std::fmt::Display for SupportConsoleActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SupportConsoleActions::CheckSubscription => {
                write!(f, "support-console:CheckSubscription")
            }
            SupportConsoleActions::CreateCaseDraft => write!(f, "support-console:CreateCaseDraft"),
            SupportConsoleActions::CreateContact => write!(f, "support-console:CreateContact"),
            SupportConsoleActions::DeleteCaseDraft => write!(f, "support-console:DeleteCaseDraft"),
            SupportConsoleActions::DescribeDynamicHelp => {
                write!(f, "support-console:DescribeDynamicHelp")
            }
            SupportConsoleActions::GetAccountGovCloudEnabled => {
                write!(f, "support-console:GetAccountGovCloudEnabled")
            }
            SupportConsoleActions::GetAccountState => write!(f, "support-console:GetAccountState"),
            SupportConsoleActions::GetBanner => write!(f, "support-console:GetBanner"),
            SupportConsoleActions::GetCaseDraft => write!(f, "support-console:GetCaseDraft"),
            SupportConsoleActions::GetQuestionnaire => {
                write!(f, "support-console:GetQuestionnaire")
            }
            SupportConsoleActions::SaveFeedback => write!(f, "support-console:SaveFeedback"),
        }
    }
}
