// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum RefactorSpacesActions {
    CreateApplication,
    CreateEnvironment,
    CreateRoute,
    CreateService,
    DeleteApplication,
    DeleteEnvironment,
    DeleteResourcePolicy,
    DeleteRoute,
    DeleteService,
    GetApplication,
    GetEnvironment,
    GetResourcePolicy,
    GetRoute,
    GetService,
    ListApplications,
    ListEnvironmentVpcs,
    ListEnvironments,
    ListRoutes,
    ListServices,
    ListTagsForResource,
    PutResourcePolicy,
    TagResource,
    UntagResource,
    UpdateRoute,
}
impl std::fmt::Display for RefactorSpacesActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RefactorSpacesActions::CreateApplication => {
                write!(f, "refactor-spaces:CreateApplication")
            }
            RefactorSpacesActions::CreateEnvironment => {
                write!(f, "refactor-spaces:CreateEnvironment")
            }
            RefactorSpacesActions::CreateRoute => write!(f, "refactor-spaces:CreateRoute"),
            RefactorSpacesActions::CreateService => write!(f, "refactor-spaces:CreateService"),
            RefactorSpacesActions::DeleteApplication => {
                write!(f, "refactor-spaces:DeleteApplication")
            }
            RefactorSpacesActions::DeleteEnvironment => {
                write!(f, "refactor-spaces:DeleteEnvironment")
            }
            RefactorSpacesActions::DeleteResourcePolicy => {
                write!(f, "refactor-spaces:DeleteResourcePolicy")
            }
            RefactorSpacesActions::DeleteRoute => write!(f, "refactor-spaces:DeleteRoute"),
            RefactorSpacesActions::DeleteService => write!(f, "refactor-spaces:DeleteService"),
            RefactorSpacesActions::GetApplication => write!(f, "refactor-spaces:GetApplication"),
            RefactorSpacesActions::GetEnvironment => write!(f, "refactor-spaces:GetEnvironment"),
            RefactorSpacesActions::GetResourcePolicy => {
                write!(f, "refactor-spaces:GetResourcePolicy")
            }
            RefactorSpacesActions::GetRoute => write!(f, "refactor-spaces:GetRoute"),
            RefactorSpacesActions::GetService => write!(f, "refactor-spaces:GetService"),
            RefactorSpacesActions::ListApplications => {
                write!(f, "refactor-spaces:ListApplications")
            }
            RefactorSpacesActions::ListEnvironmentVpcs => {
                write!(f, "refactor-spaces:ListEnvironmentVpcs")
            }
            RefactorSpacesActions::ListEnvironments => {
                write!(f, "refactor-spaces:ListEnvironments")
            }
            RefactorSpacesActions::ListRoutes => write!(f, "refactor-spaces:ListRoutes"),
            RefactorSpacesActions::ListServices => write!(f, "refactor-spaces:ListServices"),
            RefactorSpacesActions::ListTagsForResource => {
                write!(f, "refactor-spaces:ListTagsForResource")
            }
            RefactorSpacesActions::PutResourcePolicy => {
                write!(f, "refactor-spaces:PutResourcePolicy")
            }
            RefactorSpacesActions::TagResource => write!(f, "refactor-spaces:TagResource"),
            RefactorSpacesActions::UntagResource => write!(f, "refactor-spaces:UntagResource"),
            RefactorSpacesActions::UpdateRoute => write!(f, "refactor-spaces:UpdateRoute"),
        }
    }
}
