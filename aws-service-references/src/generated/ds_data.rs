// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum DsDataActions {
    AddGroupMember,
    CreateGroup,
    CreateUser,
    DeleteGroup,
    DeleteUser,
    DescribeGroup,
    DescribeUser,
    DisableUser,
    ListGroupMembers,
    ListGroups,
    ListGroupsForMember,
    ListUsers,
    RemoveGroupMember,
    SearchGroups,
    SearchUsers,
    UpdateGroup,
    UpdateUser,
}
impl std::fmt::Display for DsDataActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DsDataActions::AddGroupMember => write!(f, "ds-data:AddGroupMember"),
            DsDataActions::CreateGroup => write!(f, "ds-data:CreateGroup"),
            DsDataActions::CreateUser => write!(f, "ds-data:CreateUser"),
            DsDataActions::DeleteGroup => write!(f, "ds-data:DeleteGroup"),
            DsDataActions::DeleteUser => write!(f, "ds-data:DeleteUser"),
            DsDataActions::DescribeGroup => write!(f, "ds-data:DescribeGroup"),
            DsDataActions::DescribeUser => write!(f, "ds-data:DescribeUser"),
            DsDataActions::DisableUser => write!(f, "ds-data:DisableUser"),
            DsDataActions::ListGroupMembers => write!(f, "ds-data:ListGroupMembers"),
            DsDataActions::ListGroups => write!(f, "ds-data:ListGroups"),
            DsDataActions::ListGroupsForMember => write!(f, "ds-data:ListGroupsForMember"),
            DsDataActions::ListUsers => write!(f, "ds-data:ListUsers"),
            DsDataActions::RemoveGroupMember => write!(f, "ds-data:RemoveGroupMember"),
            DsDataActions::SearchGroups => write!(f, "ds-data:SearchGroups"),
            DsDataActions::SearchUsers => write!(f, "ds-data:SearchUsers"),
            DsDataActions::UpdateGroup => write!(f, "ds-data:UpdateGroup"),
            DsDataActions::UpdateUser => write!(f, "ds-data:UpdateUser"),
        }
    }
}
