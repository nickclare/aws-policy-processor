// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum IotmanagedintegrationsActions {
    CreateAccountAssociation,
    CreateCloudConnector,
    CreateConnectorDestination,
    CreateCredentialLocker,
    CreateDestination,
    CreateEventLogConfiguration,
    CreateManagedThing,
    CreateNotificationConfiguration,
    CreateOtaTask,
    CreateOtaTaskConfiguration,
    CreateProvisioningProfile,
    DeleteAccountAssociation,
    DeleteCloudConnector,
    DeleteConnectorDestination,
    DeleteCredentialLocker,
    DeleteDestination,
    DeleteEventLogConfiguration,
    DeleteManagedThing,
    DeleteNotificationConfiguration,
    DeleteOtaTask,
    DeleteOtaTaskConfiguration,
    DeleteProvisioningProfile,
    DeregisterAccountAssociation,
    GetAccountAssociation,
    GetCloudConnector,
    GetConnectorDestination,
    GetCredentialLocker,
    GetCustomEndpoint,
    GetDefaultEncryptionConfiguration,
    GetDestination,
    GetDeviceDiscovery,
    GetEventLogConfiguration,
    GetHubConfiguration,
    GetManagedThing,
    GetManagedThingCapabilities,
    GetManagedThingConnectivityData,
    GetManagedThingMetaData,
    GetManagedThingState,
    GetNotificationConfiguration,
    GetOtaTask,
    GetOtaTaskConfiguration,
    GetProvisioningProfile,
    GetRuntimeLogConfiguration,
    GetSchemaVersion,
    ListAccountAssociations,
    ListCloudConnectors,
    ListConnectorDestinations,
    ListCredentialLockers,
    ListDestinations,
    ListDeviceDiscoveries,
    ListDiscoveredDevices,
    ListEventLogConfigurations,
    ListManagedThingAccountAssociations,
    ListManagedThingSchemas,
    ListManagedThings,
    ListNotificationConfigurations,
    ListOtaTaskConfigurations,
    ListOtaTaskExecutions,
    ListOtaTasks,
    ListProvisioningProfiles,
    ListSchemaVersions,
    ListTagsForResource,
    PutDefaultEncryptionConfiguration,
    PutHubConfiguration,
    PutRuntimeLogConfiguration,
    RegisterAccountAssociation,
    RegisterCustomEndpoint,
    ResetRuntimeLogConfiguration,
    SendConnectorEvent,
    SendManagedThingCommand,
    StartAccountAssociationRefresh,
    StartDeviceDiscovery,
    TagResource,
    UntagResource,
    UpdateAccountAssociation,
    UpdateCloudConnector,
    UpdateConnectorDestination,
    UpdateDestination,
    UpdateEventLogConfiguration,
    UpdateManagedThing,
    UpdateNotificationConfiguration,
    UpdateOtaTask,
}
impl std::fmt::Display for IotmanagedintegrationsActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IotmanagedintegrationsActions::CreateAccountAssociation => {
                write!(f, "iotmanagedintegrations:CreateAccountAssociation")
            }
            IotmanagedintegrationsActions::CreateCloudConnector => {
                write!(f, "iotmanagedintegrations:CreateCloudConnector")
            }
            IotmanagedintegrationsActions::CreateConnectorDestination => {
                write!(f, "iotmanagedintegrations:CreateConnectorDestination")
            }
            IotmanagedintegrationsActions::CreateCredentialLocker => {
                write!(f, "iotmanagedintegrations:CreateCredentialLocker")
            }
            IotmanagedintegrationsActions::CreateDestination => {
                write!(f, "iotmanagedintegrations:CreateDestination")
            }
            IotmanagedintegrationsActions::CreateEventLogConfiguration => {
                write!(f, "iotmanagedintegrations:CreateEventLogConfiguration")
            }
            IotmanagedintegrationsActions::CreateManagedThing => {
                write!(f, "iotmanagedintegrations:CreateManagedThing")
            }
            IotmanagedintegrationsActions::CreateNotificationConfiguration => {
                write!(f, "iotmanagedintegrations:CreateNotificationConfiguration")
            }
            IotmanagedintegrationsActions::CreateOtaTask => {
                write!(f, "iotmanagedintegrations:CreateOtaTask")
            }
            IotmanagedintegrationsActions::CreateOtaTaskConfiguration => {
                write!(f, "iotmanagedintegrations:CreateOtaTaskConfiguration")
            }
            IotmanagedintegrationsActions::CreateProvisioningProfile => {
                write!(f, "iotmanagedintegrations:CreateProvisioningProfile")
            }
            IotmanagedintegrationsActions::DeleteAccountAssociation => {
                write!(f, "iotmanagedintegrations:DeleteAccountAssociation")
            }
            IotmanagedintegrationsActions::DeleteCloudConnector => {
                write!(f, "iotmanagedintegrations:DeleteCloudConnector")
            }
            IotmanagedintegrationsActions::DeleteConnectorDestination => {
                write!(f, "iotmanagedintegrations:DeleteConnectorDestination")
            }
            IotmanagedintegrationsActions::DeleteCredentialLocker => {
                write!(f, "iotmanagedintegrations:DeleteCredentialLocker")
            }
            IotmanagedintegrationsActions::DeleteDestination => {
                write!(f, "iotmanagedintegrations:DeleteDestination")
            }
            IotmanagedintegrationsActions::DeleteEventLogConfiguration => {
                write!(f, "iotmanagedintegrations:DeleteEventLogConfiguration")
            }
            IotmanagedintegrationsActions::DeleteManagedThing => {
                write!(f, "iotmanagedintegrations:DeleteManagedThing")
            }
            IotmanagedintegrationsActions::DeleteNotificationConfiguration => {
                write!(f, "iotmanagedintegrations:DeleteNotificationConfiguration")
            }
            IotmanagedintegrationsActions::DeleteOtaTask => {
                write!(f, "iotmanagedintegrations:DeleteOtaTask")
            }
            IotmanagedintegrationsActions::DeleteOtaTaskConfiguration => {
                write!(f, "iotmanagedintegrations:DeleteOtaTaskConfiguration")
            }
            IotmanagedintegrationsActions::DeleteProvisioningProfile => {
                write!(f, "iotmanagedintegrations:DeleteProvisioningProfile")
            }
            IotmanagedintegrationsActions::DeregisterAccountAssociation => {
                write!(f, "iotmanagedintegrations:DeregisterAccountAssociation")
            }
            IotmanagedintegrationsActions::GetAccountAssociation => {
                write!(f, "iotmanagedintegrations:GetAccountAssociation")
            }
            IotmanagedintegrationsActions::GetCloudConnector => {
                write!(f, "iotmanagedintegrations:GetCloudConnector")
            }
            IotmanagedintegrationsActions::GetConnectorDestination => {
                write!(f, "iotmanagedintegrations:GetConnectorDestination")
            }
            IotmanagedintegrationsActions::GetCredentialLocker => {
                write!(f, "iotmanagedintegrations:GetCredentialLocker")
            }
            IotmanagedintegrationsActions::GetCustomEndpoint => {
                write!(f, "iotmanagedintegrations:GetCustomEndpoint")
            }
            IotmanagedintegrationsActions::GetDefaultEncryptionConfiguration => write!(
                f,
                "iotmanagedintegrations:GetDefaultEncryptionConfiguration"
            ),
            IotmanagedintegrationsActions::GetDestination => {
                write!(f, "iotmanagedintegrations:GetDestination")
            }
            IotmanagedintegrationsActions::GetDeviceDiscovery => {
                write!(f, "iotmanagedintegrations:GetDeviceDiscovery")
            }
            IotmanagedintegrationsActions::GetEventLogConfiguration => {
                write!(f, "iotmanagedintegrations:GetEventLogConfiguration")
            }
            IotmanagedintegrationsActions::GetHubConfiguration => {
                write!(f, "iotmanagedintegrations:GetHubConfiguration")
            }
            IotmanagedintegrationsActions::GetManagedThing => {
                write!(f, "iotmanagedintegrations:GetManagedThing")
            }
            IotmanagedintegrationsActions::GetManagedThingCapabilities => {
                write!(f, "iotmanagedintegrations:GetManagedThingCapabilities")
            }
            IotmanagedintegrationsActions::GetManagedThingConnectivityData => {
                write!(f, "iotmanagedintegrations:GetManagedThingConnectivityData")
            }
            IotmanagedintegrationsActions::GetManagedThingMetaData => {
                write!(f, "iotmanagedintegrations:GetManagedThingMetaData")
            }
            IotmanagedintegrationsActions::GetManagedThingState => {
                write!(f, "iotmanagedintegrations:GetManagedThingState")
            }
            IotmanagedintegrationsActions::GetNotificationConfiguration => {
                write!(f, "iotmanagedintegrations:GetNotificationConfiguration")
            }
            IotmanagedintegrationsActions::GetOtaTask => {
                write!(f, "iotmanagedintegrations:GetOtaTask")
            }
            IotmanagedintegrationsActions::GetOtaTaskConfiguration => {
                write!(f, "iotmanagedintegrations:GetOtaTaskConfiguration")
            }
            IotmanagedintegrationsActions::GetProvisioningProfile => {
                write!(f, "iotmanagedintegrations:GetProvisioningProfile")
            }
            IotmanagedintegrationsActions::GetRuntimeLogConfiguration => {
                write!(f, "iotmanagedintegrations:GetRuntimeLogConfiguration")
            }
            IotmanagedintegrationsActions::GetSchemaVersion => {
                write!(f, "iotmanagedintegrations:GetSchemaVersion")
            }
            IotmanagedintegrationsActions::ListAccountAssociations => {
                write!(f, "iotmanagedintegrations:ListAccountAssociations")
            }
            IotmanagedintegrationsActions::ListCloudConnectors => {
                write!(f, "iotmanagedintegrations:ListCloudConnectors")
            }
            IotmanagedintegrationsActions::ListConnectorDestinations => {
                write!(f, "iotmanagedintegrations:ListConnectorDestinations")
            }
            IotmanagedintegrationsActions::ListCredentialLockers => {
                write!(f, "iotmanagedintegrations:ListCredentialLockers")
            }
            IotmanagedintegrationsActions::ListDestinations => {
                write!(f, "iotmanagedintegrations:ListDestinations")
            }
            IotmanagedintegrationsActions::ListDeviceDiscoveries => {
                write!(f, "iotmanagedintegrations:ListDeviceDiscoveries")
            }
            IotmanagedintegrationsActions::ListDiscoveredDevices => {
                write!(f, "iotmanagedintegrations:ListDiscoveredDevices")
            }
            IotmanagedintegrationsActions::ListEventLogConfigurations => {
                write!(f, "iotmanagedintegrations:ListEventLogConfigurations")
            }
            IotmanagedintegrationsActions::ListManagedThingAccountAssociations => write!(
                f,
                "iotmanagedintegrations:ListManagedThingAccountAssociations"
            ),
            IotmanagedintegrationsActions::ListManagedThingSchemas => {
                write!(f, "iotmanagedintegrations:ListManagedThingSchemas")
            }
            IotmanagedintegrationsActions::ListManagedThings => {
                write!(f, "iotmanagedintegrations:ListManagedThings")
            }
            IotmanagedintegrationsActions::ListNotificationConfigurations => {
                write!(f, "iotmanagedintegrations:ListNotificationConfigurations")
            }
            IotmanagedintegrationsActions::ListOtaTaskConfigurations => {
                write!(f, "iotmanagedintegrations:ListOtaTaskConfigurations")
            }
            IotmanagedintegrationsActions::ListOtaTaskExecutions => {
                write!(f, "iotmanagedintegrations:ListOtaTaskExecutions")
            }
            IotmanagedintegrationsActions::ListOtaTasks => {
                write!(f, "iotmanagedintegrations:ListOtaTasks")
            }
            IotmanagedintegrationsActions::ListProvisioningProfiles => {
                write!(f, "iotmanagedintegrations:ListProvisioningProfiles")
            }
            IotmanagedintegrationsActions::ListSchemaVersions => {
                write!(f, "iotmanagedintegrations:ListSchemaVersions")
            }
            IotmanagedintegrationsActions::ListTagsForResource => {
                write!(f, "iotmanagedintegrations:ListTagsForResource")
            }
            IotmanagedintegrationsActions::PutDefaultEncryptionConfiguration => write!(
                f,
                "iotmanagedintegrations:PutDefaultEncryptionConfiguration"
            ),
            IotmanagedintegrationsActions::PutHubConfiguration => {
                write!(f, "iotmanagedintegrations:PutHubConfiguration")
            }
            IotmanagedintegrationsActions::PutRuntimeLogConfiguration => {
                write!(f, "iotmanagedintegrations:PutRuntimeLogConfiguration")
            }
            IotmanagedintegrationsActions::RegisterAccountAssociation => {
                write!(f, "iotmanagedintegrations:RegisterAccountAssociation")
            }
            IotmanagedintegrationsActions::RegisterCustomEndpoint => {
                write!(f, "iotmanagedintegrations:RegisterCustomEndpoint")
            }
            IotmanagedintegrationsActions::ResetRuntimeLogConfiguration => {
                write!(f, "iotmanagedintegrations:ResetRuntimeLogConfiguration")
            }
            IotmanagedintegrationsActions::SendConnectorEvent => {
                write!(f, "iotmanagedintegrations:SendConnectorEvent")
            }
            IotmanagedintegrationsActions::SendManagedThingCommand => {
                write!(f, "iotmanagedintegrations:SendManagedThingCommand")
            }
            IotmanagedintegrationsActions::StartAccountAssociationRefresh => {
                write!(f, "iotmanagedintegrations:StartAccountAssociationRefresh")
            }
            IotmanagedintegrationsActions::StartDeviceDiscovery => {
                write!(f, "iotmanagedintegrations:StartDeviceDiscovery")
            }
            IotmanagedintegrationsActions::TagResource => {
                write!(f, "iotmanagedintegrations:TagResource")
            }
            IotmanagedintegrationsActions::UntagResource => {
                write!(f, "iotmanagedintegrations:UntagResource")
            }
            IotmanagedintegrationsActions::UpdateAccountAssociation => {
                write!(f, "iotmanagedintegrations:UpdateAccountAssociation")
            }
            IotmanagedintegrationsActions::UpdateCloudConnector => {
                write!(f, "iotmanagedintegrations:UpdateCloudConnector")
            }
            IotmanagedintegrationsActions::UpdateConnectorDestination => {
                write!(f, "iotmanagedintegrations:UpdateConnectorDestination")
            }
            IotmanagedintegrationsActions::UpdateDestination => {
                write!(f, "iotmanagedintegrations:UpdateDestination")
            }
            IotmanagedintegrationsActions::UpdateEventLogConfiguration => {
                write!(f, "iotmanagedintegrations:UpdateEventLogConfiguration")
            }
            IotmanagedintegrationsActions::UpdateManagedThing => {
                write!(f, "iotmanagedintegrations:UpdateManagedThing")
            }
            IotmanagedintegrationsActions::UpdateNotificationConfiguration => {
                write!(f, "iotmanagedintegrations:UpdateNotificationConfiguration")
            }
            IotmanagedintegrationsActions::UpdateOtaTask => {
                write!(f, "iotmanagedintegrations:UpdateOtaTask")
            }
        }
    }
}
