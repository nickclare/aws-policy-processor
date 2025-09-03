// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum DatasyncActions {
    AddStorageSystem,
    CancelTaskExecution,
    CreateAgent,
    CreateLocationAzureBlob,
    CreateLocationEfs,
    CreateLocationFsxLustre,
    CreateLocationFsxOntap,
    CreateLocationFsxOpenZfs,
    CreateLocationFsxWindows,
    CreateLocationHdfs,
    CreateLocationNfs,
    CreateLocationObjectStorage,
    CreateLocationS3,
    CreateLocationSmb,
    CreateTask,
    DeleteAgent,
    DeleteLocation,
    DeleteTask,
    DescribeAgent,
    DescribeDiscoveryJob,
    DescribeLocationAzureBlob,
    DescribeLocationEfs,
    DescribeLocationFsxLustre,
    DescribeLocationFsxOntap,
    DescribeLocationFsxOpenZfs,
    DescribeLocationFsxWindows,
    DescribeLocationHdfs,
    DescribeLocationNfs,
    DescribeLocationObjectStorage,
    DescribeLocationS3,
    DescribeLocationSmb,
    DescribeStorageSystem,
    DescribeStorageSystemResourceMetrics,
    DescribeStorageSystemResources,
    DescribeTask,
    DescribeTaskExecution,
    GenerateRecommendations,
    ListAgents,
    ListDiscoveryJobs,
    ListLocations,
    ListStorageSystems,
    ListTagsForResource,
    ListTaskExecutions,
    ListTasks,
    RemoveStorageSystem,
    StartDiscoveryJob,
    StartTaskExecution,
    StopDiscoveryJob,
    TagResource,
    UntagResource,
    UpdateAgent,
    UpdateDiscoveryJob,
    UpdateLocationAzureBlob,
    UpdateLocationEfs,
    UpdateLocationFsxLustre,
    UpdateLocationFsxOntap,
    UpdateLocationFsxOpenZfs,
    UpdateLocationFsxWindows,
    UpdateLocationHdfs,
    UpdateLocationNfs,
    UpdateLocationObjectStorage,
    UpdateLocationS3,
    UpdateLocationSmb,
    UpdateStorageSystem,
    UpdateTask,
    UpdateTaskExecution,
}
impl std::fmt::Display for DatasyncActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DatasyncActions::AddStorageSystem => write!(f, "datasync:AddStorageSystem"),
            DatasyncActions::CancelTaskExecution => write!(f, "datasync:CancelTaskExecution"),
            DatasyncActions::CreateAgent => write!(f, "datasync:CreateAgent"),
            DatasyncActions::CreateLocationAzureBlob => {
                write!(f, "datasync:CreateLocationAzureBlob")
            }
            DatasyncActions::CreateLocationEfs => write!(f, "datasync:CreateLocationEfs"),
            DatasyncActions::CreateLocationFsxLustre => {
                write!(f, "datasync:CreateLocationFsxLustre")
            }
            DatasyncActions::CreateLocationFsxOntap => write!(f, "datasync:CreateLocationFsxOntap"),
            DatasyncActions::CreateLocationFsxOpenZfs => {
                write!(f, "datasync:CreateLocationFsxOpenZfs")
            }
            DatasyncActions::CreateLocationFsxWindows => {
                write!(f, "datasync:CreateLocationFsxWindows")
            }
            DatasyncActions::CreateLocationHdfs => write!(f, "datasync:CreateLocationHdfs"),
            DatasyncActions::CreateLocationNfs => write!(f, "datasync:CreateLocationNfs"),
            DatasyncActions::CreateLocationObjectStorage => {
                write!(f, "datasync:CreateLocationObjectStorage")
            }
            DatasyncActions::CreateLocationS3 => write!(f, "datasync:CreateLocationS3"),
            DatasyncActions::CreateLocationSmb => write!(f, "datasync:CreateLocationSmb"),
            DatasyncActions::CreateTask => write!(f, "datasync:CreateTask"),
            DatasyncActions::DeleteAgent => write!(f, "datasync:DeleteAgent"),
            DatasyncActions::DeleteLocation => write!(f, "datasync:DeleteLocation"),
            DatasyncActions::DeleteTask => write!(f, "datasync:DeleteTask"),
            DatasyncActions::DescribeAgent => write!(f, "datasync:DescribeAgent"),
            DatasyncActions::DescribeDiscoveryJob => write!(f, "datasync:DescribeDiscoveryJob"),
            DatasyncActions::DescribeLocationAzureBlob => {
                write!(f, "datasync:DescribeLocationAzureBlob")
            }
            DatasyncActions::DescribeLocationEfs => write!(f, "datasync:DescribeLocationEfs"),
            DatasyncActions::DescribeLocationFsxLustre => {
                write!(f, "datasync:DescribeLocationFsxLustre")
            }
            DatasyncActions::DescribeLocationFsxOntap => {
                write!(f, "datasync:DescribeLocationFsxOntap")
            }
            DatasyncActions::DescribeLocationFsxOpenZfs => {
                write!(f, "datasync:DescribeLocationFsxOpenZfs")
            }
            DatasyncActions::DescribeLocationFsxWindows => {
                write!(f, "datasync:DescribeLocationFsxWindows")
            }
            DatasyncActions::DescribeLocationHdfs => write!(f, "datasync:DescribeLocationHdfs"),
            DatasyncActions::DescribeLocationNfs => write!(f, "datasync:DescribeLocationNfs"),
            DatasyncActions::DescribeLocationObjectStorage => {
                write!(f, "datasync:DescribeLocationObjectStorage")
            }
            DatasyncActions::DescribeLocationS3 => write!(f, "datasync:DescribeLocationS3"),
            DatasyncActions::DescribeLocationSmb => write!(f, "datasync:DescribeLocationSmb"),
            DatasyncActions::DescribeStorageSystem => write!(f, "datasync:DescribeStorageSystem"),
            DatasyncActions::DescribeStorageSystemResourceMetrics => {
                write!(f, "datasync:DescribeStorageSystemResourceMetrics")
            }
            DatasyncActions::DescribeStorageSystemResources => {
                write!(f, "datasync:DescribeStorageSystemResources")
            }
            DatasyncActions::DescribeTask => write!(f, "datasync:DescribeTask"),
            DatasyncActions::DescribeTaskExecution => write!(f, "datasync:DescribeTaskExecution"),
            DatasyncActions::GenerateRecommendations => {
                write!(f, "datasync:GenerateRecommendations")
            }
            DatasyncActions::ListAgents => write!(f, "datasync:ListAgents"),
            DatasyncActions::ListDiscoveryJobs => write!(f, "datasync:ListDiscoveryJobs"),
            DatasyncActions::ListLocations => write!(f, "datasync:ListLocations"),
            DatasyncActions::ListStorageSystems => write!(f, "datasync:ListStorageSystems"),
            DatasyncActions::ListTagsForResource => write!(f, "datasync:ListTagsForResource"),
            DatasyncActions::ListTaskExecutions => write!(f, "datasync:ListTaskExecutions"),
            DatasyncActions::ListTasks => write!(f, "datasync:ListTasks"),
            DatasyncActions::RemoveStorageSystem => write!(f, "datasync:RemoveStorageSystem"),
            DatasyncActions::StartDiscoveryJob => write!(f, "datasync:StartDiscoveryJob"),
            DatasyncActions::StartTaskExecution => write!(f, "datasync:StartTaskExecution"),
            DatasyncActions::StopDiscoveryJob => write!(f, "datasync:StopDiscoveryJob"),
            DatasyncActions::TagResource => write!(f, "datasync:TagResource"),
            DatasyncActions::UntagResource => write!(f, "datasync:UntagResource"),
            DatasyncActions::UpdateAgent => write!(f, "datasync:UpdateAgent"),
            DatasyncActions::UpdateDiscoveryJob => write!(f, "datasync:UpdateDiscoveryJob"),
            DatasyncActions::UpdateLocationAzureBlob => {
                write!(f, "datasync:UpdateLocationAzureBlob")
            }
            DatasyncActions::UpdateLocationEfs => write!(f, "datasync:UpdateLocationEfs"),
            DatasyncActions::UpdateLocationFsxLustre => {
                write!(f, "datasync:UpdateLocationFsxLustre")
            }
            DatasyncActions::UpdateLocationFsxOntap => write!(f, "datasync:UpdateLocationFsxOntap"),
            DatasyncActions::UpdateLocationFsxOpenZfs => {
                write!(f, "datasync:UpdateLocationFsxOpenZfs")
            }
            DatasyncActions::UpdateLocationFsxWindows => {
                write!(f, "datasync:UpdateLocationFsxWindows")
            }
            DatasyncActions::UpdateLocationHdfs => write!(f, "datasync:UpdateLocationHdfs"),
            DatasyncActions::UpdateLocationNfs => write!(f, "datasync:UpdateLocationNfs"),
            DatasyncActions::UpdateLocationObjectStorage => {
                write!(f, "datasync:UpdateLocationObjectStorage")
            }
            DatasyncActions::UpdateLocationS3 => write!(f, "datasync:UpdateLocationS3"),
            DatasyncActions::UpdateLocationSmb => write!(f, "datasync:UpdateLocationSmb"),
            DatasyncActions::UpdateStorageSystem => write!(f, "datasync:UpdateStorageSystem"),
            DatasyncActions::UpdateTask => write!(f, "datasync:UpdateTask"),
            DatasyncActions::UpdateTaskExecution => write!(f, "datasync:UpdateTaskExecution"),
        }
    }
}
