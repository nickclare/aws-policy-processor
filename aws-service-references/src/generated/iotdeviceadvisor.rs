// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum IotdeviceadvisorActions {
    CreateSuiteDefinition,
    DeleteSuiteDefinition,
    GetEndpoint,
    GetSuiteDefinition,
    GetSuiteRun,
    GetSuiteRunReport,
    ListSuiteDefinitions,
    ListSuiteRuns,
    ListTagsForResource,
    StartSuiteRun,
    StopSuiteRun,
    TagResource,
    UntagResource,
    UpdateSuiteDefinition,
}
impl std::fmt::Display for IotdeviceadvisorActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IotdeviceadvisorActions::CreateSuiteDefinition => {
                write!(f, "iotdeviceadvisor:CreateSuiteDefinition")
            }
            IotdeviceadvisorActions::DeleteSuiteDefinition => {
                write!(f, "iotdeviceadvisor:DeleteSuiteDefinition")
            }
            IotdeviceadvisorActions::GetEndpoint => write!(f, "iotdeviceadvisor:GetEndpoint"),
            IotdeviceadvisorActions::GetSuiteDefinition => {
                write!(f, "iotdeviceadvisor:GetSuiteDefinition")
            }
            IotdeviceadvisorActions::GetSuiteRun => write!(f, "iotdeviceadvisor:GetSuiteRun"),
            IotdeviceadvisorActions::GetSuiteRunReport => {
                write!(f, "iotdeviceadvisor:GetSuiteRunReport")
            }
            IotdeviceadvisorActions::ListSuiteDefinitions => {
                write!(f, "iotdeviceadvisor:ListSuiteDefinitions")
            }
            IotdeviceadvisorActions::ListSuiteRuns => write!(f, "iotdeviceadvisor:ListSuiteRuns"),
            IotdeviceadvisorActions::ListTagsForResource => {
                write!(f, "iotdeviceadvisor:ListTagsForResource")
            }
            IotdeviceadvisorActions::StartSuiteRun => write!(f, "iotdeviceadvisor:StartSuiteRun"),
            IotdeviceadvisorActions::StopSuiteRun => write!(f, "iotdeviceadvisor:StopSuiteRun"),
            IotdeviceadvisorActions::TagResource => write!(f, "iotdeviceadvisor:TagResource"),
            IotdeviceadvisorActions::UntagResource => write!(f, "iotdeviceadvisor:UntagResource"),
            IotdeviceadvisorActions::UpdateSuiteDefinition => {
                write!(f, "iotdeviceadvisor:UpdateSuiteDefinition")
            }
        }
    }
}
