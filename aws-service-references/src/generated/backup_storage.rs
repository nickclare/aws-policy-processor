// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum BackupStorageActions {
    CommitBackupJob,
    DeleteObjects,
    DescribeBackupJob,
    GetBaseBackup,
    GetChunk,
    GetIncrementalBaseBackup,
    GetObjectMetadata,
    ListChunks,
    ListObjects,
    MountCapsule,
    NotifyObjectComplete,
    PutChunk,
    PutObject,
    StartObject,
    UpdateObjectComplete,
}
impl std::fmt::Display for BackupStorageActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BackupStorageActions::CommitBackupJob => write!(f, "backup-storage:CommitBackupJob"),
            BackupStorageActions::DeleteObjects => write!(f, "backup-storage:DeleteObjects"),
            BackupStorageActions::DescribeBackupJob => {
                write!(f, "backup-storage:DescribeBackupJob")
            }
            BackupStorageActions::GetBaseBackup => write!(f, "backup-storage:GetBaseBackup"),
            BackupStorageActions::GetChunk => write!(f, "backup-storage:GetChunk"),
            BackupStorageActions::GetIncrementalBaseBackup => {
                write!(f, "backup-storage:GetIncrementalBaseBackup")
            }
            BackupStorageActions::GetObjectMetadata => {
                write!(f, "backup-storage:GetObjectMetadata")
            }
            BackupStorageActions::ListChunks => write!(f, "backup-storage:ListChunks"),
            BackupStorageActions::ListObjects => write!(f, "backup-storage:ListObjects"),
            BackupStorageActions::MountCapsule => write!(f, "backup-storage:MountCapsule"),
            BackupStorageActions::NotifyObjectComplete => {
                write!(f, "backup-storage:NotifyObjectComplete")
            }
            BackupStorageActions::PutChunk => write!(f, "backup-storage:PutChunk"),
            BackupStorageActions::PutObject => write!(f, "backup-storage:PutObject"),
            BackupStorageActions::StartObject => write!(f, "backup-storage:StartObject"),
            BackupStorageActions::UpdateObjectComplete => {
                write!(f, "backup-storage:UpdateObjectComplete")
            }
        }
    }
}
