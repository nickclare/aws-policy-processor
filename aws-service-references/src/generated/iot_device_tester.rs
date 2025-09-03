// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum IotDeviceTesterActions {
    CheckVersion,
    DownloadTestSuite,
    LatestIdt,
    SendMetrics,
    SupportedVersion,
}
impl std::fmt::Display for IotDeviceTesterActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IotDeviceTesterActions::CheckVersion => write!(f, "iot-device-tester:CheckVersion"),
            IotDeviceTesterActions::DownloadTestSuite => {
                write!(f, "iot-device-tester:DownloadTestSuite")
            }
            IotDeviceTesterActions::LatestIdt => write!(f, "iot-device-tester:LatestIdt"),
            IotDeviceTesterActions::SendMetrics => write!(f, "iot-device-tester:SendMetrics"),
            IotDeviceTesterActions::SupportedVersion => {
                write!(f, "iot-device-tester:SupportedVersion")
            }
        }
    }
}
