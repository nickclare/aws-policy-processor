// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum EcrPublicActions {
    BatchCheckLayerAvailability,
    BatchDeleteImage,
    CompleteLayerUpload,
    CreateRepository,
    DeleteRepository,
    DeleteRepositoryPolicy,
    DescribeImageTags,
    DescribeImages,
    DescribeRegistries,
    DescribeRepositories,
    GetAuthorizationToken,
    GetRegistryCatalogData,
    GetRepositoryCatalogData,
    GetRepositoryPolicy,
    InitiateLayerUpload,
    ListTagsForResource,
    PutImage,
    PutRegistryCatalogData,
    PutRepositoryCatalogData,
    SetRepositoryPolicy,
    TagResource,
    UntagResource,
    UploadLayerPart,
}
impl std::fmt::Display for EcrPublicActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EcrPublicActions::BatchCheckLayerAvailability => {
                write!(f, "ecr-public:BatchCheckLayerAvailability")
            }
            EcrPublicActions::BatchDeleteImage => write!(f, "ecr-public:BatchDeleteImage"),
            EcrPublicActions::CompleteLayerUpload => write!(f, "ecr-public:CompleteLayerUpload"),
            EcrPublicActions::CreateRepository => write!(f, "ecr-public:CreateRepository"),
            EcrPublicActions::DeleteRepository => write!(f, "ecr-public:DeleteRepository"),
            EcrPublicActions::DeleteRepositoryPolicy => {
                write!(f, "ecr-public:DeleteRepositoryPolicy")
            }
            EcrPublicActions::DescribeImageTags => write!(f, "ecr-public:DescribeImageTags"),
            EcrPublicActions::DescribeImages => write!(f, "ecr-public:DescribeImages"),
            EcrPublicActions::DescribeRegistries => write!(f, "ecr-public:DescribeRegistries"),
            EcrPublicActions::DescribeRepositories => write!(f, "ecr-public:DescribeRepositories"),
            EcrPublicActions::GetAuthorizationToken => {
                write!(f, "ecr-public:GetAuthorizationToken")
            }
            EcrPublicActions::GetRegistryCatalogData => {
                write!(f, "ecr-public:GetRegistryCatalogData")
            }
            EcrPublicActions::GetRepositoryCatalogData => {
                write!(f, "ecr-public:GetRepositoryCatalogData")
            }
            EcrPublicActions::GetRepositoryPolicy => write!(f, "ecr-public:GetRepositoryPolicy"),
            EcrPublicActions::InitiateLayerUpload => write!(f, "ecr-public:InitiateLayerUpload"),
            EcrPublicActions::ListTagsForResource => write!(f, "ecr-public:ListTagsForResource"),
            EcrPublicActions::PutImage => write!(f, "ecr-public:PutImage"),
            EcrPublicActions::PutRegistryCatalogData => {
                write!(f, "ecr-public:PutRegistryCatalogData")
            }
            EcrPublicActions::PutRepositoryCatalogData => {
                write!(f, "ecr-public:PutRepositoryCatalogData")
            }
            EcrPublicActions::SetRepositoryPolicy => write!(f, "ecr-public:SetRepositoryPolicy"),
            EcrPublicActions::TagResource => write!(f, "ecr-public:TagResource"),
            EcrPublicActions::UntagResource => write!(f, "ecr-public:UntagResource"),
            EcrPublicActions::UploadLayerPart => write!(f, "ecr-public:UploadLayerPart"),
        }
    }
}
