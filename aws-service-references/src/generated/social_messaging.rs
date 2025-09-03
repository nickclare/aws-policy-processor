// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum SocialMessagingActions {
    AssociateWhatsAppBusinessAccount,
    CreateWhatsAppMessageTemplate,
    CreateWhatsAppMessageTemplateFromLibrary,
    CreateWhatsAppMessageTemplateMedia,
    DeleteWhatsAppMessageMedia,
    DeleteWhatsAppMessageTemplate,
    DisassociateWhatsAppBusinessAccount,
    GetLinkedWhatsAppBusinessAccount,
    GetLinkedWhatsAppBusinessAccountPhoneNumber,
    GetWhatsAppMessageMedia,
    GetWhatsAppMessageTemplate,
    ListLinkedWhatsAppBusinessAccounts,
    ListTagsForResource,
    ListWhatsAppMessageTemplates,
    ListWhatsAppTemplateLibrary,
    PostWhatsAppMessageMedia,
    PutWhatsAppBusinessAccountEventDestinations,
    SendWhatsAppMessage,
    TagResource,
    UntagResource,
    UpdateWhatsAppMessageTemplate,
}
impl std::fmt::Display for SocialMessagingActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SocialMessagingActions::AssociateWhatsAppBusinessAccount => {
                write!(f, "social-messaging:AssociateWhatsAppBusinessAccount")
            }
            SocialMessagingActions::CreateWhatsAppMessageTemplate => {
                write!(f, "social-messaging:CreateWhatsAppMessageTemplate")
            }
            SocialMessagingActions::CreateWhatsAppMessageTemplateFromLibrary => write!(
                f,
                "social-messaging:CreateWhatsAppMessageTemplateFromLibrary"
            ),
            SocialMessagingActions::CreateWhatsAppMessageTemplateMedia => {
                write!(f, "social-messaging:CreateWhatsAppMessageTemplateMedia")
            }
            SocialMessagingActions::DeleteWhatsAppMessageMedia => {
                write!(f, "social-messaging:DeleteWhatsAppMessageMedia")
            }
            SocialMessagingActions::DeleteWhatsAppMessageTemplate => {
                write!(f, "social-messaging:DeleteWhatsAppMessageTemplate")
            }
            SocialMessagingActions::DisassociateWhatsAppBusinessAccount => {
                write!(f, "social-messaging:DisassociateWhatsAppBusinessAccount")
            }
            SocialMessagingActions::GetLinkedWhatsAppBusinessAccount => {
                write!(f, "social-messaging:GetLinkedWhatsAppBusinessAccount")
            }
            SocialMessagingActions::GetLinkedWhatsAppBusinessAccountPhoneNumber => write!(
                f,
                "social-messaging:GetLinkedWhatsAppBusinessAccountPhoneNumber"
            ),
            SocialMessagingActions::GetWhatsAppMessageMedia => {
                write!(f, "social-messaging:GetWhatsAppMessageMedia")
            }
            SocialMessagingActions::GetWhatsAppMessageTemplate => {
                write!(f, "social-messaging:GetWhatsAppMessageTemplate")
            }
            SocialMessagingActions::ListLinkedWhatsAppBusinessAccounts => {
                write!(f, "social-messaging:ListLinkedWhatsAppBusinessAccounts")
            }
            SocialMessagingActions::ListTagsForResource => {
                write!(f, "social-messaging:ListTagsForResource")
            }
            SocialMessagingActions::ListWhatsAppMessageTemplates => {
                write!(f, "social-messaging:ListWhatsAppMessageTemplates")
            }
            SocialMessagingActions::ListWhatsAppTemplateLibrary => {
                write!(f, "social-messaging:ListWhatsAppTemplateLibrary")
            }
            SocialMessagingActions::PostWhatsAppMessageMedia => {
                write!(f, "social-messaging:PostWhatsAppMessageMedia")
            }
            SocialMessagingActions::PutWhatsAppBusinessAccountEventDestinations => write!(
                f,
                "social-messaging:PutWhatsAppBusinessAccountEventDestinations"
            ),
            SocialMessagingActions::SendWhatsAppMessage => {
                write!(f, "social-messaging:SendWhatsAppMessage")
            }
            SocialMessagingActions::TagResource => write!(f, "social-messaging:TagResource"),
            SocialMessagingActions::UntagResource => write!(f, "social-messaging:UntagResource"),
            SocialMessagingActions::UpdateWhatsAppMessageTemplate => {
                write!(f, "social-messaging:UpdateWhatsAppMessageTemplate")
            }
        }
    }
}
