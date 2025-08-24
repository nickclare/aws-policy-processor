// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum ProtonActions {
    AcceptEnvironmentAccountConnection,
    CancelComponentDeployment,
    CancelEnvironmentDeployment,
    CancelServiceInstanceDeployment,
    CancelServicePipelineDeployment,
    CreateComponent,
    CreateEnvironment,
    CreateEnvironmentAccountConnection,
    CreateEnvironmentTemplate,
    CreateEnvironmentTemplateMajorVersion,
    CreateEnvironmentTemplateMinorVersion,
    CreateEnvironmentTemplateVersion,
    CreateRepository,
    CreateService,
    CreateServiceInstance,
    CreateServiceSyncConfig,
    CreateServiceTemplate,
    CreateServiceTemplateMajorVersion,
    CreateServiceTemplateMinorVersion,
    CreateServiceTemplateVersion,
    CreateTemplateSyncConfig,
    DeleteAccountRoles,
    DeleteComponent,
    DeleteDeployment,
    DeleteEnvironment,
    DeleteEnvironmentAccountConnection,
    DeleteEnvironmentTemplate,
    DeleteEnvironmentTemplateMajorVersion,
    DeleteEnvironmentTemplateMinorVersion,
    DeleteEnvironmentTemplateVersion,
    DeleteRepository,
    DeleteService,
    DeleteServiceSyncConfig,
    DeleteServiceTemplate,
    DeleteServiceTemplateMajorVersion,
    DeleteServiceTemplateMinorVersion,
    DeleteServiceTemplateVersion,
    DeleteTemplateSyncConfig,
    GetAccountRoles,
    GetAccountSettings,
    GetComponent,
    GetDeployment,
    GetEnvironment,
    GetEnvironmentAccountConnection,
    GetEnvironmentTemplate,
    GetEnvironmentTemplateMajorVersion,
    GetEnvironmentTemplateMinorVersion,
    GetEnvironmentTemplateVersion,
    GetRepository,
    GetRepositorySyncStatus,
    GetResourceTemplateVersionStatusCounts,
    GetResourcesSummary,
    GetService,
    GetServiceInstance,
    GetServiceInstanceSyncStatus,
    GetServiceSyncBlockerSummary,
    GetServiceSyncConfig,
    GetServiceTemplate,
    GetServiceTemplateMajorVersion,
    GetServiceTemplateMinorVersion,
    GetServiceTemplateVersion,
    GetTemplateSyncConfig,
    GetTemplateSyncStatus,
    ListComponentOutputs,
    ListComponentProvisionedResources,
    ListComponents,
    ListDeployments,
    ListEnvironmentAccountConnections,
    ListEnvironmentOutputs,
    ListEnvironmentProvisionedResources,
    ListEnvironmentTemplateMajorVersions,
    ListEnvironmentTemplateMinorVersions,
    ListEnvironmentTemplateVersions,
    ListEnvironmentTemplates,
    ListEnvironments,
    ListRepositories,
    ListRepositorySyncDefinitions,
    ListServiceInstanceOutputs,
    ListServiceInstanceProvisionedResources,
    ListServiceInstances,
    ListServicePipelineOutputs,
    ListServicePipelineProvisionedResources,
    ListServiceTemplateMajorVersions,
    ListServiceTemplateMinorVersions,
    ListServiceTemplateVersions,
    ListServiceTemplates,
    ListServices,
    ListTagsForResource,
    NotifyResourceDeploymentStatusChange,
    RejectEnvironmentAccountConnection,
    TagResource,
    UntagResource,
    UpdateAccountRoles,
    UpdateAccountSettings,
    UpdateComponent,
    UpdateEnvironment,
    UpdateEnvironmentAccountConnection,
    UpdateEnvironmentTemplate,
    UpdateEnvironmentTemplateMajorVersion,
    UpdateEnvironmentTemplateMinorVersion,
    UpdateEnvironmentTemplateVersion,
    UpdateService,
    UpdateServiceInstance,
    UpdateServicePipeline,
    UpdateServiceSyncBlocker,
    UpdateServiceSyncConfig,
    UpdateServiceTemplate,
    UpdateServiceTemplateMajorVersion,
    UpdateServiceTemplateMinorVersion,
    UpdateServiceTemplateVersion,
    UpdateTemplateSyncConfig,
}
impl std::fmt::Display for ProtonActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProtonActions::AcceptEnvironmentAccountConnection => {
                write!(f, "proton:AcceptEnvironmentAccountConnection")
            }
            ProtonActions::CancelComponentDeployment => {
                write!(f, "proton:CancelComponentDeployment")
            }
            ProtonActions::CancelEnvironmentDeployment => {
                write!(f, "proton:CancelEnvironmentDeployment")
            }
            ProtonActions::CancelServiceInstanceDeployment => {
                write!(f, "proton:CancelServiceInstanceDeployment")
            }
            ProtonActions::CancelServicePipelineDeployment => {
                write!(f, "proton:CancelServicePipelineDeployment")
            }
            ProtonActions::CreateComponent => write!(f, "proton:CreateComponent"),
            ProtonActions::CreateEnvironment => write!(f, "proton:CreateEnvironment"),
            ProtonActions::CreateEnvironmentAccountConnection => {
                write!(f, "proton:CreateEnvironmentAccountConnection")
            }
            ProtonActions::CreateEnvironmentTemplate => {
                write!(f, "proton:CreateEnvironmentTemplate")
            }
            ProtonActions::CreateEnvironmentTemplateMajorVersion => {
                write!(f, "proton:CreateEnvironmentTemplateMajorVersion")
            }
            ProtonActions::CreateEnvironmentTemplateMinorVersion => {
                write!(f, "proton:CreateEnvironmentTemplateMinorVersion")
            }
            ProtonActions::CreateEnvironmentTemplateVersion => {
                write!(f, "proton:CreateEnvironmentTemplateVersion")
            }
            ProtonActions::CreateRepository => write!(f, "proton:CreateRepository"),
            ProtonActions::CreateService => write!(f, "proton:CreateService"),
            ProtonActions::CreateServiceInstance => write!(f, "proton:CreateServiceInstance"),
            ProtonActions::CreateServiceSyncConfig => write!(f, "proton:CreateServiceSyncConfig"),
            ProtonActions::CreateServiceTemplate => write!(f, "proton:CreateServiceTemplate"),
            ProtonActions::CreateServiceTemplateMajorVersion => {
                write!(f, "proton:CreateServiceTemplateMajorVersion")
            }
            ProtonActions::CreateServiceTemplateMinorVersion => {
                write!(f, "proton:CreateServiceTemplateMinorVersion")
            }
            ProtonActions::CreateServiceTemplateVersion => {
                write!(f, "proton:CreateServiceTemplateVersion")
            }
            ProtonActions::CreateTemplateSyncConfig => write!(f, "proton:CreateTemplateSyncConfig"),
            ProtonActions::DeleteAccountRoles => write!(f, "proton:DeleteAccountRoles"),
            ProtonActions::DeleteComponent => write!(f, "proton:DeleteComponent"),
            ProtonActions::DeleteDeployment => write!(f, "proton:DeleteDeployment"),
            ProtonActions::DeleteEnvironment => write!(f, "proton:DeleteEnvironment"),
            ProtonActions::DeleteEnvironmentAccountConnection => {
                write!(f, "proton:DeleteEnvironmentAccountConnection")
            }
            ProtonActions::DeleteEnvironmentTemplate => {
                write!(f, "proton:DeleteEnvironmentTemplate")
            }
            ProtonActions::DeleteEnvironmentTemplateMajorVersion => {
                write!(f, "proton:DeleteEnvironmentTemplateMajorVersion")
            }
            ProtonActions::DeleteEnvironmentTemplateMinorVersion => {
                write!(f, "proton:DeleteEnvironmentTemplateMinorVersion")
            }
            ProtonActions::DeleteEnvironmentTemplateVersion => {
                write!(f, "proton:DeleteEnvironmentTemplateVersion")
            }
            ProtonActions::DeleteRepository => write!(f, "proton:DeleteRepository"),
            ProtonActions::DeleteService => write!(f, "proton:DeleteService"),
            ProtonActions::DeleteServiceSyncConfig => write!(f, "proton:DeleteServiceSyncConfig"),
            ProtonActions::DeleteServiceTemplate => write!(f, "proton:DeleteServiceTemplate"),
            ProtonActions::DeleteServiceTemplateMajorVersion => {
                write!(f, "proton:DeleteServiceTemplateMajorVersion")
            }
            ProtonActions::DeleteServiceTemplateMinorVersion => {
                write!(f, "proton:DeleteServiceTemplateMinorVersion")
            }
            ProtonActions::DeleteServiceTemplateVersion => {
                write!(f, "proton:DeleteServiceTemplateVersion")
            }
            ProtonActions::DeleteTemplateSyncConfig => write!(f, "proton:DeleteTemplateSyncConfig"),
            ProtonActions::GetAccountRoles => write!(f, "proton:GetAccountRoles"),
            ProtonActions::GetAccountSettings => write!(f, "proton:GetAccountSettings"),
            ProtonActions::GetComponent => write!(f, "proton:GetComponent"),
            ProtonActions::GetDeployment => write!(f, "proton:GetDeployment"),
            ProtonActions::GetEnvironment => write!(f, "proton:GetEnvironment"),
            ProtonActions::GetEnvironmentAccountConnection => {
                write!(f, "proton:GetEnvironmentAccountConnection")
            }
            ProtonActions::GetEnvironmentTemplate => write!(f, "proton:GetEnvironmentTemplate"),
            ProtonActions::GetEnvironmentTemplateMajorVersion => {
                write!(f, "proton:GetEnvironmentTemplateMajorVersion")
            }
            ProtonActions::GetEnvironmentTemplateMinorVersion => {
                write!(f, "proton:GetEnvironmentTemplateMinorVersion")
            }
            ProtonActions::GetEnvironmentTemplateVersion => {
                write!(f, "proton:GetEnvironmentTemplateVersion")
            }
            ProtonActions::GetRepository => write!(f, "proton:GetRepository"),
            ProtonActions::GetRepositorySyncStatus => write!(f, "proton:GetRepositorySyncStatus"),
            ProtonActions::GetResourceTemplateVersionStatusCounts => {
                write!(f, "proton:GetResourceTemplateVersionStatusCounts")
            }
            ProtonActions::GetResourcesSummary => write!(f, "proton:GetResourcesSummary"),
            ProtonActions::GetService => write!(f, "proton:GetService"),
            ProtonActions::GetServiceInstance => write!(f, "proton:GetServiceInstance"),
            ProtonActions::GetServiceInstanceSyncStatus => {
                write!(f, "proton:GetServiceInstanceSyncStatus")
            }
            ProtonActions::GetServiceSyncBlockerSummary => {
                write!(f, "proton:GetServiceSyncBlockerSummary")
            }
            ProtonActions::GetServiceSyncConfig => write!(f, "proton:GetServiceSyncConfig"),
            ProtonActions::GetServiceTemplate => write!(f, "proton:GetServiceTemplate"),
            ProtonActions::GetServiceTemplateMajorVersion => {
                write!(f, "proton:GetServiceTemplateMajorVersion")
            }
            ProtonActions::GetServiceTemplateMinorVersion => {
                write!(f, "proton:GetServiceTemplateMinorVersion")
            }
            ProtonActions::GetServiceTemplateVersion => {
                write!(f, "proton:GetServiceTemplateVersion")
            }
            ProtonActions::GetTemplateSyncConfig => write!(f, "proton:GetTemplateSyncConfig"),
            ProtonActions::GetTemplateSyncStatus => write!(f, "proton:GetTemplateSyncStatus"),
            ProtonActions::ListComponentOutputs => write!(f, "proton:ListComponentOutputs"),
            ProtonActions::ListComponentProvisionedResources => {
                write!(f, "proton:ListComponentProvisionedResources")
            }
            ProtonActions::ListComponents => write!(f, "proton:ListComponents"),
            ProtonActions::ListDeployments => write!(f, "proton:ListDeployments"),
            ProtonActions::ListEnvironmentAccountConnections => {
                write!(f, "proton:ListEnvironmentAccountConnections")
            }
            ProtonActions::ListEnvironmentOutputs => write!(f, "proton:ListEnvironmentOutputs"),
            ProtonActions::ListEnvironmentProvisionedResources => {
                write!(f, "proton:ListEnvironmentProvisionedResources")
            }
            ProtonActions::ListEnvironmentTemplateMajorVersions => {
                write!(f, "proton:ListEnvironmentTemplateMajorVersions")
            }
            ProtonActions::ListEnvironmentTemplateMinorVersions => {
                write!(f, "proton:ListEnvironmentTemplateMinorVersions")
            }
            ProtonActions::ListEnvironmentTemplateVersions => {
                write!(f, "proton:ListEnvironmentTemplateVersions")
            }
            ProtonActions::ListEnvironmentTemplates => write!(f, "proton:ListEnvironmentTemplates"),
            ProtonActions::ListEnvironments => write!(f, "proton:ListEnvironments"),
            ProtonActions::ListRepositories => write!(f, "proton:ListRepositories"),
            ProtonActions::ListRepositorySyncDefinitions => {
                write!(f, "proton:ListRepositorySyncDefinitions")
            }
            ProtonActions::ListServiceInstanceOutputs => {
                write!(f, "proton:ListServiceInstanceOutputs")
            }
            ProtonActions::ListServiceInstanceProvisionedResources => {
                write!(f, "proton:ListServiceInstanceProvisionedResources")
            }
            ProtonActions::ListServiceInstances => write!(f, "proton:ListServiceInstances"),
            ProtonActions::ListServicePipelineOutputs => {
                write!(f, "proton:ListServicePipelineOutputs")
            }
            ProtonActions::ListServicePipelineProvisionedResources => {
                write!(f, "proton:ListServicePipelineProvisionedResources")
            }
            ProtonActions::ListServiceTemplateMajorVersions => {
                write!(f, "proton:ListServiceTemplateMajorVersions")
            }
            ProtonActions::ListServiceTemplateMinorVersions => {
                write!(f, "proton:ListServiceTemplateMinorVersions")
            }
            ProtonActions::ListServiceTemplateVersions => {
                write!(f, "proton:ListServiceTemplateVersions")
            }
            ProtonActions::ListServiceTemplates => write!(f, "proton:ListServiceTemplates"),
            ProtonActions::ListServices => write!(f, "proton:ListServices"),
            ProtonActions::ListTagsForResource => write!(f, "proton:ListTagsForResource"),
            ProtonActions::NotifyResourceDeploymentStatusChange => {
                write!(f, "proton:NotifyResourceDeploymentStatusChange")
            }
            ProtonActions::RejectEnvironmentAccountConnection => {
                write!(f, "proton:RejectEnvironmentAccountConnection")
            }
            ProtonActions::TagResource => write!(f, "proton:TagResource"),
            ProtonActions::UntagResource => write!(f, "proton:UntagResource"),
            ProtonActions::UpdateAccountRoles => write!(f, "proton:UpdateAccountRoles"),
            ProtonActions::UpdateAccountSettings => write!(f, "proton:UpdateAccountSettings"),
            ProtonActions::UpdateComponent => write!(f, "proton:UpdateComponent"),
            ProtonActions::UpdateEnvironment => write!(f, "proton:UpdateEnvironment"),
            ProtonActions::UpdateEnvironmentAccountConnection => {
                write!(f, "proton:UpdateEnvironmentAccountConnection")
            }
            ProtonActions::UpdateEnvironmentTemplate => {
                write!(f, "proton:UpdateEnvironmentTemplate")
            }
            ProtonActions::UpdateEnvironmentTemplateMajorVersion => {
                write!(f, "proton:UpdateEnvironmentTemplateMajorVersion")
            }
            ProtonActions::UpdateEnvironmentTemplateMinorVersion => {
                write!(f, "proton:UpdateEnvironmentTemplateMinorVersion")
            }
            ProtonActions::UpdateEnvironmentTemplateVersion => {
                write!(f, "proton:UpdateEnvironmentTemplateVersion")
            }
            ProtonActions::UpdateService => write!(f, "proton:UpdateService"),
            ProtonActions::UpdateServiceInstance => write!(f, "proton:UpdateServiceInstance"),
            ProtonActions::UpdateServicePipeline => write!(f, "proton:UpdateServicePipeline"),
            ProtonActions::UpdateServiceSyncBlocker => write!(f, "proton:UpdateServiceSyncBlocker"),
            ProtonActions::UpdateServiceSyncConfig => write!(f, "proton:UpdateServiceSyncConfig"),
            ProtonActions::UpdateServiceTemplate => write!(f, "proton:UpdateServiceTemplate"),
            ProtonActions::UpdateServiceTemplateMajorVersion => {
                write!(f, "proton:UpdateServiceTemplateMajorVersion")
            }
            ProtonActions::UpdateServiceTemplateMinorVersion => {
                write!(f, "proton:UpdateServiceTemplateMinorVersion")
            }
            ProtonActions::UpdateServiceTemplateVersion => {
                write!(f, "proton:UpdateServiceTemplateVersion")
            }
            ProtonActions::UpdateTemplateSyncConfig => write!(f, "proton:UpdateTemplateSyncConfig"),
        }
    }
}
