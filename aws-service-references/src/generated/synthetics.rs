// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum SyntheticsActions {
    AssociateResource,
    CreateCanary,
    CreateGroup,
    DeleteCanary,
    DeleteGroup,
    DescribeCanaries,
    DescribeCanariesLastRun,
    DescribeRuntimeVersions,
    DisassociateResource,
    GetCanary,
    GetCanaryRuns,
    GetGroup,
    ListAssociatedGroups,
    ListGroupResources,
    ListGroups,
    ListTagsForResource,
    StartCanary,
    StartCanaryDryRun,
    StopCanary,
    TagResource,
    UntagResource,
    UpdateCanary,
}
impl std::fmt::Display for SyntheticsActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SyntheticsActions::AssociateResource => write!(f, "synthetics:AssociateResource"),
            SyntheticsActions::CreateCanary => write!(f, "synthetics:CreateCanary"),
            SyntheticsActions::CreateGroup => write!(f, "synthetics:CreateGroup"),
            SyntheticsActions::DeleteCanary => write!(f, "synthetics:DeleteCanary"),
            SyntheticsActions::DeleteGroup => write!(f, "synthetics:DeleteGroup"),
            SyntheticsActions::DescribeCanaries => write!(f, "synthetics:DescribeCanaries"),
            SyntheticsActions::DescribeCanariesLastRun => {
                write!(f, "synthetics:DescribeCanariesLastRun")
            }
            SyntheticsActions::DescribeRuntimeVersions => {
                write!(f, "synthetics:DescribeRuntimeVersions")
            }
            SyntheticsActions::DisassociateResource => write!(f, "synthetics:DisassociateResource"),
            SyntheticsActions::GetCanary => write!(f, "synthetics:GetCanary"),
            SyntheticsActions::GetCanaryRuns => write!(f, "synthetics:GetCanaryRuns"),
            SyntheticsActions::GetGroup => write!(f, "synthetics:GetGroup"),
            SyntheticsActions::ListAssociatedGroups => write!(f, "synthetics:ListAssociatedGroups"),
            SyntheticsActions::ListGroupResources => write!(f, "synthetics:ListGroupResources"),
            SyntheticsActions::ListGroups => write!(f, "synthetics:ListGroups"),
            SyntheticsActions::ListTagsForResource => write!(f, "synthetics:ListTagsForResource"),
            SyntheticsActions::StartCanary => write!(f, "synthetics:StartCanary"),
            SyntheticsActions::StartCanaryDryRun => write!(f, "synthetics:StartCanaryDryRun"),
            SyntheticsActions::StopCanary => write!(f, "synthetics:StopCanary"),
            SyntheticsActions::TagResource => write!(f, "synthetics:TagResource"),
            SyntheticsActions::UntagResource => write!(f, "synthetics:UntagResource"),
            SyntheticsActions::UpdateCanary => write!(f, "synthetics:UpdateCanary"),
        }
    }
}
