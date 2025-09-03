// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum GroundtruthlabelingActions {
    AssociatePatchToManifestJob,
    CreateBatch,
    CreateIntakeForm,
    CreateProject,
    CreateWorkflowDefinition,
    DescribeConsoleJob,
    GenerateLidarPreviewTaskConfigJob,
    GetBatch,
    GetIntakeFormStatus,
    ListBatches,
    ListDatasetObjects,
    ListProjects,
    RunFilterOrSampleDatasetJob,
    RunGenerateManifestByCrawlingJob,
    RunGenerateManifestMetricsJob,
    UpdateBatch,
}
impl std::fmt::Display for GroundtruthlabelingActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GroundtruthlabelingActions::AssociatePatchToManifestJob => {
                write!(f, "groundtruthlabeling:AssociatePatchToManifestJob")
            }
            GroundtruthlabelingActions::CreateBatch => write!(f, "groundtruthlabeling:CreateBatch"),
            GroundtruthlabelingActions::CreateIntakeForm => {
                write!(f, "groundtruthlabeling:CreateIntakeForm")
            }
            GroundtruthlabelingActions::CreateProject => {
                write!(f, "groundtruthlabeling:CreateProject")
            }
            GroundtruthlabelingActions::CreateWorkflowDefinition => {
                write!(f, "groundtruthlabeling:CreateWorkflowDefinition")
            }
            GroundtruthlabelingActions::DescribeConsoleJob => {
                write!(f, "groundtruthlabeling:DescribeConsoleJob")
            }
            GroundtruthlabelingActions::GenerateLidarPreviewTaskConfigJob => {
                write!(f, "groundtruthlabeling:GenerateLIDARPreviewTaskConfigJob")
            }
            GroundtruthlabelingActions::GetBatch => write!(f, "groundtruthlabeling:GetBatch"),
            GroundtruthlabelingActions::GetIntakeFormStatus => {
                write!(f, "groundtruthlabeling:GetIntakeFormStatus")
            }
            GroundtruthlabelingActions::ListBatches => write!(f, "groundtruthlabeling:ListBatches"),
            GroundtruthlabelingActions::ListDatasetObjects => {
                write!(f, "groundtruthlabeling:ListDatasetObjects")
            }
            GroundtruthlabelingActions::ListProjects => {
                write!(f, "groundtruthlabeling:ListProjects")
            }
            GroundtruthlabelingActions::RunFilterOrSampleDatasetJob => {
                write!(f, "groundtruthlabeling:RunFilterOrSampleDatasetJob")
            }
            GroundtruthlabelingActions::RunGenerateManifestByCrawlingJob => {
                write!(f, "groundtruthlabeling:RunGenerateManifestByCrawlingJob")
            }
            GroundtruthlabelingActions::RunGenerateManifestMetricsJob => {
                write!(f, "groundtruthlabeling:RunGenerateManifestMetricsJob")
            }
            GroundtruthlabelingActions::UpdateBatch => write!(f, "groundtruthlabeling:UpdateBatch"),
        }
    }
}
