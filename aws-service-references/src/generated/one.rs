// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum OneActions {
    CreateDeviceActivationQrCode,
    CreateDeviceConfigurationTemplate,
    CreateDeviceInstance,
    CreateDeviceInstanceConfiguration,
    CreateSite,
    DeleteAssociatedDevice,
    DeleteDeviceConfigurationTemplate,
    DeleteDeviceInstance,
    DeleteSite,
    DeleteUserV1,
    GetDeviceConfigurationTemplate,
    GetDeviceInstance,
    GetDeviceInstanceConfiguration,
    GetSite,
    GetSiteAddress,
    ListDeviceConfigurationTemplates,
    ListDeviceInstances,
    ListSites,
    ListTagsForResource,
    ListUsersV1,
    RebootDevice,
    TagResource,
    UntagResource,
    UpdateDeviceConfigurationTemplate,
    UpdateDeviceInstance,
    UpdateSite,
    UpdateSiteAddress,
}
impl std::fmt::Display for OneActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OneActions::CreateDeviceActivationQrCode => {
                write!(f, "one:CreateDeviceActivationQrCode")
            }
            OneActions::CreateDeviceConfigurationTemplate => {
                write!(f, "one:CreateDeviceConfigurationTemplate")
            }
            OneActions::CreateDeviceInstance => write!(f, "one:CreateDeviceInstance"),
            OneActions::CreateDeviceInstanceConfiguration => {
                write!(f, "one:CreateDeviceInstanceConfiguration")
            }
            OneActions::CreateSite => write!(f, "one:CreateSite"),
            OneActions::DeleteAssociatedDevice => write!(f, "one:DeleteAssociatedDevice"),
            OneActions::DeleteDeviceConfigurationTemplate => {
                write!(f, "one:DeleteDeviceConfigurationTemplate")
            }
            OneActions::DeleteDeviceInstance => write!(f, "one:DeleteDeviceInstance"),
            OneActions::DeleteSite => write!(f, "one:DeleteSite"),
            OneActions::DeleteUserV1 => write!(f, "one:DeleteUserV1"),
            OneActions::GetDeviceConfigurationTemplate => {
                write!(f, "one:GetDeviceConfigurationTemplate")
            }
            OneActions::GetDeviceInstance => write!(f, "one:GetDeviceInstance"),
            OneActions::GetDeviceInstanceConfiguration => {
                write!(f, "one:GetDeviceInstanceConfiguration")
            }
            OneActions::GetSite => write!(f, "one:GetSite"),
            OneActions::GetSiteAddress => write!(f, "one:GetSiteAddress"),
            OneActions::ListDeviceConfigurationTemplates => {
                write!(f, "one:ListDeviceConfigurationTemplates")
            }
            OneActions::ListDeviceInstances => write!(f, "one:ListDeviceInstances"),
            OneActions::ListSites => write!(f, "one:ListSites"),
            OneActions::ListTagsForResource => write!(f, "one:ListTagsForResource"),
            OneActions::ListUsersV1 => write!(f, "one:ListUsersV1"),
            OneActions::RebootDevice => write!(f, "one:RebootDevice"),
            OneActions::TagResource => write!(f, "one:TagResource"),
            OneActions::UntagResource => write!(f, "one:UntagResource"),
            OneActions::UpdateDeviceConfigurationTemplate => {
                write!(f, "one:UpdateDeviceConfigurationTemplate")
            }
            OneActions::UpdateDeviceInstance => write!(f, "one:UpdateDeviceInstance"),
            OneActions::UpdateSite => write!(f, "one:UpdateSite"),
            OneActions::UpdateSiteAddress => write!(f, "one:UpdateSiteAddress"),
        }
    }
}
