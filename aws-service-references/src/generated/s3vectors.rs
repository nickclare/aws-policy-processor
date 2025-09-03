// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum S3vectorsActions {
    CreateIndex,
    CreateVectorBucket,
    DeleteIndex,
    DeleteVectorBucket,
    DeleteVectorBucketPolicy,
    DeleteVectors,
    GetIndex,
    GetVectorBucket,
    GetVectorBucketPolicy,
    GetVectors,
    ListIndexes,
    ListVectorBuckets,
    ListVectors,
    PutVectorBucketPolicy,
    PutVectors,
    QueryVectors,
}
impl std::fmt::Display for S3vectorsActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            S3vectorsActions::CreateIndex => write!(f, "s3vectors:CreateIndex"),
            S3vectorsActions::CreateVectorBucket => write!(f, "s3vectors:CreateVectorBucket"),
            S3vectorsActions::DeleteIndex => write!(f, "s3vectors:DeleteIndex"),
            S3vectorsActions::DeleteVectorBucket => write!(f, "s3vectors:DeleteVectorBucket"),
            S3vectorsActions::DeleteVectorBucketPolicy => {
                write!(f, "s3vectors:DeleteVectorBucketPolicy")
            }
            S3vectorsActions::DeleteVectors => write!(f, "s3vectors:DeleteVectors"),
            S3vectorsActions::GetIndex => write!(f, "s3vectors:GetIndex"),
            S3vectorsActions::GetVectorBucket => write!(f, "s3vectors:GetVectorBucket"),
            S3vectorsActions::GetVectorBucketPolicy => write!(f, "s3vectors:GetVectorBucketPolicy"),
            S3vectorsActions::GetVectors => write!(f, "s3vectors:GetVectors"),
            S3vectorsActions::ListIndexes => write!(f, "s3vectors:ListIndexes"),
            S3vectorsActions::ListVectorBuckets => write!(f, "s3vectors:ListVectorBuckets"),
            S3vectorsActions::ListVectors => write!(f, "s3vectors:ListVectors"),
            S3vectorsActions::PutVectorBucketPolicy => write!(f, "s3vectors:PutVectorBucketPolicy"),
            S3vectorsActions::PutVectors => write!(f, "s3vectors:PutVectors"),
            S3vectorsActions::QueryVectors => write!(f, "s3vectors:QueryVectors"),
        }
    }
}
