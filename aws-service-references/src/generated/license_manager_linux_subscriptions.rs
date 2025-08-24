// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum LicenseManagerLinuxSubscriptionsActions {
    DeregisterSubscriptionProvider,
    GetRegisteredSubscriptionProvider,
    GetServiceSettings,
    ListLinuxSubscriptionInstances,
    ListLinuxSubscriptions,
    ListRegisteredSubscriptionProviders,
    ListTagsForResource,
    RegisterSubscriptionProvider,
    TagResource,
    UntagResource,
    UpdateServiceSettings,
}
impl std::fmt::Display for LicenseManagerLinuxSubscriptionsActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LicenseManagerLinuxSubscriptionsActions::DeregisterSubscriptionProvider => write!(
                f,
                "license-manager-linux-subscriptions:DeregisterSubscriptionProvider"
            ),
            LicenseManagerLinuxSubscriptionsActions::GetRegisteredSubscriptionProvider => write!(
                f,
                "license-manager-linux-subscriptions:GetRegisteredSubscriptionProvider"
            ),
            LicenseManagerLinuxSubscriptionsActions::GetServiceSettings => {
                write!(f, "license-manager-linux-subscriptions:GetServiceSettings")
            }
            LicenseManagerLinuxSubscriptionsActions::ListLinuxSubscriptionInstances => write!(
                f,
                "license-manager-linux-subscriptions:ListLinuxSubscriptionInstances"
            ),
            LicenseManagerLinuxSubscriptionsActions::ListLinuxSubscriptions => write!(
                f,
                "license-manager-linux-subscriptions:ListLinuxSubscriptions"
            ),
            LicenseManagerLinuxSubscriptionsActions::ListRegisteredSubscriptionProviders => write!(
                f,
                "license-manager-linux-subscriptions:ListRegisteredSubscriptionProviders"
            ),
            LicenseManagerLinuxSubscriptionsActions::ListTagsForResource => {
                write!(f, "license-manager-linux-subscriptions:ListTagsForResource")
            }
            LicenseManagerLinuxSubscriptionsActions::RegisterSubscriptionProvider => write!(
                f,
                "license-manager-linux-subscriptions:RegisterSubscriptionProvider"
            ),
            LicenseManagerLinuxSubscriptionsActions::TagResource => {
                write!(f, "license-manager-linux-subscriptions:TagResource")
            }
            LicenseManagerLinuxSubscriptionsActions::UntagResource => {
                write!(f, "license-manager-linux-subscriptions:UntagResource")
            }
            LicenseManagerLinuxSubscriptionsActions::UpdateServiceSettings => write!(
                f,
                "license-manager-linux-subscriptions:UpdateServiceSettings"
            ),
        }
    }
}
