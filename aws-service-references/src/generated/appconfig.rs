// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum AppconfigActions {
    CreateApplication,
    CreateConfigurationProfile,
    CreateDeploymentStrategy,
    CreateEnvironment,
    CreateExtension,
    CreateExtensionAssociation,
    CreateHostedConfigurationVersion,
    DeleteApplication,
    DeleteConfigurationProfile,
    DeleteDeploymentStrategy,
    DeleteEnvironment,
    DeleteExtension,
    DeleteExtensionAssociation,
    DeleteHostedConfigurationVersion,
    GetAccountSettings,
    GetApplication,
    GetConfiguration,
    GetConfigurationProfile,
    GetDeployment,
    GetDeploymentStrategy,
    GetEnvironment,
    GetExtension,
    GetExtensionAssociation,
    GetHostedConfigurationVersion,
    GetLatestConfiguration,
    ListApplications,
    ListConfigurationProfiles,
    ListDeploymentStrategies,
    ListDeployments,
    ListEnvironments,
    ListExtensionAssociations,
    ListExtensions,
    ListHostedConfigurationVersions,
    ListTagsForResource,
    StartConfigurationSession,
    StartDeployment,
    StopDeployment,
    TagResource,
    UntagResource,
    UpdateAccountSettings,
    UpdateApplication,
    UpdateConfigurationProfile,
    UpdateDeploymentStrategy,
    UpdateEnvironment,
    UpdateExtension,
    UpdateExtensionAssociation,
    ValidateConfiguration,
}
impl std::fmt::Display for AppconfigActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppconfigActions::CreateApplication => write!(f, "appconfig:CreateApplication"),
            AppconfigActions::CreateConfigurationProfile => {
                write!(f, "appconfig:CreateConfigurationProfile")
            }
            AppconfigActions::CreateDeploymentStrategy => {
                write!(f, "appconfig:CreateDeploymentStrategy")
            }
            AppconfigActions::CreateEnvironment => write!(f, "appconfig:CreateEnvironment"),
            AppconfigActions::CreateExtension => write!(f, "appconfig:CreateExtension"),
            AppconfigActions::CreateExtensionAssociation => {
                write!(f, "appconfig:CreateExtensionAssociation")
            }
            AppconfigActions::CreateHostedConfigurationVersion => {
                write!(f, "appconfig:CreateHostedConfigurationVersion")
            }
            AppconfigActions::DeleteApplication => write!(f, "appconfig:DeleteApplication"),
            AppconfigActions::DeleteConfigurationProfile => {
                write!(f, "appconfig:DeleteConfigurationProfile")
            }
            AppconfigActions::DeleteDeploymentStrategy => {
                write!(f, "appconfig:DeleteDeploymentStrategy")
            }
            AppconfigActions::DeleteEnvironment => write!(f, "appconfig:DeleteEnvironment"),
            AppconfigActions::DeleteExtension => write!(f, "appconfig:DeleteExtension"),
            AppconfigActions::DeleteExtensionAssociation => {
                write!(f, "appconfig:DeleteExtensionAssociation")
            }
            AppconfigActions::DeleteHostedConfigurationVersion => {
                write!(f, "appconfig:DeleteHostedConfigurationVersion")
            }
            AppconfigActions::GetAccountSettings => write!(f, "appconfig:GetAccountSettings"),
            AppconfigActions::GetApplication => write!(f, "appconfig:GetApplication"),
            AppconfigActions::GetConfiguration => write!(f, "appconfig:GetConfiguration"),
            AppconfigActions::GetConfigurationProfile => {
                write!(f, "appconfig:GetConfigurationProfile")
            }
            AppconfigActions::GetDeployment => write!(f, "appconfig:GetDeployment"),
            AppconfigActions::GetDeploymentStrategy => write!(f, "appconfig:GetDeploymentStrategy"),
            AppconfigActions::GetEnvironment => write!(f, "appconfig:GetEnvironment"),
            AppconfigActions::GetExtension => write!(f, "appconfig:GetExtension"),
            AppconfigActions::GetExtensionAssociation => {
                write!(f, "appconfig:GetExtensionAssociation")
            }
            AppconfigActions::GetHostedConfigurationVersion => {
                write!(f, "appconfig:GetHostedConfigurationVersion")
            }
            AppconfigActions::GetLatestConfiguration => {
                write!(f, "appconfig:GetLatestConfiguration")
            }
            AppconfigActions::ListApplications => write!(f, "appconfig:ListApplications"),
            AppconfigActions::ListConfigurationProfiles => {
                write!(f, "appconfig:ListConfigurationProfiles")
            }
            AppconfigActions::ListDeploymentStrategies => {
                write!(f, "appconfig:ListDeploymentStrategies")
            }
            AppconfigActions::ListDeployments => write!(f, "appconfig:ListDeployments"),
            AppconfigActions::ListEnvironments => write!(f, "appconfig:ListEnvironments"),
            AppconfigActions::ListExtensionAssociations => {
                write!(f, "appconfig:ListExtensionAssociations")
            }
            AppconfigActions::ListExtensions => write!(f, "appconfig:ListExtensions"),
            AppconfigActions::ListHostedConfigurationVersions => {
                write!(f, "appconfig:ListHostedConfigurationVersions")
            }
            AppconfigActions::ListTagsForResource => write!(f, "appconfig:ListTagsForResource"),
            AppconfigActions::StartConfigurationSession => {
                write!(f, "appconfig:StartConfigurationSession")
            }
            AppconfigActions::StartDeployment => write!(f, "appconfig:StartDeployment"),
            AppconfigActions::StopDeployment => write!(f, "appconfig:StopDeployment"),
            AppconfigActions::TagResource => write!(f, "appconfig:TagResource"),
            AppconfigActions::UntagResource => write!(f, "appconfig:UntagResource"),
            AppconfigActions::UpdateAccountSettings => write!(f, "appconfig:UpdateAccountSettings"),
            AppconfigActions::UpdateApplication => write!(f, "appconfig:UpdateApplication"),
            AppconfigActions::UpdateConfigurationProfile => {
                write!(f, "appconfig:UpdateConfigurationProfile")
            }
            AppconfigActions::UpdateDeploymentStrategy => {
                write!(f, "appconfig:UpdateDeploymentStrategy")
            }
            AppconfigActions::UpdateEnvironment => write!(f, "appconfig:UpdateEnvironment"),
            AppconfigActions::UpdateExtension => write!(f, "appconfig:UpdateExtension"),
            AppconfigActions::UpdateExtensionAssociation => {
                write!(f, "appconfig:UpdateExtensionAssociation")
            }
            AppconfigActions::ValidateConfiguration => write!(f, "appconfig:ValidateConfiguration"),
        }
    }
}
