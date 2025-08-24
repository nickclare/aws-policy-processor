// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum LaunchwizardActions {
    CreateAdditionalNode,
    CreateDeployment,
    CreateSettingsSet,
    DeleteAdditionalNode,
    DeleteApp,
    DeleteDeployment,
    DeleteSettingsSet,
    DescribeAdditionalNode,
    DescribeProvisionedApp,
    DescribeProvisioningEvents,
    DescribeSettingsSet,
    GetDeployment,
    GetInfrastructureSuggestion,
    GetIpAddress,
    GetResourceCostEstimate,
    GetResourceRecommendation,
    GetSettingsSet,
    GetWorkload,
    GetWorkloadAsset,
    GetWorkloadAssets,
    GetWorkloadDeploymentPattern,
    ListAdditionalNodes,
    ListAllowedResources,
    ListDeploymentEvents,
    ListDeployments,
    ListProvisionedApps,
    ListResourceCostEstimates,
    ListSettingsSets,
    ListTagsForResource,
    ListWorkloadDeploymentOptions,
    ListWorkloadDeploymentPatterns,
    ListWorkloads,
    PutSettingsSet,
    StartProvisioning,
    TagResource,
    UntagResource,
    UpdateSettingsSet,
}
impl std::fmt::Display for LaunchwizardActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LaunchwizardActions::CreateAdditionalNode => {
                write!(f, "launchwizard:CreateAdditionalNode")
            }
            LaunchwizardActions::CreateDeployment => write!(f, "launchwizard:CreateDeployment"),
            LaunchwizardActions::CreateSettingsSet => write!(f, "launchwizard:CreateSettingsSet"),
            LaunchwizardActions::DeleteAdditionalNode => {
                write!(f, "launchwizard:DeleteAdditionalNode")
            }
            LaunchwizardActions::DeleteApp => write!(f, "launchwizard:DeleteApp"),
            LaunchwizardActions::DeleteDeployment => write!(f, "launchwizard:DeleteDeployment"),
            LaunchwizardActions::DeleteSettingsSet => write!(f, "launchwizard:DeleteSettingsSet"),
            LaunchwizardActions::DescribeAdditionalNode => {
                write!(f, "launchwizard:DescribeAdditionalNode")
            }
            LaunchwizardActions::DescribeProvisionedApp => {
                write!(f, "launchwizard:DescribeProvisionedApp")
            }
            LaunchwizardActions::DescribeProvisioningEvents => {
                write!(f, "launchwizard:DescribeProvisioningEvents")
            }
            LaunchwizardActions::DescribeSettingsSet => {
                write!(f, "launchwizard:DescribeSettingsSet")
            }
            LaunchwizardActions::GetDeployment => write!(f, "launchwizard:GetDeployment"),
            LaunchwizardActions::GetInfrastructureSuggestion => {
                write!(f, "launchwizard:GetInfrastructureSuggestion")
            }
            LaunchwizardActions::GetIpAddress => write!(f, "launchwizard:GetIpAddress"),
            LaunchwizardActions::GetResourceCostEstimate => {
                write!(f, "launchwizard:GetResourceCostEstimate")
            }
            LaunchwizardActions::GetResourceRecommendation => {
                write!(f, "launchwizard:GetResourceRecommendation")
            }
            LaunchwizardActions::GetSettingsSet => write!(f, "launchwizard:GetSettingsSet"),
            LaunchwizardActions::GetWorkload => write!(f, "launchwizard:GetWorkload"),
            LaunchwizardActions::GetWorkloadAsset => write!(f, "launchwizard:GetWorkloadAsset"),
            LaunchwizardActions::GetWorkloadAssets => write!(f, "launchwizard:GetWorkloadAssets"),
            LaunchwizardActions::GetWorkloadDeploymentPattern => {
                write!(f, "launchwizard:GetWorkloadDeploymentPattern")
            }
            LaunchwizardActions::ListAdditionalNodes => {
                write!(f, "launchwizard:ListAdditionalNodes")
            }
            LaunchwizardActions::ListAllowedResources => {
                write!(f, "launchwizard:ListAllowedResources")
            }
            LaunchwizardActions::ListDeploymentEvents => {
                write!(f, "launchwizard:ListDeploymentEvents")
            }
            LaunchwizardActions::ListDeployments => write!(f, "launchwizard:ListDeployments"),
            LaunchwizardActions::ListProvisionedApps => {
                write!(f, "launchwizard:ListProvisionedApps")
            }
            LaunchwizardActions::ListResourceCostEstimates => {
                write!(f, "launchwizard:ListResourceCostEstimates")
            }
            LaunchwizardActions::ListSettingsSets => write!(f, "launchwizard:ListSettingsSets"),
            LaunchwizardActions::ListTagsForResource => {
                write!(f, "launchwizard:ListTagsForResource")
            }
            LaunchwizardActions::ListWorkloadDeploymentOptions => {
                write!(f, "launchwizard:ListWorkloadDeploymentOptions")
            }
            LaunchwizardActions::ListWorkloadDeploymentPatterns => {
                write!(f, "launchwizard:ListWorkloadDeploymentPatterns")
            }
            LaunchwizardActions::ListWorkloads => write!(f, "launchwizard:ListWorkloads"),
            LaunchwizardActions::PutSettingsSet => write!(f, "launchwizard:PutSettingsSet"),
            LaunchwizardActions::StartProvisioning => write!(f, "launchwizard:StartProvisioning"),
            LaunchwizardActions::TagResource => write!(f, "launchwizard:TagResource"),
            LaunchwizardActions::UntagResource => write!(f, "launchwizard:UntagResource"),
            LaunchwizardActions::UpdateSettingsSet => write!(f, "launchwizard:UpdateSettingsSet"),
        }
    }
}
