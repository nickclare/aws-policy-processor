// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum ResourceExplorerActions {
    ListResourceTypes,
    ListResources,
    ListTags,
}
impl std::fmt::Display for ResourceExplorerActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResourceExplorerActions::ListResourceTypes => {
                write!(f, "resource-explorer:ListResourceTypes")
            }
            ResourceExplorerActions::ListResources => write!(f, "resource-explorer:ListResources"),
            ResourceExplorerActions::ListTags => write!(f, "resource-explorer:ListTags"),
        }
    }
}
