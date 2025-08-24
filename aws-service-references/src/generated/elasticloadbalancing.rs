// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum ElasticloadbalancingActions {
    AddListenerCertificates,
    AddTags,
    AddTrustStoreRevocations,
    ApplySecurityGroupsToLoadBalancer,
    AttachLoadBalancerToSubnets,
    ConfigureHealthCheck,
    CreateAppCookieStickinessPolicy,
    CreateLbCookieStickinessPolicy,
    CreateListener,
    CreateLoadBalancer,
    CreateLoadBalancerListeners,
    CreateLoadBalancerPolicy,
    CreateRule,
    CreateTargetGroup,
    CreateTrustStore,
    DeleteListener,
    DeleteLoadBalancer,
    DeleteLoadBalancerListeners,
    DeleteLoadBalancerPolicy,
    DeleteRule,
    DeleteSharedTrustStoreAssociation,
    DeleteTargetGroup,
    DeleteTrustStore,
    DeregisterInstancesFromLoadBalancer,
    DeregisterTargets,
    DescribeAccountLimits,
    DescribeCapacityReservation,
    DescribeInstanceHealth,
    DescribeListenerAttributes,
    DescribeListenerCertificates,
    DescribeListeners,
    DescribeLoadBalancerAttributes,
    DescribeLoadBalancerPolicies,
    DescribeLoadBalancerPolicyTypes,
    DescribeLoadBalancers,
    DescribeRules,
    DescribeSslPolicies,
    DescribeTags,
    DescribeTargetGroupAttributes,
    DescribeTargetGroups,
    DescribeTargetHealth,
    DescribeTrustStoreAssociations,
    DescribeTrustStoreRevocations,
    DescribeTrustStores,
    DetachLoadBalancerFromSubnets,
    DisableAvailabilityZonesForLoadBalancer,
    EnableAvailabilityZonesForLoadBalancer,
    GetResourcePolicy,
    GetTrustStoreCaCertificatesBundle,
    GetTrustStoreRevocationContent,
    ModifyCapacityReservation,
    ModifyIpPools,
    ModifyListener,
    ModifyListenerAttributes,
    ModifyLoadBalancerAttributes,
    ModifyRule,
    ModifyTargetGroup,
    ModifyTargetGroupAttributes,
    ModifyTrustStore,
    RegisterInstancesWithLoadBalancer,
    RegisterTargets,
    RemoveListenerCertificates,
    RemoveTags,
    RemoveTrustStoreRevocations,
    SetIpAddressType,
    SetLoadBalancerListenerSslCertificate,
    SetLoadBalancerPoliciesForBackendServer,
    SetLoadBalancerPoliciesOfListener,
    SetRulePriorities,
    SetSecurityGroups,
    SetSubnets,
    SetWebAcl,
}
impl std::fmt::Display for ElasticloadbalancingActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ElasticloadbalancingActions::AddListenerCertificates => {
                write!(f, "elasticloadbalancing:AddListenerCertificates")
            }
            ElasticloadbalancingActions::AddTags => write!(f, "elasticloadbalancing:AddTags"),
            ElasticloadbalancingActions::AddTrustStoreRevocations => {
                write!(f, "elasticloadbalancing:AddTrustStoreRevocations")
            }
            ElasticloadbalancingActions::ApplySecurityGroupsToLoadBalancer => {
                write!(f, "elasticloadbalancing:ApplySecurityGroupsToLoadBalancer")
            }
            ElasticloadbalancingActions::AttachLoadBalancerToSubnets => {
                write!(f, "elasticloadbalancing:AttachLoadBalancerToSubnets")
            }
            ElasticloadbalancingActions::ConfigureHealthCheck => {
                write!(f, "elasticloadbalancing:ConfigureHealthCheck")
            }
            ElasticloadbalancingActions::CreateAppCookieStickinessPolicy => {
                write!(f, "elasticloadbalancing:CreateAppCookieStickinessPolicy")
            }
            ElasticloadbalancingActions::CreateLbCookieStickinessPolicy => {
                write!(f, "elasticloadbalancing:CreateLBCookieStickinessPolicy")
            }
            ElasticloadbalancingActions::CreateListener => {
                write!(f, "elasticloadbalancing:CreateListener")
            }
            ElasticloadbalancingActions::CreateLoadBalancer => {
                write!(f, "elasticloadbalancing:CreateLoadBalancer")
            }
            ElasticloadbalancingActions::CreateLoadBalancerListeners => {
                write!(f, "elasticloadbalancing:CreateLoadBalancerListeners")
            }
            ElasticloadbalancingActions::CreateLoadBalancerPolicy => {
                write!(f, "elasticloadbalancing:CreateLoadBalancerPolicy")
            }
            ElasticloadbalancingActions::CreateRule => write!(f, "elasticloadbalancing:CreateRule"),
            ElasticloadbalancingActions::CreateTargetGroup => {
                write!(f, "elasticloadbalancing:CreateTargetGroup")
            }
            ElasticloadbalancingActions::CreateTrustStore => {
                write!(f, "elasticloadbalancing:CreateTrustStore")
            }
            ElasticloadbalancingActions::DeleteListener => {
                write!(f, "elasticloadbalancing:DeleteListener")
            }
            ElasticloadbalancingActions::DeleteLoadBalancer => {
                write!(f, "elasticloadbalancing:DeleteLoadBalancer")
            }
            ElasticloadbalancingActions::DeleteLoadBalancerListeners => {
                write!(f, "elasticloadbalancing:DeleteLoadBalancerListeners")
            }
            ElasticloadbalancingActions::DeleteLoadBalancerPolicy => {
                write!(f, "elasticloadbalancing:DeleteLoadBalancerPolicy")
            }
            ElasticloadbalancingActions::DeleteRule => write!(f, "elasticloadbalancing:DeleteRule"),
            ElasticloadbalancingActions::DeleteSharedTrustStoreAssociation => {
                write!(f, "elasticloadbalancing:DeleteSharedTrustStoreAssociation")
            }
            ElasticloadbalancingActions::DeleteTargetGroup => {
                write!(f, "elasticloadbalancing:DeleteTargetGroup")
            }
            ElasticloadbalancingActions::DeleteTrustStore => {
                write!(f, "elasticloadbalancing:DeleteTrustStore")
            }
            ElasticloadbalancingActions::DeregisterInstancesFromLoadBalancer => write!(
                f,
                "elasticloadbalancing:DeregisterInstancesFromLoadBalancer"
            ),
            ElasticloadbalancingActions::DeregisterTargets => {
                write!(f, "elasticloadbalancing:DeregisterTargets")
            }
            ElasticloadbalancingActions::DescribeAccountLimits => {
                write!(f, "elasticloadbalancing:DescribeAccountLimits")
            }
            ElasticloadbalancingActions::DescribeCapacityReservation => {
                write!(f, "elasticloadbalancing:DescribeCapacityReservation")
            }
            ElasticloadbalancingActions::DescribeInstanceHealth => {
                write!(f, "elasticloadbalancing:DescribeInstanceHealth")
            }
            ElasticloadbalancingActions::DescribeListenerAttributes => {
                write!(f, "elasticloadbalancing:DescribeListenerAttributes")
            }
            ElasticloadbalancingActions::DescribeListenerCertificates => {
                write!(f, "elasticloadbalancing:DescribeListenerCertificates")
            }
            ElasticloadbalancingActions::DescribeListeners => {
                write!(f, "elasticloadbalancing:DescribeListeners")
            }
            ElasticloadbalancingActions::DescribeLoadBalancerAttributes => {
                write!(f, "elasticloadbalancing:DescribeLoadBalancerAttributes")
            }
            ElasticloadbalancingActions::DescribeLoadBalancerPolicies => {
                write!(f, "elasticloadbalancing:DescribeLoadBalancerPolicies")
            }
            ElasticloadbalancingActions::DescribeLoadBalancerPolicyTypes => {
                write!(f, "elasticloadbalancing:DescribeLoadBalancerPolicyTypes")
            }
            ElasticloadbalancingActions::DescribeLoadBalancers => {
                write!(f, "elasticloadbalancing:DescribeLoadBalancers")
            }
            ElasticloadbalancingActions::DescribeRules => {
                write!(f, "elasticloadbalancing:DescribeRules")
            }
            ElasticloadbalancingActions::DescribeSslPolicies => {
                write!(f, "elasticloadbalancing:DescribeSSLPolicies")
            }
            ElasticloadbalancingActions::DescribeTags => {
                write!(f, "elasticloadbalancing:DescribeTags")
            }
            ElasticloadbalancingActions::DescribeTargetGroupAttributes => {
                write!(f, "elasticloadbalancing:DescribeTargetGroupAttributes")
            }
            ElasticloadbalancingActions::DescribeTargetGroups => {
                write!(f, "elasticloadbalancing:DescribeTargetGroups")
            }
            ElasticloadbalancingActions::DescribeTargetHealth => {
                write!(f, "elasticloadbalancing:DescribeTargetHealth")
            }
            ElasticloadbalancingActions::DescribeTrustStoreAssociations => {
                write!(f, "elasticloadbalancing:DescribeTrustStoreAssociations")
            }
            ElasticloadbalancingActions::DescribeTrustStoreRevocations => {
                write!(f, "elasticloadbalancing:DescribeTrustStoreRevocations")
            }
            ElasticloadbalancingActions::DescribeTrustStores => {
                write!(f, "elasticloadbalancing:DescribeTrustStores")
            }
            ElasticloadbalancingActions::DetachLoadBalancerFromSubnets => {
                write!(f, "elasticloadbalancing:DetachLoadBalancerFromSubnets")
            }
            ElasticloadbalancingActions::DisableAvailabilityZonesForLoadBalancer => write!(
                f,
                "elasticloadbalancing:DisableAvailabilityZonesForLoadBalancer"
            ),
            ElasticloadbalancingActions::EnableAvailabilityZonesForLoadBalancer => write!(
                f,
                "elasticloadbalancing:EnableAvailabilityZonesForLoadBalancer"
            ),
            ElasticloadbalancingActions::GetResourcePolicy => {
                write!(f, "elasticloadbalancing:GetResourcePolicy")
            }
            ElasticloadbalancingActions::GetTrustStoreCaCertificatesBundle => {
                write!(f, "elasticloadbalancing:GetTrustStoreCaCertificatesBundle")
            }
            ElasticloadbalancingActions::GetTrustStoreRevocationContent => {
                write!(f, "elasticloadbalancing:GetTrustStoreRevocationContent")
            }
            ElasticloadbalancingActions::ModifyCapacityReservation => {
                write!(f, "elasticloadbalancing:ModifyCapacityReservation")
            }
            ElasticloadbalancingActions::ModifyIpPools => {
                write!(f, "elasticloadbalancing:ModifyIpPools")
            }
            ElasticloadbalancingActions::ModifyListener => {
                write!(f, "elasticloadbalancing:ModifyListener")
            }
            ElasticloadbalancingActions::ModifyListenerAttributes => {
                write!(f, "elasticloadbalancing:ModifyListenerAttributes")
            }
            ElasticloadbalancingActions::ModifyLoadBalancerAttributes => {
                write!(f, "elasticloadbalancing:ModifyLoadBalancerAttributes")
            }
            ElasticloadbalancingActions::ModifyRule => write!(f, "elasticloadbalancing:ModifyRule"),
            ElasticloadbalancingActions::ModifyTargetGroup => {
                write!(f, "elasticloadbalancing:ModifyTargetGroup")
            }
            ElasticloadbalancingActions::ModifyTargetGroupAttributes => {
                write!(f, "elasticloadbalancing:ModifyTargetGroupAttributes")
            }
            ElasticloadbalancingActions::ModifyTrustStore => {
                write!(f, "elasticloadbalancing:ModifyTrustStore")
            }
            ElasticloadbalancingActions::RegisterInstancesWithLoadBalancer => {
                write!(f, "elasticloadbalancing:RegisterInstancesWithLoadBalancer")
            }
            ElasticloadbalancingActions::RegisterTargets => {
                write!(f, "elasticloadbalancing:RegisterTargets")
            }
            ElasticloadbalancingActions::RemoveListenerCertificates => {
                write!(f, "elasticloadbalancing:RemoveListenerCertificates")
            }
            ElasticloadbalancingActions::RemoveTags => write!(f, "elasticloadbalancing:RemoveTags"),
            ElasticloadbalancingActions::RemoveTrustStoreRevocations => {
                write!(f, "elasticloadbalancing:RemoveTrustStoreRevocations")
            }
            ElasticloadbalancingActions::SetIpAddressType => {
                write!(f, "elasticloadbalancing:SetIpAddressType")
            }
            ElasticloadbalancingActions::SetLoadBalancerListenerSslCertificate => write!(
                f,
                "elasticloadbalancing:SetLoadBalancerListenerSSLCertificate"
            ),
            ElasticloadbalancingActions::SetLoadBalancerPoliciesForBackendServer => write!(
                f,
                "elasticloadbalancing:SetLoadBalancerPoliciesForBackendServer"
            ),
            ElasticloadbalancingActions::SetLoadBalancerPoliciesOfListener => {
                write!(f, "elasticloadbalancing:SetLoadBalancerPoliciesOfListener")
            }
            ElasticloadbalancingActions::SetRulePriorities => {
                write!(f, "elasticloadbalancing:SetRulePriorities")
            }
            ElasticloadbalancingActions::SetSecurityGroups => {
                write!(f, "elasticloadbalancing:SetSecurityGroups")
            }
            ElasticloadbalancingActions::SetSubnets => write!(f, "elasticloadbalancing:SetSubnets"),
            ElasticloadbalancingActions::SetWebAcl => write!(f, "elasticloadbalancing:SetWebAcl"),
        }
    }
}
