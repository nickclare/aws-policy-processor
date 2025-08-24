// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum S3expressActions {
    CreateAccessPoint,
    CreateBucket,
    CreateSession,
    DeleteAccessPoint,
    DeleteAccessPointPolicy,
    DeleteAccessPointScope,
    DeleteBucket,
    DeleteBucketPolicy,
    GetAccessPoint,
    GetAccessPointPolicy,
    GetAccessPointScope,
    GetBucketPolicy,
    GetEncryptionConfiguration,
    GetLifecycleConfiguration,
    ListAccessPointsForDirectoryBuckets,
    ListAllMyDirectoryBuckets,
    ListTagsForResource,
    PutAccessPointPolicy,
    PutAccessPointScope,
    PutBucketPolicy,
    PutEncryptionConfiguration,
    PutLifecycleConfiguration,
    TagResource,
    UntagResource,
}
impl std::fmt::Display for S3expressActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            S3expressActions::CreateAccessPoint => write!(f, "s3express:CreateAccessPoint"),
            S3expressActions::CreateBucket => write!(f, "s3express:CreateBucket"),
            S3expressActions::CreateSession => write!(f, "s3express:CreateSession"),
            S3expressActions::DeleteAccessPoint => write!(f, "s3express:DeleteAccessPoint"),
            S3expressActions::DeleteAccessPointPolicy => {
                write!(f, "s3express:DeleteAccessPointPolicy")
            }
            S3expressActions::DeleteAccessPointScope => {
                write!(f, "s3express:DeleteAccessPointScope")
            }
            S3expressActions::DeleteBucket => write!(f, "s3express:DeleteBucket"),
            S3expressActions::DeleteBucketPolicy => write!(f, "s3express:DeleteBucketPolicy"),
            S3expressActions::GetAccessPoint => write!(f, "s3express:GetAccessPoint"),
            S3expressActions::GetAccessPointPolicy => write!(f, "s3express:GetAccessPointPolicy"),
            S3expressActions::GetAccessPointScope => write!(f, "s3express:GetAccessPointScope"),
            S3expressActions::GetBucketPolicy => write!(f, "s3express:GetBucketPolicy"),
            S3expressActions::GetEncryptionConfiguration => {
                write!(f, "s3express:GetEncryptionConfiguration")
            }
            S3expressActions::GetLifecycleConfiguration => {
                write!(f, "s3express:GetLifecycleConfiguration")
            }
            S3expressActions::ListAccessPointsForDirectoryBuckets => {
                write!(f, "s3express:ListAccessPointsForDirectoryBuckets")
            }
            S3expressActions::ListAllMyDirectoryBuckets => {
                write!(f, "s3express:ListAllMyDirectoryBuckets")
            }
            S3expressActions::ListTagsForResource => write!(f, "s3express:ListTagsForResource"),
            S3expressActions::PutAccessPointPolicy => write!(f, "s3express:PutAccessPointPolicy"),
            S3expressActions::PutAccessPointScope => write!(f, "s3express:PutAccessPointScope"),
            S3expressActions::PutBucketPolicy => write!(f, "s3express:PutBucketPolicy"),
            S3expressActions::PutEncryptionConfiguration => {
                write!(f, "s3express:PutEncryptionConfiguration")
            }
            S3expressActions::PutLifecycleConfiguration => {
                write!(f, "s3express:PutLifecycleConfiguration")
            }
            S3expressActions::TagResource => write!(f, "s3express:TagResource"),
            S3expressActions::UntagResource => write!(f, "s3express:UntagResource"),
        }
    }
}
