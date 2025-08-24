// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum NotificationsContactsActions {
    ActivateEmailContact,
    CreateEmailContact,
    DeleteEmailContact,
    GetEmailContact,
    ListEmailContacts,
    ListTagsForResource,
    SendActivationCode,
    TagResource,
    UntagResource,
}
impl std::fmt::Display for NotificationsContactsActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NotificationsContactsActions::ActivateEmailContact => {
                write!(f, "notifications-contacts:ActivateEmailContact")
            }
            NotificationsContactsActions::CreateEmailContact => {
                write!(f, "notifications-contacts:CreateEmailContact")
            }
            NotificationsContactsActions::DeleteEmailContact => {
                write!(f, "notifications-contacts:DeleteEmailContact")
            }
            NotificationsContactsActions::GetEmailContact => {
                write!(f, "notifications-contacts:GetEmailContact")
            }
            NotificationsContactsActions::ListEmailContacts => {
                write!(f, "notifications-contacts:ListEmailContacts")
            }
            NotificationsContactsActions::ListTagsForResource => {
                write!(f, "notifications-contacts:ListTagsForResource")
            }
            NotificationsContactsActions::SendActivationCode => {
                write!(f, "notifications-contacts:SendActivationCode")
            }
            NotificationsContactsActions::TagResource => {
                write!(f, "notifications-contacts:TagResource")
            }
            NotificationsContactsActions::UntagResource => {
                write!(f, "notifications-contacts:UntagResource")
            }
        }
    }
}
