// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum BillingActions {
    CreateBillingView,
    DeleteBillingView,
    DeleteResourcePolicy,
    GetBillingData,
    GetBillingDetails,
    GetBillingNotifications,
    GetBillingPreferences,
    GetBillingView,
    GetBillingViewData,
    GetContractInformation,
    GetCredits,
    GetIamAccessPreference,
    GetResourcePolicy,
    GetSellerOfRecord,
    ListBillingViews,
    ListSourceViewsForBillingView,
    ListTagsForResource,
    PutContractInformation,
    PutResourcePolicy,
    RedeemCredits,
    TagResource,
    UntagResource,
    UpdateBillingPreferences,
    UpdateBillingView,
    UpdateIamAccessPreference,
}
impl std::fmt::Display for BillingActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BillingActions::CreateBillingView => write!(f, "billing:CreateBillingView"),
            BillingActions::DeleteBillingView => write!(f, "billing:DeleteBillingView"),
            BillingActions::DeleteResourcePolicy => write!(f, "billing:DeleteResourcePolicy"),
            BillingActions::GetBillingData => write!(f, "billing:GetBillingData"),
            BillingActions::GetBillingDetails => write!(f, "billing:GetBillingDetails"),
            BillingActions::GetBillingNotifications => write!(f, "billing:GetBillingNotifications"),
            BillingActions::GetBillingPreferences => write!(f, "billing:GetBillingPreferences"),
            BillingActions::GetBillingView => write!(f, "billing:GetBillingView"),
            BillingActions::GetBillingViewData => write!(f, "billing:GetBillingViewData"),
            BillingActions::GetContractInformation => write!(f, "billing:GetContractInformation"),
            BillingActions::GetCredits => write!(f, "billing:GetCredits"),
            BillingActions::GetIamAccessPreference => write!(f, "billing:GetIAMAccessPreference"),
            BillingActions::GetResourcePolicy => write!(f, "billing:GetResourcePolicy"),
            BillingActions::GetSellerOfRecord => write!(f, "billing:GetSellerOfRecord"),
            BillingActions::ListBillingViews => write!(f, "billing:ListBillingViews"),
            BillingActions::ListSourceViewsForBillingView => {
                write!(f, "billing:ListSourceViewsForBillingView")
            }
            BillingActions::ListTagsForResource => write!(f, "billing:ListTagsForResource"),
            BillingActions::PutContractInformation => write!(f, "billing:PutContractInformation"),
            BillingActions::PutResourcePolicy => write!(f, "billing:PutResourcePolicy"),
            BillingActions::RedeemCredits => write!(f, "billing:RedeemCredits"),
            BillingActions::TagResource => write!(f, "billing:TagResource"),
            BillingActions::UntagResource => write!(f, "billing:UntagResource"),
            BillingActions::UpdateBillingPreferences => {
                write!(f, "billing:UpdateBillingPreferences")
            }
            BillingActions::UpdateBillingView => write!(f, "billing:UpdateBillingView"),
            BillingActions::UpdateIamAccessPreference => {
                write!(f, "billing:UpdateIAMAccessPreference")
            }
        }
    }
}
