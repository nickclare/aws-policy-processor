// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum CodestarNotificationsActions {
    CreateNotificationRule,
    DeleteNotificationRule,
    DeleteTarget,
    DescribeNotificationRule,
    ListEventTypes,
    ListNotificationRules,
    ListTagsForResource,
    ListTargets,
    Subscribe,
    TagResource,
    Unsubscribe,
    UntagResource,
    UpdateNotificationRule,
}
impl std::fmt::Display for CodestarNotificationsActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CodestarNotificationsActions::CreateNotificationRule => {
                write!(f, "codestar-notifications:CreateNotificationRule")
            }
            CodestarNotificationsActions::DeleteNotificationRule => {
                write!(f, "codestar-notifications:DeleteNotificationRule")
            }
            CodestarNotificationsActions::DeleteTarget => {
                write!(f, "codestar-notifications:DeleteTarget")
            }
            CodestarNotificationsActions::DescribeNotificationRule => {
                write!(f, "codestar-notifications:DescribeNotificationRule")
            }
            CodestarNotificationsActions::ListEventTypes => {
                write!(f, "codestar-notifications:ListEventTypes")
            }
            CodestarNotificationsActions::ListNotificationRules => {
                write!(f, "codestar-notifications:ListNotificationRules")
            }
            CodestarNotificationsActions::ListTagsForResource => {
                write!(f, "codestar-notifications:ListTagsForResource")
            }
            CodestarNotificationsActions::ListTargets => {
                write!(f, "codestar-notifications:ListTargets")
            }
            CodestarNotificationsActions::Subscribe => {
                write!(f, "codestar-notifications:Subscribe")
            }
            CodestarNotificationsActions::TagResource => {
                write!(f, "codestar-notifications:TagResource")
            }
            CodestarNotificationsActions::Unsubscribe => {
                write!(f, "codestar-notifications:Unsubscribe")
            }
            CodestarNotificationsActions::UntagResource => {
                write!(f, "codestar-notifications:UntagResource")
            }
            CodestarNotificationsActions::UpdateNotificationRule => {
                write!(f, "codestar-notifications:UpdateNotificationRule")
            }
        }
    }
}
