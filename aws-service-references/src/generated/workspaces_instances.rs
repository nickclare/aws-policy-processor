// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum WorkspacesInstancesActions {
    AssociateVolume,
    CreateVolume,
    CreateWorkspaceInstance,
    DeleteVolume,
    DeleteWorkspaceInstance,
    DisassociateVolume,
    GetWorkspaceInstance,
    ListInstanceTypes,
    ListRegions,
    ListTagsForResource,
    ListWorkspaceInstances,
    TagResource,
    UntagResource,
}
impl std::fmt::Display for WorkspacesInstancesActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WorkspacesInstancesActions::AssociateVolume => {
                write!(f, "workspaces-instances:AssociateVolume")
            }
            WorkspacesInstancesActions::CreateVolume => {
                write!(f, "workspaces-instances:CreateVolume")
            }
            WorkspacesInstancesActions::CreateWorkspaceInstance => {
                write!(f, "workspaces-instances:CreateWorkspaceInstance")
            }
            WorkspacesInstancesActions::DeleteVolume => {
                write!(f, "workspaces-instances:DeleteVolume")
            }
            WorkspacesInstancesActions::DeleteWorkspaceInstance => {
                write!(f, "workspaces-instances:DeleteWorkspaceInstance")
            }
            WorkspacesInstancesActions::DisassociateVolume => {
                write!(f, "workspaces-instances:DisassociateVolume")
            }
            WorkspacesInstancesActions::GetWorkspaceInstance => {
                write!(f, "workspaces-instances:GetWorkspaceInstance")
            }
            WorkspacesInstancesActions::ListInstanceTypes => {
                write!(f, "workspaces-instances:ListInstanceTypes")
            }
            WorkspacesInstancesActions::ListRegions => {
                write!(f, "workspaces-instances:ListRegions")
            }
            WorkspacesInstancesActions::ListTagsForResource => {
                write!(f, "workspaces-instances:ListTagsForResource")
            }
            WorkspacesInstancesActions::ListWorkspaceInstances => {
                write!(f, "workspaces-instances:ListWorkspaceInstances")
            }
            WorkspacesInstancesActions::TagResource => {
                write!(f, "workspaces-instances:TagResource")
            }
            WorkspacesInstancesActions::UntagResource => {
                write!(f, "workspaces-instances:UntagResource")
            }
        }
    }
}
