// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum IotwirelessActions {
    AssociateAwsAccountWithPartnerAccount,
    AssociateMulticastGroupWithFuotaTask,
    AssociateWirelessDeviceWithFuotaTask,
    AssociateWirelessDeviceWithMulticastGroup,
    AssociateWirelessDeviceWithThing,
    AssociateWirelessGatewayWithCertificate,
    AssociateWirelessGatewayWithThing,
    CancelMulticastGroupSession,
    CreateDestination,
    CreateDeviceProfile,
    CreateFuotaTask,
    CreateMulticastGroup,
    CreateNetworkAnalyzerConfiguration,
    CreateServiceProfile,
    CreateWirelessDevice,
    CreateWirelessGateway,
    CreateWirelessGatewayTask,
    CreateWirelessGatewayTaskDefinition,
    DeleteDestination,
    DeleteDeviceProfile,
    DeleteFuotaTask,
    DeleteMulticastGroup,
    DeleteNetworkAnalyzerConfiguration,
    DeleteQueuedMessages,
    DeleteServiceProfile,
    DeleteWirelessDevice,
    DeleteWirelessDeviceImportTask,
    DeleteWirelessGateway,
    DeleteWirelessGatewayTask,
    DeleteWirelessGatewayTaskDefinition,
    DeregisterWirelessDevice,
    DisassociateAwsAccountFromPartnerAccount,
    DisassociateMulticastGroupFromFuotaTask,
    DisassociateWirelessDeviceFromFuotaTask,
    DisassociateWirelessDeviceFromMulticastGroup,
    DisassociateWirelessDeviceFromThing,
    DisassociateWirelessGatewayFromCertificate,
    DisassociateWirelessGatewayFromThing,
    GetDestination,
    GetDeviceProfile,
    GetEventConfigurationByResourceTypes,
    GetFuotaTask,
    GetLogLevelsByResourceTypes,
    GetMetricConfiguration,
    GetMetrics,
    GetMulticastGroup,
    GetMulticastGroupSession,
    GetNetworkAnalyzerConfiguration,
    GetPartnerAccount,
    GetPosition,
    GetPositionConfiguration,
    GetPositionEstimate,
    GetResourceEventConfiguration,
    GetResourceLogLevel,
    GetResourcePosition,
    GetServiceEndpoint,
    GetServiceProfile,
    GetWirelessDevice,
    GetWirelessDeviceImportTask,
    GetWirelessDeviceStatistics,
    GetWirelessGateway,
    GetWirelessGatewayCertificate,
    GetWirelessGatewayFirmwareInformation,
    GetWirelessGatewayStatistics,
    GetWirelessGatewayTask,
    GetWirelessGatewayTaskDefinition,
    ListDestinations,
    ListDeviceProfiles,
    ListDevicesForWirelessDeviceImportTask,
    ListEventConfigurations,
    ListFuotaTasks,
    ListMulticastGroups,
    ListMulticastGroupsByFuotaTask,
    ListNetworkAnalyzerConfigurations,
    ListPartnerAccounts,
    ListPositionConfigurations,
    ListQueuedMessages,
    ListServiceProfiles,
    ListTagsForResource,
    ListWirelessDeviceImportTasks,
    ListWirelessDevices,
    ListWirelessGatewayTaskDefinitions,
    ListWirelessGateways,
    PutPositionConfiguration,
    PutResourceLogLevel,
    ResetAllResourceLogLevels,
    ResetResourceLogLevel,
    SendDataToMulticastGroup,
    SendDataToWirelessDevice,
    StartBulkAssociateWirelessDeviceWithMulticastGroup,
    StartBulkDisassociateWirelessDeviceFromMulticastGroup,
    StartFuotaTask,
    StartMulticastGroupSession,
    StartNetworkAnalyzerStream,
    StartSingleWirelessDeviceImportTask,
    StartWirelessDeviceImportTask,
    TagResource,
    TestWirelessDevice,
    UntagResource,
    UpdateDestination,
    UpdateEventConfigurationByResourceTypes,
    UpdateFuotaTask,
    UpdateLogLevelsByResourceTypes,
    UpdateMetricConfiguration,
    UpdateMulticastGroup,
    UpdateNetworkAnalyzerConfiguration,
    UpdatePartnerAccount,
    UpdatePosition,
    UpdateResourceEventConfiguration,
    UpdateResourcePosition,
    UpdateWirelessDevice,
    UpdateWirelessDeviceImportTask,
    UpdateWirelessGateway,
}
impl std::fmt::Display for IotwirelessActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IotwirelessActions::AssociateAwsAccountWithPartnerAccount => {
                write!(f, "iotwireless:AssociateAwsAccountWithPartnerAccount")
            }
            IotwirelessActions::AssociateMulticastGroupWithFuotaTask => {
                write!(f, "iotwireless:AssociateMulticastGroupWithFuotaTask")
            }
            IotwirelessActions::AssociateWirelessDeviceWithFuotaTask => {
                write!(f, "iotwireless:AssociateWirelessDeviceWithFuotaTask")
            }
            IotwirelessActions::AssociateWirelessDeviceWithMulticastGroup => {
                write!(f, "iotwireless:AssociateWirelessDeviceWithMulticastGroup")
            }
            IotwirelessActions::AssociateWirelessDeviceWithThing => {
                write!(f, "iotwireless:AssociateWirelessDeviceWithThing")
            }
            IotwirelessActions::AssociateWirelessGatewayWithCertificate => {
                write!(f, "iotwireless:AssociateWirelessGatewayWithCertificate")
            }
            IotwirelessActions::AssociateWirelessGatewayWithThing => {
                write!(f, "iotwireless:AssociateWirelessGatewayWithThing")
            }
            IotwirelessActions::CancelMulticastGroupSession => {
                write!(f, "iotwireless:CancelMulticastGroupSession")
            }
            IotwirelessActions::CreateDestination => write!(f, "iotwireless:CreateDestination"),
            IotwirelessActions::CreateDeviceProfile => write!(f, "iotwireless:CreateDeviceProfile"),
            IotwirelessActions::CreateFuotaTask => write!(f, "iotwireless:CreateFuotaTask"),
            IotwirelessActions::CreateMulticastGroup => {
                write!(f, "iotwireless:CreateMulticastGroup")
            }
            IotwirelessActions::CreateNetworkAnalyzerConfiguration => {
                write!(f, "iotwireless:CreateNetworkAnalyzerConfiguration")
            }
            IotwirelessActions::CreateServiceProfile => {
                write!(f, "iotwireless:CreateServiceProfile")
            }
            IotwirelessActions::CreateWirelessDevice => {
                write!(f, "iotwireless:CreateWirelessDevice")
            }
            IotwirelessActions::CreateWirelessGateway => {
                write!(f, "iotwireless:CreateWirelessGateway")
            }
            IotwirelessActions::CreateWirelessGatewayTask => {
                write!(f, "iotwireless:CreateWirelessGatewayTask")
            }
            IotwirelessActions::CreateWirelessGatewayTaskDefinition => {
                write!(f, "iotwireless:CreateWirelessGatewayTaskDefinition")
            }
            IotwirelessActions::DeleteDestination => write!(f, "iotwireless:DeleteDestination"),
            IotwirelessActions::DeleteDeviceProfile => write!(f, "iotwireless:DeleteDeviceProfile"),
            IotwirelessActions::DeleteFuotaTask => write!(f, "iotwireless:DeleteFuotaTask"),
            IotwirelessActions::DeleteMulticastGroup => {
                write!(f, "iotwireless:DeleteMulticastGroup")
            }
            IotwirelessActions::DeleteNetworkAnalyzerConfiguration => {
                write!(f, "iotwireless:DeleteNetworkAnalyzerConfiguration")
            }
            IotwirelessActions::DeleteQueuedMessages => {
                write!(f, "iotwireless:DeleteQueuedMessages")
            }
            IotwirelessActions::DeleteServiceProfile => {
                write!(f, "iotwireless:DeleteServiceProfile")
            }
            IotwirelessActions::DeleteWirelessDevice => {
                write!(f, "iotwireless:DeleteWirelessDevice")
            }
            IotwirelessActions::DeleteWirelessDeviceImportTask => {
                write!(f, "iotwireless:DeleteWirelessDeviceImportTask")
            }
            IotwirelessActions::DeleteWirelessGateway => {
                write!(f, "iotwireless:DeleteWirelessGateway")
            }
            IotwirelessActions::DeleteWirelessGatewayTask => {
                write!(f, "iotwireless:DeleteWirelessGatewayTask")
            }
            IotwirelessActions::DeleteWirelessGatewayTaskDefinition => {
                write!(f, "iotwireless:DeleteWirelessGatewayTaskDefinition")
            }
            IotwirelessActions::DeregisterWirelessDevice => {
                write!(f, "iotwireless:DeregisterWirelessDevice")
            }
            IotwirelessActions::DisassociateAwsAccountFromPartnerAccount => {
                write!(f, "iotwireless:DisassociateAwsAccountFromPartnerAccount")
            }
            IotwirelessActions::DisassociateMulticastGroupFromFuotaTask => {
                write!(f, "iotwireless:DisassociateMulticastGroupFromFuotaTask")
            }
            IotwirelessActions::DisassociateWirelessDeviceFromFuotaTask => {
                write!(f, "iotwireless:DisassociateWirelessDeviceFromFuotaTask")
            }
            IotwirelessActions::DisassociateWirelessDeviceFromMulticastGroup => write!(
                f,
                "iotwireless:DisassociateWirelessDeviceFromMulticastGroup"
            ),
            IotwirelessActions::DisassociateWirelessDeviceFromThing => {
                write!(f, "iotwireless:DisassociateWirelessDeviceFromThing")
            }
            IotwirelessActions::DisassociateWirelessGatewayFromCertificate => {
                write!(f, "iotwireless:DisassociateWirelessGatewayFromCertificate")
            }
            IotwirelessActions::DisassociateWirelessGatewayFromThing => {
                write!(f, "iotwireless:DisassociateWirelessGatewayFromThing")
            }
            IotwirelessActions::GetDestination => write!(f, "iotwireless:GetDestination"),
            IotwirelessActions::GetDeviceProfile => write!(f, "iotwireless:GetDeviceProfile"),
            IotwirelessActions::GetEventConfigurationByResourceTypes => {
                write!(f, "iotwireless:GetEventConfigurationByResourceTypes")
            }
            IotwirelessActions::GetFuotaTask => write!(f, "iotwireless:GetFuotaTask"),
            IotwirelessActions::GetLogLevelsByResourceTypes => {
                write!(f, "iotwireless:GetLogLevelsByResourceTypes")
            }
            IotwirelessActions::GetMetricConfiguration => {
                write!(f, "iotwireless:GetMetricConfiguration")
            }
            IotwirelessActions::GetMetrics => write!(f, "iotwireless:GetMetrics"),
            IotwirelessActions::GetMulticastGroup => write!(f, "iotwireless:GetMulticastGroup"),
            IotwirelessActions::GetMulticastGroupSession => {
                write!(f, "iotwireless:GetMulticastGroupSession")
            }
            IotwirelessActions::GetNetworkAnalyzerConfiguration => {
                write!(f, "iotwireless:GetNetworkAnalyzerConfiguration")
            }
            IotwirelessActions::GetPartnerAccount => write!(f, "iotwireless:GetPartnerAccount"),
            IotwirelessActions::GetPosition => write!(f, "iotwireless:GetPosition"),
            IotwirelessActions::GetPositionConfiguration => {
                write!(f, "iotwireless:GetPositionConfiguration")
            }
            IotwirelessActions::GetPositionEstimate => write!(f, "iotwireless:GetPositionEstimate"),
            IotwirelessActions::GetResourceEventConfiguration => {
                write!(f, "iotwireless:GetResourceEventConfiguration")
            }
            IotwirelessActions::GetResourceLogLevel => write!(f, "iotwireless:GetResourceLogLevel"),
            IotwirelessActions::GetResourcePosition => write!(f, "iotwireless:GetResourcePosition"),
            IotwirelessActions::GetServiceEndpoint => write!(f, "iotwireless:GetServiceEndpoint"),
            IotwirelessActions::GetServiceProfile => write!(f, "iotwireless:GetServiceProfile"),
            IotwirelessActions::GetWirelessDevice => write!(f, "iotwireless:GetWirelessDevice"),
            IotwirelessActions::GetWirelessDeviceImportTask => {
                write!(f, "iotwireless:GetWirelessDeviceImportTask")
            }
            IotwirelessActions::GetWirelessDeviceStatistics => {
                write!(f, "iotwireless:GetWirelessDeviceStatistics")
            }
            IotwirelessActions::GetWirelessGateway => write!(f, "iotwireless:GetWirelessGateway"),
            IotwirelessActions::GetWirelessGatewayCertificate => {
                write!(f, "iotwireless:GetWirelessGatewayCertificate")
            }
            IotwirelessActions::GetWirelessGatewayFirmwareInformation => {
                write!(f, "iotwireless:GetWirelessGatewayFirmwareInformation")
            }
            IotwirelessActions::GetWirelessGatewayStatistics => {
                write!(f, "iotwireless:GetWirelessGatewayStatistics")
            }
            IotwirelessActions::GetWirelessGatewayTask => {
                write!(f, "iotwireless:GetWirelessGatewayTask")
            }
            IotwirelessActions::GetWirelessGatewayTaskDefinition => {
                write!(f, "iotwireless:GetWirelessGatewayTaskDefinition")
            }
            IotwirelessActions::ListDestinations => write!(f, "iotwireless:ListDestinations"),
            IotwirelessActions::ListDeviceProfiles => write!(f, "iotwireless:ListDeviceProfiles"),
            IotwirelessActions::ListDevicesForWirelessDeviceImportTask => {
                write!(f, "iotwireless:ListDevicesForWirelessDeviceImportTask")
            }
            IotwirelessActions::ListEventConfigurations => {
                write!(f, "iotwireless:ListEventConfigurations")
            }
            IotwirelessActions::ListFuotaTasks => write!(f, "iotwireless:ListFuotaTasks"),
            IotwirelessActions::ListMulticastGroups => write!(f, "iotwireless:ListMulticastGroups"),
            IotwirelessActions::ListMulticastGroupsByFuotaTask => {
                write!(f, "iotwireless:ListMulticastGroupsByFuotaTask")
            }
            IotwirelessActions::ListNetworkAnalyzerConfigurations => {
                write!(f, "iotwireless:ListNetworkAnalyzerConfigurations")
            }
            IotwirelessActions::ListPartnerAccounts => write!(f, "iotwireless:ListPartnerAccounts"),
            IotwirelessActions::ListPositionConfigurations => {
                write!(f, "iotwireless:ListPositionConfigurations")
            }
            IotwirelessActions::ListQueuedMessages => write!(f, "iotwireless:ListQueuedMessages"),
            IotwirelessActions::ListServiceProfiles => write!(f, "iotwireless:ListServiceProfiles"),
            IotwirelessActions::ListTagsForResource => write!(f, "iotwireless:ListTagsForResource"),
            IotwirelessActions::ListWirelessDeviceImportTasks => {
                write!(f, "iotwireless:ListWirelessDeviceImportTasks")
            }
            IotwirelessActions::ListWirelessDevices => write!(f, "iotwireless:ListWirelessDevices"),
            IotwirelessActions::ListWirelessGatewayTaskDefinitions => {
                write!(f, "iotwireless:ListWirelessGatewayTaskDefinitions")
            }
            IotwirelessActions::ListWirelessGateways => {
                write!(f, "iotwireless:ListWirelessGateways")
            }
            IotwirelessActions::PutPositionConfiguration => {
                write!(f, "iotwireless:PutPositionConfiguration")
            }
            IotwirelessActions::PutResourceLogLevel => write!(f, "iotwireless:PutResourceLogLevel"),
            IotwirelessActions::ResetAllResourceLogLevels => {
                write!(f, "iotwireless:ResetAllResourceLogLevels")
            }
            IotwirelessActions::ResetResourceLogLevel => {
                write!(f, "iotwireless:ResetResourceLogLevel")
            }
            IotwirelessActions::SendDataToMulticastGroup => {
                write!(f, "iotwireless:SendDataToMulticastGroup")
            }
            IotwirelessActions::SendDataToWirelessDevice => {
                write!(f, "iotwireless:SendDataToWirelessDevice")
            }
            IotwirelessActions::StartBulkAssociateWirelessDeviceWithMulticastGroup => write!(
                f,
                "iotwireless:StartBulkAssociateWirelessDeviceWithMulticastGroup"
            ),
            IotwirelessActions::StartBulkDisassociateWirelessDeviceFromMulticastGroup => write!(
                f,
                "iotwireless:StartBulkDisassociateWirelessDeviceFromMulticastGroup"
            ),
            IotwirelessActions::StartFuotaTask => write!(f, "iotwireless:StartFuotaTask"),
            IotwirelessActions::StartMulticastGroupSession => {
                write!(f, "iotwireless:StartMulticastGroupSession")
            }
            IotwirelessActions::StartNetworkAnalyzerStream => {
                write!(f, "iotwireless:StartNetworkAnalyzerStream")
            }
            IotwirelessActions::StartSingleWirelessDeviceImportTask => {
                write!(f, "iotwireless:StartSingleWirelessDeviceImportTask")
            }
            IotwirelessActions::StartWirelessDeviceImportTask => {
                write!(f, "iotwireless:StartWirelessDeviceImportTask")
            }
            IotwirelessActions::TagResource => write!(f, "iotwireless:TagResource"),
            IotwirelessActions::TestWirelessDevice => write!(f, "iotwireless:TestWirelessDevice"),
            IotwirelessActions::UntagResource => write!(f, "iotwireless:UntagResource"),
            IotwirelessActions::UpdateDestination => write!(f, "iotwireless:UpdateDestination"),
            IotwirelessActions::UpdateEventConfigurationByResourceTypes => {
                write!(f, "iotwireless:UpdateEventConfigurationByResourceTypes")
            }
            IotwirelessActions::UpdateFuotaTask => write!(f, "iotwireless:UpdateFuotaTask"),
            IotwirelessActions::UpdateLogLevelsByResourceTypes => {
                write!(f, "iotwireless:UpdateLogLevelsByResourceTypes")
            }
            IotwirelessActions::UpdateMetricConfiguration => {
                write!(f, "iotwireless:UpdateMetricConfiguration")
            }
            IotwirelessActions::UpdateMulticastGroup => {
                write!(f, "iotwireless:UpdateMulticastGroup")
            }
            IotwirelessActions::UpdateNetworkAnalyzerConfiguration => {
                write!(f, "iotwireless:UpdateNetworkAnalyzerConfiguration")
            }
            IotwirelessActions::UpdatePartnerAccount => {
                write!(f, "iotwireless:UpdatePartnerAccount")
            }
            IotwirelessActions::UpdatePosition => write!(f, "iotwireless:UpdatePosition"),
            IotwirelessActions::UpdateResourceEventConfiguration => {
                write!(f, "iotwireless:UpdateResourceEventConfiguration")
            }
            IotwirelessActions::UpdateResourcePosition => {
                write!(f, "iotwireless:UpdateResourcePosition")
            }
            IotwirelessActions::UpdateWirelessDevice => {
                write!(f, "iotwireless:UpdateWirelessDevice")
            }
            IotwirelessActions::UpdateWirelessDeviceImportTask => {
                write!(f, "iotwireless:UpdateWirelessDeviceImportTask")
            }
            IotwirelessActions::UpdateWirelessGateway => {
                write!(f, "iotwireless:UpdateWirelessGateway")
            }
        }
    }
}
