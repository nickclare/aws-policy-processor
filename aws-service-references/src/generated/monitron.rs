// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum MonitronActions {
    AssociateProjectAdminUser,
    CreateProject,
    CreateProjectUserAssociation,
    CreateUserAccessRoleAssociation,
    DeleteProject,
    DeleteProjectUserAssociation,
    DeleteUserAccessRoleAssociation,
    DisassociateProjectAdminUser,
    GetProject,
    GetProjectAdminUser,
    ListProjectAdminUsers,
    ListProjectUserAssociations,
    ListProjects,
    ListTagsForResource,
    ListUserAccessRoleAssociations,
    TagResource,
    UntagResource,
    UpdateProject,
}
impl std::fmt::Display for MonitronActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MonitronActions::AssociateProjectAdminUser => {
                write!(f, "monitron:AssociateProjectAdminUser")
            }
            MonitronActions::CreateProject => write!(f, "monitron:CreateProject"),
            MonitronActions::CreateProjectUserAssociation => {
                write!(f, "monitron:CreateProjectUserAssociation")
            }
            MonitronActions::CreateUserAccessRoleAssociation => {
                write!(f, "monitron:CreateUserAccessRoleAssociation")
            }
            MonitronActions::DeleteProject => write!(f, "monitron:DeleteProject"),
            MonitronActions::DeleteProjectUserAssociation => {
                write!(f, "monitron:DeleteProjectUserAssociation")
            }
            MonitronActions::DeleteUserAccessRoleAssociation => {
                write!(f, "monitron:DeleteUserAccessRoleAssociation")
            }
            MonitronActions::DisassociateProjectAdminUser => {
                write!(f, "monitron:DisassociateProjectAdminUser")
            }
            MonitronActions::GetProject => write!(f, "monitron:GetProject"),
            MonitronActions::GetProjectAdminUser => write!(f, "monitron:GetProjectAdminUser"),
            MonitronActions::ListProjectAdminUsers => write!(f, "monitron:ListProjectAdminUsers"),
            MonitronActions::ListProjectUserAssociations => {
                write!(f, "monitron:ListProjectUserAssociations")
            }
            MonitronActions::ListProjects => write!(f, "monitron:ListProjects"),
            MonitronActions::ListTagsForResource => write!(f, "monitron:ListTagsForResource"),
            MonitronActions::ListUserAccessRoleAssociations => {
                write!(f, "monitron:ListUserAccessRoleAssociations")
            }
            MonitronActions::TagResource => write!(f, "monitron:TagResource"),
            MonitronActions::UntagResource => write!(f, "monitron:UntagResource"),
            MonitronActions::UpdateProject => write!(f, "monitron:UpdateProject"),
        }
    }
}
