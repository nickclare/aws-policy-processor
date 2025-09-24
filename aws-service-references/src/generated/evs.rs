// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum EvsActions {
    AssociateEipToVlan,
    CreateEnvironment,
    CreateEnvironmentHost,
    DeleteEnvironment,
    DeleteEnvironmentHost,
    DisassociateEipFromVlan,
    GetEnvironment,
    ListEnvironmentHosts,
    ListEnvironmentVlans,
    ListEnvironments,
    ListTagsForResource,
    TagResource,
    UntagResource,
}
impl std::fmt::Display for EvsActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EvsActions::AssociateEipToVlan => write!(f, "evs:AssociateEipToVlan"),
            EvsActions::CreateEnvironment => write!(f, "evs:CreateEnvironment"),
            EvsActions::CreateEnvironmentHost => write!(f, "evs:CreateEnvironmentHost"),
            EvsActions::DeleteEnvironment => write!(f, "evs:DeleteEnvironment"),
            EvsActions::DeleteEnvironmentHost => write!(f, "evs:DeleteEnvironmentHost"),
            EvsActions::DisassociateEipFromVlan => write!(f, "evs:DisassociateEipFromVlan"),
            EvsActions::GetEnvironment => write!(f, "evs:GetEnvironment"),
            EvsActions::ListEnvironmentHosts => write!(f, "evs:ListEnvironmentHosts"),
            EvsActions::ListEnvironmentVlans => write!(f, "evs:ListEnvironmentVlans"),
            EvsActions::ListEnvironments => write!(f, "evs:ListEnvironments"),
            EvsActions::ListTagsForResource => write!(f, "evs:ListTagsForResource"),
            EvsActions::TagResource => write!(f, "evs:TagResource"),
            EvsActions::UntagResource => write!(f, "evs:UntagResource"),
        }
    }
}
