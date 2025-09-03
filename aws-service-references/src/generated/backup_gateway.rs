// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum BackupGatewayActions {
    AssociateGatewayToServer,
    Backup,
    CreateGateway,
    DeleteGateway,
    DeleteHypervisor,
    DisassociateGatewayFromServer,
    GetBandwidthRateLimitSchedule,
    GetGateway,
    GetHypervisor,
    GetHypervisorPropertyMappings,
    GetVirtualMachine,
    ImportHypervisorConfiguration,
    ListGateways,
    ListHypervisors,
    ListTagsForResource,
    ListVirtualMachines,
    PutBandwidthRateLimitSchedule,
    PutHypervisorPropertyMappings,
    PutMaintenanceStartTime,
    Restore,
    StartVirtualMachinesMetadataSync,
    TagResource,
    TestHypervisorConfiguration,
    UntagResource,
    UpdateGatewayInformation,
    UpdateGatewaySoftwareNow,
    UpdateHypervisor,
}
impl std::fmt::Display for BackupGatewayActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BackupGatewayActions::AssociateGatewayToServer => {
                write!(f, "backup-gateway:AssociateGatewayToServer")
            }
            BackupGatewayActions::Backup => write!(f, "backup-gateway:Backup"),
            BackupGatewayActions::CreateGateway => write!(f, "backup-gateway:CreateGateway"),
            BackupGatewayActions::DeleteGateway => write!(f, "backup-gateway:DeleteGateway"),
            BackupGatewayActions::DeleteHypervisor => write!(f, "backup-gateway:DeleteHypervisor"),
            BackupGatewayActions::DisassociateGatewayFromServer => {
                write!(f, "backup-gateway:DisassociateGatewayFromServer")
            }
            BackupGatewayActions::GetBandwidthRateLimitSchedule => {
                write!(f, "backup-gateway:GetBandwidthRateLimitSchedule")
            }
            BackupGatewayActions::GetGateway => write!(f, "backup-gateway:GetGateway"),
            BackupGatewayActions::GetHypervisor => write!(f, "backup-gateway:GetHypervisor"),
            BackupGatewayActions::GetHypervisorPropertyMappings => {
                write!(f, "backup-gateway:GetHypervisorPropertyMappings")
            }
            BackupGatewayActions::GetVirtualMachine => {
                write!(f, "backup-gateway:GetVirtualMachine")
            }
            BackupGatewayActions::ImportHypervisorConfiguration => {
                write!(f, "backup-gateway:ImportHypervisorConfiguration")
            }
            BackupGatewayActions::ListGateways => write!(f, "backup-gateway:ListGateways"),
            BackupGatewayActions::ListHypervisors => write!(f, "backup-gateway:ListHypervisors"),
            BackupGatewayActions::ListTagsForResource => {
                write!(f, "backup-gateway:ListTagsForResource")
            }
            BackupGatewayActions::ListVirtualMachines => {
                write!(f, "backup-gateway:ListVirtualMachines")
            }
            BackupGatewayActions::PutBandwidthRateLimitSchedule => {
                write!(f, "backup-gateway:PutBandwidthRateLimitSchedule")
            }
            BackupGatewayActions::PutHypervisorPropertyMappings => {
                write!(f, "backup-gateway:PutHypervisorPropertyMappings")
            }
            BackupGatewayActions::PutMaintenanceStartTime => {
                write!(f, "backup-gateway:PutMaintenanceStartTime")
            }
            BackupGatewayActions::Restore => write!(f, "backup-gateway:Restore"),
            BackupGatewayActions::StartVirtualMachinesMetadataSync => {
                write!(f, "backup-gateway:StartVirtualMachinesMetadataSync")
            }
            BackupGatewayActions::TagResource => write!(f, "backup-gateway:TagResource"),
            BackupGatewayActions::TestHypervisorConfiguration => {
                write!(f, "backup-gateway:TestHypervisorConfiguration")
            }
            BackupGatewayActions::UntagResource => write!(f, "backup-gateway:UntagResource"),
            BackupGatewayActions::UpdateGatewayInformation => {
                write!(f, "backup-gateway:UpdateGatewayInformation")
            }
            BackupGatewayActions::UpdateGatewaySoftwareNow => {
                write!(f, "backup-gateway:UpdateGatewaySoftwareNow")
            }
            BackupGatewayActions::UpdateHypervisor => write!(f, "backup-gateway:UpdateHypervisor"),
        }
    }
}
