// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum CodestarActions {
    AssociateTeamMember,
    CreateProject,
    CreateUserProfile,
    DeleteExtendedAccess,
    DeleteProject,
    DeleteUserProfile,
    DescribeProject,
    DescribeUserProfile,
    DisassociateTeamMember,
    GetExtendedAccess,
    ListProjects,
    ListResources,
    ListTagsForProject,
    ListTeamMembers,
    ListUserProfiles,
    PutExtendedAccess,
    TagProject,
    UntagProject,
    UpdateProject,
    UpdateTeamMember,
    UpdateUserProfile,
    VerifyServiceRole,
}
impl std::fmt::Display for CodestarActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CodestarActions::AssociateTeamMember => write!(f, "codestar:AssociateTeamMember"),
            CodestarActions::CreateProject => write!(f, "codestar:CreateProject"),
            CodestarActions::CreateUserProfile => write!(f, "codestar:CreateUserProfile"),
            CodestarActions::DeleteExtendedAccess => write!(f, "codestar:DeleteExtendedAccess"),
            CodestarActions::DeleteProject => write!(f, "codestar:DeleteProject"),
            CodestarActions::DeleteUserProfile => write!(f, "codestar:DeleteUserProfile"),
            CodestarActions::DescribeProject => write!(f, "codestar:DescribeProject"),
            CodestarActions::DescribeUserProfile => write!(f, "codestar:DescribeUserProfile"),
            CodestarActions::DisassociateTeamMember => write!(f, "codestar:DisassociateTeamMember"),
            CodestarActions::GetExtendedAccess => write!(f, "codestar:GetExtendedAccess"),
            CodestarActions::ListProjects => write!(f, "codestar:ListProjects"),
            CodestarActions::ListResources => write!(f, "codestar:ListResources"),
            CodestarActions::ListTagsForProject => write!(f, "codestar:ListTagsForProject"),
            CodestarActions::ListTeamMembers => write!(f, "codestar:ListTeamMembers"),
            CodestarActions::ListUserProfiles => write!(f, "codestar:ListUserProfiles"),
            CodestarActions::PutExtendedAccess => write!(f, "codestar:PutExtendedAccess"),
            CodestarActions::TagProject => write!(f, "codestar:TagProject"),
            CodestarActions::UntagProject => write!(f, "codestar:UntagProject"),
            CodestarActions::UpdateProject => write!(f, "codestar:UpdateProject"),
            CodestarActions::UpdateTeamMember => write!(f, "codestar:UpdateTeamMember"),
            CodestarActions::UpdateUserProfile => write!(f, "codestar:UpdateUserProfile"),
            CodestarActions::VerifyServiceRole => write!(f, "codestar:VerifyServiceRole"),
        }
    }
}
