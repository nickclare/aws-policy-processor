// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum CloudhsmActions {
    CopyBackupToRegion,
    CreateCluster,
    CreateHsm,
    DeleteBackup,
    DeleteCluster,
    DeleteHsm,
    DeleteResourcePolicy,
    DescribeBackups,
    DescribeClusters,
    GetResourcePolicy,
    InitializeCluster,
    ListTags,
    ModifyBackupAttributes,
    ModifyCluster,
    PutResourcePolicy,
    RestoreBackup,
    TagResource,
    UntagResource,
}
impl std::fmt::Display for CloudhsmActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CloudhsmActions::CopyBackupToRegion => write!(f, "cloudhsm:CopyBackupToRegion"),
            CloudhsmActions::CreateCluster => write!(f, "cloudhsm:CreateCluster"),
            CloudhsmActions::CreateHsm => write!(f, "cloudhsm:CreateHsm"),
            CloudhsmActions::DeleteBackup => write!(f, "cloudhsm:DeleteBackup"),
            CloudhsmActions::DeleteCluster => write!(f, "cloudhsm:DeleteCluster"),
            CloudhsmActions::DeleteHsm => write!(f, "cloudhsm:DeleteHsm"),
            CloudhsmActions::DeleteResourcePolicy => write!(f, "cloudhsm:DeleteResourcePolicy"),
            CloudhsmActions::DescribeBackups => write!(f, "cloudhsm:DescribeBackups"),
            CloudhsmActions::DescribeClusters => write!(f, "cloudhsm:DescribeClusters"),
            CloudhsmActions::GetResourcePolicy => write!(f, "cloudhsm:GetResourcePolicy"),
            CloudhsmActions::InitializeCluster => write!(f, "cloudhsm:InitializeCluster"),
            CloudhsmActions::ListTags => write!(f, "cloudhsm:ListTags"),
            CloudhsmActions::ModifyBackupAttributes => write!(f, "cloudhsm:ModifyBackupAttributes"),
            CloudhsmActions::ModifyCluster => write!(f, "cloudhsm:ModifyCluster"),
            CloudhsmActions::PutResourcePolicy => write!(f, "cloudhsm:PutResourcePolicy"),
            CloudhsmActions::RestoreBackup => write!(f, "cloudhsm:RestoreBackup"),
            CloudhsmActions::TagResource => write!(f, "cloudhsm:TagResource"),
            CloudhsmActions::UntagResource => write!(f, "cloudhsm:UntagResource"),
        }
    }
}
