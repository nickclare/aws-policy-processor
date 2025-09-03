// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum LookoutequipmentActions {
    CreateDataset,
    CreateInferenceScheduler,
    CreateLabel,
    CreateLabelGroup,
    CreateModel,
    CreateRetrainingScheduler,
    DeleteDataset,
    DeleteInferenceScheduler,
    DeleteLabel,
    DeleteLabelGroup,
    DeleteModel,
    DeleteResourcePolicy,
    DeleteRetrainingScheduler,
    DescribeDataIngestionJob,
    DescribeDataset,
    DescribeInferenceScheduler,
    DescribeLabelGroup,
    DescribeModel,
    DescribeModelVersion,
    DescribeResourcePolicy,
    DescribeRetrainingScheduler,
    Describelabel,
    ImportDataset,
    ImportModelVersion,
    ListDataIngestionJobs,
    ListDatasets,
    ListInferenceEvents,
    ListInferenceExecutions,
    ListInferenceSchedulers,
    ListLabelGroups,
    ListLabels,
    ListModelVersions,
    ListModels,
    ListRetrainingSchedulers,
    ListSensorStatistics,
    ListTagsForResource,
    PutResourcePolicy,
    StartDataIngestionJob,
    StartInferenceScheduler,
    StartRetrainingScheduler,
    StopInferenceScheduler,
    StopRetrainingScheduler,
    TagResource,
    UntagResource,
    UpdateActiveModelVersion,
    UpdateInferenceScheduler,
    UpdateLabelGroup,
    UpdateModel,
    UpdateRetrainingScheduler,
}
impl std::fmt::Display for LookoutequipmentActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LookoutequipmentActions::CreateDataset => write!(f, "lookoutequipment:CreateDataset"),
            LookoutequipmentActions::CreateInferenceScheduler => {
                write!(f, "lookoutequipment:CreateInferenceScheduler")
            }
            LookoutequipmentActions::CreateLabel => write!(f, "lookoutequipment:CreateLabel"),
            LookoutequipmentActions::CreateLabelGroup => {
                write!(f, "lookoutequipment:CreateLabelGroup")
            }
            LookoutequipmentActions::CreateModel => write!(f, "lookoutequipment:CreateModel"),
            LookoutequipmentActions::CreateRetrainingScheduler => {
                write!(f, "lookoutequipment:CreateRetrainingScheduler")
            }
            LookoutequipmentActions::DeleteDataset => write!(f, "lookoutequipment:DeleteDataset"),
            LookoutequipmentActions::DeleteInferenceScheduler => {
                write!(f, "lookoutequipment:DeleteInferenceScheduler")
            }
            LookoutequipmentActions::DeleteLabel => write!(f, "lookoutequipment:DeleteLabel"),
            LookoutequipmentActions::DeleteLabelGroup => {
                write!(f, "lookoutequipment:DeleteLabelGroup")
            }
            LookoutequipmentActions::DeleteModel => write!(f, "lookoutequipment:DeleteModel"),
            LookoutequipmentActions::DeleteResourcePolicy => {
                write!(f, "lookoutequipment:DeleteResourcePolicy")
            }
            LookoutequipmentActions::DeleteRetrainingScheduler => {
                write!(f, "lookoutequipment:DeleteRetrainingScheduler")
            }
            LookoutequipmentActions::DescribeDataIngestionJob => {
                write!(f, "lookoutequipment:DescribeDataIngestionJob")
            }
            LookoutequipmentActions::DescribeDataset => {
                write!(f, "lookoutequipment:DescribeDataset")
            }
            LookoutequipmentActions::DescribeInferenceScheduler => {
                write!(f, "lookoutequipment:DescribeInferenceScheduler")
            }
            LookoutequipmentActions::DescribeLabelGroup => {
                write!(f, "lookoutequipment:DescribeLabelGroup")
            }
            LookoutequipmentActions::DescribeModel => write!(f, "lookoutequipment:DescribeModel"),
            LookoutequipmentActions::DescribeModelVersion => {
                write!(f, "lookoutequipment:DescribeModelVersion")
            }
            LookoutequipmentActions::DescribeResourcePolicy => {
                write!(f, "lookoutequipment:DescribeResourcePolicy")
            }
            LookoutequipmentActions::DescribeRetrainingScheduler => {
                write!(f, "lookoutequipment:DescribeRetrainingScheduler")
            }
            LookoutequipmentActions::Describelabel => write!(f, "lookoutequipment:Describelabel"),
            LookoutequipmentActions::ImportDataset => write!(f, "lookoutequipment:ImportDataset"),
            LookoutequipmentActions::ImportModelVersion => {
                write!(f, "lookoutequipment:ImportModelVersion")
            }
            LookoutequipmentActions::ListDataIngestionJobs => {
                write!(f, "lookoutequipment:ListDataIngestionJobs")
            }
            LookoutequipmentActions::ListDatasets => write!(f, "lookoutequipment:ListDatasets"),
            LookoutequipmentActions::ListInferenceEvents => {
                write!(f, "lookoutequipment:ListInferenceEvents")
            }
            LookoutequipmentActions::ListInferenceExecutions => {
                write!(f, "lookoutequipment:ListInferenceExecutions")
            }
            LookoutequipmentActions::ListInferenceSchedulers => {
                write!(f, "lookoutequipment:ListInferenceSchedulers")
            }
            LookoutequipmentActions::ListLabelGroups => {
                write!(f, "lookoutequipment:ListLabelGroups")
            }
            LookoutequipmentActions::ListLabels => write!(f, "lookoutequipment:ListLabels"),
            LookoutequipmentActions::ListModelVersions => {
                write!(f, "lookoutequipment:ListModelVersions")
            }
            LookoutequipmentActions::ListModels => write!(f, "lookoutequipment:ListModels"),
            LookoutequipmentActions::ListRetrainingSchedulers => {
                write!(f, "lookoutequipment:ListRetrainingSchedulers")
            }
            LookoutequipmentActions::ListSensorStatistics => {
                write!(f, "lookoutequipment:ListSensorStatistics")
            }
            LookoutequipmentActions::ListTagsForResource => {
                write!(f, "lookoutequipment:ListTagsForResource")
            }
            LookoutequipmentActions::PutResourcePolicy => {
                write!(f, "lookoutequipment:PutResourcePolicy")
            }
            LookoutequipmentActions::StartDataIngestionJob => {
                write!(f, "lookoutequipment:StartDataIngestionJob")
            }
            LookoutequipmentActions::StartInferenceScheduler => {
                write!(f, "lookoutequipment:StartInferenceScheduler")
            }
            LookoutequipmentActions::StartRetrainingScheduler => {
                write!(f, "lookoutequipment:StartRetrainingScheduler")
            }
            LookoutequipmentActions::StopInferenceScheduler => {
                write!(f, "lookoutequipment:StopInferenceScheduler")
            }
            LookoutequipmentActions::StopRetrainingScheduler => {
                write!(f, "lookoutequipment:StopRetrainingScheduler")
            }
            LookoutequipmentActions::TagResource => write!(f, "lookoutequipment:TagResource"),
            LookoutequipmentActions::UntagResource => write!(f, "lookoutequipment:UntagResource"),
            LookoutequipmentActions::UpdateActiveModelVersion => {
                write!(f, "lookoutequipment:UpdateActiveModelVersion")
            }
            LookoutequipmentActions::UpdateInferenceScheduler => {
                write!(f, "lookoutequipment:UpdateInferenceScheduler")
            }
            LookoutequipmentActions::UpdateLabelGroup => {
                write!(f, "lookoutequipment:UpdateLabelGroup")
            }
            LookoutequipmentActions::UpdateModel => write!(f, "lookoutequipment:UpdateModel"),
            LookoutequipmentActions::UpdateRetrainingScheduler => {
                write!(f, "lookoutequipment:UpdateRetrainingScheduler")
            }
        }
    }
}
