// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum SnowDeviceManagementActions {
    CancelTask,
    CreateTask,
    DescribeDevice,
    DescribeDeviceEc2Instances,
    DescribeExecution,
    DescribeTask,
    ListDeviceResources,
    ListDevices,
    ListExecutions,
    ListTagsForResource,
    ListTasks,
    TagResource,
    UntagResource,
}
impl std::fmt::Display for SnowDeviceManagementActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SnowDeviceManagementActions::CancelTask => {
                write!(f, "snow-device-management:CancelTask")
            }
            SnowDeviceManagementActions::CreateTask => {
                write!(f, "snow-device-management:CreateTask")
            }
            SnowDeviceManagementActions::DescribeDevice => {
                write!(f, "snow-device-management:DescribeDevice")
            }
            SnowDeviceManagementActions::DescribeDeviceEc2Instances => {
                write!(f, "snow-device-management:DescribeDeviceEc2Instances")
            }
            SnowDeviceManagementActions::DescribeExecution => {
                write!(f, "snow-device-management:DescribeExecution")
            }
            SnowDeviceManagementActions::DescribeTask => {
                write!(f, "snow-device-management:DescribeTask")
            }
            SnowDeviceManagementActions::ListDeviceResources => {
                write!(f, "snow-device-management:ListDeviceResources")
            }
            SnowDeviceManagementActions::ListDevices => {
                write!(f, "snow-device-management:ListDevices")
            }
            SnowDeviceManagementActions::ListExecutions => {
                write!(f, "snow-device-management:ListExecutions")
            }
            SnowDeviceManagementActions::ListTagsForResource => {
                write!(f, "snow-device-management:ListTagsForResource")
            }
            SnowDeviceManagementActions::ListTasks => write!(f, "snow-device-management:ListTasks"),
            SnowDeviceManagementActions::TagResource => {
                write!(f, "snow-device-management:TagResource")
            }
            SnowDeviceManagementActions::UntagResource => {
                write!(f, "snow-device-management:UntagResource")
            }
        }
    }
}
