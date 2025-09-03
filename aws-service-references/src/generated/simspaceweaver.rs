// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum SimspaceweaverActions {
    CreateSnapshot,
    DeleteApp,
    DeleteSimulation,
    DescribeApp,
    DescribeSimulation,
    ListApps,
    ListSimulations,
    ListTagsForResource,
    StartApp,
    StartClock,
    StartSimulation,
    StopApp,
    StopClock,
    StopSimulation,
    TagResource,
    UntagResource,
}
impl std::fmt::Display for SimspaceweaverActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SimspaceweaverActions::CreateSnapshot => write!(f, "simspaceweaver:CreateSnapshot"),
            SimspaceweaverActions::DeleteApp => write!(f, "simspaceweaver:DeleteApp"),
            SimspaceweaverActions::DeleteSimulation => write!(f, "simspaceweaver:DeleteSimulation"),
            SimspaceweaverActions::DescribeApp => write!(f, "simspaceweaver:DescribeApp"),
            SimspaceweaverActions::DescribeSimulation => {
                write!(f, "simspaceweaver:DescribeSimulation")
            }
            SimspaceweaverActions::ListApps => write!(f, "simspaceweaver:ListApps"),
            SimspaceweaverActions::ListSimulations => write!(f, "simspaceweaver:ListSimulations"),
            SimspaceweaverActions::ListTagsForResource => {
                write!(f, "simspaceweaver:ListTagsForResource")
            }
            SimspaceweaverActions::StartApp => write!(f, "simspaceweaver:StartApp"),
            SimspaceweaverActions::StartClock => write!(f, "simspaceweaver:StartClock"),
            SimspaceweaverActions::StartSimulation => write!(f, "simspaceweaver:StartSimulation"),
            SimspaceweaverActions::StopApp => write!(f, "simspaceweaver:StopApp"),
            SimspaceweaverActions::StopClock => write!(f, "simspaceweaver:StopClock"),
            SimspaceweaverActions::StopSimulation => write!(f, "simspaceweaver:StopSimulation"),
            SimspaceweaverActions::TagResource => write!(f, "simspaceweaver:TagResource"),
            SimspaceweaverActions::UntagResource => write!(f, "simspaceweaver:UntagResource"),
        }
    }
}
