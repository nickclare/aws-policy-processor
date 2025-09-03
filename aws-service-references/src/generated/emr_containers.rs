// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum EmrContainersActions {
    CancelJobRun,
    CreateCertificate,
    CreateJobTemplate,
    CreateManagedEndpoint,
    CreateSecurityConfiguration,
    CreateVirtualCluster,
    DeleteJobTemplate,
    DeleteManagedEndpoint,
    DeleteVirtualCluster,
    DescribeJobRun,
    DescribeJobTemplate,
    DescribeManagedEndpoint,
    DescribeSecurityConfiguration,
    DescribeVirtualCluster,
    GetManagedEndpointSessionCredentials,
    ListJobRuns,
    ListJobTemplates,
    ListManagedEndpoints,
    ListSecurityConfigurations,
    ListTagsForResource,
    ListVirtualClusters,
    StartJobRun,
    TagResource,
    UntagResource,
}
impl std::fmt::Display for EmrContainersActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EmrContainersActions::CancelJobRun => write!(f, "emr-containers:CancelJobRun"),
            EmrContainersActions::CreateCertificate => {
                write!(f, "emr-containers:CreateCertificate")
            }
            EmrContainersActions::CreateJobTemplate => {
                write!(f, "emr-containers:CreateJobTemplate")
            }
            EmrContainersActions::CreateManagedEndpoint => {
                write!(f, "emr-containers:CreateManagedEndpoint")
            }
            EmrContainersActions::CreateSecurityConfiguration => {
                write!(f, "emr-containers:CreateSecurityConfiguration")
            }
            EmrContainersActions::CreateVirtualCluster => {
                write!(f, "emr-containers:CreateVirtualCluster")
            }
            EmrContainersActions::DeleteJobTemplate => {
                write!(f, "emr-containers:DeleteJobTemplate")
            }
            EmrContainersActions::DeleteManagedEndpoint => {
                write!(f, "emr-containers:DeleteManagedEndpoint")
            }
            EmrContainersActions::DeleteVirtualCluster => {
                write!(f, "emr-containers:DeleteVirtualCluster")
            }
            EmrContainersActions::DescribeJobRun => write!(f, "emr-containers:DescribeJobRun"),
            EmrContainersActions::DescribeJobTemplate => {
                write!(f, "emr-containers:DescribeJobTemplate")
            }
            EmrContainersActions::DescribeManagedEndpoint => {
                write!(f, "emr-containers:DescribeManagedEndpoint")
            }
            EmrContainersActions::DescribeSecurityConfiguration => {
                write!(f, "emr-containers:DescribeSecurityConfiguration")
            }
            EmrContainersActions::DescribeVirtualCluster => {
                write!(f, "emr-containers:DescribeVirtualCluster")
            }
            EmrContainersActions::GetManagedEndpointSessionCredentials => {
                write!(f, "emr-containers:GetManagedEndpointSessionCredentials")
            }
            EmrContainersActions::ListJobRuns => write!(f, "emr-containers:ListJobRuns"),
            EmrContainersActions::ListJobTemplates => write!(f, "emr-containers:ListJobTemplates"),
            EmrContainersActions::ListManagedEndpoints => {
                write!(f, "emr-containers:ListManagedEndpoints")
            }
            EmrContainersActions::ListSecurityConfigurations => {
                write!(f, "emr-containers:ListSecurityConfigurations")
            }
            EmrContainersActions::ListTagsForResource => {
                write!(f, "emr-containers:ListTagsForResource")
            }
            EmrContainersActions::ListVirtualClusters => {
                write!(f, "emr-containers:ListVirtualClusters")
            }
            EmrContainersActions::StartJobRun => write!(f, "emr-containers:StartJobRun"),
            EmrContainersActions::TagResource => write!(f, "emr-containers:TagResource"),
            EmrContainersActions::UntagResource => write!(f, "emr-containers:UntagResource"),
        }
    }
}
