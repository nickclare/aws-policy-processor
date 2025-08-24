// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum ElasticfilesystemActions {
    Backup,
    ClientMount,
    ClientRootAccess,
    ClientWrite,
    CreateAccessPoint,
    CreateFileSystem,
    CreateMountTarget,
    CreateReplicationConfiguration,
    CreateTags,
    DeleteAccessPoint,
    DeleteFileSystem,
    DeleteFileSystemPolicy,
    DeleteMountTarget,
    DeleteReplicationConfiguration,
    DeleteTags,
    DescribeAccessPoints,
    DescribeAccountPreferences,
    DescribeBackupPolicy,
    DescribeFileSystemPolicy,
    DescribeFileSystems,
    DescribeLifecycleConfiguration,
    DescribeMountTargetSecurityGroups,
    DescribeMountTargets,
    DescribeReplicationConfigurations,
    DescribeTags,
    ListTagsForResource,
    ModifyMountTargetSecurityGroups,
    PutAccountPreferences,
    PutBackupPolicy,
    PutFileSystemPolicy,
    PutLifecycleConfiguration,
    ReplicationRead,
    ReplicationWrite,
    Restore,
    TagResource,
    UntagResource,
    UpdateFileSystem,
    UpdateFileSystemProtection,
}
impl std::fmt::Display for ElasticfilesystemActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ElasticfilesystemActions::Backup => write!(f, "elasticfilesystem:Backup"),
            ElasticfilesystemActions::ClientMount => write!(f, "elasticfilesystem:ClientMount"),
            ElasticfilesystemActions::ClientRootAccess => {
                write!(f, "elasticfilesystem:ClientRootAccess")
            }
            ElasticfilesystemActions::ClientWrite => write!(f, "elasticfilesystem:ClientWrite"),
            ElasticfilesystemActions::CreateAccessPoint => {
                write!(f, "elasticfilesystem:CreateAccessPoint")
            }
            ElasticfilesystemActions::CreateFileSystem => {
                write!(f, "elasticfilesystem:CreateFileSystem")
            }
            ElasticfilesystemActions::CreateMountTarget => {
                write!(f, "elasticfilesystem:CreateMountTarget")
            }
            ElasticfilesystemActions::CreateReplicationConfiguration => {
                write!(f, "elasticfilesystem:CreateReplicationConfiguration")
            }
            ElasticfilesystemActions::CreateTags => write!(f, "elasticfilesystem:CreateTags"),
            ElasticfilesystemActions::DeleteAccessPoint => {
                write!(f, "elasticfilesystem:DeleteAccessPoint")
            }
            ElasticfilesystemActions::DeleteFileSystem => {
                write!(f, "elasticfilesystem:DeleteFileSystem")
            }
            ElasticfilesystemActions::DeleteFileSystemPolicy => {
                write!(f, "elasticfilesystem:DeleteFileSystemPolicy")
            }
            ElasticfilesystemActions::DeleteMountTarget => {
                write!(f, "elasticfilesystem:DeleteMountTarget")
            }
            ElasticfilesystemActions::DeleteReplicationConfiguration => {
                write!(f, "elasticfilesystem:DeleteReplicationConfiguration")
            }
            ElasticfilesystemActions::DeleteTags => write!(f, "elasticfilesystem:DeleteTags"),
            ElasticfilesystemActions::DescribeAccessPoints => {
                write!(f, "elasticfilesystem:DescribeAccessPoints")
            }
            ElasticfilesystemActions::DescribeAccountPreferences => {
                write!(f, "elasticfilesystem:DescribeAccountPreferences")
            }
            ElasticfilesystemActions::DescribeBackupPolicy => {
                write!(f, "elasticfilesystem:DescribeBackupPolicy")
            }
            ElasticfilesystemActions::DescribeFileSystemPolicy => {
                write!(f, "elasticfilesystem:DescribeFileSystemPolicy")
            }
            ElasticfilesystemActions::DescribeFileSystems => {
                write!(f, "elasticfilesystem:DescribeFileSystems")
            }
            ElasticfilesystemActions::DescribeLifecycleConfiguration => {
                write!(f, "elasticfilesystem:DescribeLifecycleConfiguration")
            }
            ElasticfilesystemActions::DescribeMountTargetSecurityGroups => {
                write!(f, "elasticfilesystem:DescribeMountTargetSecurityGroups")
            }
            ElasticfilesystemActions::DescribeMountTargets => {
                write!(f, "elasticfilesystem:DescribeMountTargets")
            }
            ElasticfilesystemActions::DescribeReplicationConfigurations => {
                write!(f, "elasticfilesystem:DescribeReplicationConfigurations")
            }
            ElasticfilesystemActions::DescribeTags => write!(f, "elasticfilesystem:DescribeTags"),
            ElasticfilesystemActions::ListTagsForResource => {
                write!(f, "elasticfilesystem:ListTagsForResource")
            }
            ElasticfilesystemActions::ModifyMountTargetSecurityGroups => {
                write!(f, "elasticfilesystem:ModifyMountTargetSecurityGroups")
            }
            ElasticfilesystemActions::PutAccountPreferences => {
                write!(f, "elasticfilesystem:PutAccountPreferences")
            }
            ElasticfilesystemActions::PutBackupPolicy => {
                write!(f, "elasticfilesystem:PutBackupPolicy")
            }
            ElasticfilesystemActions::PutFileSystemPolicy => {
                write!(f, "elasticfilesystem:PutFileSystemPolicy")
            }
            ElasticfilesystemActions::PutLifecycleConfiguration => {
                write!(f, "elasticfilesystem:PutLifecycleConfiguration")
            }
            ElasticfilesystemActions::ReplicationRead => {
                write!(f, "elasticfilesystem:ReplicationRead")
            }
            ElasticfilesystemActions::ReplicationWrite => {
                write!(f, "elasticfilesystem:ReplicationWrite")
            }
            ElasticfilesystemActions::Restore => write!(f, "elasticfilesystem:Restore"),
            ElasticfilesystemActions::TagResource => write!(f, "elasticfilesystem:TagResource"),
            ElasticfilesystemActions::UntagResource => write!(f, "elasticfilesystem:UntagResource"),
            ElasticfilesystemActions::UpdateFileSystem => {
                write!(f, "elasticfilesystem:UpdateFileSystem")
            }
            ElasticfilesystemActions::UpdateFileSystemProtection => {
                write!(f, "elasticfilesystem:UpdateFileSystemProtection")
            }
        }
    }
}
