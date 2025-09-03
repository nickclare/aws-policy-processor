// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum CustomerVerificationActions {
    CreateCustomerVerificationDetails,
    CreateUploadUrls,
    GetCustomerVerificationDetails,
    GetCustomerVerificationEligibility,
    UpdateCustomerVerificationDetails,
}
impl std::fmt::Display for CustomerVerificationActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CustomerVerificationActions::CreateCustomerVerificationDetails => {
                write!(f, "customer-verification:CreateCustomerVerificationDetails")
            }
            CustomerVerificationActions::CreateUploadUrls => {
                write!(f, "customer-verification:CreateUploadUrls")
            }
            CustomerVerificationActions::GetCustomerVerificationDetails => {
                write!(f, "customer-verification:GetCustomerVerificationDetails")
            }
            CustomerVerificationActions::GetCustomerVerificationEligibility => write!(
                f,
                "customer-verification:GetCustomerVerificationEligibility"
            ),
            CustomerVerificationActions::UpdateCustomerVerificationDetails => {
                write!(f, "customer-verification:UpdateCustomerVerificationDetails")
            }
        }
    }
}
