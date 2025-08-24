// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum NotificationsActions {
    AssociateChannel,
    AssociateManagedNotificationAccountContact,
    AssociateManagedNotificationAdditionalChannel,
    CreateEventRule,
    CreateNotificationConfiguration,
    DeleteEventRule,
    DeleteNotificationConfiguration,
    DeregisterNotificationHub,
    DisableNotificationsAccessForOrganization,
    DisassociateChannel,
    DisassociateManagedNotificationAccountContact,
    DisassociateManagedNotificationAdditionalChannel,
    EnableNotificationsAccessForOrganization,
    GetEventRule,
    GetFeatureOptInStatus,
    GetManagedNotificationChildEvent,
    GetManagedNotificationConfiguration,
    GetManagedNotificationEvent,
    GetNotificationConfiguration,
    GetNotificationEvent,
    GetNotificationsAccessForOrganization,
    ListChannels,
    ListEventRules,
    ListManagedNotificationChannelAssociations,
    ListManagedNotificationChildEvents,
    ListManagedNotificationConfigurations,
    ListManagedNotificationEvents,
    ListNotificationConfigurations,
    ListNotificationEvents,
    ListNotificationHubs,
    ListTagsForResource,
    PutFeatureOptInStatus,
    RegisterNotificationHub,
    TagResource,
    UntagResource,
    UpdateEventRule,
    UpdateNotificationConfiguration,
}
impl std::fmt::Display for NotificationsActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NotificationsActions::AssociateChannel => write!(f, "notifications:AssociateChannel"),
            NotificationsActions::AssociateManagedNotificationAccountContact => write!(
                f,
                "notifications:AssociateManagedNotificationAccountContact"
            ),
            NotificationsActions::AssociateManagedNotificationAdditionalChannel => write!(
                f,
                "notifications:AssociateManagedNotificationAdditionalChannel"
            ),
            NotificationsActions::CreateEventRule => write!(f, "notifications:CreateEventRule"),
            NotificationsActions::CreateNotificationConfiguration => {
                write!(f, "notifications:CreateNotificationConfiguration")
            }
            NotificationsActions::DeleteEventRule => write!(f, "notifications:DeleteEventRule"),
            NotificationsActions::DeleteNotificationConfiguration => {
                write!(f, "notifications:DeleteNotificationConfiguration")
            }
            NotificationsActions::DeregisterNotificationHub => {
                write!(f, "notifications:DeregisterNotificationHub")
            }
            NotificationsActions::DisableNotificationsAccessForOrganization => {
                write!(f, "notifications:DisableNotificationsAccessForOrganization")
            }
            NotificationsActions::DisassociateChannel => {
                write!(f, "notifications:DisassociateChannel")
            }
            NotificationsActions::DisassociateManagedNotificationAccountContact => write!(
                f,
                "notifications:DisassociateManagedNotificationAccountContact"
            ),
            NotificationsActions::DisassociateManagedNotificationAdditionalChannel => write!(
                f,
                "notifications:DisassociateManagedNotificationAdditionalChannel"
            ),
            NotificationsActions::EnableNotificationsAccessForOrganization => {
                write!(f, "notifications:EnableNotificationsAccessForOrganization")
            }
            NotificationsActions::GetEventRule => write!(f, "notifications:GetEventRule"),
            NotificationsActions::GetFeatureOptInStatus => {
                write!(f, "notifications:GetFeatureOptInStatus")
            }
            NotificationsActions::GetManagedNotificationChildEvent => {
                write!(f, "notifications:GetManagedNotificationChildEvent")
            }
            NotificationsActions::GetManagedNotificationConfiguration => {
                write!(f, "notifications:GetManagedNotificationConfiguration")
            }
            NotificationsActions::GetManagedNotificationEvent => {
                write!(f, "notifications:GetManagedNotificationEvent")
            }
            NotificationsActions::GetNotificationConfiguration => {
                write!(f, "notifications:GetNotificationConfiguration")
            }
            NotificationsActions::GetNotificationEvent => {
                write!(f, "notifications:GetNotificationEvent")
            }
            NotificationsActions::GetNotificationsAccessForOrganization => {
                write!(f, "notifications:GetNotificationsAccessForOrganization")
            }
            NotificationsActions::ListChannels => write!(f, "notifications:ListChannels"),
            NotificationsActions::ListEventRules => write!(f, "notifications:ListEventRules"),
            NotificationsActions::ListManagedNotificationChannelAssociations => write!(
                f,
                "notifications:ListManagedNotificationChannelAssociations"
            ),
            NotificationsActions::ListManagedNotificationChildEvents => {
                write!(f, "notifications:ListManagedNotificationChildEvents")
            }
            NotificationsActions::ListManagedNotificationConfigurations => {
                write!(f, "notifications:ListManagedNotificationConfigurations")
            }
            NotificationsActions::ListManagedNotificationEvents => {
                write!(f, "notifications:ListManagedNotificationEvents")
            }
            NotificationsActions::ListNotificationConfigurations => {
                write!(f, "notifications:ListNotificationConfigurations")
            }
            NotificationsActions::ListNotificationEvents => {
                write!(f, "notifications:ListNotificationEvents")
            }
            NotificationsActions::ListNotificationHubs => {
                write!(f, "notifications:ListNotificationHubs")
            }
            NotificationsActions::ListTagsForResource => {
                write!(f, "notifications:ListTagsForResource")
            }
            NotificationsActions::PutFeatureOptInStatus => {
                write!(f, "notifications:PutFeatureOptInStatus")
            }
            NotificationsActions::RegisterNotificationHub => {
                write!(f, "notifications:RegisterNotificationHub")
            }
            NotificationsActions::TagResource => write!(f, "notifications:TagResource"),
            NotificationsActions::UntagResource => write!(f, "notifications:UntagResource"),
            NotificationsActions::UpdateEventRule => write!(f, "notifications:UpdateEventRule"),
            NotificationsActions::UpdateNotificationConfiguration => {
                write!(f, "notifications:UpdateNotificationConfiguration")
            }
        }
    }
}
