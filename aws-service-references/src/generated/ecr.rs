// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum EcrActions {
    BatchCheckLayerAvailability,
    BatchDeleteImage,
    BatchGetImage,
    BatchGetRepositoryScanningConfiguration,
    BatchImportUpstreamImage,
    CompleteLayerUpload,
    CreatePullThroughCacheRule,
    CreateRepository,
    CreateRepositoryCreationTemplate,
    DeleteLifecyclePolicy,
    DeletePullThroughCacheRule,
    DeleteRegistryPolicy,
    DeleteRepository,
    DeleteRepositoryCreationTemplate,
    DeleteRepositoryPolicy,
    DescribeImageReplicationStatus,
    DescribeImageScanFindings,
    DescribeImages,
    DescribePullThroughCacheRules,
    DescribeRegistry,
    DescribeRepositories,
    DescribeRepositoryCreationTemplates,
    GetAccountSetting,
    GetAuthorizationToken,
    GetDownloadUrlForLayer,
    GetImageCopyStatus,
    GetLifecyclePolicy,
    GetLifecyclePolicyPreview,
    GetRegistryPolicy,
    GetRegistryScanningConfiguration,
    GetRepositoryPolicy,
    InitiateLayerUpload,
    ListImages,
    ListTagsForResource,
    PutAccountSetting,
    PutImage,
    PutImageScanningConfiguration,
    PutImageTagMutability,
    PutLifecyclePolicy,
    PutRegistryPolicy,
    PutRegistryScanningConfiguration,
    PutReplicationConfiguration,
    ReplicateImage,
    SetRepositoryPolicy,
    StartImageScan,
    StartLifecyclePolicyPreview,
    TagResource,
    UntagResource,
    UpdatePullThroughCacheRule,
    UpdateRepositoryCreationTemplate,
    UploadLayerPart,
    ValidatePullThroughCacheRule,
}
impl std::fmt::Display for EcrActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EcrActions::BatchCheckLayerAvailability => write!(f, "ecr:BatchCheckLayerAvailability"),
            EcrActions::BatchDeleteImage => write!(f, "ecr:BatchDeleteImage"),
            EcrActions::BatchGetImage => write!(f, "ecr:BatchGetImage"),
            EcrActions::BatchGetRepositoryScanningConfiguration => {
                write!(f, "ecr:BatchGetRepositoryScanningConfiguration")
            }
            EcrActions::BatchImportUpstreamImage => write!(f, "ecr:BatchImportUpstreamImage"),
            EcrActions::CompleteLayerUpload => write!(f, "ecr:CompleteLayerUpload"),
            EcrActions::CreatePullThroughCacheRule => write!(f, "ecr:CreatePullThroughCacheRule"),
            EcrActions::CreateRepository => write!(f, "ecr:CreateRepository"),
            EcrActions::CreateRepositoryCreationTemplate => {
                write!(f, "ecr:CreateRepositoryCreationTemplate")
            }
            EcrActions::DeleteLifecyclePolicy => write!(f, "ecr:DeleteLifecyclePolicy"),
            EcrActions::DeletePullThroughCacheRule => write!(f, "ecr:DeletePullThroughCacheRule"),
            EcrActions::DeleteRegistryPolicy => write!(f, "ecr:DeleteRegistryPolicy"),
            EcrActions::DeleteRepository => write!(f, "ecr:DeleteRepository"),
            EcrActions::DeleteRepositoryCreationTemplate => {
                write!(f, "ecr:DeleteRepositoryCreationTemplate")
            }
            EcrActions::DeleteRepositoryPolicy => write!(f, "ecr:DeleteRepositoryPolicy"),
            EcrActions::DescribeImageReplicationStatus => {
                write!(f, "ecr:DescribeImageReplicationStatus")
            }
            EcrActions::DescribeImageScanFindings => write!(f, "ecr:DescribeImageScanFindings"),
            EcrActions::DescribeImages => write!(f, "ecr:DescribeImages"),
            EcrActions::DescribePullThroughCacheRules => {
                write!(f, "ecr:DescribePullThroughCacheRules")
            }
            EcrActions::DescribeRegistry => write!(f, "ecr:DescribeRegistry"),
            EcrActions::DescribeRepositories => write!(f, "ecr:DescribeRepositories"),
            EcrActions::DescribeRepositoryCreationTemplates => {
                write!(f, "ecr:DescribeRepositoryCreationTemplates")
            }
            EcrActions::GetAccountSetting => write!(f, "ecr:GetAccountSetting"),
            EcrActions::GetAuthorizationToken => write!(f, "ecr:GetAuthorizationToken"),
            EcrActions::GetDownloadUrlForLayer => write!(f, "ecr:GetDownloadUrlForLayer"),
            EcrActions::GetImageCopyStatus => write!(f, "ecr:GetImageCopyStatus"),
            EcrActions::GetLifecyclePolicy => write!(f, "ecr:GetLifecyclePolicy"),
            EcrActions::GetLifecyclePolicyPreview => write!(f, "ecr:GetLifecyclePolicyPreview"),
            EcrActions::GetRegistryPolicy => write!(f, "ecr:GetRegistryPolicy"),
            EcrActions::GetRegistryScanningConfiguration => {
                write!(f, "ecr:GetRegistryScanningConfiguration")
            }
            EcrActions::GetRepositoryPolicy => write!(f, "ecr:GetRepositoryPolicy"),
            EcrActions::InitiateLayerUpload => write!(f, "ecr:InitiateLayerUpload"),
            EcrActions::ListImages => write!(f, "ecr:ListImages"),
            EcrActions::ListTagsForResource => write!(f, "ecr:ListTagsForResource"),
            EcrActions::PutAccountSetting => write!(f, "ecr:PutAccountSetting"),
            EcrActions::PutImage => write!(f, "ecr:PutImage"),
            EcrActions::PutImageScanningConfiguration => {
                write!(f, "ecr:PutImageScanningConfiguration")
            }
            EcrActions::PutImageTagMutability => write!(f, "ecr:PutImageTagMutability"),
            EcrActions::PutLifecyclePolicy => write!(f, "ecr:PutLifecyclePolicy"),
            EcrActions::PutRegistryPolicy => write!(f, "ecr:PutRegistryPolicy"),
            EcrActions::PutRegistryScanningConfiguration => {
                write!(f, "ecr:PutRegistryScanningConfiguration")
            }
            EcrActions::PutReplicationConfiguration => write!(f, "ecr:PutReplicationConfiguration"),
            EcrActions::ReplicateImage => write!(f, "ecr:ReplicateImage"),
            EcrActions::SetRepositoryPolicy => write!(f, "ecr:SetRepositoryPolicy"),
            EcrActions::StartImageScan => write!(f, "ecr:StartImageScan"),
            EcrActions::StartLifecyclePolicyPreview => write!(f, "ecr:StartLifecyclePolicyPreview"),
            EcrActions::TagResource => write!(f, "ecr:TagResource"),
            EcrActions::UntagResource => write!(f, "ecr:UntagResource"),
            EcrActions::UpdatePullThroughCacheRule => write!(f, "ecr:UpdatePullThroughCacheRule"),
            EcrActions::UpdateRepositoryCreationTemplate => {
                write!(f, "ecr:UpdateRepositoryCreationTemplate")
            }
            EcrActions::UploadLayerPart => write!(f, "ecr:UploadLayerPart"),
            EcrActions::ValidatePullThroughCacheRule => {
                write!(f, "ecr:ValidatePullThroughCacheRule")
            }
        }
    }
}
