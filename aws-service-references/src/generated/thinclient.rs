// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum ThinclientActions {
    CreateEnvironment,
    DeleteDevice,
    DeleteEnvironment,
    DeregisterDevice,
    GetDevice,
    GetDeviceDetails,
    GetEnvironment,
    GetSoftwareSet,
    ListDeviceSessions,
    ListDevices,
    ListEnvironments,
    ListSoftwareSets,
    ListTagsForResource,
    TagResource,
    UntagResource,
    UpdateDevice,
    UpdateEnvironment,
    UpdateSoftwareSet,
}
impl std::fmt::Display for ThinclientActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ThinclientActions::CreateEnvironment => write!(f, "thinclient:CreateEnvironment"),
            ThinclientActions::DeleteDevice => write!(f, "thinclient:DeleteDevice"),
            ThinclientActions::DeleteEnvironment => write!(f, "thinclient:DeleteEnvironment"),
            ThinclientActions::DeregisterDevice => write!(f, "thinclient:DeregisterDevice"),
            ThinclientActions::GetDevice => write!(f, "thinclient:GetDevice"),
            ThinclientActions::GetDeviceDetails => write!(f, "thinclient:GetDeviceDetails"),
            ThinclientActions::GetEnvironment => write!(f, "thinclient:GetEnvironment"),
            ThinclientActions::GetSoftwareSet => write!(f, "thinclient:GetSoftwareSet"),
            ThinclientActions::ListDeviceSessions => write!(f, "thinclient:ListDeviceSessions"),
            ThinclientActions::ListDevices => write!(f, "thinclient:ListDevices"),
            ThinclientActions::ListEnvironments => write!(f, "thinclient:ListEnvironments"),
            ThinclientActions::ListSoftwareSets => write!(f, "thinclient:ListSoftwareSets"),
            ThinclientActions::ListTagsForResource => write!(f, "thinclient:ListTagsForResource"),
            ThinclientActions::TagResource => write!(f, "thinclient:TagResource"),
            ThinclientActions::UntagResource => write!(f, "thinclient:UntagResource"),
            ThinclientActions::UpdateDevice => write!(f, "thinclient:UpdateDevice"),
            ThinclientActions::UpdateEnvironment => write!(f, "thinclient:UpdateEnvironment"),
            ThinclientActions::UpdateSoftwareSet => write!(f, "thinclient:UpdateSoftwareSet"),
        }
    }
}
