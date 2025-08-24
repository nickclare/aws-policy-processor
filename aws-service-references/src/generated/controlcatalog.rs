// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum ControlcatalogActions {
    GetControl,
    ListCommonControls,
    ListControlMappings,
    ListControls,
    ListDomains,
    ListObjectives,
}
impl std::fmt::Display for ControlcatalogActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ControlcatalogActions::GetControl => write!(f, "controlcatalog:GetControl"),
            ControlcatalogActions::ListCommonControls => {
                write!(f, "controlcatalog:ListCommonControls")
            }
            ControlcatalogActions::ListControlMappings => {
                write!(f, "controlcatalog:ListControlMappings")
            }
            ControlcatalogActions::ListControls => write!(f, "controlcatalog:ListControls"),
            ControlcatalogActions::ListDomains => write!(f, "controlcatalog:ListDomains"),
            ControlcatalogActions::ListObjectives => write!(f, "controlcatalog:ListObjectives"),
        }
    }
}
