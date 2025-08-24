// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum ElasticbeanstalkActions {
    AbortEnvironmentUpdate,
    AddTags,
    ApplyEnvironmentManagedAction,
    AssociateEnvironmentOperationsRole,
    CheckDnsAvailability,
    ComposeEnvironments,
    CreateApplication,
    CreateApplicationVersion,
    CreateConfigurationTemplate,
    CreateEnvironment,
    CreatePlatformVersion,
    CreateStorageLocation,
    DeleteApplication,
    DeleteApplicationVersion,
    DeleteConfigurationTemplate,
    DeleteEnvironmentConfiguration,
    DeletePlatformVersion,
    DescribeAccountAttributes,
    DescribeApplicationVersions,
    DescribeApplications,
    DescribeConfigurationOptions,
    DescribeConfigurationSettings,
    DescribeEnvironmentHealth,
    DescribeEnvironmentManagedActionHistory,
    DescribeEnvironmentManagedActions,
    DescribeEnvironmentResources,
    DescribeEnvironments,
    DescribeEvents,
    DescribeInstancesHealth,
    DescribePlatformVersion,
    DisassociateEnvironmentOperationsRole,
    ListAvailableSolutionStacks,
    ListPlatformBranches,
    ListPlatformVersions,
    ListTagsForResource,
    PutInstanceStatistics,
    RebuildEnvironment,
    RemoveTags,
    RequestEnvironmentInfo,
    RestartAppServer,
    RetrieveEnvironmentInfo,
    SwapEnvironmentCnamEs,
    TerminateEnvironment,
    UpdateApplication,
    UpdateApplicationResourceLifecycle,
    UpdateApplicationVersion,
    UpdateConfigurationTemplate,
    UpdateEnvironment,
    UpdateTagsForResource,
    ValidateConfigurationSettings,
}
impl std::fmt::Display for ElasticbeanstalkActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ElasticbeanstalkActions::AbortEnvironmentUpdate => {
                write!(f, "elasticbeanstalk:AbortEnvironmentUpdate")
            }
            ElasticbeanstalkActions::AddTags => write!(f, "elasticbeanstalk:AddTags"),
            ElasticbeanstalkActions::ApplyEnvironmentManagedAction => {
                write!(f, "elasticbeanstalk:ApplyEnvironmentManagedAction")
            }
            ElasticbeanstalkActions::AssociateEnvironmentOperationsRole => {
                write!(f, "elasticbeanstalk:AssociateEnvironmentOperationsRole")
            }
            ElasticbeanstalkActions::CheckDnsAvailability => {
                write!(f, "elasticbeanstalk:CheckDNSAvailability")
            }
            ElasticbeanstalkActions::ComposeEnvironments => {
                write!(f, "elasticbeanstalk:ComposeEnvironments")
            }
            ElasticbeanstalkActions::CreateApplication => {
                write!(f, "elasticbeanstalk:CreateApplication")
            }
            ElasticbeanstalkActions::CreateApplicationVersion => {
                write!(f, "elasticbeanstalk:CreateApplicationVersion")
            }
            ElasticbeanstalkActions::CreateConfigurationTemplate => {
                write!(f, "elasticbeanstalk:CreateConfigurationTemplate")
            }
            ElasticbeanstalkActions::CreateEnvironment => {
                write!(f, "elasticbeanstalk:CreateEnvironment")
            }
            ElasticbeanstalkActions::CreatePlatformVersion => {
                write!(f, "elasticbeanstalk:CreatePlatformVersion")
            }
            ElasticbeanstalkActions::CreateStorageLocation => {
                write!(f, "elasticbeanstalk:CreateStorageLocation")
            }
            ElasticbeanstalkActions::DeleteApplication => {
                write!(f, "elasticbeanstalk:DeleteApplication")
            }
            ElasticbeanstalkActions::DeleteApplicationVersion => {
                write!(f, "elasticbeanstalk:DeleteApplicationVersion")
            }
            ElasticbeanstalkActions::DeleteConfigurationTemplate => {
                write!(f, "elasticbeanstalk:DeleteConfigurationTemplate")
            }
            ElasticbeanstalkActions::DeleteEnvironmentConfiguration => {
                write!(f, "elasticbeanstalk:DeleteEnvironmentConfiguration")
            }
            ElasticbeanstalkActions::DeletePlatformVersion => {
                write!(f, "elasticbeanstalk:DeletePlatformVersion")
            }
            ElasticbeanstalkActions::DescribeAccountAttributes => {
                write!(f, "elasticbeanstalk:DescribeAccountAttributes")
            }
            ElasticbeanstalkActions::DescribeApplicationVersions => {
                write!(f, "elasticbeanstalk:DescribeApplicationVersions")
            }
            ElasticbeanstalkActions::DescribeApplications => {
                write!(f, "elasticbeanstalk:DescribeApplications")
            }
            ElasticbeanstalkActions::DescribeConfigurationOptions => {
                write!(f, "elasticbeanstalk:DescribeConfigurationOptions")
            }
            ElasticbeanstalkActions::DescribeConfigurationSettings => {
                write!(f, "elasticbeanstalk:DescribeConfigurationSettings")
            }
            ElasticbeanstalkActions::DescribeEnvironmentHealth => {
                write!(f, "elasticbeanstalk:DescribeEnvironmentHealth")
            }
            ElasticbeanstalkActions::DescribeEnvironmentManagedActionHistory => write!(
                f,
                "elasticbeanstalk:DescribeEnvironmentManagedActionHistory"
            ),
            ElasticbeanstalkActions::DescribeEnvironmentManagedActions => {
                write!(f, "elasticbeanstalk:DescribeEnvironmentManagedActions")
            }
            ElasticbeanstalkActions::DescribeEnvironmentResources => {
                write!(f, "elasticbeanstalk:DescribeEnvironmentResources")
            }
            ElasticbeanstalkActions::DescribeEnvironments => {
                write!(f, "elasticbeanstalk:DescribeEnvironments")
            }
            ElasticbeanstalkActions::DescribeEvents => write!(f, "elasticbeanstalk:DescribeEvents"),
            ElasticbeanstalkActions::DescribeInstancesHealth => {
                write!(f, "elasticbeanstalk:DescribeInstancesHealth")
            }
            ElasticbeanstalkActions::DescribePlatformVersion => {
                write!(f, "elasticbeanstalk:DescribePlatformVersion")
            }
            ElasticbeanstalkActions::DisassociateEnvironmentOperationsRole => {
                write!(f, "elasticbeanstalk:DisassociateEnvironmentOperationsRole")
            }
            ElasticbeanstalkActions::ListAvailableSolutionStacks => {
                write!(f, "elasticbeanstalk:ListAvailableSolutionStacks")
            }
            ElasticbeanstalkActions::ListPlatformBranches => {
                write!(f, "elasticbeanstalk:ListPlatformBranches")
            }
            ElasticbeanstalkActions::ListPlatformVersions => {
                write!(f, "elasticbeanstalk:ListPlatformVersions")
            }
            ElasticbeanstalkActions::ListTagsForResource => {
                write!(f, "elasticbeanstalk:ListTagsForResource")
            }
            ElasticbeanstalkActions::PutInstanceStatistics => {
                write!(f, "elasticbeanstalk:PutInstanceStatistics")
            }
            ElasticbeanstalkActions::RebuildEnvironment => {
                write!(f, "elasticbeanstalk:RebuildEnvironment")
            }
            ElasticbeanstalkActions::RemoveTags => write!(f, "elasticbeanstalk:RemoveTags"),
            ElasticbeanstalkActions::RequestEnvironmentInfo => {
                write!(f, "elasticbeanstalk:RequestEnvironmentInfo")
            }
            ElasticbeanstalkActions::RestartAppServer => {
                write!(f, "elasticbeanstalk:RestartAppServer")
            }
            ElasticbeanstalkActions::RetrieveEnvironmentInfo => {
                write!(f, "elasticbeanstalk:RetrieveEnvironmentInfo")
            }
            ElasticbeanstalkActions::SwapEnvironmentCnamEs => {
                write!(f, "elasticbeanstalk:SwapEnvironmentCNAMEs")
            }
            ElasticbeanstalkActions::TerminateEnvironment => {
                write!(f, "elasticbeanstalk:TerminateEnvironment")
            }
            ElasticbeanstalkActions::UpdateApplication => {
                write!(f, "elasticbeanstalk:UpdateApplication")
            }
            ElasticbeanstalkActions::UpdateApplicationResourceLifecycle => {
                write!(f, "elasticbeanstalk:UpdateApplicationResourceLifecycle")
            }
            ElasticbeanstalkActions::UpdateApplicationVersion => {
                write!(f, "elasticbeanstalk:UpdateApplicationVersion")
            }
            ElasticbeanstalkActions::UpdateConfigurationTemplate => {
                write!(f, "elasticbeanstalk:UpdateConfigurationTemplate")
            }
            ElasticbeanstalkActions::UpdateEnvironment => {
                write!(f, "elasticbeanstalk:UpdateEnvironment")
            }
            ElasticbeanstalkActions::UpdateTagsForResource => {
                write!(f, "elasticbeanstalk:UpdateTagsForResource")
            }
            ElasticbeanstalkActions::ValidateConfigurationSettings => {
                write!(f, "elasticbeanstalk:ValidateConfigurationSettings")
            }
        }
    }
}
