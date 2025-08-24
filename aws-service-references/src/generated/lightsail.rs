// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum LightsailActions {
    AllocateStaticIp,
    AttachCertificateToDistribution,
    AttachDisk,
    AttachInstancesToLoadBalancer,
    AttachLoadBalancerTlsCertificate,
    AttachStaticIp,
    CloseInstancePublicPorts,
    CopySnapshot,
    CreateBucket,
    CreateBucketAccessKey,
    CreateCertificate,
    CreateCloudFormationStack,
    CreateContactMethod,
    CreateContainerService,
    CreateContainerServiceDeployment,
    CreateContainerServiceRegistryLogin,
    CreateDisk,
    CreateDiskFromSnapshot,
    CreateDiskSnapshot,
    CreateDistribution,
    CreateDomain,
    CreateDomainEntry,
    CreateGuiSessionAccessDetails,
    CreateInstanceSnapshot,
    CreateInstances,
    CreateInstancesFromSnapshot,
    CreateKeyPair,
    CreateLoadBalancer,
    CreateLoadBalancerTlsCertificate,
    CreateRelationalDatabase,
    CreateRelationalDatabaseFromSnapshot,
    CreateRelationalDatabaseSnapshot,
    DeleteAlarm,
    DeleteAutoSnapshot,
    DeleteBucket,
    DeleteBucketAccessKey,
    DeleteCertificate,
    DeleteContactMethod,
    DeleteContainerImage,
    DeleteContainerService,
    DeleteDisk,
    DeleteDiskSnapshot,
    DeleteDistribution,
    DeleteDomain,
    DeleteDomainEntry,
    DeleteInstance,
    DeleteInstanceSnapshot,
    DeleteKeyPair,
    DeleteKnownHostKeys,
    DeleteLoadBalancer,
    DeleteLoadBalancerTlsCertificate,
    DeleteRelationalDatabase,
    DeleteRelationalDatabaseSnapshot,
    DetachCertificateFromDistribution,
    DetachDisk,
    DetachInstancesFromLoadBalancer,
    DetachStaticIp,
    DisableAddOn,
    DownloadDefaultKeyPair,
    EnableAddOn,
    ExportSnapshot,
    GetActiveNames,
    GetAlarms,
    GetAutoSnapshots,
    GetBlueprints,
    GetBucketAccessKeys,
    GetBucketBundles,
    GetBucketMetricData,
    GetBuckets,
    GetBundles,
    GetCertificates,
    GetCloudFormationStackRecords,
    GetContactMethods,
    GetContainerApiMetadata,
    GetContainerImages,
    GetContainerLog,
    GetContainerServiceDeployments,
    GetContainerServiceMetricData,
    GetContainerServicePowers,
    GetContainerServices,
    GetCostEstimate,
    GetDisk,
    GetDiskSnapshot,
    GetDiskSnapshots,
    GetDisks,
    GetDistributionBundles,
    GetDistributionLatestCacheReset,
    GetDistributionMetricData,
    GetDistributions,
    GetDomain,
    GetDomains,
    GetExportSnapshotRecords,
    GetInstance,
    GetInstanceAccessDetails,
    GetInstanceMetricData,
    GetInstancePortStates,
    GetInstanceSnapshot,
    GetInstanceSnapshots,
    GetInstanceState,
    GetInstances,
    GetKeyPair,
    GetKeyPairs,
    GetLoadBalancer,
    GetLoadBalancerMetricData,
    GetLoadBalancerTlsCertificates,
    GetLoadBalancerTlsPolicies,
    GetLoadBalancers,
    GetOperation,
    GetOperations,
    GetOperationsForResource,
    GetRegions,
    GetRelationalDatabase,
    GetRelationalDatabaseBlueprints,
    GetRelationalDatabaseBundles,
    GetRelationalDatabaseEvents,
    GetRelationalDatabaseLogEvents,
    GetRelationalDatabaseLogStreams,
    GetRelationalDatabaseMasterUserPassword,
    GetRelationalDatabaseMetricData,
    GetRelationalDatabaseParameters,
    GetRelationalDatabaseSnapshot,
    GetRelationalDatabaseSnapshots,
    GetRelationalDatabases,
    GetSetupHistory,
    GetStaticIp,
    GetStaticIps,
    ImportKeyPair,
    IsVpcPeered,
    OpenInstancePublicPorts,
    PeerVpc,
    PutAlarm,
    PutInstancePublicPorts,
    RebootInstance,
    RebootRelationalDatabase,
    RegisterContainerImage,
    ReleaseStaticIp,
    ResetDistributionCache,
    SendContactMethodVerification,
    SetIpAddressType,
    SetResourceAccessForBucket,
    SetupInstanceHttps,
    StartGuiSession,
    StartInstance,
    StartRelationalDatabase,
    StopGuiSession,
    StopInstance,
    StopRelationalDatabase,
    TagResource,
    TestAlarm,
    UnpeerVpc,
    UntagResource,
    UpdateBucket,
    UpdateBucketBundle,
    UpdateContainerService,
    UpdateDistribution,
    UpdateDistributionBundle,
    UpdateDomainEntry,
    UpdateInstanceMetadataOptions,
    UpdateLoadBalancerAttribute,
    UpdateRelationalDatabase,
    UpdateRelationalDatabaseParameters,
}
impl std::fmt::Display for LightsailActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LightsailActions::AllocateStaticIp => write!(f, "lightsail:AllocateStaticIp"),
            LightsailActions::AttachCertificateToDistribution => {
                write!(f, "lightsail:AttachCertificateToDistribution")
            }
            LightsailActions::AttachDisk => write!(f, "lightsail:AttachDisk"),
            LightsailActions::AttachInstancesToLoadBalancer => {
                write!(f, "lightsail:AttachInstancesToLoadBalancer")
            }
            LightsailActions::AttachLoadBalancerTlsCertificate => {
                write!(f, "lightsail:AttachLoadBalancerTlsCertificate")
            }
            LightsailActions::AttachStaticIp => write!(f, "lightsail:AttachStaticIp"),
            LightsailActions::CloseInstancePublicPorts => {
                write!(f, "lightsail:CloseInstancePublicPorts")
            }
            LightsailActions::CopySnapshot => write!(f, "lightsail:CopySnapshot"),
            LightsailActions::CreateBucket => write!(f, "lightsail:CreateBucket"),
            LightsailActions::CreateBucketAccessKey => write!(f, "lightsail:CreateBucketAccessKey"),
            LightsailActions::CreateCertificate => write!(f, "lightsail:CreateCertificate"),
            LightsailActions::CreateCloudFormationStack => {
                write!(f, "lightsail:CreateCloudFormationStack")
            }
            LightsailActions::CreateContactMethod => write!(f, "lightsail:CreateContactMethod"),
            LightsailActions::CreateContainerService => {
                write!(f, "lightsail:CreateContainerService")
            }
            LightsailActions::CreateContainerServiceDeployment => {
                write!(f, "lightsail:CreateContainerServiceDeployment")
            }
            LightsailActions::CreateContainerServiceRegistryLogin => {
                write!(f, "lightsail:CreateContainerServiceRegistryLogin")
            }
            LightsailActions::CreateDisk => write!(f, "lightsail:CreateDisk"),
            LightsailActions::CreateDiskFromSnapshot => {
                write!(f, "lightsail:CreateDiskFromSnapshot")
            }
            LightsailActions::CreateDiskSnapshot => write!(f, "lightsail:CreateDiskSnapshot"),
            LightsailActions::CreateDistribution => write!(f, "lightsail:CreateDistribution"),
            LightsailActions::CreateDomain => write!(f, "lightsail:CreateDomain"),
            LightsailActions::CreateDomainEntry => write!(f, "lightsail:CreateDomainEntry"),
            LightsailActions::CreateGuiSessionAccessDetails => {
                write!(f, "lightsail:CreateGUISessionAccessDetails")
            }
            LightsailActions::CreateInstanceSnapshot => {
                write!(f, "lightsail:CreateInstanceSnapshot")
            }
            LightsailActions::CreateInstances => write!(f, "lightsail:CreateInstances"),
            LightsailActions::CreateInstancesFromSnapshot => {
                write!(f, "lightsail:CreateInstancesFromSnapshot")
            }
            LightsailActions::CreateKeyPair => write!(f, "lightsail:CreateKeyPair"),
            LightsailActions::CreateLoadBalancer => write!(f, "lightsail:CreateLoadBalancer"),
            LightsailActions::CreateLoadBalancerTlsCertificate => {
                write!(f, "lightsail:CreateLoadBalancerTlsCertificate")
            }
            LightsailActions::CreateRelationalDatabase => {
                write!(f, "lightsail:CreateRelationalDatabase")
            }
            LightsailActions::CreateRelationalDatabaseFromSnapshot => {
                write!(f, "lightsail:CreateRelationalDatabaseFromSnapshot")
            }
            LightsailActions::CreateRelationalDatabaseSnapshot => {
                write!(f, "lightsail:CreateRelationalDatabaseSnapshot")
            }
            LightsailActions::DeleteAlarm => write!(f, "lightsail:DeleteAlarm"),
            LightsailActions::DeleteAutoSnapshot => write!(f, "lightsail:DeleteAutoSnapshot"),
            LightsailActions::DeleteBucket => write!(f, "lightsail:DeleteBucket"),
            LightsailActions::DeleteBucketAccessKey => write!(f, "lightsail:DeleteBucketAccessKey"),
            LightsailActions::DeleteCertificate => write!(f, "lightsail:DeleteCertificate"),
            LightsailActions::DeleteContactMethod => write!(f, "lightsail:DeleteContactMethod"),
            LightsailActions::DeleteContainerImage => write!(f, "lightsail:DeleteContainerImage"),
            LightsailActions::DeleteContainerService => {
                write!(f, "lightsail:DeleteContainerService")
            }
            LightsailActions::DeleteDisk => write!(f, "lightsail:DeleteDisk"),
            LightsailActions::DeleteDiskSnapshot => write!(f, "lightsail:DeleteDiskSnapshot"),
            LightsailActions::DeleteDistribution => write!(f, "lightsail:DeleteDistribution"),
            LightsailActions::DeleteDomain => write!(f, "lightsail:DeleteDomain"),
            LightsailActions::DeleteDomainEntry => write!(f, "lightsail:DeleteDomainEntry"),
            LightsailActions::DeleteInstance => write!(f, "lightsail:DeleteInstance"),
            LightsailActions::DeleteInstanceSnapshot => {
                write!(f, "lightsail:DeleteInstanceSnapshot")
            }
            LightsailActions::DeleteKeyPair => write!(f, "lightsail:DeleteKeyPair"),
            LightsailActions::DeleteKnownHostKeys => write!(f, "lightsail:DeleteKnownHostKeys"),
            LightsailActions::DeleteLoadBalancer => write!(f, "lightsail:DeleteLoadBalancer"),
            LightsailActions::DeleteLoadBalancerTlsCertificate => {
                write!(f, "lightsail:DeleteLoadBalancerTlsCertificate")
            }
            LightsailActions::DeleteRelationalDatabase => {
                write!(f, "lightsail:DeleteRelationalDatabase")
            }
            LightsailActions::DeleteRelationalDatabaseSnapshot => {
                write!(f, "lightsail:DeleteRelationalDatabaseSnapshot")
            }
            LightsailActions::DetachCertificateFromDistribution => {
                write!(f, "lightsail:DetachCertificateFromDistribution")
            }
            LightsailActions::DetachDisk => write!(f, "lightsail:DetachDisk"),
            LightsailActions::DetachInstancesFromLoadBalancer => {
                write!(f, "lightsail:DetachInstancesFromLoadBalancer")
            }
            LightsailActions::DetachStaticIp => write!(f, "lightsail:DetachStaticIp"),
            LightsailActions::DisableAddOn => write!(f, "lightsail:DisableAddOn"),
            LightsailActions::DownloadDefaultKeyPair => {
                write!(f, "lightsail:DownloadDefaultKeyPair")
            }
            LightsailActions::EnableAddOn => write!(f, "lightsail:EnableAddOn"),
            LightsailActions::ExportSnapshot => write!(f, "lightsail:ExportSnapshot"),
            LightsailActions::GetActiveNames => write!(f, "lightsail:GetActiveNames"),
            LightsailActions::GetAlarms => write!(f, "lightsail:GetAlarms"),
            LightsailActions::GetAutoSnapshots => write!(f, "lightsail:GetAutoSnapshots"),
            LightsailActions::GetBlueprints => write!(f, "lightsail:GetBlueprints"),
            LightsailActions::GetBucketAccessKeys => write!(f, "lightsail:GetBucketAccessKeys"),
            LightsailActions::GetBucketBundles => write!(f, "lightsail:GetBucketBundles"),
            LightsailActions::GetBucketMetricData => write!(f, "lightsail:GetBucketMetricData"),
            LightsailActions::GetBuckets => write!(f, "lightsail:GetBuckets"),
            LightsailActions::GetBundles => write!(f, "lightsail:GetBundles"),
            LightsailActions::GetCertificates => write!(f, "lightsail:GetCertificates"),
            LightsailActions::GetCloudFormationStackRecords => {
                write!(f, "lightsail:GetCloudFormationStackRecords")
            }
            LightsailActions::GetContactMethods => write!(f, "lightsail:GetContactMethods"),
            LightsailActions::GetContainerApiMetadata => {
                write!(f, "lightsail:GetContainerAPIMetadata")
            }
            LightsailActions::GetContainerImages => write!(f, "lightsail:GetContainerImages"),
            LightsailActions::GetContainerLog => write!(f, "lightsail:GetContainerLog"),
            LightsailActions::GetContainerServiceDeployments => {
                write!(f, "lightsail:GetContainerServiceDeployments")
            }
            LightsailActions::GetContainerServiceMetricData => {
                write!(f, "lightsail:GetContainerServiceMetricData")
            }
            LightsailActions::GetContainerServicePowers => {
                write!(f, "lightsail:GetContainerServicePowers")
            }
            LightsailActions::GetContainerServices => write!(f, "lightsail:GetContainerServices"),
            LightsailActions::GetCostEstimate => write!(f, "lightsail:GetCostEstimate"),
            LightsailActions::GetDisk => write!(f, "lightsail:GetDisk"),
            LightsailActions::GetDiskSnapshot => write!(f, "lightsail:GetDiskSnapshot"),
            LightsailActions::GetDiskSnapshots => write!(f, "lightsail:GetDiskSnapshots"),
            LightsailActions::GetDisks => write!(f, "lightsail:GetDisks"),
            LightsailActions::GetDistributionBundles => {
                write!(f, "lightsail:GetDistributionBundles")
            }
            LightsailActions::GetDistributionLatestCacheReset => {
                write!(f, "lightsail:GetDistributionLatestCacheReset")
            }
            LightsailActions::GetDistributionMetricData => {
                write!(f, "lightsail:GetDistributionMetricData")
            }
            LightsailActions::GetDistributions => write!(f, "lightsail:GetDistributions"),
            LightsailActions::GetDomain => write!(f, "lightsail:GetDomain"),
            LightsailActions::GetDomains => write!(f, "lightsail:GetDomains"),
            LightsailActions::GetExportSnapshotRecords => {
                write!(f, "lightsail:GetExportSnapshotRecords")
            }
            LightsailActions::GetInstance => write!(f, "lightsail:GetInstance"),
            LightsailActions::GetInstanceAccessDetails => {
                write!(f, "lightsail:GetInstanceAccessDetails")
            }
            LightsailActions::GetInstanceMetricData => write!(f, "lightsail:GetInstanceMetricData"),
            LightsailActions::GetInstancePortStates => write!(f, "lightsail:GetInstancePortStates"),
            LightsailActions::GetInstanceSnapshot => write!(f, "lightsail:GetInstanceSnapshot"),
            LightsailActions::GetInstanceSnapshots => write!(f, "lightsail:GetInstanceSnapshots"),
            LightsailActions::GetInstanceState => write!(f, "lightsail:GetInstanceState"),
            LightsailActions::GetInstances => write!(f, "lightsail:GetInstances"),
            LightsailActions::GetKeyPair => write!(f, "lightsail:GetKeyPair"),
            LightsailActions::GetKeyPairs => write!(f, "lightsail:GetKeyPairs"),
            LightsailActions::GetLoadBalancer => write!(f, "lightsail:GetLoadBalancer"),
            LightsailActions::GetLoadBalancerMetricData => {
                write!(f, "lightsail:GetLoadBalancerMetricData")
            }
            LightsailActions::GetLoadBalancerTlsCertificates => {
                write!(f, "lightsail:GetLoadBalancerTlsCertificates")
            }
            LightsailActions::GetLoadBalancerTlsPolicies => {
                write!(f, "lightsail:GetLoadBalancerTlsPolicies")
            }
            LightsailActions::GetLoadBalancers => write!(f, "lightsail:GetLoadBalancers"),
            LightsailActions::GetOperation => write!(f, "lightsail:GetOperation"),
            LightsailActions::GetOperations => write!(f, "lightsail:GetOperations"),
            LightsailActions::GetOperationsForResource => {
                write!(f, "lightsail:GetOperationsForResource")
            }
            LightsailActions::GetRegions => write!(f, "lightsail:GetRegions"),
            LightsailActions::GetRelationalDatabase => write!(f, "lightsail:GetRelationalDatabase"),
            LightsailActions::GetRelationalDatabaseBlueprints => {
                write!(f, "lightsail:GetRelationalDatabaseBlueprints")
            }
            LightsailActions::GetRelationalDatabaseBundles => {
                write!(f, "lightsail:GetRelationalDatabaseBundles")
            }
            LightsailActions::GetRelationalDatabaseEvents => {
                write!(f, "lightsail:GetRelationalDatabaseEvents")
            }
            LightsailActions::GetRelationalDatabaseLogEvents => {
                write!(f, "lightsail:GetRelationalDatabaseLogEvents")
            }
            LightsailActions::GetRelationalDatabaseLogStreams => {
                write!(f, "lightsail:GetRelationalDatabaseLogStreams")
            }
            LightsailActions::GetRelationalDatabaseMasterUserPassword => {
                write!(f, "lightsail:GetRelationalDatabaseMasterUserPassword")
            }
            LightsailActions::GetRelationalDatabaseMetricData => {
                write!(f, "lightsail:GetRelationalDatabaseMetricData")
            }
            LightsailActions::GetRelationalDatabaseParameters => {
                write!(f, "lightsail:GetRelationalDatabaseParameters")
            }
            LightsailActions::GetRelationalDatabaseSnapshot => {
                write!(f, "lightsail:GetRelationalDatabaseSnapshot")
            }
            LightsailActions::GetRelationalDatabaseSnapshots => {
                write!(f, "lightsail:GetRelationalDatabaseSnapshots")
            }
            LightsailActions::GetRelationalDatabases => {
                write!(f, "lightsail:GetRelationalDatabases")
            }
            LightsailActions::GetSetupHistory => write!(f, "lightsail:GetSetupHistory"),
            LightsailActions::GetStaticIp => write!(f, "lightsail:GetStaticIp"),
            LightsailActions::GetStaticIps => write!(f, "lightsail:GetStaticIps"),
            LightsailActions::ImportKeyPair => write!(f, "lightsail:ImportKeyPair"),
            LightsailActions::IsVpcPeered => write!(f, "lightsail:IsVpcPeered"),
            LightsailActions::OpenInstancePublicPorts => {
                write!(f, "lightsail:OpenInstancePublicPorts")
            }
            LightsailActions::PeerVpc => write!(f, "lightsail:PeerVpc"),
            LightsailActions::PutAlarm => write!(f, "lightsail:PutAlarm"),
            LightsailActions::PutInstancePublicPorts => {
                write!(f, "lightsail:PutInstancePublicPorts")
            }
            LightsailActions::RebootInstance => write!(f, "lightsail:RebootInstance"),
            LightsailActions::RebootRelationalDatabase => {
                write!(f, "lightsail:RebootRelationalDatabase")
            }
            LightsailActions::RegisterContainerImage => {
                write!(f, "lightsail:RegisterContainerImage")
            }
            LightsailActions::ReleaseStaticIp => write!(f, "lightsail:ReleaseStaticIp"),
            LightsailActions::ResetDistributionCache => {
                write!(f, "lightsail:ResetDistributionCache")
            }
            LightsailActions::SendContactMethodVerification => {
                write!(f, "lightsail:SendContactMethodVerification")
            }
            LightsailActions::SetIpAddressType => write!(f, "lightsail:SetIpAddressType"),
            LightsailActions::SetResourceAccessForBucket => {
                write!(f, "lightsail:SetResourceAccessForBucket")
            }
            LightsailActions::SetupInstanceHttps => write!(f, "lightsail:SetupInstanceHttps"),
            LightsailActions::StartGuiSession => write!(f, "lightsail:StartGUISession"),
            LightsailActions::StartInstance => write!(f, "lightsail:StartInstance"),
            LightsailActions::StartRelationalDatabase => {
                write!(f, "lightsail:StartRelationalDatabase")
            }
            LightsailActions::StopGuiSession => write!(f, "lightsail:StopGUISession"),
            LightsailActions::StopInstance => write!(f, "lightsail:StopInstance"),
            LightsailActions::StopRelationalDatabase => {
                write!(f, "lightsail:StopRelationalDatabase")
            }
            LightsailActions::TagResource => write!(f, "lightsail:TagResource"),
            LightsailActions::TestAlarm => write!(f, "lightsail:TestAlarm"),
            LightsailActions::UnpeerVpc => write!(f, "lightsail:UnpeerVpc"),
            LightsailActions::UntagResource => write!(f, "lightsail:UntagResource"),
            LightsailActions::UpdateBucket => write!(f, "lightsail:UpdateBucket"),
            LightsailActions::UpdateBucketBundle => write!(f, "lightsail:UpdateBucketBundle"),
            LightsailActions::UpdateContainerService => {
                write!(f, "lightsail:UpdateContainerService")
            }
            LightsailActions::UpdateDistribution => write!(f, "lightsail:UpdateDistribution"),
            LightsailActions::UpdateDistributionBundle => {
                write!(f, "lightsail:UpdateDistributionBundle")
            }
            LightsailActions::UpdateDomainEntry => write!(f, "lightsail:UpdateDomainEntry"),
            LightsailActions::UpdateInstanceMetadataOptions => {
                write!(f, "lightsail:UpdateInstanceMetadataOptions")
            }
            LightsailActions::UpdateLoadBalancerAttribute => {
                write!(f, "lightsail:UpdateLoadBalancerAttribute")
            }
            LightsailActions::UpdateRelationalDatabase => {
                write!(f, "lightsail:UpdateRelationalDatabase")
            }
            LightsailActions::UpdateRelationalDatabaseParameters => {
                write!(f, "lightsail:UpdateRelationalDatabaseParameters")
            }
        }
    }
}
