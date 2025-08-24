// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum ResourceGroupsActions {
    AssociateResource,
    CancelTagSyncTask,
    CreateGroup,
    DeleteGroup,
    DeleteGroupPolicy,
    DisassociateResource,
    GetAccountSettings,
    GetGroup,
    GetGroupConfiguration,
    GetGroupPolicy,
    GetGroupQuery,
    GetTagSyncTask,
    GetTags,
    GroupResources,
    ListGroupResources,
    ListGroupingStatuses,
    ListGroups,
    ListResourceTypes,
    ListTagSyncTasks,
    PutGroupConfiguration,
    PutGroupPolicy,
    SearchResources,
    StartTagSyncTask,
    Tag,
    UngroupResources,
    Untag,
    UpdateAccountSettings,
    UpdateGroup,
    UpdateGroupQuery,
}
impl std::fmt::Display for ResourceGroupsActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResourceGroupsActions::AssociateResource => {
                write!(f, "resource-groups:AssociateResource")
            }
            ResourceGroupsActions::CancelTagSyncTask => {
                write!(f, "resource-groups:CancelTagSyncTask")
            }
            ResourceGroupsActions::CreateGroup => write!(f, "resource-groups:CreateGroup"),
            ResourceGroupsActions::DeleteGroup => write!(f, "resource-groups:DeleteGroup"),
            ResourceGroupsActions::DeleteGroupPolicy => {
                write!(f, "resource-groups:DeleteGroupPolicy")
            }
            ResourceGroupsActions::DisassociateResource => {
                write!(f, "resource-groups:DisassociateResource")
            }
            ResourceGroupsActions::GetAccountSettings => {
                write!(f, "resource-groups:GetAccountSettings")
            }
            ResourceGroupsActions::GetGroup => write!(f, "resource-groups:GetGroup"),
            ResourceGroupsActions::GetGroupConfiguration => {
                write!(f, "resource-groups:GetGroupConfiguration")
            }
            ResourceGroupsActions::GetGroupPolicy => write!(f, "resource-groups:GetGroupPolicy"),
            ResourceGroupsActions::GetGroupQuery => write!(f, "resource-groups:GetGroupQuery"),
            ResourceGroupsActions::GetTagSyncTask => write!(f, "resource-groups:GetTagSyncTask"),
            ResourceGroupsActions::GetTags => write!(f, "resource-groups:GetTags"),
            ResourceGroupsActions::GroupResources => write!(f, "resource-groups:GroupResources"),
            ResourceGroupsActions::ListGroupResources => {
                write!(f, "resource-groups:ListGroupResources")
            }
            ResourceGroupsActions::ListGroupingStatuses => {
                write!(f, "resource-groups:ListGroupingStatuses")
            }
            ResourceGroupsActions::ListGroups => write!(f, "resource-groups:ListGroups"),
            ResourceGroupsActions::ListResourceTypes => {
                write!(f, "resource-groups:ListResourceTypes")
            }
            ResourceGroupsActions::ListTagSyncTasks => {
                write!(f, "resource-groups:ListTagSyncTasks")
            }
            ResourceGroupsActions::PutGroupConfiguration => {
                write!(f, "resource-groups:PutGroupConfiguration")
            }
            ResourceGroupsActions::PutGroupPolicy => write!(f, "resource-groups:PutGroupPolicy"),
            ResourceGroupsActions::SearchResources => write!(f, "resource-groups:SearchResources"),
            ResourceGroupsActions::StartTagSyncTask => {
                write!(f, "resource-groups:StartTagSyncTask")
            }
            ResourceGroupsActions::Tag => write!(f, "resource-groups:Tag"),
            ResourceGroupsActions::UngroupResources => {
                write!(f, "resource-groups:UngroupResources")
            }
            ResourceGroupsActions::Untag => write!(f, "resource-groups:Untag"),
            ResourceGroupsActions::UpdateAccountSettings => {
                write!(f, "resource-groups:UpdateAccountSettings")
            }
            ResourceGroupsActions::UpdateGroup => write!(f, "resource-groups:UpdateGroup"),
            ResourceGroupsActions::UpdateGroupQuery => {
                write!(f, "resource-groups:UpdateGroupQuery")
            }
        }
    }
}
