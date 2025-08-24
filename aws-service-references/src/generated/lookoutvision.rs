// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum LookoutvisionActions {
    CreateDataset,
    CreateModel,
    CreateProject,
    DeleteDataset,
    DeleteModel,
    DeleteProject,
    DescribeDataset,
    DescribeModel,
    DescribeModelPackagingJob,
    DescribeProject,
    DescribeTrialDetection,
    DetectAnomalies,
    ListDatasetEntries,
    ListModelPackagingJobs,
    ListModels,
    ListProjects,
    ListTagsForResource,
    ListTrialDetections,
    StartModel,
    StartModelPackagingJob,
    StartTrialDetection,
    StopModel,
    TagResource,
    UntagResource,
    UpdateDatasetEntries,
}
impl std::fmt::Display for LookoutvisionActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LookoutvisionActions::CreateDataset => write!(f, "lookoutvision:CreateDataset"),
            LookoutvisionActions::CreateModel => write!(f, "lookoutvision:CreateModel"),
            LookoutvisionActions::CreateProject => write!(f, "lookoutvision:CreateProject"),
            LookoutvisionActions::DeleteDataset => write!(f, "lookoutvision:DeleteDataset"),
            LookoutvisionActions::DeleteModel => write!(f, "lookoutvision:DeleteModel"),
            LookoutvisionActions::DeleteProject => write!(f, "lookoutvision:DeleteProject"),
            LookoutvisionActions::DescribeDataset => write!(f, "lookoutvision:DescribeDataset"),
            LookoutvisionActions::DescribeModel => write!(f, "lookoutvision:DescribeModel"),
            LookoutvisionActions::DescribeModelPackagingJob => {
                write!(f, "lookoutvision:DescribeModelPackagingJob")
            }
            LookoutvisionActions::DescribeProject => write!(f, "lookoutvision:DescribeProject"),
            LookoutvisionActions::DescribeTrialDetection => {
                write!(f, "lookoutvision:DescribeTrialDetection")
            }
            LookoutvisionActions::DetectAnomalies => write!(f, "lookoutvision:DetectAnomalies"),
            LookoutvisionActions::ListDatasetEntries => {
                write!(f, "lookoutvision:ListDatasetEntries")
            }
            LookoutvisionActions::ListModelPackagingJobs => {
                write!(f, "lookoutvision:ListModelPackagingJobs")
            }
            LookoutvisionActions::ListModels => write!(f, "lookoutvision:ListModels"),
            LookoutvisionActions::ListProjects => write!(f, "lookoutvision:ListProjects"),
            LookoutvisionActions::ListTagsForResource => {
                write!(f, "lookoutvision:ListTagsForResource")
            }
            LookoutvisionActions::ListTrialDetections => {
                write!(f, "lookoutvision:ListTrialDetections")
            }
            LookoutvisionActions::StartModel => write!(f, "lookoutvision:StartModel"),
            LookoutvisionActions::StartModelPackagingJob => {
                write!(f, "lookoutvision:StartModelPackagingJob")
            }
            LookoutvisionActions::StartTrialDetection => {
                write!(f, "lookoutvision:StartTrialDetection")
            }
            LookoutvisionActions::StopModel => write!(f, "lookoutvision:StopModel"),
            LookoutvisionActions::TagResource => write!(f, "lookoutvision:TagResource"),
            LookoutvisionActions::UntagResource => write!(f, "lookoutvision:UntagResource"),
            LookoutvisionActions::UpdateDatasetEntries => {
                write!(f, "lookoutvision:UpdateDatasetEntries")
            }
        }
    }
}
