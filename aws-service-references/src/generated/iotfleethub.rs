// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum IotfleethubActions {
    CreateApplication,
    DeleteApplication,
    DescribeApplication,
    ListApplications,
    ListTagsForResource,
    TagResource,
    UntagResource,
    UpdateApplication,
}
impl std::fmt::Display for IotfleethubActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IotfleethubActions::CreateApplication => write!(f, "iotfleethub:CreateApplication"),
            IotfleethubActions::DeleteApplication => write!(f, "iotfleethub:DeleteApplication"),
            IotfleethubActions::DescribeApplication => write!(f, "iotfleethub:DescribeApplication"),
            IotfleethubActions::ListApplications => write!(f, "iotfleethub:ListApplications"),
            IotfleethubActions::ListTagsForResource => write!(f, "iotfleethub:ListTagsForResource"),
            IotfleethubActions::TagResource => write!(f, "iotfleethub:TagResource"),
            IotfleethubActions::UntagResource => write!(f, "iotfleethub:UntagResource"),
            IotfleethubActions::UpdateApplication => write!(f, "iotfleethub:UpdateApplication"),
        }
    }
}
