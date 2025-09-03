// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum ResourceExplorer2Actions {
    AssociateDefaultView,
    BatchGetView,
    CreateIndex,
    CreateManagedView,
    CreateView,
    DeleteIndex,
    DeleteResourcePolicy,
    DeleteView,
    DisassociateDefaultView,
    GetAccountLevelServiceConfiguration,
    GetDefaultView,
    GetIndex,
    GetManagedView,
    GetResourcePolicy,
    GetView,
    ListIndexes,
    ListIndexesForMembers,
    ListManagedViews,
    ListSupportedResourceTypes,
    ListTagsForResource,
    ListViews,
    PutResourcePolicy,
    Search,
    TagResource,
    UntagResource,
    UpdateIndexType,
    UpdateView,
}
impl std::fmt::Display for ResourceExplorer2Actions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResourceExplorer2Actions::AssociateDefaultView => {
                write!(f, "resource-explorer-2:AssociateDefaultView")
            }
            ResourceExplorer2Actions::BatchGetView => write!(f, "resource-explorer-2:BatchGetView"),
            ResourceExplorer2Actions::CreateIndex => write!(f, "resource-explorer-2:CreateIndex"),
            ResourceExplorer2Actions::CreateManagedView => {
                write!(f, "resource-explorer-2:CreateManagedView")
            }
            ResourceExplorer2Actions::CreateView => write!(f, "resource-explorer-2:CreateView"),
            ResourceExplorer2Actions::DeleteIndex => write!(f, "resource-explorer-2:DeleteIndex"),
            ResourceExplorer2Actions::DeleteResourcePolicy => {
                write!(f, "resource-explorer-2:DeleteResourcePolicy")
            }
            ResourceExplorer2Actions::DeleteView => write!(f, "resource-explorer-2:DeleteView"),
            ResourceExplorer2Actions::DisassociateDefaultView => {
                write!(f, "resource-explorer-2:DisassociateDefaultView")
            }
            ResourceExplorer2Actions::GetAccountLevelServiceConfiguration => {
                write!(f, "resource-explorer-2:GetAccountLevelServiceConfiguration")
            }
            ResourceExplorer2Actions::GetDefaultView => {
                write!(f, "resource-explorer-2:GetDefaultView")
            }
            ResourceExplorer2Actions::GetIndex => write!(f, "resource-explorer-2:GetIndex"),
            ResourceExplorer2Actions::GetManagedView => {
                write!(f, "resource-explorer-2:GetManagedView")
            }
            ResourceExplorer2Actions::GetResourcePolicy => {
                write!(f, "resource-explorer-2:GetResourcePolicy")
            }
            ResourceExplorer2Actions::GetView => write!(f, "resource-explorer-2:GetView"),
            ResourceExplorer2Actions::ListIndexes => write!(f, "resource-explorer-2:ListIndexes"),
            ResourceExplorer2Actions::ListIndexesForMembers => {
                write!(f, "resource-explorer-2:ListIndexesForMembers")
            }
            ResourceExplorer2Actions::ListManagedViews => {
                write!(f, "resource-explorer-2:ListManagedViews")
            }
            ResourceExplorer2Actions::ListSupportedResourceTypes => {
                write!(f, "resource-explorer-2:ListSupportedResourceTypes")
            }
            ResourceExplorer2Actions::ListTagsForResource => {
                write!(f, "resource-explorer-2:ListTagsForResource")
            }
            ResourceExplorer2Actions::ListViews => write!(f, "resource-explorer-2:ListViews"),
            ResourceExplorer2Actions::PutResourcePolicy => {
                write!(f, "resource-explorer-2:PutResourcePolicy")
            }
            ResourceExplorer2Actions::Search => write!(f, "resource-explorer-2:Search"),
            ResourceExplorer2Actions::TagResource => write!(f, "resource-explorer-2:TagResource"),
            ResourceExplorer2Actions::UntagResource => {
                write!(f, "resource-explorer-2:UntagResource")
            }
            ResourceExplorer2Actions::UpdateIndexType => {
                write!(f, "resource-explorer-2:UpdateIndexType")
            }
            ResourceExplorer2Actions::UpdateView => write!(f, "resource-explorer-2:UpdateView"),
        }
    }
}
