// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum MediastoreActions {
    CreateContainer,
    DeleteContainer,
    DeleteContainerPolicy,
    DeleteCorsPolicy,
    DeleteLifecyclePolicy,
    DeleteMetricPolicy,
    DeleteObject,
    DescribeContainer,
    DescribeObject,
    GetContainerPolicy,
    GetCorsPolicy,
    GetLifecyclePolicy,
    GetMetricPolicy,
    GetObject,
    ListContainers,
    ListItems,
    ListTagsForResource,
    PutContainerPolicy,
    PutCorsPolicy,
    PutLifecyclePolicy,
    PutMetricPolicy,
    PutObject,
    StartAccessLogging,
    StopAccessLogging,
    TagResource,
    UntagResource,
}
impl std::fmt::Display for MediastoreActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MediastoreActions::CreateContainer => write!(f, "mediastore:CreateContainer"),
            MediastoreActions::DeleteContainer => write!(f, "mediastore:DeleteContainer"),
            MediastoreActions::DeleteContainerPolicy => {
                write!(f, "mediastore:DeleteContainerPolicy")
            }
            MediastoreActions::DeleteCorsPolicy => write!(f, "mediastore:DeleteCorsPolicy"),
            MediastoreActions::DeleteLifecyclePolicy => {
                write!(f, "mediastore:DeleteLifecyclePolicy")
            }
            MediastoreActions::DeleteMetricPolicy => write!(f, "mediastore:DeleteMetricPolicy"),
            MediastoreActions::DeleteObject => write!(f, "mediastore:DeleteObject"),
            MediastoreActions::DescribeContainer => write!(f, "mediastore:DescribeContainer"),
            MediastoreActions::DescribeObject => write!(f, "mediastore:DescribeObject"),
            MediastoreActions::GetContainerPolicy => write!(f, "mediastore:GetContainerPolicy"),
            MediastoreActions::GetCorsPolicy => write!(f, "mediastore:GetCorsPolicy"),
            MediastoreActions::GetLifecyclePolicy => write!(f, "mediastore:GetLifecyclePolicy"),
            MediastoreActions::GetMetricPolicy => write!(f, "mediastore:GetMetricPolicy"),
            MediastoreActions::GetObject => write!(f, "mediastore:GetObject"),
            MediastoreActions::ListContainers => write!(f, "mediastore:ListContainers"),
            MediastoreActions::ListItems => write!(f, "mediastore:ListItems"),
            MediastoreActions::ListTagsForResource => write!(f, "mediastore:ListTagsForResource"),
            MediastoreActions::PutContainerPolicy => write!(f, "mediastore:PutContainerPolicy"),
            MediastoreActions::PutCorsPolicy => write!(f, "mediastore:PutCorsPolicy"),
            MediastoreActions::PutLifecyclePolicy => write!(f, "mediastore:PutLifecyclePolicy"),
            MediastoreActions::PutMetricPolicy => write!(f, "mediastore:PutMetricPolicy"),
            MediastoreActions::PutObject => write!(f, "mediastore:PutObject"),
            MediastoreActions::StartAccessLogging => write!(f, "mediastore:StartAccessLogging"),
            MediastoreActions::StopAccessLogging => write!(f, "mediastore:StopAccessLogging"),
            MediastoreActions::TagResource => write!(f, "mediastore:TagResource"),
            MediastoreActions::UntagResource => write!(f, "mediastore:UntagResource"),
        }
    }
}
