// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum PanoramaActions {
    CreateApplicationInstance,
    CreateJobForDevices,
    CreateNodeFromTemplateJob,
    CreatePackage,
    CreatePackageImportJob,
    DeleteDevice,
    DeletePackage,
    DeregisterPackageVersion,
    DescribeApplicationInstance,
    DescribeApplicationInstanceDetails,
    DescribeDevice,
    DescribeDeviceJob,
    DescribeNode,
    DescribeNodeFromTemplateJob,
    DescribePackage,
    DescribePackageImportJob,
    DescribePackageVersion,
    DescribeSoftware,
    GetWebSocketUrl,
    ListApplicationInstanceDependencies,
    ListApplicationInstanceNodeInstances,
    ListApplicationInstances,
    ListDevices,
    ListDevicesJobs,
    ListNodeFromTemplateJobs,
    ListNodes,
    ListPackageImportJobs,
    ListPackages,
    ListTagsForResource,
    ProvisionDevice,
    RegisterPackageVersion,
    RemoveApplicationInstance,
    SignalApplicationInstanceNodeInstances,
    TagResource,
    UntagResource,
    UpdateDeviceMetadata,
}
impl std::fmt::Display for PanoramaActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PanoramaActions::CreateApplicationInstance => {
                write!(f, "panorama:CreateApplicationInstance")
            }
            PanoramaActions::CreateJobForDevices => write!(f, "panorama:CreateJobForDevices"),
            PanoramaActions::CreateNodeFromTemplateJob => {
                write!(f, "panorama:CreateNodeFromTemplateJob")
            }
            PanoramaActions::CreatePackage => write!(f, "panorama:CreatePackage"),
            PanoramaActions::CreatePackageImportJob => write!(f, "panorama:CreatePackageImportJob"),
            PanoramaActions::DeleteDevice => write!(f, "panorama:DeleteDevice"),
            PanoramaActions::DeletePackage => write!(f, "panorama:DeletePackage"),
            PanoramaActions::DeregisterPackageVersion => {
                write!(f, "panorama:DeregisterPackageVersion")
            }
            PanoramaActions::DescribeApplicationInstance => {
                write!(f, "panorama:DescribeApplicationInstance")
            }
            PanoramaActions::DescribeApplicationInstanceDetails => {
                write!(f, "panorama:DescribeApplicationInstanceDetails")
            }
            PanoramaActions::DescribeDevice => write!(f, "panorama:DescribeDevice"),
            PanoramaActions::DescribeDeviceJob => write!(f, "panorama:DescribeDeviceJob"),
            PanoramaActions::DescribeNode => write!(f, "panorama:DescribeNode"),
            PanoramaActions::DescribeNodeFromTemplateJob => {
                write!(f, "panorama:DescribeNodeFromTemplateJob")
            }
            PanoramaActions::DescribePackage => write!(f, "panorama:DescribePackage"),
            PanoramaActions::DescribePackageImportJob => {
                write!(f, "panorama:DescribePackageImportJob")
            }
            PanoramaActions::DescribePackageVersion => write!(f, "panorama:DescribePackageVersion"),
            PanoramaActions::DescribeSoftware => write!(f, "panorama:DescribeSoftware"),
            PanoramaActions::GetWebSocketUrl => write!(f, "panorama:GetWebSocketURL"),
            PanoramaActions::ListApplicationInstanceDependencies => {
                write!(f, "panorama:ListApplicationInstanceDependencies")
            }
            PanoramaActions::ListApplicationInstanceNodeInstances => {
                write!(f, "panorama:ListApplicationInstanceNodeInstances")
            }
            PanoramaActions::ListApplicationInstances => {
                write!(f, "panorama:ListApplicationInstances")
            }
            PanoramaActions::ListDevices => write!(f, "panorama:ListDevices"),
            PanoramaActions::ListDevicesJobs => write!(f, "panorama:ListDevicesJobs"),
            PanoramaActions::ListNodeFromTemplateJobs => {
                write!(f, "panorama:ListNodeFromTemplateJobs")
            }
            PanoramaActions::ListNodes => write!(f, "panorama:ListNodes"),
            PanoramaActions::ListPackageImportJobs => write!(f, "panorama:ListPackageImportJobs"),
            PanoramaActions::ListPackages => write!(f, "panorama:ListPackages"),
            PanoramaActions::ListTagsForResource => write!(f, "panorama:ListTagsForResource"),
            PanoramaActions::ProvisionDevice => write!(f, "panorama:ProvisionDevice"),
            PanoramaActions::RegisterPackageVersion => write!(f, "panorama:RegisterPackageVersion"),
            PanoramaActions::RemoveApplicationInstance => {
                write!(f, "panorama:RemoveApplicationInstance")
            }
            PanoramaActions::SignalApplicationInstanceNodeInstances => {
                write!(f, "panorama:SignalApplicationInstanceNodeInstances")
            }
            PanoramaActions::TagResource => write!(f, "panorama:TagResource"),
            PanoramaActions::UntagResource => write!(f, "panorama:UntagResource"),
            PanoramaActions::UpdateDeviceMetadata => write!(f, "panorama:UpdateDeviceMetadata"),
        }
    }
}
