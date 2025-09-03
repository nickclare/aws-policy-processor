// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum IdentitystoreActions {
    CreateGroup,
    CreateGroupMembership,
    CreateUser,
    DeleteGroup,
    DeleteGroupMembership,
    DeleteUser,
    DescribeGroup,
    DescribeGroupMembership,
    DescribeUser,
    GetGroupId,
    GetGroupMembershipId,
    GetUserId,
    IsMemberInGroups,
    ListGroupMemberships,
    ListGroupMembershipsForMember,
    ListGroups,
    ListUsers,
    UpdateGroup,
    UpdateUser,
}
impl std::fmt::Display for IdentitystoreActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IdentitystoreActions::CreateGroup => write!(f, "identitystore:CreateGroup"),
            IdentitystoreActions::CreateGroupMembership => {
                write!(f, "identitystore:CreateGroupMembership")
            }
            IdentitystoreActions::CreateUser => write!(f, "identitystore:CreateUser"),
            IdentitystoreActions::DeleteGroup => write!(f, "identitystore:DeleteGroup"),
            IdentitystoreActions::DeleteGroupMembership => {
                write!(f, "identitystore:DeleteGroupMembership")
            }
            IdentitystoreActions::DeleteUser => write!(f, "identitystore:DeleteUser"),
            IdentitystoreActions::DescribeGroup => write!(f, "identitystore:DescribeGroup"),
            IdentitystoreActions::DescribeGroupMembership => {
                write!(f, "identitystore:DescribeGroupMembership")
            }
            IdentitystoreActions::DescribeUser => write!(f, "identitystore:DescribeUser"),
            IdentitystoreActions::GetGroupId => write!(f, "identitystore:GetGroupId"),
            IdentitystoreActions::GetGroupMembershipId => {
                write!(f, "identitystore:GetGroupMembershipId")
            }
            IdentitystoreActions::GetUserId => write!(f, "identitystore:GetUserId"),
            IdentitystoreActions::IsMemberInGroups => write!(f, "identitystore:IsMemberInGroups"),
            IdentitystoreActions::ListGroupMemberships => {
                write!(f, "identitystore:ListGroupMemberships")
            }
            IdentitystoreActions::ListGroupMembershipsForMember => {
                write!(f, "identitystore:ListGroupMembershipsForMember")
            }
            IdentitystoreActions::ListGroups => write!(f, "identitystore:ListGroups"),
            IdentitystoreActions::ListUsers => write!(f, "identitystore:ListUsers"),
            IdentitystoreActions::UpdateGroup => write!(f, "identitystore:UpdateGroup"),
            IdentitystoreActions::UpdateUser => write!(f, "identitystore:UpdateUser"),
        }
    }
}
