// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum Iot1clickActions {
    AssociateDeviceWithPlacement,
    ClaimDevicesByClaimCode,
    CreatePlacement,
    CreateProject,
    DeletePlacement,
    DeleteProject,
    DescribeDevice,
    DescribePlacement,
    DescribeProject,
    DisassociateDeviceFromPlacement,
    FinalizeDeviceClaim,
    GetDeviceMethods,
    GetDevicesInPlacement,
    InitiateDeviceClaim,
    InvokeDeviceMethod,
    ListDeviceEvents,
    ListDevices,
    ListPlacements,
    ListProjects,
    ListTagsForResource,
    TagResource,
    UnclaimDevice,
    UntagResource,
    UpdateDeviceState,
    UpdatePlacement,
    UpdateProject,
}
impl std::fmt::Display for Iot1clickActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Iot1clickActions::AssociateDeviceWithPlacement => {
                write!(f, "iot1click:AssociateDeviceWithPlacement")
            }
            Iot1clickActions::ClaimDevicesByClaimCode => {
                write!(f, "iot1click:ClaimDevicesByClaimCode")
            }
            Iot1clickActions::CreatePlacement => write!(f, "iot1click:CreatePlacement"),
            Iot1clickActions::CreateProject => write!(f, "iot1click:CreateProject"),
            Iot1clickActions::DeletePlacement => write!(f, "iot1click:DeletePlacement"),
            Iot1clickActions::DeleteProject => write!(f, "iot1click:DeleteProject"),
            Iot1clickActions::DescribeDevice => write!(f, "iot1click:DescribeDevice"),
            Iot1clickActions::DescribePlacement => write!(f, "iot1click:DescribePlacement"),
            Iot1clickActions::DescribeProject => write!(f, "iot1click:DescribeProject"),
            Iot1clickActions::DisassociateDeviceFromPlacement => {
                write!(f, "iot1click:DisassociateDeviceFromPlacement")
            }
            Iot1clickActions::FinalizeDeviceClaim => write!(f, "iot1click:FinalizeDeviceClaim"),
            Iot1clickActions::GetDeviceMethods => write!(f, "iot1click:GetDeviceMethods"),
            Iot1clickActions::GetDevicesInPlacement => write!(f, "iot1click:GetDevicesInPlacement"),
            Iot1clickActions::InitiateDeviceClaim => write!(f, "iot1click:InitiateDeviceClaim"),
            Iot1clickActions::InvokeDeviceMethod => write!(f, "iot1click:InvokeDeviceMethod"),
            Iot1clickActions::ListDeviceEvents => write!(f, "iot1click:ListDeviceEvents"),
            Iot1clickActions::ListDevices => write!(f, "iot1click:ListDevices"),
            Iot1clickActions::ListPlacements => write!(f, "iot1click:ListPlacements"),
            Iot1clickActions::ListProjects => write!(f, "iot1click:ListProjects"),
            Iot1clickActions::ListTagsForResource => write!(f, "iot1click:ListTagsForResource"),
            Iot1clickActions::TagResource => write!(f, "iot1click:TagResource"),
            Iot1clickActions::UnclaimDevice => write!(f, "iot1click:UnclaimDevice"),
            Iot1clickActions::UntagResource => write!(f, "iot1click:UntagResource"),
            Iot1clickActions::UpdateDeviceState => write!(f, "iot1click:UpdateDeviceState"),
            Iot1clickActions::UpdatePlacement => write!(f, "iot1click:UpdatePlacement"),
            Iot1clickActions::UpdateProject => write!(f, "iot1click:UpdateProject"),
        }
    }
}
