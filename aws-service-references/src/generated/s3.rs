// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum S3Actions {
    AbortMultipartUpload,
    AssociateAccessGrantsIdentityCenter,
    BypassGovernanceRetention,
    CreateAccessGrant,
    CreateAccessGrantsInstance,
    CreateAccessGrantsLocation,
    CreateAccessPoint,
    CreateAccessPointForObjectLambda,
    CreateBucket,
    CreateBucketMetadataTableConfiguration,
    CreateJob,
    CreateMultiRegionAccessPoint,
    CreateStorageLensGroup,
    DeleteAccessGrant,
    DeleteAccessGrantsInstance,
    DeleteAccessGrantsInstanceResourcePolicy,
    DeleteAccessGrantsLocation,
    DeleteAccessPoint,
    DeleteAccessPointForObjectLambda,
    DeleteAccessPointPolicy,
    DeleteAccessPointPolicyForObjectLambda,
    DeleteBucket,
    DeleteBucketMetadataTableConfiguration,
    DeleteBucketPolicy,
    DeleteBucketWebsite,
    DeleteJobTagging,
    DeleteMultiRegionAccessPoint,
    DeleteObject,
    DeleteObjectTagging,
    DeleteObjectVersion,
    DeleteObjectVersionTagging,
    DeleteStorageLensConfiguration,
    DeleteStorageLensConfigurationTagging,
    DeleteStorageLensGroup,
    DescribeJob,
    DescribeMultiRegionAccessPointOperation,
    DissociateAccessGrantsIdentityCenter,
    GetAccelerateConfiguration,
    GetAccessGrant,
    GetAccessGrantsInstance,
    GetAccessGrantsInstanceForPrefix,
    GetAccessGrantsInstanceResourcePolicy,
    GetAccessGrantsLocation,
    GetAccessPoint,
    GetAccessPointConfigurationForObjectLambda,
    GetAccessPointForObjectLambda,
    GetAccessPointPolicy,
    GetAccessPointPolicyForObjectLambda,
    GetAccessPointPolicyStatus,
    GetAccessPointPolicyStatusForObjectLambda,
    GetAccountPublicAccessBlock,
    GetAnalyticsConfiguration,
    GetBucketAcl,
    GetBucketCors,
    GetBucketLocation,
    GetBucketLogging,
    GetBucketMetadataTableConfiguration,
    GetBucketNotification,
    GetBucketObjectLockConfiguration,
    GetBucketOwnershipControls,
    GetBucketPolicy,
    GetBucketPolicyStatus,
    GetBucketPublicAccessBlock,
    GetBucketRequestPayment,
    GetBucketTagging,
    GetBucketVersioning,
    GetBucketWebsite,
    GetDataAccess,
    GetEncryptionConfiguration,
    GetIntelligentTieringConfiguration,
    GetInventoryConfiguration,
    GetJobTagging,
    GetLifecycleConfiguration,
    GetMetricsConfiguration,
    GetMultiRegionAccessPoint,
    GetMultiRegionAccessPointPolicy,
    GetMultiRegionAccessPointPolicyStatus,
    GetMultiRegionAccessPointRoutes,
    GetObject,
    GetObjectAcl,
    GetObjectAttributes,
    GetObjectLegalHold,
    GetObjectRetention,
    GetObjectTagging,
    GetObjectTorrent,
    GetObjectVersion,
    GetObjectVersionAcl,
    GetObjectVersionAttributes,
    GetObjectVersionForReplication,
    GetObjectVersionTagging,
    GetObjectVersionTorrent,
    GetReplicationConfiguration,
    GetStorageLensConfiguration,
    GetStorageLensConfigurationTagging,
    GetStorageLensDashboard,
    GetStorageLensGroup,
    InitiateReplication,
    ListAccessGrants,
    ListAccessGrantsInstances,
    ListAccessGrantsLocations,
    ListAccessPoints,
    ListAccessPointsForObjectLambda,
    ListAllMyBuckets,
    ListBucket,
    ListBucketMultipartUploads,
    ListBucketVersions,
    ListCallerAccessGrants,
    ListJobs,
    ListMultiRegionAccessPoints,
    ListMultipartUploadParts,
    ListStorageLensConfigurations,
    ListStorageLensGroups,
    ListTagsForResource,
    ObjectOwnerOverrideToBucketOwner,
    PauseReplication,
    PutAccelerateConfiguration,
    PutAccessGrantsInstanceResourcePolicy,
    PutAccessPointConfigurationForObjectLambda,
    PutAccessPointPolicy,
    PutAccessPointPolicyForObjectLambda,
    PutAccessPointPublicAccessBlock,
    PutAccountPublicAccessBlock,
    PutAnalyticsConfiguration,
    PutBucketAcl,
    PutBucketCors,
    PutBucketLogging,
    PutBucketNotification,
    PutBucketObjectLockConfiguration,
    PutBucketOwnershipControls,
    PutBucketPolicy,
    PutBucketPublicAccessBlock,
    PutBucketRequestPayment,
    PutBucketTagging,
    PutBucketVersioning,
    PutBucketWebsite,
    PutEncryptionConfiguration,
    PutIntelligentTieringConfiguration,
    PutInventoryConfiguration,
    PutJobTagging,
    PutLifecycleConfiguration,
    PutMetricsConfiguration,
    PutMultiRegionAccessPointPolicy,
    PutObject,
    PutObjectAcl,
    PutObjectLegalHold,
    PutObjectRetention,
    PutObjectTagging,
    PutObjectVersionAcl,
    PutObjectVersionTagging,
    PutReplicationConfiguration,
    PutStorageLensConfiguration,
    PutStorageLensConfigurationTagging,
    ReplicateDelete,
    ReplicateObject,
    ReplicateTags,
    RestoreObject,
    SubmitMultiRegionAccessPointRoutes,
    TagResource,
    UntagResource,
    UpdateAccessGrantsLocation,
    UpdateBucketMetadataInventoryTableConfiguration,
    UpdateBucketMetadataJournalTableConfiguration,
    UpdateJobPriority,
    UpdateJobStatus,
    UpdateStorageLensGroup,
}
impl std::fmt::Display for S3Actions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            S3Actions::AbortMultipartUpload => write!(f, "s3:AbortMultipartUpload"),
            S3Actions::AssociateAccessGrantsIdentityCenter => {
                write!(f, "s3:AssociateAccessGrantsIdentityCenter")
            }
            S3Actions::BypassGovernanceRetention => write!(f, "s3:BypassGovernanceRetention"),
            S3Actions::CreateAccessGrant => write!(f, "s3:CreateAccessGrant"),
            S3Actions::CreateAccessGrantsInstance => write!(f, "s3:CreateAccessGrantsInstance"),
            S3Actions::CreateAccessGrantsLocation => write!(f, "s3:CreateAccessGrantsLocation"),
            S3Actions::CreateAccessPoint => write!(f, "s3:CreateAccessPoint"),
            S3Actions::CreateAccessPointForObjectLambda => {
                write!(f, "s3:CreateAccessPointForObjectLambda")
            }
            S3Actions::CreateBucket => write!(f, "s3:CreateBucket"),
            S3Actions::CreateBucketMetadataTableConfiguration => {
                write!(f, "s3:CreateBucketMetadataTableConfiguration")
            }
            S3Actions::CreateJob => write!(f, "s3:CreateJob"),
            S3Actions::CreateMultiRegionAccessPoint => write!(f, "s3:CreateMultiRegionAccessPoint"),
            S3Actions::CreateStorageLensGroup => write!(f, "s3:CreateStorageLensGroup"),
            S3Actions::DeleteAccessGrant => write!(f, "s3:DeleteAccessGrant"),
            S3Actions::DeleteAccessGrantsInstance => write!(f, "s3:DeleteAccessGrantsInstance"),
            S3Actions::DeleteAccessGrantsInstanceResourcePolicy => {
                write!(f, "s3:DeleteAccessGrantsInstanceResourcePolicy")
            }
            S3Actions::DeleteAccessGrantsLocation => write!(f, "s3:DeleteAccessGrantsLocation"),
            S3Actions::DeleteAccessPoint => write!(f, "s3:DeleteAccessPoint"),
            S3Actions::DeleteAccessPointForObjectLambda => {
                write!(f, "s3:DeleteAccessPointForObjectLambda")
            }
            S3Actions::DeleteAccessPointPolicy => write!(f, "s3:DeleteAccessPointPolicy"),
            S3Actions::DeleteAccessPointPolicyForObjectLambda => {
                write!(f, "s3:DeleteAccessPointPolicyForObjectLambda")
            }
            S3Actions::DeleteBucket => write!(f, "s3:DeleteBucket"),
            S3Actions::DeleteBucketMetadataTableConfiguration => {
                write!(f, "s3:DeleteBucketMetadataTableConfiguration")
            }
            S3Actions::DeleteBucketPolicy => write!(f, "s3:DeleteBucketPolicy"),
            S3Actions::DeleteBucketWebsite => write!(f, "s3:DeleteBucketWebsite"),
            S3Actions::DeleteJobTagging => write!(f, "s3:DeleteJobTagging"),
            S3Actions::DeleteMultiRegionAccessPoint => write!(f, "s3:DeleteMultiRegionAccessPoint"),
            S3Actions::DeleteObject => write!(f, "s3:DeleteObject"),
            S3Actions::DeleteObjectTagging => write!(f, "s3:DeleteObjectTagging"),
            S3Actions::DeleteObjectVersion => write!(f, "s3:DeleteObjectVersion"),
            S3Actions::DeleteObjectVersionTagging => write!(f, "s3:DeleteObjectVersionTagging"),
            S3Actions::DeleteStorageLensConfiguration => {
                write!(f, "s3:DeleteStorageLensConfiguration")
            }
            S3Actions::DeleteStorageLensConfigurationTagging => {
                write!(f, "s3:DeleteStorageLensConfigurationTagging")
            }
            S3Actions::DeleteStorageLensGroup => write!(f, "s3:DeleteStorageLensGroup"),
            S3Actions::DescribeJob => write!(f, "s3:DescribeJob"),
            S3Actions::DescribeMultiRegionAccessPointOperation => {
                write!(f, "s3:DescribeMultiRegionAccessPointOperation")
            }
            S3Actions::DissociateAccessGrantsIdentityCenter => {
                write!(f, "s3:DissociateAccessGrantsIdentityCenter")
            }
            S3Actions::GetAccelerateConfiguration => write!(f, "s3:GetAccelerateConfiguration"),
            S3Actions::GetAccessGrant => write!(f, "s3:GetAccessGrant"),
            S3Actions::GetAccessGrantsInstance => write!(f, "s3:GetAccessGrantsInstance"),
            S3Actions::GetAccessGrantsInstanceForPrefix => {
                write!(f, "s3:GetAccessGrantsInstanceForPrefix")
            }
            S3Actions::GetAccessGrantsInstanceResourcePolicy => {
                write!(f, "s3:GetAccessGrantsInstanceResourcePolicy")
            }
            S3Actions::GetAccessGrantsLocation => write!(f, "s3:GetAccessGrantsLocation"),
            S3Actions::GetAccessPoint => write!(f, "s3:GetAccessPoint"),
            S3Actions::GetAccessPointConfigurationForObjectLambda => {
                write!(f, "s3:GetAccessPointConfigurationForObjectLambda")
            }
            S3Actions::GetAccessPointForObjectLambda => {
                write!(f, "s3:GetAccessPointForObjectLambda")
            }
            S3Actions::GetAccessPointPolicy => write!(f, "s3:GetAccessPointPolicy"),
            S3Actions::GetAccessPointPolicyForObjectLambda => {
                write!(f, "s3:GetAccessPointPolicyForObjectLambda")
            }
            S3Actions::GetAccessPointPolicyStatus => write!(f, "s3:GetAccessPointPolicyStatus"),
            S3Actions::GetAccessPointPolicyStatusForObjectLambda => {
                write!(f, "s3:GetAccessPointPolicyStatusForObjectLambda")
            }
            S3Actions::GetAccountPublicAccessBlock => write!(f, "s3:GetAccountPublicAccessBlock"),
            S3Actions::GetAnalyticsConfiguration => write!(f, "s3:GetAnalyticsConfiguration"),
            S3Actions::GetBucketAcl => write!(f, "s3:GetBucketAcl"),
            S3Actions::GetBucketCors => write!(f, "s3:GetBucketCORS"),
            S3Actions::GetBucketLocation => write!(f, "s3:GetBucketLocation"),
            S3Actions::GetBucketLogging => write!(f, "s3:GetBucketLogging"),
            S3Actions::GetBucketMetadataTableConfiguration => {
                write!(f, "s3:GetBucketMetadataTableConfiguration")
            }
            S3Actions::GetBucketNotification => write!(f, "s3:GetBucketNotification"),
            S3Actions::GetBucketObjectLockConfiguration => {
                write!(f, "s3:GetBucketObjectLockConfiguration")
            }
            S3Actions::GetBucketOwnershipControls => write!(f, "s3:GetBucketOwnershipControls"),
            S3Actions::GetBucketPolicy => write!(f, "s3:GetBucketPolicy"),
            S3Actions::GetBucketPolicyStatus => write!(f, "s3:GetBucketPolicyStatus"),
            S3Actions::GetBucketPublicAccessBlock => write!(f, "s3:GetBucketPublicAccessBlock"),
            S3Actions::GetBucketRequestPayment => write!(f, "s3:GetBucketRequestPayment"),
            S3Actions::GetBucketTagging => write!(f, "s3:GetBucketTagging"),
            S3Actions::GetBucketVersioning => write!(f, "s3:GetBucketVersioning"),
            S3Actions::GetBucketWebsite => write!(f, "s3:GetBucketWebsite"),
            S3Actions::GetDataAccess => write!(f, "s3:GetDataAccess"),
            S3Actions::GetEncryptionConfiguration => write!(f, "s3:GetEncryptionConfiguration"),
            S3Actions::GetIntelligentTieringConfiguration => {
                write!(f, "s3:GetIntelligentTieringConfiguration")
            }
            S3Actions::GetInventoryConfiguration => write!(f, "s3:GetInventoryConfiguration"),
            S3Actions::GetJobTagging => write!(f, "s3:GetJobTagging"),
            S3Actions::GetLifecycleConfiguration => write!(f, "s3:GetLifecycleConfiguration"),
            S3Actions::GetMetricsConfiguration => write!(f, "s3:GetMetricsConfiguration"),
            S3Actions::GetMultiRegionAccessPoint => write!(f, "s3:GetMultiRegionAccessPoint"),
            S3Actions::GetMultiRegionAccessPointPolicy => {
                write!(f, "s3:GetMultiRegionAccessPointPolicy")
            }
            S3Actions::GetMultiRegionAccessPointPolicyStatus => {
                write!(f, "s3:GetMultiRegionAccessPointPolicyStatus")
            }
            S3Actions::GetMultiRegionAccessPointRoutes => {
                write!(f, "s3:GetMultiRegionAccessPointRoutes")
            }
            S3Actions::GetObject => write!(f, "s3:GetObject"),
            S3Actions::GetObjectAcl => write!(f, "s3:GetObjectAcl"),
            S3Actions::GetObjectAttributes => write!(f, "s3:GetObjectAttributes"),
            S3Actions::GetObjectLegalHold => write!(f, "s3:GetObjectLegalHold"),
            S3Actions::GetObjectRetention => write!(f, "s3:GetObjectRetention"),
            S3Actions::GetObjectTagging => write!(f, "s3:GetObjectTagging"),
            S3Actions::GetObjectTorrent => write!(f, "s3:GetObjectTorrent"),
            S3Actions::GetObjectVersion => write!(f, "s3:GetObjectVersion"),
            S3Actions::GetObjectVersionAcl => write!(f, "s3:GetObjectVersionAcl"),
            S3Actions::GetObjectVersionAttributes => write!(f, "s3:GetObjectVersionAttributes"),
            S3Actions::GetObjectVersionForReplication => {
                write!(f, "s3:GetObjectVersionForReplication")
            }
            S3Actions::GetObjectVersionTagging => write!(f, "s3:GetObjectVersionTagging"),
            S3Actions::GetObjectVersionTorrent => write!(f, "s3:GetObjectVersionTorrent"),
            S3Actions::GetReplicationConfiguration => write!(f, "s3:GetReplicationConfiguration"),
            S3Actions::GetStorageLensConfiguration => write!(f, "s3:GetStorageLensConfiguration"),
            S3Actions::GetStorageLensConfigurationTagging => {
                write!(f, "s3:GetStorageLensConfigurationTagging")
            }
            S3Actions::GetStorageLensDashboard => write!(f, "s3:GetStorageLensDashboard"),
            S3Actions::GetStorageLensGroup => write!(f, "s3:GetStorageLensGroup"),
            S3Actions::InitiateReplication => write!(f, "s3:InitiateReplication"),
            S3Actions::ListAccessGrants => write!(f, "s3:ListAccessGrants"),
            S3Actions::ListAccessGrantsInstances => write!(f, "s3:ListAccessGrantsInstances"),
            S3Actions::ListAccessGrantsLocations => write!(f, "s3:ListAccessGrantsLocations"),
            S3Actions::ListAccessPoints => write!(f, "s3:ListAccessPoints"),
            S3Actions::ListAccessPointsForObjectLambda => {
                write!(f, "s3:ListAccessPointsForObjectLambda")
            }
            S3Actions::ListAllMyBuckets => write!(f, "s3:ListAllMyBuckets"),
            S3Actions::ListBucket => write!(f, "s3:ListBucket"),
            S3Actions::ListBucketMultipartUploads => write!(f, "s3:ListBucketMultipartUploads"),
            S3Actions::ListBucketVersions => write!(f, "s3:ListBucketVersions"),
            S3Actions::ListCallerAccessGrants => write!(f, "s3:ListCallerAccessGrants"),
            S3Actions::ListJobs => write!(f, "s3:ListJobs"),
            S3Actions::ListMultiRegionAccessPoints => write!(f, "s3:ListMultiRegionAccessPoints"),
            S3Actions::ListMultipartUploadParts => write!(f, "s3:ListMultipartUploadParts"),
            S3Actions::ListStorageLensConfigurations => {
                write!(f, "s3:ListStorageLensConfigurations")
            }
            S3Actions::ListStorageLensGroups => write!(f, "s3:ListStorageLensGroups"),
            S3Actions::ListTagsForResource => write!(f, "s3:ListTagsForResource"),
            S3Actions::ObjectOwnerOverrideToBucketOwner => {
                write!(f, "s3:ObjectOwnerOverrideToBucketOwner")
            }
            S3Actions::PauseReplication => write!(f, "s3:PauseReplication"),
            S3Actions::PutAccelerateConfiguration => write!(f, "s3:PutAccelerateConfiguration"),
            S3Actions::PutAccessGrantsInstanceResourcePolicy => {
                write!(f, "s3:PutAccessGrantsInstanceResourcePolicy")
            }
            S3Actions::PutAccessPointConfigurationForObjectLambda => {
                write!(f, "s3:PutAccessPointConfigurationForObjectLambda")
            }
            S3Actions::PutAccessPointPolicy => write!(f, "s3:PutAccessPointPolicy"),
            S3Actions::PutAccessPointPolicyForObjectLambda => {
                write!(f, "s3:PutAccessPointPolicyForObjectLambda")
            }
            S3Actions::PutAccessPointPublicAccessBlock => {
                write!(f, "s3:PutAccessPointPublicAccessBlock")
            }
            S3Actions::PutAccountPublicAccessBlock => write!(f, "s3:PutAccountPublicAccessBlock"),
            S3Actions::PutAnalyticsConfiguration => write!(f, "s3:PutAnalyticsConfiguration"),
            S3Actions::PutBucketAcl => write!(f, "s3:PutBucketAcl"),
            S3Actions::PutBucketCors => write!(f, "s3:PutBucketCORS"),
            S3Actions::PutBucketLogging => write!(f, "s3:PutBucketLogging"),
            S3Actions::PutBucketNotification => write!(f, "s3:PutBucketNotification"),
            S3Actions::PutBucketObjectLockConfiguration => {
                write!(f, "s3:PutBucketObjectLockConfiguration")
            }
            S3Actions::PutBucketOwnershipControls => write!(f, "s3:PutBucketOwnershipControls"),
            S3Actions::PutBucketPolicy => write!(f, "s3:PutBucketPolicy"),
            S3Actions::PutBucketPublicAccessBlock => write!(f, "s3:PutBucketPublicAccessBlock"),
            S3Actions::PutBucketRequestPayment => write!(f, "s3:PutBucketRequestPayment"),
            S3Actions::PutBucketTagging => write!(f, "s3:PutBucketTagging"),
            S3Actions::PutBucketVersioning => write!(f, "s3:PutBucketVersioning"),
            S3Actions::PutBucketWebsite => write!(f, "s3:PutBucketWebsite"),
            S3Actions::PutEncryptionConfiguration => write!(f, "s3:PutEncryptionConfiguration"),
            S3Actions::PutIntelligentTieringConfiguration => {
                write!(f, "s3:PutIntelligentTieringConfiguration")
            }
            S3Actions::PutInventoryConfiguration => write!(f, "s3:PutInventoryConfiguration"),
            S3Actions::PutJobTagging => write!(f, "s3:PutJobTagging"),
            S3Actions::PutLifecycleConfiguration => write!(f, "s3:PutLifecycleConfiguration"),
            S3Actions::PutMetricsConfiguration => write!(f, "s3:PutMetricsConfiguration"),
            S3Actions::PutMultiRegionAccessPointPolicy => {
                write!(f, "s3:PutMultiRegionAccessPointPolicy")
            }
            S3Actions::PutObject => write!(f, "s3:PutObject"),
            S3Actions::PutObjectAcl => write!(f, "s3:PutObjectAcl"),
            S3Actions::PutObjectLegalHold => write!(f, "s3:PutObjectLegalHold"),
            S3Actions::PutObjectRetention => write!(f, "s3:PutObjectRetention"),
            S3Actions::PutObjectTagging => write!(f, "s3:PutObjectTagging"),
            S3Actions::PutObjectVersionAcl => write!(f, "s3:PutObjectVersionAcl"),
            S3Actions::PutObjectVersionTagging => write!(f, "s3:PutObjectVersionTagging"),
            S3Actions::PutReplicationConfiguration => write!(f, "s3:PutReplicationConfiguration"),
            S3Actions::PutStorageLensConfiguration => write!(f, "s3:PutStorageLensConfiguration"),
            S3Actions::PutStorageLensConfigurationTagging => {
                write!(f, "s3:PutStorageLensConfigurationTagging")
            }
            S3Actions::ReplicateDelete => write!(f, "s3:ReplicateDelete"),
            S3Actions::ReplicateObject => write!(f, "s3:ReplicateObject"),
            S3Actions::ReplicateTags => write!(f, "s3:ReplicateTags"),
            S3Actions::RestoreObject => write!(f, "s3:RestoreObject"),
            S3Actions::SubmitMultiRegionAccessPointRoutes => {
                write!(f, "s3:SubmitMultiRegionAccessPointRoutes")
            }
            S3Actions::TagResource => write!(f, "s3:TagResource"),
            S3Actions::UntagResource => write!(f, "s3:UntagResource"),
            S3Actions::UpdateAccessGrantsLocation => write!(f, "s3:UpdateAccessGrantsLocation"),
            S3Actions::UpdateBucketMetadataInventoryTableConfiguration => {
                write!(f, "s3:UpdateBucketMetadataInventoryTableConfiguration")
            }
            S3Actions::UpdateBucketMetadataJournalTableConfiguration => {
                write!(f, "s3:UpdateBucketMetadataJournalTableConfiguration")
            }
            S3Actions::UpdateJobPriority => write!(f, "s3:UpdateJobPriority"),
            S3Actions::UpdateJobStatus => write!(f, "s3:UpdateJobStatus"),
            S3Actions::UpdateStorageLensGroup => write!(f, "s3:UpdateStorageLensGroup"),
        }
    }
}
