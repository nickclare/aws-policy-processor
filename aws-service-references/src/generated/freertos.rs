// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum FreertosActions {
    CreateSoftwareConfiguration,
    CreateSubscription,
    DeleteSoftwareConfiguration,
    DescribeHardwarePlatform,
    DescribeSoftwareConfiguration,
    DescribeSubscription,
    GetEmpPatchUrl,
    GetSoftwareUrl,
    GetSoftwareUrlForConfiguration,
    GetSubscriptionBillingAmount,
    ListFreeRtosVersions,
    ListHardwarePlatforms,
    ListHardwareVendors,
    ListSoftwareConfigurations,
    ListSoftwarePatches,
    ListSubscriptionEmails,
    ListSubscriptions,
    UpdateEmailRecipients,
    UpdateSoftwareConfiguration,
    VerifyEmail,
}
impl std::fmt::Display for FreertosActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FreertosActions::CreateSoftwareConfiguration => {
                write!(f, "freertos:CreateSoftwareConfiguration")
            }
            FreertosActions::CreateSubscription => write!(f, "freertos:CreateSubscription"),
            FreertosActions::DeleteSoftwareConfiguration => {
                write!(f, "freertos:DeleteSoftwareConfiguration")
            }
            FreertosActions::DescribeHardwarePlatform => {
                write!(f, "freertos:DescribeHardwarePlatform")
            }
            FreertosActions::DescribeSoftwareConfiguration => {
                write!(f, "freertos:DescribeSoftwareConfiguration")
            }
            FreertosActions::DescribeSubscription => write!(f, "freertos:DescribeSubscription"),
            FreertosActions::GetEmpPatchUrl => write!(f, "freertos:GetEmpPatchUrl"),
            FreertosActions::GetSoftwareUrl => write!(f, "freertos:GetSoftwareURL"),
            FreertosActions::GetSoftwareUrlForConfiguration => {
                write!(f, "freertos:GetSoftwareURLForConfiguration")
            }
            FreertosActions::GetSubscriptionBillingAmount => {
                write!(f, "freertos:GetSubscriptionBillingAmount")
            }
            FreertosActions::ListFreeRtosVersions => write!(f, "freertos:ListFreeRTOSVersions"),
            FreertosActions::ListHardwarePlatforms => write!(f, "freertos:ListHardwarePlatforms"),
            FreertosActions::ListHardwareVendors => write!(f, "freertos:ListHardwareVendors"),
            FreertosActions::ListSoftwareConfigurations => {
                write!(f, "freertos:ListSoftwareConfigurations")
            }
            FreertosActions::ListSoftwarePatches => write!(f, "freertos:ListSoftwarePatches"),
            FreertosActions::ListSubscriptionEmails => write!(f, "freertos:ListSubscriptionEmails"),
            FreertosActions::ListSubscriptions => write!(f, "freertos:ListSubscriptions"),
            FreertosActions::UpdateEmailRecipients => write!(f, "freertos:UpdateEmailRecipients"),
            FreertosActions::UpdateSoftwareConfiguration => {
                write!(f, "freertos:UpdateSoftwareConfiguration")
            }
            FreertosActions::VerifyEmail => write!(f, "freertos:VerifyEmail"),
        }
    }
}
