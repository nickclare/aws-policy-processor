// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum MghActions {
    AcceptConnection,
    AssociateAutomationUnitRole,
    AssociateCreatedArtifact,
    AssociateDiscoveredResource,
    AssociateSourceResource,
    BatchAssociateIamRoleWithConnection,
    BatchDisassociateIamRoleFromConnection,
    CreateAutomationRun,
    CreateAutomationUnit,
    CreateHomeRegionControl,
    CreateProgressUpdateStream,
    DeleteAutomationRun,
    DeleteAutomationUnit,
    DeleteConnection,
    DeleteHomeRegionControl,
    DeleteProgressUpdateStream,
    DescribeApplicationState,
    DescribeAutomationRun,
    DescribeAutomationUnit,
    DescribeHomeRegionControls,
    DescribeMigrationTask,
    DisassociateAutomationUnitRole,
    DisassociateCreatedArtifact,
    DisassociateDiscoveredResource,
    DisassociateSourceResource,
    GetConnection,
    GetHomeRegion,
    ImportMigrationTask,
    ListApplicationStates,
    ListAutomationRuns,
    ListAutomationUnits,
    ListConnectionRoles,
    ListConnections,
    ListCreatedArtifacts,
    ListDiscoveredResources,
    ListMigrationTaskUpdates,
    ListMigrationTasks,
    ListProgressUpdateStreams,
    ListSourceResources,
    ListTagsForResource,
    NotifyApplicationState,
    NotifyMigrationTaskState,
    PutResourceAttributes,
    RejectConnection,
    TagResource,
    UntagResource,
}
impl std::fmt::Display for MghActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MghActions::AcceptConnection => write!(f, "mgh:AcceptConnection"),
            MghActions::AssociateAutomationUnitRole => write!(f, "mgh:AssociateAutomationUnitRole"),
            MghActions::AssociateCreatedArtifact => write!(f, "mgh:AssociateCreatedArtifact"),
            MghActions::AssociateDiscoveredResource => write!(f, "mgh:AssociateDiscoveredResource"),
            MghActions::AssociateSourceResource => write!(f, "mgh:AssociateSourceResource"),
            MghActions::BatchAssociateIamRoleWithConnection => {
                write!(f, "mgh:BatchAssociateIamRoleWithConnection")
            }
            MghActions::BatchDisassociateIamRoleFromConnection => {
                write!(f, "mgh:BatchDisassociateIamRoleFromConnection")
            }
            MghActions::CreateAutomationRun => write!(f, "mgh:CreateAutomationRun"),
            MghActions::CreateAutomationUnit => write!(f, "mgh:CreateAutomationUnit"),
            MghActions::CreateHomeRegionControl => write!(f, "mgh:CreateHomeRegionControl"),
            MghActions::CreateProgressUpdateStream => write!(f, "mgh:CreateProgressUpdateStream"),
            MghActions::DeleteAutomationRun => write!(f, "mgh:DeleteAutomationRun"),
            MghActions::DeleteAutomationUnit => write!(f, "mgh:DeleteAutomationUnit"),
            MghActions::DeleteConnection => write!(f, "mgh:DeleteConnection"),
            MghActions::DeleteHomeRegionControl => write!(f, "mgh:DeleteHomeRegionControl"),
            MghActions::DeleteProgressUpdateStream => write!(f, "mgh:DeleteProgressUpdateStream"),
            MghActions::DescribeApplicationState => write!(f, "mgh:DescribeApplicationState"),
            MghActions::DescribeAutomationRun => write!(f, "mgh:DescribeAutomationRun"),
            MghActions::DescribeAutomationUnit => write!(f, "mgh:DescribeAutomationUnit"),
            MghActions::DescribeHomeRegionControls => write!(f, "mgh:DescribeHomeRegionControls"),
            MghActions::DescribeMigrationTask => write!(f, "mgh:DescribeMigrationTask"),
            MghActions::DisassociateAutomationUnitRole => {
                write!(f, "mgh:DisassociateAutomationUnitRole")
            }
            MghActions::DisassociateCreatedArtifact => write!(f, "mgh:DisassociateCreatedArtifact"),
            MghActions::DisassociateDiscoveredResource => {
                write!(f, "mgh:DisassociateDiscoveredResource")
            }
            MghActions::DisassociateSourceResource => write!(f, "mgh:DisassociateSourceResource"),
            MghActions::GetConnection => write!(f, "mgh:GetConnection"),
            MghActions::GetHomeRegion => write!(f, "mgh:GetHomeRegion"),
            MghActions::ImportMigrationTask => write!(f, "mgh:ImportMigrationTask"),
            MghActions::ListApplicationStates => write!(f, "mgh:ListApplicationStates"),
            MghActions::ListAutomationRuns => write!(f, "mgh:ListAutomationRuns"),
            MghActions::ListAutomationUnits => write!(f, "mgh:ListAutomationUnits"),
            MghActions::ListConnectionRoles => write!(f, "mgh:ListConnectionRoles"),
            MghActions::ListConnections => write!(f, "mgh:ListConnections"),
            MghActions::ListCreatedArtifacts => write!(f, "mgh:ListCreatedArtifacts"),
            MghActions::ListDiscoveredResources => write!(f, "mgh:ListDiscoveredResources"),
            MghActions::ListMigrationTaskUpdates => write!(f, "mgh:ListMigrationTaskUpdates"),
            MghActions::ListMigrationTasks => write!(f, "mgh:ListMigrationTasks"),
            MghActions::ListProgressUpdateStreams => write!(f, "mgh:ListProgressUpdateStreams"),
            MghActions::ListSourceResources => write!(f, "mgh:ListSourceResources"),
            MghActions::ListTagsForResource => write!(f, "mgh:ListTagsForResource"),
            MghActions::NotifyApplicationState => write!(f, "mgh:NotifyApplicationState"),
            MghActions::NotifyMigrationTaskState => write!(f, "mgh:NotifyMigrationTaskState"),
            MghActions::PutResourceAttributes => write!(f, "mgh:PutResourceAttributes"),
            MghActions::RejectConnection => write!(f, "mgh:RejectConnection"),
            MghActions::TagResource => write!(f, "mgh:TagResource"),
            MghActions::UntagResource => write!(f, "mgh:UntagResource"),
        }
    }
}
