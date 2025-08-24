// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum OpsworksCmActions {
    AssociateNode,
    CreateBackup,
    CreateServer,
    DeleteBackup,
    DeleteServer,
    DescribeAccountAttributes,
    DescribeBackups,
    DescribeEvents,
    DescribeNodeAssociationStatus,
    DescribeServers,
    DisassociateNode,
    ExportServerEngineAttribute,
    ListTagsForResource,
    RestoreServer,
    StartMaintenance,
    TagResource,
    UntagResource,
    UpdateServer,
    UpdateServerEngineAttributes,
}
impl std::fmt::Display for OpsworksCmActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OpsworksCmActions::AssociateNode => write!(f, "opsworks-cm:AssociateNode"),
            OpsworksCmActions::CreateBackup => write!(f, "opsworks-cm:CreateBackup"),
            OpsworksCmActions::CreateServer => write!(f, "opsworks-cm:CreateServer"),
            OpsworksCmActions::DeleteBackup => write!(f, "opsworks-cm:DeleteBackup"),
            OpsworksCmActions::DeleteServer => write!(f, "opsworks-cm:DeleteServer"),
            OpsworksCmActions::DescribeAccountAttributes => {
                write!(f, "opsworks-cm:DescribeAccountAttributes")
            }
            OpsworksCmActions::DescribeBackups => write!(f, "opsworks-cm:DescribeBackups"),
            OpsworksCmActions::DescribeEvents => write!(f, "opsworks-cm:DescribeEvents"),
            OpsworksCmActions::DescribeNodeAssociationStatus => {
                write!(f, "opsworks-cm:DescribeNodeAssociationStatus")
            }
            OpsworksCmActions::DescribeServers => write!(f, "opsworks-cm:DescribeServers"),
            OpsworksCmActions::DisassociateNode => write!(f, "opsworks-cm:DisassociateNode"),
            OpsworksCmActions::ExportServerEngineAttribute => {
                write!(f, "opsworks-cm:ExportServerEngineAttribute")
            }
            OpsworksCmActions::ListTagsForResource => write!(f, "opsworks-cm:ListTagsForResource"),
            OpsworksCmActions::RestoreServer => write!(f, "opsworks-cm:RestoreServer"),
            OpsworksCmActions::StartMaintenance => write!(f, "opsworks-cm:StartMaintenance"),
            OpsworksCmActions::TagResource => write!(f, "opsworks-cm:TagResource"),
            OpsworksCmActions::UntagResource => write!(f, "opsworks-cm:UntagResource"),
            OpsworksCmActions::UpdateServer => write!(f, "opsworks-cm:UpdateServer"),
            OpsworksCmActions::UpdateServerEngineAttributes => {
                write!(f, "opsworks-cm:UpdateServerEngineAttributes")
            }
        }
    }
}
