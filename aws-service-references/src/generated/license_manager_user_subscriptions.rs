// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum LicenseManagerUserSubscriptionsActions {
    AssociateUser,
    CreateLicenseServerEndpoint,
    DeleteLicenseServerEndpoint,
    DeregisterIdentityProvider,
    DisassociateUser,
    ListIdentityProviders,
    ListInstances,
    ListLicenseServerEndpoints,
    ListProductSubscriptions,
    ListTagsForResource,
    ListUserAssociations,
    RegisterIdentityProvider,
    StartProductSubscription,
    StopProductSubscription,
    TagResource,
    UntagResource,
    UpdateIdentityProviderSettings,
}
impl std::fmt::Display for LicenseManagerUserSubscriptionsActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LicenseManagerUserSubscriptionsActions::AssociateUser => {
                write!(f, "license-manager-user-subscriptions:AssociateUser")
            }
            LicenseManagerUserSubscriptionsActions::CreateLicenseServerEndpoint => write!(
                f,
                "license-manager-user-subscriptions:CreateLicenseServerEndpoint"
            ),
            LicenseManagerUserSubscriptionsActions::DeleteLicenseServerEndpoint => write!(
                f,
                "license-manager-user-subscriptions:DeleteLicenseServerEndpoint"
            ),
            LicenseManagerUserSubscriptionsActions::DeregisterIdentityProvider => write!(
                f,
                "license-manager-user-subscriptions:DeregisterIdentityProvider"
            ),
            LicenseManagerUserSubscriptionsActions::DisassociateUser => {
                write!(f, "license-manager-user-subscriptions:DisassociateUser")
            }
            LicenseManagerUserSubscriptionsActions::ListIdentityProviders => write!(
                f,
                "license-manager-user-subscriptions:ListIdentityProviders"
            ),
            LicenseManagerUserSubscriptionsActions::ListInstances => {
                write!(f, "license-manager-user-subscriptions:ListInstances")
            }
            LicenseManagerUserSubscriptionsActions::ListLicenseServerEndpoints => write!(
                f,
                "license-manager-user-subscriptions:ListLicenseServerEndpoints"
            ),
            LicenseManagerUserSubscriptionsActions::ListProductSubscriptions => write!(
                f,
                "license-manager-user-subscriptions:ListProductSubscriptions"
            ),
            LicenseManagerUserSubscriptionsActions::ListTagsForResource => {
                write!(f, "license-manager-user-subscriptions:ListTagsForResource")
            }
            LicenseManagerUserSubscriptionsActions::ListUserAssociations => {
                write!(f, "license-manager-user-subscriptions:ListUserAssociations")
            }
            LicenseManagerUserSubscriptionsActions::RegisterIdentityProvider => write!(
                f,
                "license-manager-user-subscriptions:RegisterIdentityProvider"
            ),
            LicenseManagerUserSubscriptionsActions::StartProductSubscription => write!(
                f,
                "license-manager-user-subscriptions:StartProductSubscription"
            ),
            LicenseManagerUserSubscriptionsActions::StopProductSubscription => write!(
                f,
                "license-manager-user-subscriptions:StopProductSubscription"
            ),
            LicenseManagerUserSubscriptionsActions::TagResource => {
                write!(f, "license-manager-user-subscriptions:TagResource")
            }
            LicenseManagerUserSubscriptionsActions::UntagResource => {
                write!(f, "license-manager-user-subscriptions:UntagResource")
            }
            LicenseManagerUserSubscriptionsActions::UpdateIdentityProviderSettings => write!(
                f,
                "license-manager-user-subscriptions:UpdateIdentityProviderSettings"
            ),
        }
    }
}
