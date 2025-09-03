// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum EksActions {
    AccessKubernetesApi,
    AssociateAccessPolicy,
    AssociateEncryptionConfig,
    AssociateIdentityProviderConfig,
    CreateAccessEntry,
    CreateAddon,
    CreateCluster,
    CreateEksAnywhereSubscription,
    CreateFargateProfile,
    CreateNodegroup,
    CreatePodIdentityAssociation,
    DeleteAccessEntry,
    DeleteAddon,
    DeleteCluster,
    DeleteEksAnywhereSubscription,
    DeleteFargateProfile,
    DeleteNodegroup,
    DeletePodIdentityAssociation,
    DeregisterCluster,
    DescribeAccessEntry,
    DescribeAddon,
    DescribeAddonConfiguration,
    DescribeAddonVersions,
    DescribeCluster,
    DescribeClusterVersions,
    DescribeEksAnywhereSubscription,
    DescribeFargateProfile,
    DescribeIdentityProviderConfig,
    DescribeInsight,
    DescribeNodegroup,
    DescribePodIdentityAssociation,
    DescribeUpdate,
    DisassociateAccessPolicy,
    DisassociateIdentityProviderConfig,
    ListAccessEntries,
    ListAccessPolicies,
    ListAddons,
    ListAssociatedAccessPolicies,
    ListClusters,
    ListDashboardData,
    ListDashboardResources,
    ListEksAnywhereSubscriptions,
    ListFargateProfiles,
    ListIdentityProviderConfigs,
    ListInsights,
    ListNodegroups,
    ListPodIdentityAssociations,
    ListTagsForResource,
    ListUpdates,
    RegisterCluster,
    TagResource,
    UntagResource,
    UpdateAccessEntry,
    UpdateAddon,
    UpdateClusterConfig,
    UpdateClusterVersion,
    UpdateEksAnywhereSubscription,
    UpdateNodegroupConfig,
    UpdateNodegroupVersion,
    UpdatePodIdentityAssociation,
}
impl std::fmt::Display for EksActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EksActions::AccessKubernetesApi => write!(f, "eks:AccessKubernetesApi"),
            EksActions::AssociateAccessPolicy => write!(f, "eks:AssociateAccessPolicy"),
            EksActions::AssociateEncryptionConfig => write!(f, "eks:AssociateEncryptionConfig"),
            EksActions::AssociateIdentityProviderConfig => {
                write!(f, "eks:AssociateIdentityProviderConfig")
            }
            EksActions::CreateAccessEntry => write!(f, "eks:CreateAccessEntry"),
            EksActions::CreateAddon => write!(f, "eks:CreateAddon"),
            EksActions::CreateCluster => write!(f, "eks:CreateCluster"),
            EksActions::CreateEksAnywhereSubscription => {
                write!(f, "eks:CreateEksAnywhereSubscription")
            }
            EksActions::CreateFargateProfile => write!(f, "eks:CreateFargateProfile"),
            EksActions::CreateNodegroup => write!(f, "eks:CreateNodegroup"),
            EksActions::CreatePodIdentityAssociation => {
                write!(f, "eks:CreatePodIdentityAssociation")
            }
            EksActions::DeleteAccessEntry => write!(f, "eks:DeleteAccessEntry"),
            EksActions::DeleteAddon => write!(f, "eks:DeleteAddon"),
            EksActions::DeleteCluster => write!(f, "eks:DeleteCluster"),
            EksActions::DeleteEksAnywhereSubscription => {
                write!(f, "eks:DeleteEksAnywhereSubscription")
            }
            EksActions::DeleteFargateProfile => write!(f, "eks:DeleteFargateProfile"),
            EksActions::DeleteNodegroup => write!(f, "eks:DeleteNodegroup"),
            EksActions::DeletePodIdentityAssociation => {
                write!(f, "eks:DeletePodIdentityAssociation")
            }
            EksActions::DeregisterCluster => write!(f, "eks:DeregisterCluster"),
            EksActions::DescribeAccessEntry => write!(f, "eks:DescribeAccessEntry"),
            EksActions::DescribeAddon => write!(f, "eks:DescribeAddon"),
            EksActions::DescribeAddonConfiguration => write!(f, "eks:DescribeAddonConfiguration"),
            EksActions::DescribeAddonVersions => write!(f, "eks:DescribeAddonVersions"),
            EksActions::DescribeCluster => write!(f, "eks:DescribeCluster"),
            EksActions::DescribeClusterVersions => write!(f, "eks:DescribeClusterVersions"),
            EksActions::DescribeEksAnywhereSubscription => {
                write!(f, "eks:DescribeEksAnywhereSubscription")
            }
            EksActions::DescribeFargateProfile => write!(f, "eks:DescribeFargateProfile"),
            EksActions::DescribeIdentityProviderConfig => {
                write!(f, "eks:DescribeIdentityProviderConfig")
            }
            EksActions::DescribeInsight => write!(f, "eks:DescribeInsight"),
            EksActions::DescribeNodegroup => write!(f, "eks:DescribeNodegroup"),
            EksActions::DescribePodIdentityAssociation => {
                write!(f, "eks:DescribePodIdentityAssociation")
            }
            EksActions::DescribeUpdate => write!(f, "eks:DescribeUpdate"),
            EksActions::DisassociateAccessPolicy => write!(f, "eks:DisassociateAccessPolicy"),
            EksActions::DisassociateIdentityProviderConfig => {
                write!(f, "eks:DisassociateIdentityProviderConfig")
            }
            EksActions::ListAccessEntries => write!(f, "eks:ListAccessEntries"),
            EksActions::ListAccessPolicies => write!(f, "eks:ListAccessPolicies"),
            EksActions::ListAddons => write!(f, "eks:ListAddons"),
            EksActions::ListAssociatedAccessPolicies => {
                write!(f, "eks:ListAssociatedAccessPolicies")
            }
            EksActions::ListClusters => write!(f, "eks:ListClusters"),
            EksActions::ListDashboardData => write!(f, "eks:ListDashboardData"),
            EksActions::ListDashboardResources => write!(f, "eks:ListDashboardResources"),
            EksActions::ListEksAnywhereSubscriptions => {
                write!(f, "eks:ListEksAnywhereSubscriptions")
            }
            EksActions::ListFargateProfiles => write!(f, "eks:ListFargateProfiles"),
            EksActions::ListIdentityProviderConfigs => write!(f, "eks:ListIdentityProviderConfigs"),
            EksActions::ListInsights => write!(f, "eks:ListInsights"),
            EksActions::ListNodegroups => write!(f, "eks:ListNodegroups"),
            EksActions::ListPodIdentityAssociations => write!(f, "eks:ListPodIdentityAssociations"),
            EksActions::ListTagsForResource => write!(f, "eks:ListTagsForResource"),
            EksActions::ListUpdates => write!(f, "eks:ListUpdates"),
            EksActions::RegisterCluster => write!(f, "eks:RegisterCluster"),
            EksActions::TagResource => write!(f, "eks:TagResource"),
            EksActions::UntagResource => write!(f, "eks:UntagResource"),
            EksActions::UpdateAccessEntry => write!(f, "eks:UpdateAccessEntry"),
            EksActions::UpdateAddon => write!(f, "eks:UpdateAddon"),
            EksActions::UpdateClusterConfig => write!(f, "eks:UpdateClusterConfig"),
            EksActions::UpdateClusterVersion => write!(f, "eks:UpdateClusterVersion"),
            EksActions::UpdateEksAnywhereSubscription => {
                write!(f, "eks:UpdateEksAnywhereSubscription")
            }
            EksActions::UpdateNodegroupConfig => write!(f, "eks:UpdateNodegroupConfig"),
            EksActions::UpdateNodegroupVersion => write!(f, "eks:UpdateNodegroupVersion"),
            EksActions::UpdatePodIdentityAssociation => {
                write!(f, "eks:UpdatePodIdentityAssociation")
            }
        }
    }
}
