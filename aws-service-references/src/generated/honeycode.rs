// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum HoneycodeActions {
    ApproveTeamAssociation,
    BatchCreateTableRows,
    BatchDeleteTableRows,
    BatchUpdateTableRows,
    BatchUpsertTableRows,
    CreateTeam,
    CreateTenant,
    DeleteDomains,
    DeregisterGroups,
    DescribeTableDataImportJob,
    DescribeTeam,
    GetScreenData,
    InvokeScreenAutomation,
    ListDomains,
    ListGroups,
    ListTableColumns,
    ListTableRows,
    ListTables,
    ListTagsForResource,
    ListTeamAssociations,
    ListTenants,
    QueryTableRows,
    RegisterDomainForVerification,
    RegisterGroups,
    RejectTeamAssociation,
    RestartDomainVerification,
    StartTableDataImportJob,
    TagResource,
    UntagResource,
    UpdateTeam,
}
impl std::fmt::Display for HoneycodeActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HoneycodeActions::ApproveTeamAssociation => {
                write!(f, "honeycode:ApproveTeamAssociation")
            }
            HoneycodeActions::BatchCreateTableRows => write!(f, "honeycode:BatchCreateTableRows"),
            HoneycodeActions::BatchDeleteTableRows => write!(f, "honeycode:BatchDeleteTableRows"),
            HoneycodeActions::BatchUpdateTableRows => write!(f, "honeycode:BatchUpdateTableRows"),
            HoneycodeActions::BatchUpsertTableRows => write!(f, "honeycode:BatchUpsertTableRows"),
            HoneycodeActions::CreateTeam => write!(f, "honeycode:CreateTeam"),
            HoneycodeActions::CreateTenant => write!(f, "honeycode:CreateTenant"),
            HoneycodeActions::DeleteDomains => write!(f, "honeycode:DeleteDomains"),
            HoneycodeActions::DeregisterGroups => write!(f, "honeycode:DeregisterGroups"),
            HoneycodeActions::DescribeTableDataImportJob => {
                write!(f, "honeycode:DescribeTableDataImportJob")
            }
            HoneycodeActions::DescribeTeam => write!(f, "honeycode:DescribeTeam"),
            HoneycodeActions::GetScreenData => write!(f, "honeycode:GetScreenData"),
            HoneycodeActions::InvokeScreenAutomation => {
                write!(f, "honeycode:InvokeScreenAutomation")
            }
            HoneycodeActions::ListDomains => write!(f, "honeycode:ListDomains"),
            HoneycodeActions::ListGroups => write!(f, "honeycode:ListGroups"),
            HoneycodeActions::ListTableColumns => write!(f, "honeycode:ListTableColumns"),
            HoneycodeActions::ListTableRows => write!(f, "honeycode:ListTableRows"),
            HoneycodeActions::ListTables => write!(f, "honeycode:ListTables"),
            HoneycodeActions::ListTagsForResource => write!(f, "honeycode:ListTagsForResource"),
            HoneycodeActions::ListTeamAssociations => write!(f, "honeycode:ListTeamAssociations"),
            HoneycodeActions::ListTenants => write!(f, "honeycode:ListTenants"),
            HoneycodeActions::QueryTableRows => write!(f, "honeycode:QueryTableRows"),
            HoneycodeActions::RegisterDomainForVerification => {
                write!(f, "honeycode:RegisterDomainForVerification")
            }
            HoneycodeActions::RegisterGroups => write!(f, "honeycode:RegisterGroups"),
            HoneycodeActions::RejectTeamAssociation => write!(f, "honeycode:RejectTeamAssociation"),
            HoneycodeActions::RestartDomainVerification => {
                write!(f, "honeycode:RestartDomainVerification")
            }
            HoneycodeActions::StartTableDataImportJob => {
                write!(f, "honeycode:StartTableDataImportJob")
            }
            HoneycodeActions::TagResource => write!(f, "honeycode:TagResource"),
            HoneycodeActions::UntagResource => write!(f, "honeycode:UntagResource"),
            HoneycodeActions::UpdateTeam => write!(f, "honeycode:UpdateTeam"),
        }
    }
}
