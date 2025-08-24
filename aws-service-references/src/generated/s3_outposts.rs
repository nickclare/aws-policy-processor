// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum S3OutpostsActions {
    AbortMultipartUpload,
    CreateAccessPoint,
    CreateBucket,
    CreateEndpoint,
    DeleteAccessPoint,
    DeleteAccessPointPolicy,
    DeleteBucket,
    DeleteBucketPolicy,
    DeleteEndpoint,
    DeleteObject,
    DeleteObjectTagging,
    DeleteObjectVersion,
    DeleteObjectVersionTagging,
    GetAccessPoint,
    GetAccessPointPolicy,
    GetBucket,
    GetBucketPolicy,
    GetBucketTagging,
    GetBucketVersioning,
    GetLifecycleConfiguration,
    GetObject,
    GetObjectTagging,
    GetObjectVersion,
    GetObjectVersionForReplication,
    GetObjectVersionTagging,
    GetReplicationConfiguration,
    ListAccessPoints,
    ListBucket,
    ListBucketMultipartUploads,
    ListBucketVersions,
    ListEndpoints,
    ListMultipartUploadParts,
    ListOutpostsWithS3,
    ListRegionalBuckets,
    ListSharedEndpoints,
    PutAccessPointPolicy,
    PutBucketPolicy,
    PutBucketTagging,
    PutBucketVersioning,
    PutLifecycleConfiguration,
    PutObject,
    PutObjectAcl,
    PutObjectTagging,
    PutObjectVersionTagging,
    PutReplicationConfiguration,
    ReplicateDelete,
    ReplicateObject,
    ReplicateTags,
}
impl std::fmt::Display for S3OutpostsActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            S3OutpostsActions::AbortMultipartUpload => {
                write!(f, "s3-outposts:AbortMultipartUpload")
            }
            S3OutpostsActions::CreateAccessPoint => write!(f, "s3-outposts:CreateAccessPoint"),
            S3OutpostsActions::CreateBucket => write!(f, "s3-outposts:CreateBucket"),
            S3OutpostsActions::CreateEndpoint => write!(f, "s3-outposts:CreateEndpoint"),
            S3OutpostsActions::DeleteAccessPoint => write!(f, "s3-outposts:DeleteAccessPoint"),
            S3OutpostsActions::DeleteAccessPointPolicy => {
                write!(f, "s3-outposts:DeleteAccessPointPolicy")
            }
            S3OutpostsActions::DeleteBucket => write!(f, "s3-outposts:DeleteBucket"),
            S3OutpostsActions::DeleteBucketPolicy => write!(f, "s3-outposts:DeleteBucketPolicy"),
            S3OutpostsActions::DeleteEndpoint => write!(f, "s3-outposts:DeleteEndpoint"),
            S3OutpostsActions::DeleteObject => write!(f, "s3-outposts:DeleteObject"),
            S3OutpostsActions::DeleteObjectTagging => write!(f, "s3-outposts:DeleteObjectTagging"),
            S3OutpostsActions::DeleteObjectVersion => write!(f, "s3-outposts:DeleteObjectVersion"),
            S3OutpostsActions::DeleteObjectVersionTagging => {
                write!(f, "s3-outposts:DeleteObjectVersionTagging")
            }
            S3OutpostsActions::GetAccessPoint => write!(f, "s3-outposts:GetAccessPoint"),
            S3OutpostsActions::GetAccessPointPolicy => {
                write!(f, "s3-outposts:GetAccessPointPolicy")
            }
            S3OutpostsActions::GetBucket => write!(f, "s3-outposts:GetBucket"),
            S3OutpostsActions::GetBucketPolicy => write!(f, "s3-outposts:GetBucketPolicy"),
            S3OutpostsActions::GetBucketTagging => write!(f, "s3-outposts:GetBucketTagging"),
            S3OutpostsActions::GetBucketVersioning => write!(f, "s3-outposts:GetBucketVersioning"),
            S3OutpostsActions::GetLifecycleConfiguration => {
                write!(f, "s3-outposts:GetLifecycleConfiguration")
            }
            S3OutpostsActions::GetObject => write!(f, "s3-outposts:GetObject"),
            S3OutpostsActions::GetObjectTagging => write!(f, "s3-outposts:GetObjectTagging"),
            S3OutpostsActions::GetObjectVersion => write!(f, "s3-outposts:GetObjectVersion"),
            S3OutpostsActions::GetObjectVersionForReplication => {
                write!(f, "s3-outposts:GetObjectVersionForReplication")
            }
            S3OutpostsActions::GetObjectVersionTagging => {
                write!(f, "s3-outposts:GetObjectVersionTagging")
            }
            S3OutpostsActions::GetReplicationConfiguration => {
                write!(f, "s3-outposts:GetReplicationConfiguration")
            }
            S3OutpostsActions::ListAccessPoints => write!(f, "s3-outposts:ListAccessPoints"),
            S3OutpostsActions::ListBucket => write!(f, "s3-outposts:ListBucket"),
            S3OutpostsActions::ListBucketMultipartUploads => {
                write!(f, "s3-outposts:ListBucketMultipartUploads")
            }
            S3OutpostsActions::ListBucketVersions => write!(f, "s3-outposts:ListBucketVersions"),
            S3OutpostsActions::ListEndpoints => write!(f, "s3-outposts:ListEndpoints"),
            S3OutpostsActions::ListMultipartUploadParts => {
                write!(f, "s3-outposts:ListMultipartUploadParts")
            }
            S3OutpostsActions::ListOutpostsWithS3 => write!(f, "s3-outposts:ListOutpostsWithS3"),
            S3OutpostsActions::ListRegionalBuckets => write!(f, "s3-outposts:ListRegionalBuckets"),
            S3OutpostsActions::ListSharedEndpoints => write!(f, "s3-outposts:ListSharedEndpoints"),
            S3OutpostsActions::PutAccessPointPolicy => {
                write!(f, "s3-outposts:PutAccessPointPolicy")
            }
            S3OutpostsActions::PutBucketPolicy => write!(f, "s3-outposts:PutBucketPolicy"),
            S3OutpostsActions::PutBucketTagging => write!(f, "s3-outposts:PutBucketTagging"),
            S3OutpostsActions::PutBucketVersioning => write!(f, "s3-outposts:PutBucketVersioning"),
            S3OutpostsActions::PutLifecycleConfiguration => {
                write!(f, "s3-outposts:PutLifecycleConfiguration")
            }
            S3OutpostsActions::PutObject => write!(f, "s3-outposts:PutObject"),
            S3OutpostsActions::PutObjectAcl => write!(f, "s3-outposts:PutObjectAcl"),
            S3OutpostsActions::PutObjectTagging => write!(f, "s3-outposts:PutObjectTagging"),
            S3OutpostsActions::PutObjectVersionTagging => {
                write!(f, "s3-outposts:PutObjectVersionTagging")
            }
            S3OutpostsActions::PutReplicationConfiguration => {
                write!(f, "s3-outposts:PutReplicationConfiguration")
            }
            S3OutpostsActions::ReplicateDelete => write!(f, "s3-outposts:ReplicateDelete"),
            S3OutpostsActions::ReplicateObject => write!(f, "s3-outposts:ReplicateObject"),
            S3OutpostsActions::ReplicateTags => write!(f, "s3-outposts:ReplicateTags"),
        }
    }
}
