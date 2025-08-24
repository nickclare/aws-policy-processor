// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum SsmQuicksetupActions {
    CreateConfigurationManager,
    DeleteConfigurationManager,
    GetConfiguration,
    GetConfigurationManager,
    GetServiceSettings,
    ListConfigurationManagers,
    ListConfigurations,
    ListQuickSetupTypes,
    ListTagsForResource,
    TagResource,
    UntagResource,
    UpdateConfigurationDefinition,
    UpdateConfigurationManager,
    UpdateServiceSettings,
}
impl std::fmt::Display for SsmQuicksetupActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SsmQuicksetupActions::CreateConfigurationManager => {
                write!(f, "ssm-quicksetup:CreateConfigurationManager")
            }
            SsmQuicksetupActions::DeleteConfigurationManager => {
                write!(f, "ssm-quicksetup:DeleteConfigurationManager")
            }
            SsmQuicksetupActions::GetConfiguration => write!(f, "ssm-quicksetup:GetConfiguration"),
            SsmQuicksetupActions::GetConfigurationManager => {
                write!(f, "ssm-quicksetup:GetConfigurationManager")
            }
            SsmQuicksetupActions::GetServiceSettings => {
                write!(f, "ssm-quicksetup:GetServiceSettings")
            }
            SsmQuicksetupActions::ListConfigurationManagers => {
                write!(f, "ssm-quicksetup:ListConfigurationManagers")
            }
            SsmQuicksetupActions::ListConfigurations => {
                write!(f, "ssm-quicksetup:ListConfigurations")
            }
            SsmQuicksetupActions::ListQuickSetupTypes => {
                write!(f, "ssm-quicksetup:ListQuickSetupTypes")
            }
            SsmQuicksetupActions::ListTagsForResource => {
                write!(f, "ssm-quicksetup:ListTagsForResource")
            }
            SsmQuicksetupActions::TagResource => write!(f, "ssm-quicksetup:TagResource"),
            SsmQuicksetupActions::UntagResource => write!(f, "ssm-quicksetup:UntagResource"),
            SsmQuicksetupActions::UpdateConfigurationDefinition => {
                write!(f, "ssm-quicksetup:UpdateConfigurationDefinition")
            }
            SsmQuicksetupActions::UpdateConfigurationManager => {
                write!(f, "ssm-quicksetup:UpdateConfigurationManager")
            }
            SsmQuicksetupActions::UpdateServiceSettings => {
                write!(f, "ssm-quicksetup:UpdateServiceSettings")
            }
        }
    }
}
