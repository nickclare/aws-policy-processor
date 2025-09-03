// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum SnowballActions {
    CancelCluster,
    CancelJob,
    CreateAddress,
    CreateCluster,
    CreateJob,
    CreateLongTermPricing,
    CreateReturnShippingLabel,
    DescribeAddress,
    DescribeAddresses,
    DescribeCluster,
    DescribeJob,
    DescribeReturnShippingLabel,
    GetJobManifest,
    GetJobUnlockCode,
    GetSnowballUsage,
    GetSoftwareUpdates,
    ListClusterJobs,
    ListClusters,
    ListCompatibleImages,
    ListJobs,
    ListLongTermPricing,
    ListPickupLocations,
    ListServiceVersions,
    UpdateCluster,
    UpdateJob,
    UpdateJobShipmentState,
    UpdateLongTermPricing,
}
impl std::fmt::Display for SnowballActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SnowballActions::CancelCluster => write!(f, "snowball:CancelCluster"),
            SnowballActions::CancelJob => write!(f, "snowball:CancelJob"),
            SnowballActions::CreateAddress => write!(f, "snowball:CreateAddress"),
            SnowballActions::CreateCluster => write!(f, "snowball:CreateCluster"),
            SnowballActions::CreateJob => write!(f, "snowball:CreateJob"),
            SnowballActions::CreateLongTermPricing => write!(f, "snowball:CreateLongTermPricing"),
            SnowballActions::CreateReturnShippingLabel => {
                write!(f, "snowball:CreateReturnShippingLabel")
            }
            SnowballActions::DescribeAddress => write!(f, "snowball:DescribeAddress"),
            SnowballActions::DescribeAddresses => write!(f, "snowball:DescribeAddresses"),
            SnowballActions::DescribeCluster => write!(f, "snowball:DescribeCluster"),
            SnowballActions::DescribeJob => write!(f, "snowball:DescribeJob"),
            SnowballActions::DescribeReturnShippingLabel => {
                write!(f, "snowball:DescribeReturnShippingLabel")
            }
            SnowballActions::GetJobManifest => write!(f, "snowball:GetJobManifest"),
            SnowballActions::GetJobUnlockCode => write!(f, "snowball:GetJobUnlockCode"),
            SnowballActions::GetSnowballUsage => write!(f, "snowball:GetSnowballUsage"),
            SnowballActions::GetSoftwareUpdates => write!(f, "snowball:GetSoftwareUpdates"),
            SnowballActions::ListClusterJobs => write!(f, "snowball:ListClusterJobs"),
            SnowballActions::ListClusters => write!(f, "snowball:ListClusters"),
            SnowballActions::ListCompatibleImages => write!(f, "snowball:ListCompatibleImages"),
            SnowballActions::ListJobs => write!(f, "snowball:ListJobs"),
            SnowballActions::ListLongTermPricing => write!(f, "snowball:ListLongTermPricing"),
            SnowballActions::ListPickupLocations => write!(f, "snowball:ListPickupLocations"),
            SnowballActions::ListServiceVersions => write!(f, "snowball:ListServiceVersions"),
            SnowballActions::UpdateCluster => write!(f, "snowball:UpdateCluster"),
            SnowballActions::UpdateJob => write!(f, "snowball:UpdateJob"),
            SnowballActions::UpdateJobShipmentState => write!(f, "snowball:UpdateJobShipmentState"),
            SnowballActions::UpdateLongTermPricing => write!(f, "snowball:UpdateLongTermPricing"),
        }
    }
}
