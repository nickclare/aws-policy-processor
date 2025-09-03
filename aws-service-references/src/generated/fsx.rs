// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum FsxActions {
    AssociateFileGateway,
    AssociateFileSystemAliases,
    BypassSnaplockEnterpriseRetention,
    CancelDataRepositoryTask,
    CopyBackup,
    CopySnapshotAndUpdateVolume,
    CreateAndAttachS3AccessPoint,
    CreateBackup,
    CreateDataRepositoryAssociation,
    CreateDataRepositoryTask,
    CreateFileCache,
    CreateFileSystem,
    CreateFileSystemFromBackup,
    CreateSnapshot,
    CreateStorageVirtualMachine,
    CreateVolume,
    CreateVolumeFromBackup,
    DeleteBackup,
    DeleteDataRepositoryAssociation,
    DeleteFileCache,
    DeleteFileSystem,
    DeleteResourcePolicy,
    DeleteSnapshot,
    DeleteStorageVirtualMachine,
    DeleteVolume,
    DescribeAssociatedFileGateways,
    DescribeBackups,
    DescribeDataRepositoryAssociations,
    DescribeDataRepositoryTasks,
    DescribeFileCaches,
    DescribeFileSystemAliases,
    DescribeFileSystems,
    DescribeS3AccessPointAttachments,
    DescribeSharedVpcConfiguration,
    DescribeSnapshots,
    DescribeStorageVirtualMachines,
    DescribeVolumes,
    DetachAndDeleteS3AccessPoint,
    DisassociateFileGateway,
    DisassociateFileSystemAliases,
    GetResourcePolicy,
    ListTagsForResource,
    ManageBackupPrincipalAssociations,
    PutResourcePolicy,
    ReleaseFileSystemNfsV3Locks,
    RestoreVolumeFromSnapshot,
    StartMisconfiguredStateRecovery,
    TagResource,
    UntagResource,
    UpdateDataRepositoryAssociation,
    UpdateFileCache,
    UpdateFileSystem,
    UpdateSharedVpcConfiguration,
    UpdateSnapshot,
    UpdateStorageVirtualMachine,
    UpdateVolume,
}
impl std::fmt::Display for FsxActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FsxActions::AssociateFileGateway => write!(f, "fsx:AssociateFileGateway"),
            FsxActions::AssociateFileSystemAliases => write!(f, "fsx:AssociateFileSystemAliases"),
            FsxActions::BypassSnaplockEnterpriseRetention => {
                write!(f, "fsx:BypassSnaplockEnterpriseRetention")
            }
            FsxActions::CancelDataRepositoryTask => write!(f, "fsx:CancelDataRepositoryTask"),
            FsxActions::CopyBackup => write!(f, "fsx:CopyBackup"),
            FsxActions::CopySnapshotAndUpdateVolume => write!(f, "fsx:CopySnapshotAndUpdateVolume"),
            FsxActions::CreateAndAttachS3AccessPoint => {
                write!(f, "fsx:CreateAndAttachS3AccessPoint")
            }
            FsxActions::CreateBackup => write!(f, "fsx:CreateBackup"),
            FsxActions::CreateDataRepositoryAssociation => {
                write!(f, "fsx:CreateDataRepositoryAssociation")
            }
            FsxActions::CreateDataRepositoryTask => write!(f, "fsx:CreateDataRepositoryTask"),
            FsxActions::CreateFileCache => write!(f, "fsx:CreateFileCache"),
            FsxActions::CreateFileSystem => write!(f, "fsx:CreateFileSystem"),
            FsxActions::CreateFileSystemFromBackup => write!(f, "fsx:CreateFileSystemFromBackup"),
            FsxActions::CreateSnapshot => write!(f, "fsx:CreateSnapshot"),
            FsxActions::CreateStorageVirtualMachine => write!(f, "fsx:CreateStorageVirtualMachine"),
            FsxActions::CreateVolume => write!(f, "fsx:CreateVolume"),
            FsxActions::CreateVolumeFromBackup => write!(f, "fsx:CreateVolumeFromBackup"),
            FsxActions::DeleteBackup => write!(f, "fsx:DeleteBackup"),
            FsxActions::DeleteDataRepositoryAssociation => {
                write!(f, "fsx:DeleteDataRepositoryAssociation")
            }
            FsxActions::DeleteFileCache => write!(f, "fsx:DeleteFileCache"),
            FsxActions::DeleteFileSystem => write!(f, "fsx:DeleteFileSystem"),
            FsxActions::DeleteResourcePolicy => write!(f, "fsx:DeleteResourcePolicy"),
            FsxActions::DeleteSnapshot => write!(f, "fsx:DeleteSnapshot"),
            FsxActions::DeleteStorageVirtualMachine => write!(f, "fsx:DeleteStorageVirtualMachine"),
            FsxActions::DeleteVolume => write!(f, "fsx:DeleteVolume"),
            FsxActions::DescribeAssociatedFileGateways => {
                write!(f, "fsx:DescribeAssociatedFileGateways")
            }
            FsxActions::DescribeBackups => write!(f, "fsx:DescribeBackups"),
            FsxActions::DescribeDataRepositoryAssociations => {
                write!(f, "fsx:DescribeDataRepositoryAssociations")
            }
            FsxActions::DescribeDataRepositoryTasks => write!(f, "fsx:DescribeDataRepositoryTasks"),
            FsxActions::DescribeFileCaches => write!(f, "fsx:DescribeFileCaches"),
            FsxActions::DescribeFileSystemAliases => write!(f, "fsx:DescribeFileSystemAliases"),
            FsxActions::DescribeFileSystems => write!(f, "fsx:DescribeFileSystems"),
            FsxActions::DescribeS3AccessPointAttachments => {
                write!(f, "fsx:DescribeS3AccessPointAttachments")
            }
            FsxActions::DescribeSharedVpcConfiguration => {
                write!(f, "fsx:DescribeSharedVpcConfiguration")
            }
            FsxActions::DescribeSnapshots => write!(f, "fsx:DescribeSnapshots"),
            FsxActions::DescribeStorageVirtualMachines => {
                write!(f, "fsx:DescribeStorageVirtualMachines")
            }
            FsxActions::DescribeVolumes => write!(f, "fsx:DescribeVolumes"),
            FsxActions::DetachAndDeleteS3AccessPoint => {
                write!(f, "fsx:DetachAndDeleteS3AccessPoint")
            }
            FsxActions::DisassociateFileGateway => write!(f, "fsx:DisassociateFileGateway"),
            FsxActions::DisassociateFileSystemAliases => {
                write!(f, "fsx:DisassociateFileSystemAliases")
            }
            FsxActions::GetResourcePolicy => write!(f, "fsx:GetResourcePolicy"),
            FsxActions::ListTagsForResource => write!(f, "fsx:ListTagsForResource"),
            FsxActions::ManageBackupPrincipalAssociations => {
                write!(f, "fsx:ManageBackupPrincipalAssociations")
            }
            FsxActions::PutResourcePolicy => write!(f, "fsx:PutResourcePolicy"),
            FsxActions::ReleaseFileSystemNfsV3Locks => write!(f, "fsx:ReleaseFileSystemNfsV3Locks"),
            FsxActions::RestoreVolumeFromSnapshot => write!(f, "fsx:RestoreVolumeFromSnapshot"),
            FsxActions::StartMisconfiguredStateRecovery => {
                write!(f, "fsx:StartMisconfiguredStateRecovery")
            }
            FsxActions::TagResource => write!(f, "fsx:TagResource"),
            FsxActions::UntagResource => write!(f, "fsx:UntagResource"),
            FsxActions::UpdateDataRepositoryAssociation => {
                write!(f, "fsx:UpdateDataRepositoryAssociation")
            }
            FsxActions::UpdateFileCache => write!(f, "fsx:UpdateFileCache"),
            FsxActions::UpdateFileSystem => write!(f, "fsx:UpdateFileSystem"),
            FsxActions::UpdateSharedVpcConfiguration => {
                write!(f, "fsx:UpdateSharedVpcConfiguration")
            }
            FsxActions::UpdateSnapshot => write!(f, "fsx:UpdateSnapshot"),
            FsxActions::UpdateStorageVirtualMachine => write!(f, "fsx:UpdateStorageVirtualMachine"),
            FsxActions::UpdateVolume => write!(f, "fsx:UpdateVolume"),
        }
    }
}
