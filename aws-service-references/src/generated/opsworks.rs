// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum OpsworksActions {
    AssignInstance,
    AssignVolume,
    AssociateElasticIp,
    AttachElasticLoadBalancer,
    CloneStack,
    CreateApp,
    CreateDeployment,
    CreateInstance,
    CreateLayer,
    CreateStack,
    CreateUserProfile,
    DeleteApp,
    DeleteInstance,
    DeleteLayer,
    DeleteStack,
    DeleteUserProfile,
    DeregisterEcsCluster,
    DeregisterElasticIp,
    DeregisterInstance,
    DeregisterRdsDbInstance,
    DeregisterVolume,
    DescribeAgentVersions,
    DescribeApps,
    DescribeCommands,
    DescribeDeployments,
    DescribeEcsClusters,
    DescribeElasticIps,
    DescribeElasticLoadBalancers,
    DescribeInstances,
    DescribeLayers,
    DescribeLoadBasedAutoScaling,
    DescribeMyUserProfile,
    DescribeOperatingSystems,
    DescribePermissions,
    DescribeRaidArrays,
    DescribeRdsDbInstances,
    DescribeServiceErrors,
    DescribeStackProvisioningParameters,
    DescribeStackSummary,
    DescribeStacks,
    DescribeTimeBasedAutoScaling,
    DescribeUserProfiles,
    DescribeVolumes,
    DetachElasticLoadBalancer,
    DisassociateElasticIp,
    GetHostnameSuggestion,
    GrantAccess,
    ListTags,
    RebootInstance,
    RegisterEcsCluster,
    RegisterElasticIp,
    RegisterInstance,
    RegisterRdsDbInstance,
    RegisterVolume,
    SetLoadBasedAutoScaling,
    SetPermission,
    SetTimeBasedAutoScaling,
    StartInstance,
    StartStack,
    StopInstance,
    StopStack,
    TagResource,
    UnassignInstance,
    UnassignVolume,
    UntagResource,
    UpdateApp,
    UpdateElasticIp,
    UpdateInstance,
    UpdateLayer,
    UpdateMyUserProfile,
    UpdateRdsDbInstance,
    UpdateStack,
    UpdateUserProfile,
    UpdateVolume,
}
impl std::fmt::Display for OpsworksActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OpsworksActions::AssignInstance => write!(f, "opsworks:AssignInstance"),
            OpsworksActions::AssignVolume => write!(f, "opsworks:AssignVolume"),
            OpsworksActions::AssociateElasticIp => write!(f, "opsworks:AssociateElasticIp"),
            OpsworksActions::AttachElasticLoadBalancer => {
                write!(f, "opsworks:AttachElasticLoadBalancer")
            }
            OpsworksActions::CloneStack => write!(f, "opsworks:CloneStack"),
            OpsworksActions::CreateApp => write!(f, "opsworks:CreateApp"),
            OpsworksActions::CreateDeployment => write!(f, "opsworks:CreateDeployment"),
            OpsworksActions::CreateInstance => write!(f, "opsworks:CreateInstance"),
            OpsworksActions::CreateLayer => write!(f, "opsworks:CreateLayer"),
            OpsworksActions::CreateStack => write!(f, "opsworks:CreateStack"),
            OpsworksActions::CreateUserProfile => write!(f, "opsworks:CreateUserProfile"),
            OpsworksActions::DeleteApp => write!(f, "opsworks:DeleteApp"),
            OpsworksActions::DeleteInstance => write!(f, "opsworks:DeleteInstance"),
            OpsworksActions::DeleteLayer => write!(f, "opsworks:DeleteLayer"),
            OpsworksActions::DeleteStack => write!(f, "opsworks:DeleteStack"),
            OpsworksActions::DeleteUserProfile => write!(f, "opsworks:DeleteUserProfile"),
            OpsworksActions::DeregisterEcsCluster => write!(f, "opsworks:DeregisterEcsCluster"),
            OpsworksActions::DeregisterElasticIp => write!(f, "opsworks:DeregisterElasticIp"),
            OpsworksActions::DeregisterInstance => write!(f, "opsworks:DeregisterInstance"),
            OpsworksActions::DeregisterRdsDbInstance => {
                write!(f, "opsworks:DeregisterRdsDbInstance")
            }
            OpsworksActions::DeregisterVolume => write!(f, "opsworks:DeregisterVolume"),
            OpsworksActions::DescribeAgentVersions => write!(f, "opsworks:DescribeAgentVersions"),
            OpsworksActions::DescribeApps => write!(f, "opsworks:DescribeApps"),
            OpsworksActions::DescribeCommands => write!(f, "opsworks:DescribeCommands"),
            OpsworksActions::DescribeDeployments => write!(f, "opsworks:DescribeDeployments"),
            OpsworksActions::DescribeEcsClusters => write!(f, "opsworks:DescribeEcsClusters"),
            OpsworksActions::DescribeElasticIps => write!(f, "opsworks:DescribeElasticIps"),
            OpsworksActions::DescribeElasticLoadBalancers => {
                write!(f, "opsworks:DescribeElasticLoadBalancers")
            }
            OpsworksActions::DescribeInstances => write!(f, "opsworks:DescribeInstances"),
            OpsworksActions::DescribeLayers => write!(f, "opsworks:DescribeLayers"),
            OpsworksActions::DescribeLoadBasedAutoScaling => {
                write!(f, "opsworks:DescribeLoadBasedAutoScaling")
            }
            OpsworksActions::DescribeMyUserProfile => write!(f, "opsworks:DescribeMyUserProfile"),
            OpsworksActions::DescribeOperatingSystems => {
                write!(f, "opsworks:DescribeOperatingSystems")
            }
            OpsworksActions::DescribePermissions => write!(f, "opsworks:DescribePermissions"),
            OpsworksActions::DescribeRaidArrays => write!(f, "opsworks:DescribeRaidArrays"),
            OpsworksActions::DescribeRdsDbInstances => write!(f, "opsworks:DescribeRdsDbInstances"),
            OpsworksActions::DescribeServiceErrors => write!(f, "opsworks:DescribeServiceErrors"),
            OpsworksActions::DescribeStackProvisioningParameters => {
                write!(f, "opsworks:DescribeStackProvisioningParameters")
            }
            OpsworksActions::DescribeStackSummary => write!(f, "opsworks:DescribeStackSummary"),
            OpsworksActions::DescribeStacks => write!(f, "opsworks:DescribeStacks"),
            OpsworksActions::DescribeTimeBasedAutoScaling => {
                write!(f, "opsworks:DescribeTimeBasedAutoScaling")
            }
            OpsworksActions::DescribeUserProfiles => write!(f, "opsworks:DescribeUserProfiles"),
            OpsworksActions::DescribeVolumes => write!(f, "opsworks:DescribeVolumes"),
            OpsworksActions::DetachElasticLoadBalancer => {
                write!(f, "opsworks:DetachElasticLoadBalancer")
            }
            OpsworksActions::DisassociateElasticIp => write!(f, "opsworks:DisassociateElasticIp"),
            OpsworksActions::GetHostnameSuggestion => write!(f, "opsworks:GetHostnameSuggestion"),
            OpsworksActions::GrantAccess => write!(f, "opsworks:GrantAccess"),
            OpsworksActions::ListTags => write!(f, "opsworks:ListTags"),
            OpsworksActions::RebootInstance => write!(f, "opsworks:RebootInstance"),
            OpsworksActions::RegisterEcsCluster => write!(f, "opsworks:RegisterEcsCluster"),
            OpsworksActions::RegisterElasticIp => write!(f, "opsworks:RegisterElasticIp"),
            OpsworksActions::RegisterInstance => write!(f, "opsworks:RegisterInstance"),
            OpsworksActions::RegisterRdsDbInstance => write!(f, "opsworks:RegisterRdsDbInstance"),
            OpsworksActions::RegisterVolume => write!(f, "opsworks:RegisterVolume"),
            OpsworksActions::SetLoadBasedAutoScaling => {
                write!(f, "opsworks:SetLoadBasedAutoScaling")
            }
            OpsworksActions::SetPermission => write!(f, "opsworks:SetPermission"),
            OpsworksActions::SetTimeBasedAutoScaling => {
                write!(f, "opsworks:SetTimeBasedAutoScaling")
            }
            OpsworksActions::StartInstance => write!(f, "opsworks:StartInstance"),
            OpsworksActions::StartStack => write!(f, "opsworks:StartStack"),
            OpsworksActions::StopInstance => write!(f, "opsworks:StopInstance"),
            OpsworksActions::StopStack => write!(f, "opsworks:StopStack"),
            OpsworksActions::TagResource => write!(f, "opsworks:TagResource"),
            OpsworksActions::UnassignInstance => write!(f, "opsworks:UnassignInstance"),
            OpsworksActions::UnassignVolume => write!(f, "opsworks:UnassignVolume"),
            OpsworksActions::UntagResource => write!(f, "opsworks:UntagResource"),
            OpsworksActions::UpdateApp => write!(f, "opsworks:UpdateApp"),
            OpsworksActions::UpdateElasticIp => write!(f, "opsworks:UpdateElasticIp"),
            OpsworksActions::UpdateInstance => write!(f, "opsworks:UpdateInstance"),
            OpsworksActions::UpdateLayer => write!(f, "opsworks:UpdateLayer"),
            OpsworksActions::UpdateMyUserProfile => write!(f, "opsworks:UpdateMyUserProfile"),
            OpsworksActions::UpdateRdsDbInstance => write!(f, "opsworks:UpdateRdsDbInstance"),
            OpsworksActions::UpdateStack => write!(f, "opsworks:UpdateStack"),
            OpsworksActions::UpdateUserProfile => write!(f, "opsworks:UpdateUserProfile"),
            OpsworksActions::UpdateVolume => write!(f, "opsworks:UpdateVolume"),
        }
    }
}
