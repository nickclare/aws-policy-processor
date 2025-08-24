// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum FrauddetectorActions {
    BatchCreateVariable,
    BatchGetVariable,
    CancelBatchImportJob,
    CancelBatchPredictionJob,
    CreateBatchImportJob,
    CreateBatchPredictionJob,
    CreateDetectorVersion,
    CreateList,
    CreateModel,
    CreateModelVersion,
    CreateRule,
    CreateVariable,
    DeleteBatchImportJob,
    DeleteBatchPredictionJob,
    DeleteDetector,
    DeleteDetectorVersion,
    DeleteEntityType,
    DeleteEvent,
    DeleteEventType,
    DeleteEventsByEventType,
    DeleteExternalModel,
    DeleteLabel,
    DeleteList,
    DeleteModel,
    DeleteModelVersion,
    DeleteOutcome,
    DeleteRule,
    DeleteVariable,
    DescribeDetector,
    DescribeModelVersions,
    GetBatchImportJobValidationReport,
    GetBatchImportJobs,
    GetBatchPredictionJobs,
    GetDeleteEventsByEventTypeStatus,
    GetDetectorVersion,
    GetDetectors,
    GetEntityTypes,
    GetEvent,
    GetEventPrediction,
    GetEventPredictionMetadata,
    GetEventTypes,
    GetExternalModels,
    GetKmsEncryptionKey,
    GetLabels,
    GetListElements,
    GetListsMetadata,
    GetModelVersion,
    GetModels,
    GetOutcomes,
    GetRules,
    GetVariables,
    ListEventPredictions,
    ListTagsForResource,
    PutDetector,
    PutEntityType,
    PutEventType,
    PutExternalModel,
    PutKmsEncryptionKey,
    PutLabel,
    PutOutcome,
    SendEvent,
    TagResource,
    UntagResource,
    UpdateDetectorVersion,
    UpdateDetectorVersionMetadata,
    UpdateDetectorVersionStatus,
    UpdateEventLabel,
    UpdateList,
    UpdateModel,
    UpdateModelVersion,
    UpdateModelVersionStatus,
    UpdateRuleMetadata,
    UpdateRuleVersion,
    UpdateVariable,
}
impl std::fmt::Display for FrauddetectorActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FrauddetectorActions::BatchCreateVariable => {
                write!(f, "frauddetector:BatchCreateVariable")
            }
            FrauddetectorActions::BatchGetVariable => write!(f, "frauddetector:BatchGetVariable"),
            FrauddetectorActions::CancelBatchImportJob => {
                write!(f, "frauddetector:CancelBatchImportJob")
            }
            FrauddetectorActions::CancelBatchPredictionJob => {
                write!(f, "frauddetector:CancelBatchPredictionJob")
            }
            FrauddetectorActions::CreateBatchImportJob => {
                write!(f, "frauddetector:CreateBatchImportJob")
            }
            FrauddetectorActions::CreateBatchPredictionJob => {
                write!(f, "frauddetector:CreateBatchPredictionJob")
            }
            FrauddetectorActions::CreateDetectorVersion => {
                write!(f, "frauddetector:CreateDetectorVersion")
            }
            FrauddetectorActions::CreateList => write!(f, "frauddetector:CreateList"),
            FrauddetectorActions::CreateModel => write!(f, "frauddetector:CreateModel"),
            FrauddetectorActions::CreateModelVersion => {
                write!(f, "frauddetector:CreateModelVersion")
            }
            FrauddetectorActions::CreateRule => write!(f, "frauddetector:CreateRule"),
            FrauddetectorActions::CreateVariable => write!(f, "frauddetector:CreateVariable"),
            FrauddetectorActions::DeleteBatchImportJob => {
                write!(f, "frauddetector:DeleteBatchImportJob")
            }
            FrauddetectorActions::DeleteBatchPredictionJob => {
                write!(f, "frauddetector:DeleteBatchPredictionJob")
            }
            FrauddetectorActions::DeleteDetector => write!(f, "frauddetector:DeleteDetector"),
            FrauddetectorActions::DeleteDetectorVersion => {
                write!(f, "frauddetector:DeleteDetectorVersion")
            }
            FrauddetectorActions::DeleteEntityType => write!(f, "frauddetector:DeleteEntityType"),
            FrauddetectorActions::DeleteEvent => write!(f, "frauddetector:DeleteEvent"),
            FrauddetectorActions::DeleteEventType => write!(f, "frauddetector:DeleteEventType"),
            FrauddetectorActions::DeleteEventsByEventType => {
                write!(f, "frauddetector:DeleteEventsByEventType")
            }
            FrauddetectorActions::DeleteExternalModel => {
                write!(f, "frauddetector:DeleteExternalModel")
            }
            FrauddetectorActions::DeleteLabel => write!(f, "frauddetector:DeleteLabel"),
            FrauddetectorActions::DeleteList => write!(f, "frauddetector:DeleteList"),
            FrauddetectorActions::DeleteModel => write!(f, "frauddetector:DeleteModel"),
            FrauddetectorActions::DeleteModelVersion => {
                write!(f, "frauddetector:DeleteModelVersion")
            }
            FrauddetectorActions::DeleteOutcome => write!(f, "frauddetector:DeleteOutcome"),
            FrauddetectorActions::DeleteRule => write!(f, "frauddetector:DeleteRule"),
            FrauddetectorActions::DeleteVariable => write!(f, "frauddetector:DeleteVariable"),
            FrauddetectorActions::DescribeDetector => write!(f, "frauddetector:DescribeDetector"),
            FrauddetectorActions::DescribeModelVersions => {
                write!(f, "frauddetector:DescribeModelVersions")
            }
            FrauddetectorActions::GetBatchImportJobValidationReport => {
                write!(f, "frauddetector:GetBatchImportJobValidationReport")
            }
            FrauddetectorActions::GetBatchImportJobs => {
                write!(f, "frauddetector:GetBatchImportJobs")
            }
            FrauddetectorActions::GetBatchPredictionJobs => {
                write!(f, "frauddetector:GetBatchPredictionJobs")
            }
            FrauddetectorActions::GetDeleteEventsByEventTypeStatus => {
                write!(f, "frauddetector:GetDeleteEventsByEventTypeStatus")
            }
            FrauddetectorActions::GetDetectorVersion => {
                write!(f, "frauddetector:GetDetectorVersion")
            }
            FrauddetectorActions::GetDetectors => write!(f, "frauddetector:GetDetectors"),
            FrauddetectorActions::GetEntityTypes => write!(f, "frauddetector:GetEntityTypes"),
            FrauddetectorActions::GetEvent => write!(f, "frauddetector:GetEvent"),
            FrauddetectorActions::GetEventPrediction => {
                write!(f, "frauddetector:GetEventPrediction")
            }
            FrauddetectorActions::GetEventPredictionMetadata => {
                write!(f, "frauddetector:GetEventPredictionMetadata")
            }
            FrauddetectorActions::GetEventTypes => write!(f, "frauddetector:GetEventTypes"),
            FrauddetectorActions::GetExternalModels => write!(f, "frauddetector:GetExternalModels"),
            FrauddetectorActions::GetKmsEncryptionKey => {
                write!(f, "frauddetector:GetKMSEncryptionKey")
            }
            FrauddetectorActions::GetLabels => write!(f, "frauddetector:GetLabels"),
            FrauddetectorActions::GetListElements => write!(f, "frauddetector:GetListElements"),
            FrauddetectorActions::GetListsMetadata => write!(f, "frauddetector:GetListsMetadata"),
            FrauddetectorActions::GetModelVersion => write!(f, "frauddetector:GetModelVersion"),
            FrauddetectorActions::GetModels => write!(f, "frauddetector:GetModels"),
            FrauddetectorActions::GetOutcomes => write!(f, "frauddetector:GetOutcomes"),
            FrauddetectorActions::GetRules => write!(f, "frauddetector:GetRules"),
            FrauddetectorActions::GetVariables => write!(f, "frauddetector:GetVariables"),
            FrauddetectorActions::ListEventPredictions => {
                write!(f, "frauddetector:ListEventPredictions")
            }
            FrauddetectorActions::ListTagsForResource => {
                write!(f, "frauddetector:ListTagsForResource")
            }
            FrauddetectorActions::PutDetector => write!(f, "frauddetector:PutDetector"),
            FrauddetectorActions::PutEntityType => write!(f, "frauddetector:PutEntityType"),
            FrauddetectorActions::PutEventType => write!(f, "frauddetector:PutEventType"),
            FrauddetectorActions::PutExternalModel => write!(f, "frauddetector:PutExternalModel"),
            FrauddetectorActions::PutKmsEncryptionKey => {
                write!(f, "frauddetector:PutKMSEncryptionKey")
            }
            FrauddetectorActions::PutLabel => write!(f, "frauddetector:PutLabel"),
            FrauddetectorActions::PutOutcome => write!(f, "frauddetector:PutOutcome"),
            FrauddetectorActions::SendEvent => write!(f, "frauddetector:SendEvent"),
            FrauddetectorActions::TagResource => write!(f, "frauddetector:TagResource"),
            FrauddetectorActions::UntagResource => write!(f, "frauddetector:UntagResource"),
            FrauddetectorActions::UpdateDetectorVersion => {
                write!(f, "frauddetector:UpdateDetectorVersion")
            }
            FrauddetectorActions::UpdateDetectorVersionMetadata => {
                write!(f, "frauddetector:UpdateDetectorVersionMetadata")
            }
            FrauddetectorActions::UpdateDetectorVersionStatus => {
                write!(f, "frauddetector:UpdateDetectorVersionStatus")
            }
            FrauddetectorActions::UpdateEventLabel => write!(f, "frauddetector:UpdateEventLabel"),
            FrauddetectorActions::UpdateList => write!(f, "frauddetector:UpdateList"),
            FrauddetectorActions::UpdateModel => write!(f, "frauddetector:UpdateModel"),
            FrauddetectorActions::UpdateModelVersion => {
                write!(f, "frauddetector:UpdateModelVersion")
            }
            FrauddetectorActions::UpdateModelVersionStatus => {
                write!(f, "frauddetector:UpdateModelVersionStatus")
            }
            FrauddetectorActions::UpdateRuleMetadata => {
                write!(f, "frauddetector:UpdateRuleMetadata")
            }
            FrauddetectorActions::UpdateRuleVersion => write!(f, "frauddetector:UpdateRuleVersion"),
            FrauddetectorActions::UpdateVariable => write!(f, "frauddetector:UpdateVariable"),
        }
    }
}
