// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum SsmContactsActions {
    AcceptPage,
    ActivateContactChannel,
    AssociateContact,
    CreateContact,
    CreateContactChannel,
    CreateRotation,
    CreateRotationOverride,
    DeactivateContactChannel,
    DeleteContact,
    DeleteContactChannel,
    DeleteRotation,
    DeleteRotationOverride,
    DescribeEngagement,
    DescribePage,
    GetContact,
    GetContactChannel,
    GetContactPolicy,
    GetRotation,
    GetRotationOverride,
    ListContactChannels,
    ListContacts,
    ListEngagements,
    ListPageReceipts,
    ListPageResolutions,
    ListPagesByContact,
    ListPagesByEngagement,
    ListPreviewRotationShifts,
    ListRotationOverrides,
    ListRotationShifts,
    ListRotations,
    ListTagsForResource,
    PutContactPolicy,
    SendActivationCode,
    StartEngagement,
    StopEngagement,
    TagResource,
    UntagResource,
    UpdateContact,
    UpdateContactChannel,
    UpdateRotation,
}
impl std::fmt::Display for SsmContactsActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SsmContactsActions::AcceptPage => write!(f, "ssm-contacts:AcceptPage"),
            SsmContactsActions::ActivateContactChannel => {
                write!(f, "ssm-contacts:ActivateContactChannel")
            }
            SsmContactsActions::AssociateContact => write!(f, "ssm-contacts:AssociateContact"),
            SsmContactsActions::CreateContact => write!(f, "ssm-contacts:CreateContact"),
            SsmContactsActions::CreateContactChannel => {
                write!(f, "ssm-contacts:CreateContactChannel")
            }
            SsmContactsActions::CreateRotation => write!(f, "ssm-contacts:CreateRotation"),
            SsmContactsActions::CreateRotationOverride => {
                write!(f, "ssm-contacts:CreateRotationOverride")
            }
            SsmContactsActions::DeactivateContactChannel => {
                write!(f, "ssm-contacts:DeactivateContactChannel")
            }
            SsmContactsActions::DeleteContact => write!(f, "ssm-contacts:DeleteContact"),
            SsmContactsActions::DeleteContactChannel => {
                write!(f, "ssm-contacts:DeleteContactChannel")
            }
            SsmContactsActions::DeleteRotation => write!(f, "ssm-contacts:DeleteRotation"),
            SsmContactsActions::DeleteRotationOverride => {
                write!(f, "ssm-contacts:DeleteRotationOverride")
            }
            SsmContactsActions::DescribeEngagement => write!(f, "ssm-contacts:DescribeEngagement"),
            SsmContactsActions::DescribePage => write!(f, "ssm-contacts:DescribePage"),
            SsmContactsActions::GetContact => write!(f, "ssm-contacts:GetContact"),
            SsmContactsActions::GetContactChannel => write!(f, "ssm-contacts:GetContactChannel"),
            SsmContactsActions::GetContactPolicy => write!(f, "ssm-contacts:GetContactPolicy"),
            SsmContactsActions::GetRotation => write!(f, "ssm-contacts:GetRotation"),
            SsmContactsActions::GetRotationOverride => {
                write!(f, "ssm-contacts:GetRotationOverride")
            }
            SsmContactsActions::ListContactChannels => {
                write!(f, "ssm-contacts:ListContactChannels")
            }
            SsmContactsActions::ListContacts => write!(f, "ssm-contacts:ListContacts"),
            SsmContactsActions::ListEngagements => write!(f, "ssm-contacts:ListEngagements"),
            SsmContactsActions::ListPageReceipts => write!(f, "ssm-contacts:ListPageReceipts"),
            SsmContactsActions::ListPageResolutions => {
                write!(f, "ssm-contacts:ListPageResolutions")
            }
            SsmContactsActions::ListPagesByContact => write!(f, "ssm-contacts:ListPagesByContact"),
            SsmContactsActions::ListPagesByEngagement => {
                write!(f, "ssm-contacts:ListPagesByEngagement")
            }
            SsmContactsActions::ListPreviewRotationShifts => {
                write!(f, "ssm-contacts:ListPreviewRotationShifts")
            }
            SsmContactsActions::ListRotationOverrides => {
                write!(f, "ssm-contacts:ListRotationOverrides")
            }
            SsmContactsActions::ListRotationShifts => write!(f, "ssm-contacts:ListRotationShifts"),
            SsmContactsActions::ListRotations => write!(f, "ssm-contacts:ListRotations"),
            SsmContactsActions::ListTagsForResource => {
                write!(f, "ssm-contacts:ListTagsForResource")
            }
            SsmContactsActions::PutContactPolicy => write!(f, "ssm-contacts:PutContactPolicy"),
            SsmContactsActions::SendActivationCode => write!(f, "ssm-contacts:SendActivationCode"),
            SsmContactsActions::StartEngagement => write!(f, "ssm-contacts:StartEngagement"),
            SsmContactsActions::StopEngagement => write!(f, "ssm-contacts:StopEngagement"),
            SsmContactsActions::TagResource => write!(f, "ssm-contacts:TagResource"),
            SsmContactsActions::UntagResource => write!(f, "ssm-contacts:UntagResource"),
            SsmContactsActions::UpdateContact => write!(f, "ssm-contacts:UpdateContact"),
            SsmContactsActions::UpdateContactChannel => {
                write!(f, "ssm-contacts:UpdateContactChannel")
            }
            SsmContactsActions::UpdateRotation => write!(f, "ssm-contacts:UpdateRotation"),
        }
    }
}
