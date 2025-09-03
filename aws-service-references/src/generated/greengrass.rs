// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum GreengrassActions {
    AssociateRoleToGroup,
    AssociateServiceRoleToAccount,
    BatchAssociateClientDeviceWithCoreDevice,
    BatchDisassociateClientDeviceFromCoreDevice,
    CancelDeployment,
    CreateComponentVersion,
    CreateConnectorDefinition,
    CreateConnectorDefinitionVersion,
    CreateCoreDefinition,
    CreateCoreDefinitionVersion,
    CreateDeployment,
    CreateDeviceDefinition,
    CreateDeviceDefinitionVersion,
    CreateFunctionDefinition,
    CreateFunctionDefinitionVersion,
    CreateGroup,
    CreateGroupCertificateAuthority,
    CreateGroupVersion,
    CreateLoggerDefinition,
    CreateLoggerDefinitionVersion,
    CreateResourceDefinition,
    CreateResourceDefinitionVersion,
    CreateSoftwareUpdateJob,
    CreateSubscriptionDefinition,
    CreateSubscriptionDefinitionVersion,
    DeleteComponent,
    DeleteConnectorDefinition,
    DeleteCoreDefinition,
    DeleteCoreDevice,
    DeleteDeployment,
    DeleteDeviceDefinition,
    DeleteFunctionDefinition,
    DeleteGroup,
    DeleteLoggerDefinition,
    DeleteResourceDefinition,
    DeleteSubscriptionDefinition,
    DescribeComponent,
    DisassociateRoleFromGroup,
    DisassociateServiceRoleFromAccount,
    Discover,
    GetAssociatedRole,
    GetBulkDeploymentStatus,
    GetComponent,
    GetComponentVersionArtifact,
    GetConnectivityInfo,
    GetConnectorDefinition,
    GetConnectorDefinitionVersion,
    GetCoreDefinition,
    GetCoreDefinitionVersion,
    GetCoreDevice,
    GetDeployment,
    GetDeploymentStatus,
    GetDeviceDefinition,
    GetDeviceDefinitionVersion,
    GetFunctionDefinition,
    GetFunctionDefinitionVersion,
    GetGroup,
    GetGroupCertificateAuthority,
    GetGroupCertificateConfiguration,
    GetGroupVersion,
    GetLoggerDefinition,
    GetLoggerDefinitionVersion,
    GetResourceDefinition,
    GetResourceDefinitionVersion,
    GetServiceRoleForAccount,
    GetSubscriptionDefinition,
    GetSubscriptionDefinitionVersion,
    GetThingRuntimeConfiguration,
    ListBulkDeploymentDetailedReports,
    ListBulkDeployments,
    ListClientDevicesAssociatedWithCoreDevice,
    ListComponentVersions,
    ListComponents,
    ListConnectorDefinitionVersions,
    ListConnectorDefinitions,
    ListCoreDefinitionVersions,
    ListCoreDefinitions,
    ListCoreDevices,
    ListDeployments,
    ListDeviceDefinitionVersions,
    ListDeviceDefinitions,
    ListEffectiveDeployments,
    ListFunctionDefinitionVersions,
    ListFunctionDefinitions,
    ListGroupCertificateAuthorities,
    ListGroupVersions,
    ListGroups,
    ListInstalledComponents,
    ListLoggerDefinitionVersions,
    ListLoggerDefinitions,
    ListResourceDefinitionVersions,
    ListResourceDefinitions,
    ListSubscriptionDefinitionVersions,
    ListSubscriptionDefinitions,
    ListTagsForResource,
    ResetDeployments,
    ResolveComponentCandidates,
    StartBulkDeployment,
    StopBulkDeployment,
    TagResource,
    UntagResource,
    UpdateConnectivityInfo,
    UpdateConnectorDefinition,
    UpdateCoreDefinition,
    UpdateDeviceDefinition,
    UpdateFunctionDefinition,
    UpdateGroup,
    UpdateGroupCertificateConfiguration,
    UpdateLoggerDefinition,
    UpdateResourceDefinition,
    UpdateSubscriptionDefinition,
    UpdateThingRuntimeConfiguration,
}
impl std::fmt::Display for GreengrassActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GreengrassActions::AssociateRoleToGroup => write!(f, "greengrass:AssociateRoleToGroup"),
            GreengrassActions::AssociateServiceRoleToAccount => {
                write!(f, "greengrass:AssociateServiceRoleToAccount")
            }
            GreengrassActions::BatchAssociateClientDeviceWithCoreDevice => {
                write!(f, "greengrass:BatchAssociateClientDeviceWithCoreDevice")
            }
            GreengrassActions::BatchDisassociateClientDeviceFromCoreDevice => {
                write!(f, "greengrass:BatchDisassociateClientDeviceFromCoreDevice")
            }
            GreengrassActions::CancelDeployment => write!(f, "greengrass:CancelDeployment"),
            GreengrassActions::CreateComponentVersion => {
                write!(f, "greengrass:CreateComponentVersion")
            }
            GreengrassActions::CreateConnectorDefinition => {
                write!(f, "greengrass:CreateConnectorDefinition")
            }
            GreengrassActions::CreateConnectorDefinitionVersion => {
                write!(f, "greengrass:CreateConnectorDefinitionVersion")
            }
            GreengrassActions::CreateCoreDefinition => write!(f, "greengrass:CreateCoreDefinition"),
            GreengrassActions::CreateCoreDefinitionVersion => {
                write!(f, "greengrass:CreateCoreDefinitionVersion")
            }
            GreengrassActions::CreateDeployment => write!(f, "greengrass:CreateDeployment"),
            GreengrassActions::CreateDeviceDefinition => {
                write!(f, "greengrass:CreateDeviceDefinition")
            }
            GreengrassActions::CreateDeviceDefinitionVersion => {
                write!(f, "greengrass:CreateDeviceDefinitionVersion")
            }
            GreengrassActions::CreateFunctionDefinition => {
                write!(f, "greengrass:CreateFunctionDefinition")
            }
            GreengrassActions::CreateFunctionDefinitionVersion => {
                write!(f, "greengrass:CreateFunctionDefinitionVersion")
            }
            GreengrassActions::CreateGroup => write!(f, "greengrass:CreateGroup"),
            GreengrassActions::CreateGroupCertificateAuthority => {
                write!(f, "greengrass:CreateGroupCertificateAuthority")
            }
            GreengrassActions::CreateGroupVersion => write!(f, "greengrass:CreateGroupVersion"),
            GreengrassActions::CreateLoggerDefinition => {
                write!(f, "greengrass:CreateLoggerDefinition")
            }
            GreengrassActions::CreateLoggerDefinitionVersion => {
                write!(f, "greengrass:CreateLoggerDefinitionVersion")
            }
            GreengrassActions::CreateResourceDefinition => {
                write!(f, "greengrass:CreateResourceDefinition")
            }
            GreengrassActions::CreateResourceDefinitionVersion => {
                write!(f, "greengrass:CreateResourceDefinitionVersion")
            }
            GreengrassActions::CreateSoftwareUpdateJob => {
                write!(f, "greengrass:CreateSoftwareUpdateJob")
            }
            GreengrassActions::CreateSubscriptionDefinition => {
                write!(f, "greengrass:CreateSubscriptionDefinition")
            }
            GreengrassActions::CreateSubscriptionDefinitionVersion => {
                write!(f, "greengrass:CreateSubscriptionDefinitionVersion")
            }
            GreengrassActions::DeleteComponent => write!(f, "greengrass:DeleteComponent"),
            GreengrassActions::DeleteConnectorDefinition => {
                write!(f, "greengrass:DeleteConnectorDefinition")
            }
            GreengrassActions::DeleteCoreDefinition => write!(f, "greengrass:DeleteCoreDefinition"),
            GreengrassActions::DeleteCoreDevice => write!(f, "greengrass:DeleteCoreDevice"),
            GreengrassActions::DeleteDeployment => write!(f, "greengrass:DeleteDeployment"),
            GreengrassActions::DeleteDeviceDefinition => {
                write!(f, "greengrass:DeleteDeviceDefinition")
            }
            GreengrassActions::DeleteFunctionDefinition => {
                write!(f, "greengrass:DeleteFunctionDefinition")
            }
            GreengrassActions::DeleteGroup => write!(f, "greengrass:DeleteGroup"),
            GreengrassActions::DeleteLoggerDefinition => {
                write!(f, "greengrass:DeleteLoggerDefinition")
            }
            GreengrassActions::DeleteResourceDefinition => {
                write!(f, "greengrass:DeleteResourceDefinition")
            }
            GreengrassActions::DeleteSubscriptionDefinition => {
                write!(f, "greengrass:DeleteSubscriptionDefinition")
            }
            GreengrassActions::DescribeComponent => write!(f, "greengrass:DescribeComponent"),
            GreengrassActions::DisassociateRoleFromGroup => {
                write!(f, "greengrass:DisassociateRoleFromGroup")
            }
            GreengrassActions::DisassociateServiceRoleFromAccount => {
                write!(f, "greengrass:DisassociateServiceRoleFromAccount")
            }
            GreengrassActions::Discover => write!(f, "greengrass:Discover"),
            GreengrassActions::GetAssociatedRole => write!(f, "greengrass:GetAssociatedRole"),
            GreengrassActions::GetBulkDeploymentStatus => {
                write!(f, "greengrass:GetBulkDeploymentStatus")
            }
            GreengrassActions::GetComponent => write!(f, "greengrass:GetComponent"),
            GreengrassActions::GetComponentVersionArtifact => {
                write!(f, "greengrass:GetComponentVersionArtifact")
            }
            GreengrassActions::GetConnectivityInfo => write!(f, "greengrass:GetConnectivityInfo"),
            GreengrassActions::GetConnectorDefinition => {
                write!(f, "greengrass:GetConnectorDefinition")
            }
            GreengrassActions::GetConnectorDefinitionVersion => {
                write!(f, "greengrass:GetConnectorDefinitionVersion")
            }
            GreengrassActions::GetCoreDefinition => write!(f, "greengrass:GetCoreDefinition"),
            GreengrassActions::GetCoreDefinitionVersion => {
                write!(f, "greengrass:GetCoreDefinitionVersion")
            }
            GreengrassActions::GetCoreDevice => write!(f, "greengrass:GetCoreDevice"),
            GreengrassActions::GetDeployment => write!(f, "greengrass:GetDeployment"),
            GreengrassActions::GetDeploymentStatus => write!(f, "greengrass:GetDeploymentStatus"),
            GreengrassActions::GetDeviceDefinition => write!(f, "greengrass:GetDeviceDefinition"),
            GreengrassActions::GetDeviceDefinitionVersion => {
                write!(f, "greengrass:GetDeviceDefinitionVersion")
            }
            GreengrassActions::GetFunctionDefinition => {
                write!(f, "greengrass:GetFunctionDefinition")
            }
            GreengrassActions::GetFunctionDefinitionVersion => {
                write!(f, "greengrass:GetFunctionDefinitionVersion")
            }
            GreengrassActions::GetGroup => write!(f, "greengrass:GetGroup"),
            GreengrassActions::GetGroupCertificateAuthority => {
                write!(f, "greengrass:GetGroupCertificateAuthority")
            }
            GreengrassActions::GetGroupCertificateConfiguration => {
                write!(f, "greengrass:GetGroupCertificateConfiguration")
            }
            GreengrassActions::GetGroupVersion => write!(f, "greengrass:GetGroupVersion"),
            GreengrassActions::GetLoggerDefinition => write!(f, "greengrass:GetLoggerDefinition"),
            GreengrassActions::GetLoggerDefinitionVersion => {
                write!(f, "greengrass:GetLoggerDefinitionVersion")
            }
            GreengrassActions::GetResourceDefinition => {
                write!(f, "greengrass:GetResourceDefinition")
            }
            GreengrassActions::GetResourceDefinitionVersion => {
                write!(f, "greengrass:GetResourceDefinitionVersion")
            }
            GreengrassActions::GetServiceRoleForAccount => {
                write!(f, "greengrass:GetServiceRoleForAccount")
            }
            GreengrassActions::GetSubscriptionDefinition => {
                write!(f, "greengrass:GetSubscriptionDefinition")
            }
            GreengrassActions::GetSubscriptionDefinitionVersion => {
                write!(f, "greengrass:GetSubscriptionDefinitionVersion")
            }
            GreengrassActions::GetThingRuntimeConfiguration => {
                write!(f, "greengrass:GetThingRuntimeConfiguration")
            }
            GreengrassActions::ListBulkDeploymentDetailedReports => {
                write!(f, "greengrass:ListBulkDeploymentDetailedReports")
            }
            GreengrassActions::ListBulkDeployments => write!(f, "greengrass:ListBulkDeployments"),
            GreengrassActions::ListClientDevicesAssociatedWithCoreDevice => {
                write!(f, "greengrass:ListClientDevicesAssociatedWithCoreDevice")
            }
            GreengrassActions::ListComponentVersions => {
                write!(f, "greengrass:ListComponentVersions")
            }
            GreengrassActions::ListComponents => write!(f, "greengrass:ListComponents"),
            GreengrassActions::ListConnectorDefinitionVersions => {
                write!(f, "greengrass:ListConnectorDefinitionVersions")
            }
            GreengrassActions::ListConnectorDefinitions => {
                write!(f, "greengrass:ListConnectorDefinitions")
            }
            GreengrassActions::ListCoreDefinitionVersions => {
                write!(f, "greengrass:ListCoreDefinitionVersions")
            }
            GreengrassActions::ListCoreDefinitions => write!(f, "greengrass:ListCoreDefinitions"),
            GreengrassActions::ListCoreDevices => write!(f, "greengrass:ListCoreDevices"),
            GreengrassActions::ListDeployments => write!(f, "greengrass:ListDeployments"),
            GreengrassActions::ListDeviceDefinitionVersions => {
                write!(f, "greengrass:ListDeviceDefinitionVersions")
            }
            GreengrassActions::ListDeviceDefinitions => {
                write!(f, "greengrass:ListDeviceDefinitions")
            }
            GreengrassActions::ListEffectiveDeployments => {
                write!(f, "greengrass:ListEffectiveDeployments")
            }
            GreengrassActions::ListFunctionDefinitionVersions => {
                write!(f, "greengrass:ListFunctionDefinitionVersions")
            }
            GreengrassActions::ListFunctionDefinitions => {
                write!(f, "greengrass:ListFunctionDefinitions")
            }
            GreengrassActions::ListGroupCertificateAuthorities => {
                write!(f, "greengrass:ListGroupCertificateAuthorities")
            }
            GreengrassActions::ListGroupVersions => write!(f, "greengrass:ListGroupVersions"),
            GreengrassActions::ListGroups => write!(f, "greengrass:ListGroups"),
            GreengrassActions::ListInstalledComponents => {
                write!(f, "greengrass:ListInstalledComponents")
            }
            GreengrassActions::ListLoggerDefinitionVersions => {
                write!(f, "greengrass:ListLoggerDefinitionVersions")
            }
            GreengrassActions::ListLoggerDefinitions => {
                write!(f, "greengrass:ListLoggerDefinitions")
            }
            GreengrassActions::ListResourceDefinitionVersions => {
                write!(f, "greengrass:ListResourceDefinitionVersions")
            }
            GreengrassActions::ListResourceDefinitions => {
                write!(f, "greengrass:ListResourceDefinitions")
            }
            GreengrassActions::ListSubscriptionDefinitionVersions => {
                write!(f, "greengrass:ListSubscriptionDefinitionVersions")
            }
            GreengrassActions::ListSubscriptionDefinitions => {
                write!(f, "greengrass:ListSubscriptionDefinitions")
            }
            GreengrassActions::ListTagsForResource => write!(f, "greengrass:ListTagsForResource"),
            GreengrassActions::ResetDeployments => write!(f, "greengrass:ResetDeployments"),
            GreengrassActions::ResolveComponentCandidates => {
                write!(f, "greengrass:ResolveComponentCandidates")
            }
            GreengrassActions::StartBulkDeployment => write!(f, "greengrass:StartBulkDeployment"),
            GreengrassActions::StopBulkDeployment => write!(f, "greengrass:StopBulkDeployment"),
            GreengrassActions::TagResource => write!(f, "greengrass:TagResource"),
            GreengrassActions::UntagResource => write!(f, "greengrass:UntagResource"),
            GreengrassActions::UpdateConnectivityInfo => {
                write!(f, "greengrass:UpdateConnectivityInfo")
            }
            GreengrassActions::UpdateConnectorDefinition => {
                write!(f, "greengrass:UpdateConnectorDefinition")
            }
            GreengrassActions::UpdateCoreDefinition => write!(f, "greengrass:UpdateCoreDefinition"),
            GreengrassActions::UpdateDeviceDefinition => {
                write!(f, "greengrass:UpdateDeviceDefinition")
            }
            GreengrassActions::UpdateFunctionDefinition => {
                write!(f, "greengrass:UpdateFunctionDefinition")
            }
            GreengrassActions::UpdateGroup => write!(f, "greengrass:UpdateGroup"),
            GreengrassActions::UpdateGroupCertificateConfiguration => {
                write!(f, "greengrass:UpdateGroupCertificateConfiguration")
            }
            GreengrassActions::UpdateLoggerDefinition => {
                write!(f, "greengrass:UpdateLoggerDefinition")
            }
            GreengrassActions::UpdateResourceDefinition => {
                write!(f, "greengrass:UpdateResourceDefinition")
            }
            GreengrassActions::UpdateSubscriptionDefinition => {
                write!(f, "greengrass:UpdateSubscriptionDefinition")
            }
            GreengrassActions::UpdateThingRuntimeConfiguration => {
                write!(f, "greengrass:UpdateThingRuntimeConfiguration")
            }
        }
    }
}
