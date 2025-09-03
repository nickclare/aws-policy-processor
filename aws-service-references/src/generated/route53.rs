// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum Route53Actions {
    ActivateKeySigningKey,
    AssociateVpcWithHostedZone,
    ChangeCidrCollection,
    ChangeResourceRecordSets,
    ChangeTagsForResource,
    CreateCidrCollection,
    CreateHealthCheck,
    CreateHostedZone,
    CreateKeySigningKey,
    CreateQueryLoggingConfig,
    CreateReusableDelegationSet,
    CreateTrafficPolicy,
    CreateTrafficPolicyInstance,
    CreateTrafficPolicyVersion,
    CreateVpcAssociationAuthorization,
    DeactivateKeySigningKey,
    DeleteCidrCollection,
    DeleteHealthCheck,
    DeleteHostedZone,
    DeleteKeySigningKey,
    DeleteQueryLoggingConfig,
    DeleteReusableDelegationSet,
    DeleteTrafficPolicy,
    DeleteTrafficPolicyInstance,
    DeleteVpcAssociationAuthorization,
    DisableHostedZoneDnssec,
    DisassociateVpcFromHostedZone,
    EnableHostedZoneDnssec,
    GetAccountLimit,
    GetChange,
    GetCheckerIpRanges,
    GetDnssec,
    GetGeoLocation,
    GetHealthCheck,
    GetHealthCheckCount,
    GetHealthCheckLastFailureReason,
    GetHealthCheckStatus,
    GetHostedZone,
    GetHostedZoneCount,
    GetHostedZoneLimit,
    GetQueryLoggingConfig,
    GetReusableDelegationSet,
    GetReusableDelegationSetLimit,
    GetTrafficPolicy,
    GetTrafficPolicyInstance,
    GetTrafficPolicyInstanceCount,
    ListCidrBlocks,
    ListCidrCollections,
    ListCidrLocations,
    ListGeoLocations,
    ListHealthChecks,
    ListHostedZones,
    ListHostedZonesByName,
    ListHostedZonesByVpc,
    ListQueryLoggingConfigs,
    ListResourceRecordSets,
    ListReusableDelegationSets,
    ListTagsForResource,
    ListTagsForResources,
    ListTrafficPolicies,
    ListTrafficPolicyInstances,
    ListTrafficPolicyInstancesByHostedZone,
    ListTrafficPolicyInstancesByPolicy,
    ListTrafficPolicyVersions,
    ListVpcAssociationAuthorizations,
    TestDnsAnswer,
    UpdateHealthCheck,
    UpdateHostedZoneComment,
    UpdateTrafficPolicyComment,
    UpdateTrafficPolicyInstance,
}
impl std::fmt::Display for Route53Actions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Route53Actions::ActivateKeySigningKey => write!(f, "route53:ActivateKeySigningKey"),
            Route53Actions::AssociateVpcWithHostedZone => {
                write!(f, "route53:AssociateVPCWithHostedZone")
            }
            Route53Actions::ChangeCidrCollection => write!(f, "route53:ChangeCidrCollection"),
            Route53Actions::ChangeResourceRecordSets => {
                write!(f, "route53:ChangeResourceRecordSets")
            }
            Route53Actions::ChangeTagsForResource => write!(f, "route53:ChangeTagsForResource"),
            Route53Actions::CreateCidrCollection => write!(f, "route53:CreateCidrCollection"),
            Route53Actions::CreateHealthCheck => write!(f, "route53:CreateHealthCheck"),
            Route53Actions::CreateHostedZone => write!(f, "route53:CreateHostedZone"),
            Route53Actions::CreateKeySigningKey => write!(f, "route53:CreateKeySigningKey"),
            Route53Actions::CreateQueryLoggingConfig => {
                write!(f, "route53:CreateQueryLoggingConfig")
            }
            Route53Actions::CreateReusableDelegationSet => {
                write!(f, "route53:CreateReusableDelegationSet")
            }
            Route53Actions::CreateTrafficPolicy => write!(f, "route53:CreateTrafficPolicy"),
            Route53Actions::CreateTrafficPolicyInstance => {
                write!(f, "route53:CreateTrafficPolicyInstance")
            }
            Route53Actions::CreateTrafficPolicyVersion => {
                write!(f, "route53:CreateTrafficPolicyVersion")
            }
            Route53Actions::CreateVpcAssociationAuthorization => {
                write!(f, "route53:CreateVPCAssociationAuthorization")
            }
            Route53Actions::DeactivateKeySigningKey => write!(f, "route53:DeactivateKeySigningKey"),
            Route53Actions::DeleteCidrCollection => write!(f, "route53:DeleteCidrCollection"),
            Route53Actions::DeleteHealthCheck => write!(f, "route53:DeleteHealthCheck"),
            Route53Actions::DeleteHostedZone => write!(f, "route53:DeleteHostedZone"),
            Route53Actions::DeleteKeySigningKey => write!(f, "route53:DeleteKeySigningKey"),
            Route53Actions::DeleteQueryLoggingConfig => {
                write!(f, "route53:DeleteQueryLoggingConfig")
            }
            Route53Actions::DeleteReusableDelegationSet => {
                write!(f, "route53:DeleteReusableDelegationSet")
            }
            Route53Actions::DeleteTrafficPolicy => write!(f, "route53:DeleteTrafficPolicy"),
            Route53Actions::DeleteTrafficPolicyInstance => {
                write!(f, "route53:DeleteTrafficPolicyInstance")
            }
            Route53Actions::DeleteVpcAssociationAuthorization => {
                write!(f, "route53:DeleteVPCAssociationAuthorization")
            }
            Route53Actions::DisableHostedZoneDnssec => write!(f, "route53:DisableHostedZoneDNSSEC"),
            Route53Actions::DisassociateVpcFromHostedZone => {
                write!(f, "route53:DisassociateVPCFromHostedZone")
            }
            Route53Actions::EnableHostedZoneDnssec => write!(f, "route53:EnableHostedZoneDNSSEC"),
            Route53Actions::GetAccountLimit => write!(f, "route53:GetAccountLimit"),
            Route53Actions::GetChange => write!(f, "route53:GetChange"),
            Route53Actions::GetCheckerIpRanges => write!(f, "route53:GetCheckerIpRanges"),
            Route53Actions::GetDnssec => write!(f, "route53:GetDNSSEC"),
            Route53Actions::GetGeoLocation => write!(f, "route53:GetGeoLocation"),
            Route53Actions::GetHealthCheck => write!(f, "route53:GetHealthCheck"),
            Route53Actions::GetHealthCheckCount => write!(f, "route53:GetHealthCheckCount"),
            Route53Actions::GetHealthCheckLastFailureReason => {
                write!(f, "route53:GetHealthCheckLastFailureReason")
            }
            Route53Actions::GetHealthCheckStatus => write!(f, "route53:GetHealthCheckStatus"),
            Route53Actions::GetHostedZone => write!(f, "route53:GetHostedZone"),
            Route53Actions::GetHostedZoneCount => write!(f, "route53:GetHostedZoneCount"),
            Route53Actions::GetHostedZoneLimit => write!(f, "route53:GetHostedZoneLimit"),
            Route53Actions::GetQueryLoggingConfig => write!(f, "route53:GetQueryLoggingConfig"),
            Route53Actions::GetReusableDelegationSet => {
                write!(f, "route53:GetReusableDelegationSet")
            }
            Route53Actions::GetReusableDelegationSetLimit => {
                write!(f, "route53:GetReusableDelegationSetLimit")
            }
            Route53Actions::GetTrafficPolicy => write!(f, "route53:GetTrafficPolicy"),
            Route53Actions::GetTrafficPolicyInstance => {
                write!(f, "route53:GetTrafficPolicyInstance")
            }
            Route53Actions::GetTrafficPolicyInstanceCount => {
                write!(f, "route53:GetTrafficPolicyInstanceCount")
            }
            Route53Actions::ListCidrBlocks => write!(f, "route53:ListCidrBlocks"),
            Route53Actions::ListCidrCollections => write!(f, "route53:ListCidrCollections"),
            Route53Actions::ListCidrLocations => write!(f, "route53:ListCidrLocations"),
            Route53Actions::ListGeoLocations => write!(f, "route53:ListGeoLocations"),
            Route53Actions::ListHealthChecks => write!(f, "route53:ListHealthChecks"),
            Route53Actions::ListHostedZones => write!(f, "route53:ListHostedZones"),
            Route53Actions::ListHostedZonesByName => write!(f, "route53:ListHostedZonesByName"),
            Route53Actions::ListHostedZonesByVpc => write!(f, "route53:ListHostedZonesByVPC"),
            Route53Actions::ListQueryLoggingConfigs => write!(f, "route53:ListQueryLoggingConfigs"),
            Route53Actions::ListResourceRecordSets => write!(f, "route53:ListResourceRecordSets"),
            Route53Actions::ListReusableDelegationSets => {
                write!(f, "route53:ListReusableDelegationSets")
            }
            Route53Actions::ListTagsForResource => write!(f, "route53:ListTagsForResource"),
            Route53Actions::ListTagsForResources => write!(f, "route53:ListTagsForResources"),
            Route53Actions::ListTrafficPolicies => write!(f, "route53:ListTrafficPolicies"),
            Route53Actions::ListTrafficPolicyInstances => {
                write!(f, "route53:ListTrafficPolicyInstances")
            }
            Route53Actions::ListTrafficPolicyInstancesByHostedZone => {
                write!(f, "route53:ListTrafficPolicyInstancesByHostedZone")
            }
            Route53Actions::ListTrafficPolicyInstancesByPolicy => {
                write!(f, "route53:ListTrafficPolicyInstancesByPolicy")
            }
            Route53Actions::ListTrafficPolicyVersions => {
                write!(f, "route53:ListTrafficPolicyVersions")
            }
            Route53Actions::ListVpcAssociationAuthorizations => {
                write!(f, "route53:ListVPCAssociationAuthorizations")
            }
            Route53Actions::TestDnsAnswer => write!(f, "route53:TestDNSAnswer"),
            Route53Actions::UpdateHealthCheck => write!(f, "route53:UpdateHealthCheck"),
            Route53Actions::UpdateHostedZoneComment => write!(f, "route53:UpdateHostedZoneComment"),
            Route53Actions::UpdateTrafficPolicyComment => {
                write!(f, "route53:UpdateTrafficPolicyComment")
            }
            Route53Actions::UpdateTrafficPolicyInstance => {
                write!(f, "route53:UpdateTrafficPolicyInstance")
            }
        }
    }
}
