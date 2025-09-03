// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum IoteventsActions {
    BatchAcknowledgeAlarm,
    BatchDeleteDetector,
    BatchDisableAlarm,
    BatchEnableAlarm,
    BatchPutMessage,
    BatchResetAlarm,
    BatchSnoozeAlarm,
    BatchUpdateDetector,
    CreateAlarmModel,
    CreateDetectorModel,
    CreateInput,
    DeleteAlarmModel,
    DeleteDetectorModel,
    DeleteInput,
    DescribeAlarm,
    DescribeAlarmModel,
    DescribeDetector,
    DescribeDetectorModel,
    DescribeDetectorModelAnalysis,
    DescribeInput,
    DescribeLoggingOptions,
    GetDetectorModelAnalysisResults,
    ListAlarmModelVersions,
    ListAlarmModels,
    ListAlarms,
    ListDetectorModelVersions,
    ListDetectorModels,
    ListDetectors,
    ListInputRoutings,
    ListInputs,
    ListTagsForResource,
    PutLoggingOptions,
    StartDetectorModelAnalysis,
    TagResource,
    UntagResource,
    UpdateAlarmModel,
    UpdateDetectorModel,
    UpdateInput,
    UpdateInputRouting,
}
impl std::fmt::Display for IoteventsActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IoteventsActions::BatchAcknowledgeAlarm => write!(f, "iotevents:BatchAcknowledgeAlarm"),
            IoteventsActions::BatchDeleteDetector => write!(f, "iotevents:BatchDeleteDetector"),
            IoteventsActions::BatchDisableAlarm => write!(f, "iotevents:BatchDisableAlarm"),
            IoteventsActions::BatchEnableAlarm => write!(f, "iotevents:BatchEnableAlarm"),
            IoteventsActions::BatchPutMessage => write!(f, "iotevents:BatchPutMessage"),
            IoteventsActions::BatchResetAlarm => write!(f, "iotevents:BatchResetAlarm"),
            IoteventsActions::BatchSnoozeAlarm => write!(f, "iotevents:BatchSnoozeAlarm"),
            IoteventsActions::BatchUpdateDetector => write!(f, "iotevents:BatchUpdateDetector"),
            IoteventsActions::CreateAlarmModel => write!(f, "iotevents:CreateAlarmModel"),
            IoteventsActions::CreateDetectorModel => write!(f, "iotevents:CreateDetectorModel"),
            IoteventsActions::CreateInput => write!(f, "iotevents:CreateInput"),
            IoteventsActions::DeleteAlarmModel => write!(f, "iotevents:DeleteAlarmModel"),
            IoteventsActions::DeleteDetectorModel => write!(f, "iotevents:DeleteDetectorModel"),
            IoteventsActions::DeleteInput => write!(f, "iotevents:DeleteInput"),
            IoteventsActions::DescribeAlarm => write!(f, "iotevents:DescribeAlarm"),
            IoteventsActions::DescribeAlarmModel => write!(f, "iotevents:DescribeAlarmModel"),
            IoteventsActions::DescribeDetector => write!(f, "iotevents:DescribeDetector"),
            IoteventsActions::DescribeDetectorModel => write!(f, "iotevents:DescribeDetectorModel"),
            IoteventsActions::DescribeDetectorModelAnalysis => {
                write!(f, "iotevents:DescribeDetectorModelAnalysis")
            }
            IoteventsActions::DescribeInput => write!(f, "iotevents:DescribeInput"),
            IoteventsActions::DescribeLoggingOptions => {
                write!(f, "iotevents:DescribeLoggingOptions")
            }
            IoteventsActions::GetDetectorModelAnalysisResults => {
                write!(f, "iotevents:GetDetectorModelAnalysisResults")
            }
            IoteventsActions::ListAlarmModelVersions => {
                write!(f, "iotevents:ListAlarmModelVersions")
            }
            IoteventsActions::ListAlarmModels => write!(f, "iotevents:ListAlarmModels"),
            IoteventsActions::ListAlarms => write!(f, "iotevents:ListAlarms"),
            IoteventsActions::ListDetectorModelVersions => {
                write!(f, "iotevents:ListDetectorModelVersions")
            }
            IoteventsActions::ListDetectorModels => write!(f, "iotevents:ListDetectorModels"),
            IoteventsActions::ListDetectors => write!(f, "iotevents:ListDetectors"),
            IoteventsActions::ListInputRoutings => write!(f, "iotevents:ListInputRoutings"),
            IoteventsActions::ListInputs => write!(f, "iotevents:ListInputs"),
            IoteventsActions::ListTagsForResource => write!(f, "iotevents:ListTagsForResource"),
            IoteventsActions::PutLoggingOptions => write!(f, "iotevents:PutLoggingOptions"),
            IoteventsActions::StartDetectorModelAnalysis => {
                write!(f, "iotevents:StartDetectorModelAnalysis")
            }
            IoteventsActions::TagResource => write!(f, "iotevents:TagResource"),
            IoteventsActions::UntagResource => write!(f, "iotevents:UntagResource"),
            IoteventsActions::UpdateAlarmModel => write!(f, "iotevents:UpdateAlarmModel"),
            IoteventsActions::UpdateDetectorModel => write!(f, "iotevents:UpdateDetectorModel"),
            IoteventsActions::UpdateInput => write!(f, "iotevents:UpdateInput"),
            IoteventsActions::UpdateInputRouting => write!(f, "iotevents:UpdateInputRouting"),
        }
    }
}
