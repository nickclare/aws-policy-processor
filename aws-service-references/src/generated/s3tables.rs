// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum S3tablesActions {
    CreateNamespace,
    CreateTable,
    CreateTableBucket,
    DeleteNamespace,
    DeleteTable,
    DeleteTableBucket,
    DeleteTableBucketEncryption,
    DeleteTableBucketPolicy,
    DeleteTablePolicy,
    GetNamespace,
    GetTable,
    GetTableBucket,
    GetTableBucketEncryption,
    GetTableBucketMaintenanceConfiguration,
    GetTableBucketPolicy,
    GetTableData,
    GetTableEncryption,
    GetTableMaintenanceConfiguration,
    GetTableMaintenanceJobStatus,
    GetTableMetadataLocation,
    GetTablePolicy,
    ListNamespaces,
    ListTableBuckets,
    ListTables,
    PutTableBucketEncryption,
    PutTableBucketMaintenanceConfiguration,
    PutTableBucketPolicy,
    PutTableData,
    PutTableEncryption,
    PutTableMaintenanceConfiguration,
    PutTablePolicy,
    RenameTable,
    UpdateTableMetadataLocation,
}
impl std::fmt::Display for S3tablesActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            S3tablesActions::CreateNamespace => write!(f, "s3tables:CreateNamespace"),
            S3tablesActions::CreateTable => write!(f, "s3tables:CreateTable"),
            S3tablesActions::CreateTableBucket => write!(f, "s3tables:CreateTableBucket"),
            S3tablesActions::DeleteNamespace => write!(f, "s3tables:DeleteNamespace"),
            S3tablesActions::DeleteTable => write!(f, "s3tables:DeleteTable"),
            S3tablesActions::DeleteTableBucket => write!(f, "s3tables:DeleteTableBucket"),
            S3tablesActions::DeleteTableBucketEncryption => {
                write!(f, "s3tables:DeleteTableBucketEncryption")
            }
            S3tablesActions::DeleteTableBucketPolicy => {
                write!(f, "s3tables:DeleteTableBucketPolicy")
            }
            S3tablesActions::DeleteTablePolicy => write!(f, "s3tables:DeleteTablePolicy"),
            S3tablesActions::GetNamespace => write!(f, "s3tables:GetNamespace"),
            S3tablesActions::GetTable => write!(f, "s3tables:GetTable"),
            S3tablesActions::GetTableBucket => write!(f, "s3tables:GetTableBucket"),
            S3tablesActions::GetTableBucketEncryption => {
                write!(f, "s3tables:GetTableBucketEncryption")
            }
            S3tablesActions::GetTableBucketMaintenanceConfiguration => {
                write!(f, "s3tables:GetTableBucketMaintenanceConfiguration")
            }
            S3tablesActions::GetTableBucketPolicy => write!(f, "s3tables:GetTableBucketPolicy"),
            S3tablesActions::GetTableData => write!(f, "s3tables:GetTableData"),
            S3tablesActions::GetTableEncryption => write!(f, "s3tables:GetTableEncryption"),
            S3tablesActions::GetTableMaintenanceConfiguration => {
                write!(f, "s3tables:GetTableMaintenanceConfiguration")
            }
            S3tablesActions::GetTableMaintenanceJobStatus => {
                write!(f, "s3tables:GetTableMaintenanceJobStatus")
            }
            S3tablesActions::GetTableMetadataLocation => {
                write!(f, "s3tables:GetTableMetadataLocation")
            }
            S3tablesActions::GetTablePolicy => write!(f, "s3tables:GetTablePolicy"),
            S3tablesActions::ListNamespaces => write!(f, "s3tables:ListNamespaces"),
            S3tablesActions::ListTableBuckets => write!(f, "s3tables:ListTableBuckets"),
            S3tablesActions::ListTables => write!(f, "s3tables:ListTables"),
            S3tablesActions::PutTableBucketEncryption => {
                write!(f, "s3tables:PutTableBucketEncryption")
            }
            S3tablesActions::PutTableBucketMaintenanceConfiguration => {
                write!(f, "s3tables:PutTableBucketMaintenanceConfiguration")
            }
            S3tablesActions::PutTableBucketPolicy => write!(f, "s3tables:PutTableBucketPolicy"),
            S3tablesActions::PutTableData => write!(f, "s3tables:PutTableData"),
            S3tablesActions::PutTableEncryption => write!(f, "s3tables:PutTableEncryption"),
            S3tablesActions::PutTableMaintenanceConfiguration => {
                write!(f, "s3tables:PutTableMaintenanceConfiguration")
            }
            S3tablesActions::PutTablePolicy => write!(f, "s3tables:PutTablePolicy"),
            S3tablesActions::RenameTable => write!(f, "s3tables:RenameTable"),
            S3tablesActions::UpdateTableMetadataLocation => {
                write!(f, "s3tables:UpdateTableMetadataLocation")
            }
        }
    }
}
