// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum StoragegatewayActions {
    ActivateGateway,
    AddCache,
    AddTagsToResource,
    AddUploadBuffer,
    AddWorkingStorage,
    AssignTapePool,
    AssociateFileSystem,
    AttachVolume,
    BypassGovernanceRetention,
    CancelArchival,
    CancelCacheReport,
    CancelRetrieval,
    CreateCachediScsiVolume,
    CreateNfsFileShare,
    CreateSmbFileShare,
    CreateSnapshot,
    CreateSnapshotFromVolumeRecoveryPoint,
    CreateStorediScsiVolume,
    CreateTapePool,
    CreateTapeWithBarcode,
    CreateTapes,
    DeleteAutomaticTapeCreationPolicy,
    DeleteBandwidthRateLimit,
    DeleteCacheReport,
    DeleteChapCredentials,
    DeleteFileShare,
    DeleteGateway,
    DeleteSnapshotSchedule,
    DeleteTape,
    DeleteTapeArchive,
    DeleteTapePool,
    DeleteVolume,
    DescribeAvailabilityMonitorTest,
    DescribeBandwidthRateLimit,
    DescribeBandwidthRateLimitSchedule,
    DescribeCache,
    DescribeCacheReport,
    DescribeCachediScsiVolumes,
    DescribeChapCredentials,
    DescribeFileSystemAssociations,
    DescribeGatewayInformation,
    DescribeMaintenanceStartTime,
    DescribeNfsFileShares,
    DescribeSmbFileShares,
    DescribeSmbSettings,
    DescribeSnapshotSchedule,
    DescribeStorediScsiVolumes,
    DescribeTapeArchives,
    DescribeTapeRecoveryPoints,
    DescribeTapes,
    DescribeUploadBuffer,
    DescribeVtlDevices,
    DescribeWorkingStorage,
    DetachVolume,
    DisableGateway,
    DisassociateFileSystem,
    EvictFilesFailingUpload,
    JoinDomain,
    ListAutomaticTapeCreationPolicies,
    ListCacheReports,
    ListFileShares,
    ListFileSystemAssociations,
    ListGateways,
    ListLocalDisks,
    ListTagsForResource,
    ListTapePools,
    ListTapes,
    ListVolumeInitiators,
    ListVolumeRecoveryPoints,
    ListVolumes,
    NotifyWhenUploaded,
    RefreshCache,
    RemoveTagsFromResource,
    ResetCache,
    RetrieveTapeArchive,
    RetrieveTapeRecoveryPoint,
    SetLocalConsolePassword,
    SetSmbGuestPassword,
    ShutdownGateway,
    StartAvailabilityMonitorTest,
    StartCacheReport,
    StartGateway,
    UpdateAutomaticTapeCreationPolicy,
    UpdateBandwidthRateLimit,
    UpdateBandwidthRateLimitSchedule,
    UpdateChapCredentials,
    UpdateFileSystemAssociation,
    UpdateGatewayInformation,
    UpdateGatewaySoftwareNow,
    UpdateMaintenanceStartTime,
    UpdateNfsFileShare,
    UpdateSmbFileShare,
    UpdateSmbFileShareVisibility,
    UpdateSmbLocalGroups,
    UpdateSmbSecurityStrategy,
    UpdateSnapshotSchedule,
    UpdateVtlDeviceType,
}
impl std::fmt::Display for StoragegatewayActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StoragegatewayActions::ActivateGateway => write!(f, "storagegateway:ActivateGateway"),
            StoragegatewayActions::AddCache => write!(f, "storagegateway:AddCache"),
            StoragegatewayActions::AddTagsToResource => {
                write!(f, "storagegateway:AddTagsToResource")
            }
            StoragegatewayActions::AddUploadBuffer => write!(f, "storagegateway:AddUploadBuffer"),
            StoragegatewayActions::AddWorkingStorage => {
                write!(f, "storagegateway:AddWorkingStorage")
            }
            StoragegatewayActions::AssignTapePool => write!(f, "storagegateway:AssignTapePool"),
            StoragegatewayActions::AssociateFileSystem => {
                write!(f, "storagegateway:AssociateFileSystem")
            }
            StoragegatewayActions::AttachVolume => write!(f, "storagegateway:AttachVolume"),
            StoragegatewayActions::BypassGovernanceRetention => {
                write!(f, "storagegateway:BypassGovernanceRetention")
            }
            StoragegatewayActions::CancelArchival => write!(f, "storagegateway:CancelArchival"),
            StoragegatewayActions::CancelCacheReport => {
                write!(f, "storagegateway:CancelCacheReport")
            }
            StoragegatewayActions::CancelRetrieval => write!(f, "storagegateway:CancelRetrieval"),
            StoragegatewayActions::CreateCachediScsiVolume => {
                write!(f, "storagegateway:CreateCachediSCSIVolume")
            }
            StoragegatewayActions::CreateNfsFileShare => {
                write!(f, "storagegateway:CreateNFSFileShare")
            }
            StoragegatewayActions::CreateSmbFileShare => {
                write!(f, "storagegateway:CreateSMBFileShare")
            }
            StoragegatewayActions::CreateSnapshot => write!(f, "storagegateway:CreateSnapshot"),
            StoragegatewayActions::CreateSnapshotFromVolumeRecoveryPoint => {
                write!(f, "storagegateway:CreateSnapshotFromVolumeRecoveryPoint")
            }
            StoragegatewayActions::CreateStorediScsiVolume => {
                write!(f, "storagegateway:CreateStorediSCSIVolume")
            }
            StoragegatewayActions::CreateTapePool => write!(f, "storagegateway:CreateTapePool"),
            StoragegatewayActions::CreateTapeWithBarcode => {
                write!(f, "storagegateway:CreateTapeWithBarcode")
            }
            StoragegatewayActions::CreateTapes => write!(f, "storagegateway:CreateTapes"),
            StoragegatewayActions::DeleteAutomaticTapeCreationPolicy => {
                write!(f, "storagegateway:DeleteAutomaticTapeCreationPolicy")
            }
            StoragegatewayActions::DeleteBandwidthRateLimit => {
                write!(f, "storagegateway:DeleteBandwidthRateLimit")
            }
            StoragegatewayActions::DeleteCacheReport => {
                write!(f, "storagegateway:DeleteCacheReport")
            }
            StoragegatewayActions::DeleteChapCredentials => {
                write!(f, "storagegateway:DeleteChapCredentials")
            }
            StoragegatewayActions::DeleteFileShare => write!(f, "storagegateway:DeleteFileShare"),
            StoragegatewayActions::DeleteGateway => write!(f, "storagegateway:DeleteGateway"),
            StoragegatewayActions::DeleteSnapshotSchedule => {
                write!(f, "storagegateway:DeleteSnapshotSchedule")
            }
            StoragegatewayActions::DeleteTape => write!(f, "storagegateway:DeleteTape"),
            StoragegatewayActions::DeleteTapeArchive => {
                write!(f, "storagegateway:DeleteTapeArchive")
            }
            StoragegatewayActions::DeleteTapePool => write!(f, "storagegateway:DeleteTapePool"),
            StoragegatewayActions::DeleteVolume => write!(f, "storagegateway:DeleteVolume"),
            StoragegatewayActions::DescribeAvailabilityMonitorTest => {
                write!(f, "storagegateway:DescribeAvailabilityMonitorTest")
            }
            StoragegatewayActions::DescribeBandwidthRateLimit => {
                write!(f, "storagegateway:DescribeBandwidthRateLimit")
            }
            StoragegatewayActions::DescribeBandwidthRateLimitSchedule => {
                write!(f, "storagegateway:DescribeBandwidthRateLimitSchedule")
            }
            StoragegatewayActions::DescribeCache => write!(f, "storagegateway:DescribeCache"),
            StoragegatewayActions::DescribeCacheReport => {
                write!(f, "storagegateway:DescribeCacheReport")
            }
            StoragegatewayActions::DescribeCachediScsiVolumes => {
                write!(f, "storagegateway:DescribeCachediSCSIVolumes")
            }
            StoragegatewayActions::DescribeChapCredentials => {
                write!(f, "storagegateway:DescribeChapCredentials")
            }
            StoragegatewayActions::DescribeFileSystemAssociations => {
                write!(f, "storagegateway:DescribeFileSystemAssociations")
            }
            StoragegatewayActions::DescribeGatewayInformation => {
                write!(f, "storagegateway:DescribeGatewayInformation")
            }
            StoragegatewayActions::DescribeMaintenanceStartTime => {
                write!(f, "storagegateway:DescribeMaintenanceStartTime")
            }
            StoragegatewayActions::DescribeNfsFileShares => {
                write!(f, "storagegateway:DescribeNFSFileShares")
            }
            StoragegatewayActions::DescribeSmbFileShares => {
                write!(f, "storagegateway:DescribeSMBFileShares")
            }
            StoragegatewayActions::DescribeSmbSettings => {
                write!(f, "storagegateway:DescribeSMBSettings")
            }
            StoragegatewayActions::DescribeSnapshotSchedule => {
                write!(f, "storagegateway:DescribeSnapshotSchedule")
            }
            StoragegatewayActions::DescribeStorediScsiVolumes => {
                write!(f, "storagegateway:DescribeStorediSCSIVolumes")
            }
            StoragegatewayActions::DescribeTapeArchives => {
                write!(f, "storagegateway:DescribeTapeArchives")
            }
            StoragegatewayActions::DescribeTapeRecoveryPoints => {
                write!(f, "storagegateway:DescribeTapeRecoveryPoints")
            }
            StoragegatewayActions::DescribeTapes => write!(f, "storagegateway:DescribeTapes"),
            StoragegatewayActions::DescribeUploadBuffer => {
                write!(f, "storagegateway:DescribeUploadBuffer")
            }
            StoragegatewayActions::DescribeVtlDevices => {
                write!(f, "storagegateway:DescribeVTLDevices")
            }
            StoragegatewayActions::DescribeWorkingStorage => {
                write!(f, "storagegateway:DescribeWorkingStorage")
            }
            StoragegatewayActions::DetachVolume => write!(f, "storagegateway:DetachVolume"),
            StoragegatewayActions::DisableGateway => write!(f, "storagegateway:DisableGateway"),
            StoragegatewayActions::DisassociateFileSystem => {
                write!(f, "storagegateway:DisassociateFileSystem")
            }
            StoragegatewayActions::EvictFilesFailingUpload => {
                write!(f, "storagegateway:EvictFilesFailingUpload")
            }
            StoragegatewayActions::JoinDomain => write!(f, "storagegateway:JoinDomain"),
            StoragegatewayActions::ListAutomaticTapeCreationPolicies => {
                write!(f, "storagegateway:ListAutomaticTapeCreationPolicies")
            }
            StoragegatewayActions::ListCacheReports => write!(f, "storagegateway:ListCacheReports"),
            StoragegatewayActions::ListFileShares => write!(f, "storagegateway:ListFileShares"),
            StoragegatewayActions::ListFileSystemAssociations => {
                write!(f, "storagegateway:ListFileSystemAssociations")
            }
            StoragegatewayActions::ListGateways => write!(f, "storagegateway:ListGateways"),
            StoragegatewayActions::ListLocalDisks => write!(f, "storagegateway:ListLocalDisks"),
            StoragegatewayActions::ListTagsForResource => {
                write!(f, "storagegateway:ListTagsForResource")
            }
            StoragegatewayActions::ListTapePools => write!(f, "storagegateway:ListTapePools"),
            StoragegatewayActions::ListTapes => write!(f, "storagegateway:ListTapes"),
            StoragegatewayActions::ListVolumeInitiators => {
                write!(f, "storagegateway:ListVolumeInitiators")
            }
            StoragegatewayActions::ListVolumeRecoveryPoints => {
                write!(f, "storagegateway:ListVolumeRecoveryPoints")
            }
            StoragegatewayActions::ListVolumes => write!(f, "storagegateway:ListVolumes"),
            StoragegatewayActions::NotifyWhenUploaded => {
                write!(f, "storagegateway:NotifyWhenUploaded")
            }
            StoragegatewayActions::RefreshCache => write!(f, "storagegateway:RefreshCache"),
            StoragegatewayActions::RemoveTagsFromResource => {
                write!(f, "storagegateway:RemoveTagsFromResource")
            }
            StoragegatewayActions::ResetCache => write!(f, "storagegateway:ResetCache"),
            StoragegatewayActions::RetrieveTapeArchive => {
                write!(f, "storagegateway:RetrieveTapeArchive")
            }
            StoragegatewayActions::RetrieveTapeRecoveryPoint => {
                write!(f, "storagegateway:RetrieveTapeRecoveryPoint")
            }
            StoragegatewayActions::SetLocalConsolePassword => {
                write!(f, "storagegateway:SetLocalConsolePassword")
            }
            StoragegatewayActions::SetSmbGuestPassword => {
                write!(f, "storagegateway:SetSMBGuestPassword")
            }
            StoragegatewayActions::ShutdownGateway => write!(f, "storagegateway:ShutdownGateway"),
            StoragegatewayActions::StartAvailabilityMonitorTest => {
                write!(f, "storagegateway:StartAvailabilityMonitorTest")
            }
            StoragegatewayActions::StartCacheReport => write!(f, "storagegateway:StartCacheReport"),
            StoragegatewayActions::StartGateway => write!(f, "storagegateway:StartGateway"),
            StoragegatewayActions::UpdateAutomaticTapeCreationPolicy => {
                write!(f, "storagegateway:UpdateAutomaticTapeCreationPolicy")
            }
            StoragegatewayActions::UpdateBandwidthRateLimit => {
                write!(f, "storagegateway:UpdateBandwidthRateLimit")
            }
            StoragegatewayActions::UpdateBandwidthRateLimitSchedule => {
                write!(f, "storagegateway:UpdateBandwidthRateLimitSchedule")
            }
            StoragegatewayActions::UpdateChapCredentials => {
                write!(f, "storagegateway:UpdateChapCredentials")
            }
            StoragegatewayActions::UpdateFileSystemAssociation => {
                write!(f, "storagegateway:UpdateFileSystemAssociation")
            }
            StoragegatewayActions::UpdateGatewayInformation => {
                write!(f, "storagegateway:UpdateGatewayInformation")
            }
            StoragegatewayActions::UpdateGatewaySoftwareNow => {
                write!(f, "storagegateway:UpdateGatewaySoftwareNow")
            }
            StoragegatewayActions::UpdateMaintenanceStartTime => {
                write!(f, "storagegateway:UpdateMaintenanceStartTime")
            }
            StoragegatewayActions::UpdateNfsFileShare => {
                write!(f, "storagegateway:UpdateNFSFileShare")
            }
            StoragegatewayActions::UpdateSmbFileShare => {
                write!(f, "storagegateway:UpdateSMBFileShare")
            }
            StoragegatewayActions::UpdateSmbFileShareVisibility => {
                write!(f, "storagegateway:UpdateSMBFileShareVisibility")
            }
            StoragegatewayActions::UpdateSmbLocalGroups => {
                write!(f, "storagegateway:UpdateSMBLocalGroups")
            }
            StoragegatewayActions::UpdateSmbSecurityStrategy => {
                write!(f, "storagegateway:UpdateSMBSecurityStrategy")
            }
            StoragegatewayActions::UpdateSnapshotSchedule => {
                write!(f, "storagegateway:UpdateSnapshotSchedule")
            }
            StoragegatewayActions::UpdateVtlDeviceType => {
                write!(f, "storagegateway:UpdateVTLDeviceType")
            }
        }
    }
}
