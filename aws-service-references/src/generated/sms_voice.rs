// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum SmsVoiceActions {
    AssociateOriginationIdentity,
    AssociateProtectConfiguration,
    CreateConfigurationSet,
    CreateConfigurationSetEventDestination,
    CreateEventDestination,
    CreateOptOutList,
    CreatePool,
    CreateProtectConfiguration,
    CreateRegistration,
    CreateRegistrationAssociation,
    CreateRegistrationAttachment,
    CreateRegistrationVersion,
    CreateVerifiedDestinationNumber,
    DeleteAccountDefaultProtectConfiguration,
    DeleteConfigurationSet,
    DeleteConfigurationSetEventDestination,
    DeleteDefaultMessageType,
    DeleteDefaultSenderId,
    DeleteEventDestination,
    DeleteKeyword,
    DeleteMediaMessageSpendLimitOverride,
    DeleteOptOutList,
    DeleteOptedOutNumber,
    DeletePool,
    DeleteProtectConfiguration,
    DeleteProtectConfigurationRuleSetNumberOverride,
    DeleteRegistration,
    DeleteRegistrationAttachment,
    DeleteRegistrationFieldValue,
    DeleteResourcePolicy,
    DeleteTextMessageSpendLimitOverride,
    DeleteVerifiedDestinationNumber,
    DeleteVoiceMessageSpendLimitOverride,
    DescribeAccountAttributes,
    DescribeAccountLimits,
    DescribeConfigurationSets,
    DescribeKeywords,
    DescribeOptOutLists,
    DescribeOptedOutNumbers,
    DescribePhoneNumbers,
    DescribePools,
    DescribeProtectConfigurations,
    DescribeRegistrationAttachments,
    DescribeRegistrationFieldDefinitions,
    DescribeRegistrationFieldValues,
    DescribeRegistrationSectionDefinitions,
    DescribeRegistrationTypeDefinitions,
    DescribeRegistrationVersions,
    DescribeRegistrations,
    DescribeSenderIds,
    DescribeSpendLimits,
    DescribeVerifiedDestinationNumbers,
    DisassociateOriginationIdentity,
    DisassociateProtectConfiguration,
    DiscardRegistrationVersion,
    GetConfigurationSetEventDestinations,
    GetProtectConfigurationCountryRuleSet,
    GetResourcePolicy,
    ListConfigurationSets,
    ListPoolOriginationIdentities,
    ListProtectConfigurationRuleSetNumberOverrides,
    ListRegistrationAssociations,
    ListTagsForResource,
    PutKeyword,
    PutMessageFeedback,
    PutOptedOutNumber,
    PutProtectConfigurationRuleSetNumberOverride,
    PutRegistrationFieldValue,
    PutResourcePolicy,
    ReleasePhoneNumber,
    ReleaseSenderId,
    RequestPhoneNumber,
    RequestSenderId,
    SendDestinationNumberVerificationCode,
    SendMediaMessage,
    SendTextMessage,
    SendVoiceMessage,
    SetAccountDefaultProtectConfiguration,
    SetDefaultMessageFeedbackEnabled,
    SetDefaultMessageType,
    SetDefaultSenderId,
    SetMediaMessageSpendLimitOverride,
    SetTextMessageSpendLimitOverride,
    SetVoiceMessageSpendLimitOverride,
    SubmitRegistrationVersion,
    TagResource,
    UntagResource,
    UpdateConfigurationSetEventDestination,
    UpdateEventDestination,
    UpdatePhoneNumber,
    UpdatePool,
    UpdateProtectConfiguration,
    UpdateProtectConfigurationCountryRuleSet,
    UpdateSenderId,
    VerifyDestinationNumber,
}
impl std::fmt::Display for SmsVoiceActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SmsVoiceActions::AssociateOriginationIdentity => {
                write!(f, "sms-voice:AssociateOriginationIdentity")
            }
            SmsVoiceActions::AssociateProtectConfiguration => {
                write!(f, "sms-voice:AssociateProtectConfiguration")
            }
            SmsVoiceActions::CreateConfigurationSet => {
                write!(f, "sms-voice:CreateConfigurationSet")
            }
            SmsVoiceActions::CreateConfigurationSetEventDestination => {
                write!(f, "sms-voice:CreateConfigurationSetEventDestination")
            }
            SmsVoiceActions::CreateEventDestination => {
                write!(f, "sms-voice:CreateEventDestination")
            }
            SmsVoiceActions::CreateOptOutList => write!(f, "sms-voice:CreateOptOutList"),
            SmsVoiceActions::CreatePool => write!(f, "sms-voice:CreatePool"),
            SmsVoiceActions::CreateProtectConfiguration => {
                write!(f, "sms-voice:CreateProtectConfiguration")
            }
            SmsVoiceActions::CreateRegistration => write!(f, "sms-voice:CreateRegistration"),
            SmsVoiceActions::CreateRegistrationAssociation => {
                write!(f, "sms-voice:CreateRegistrationAssociation")
            }
            SmsVoiceActions::CreateRegistrationAttachment => {
                write!(f, "sms-voice:CreateRegistrationAttachment")
            }
            SmsVoiceActions::CreateRegistrationVersion => {
                write!(f, "sms-voice:CreateRegistrationVersion")
            }
            SmsVoiceActions::CreateVerifiedDestinationNumber => {
                write!(f, "sms-voice:CreateVerifiedDestinationNumber")
            }
            SmsVoiceActions::DeleteAccountDefaultProtectConfiguration => {
                write!(f, "sms-voice:DeleteAccountDefaultProtectConfiguration")
            }
            SmsVoiceActions::DeleteConfigurationSet => {
                write!(f, "sms-voice:DeleteConfigurationSet")
            }
            SmsVoiceActions::DeleteConfigurationSetEventDestination => {
                write!(f, "sms-voice:DeleteConfigurationSetEventDestination")
            }
            SmsVoiceActions::DeleteDefaultMessageType => {
                write!(f, "sms-voice:DeleteDefaultMessageType")
            }
            SmsVoiceActions::DeleteDefaultSenderId => write!(f, "sms-voice:DeleteDefaultSenderId"),
            SmsVoiceActions::DeleteEventDestination => {
                write!(f, "sms-voice:DeleteEventDestination")
            }
            SmsVoiceActions::DeleteKeyword => write!(f, "sms-voice:DeleteKeyword"),
            SmsVoiceActions::DeleteMediaMessageSpendLimitOverride => {
                write!(f, "sms-voice:DeleteMediaMessageSpendLimitOverride")
            }
            SmsVoiceActions::DeleteOptOutList => write!(f, "sms-voice:DeleteOptOutList"),
            SmsVoiceActions::DeleteOptedOutNumber => write!(f, "sms-voice:DeleteOptedOutNumber"),
            SmsVoiceActions::DeletePool => write!(f, "sms-voice:DeletePool"),
            SmsVoiceActions::DeleteProtectConfiguration => {
                write!(f, "sms-voice:DeleteProtectConfiguration")
            }
            SmsVoiceActions::DeleteProtectConfigurationRuleSetNumberOverride => write!(
                f,
                "sms-voice:DeleteProtectConfigurationRuleSetNumberOverride"
            ),
            SmsVoiceActions::DeleteRegistration => write!(f, "sms-voice:DeleteRegistration"),
            SmsVoiceActions::DeleteRegistrationAttachment => {
                write!(f, "sms-voice:DeleteRegistrationAttachment")
            }
            SmsVoiceActions::DeleteRegistrationFieldValue => {
                write!(f, "sms-voice:DeleteRegistrationFieldValue")
            }
            SmsVoiceActions::DeleteResourcePolicy => write!(f, "sms-voice:DeleteResourcePolicy"),
            SmsVoiceActions::DeleteTextMessageSpendLimitOverride => {
                write!(f, "sms-voice:DeleteTextMessageSpendLimitOverride")
            }
            SmsVoiceActions::DeleteVerifiedDestinationNumber => {
                write!(f, "sms-voice:DeleteVerifiedDestinationNumber")
            }
            SmsVoiceActions::DeleteVoiceMessageSpendLimitOverride => {
                write!(f, "sms-voice:DeleteVoiceMessageSpendLimitOverride")
            }
            SmsVoiceActions::DescribeAccountAttributes => {
                write!(f, "sms-voice:DescribeAccountAttributes")
            }
            SmsVoiceActions::DescribeAccountLimits => write!(f, "sms-voice:DescribeAccountLimits"),
            SmsVoiceActions::DescribeConfigurationSets => {
                write!(f, "sms-voice:DescribeConfigurationSets")
            }
            SmsVoiceActions::DescribeKeywords => write!(f, "sms-voice:DescribeKeywords"),
            SmsVoiceActions::DescribeOptOutLists => write!(f, "sms-voice:DescribeOptOutLists"),
            SmsVoiceActions::DescribeOptedOutNumbers => {
                write!(f, "sms-voice:DescribeOptedOutNumbers")
            }
            SmsVoiceActions::DescribePhoneNumbers => write!(f, "sms-voice:DescribePhoneNumbers"),
            SmsVoiceActions::DescribePools => write!(f, "sms-voice:DescribePools"),
            SmsVoiceActions::DescribeProtectConfigurations => {
                write!(f, "sms-voice:DescribeProtectConfigurations")
            }
            SmsVoiceActions::DescribeRegistrationAttachments => {
                write!(f, "sms-voice:DescribeRegistrationAttachments")
            }
            SmsVoiceActions::DescribeRegistrationFieldDefinitions => {
                write!(f, "sms-voice:DescribeRegistrationFieldDefinitions")
            }
            SmsVoiceActions::DescribeRegistrationFieldValues => {
                write!(f, "sms-voice:DescribeRegistrationFieldValues")
            }
            SmsVoiceActions::DescribeRegistrationSectionDefinitions => {
                write!(f, "sms-voice:DescribeRegistrationSectionDefinitions")
            }
            SmsVoiceActions::DescribeRegistrationTypeDefinitions => {
                write!(f, "sms-voice:DescribeRegistrationTypeDefinitions")
            }
            SmsVoiceActions::DescribeRegistrationVersions => {
                write!(f, "sms-voice:DescribeRegistrationVersions")
            }
            SmsVoiceActions::DescribeRegistrations => write!(f, "sms-voice:DescribeRegistrations"),
            SmsVoiceActions::DescribeSenderIds => write!(f, "sms-voice:DescribeSenderIds"),
            SmsVoiceActions::DescribeSpendLimits => write!(f, "sms-voice:DescribeSpendLimits"),
            SmsVoiceActions::DescribeVerifiedDestinationNumbers => {
                write!(f, "sms-voice:DescribeVerifiedDestinationNumbers")
            }
            SmsVoiceActions::DisassociateOriginationIdentity => {
                write!(f, "sms-voice:DisassociateOriginationIdentity")
            }
            SmsVoiceActions::DisassociateProtectConfiguration => {
                write!(f, "sms-voice:DisassociateProtectConfiguration")
            }
            SmsVoiceActions::DiscardRegistrationVersion => {
                write!(f, "sms-voice:DiscardRegistrationVersion")
            }
            SmsVoiceActions::GetConfigurationSetEventDestinations => {
                write!(f, "sms-voice:GetConfigurationSetEventDestinations")
            }
            SmsVoiceActions::GetProtectConfigurationCountryRuleSet => {
                write!(f, "sms-voice:GetProtectConfigurationCountryRuleSet")
            }
            SmsVoiceActions::GetResourcePolicy => write!(f, "sms-voice:GetResourcePolicy"),
            SmsVoiceActions::ListConfigurationSets => write!(f, "sms-voice:ListConfigurationSets"),
            SmsVoiceActions::ListPoolOriginationIdentities => {
                write!(f, "sms-voice:ListPoolOriginationIdentities")
            }
            SmsVoiceActions::ListProtectConfigurationRuleSetNumberOverrides => write!(
                f,
                "sms-voice:ListProtectConfigurationRuleSetNumberOverrides"
            ),
            SmsVoiceActions::ListRegistrationAssociations => {
                write!(f, "sms-voice:ListRegistrationAssociations")
            }
            SmsVoiceActions::ListTagsForResource => write!(f, "sms-voice:ListTagsForResource"),
            SmsVoiceActions::PutKeyword => write!(f, "sms-voice:PutKeyword"),
            SmsVoiceActions::PutMessageFeedback => write!(f, "sms-voice:PutMessageFeedback"),
            SmsVoiceActions::PutOptedOutNumber => write!(f, "sms-voice:PutOptedOutNumber"),
            SmsVoiceActions::PutProtectConfigurationRuleSetNumberOverride => {
                write!(f, "sms-voice:PutProtectConfigurationRuleSetNumberOverride")
            }
            SmsVoiceActions::PutRegistrationFieldValue => {
                write!(f, "sms-voice:PutRegistrationFieldValue")
            }
            SmsVoiceActions::PutResourcePolicy => write!(f, "sms-voice:PutResourcePolicy"),
            SmsVoiceActions::ReleasePhoneNumber => write!(f, "sms-voice:ReleasePhoneNumber"),
            SmsVoiceActions::ReleaseSenderId => write!(f, "sms-voice:ReleaseSenderId"),
            SmsVoiceActions::RequestPhoneNumber => write!(f, "sms-voice:RequestPhoneNumber"),
            SmsVoiceActions::RequestSenderId => write!(f, "sms-voice:RequestSenderId"),
            SmsVoiceActions::SendDestinationNumberVerificationCode => {
                write!(f, "sms-voice:SendDestinationNumberVerificationCode")
            }
            SmsVoiceActions::SendMediaMessage => write!(f, "sms-voice:SendMediaMessage"),
            SmsVoiceActions::SendTextMessage => write!(f, "sms-voice:SendTextMessage"),
            SmsVoiceActions::SendVoiceMessage => write!(f, "sms-voice:SendVoiceMessage"),
            SmsVoiceActions::SetAccountDefaultProtectConfiguration => {
                write!(f, "sms-voice:SetAccountDefaultProtectConfiguration")
            }
            SmsVoiceActions::SetDefaultMessageFeedbackEnabled => {
                write!(f, "sms-voice:SetDefaultMessageFeedbackEnabled")
            }
            SmsVoiceActions::SetDefaultMessageType => write!(f, "sms-voice:SetDefaultMessageType"),
            SmsVoiceActions::SetDefaultSenderId => write!(f, "sms-voice:SetDefaultSenderId"),
            SmsVoiceActions::SetMediaMessageSpendLimitOverride => {
                write!(f, "sms-voice:SetMediaMessageSpendLimitOverride")
            }
            SmsVoiceActions::SetTextMessageSpendLimitOverride => {
                write!(f, "sms-voice:SetTextMessageSpendLimitOverride")
            }
            SmsVoiceActions::SetVoiceMessageSpendLimitOverride => {
                write!(f, "sms-voice:SetVoiceMessageSpendLimitOverride")
            }
            SmsVoiceActions::SubmitRegistrationVersion => {
                write!(f, "sms-voice:SubmitRegistrationVersion")
            }
            SmsVoiceActions::TagResource => write!(f, "sms-voice:TagResource"),
            SmsVoiceActions::UntagResource => write!(f, "sms-voice:UntagResource"),
            SmsVoiceActions::UpdateConfigurationSetEventDestination => {
                write!(f, "sms-voice:UpdateConfigurationSetEventDestination")
            }
            SmsVoiceActions::UpdateEventDestination => {
                write!(f, "sms-voice:UpdateEventDestination")
            }
            SmsVoiceActions::UpdatePhoneNumber => write!(f, "sms-voice:UpdatePhoneNumber"),
            SmsVoiceActions::UpdatePool => write!(f, "sms-voice:UpdatePool"),
            SmsVoiceActions::UpdateProtectConfiguration => {
                write!(f, "sms-voice:UpdateProtectConfiguration")
            }
            SmsVoiceActions::UpdateProtectConfigurationCountryRuleSet => {
                write!(f, "sms-voice:UpdateProtectConfigurationCountryRuleSet")
            }
            SmsVoiceActions::UpdateSenderId => write!(f, "sms-voice:UpdateSenderId"),
            SmsVoiceActions::VerifyDestinationNumber => {
                write!(f, "sms-voice:VerifyDestinationNumber")
            }
        }
    }
}
