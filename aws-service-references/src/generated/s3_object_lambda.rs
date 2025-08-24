// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum S3ObjectLambdaActions {
    AbortMultipartUpload,
    DeleteObject,
    DeleteObjectTagging,
    DeleteObjectVersion,
    DeleteObjectVersionTagging,
    GetObject,
    GetObjectAcl,
    GetObjectLegalHold,
    GetObjectRetention,
    GetObjectTagging,
    GetObjectVersion,
    GetObjectVersionAcl,
    GetObjectVersionTagging,
    ListBucket,
    ListBucketMultipartUploads,
    ListBucketVersions,
    ListMultipartUploadParts,
    PutObject,
    PutObjectAcl,
    PutObjectLegalHold,
    PutObjectRetention,
    PutObjectTagging,
    PutObjectVersionAcl,
    PutObjectVersionTagging,
    RestoreObject,
    WriteGetObjectResponse,
}
impl std::fmt::Display for S3ObjectLambdaActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            S3ObjectLambdaActions::AbortMultipartUpload => {
                write!(f, "s3-object-lambda:AbortMultipartUpload")
            }
            S3ObjectLambdaActions::DeleteObject => write!(f, "s3-object-lambda:DeleteObject"),
            S3ObjectLambdaActions::DeleteObjectTagging => {
                write!(f, "s3-object-lambda:DeleteObjectTagging")
            }
            S3ObjectLambdaActions::DeleteObjectVersion => {
                write!(f, "s3-object-lambda:DeleteObjectVersion")
            }
            S3ObjectLambdaActions::DeleteObjectVersionTagging => {
                write!(f, "s3-object-lambda:DeleteObjectVersionTagging")
            }
            S3ObjectLambdaActions::GetObject => write!(f, "s3-object-lambda:GetObject"),
            S3ObjectLambdaActions::GetObjectAcl => write!(f, "s3-object-lambda:GetObjectAcl"),
            S3ObjectLambdaActions::GetObjectLegalHold => {
                write!(f, "s3-object-lambda:GetObjectLegalHold")
            }
            S3ObjectLambdaActions::GetObjectRetention => {
                write!(f, "s3-object-lambda:GetObjectRetention")
            }
            S3ObjectLambdaActions::GetObjectTagging => {
                write!(f, "s3-object-lambda:GetObjectTagging")
            }
            S3ObjectLambdaActions::GetObjectVersion => {
                write!(f, "s3-object-lambda:GetObjectVersion")
            }
            S3ObjectLambdaActions::GetObjectVersionAcl => {
                write!(f, "s3-object-lambda:GetObjectVersionAcl")
            }
            S3ObjectLambdaActions::GetObjectVersionTagging => {
                write!(f, "s3-object-lambda:GetObjectVersionTagging")
            }
            S3ObjectLambdaActions::ListBucket => write!(f, "s3-object-lambda:ListBucket"),
            S3ObjectLambdaActions::ListBucketMultipartUploads => {
                write!(f, "s3-object-lambda:ListBucketMultipartUploads")
            }
            S3ObjectLambdaActions::ListBucketVersions => {
                write!(f, "s3-object-lambda:ListBucketVersions")
            }
            S3ObjectLambdaActions::ListMultipartUploadParts => {
                write!(f, "s3-object-lambda:ListMultipartUploadParts")
            }
            S3ObjectLambdaActions::PutObject => write!(f, "s3-object-lambda:PutObject"),
            S3ObjectLambdaActions::PutObjectAcl => write!(f, "s3-object-lambda:PutObjectAcl"),
            S3ObjectLambdaActions::PutObjectLegalHold => {
                write!(f, "s3-object-lambda:PutObjectLegalHold")
            }
            S3ObjectLambdaActions::PutObjectRetention => {
                write!(f, "s3-object-lambda:PutObjectRetention")
            }
            S3ObjectLambdaActions::PutObjectTagging => {
                write!(f, "s3-object-lambda:PutObjectTagging")
            }
            S3ObjectLambdaActions::PutObjectVersionAcl => {
                write!(f, "s3-object-lambda:PutObjectVersionAcl")
            }
            S3ObjectLambdaActions::PutObjectVersionTagging => {
                write!(f, "s3-object-lambda:PutObjectVersionTagging")
            }
            S3ObjectLambdaActions::RestoreObject => write!(f, "s3-object-lambda:RestoreObject"),
            S3ObjectLambdaActions::WriteGetObjectResponse => {
                write!(f, "s3-object-lambda:WriteGetObjectResponse")
            }
        }
    }
}
