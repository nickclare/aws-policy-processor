// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum CloudfrontActions {
    AllowVendedLogDeliveryForResource,
    AssociateAlias,
    AssociateDistributionTenantWebAcl,
    AssociateDistributionWebAcl,
    CopyDistribution,
    CreateAnycastIpList,
    CreateCachePolicy,
    CreateCloudFrontOriginAccessIdentity,
    CreateConnectionGroup,
    CreateContinuousDeploymentPolicy,
    CreateDistribution,
    CreateDistributionTenant,
    CreateFieldLevelEncryptionConfig,
    CreateFieldLevelEncryptionProfile,
    CreateFunction,
    CreateInvalidation,
    CreateInvalidationForDistributionTenant,
    CreateKeyGroup,
    CreateKeyValueStore,
    CreateMonitoringSubscription,
    CreateOriginAccessControl,
    CreateOriginRequestPolicy,
    CreatePublicKey,
    CreateRealtimeLogConfig,
    CreateResponseHeadersPolicy,
    CreateSavingsPlan,
    CreateStreamingDistribution,
    CreateStreamingDistributionWithTags,
    CreateVpcOrigin,
    DeleteAnycastIpList,
    DeleteCachePolicy,
    DeleteCloudFrontOriginAccessIdentity,
    DeleteConnectionGroup,
    DeleteContinuousDeploymentPolicy,
    DeleteDistribution,
    DeleteDistributionTenant,
    DeleteFieldLevelEncryptionConfig,
    DeleteFieldLevelEncryptionProfile,
    DeleteFunction,
    DeleteKeyGroup,
    DeleteKeyValueStore,
    DeleteMonitoringSubscription,
    DeleteOriginAccessControl,
    DeleteOriginRequestPolicy,
    DeletePublicKey,
    DeleteRealtimeLogConfig,
    DeleteResponseHeadersPolicy,
    DeleteStreamingDistribution,
    DeleteVpcOrigin,
    DescribeFunction,
    DescribeKeyValueStore,
    DisassociateDistributionTenantWebAcl,
    DisassociateDistributionWebAcl,
    GetAnycastIpList,
    GetCachePolicy,
    GetCachePolicyConfig,
    GetCloudFrontOriginAccessIdentity,
    GetCloudFrontOriginAccessIdentityConfig,
    GetConnectionGroup,
    GetConnectionGroupByRoutingEndpoint,
    GetContinuousDeploymentPolicy,
    GetContinuousDeploymentPolicyConfig,
    GetDistribution,
    GetDistributionConfig,
    GetDistributionTenant,
    GetDistributionTenantByDomain,
    GetFieldLevelEncryption,
    GetFieldLevelEncryptionConfig,
    GetFieldLevelEncryptionProfile,
    GetFieldLevelEncryptionProfileConfig,
    GetFunction,
    GetInvalidation,
    GetInvalidationForDistributionTenant,
    GetKeyGroup,
    GetKeyGroupConfig,
    GetManagedCertificateDetails,
    GetMonitoringSubscription,
    GetOriginAccessControl,
    GetOriginAccessControlConfig,
    GetOriginRequestPolicy,
    GetOriginRequestPolicyConfig,
    GetPublicKey,
    GetPublicKeyConfig,
    GetRealtimeLogConfig,
    GetResponseHeadersPolicy,
    GetResponseHeadersPolicyConfig,
    GetSavingsPlan,
    GetStreamingDistribution,
    GetStreamingDistributionConfig,
    GetVpcOrigin,
    ListAnycastIpLists,
    ListCachePolicies,
    ListCloudFrontOriginAccessIdentities,
    ListConflictingAliases,
    ListConnectionGroups,
    ListContinuousDeploymentPolicies,
    ListDistributionTenants,
    ListDistributionTenantsByCustomization,
    ListDistributions,
    ListDistributionsByAnycastIpListId,
    ListDistributionsByCachePolicyId,
    ListDistributionsByConnectionMode,
    ListDistributionsByKeyGroup,
    ListDistributionsByLambdaFunction,
    ListDistributionsByOriginRequestPolicyId,
    ListDistributionsByRealtimeLogConfig,
    ListDistributionsByResponseHeadersPolicyId,
    ListDistributionsByVpcOriginId,
    ListDistributionsByWebAclId,
    ListDomainConflicts,
    ListFieldLevelEncryptionConfigs,
    ListFieldLevelEncryptionProfiles,
    ListFunctions,
    ListInvalidations,
    ListInvalidationsForDistributionTenant,
    ListKeyGroups,
    ListKeyValueStores,
    ListOriginAccessControls,
    ListOriginRequestPolicies,
    ListPublicKeys,
    ListRateCards,
    ListRealtimeLogConfigs,
    ListResponseHeadersPolicies,
    ListSavingsPlans,
    ListStreamingDistributions,
    ListTagsForResource,
    ListUsages,
    ListVpcOrigins,
    PublishFunction,
    TagResource,
    TestFunction,
    UntagResource,
    UpdateCachePolicy,
    UpdateCloudFrontOriginAccessIdentity,
    UpdateConnectionGroup,
    UpdateContinuousDeploymentPolicy,
    UpdateDistribution,
    UpdateDistributionTenant,
    UpdateDistributionWithStagingConfig,
    UpdateDomainAssociation,
    UpdateFieldLevelEncryptionConfig,
    UpdateFieldLevelEncryptionProfile,
    UpdateFunction,
    UpdateKeyGroup,
    UpdateKeyValueStore,
    UpdateOriginAccessControl,
    UpdateOriginRequestPolicy,
    UpdatePublicKey,
    UpdateRealtimeLogConfig,
    UpdateResponseHeadersPolicy,
    UpdateSavingsPlan,
    UpdateStreamingDistribution,
    UpdateVpcOrigin,
    VerifyDnsConfiguration,
}
impl std::fmt::Display for CloudfrontActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CloudfrontActions::AllowVendedLogDeliveryForResource => {
                write!(f, "cloudfront:AllowVendedLogDeliveryForResource")
            }
            CloudfrontActions::AssociateAlias => write!(f, "cloudfront:AssociateAlias"),
            CloudfrontActions::AssociateDistributionTenantWebAcl => {
                write!(f, "cloudfront:AssociateDistributionTenantWebACL")
            }
            CloudfrontActions::AssociateDistributionWebAcl => {
                write!(f, "cloudfront:AssociateDistributionWebACL")
            }
            CloudfrontActions::CopyDistribution => write!(f, "cloudfront:CopyDistribution"),
            CloudfrontActions::CreateAnycastIpList => write!(f, "cloudfront:CreateAnycastIpList"),
            CloudfrontActions::CreateCachePolicy => write!(f, "cloudfront:CreateCachePolicy"),
            CloudfrontActions::CreateCloudFrontOriginAccessIdentity => {
                write!(f, "cloudfront:CreateCloudFrontOriginAccessIdentity")
            }
            CloudfrontActions::CreateConnectionGroup => {
                write!(f, "cloudfront:CreateConnectionGroup")
            }
            CloudfrontActions::CreateContinuousDeploymentPolicy => {
                write!(f, "cloudfront:CreateContinuousDeploymentPolicy")
            }
            CloudfrontActions::CreateDistribution => write!(f, "cloudfront:CreateDistribution"),
            CloudfrontActions::CreateDistributionTenant => {
                write!(f, "cloudfront:CreateDistributionTenant")
            }
            CloudfrontActions::CreateFieldLevelEncryptionConfig => {
                write!(f, "cloudfront:CreateFieldLevelEncryptionConfig")
            }
            CloudfrontActions::CreateFieldLevelEncryptionProfile => {
                write!(f, "cloudfront:CreateFieldLevelEncryptionProfile")
            }
            CloudfrontActions::CreateFunction => write!(f, "cloudfront:CreateFunction"),
            CloudfrontActions::CreateInvalidation => write!(f, "cloudfront:CreateInvalidation"),
            CloudfrontActions::CreateInvalidationForDistributionTenant => {
                write!(f, "cloudfront:CreateInvalidationForDistributionTenant")
            }
            CloudfrontActions::CreateKeyGroup => write!(f, "cloudfront:CreateKeyGroup"),
            CloudfrontActions::CreateKeyValueStore => write!(f, "cloudfront:CreateKeyValueStore"),
            CloudfrontActions::CreateMonitoringSubscription => {
                write!(f, "cloudfront:CreateMonitoringSubscription")
            }
            CloudfrontActions::CreateOriginAccessControl => {
                write!(f, "cloudfront:CreateOriginAccessControl")
            }
            CloudfrontActions::CreateOriginRequestPolicy => {
                write!(f, "cloudfront:CreateOriginRequestPolicy")
            }
            CloudfrontActions::CreatePublicKey => write!(f, "cloudfront:CreatePublicKey"),
            CloudfrontActions::CreateRealtimeLogConfig => {
                write!(f, "cloudfront:CreateRealtimeLogConfig")
            }
            CloudfrontActions::CreateResponseHeadersPolicy => {
                write!(f, "cloudfront:CreateResponseHeadersPolicy")
            }
            CloudfrontActions::CreateSavingsPlan => write!(f, "cloudfront:CreateSavingsPlan"),
            CloudfrontActions::CreateStreamingDistribution => {
                write!(f, "cloudfront:CreateStreamingDistribution")
            }
            CloudfrontActions::CreateStreamingDistributionWithTags => {
                write!(f, "cloudfront:CreateStreamingDistributionWithTags")
            }
            CloudfrontActions::CreateVpcOrigin => write!(f, "cloudfront:CreateVpcOrigin"),
            CloudfrontActions::DeleteAnycastIpList => write!(f, "cloudfront:DeleteAnycastIpList"),
            CloudfrontActions::DeleteCachePolicy => write!(f, "cloudfront:DeleteCachePolicy"),
            CloudfrontActions::DeleteCloudFrontOriginAccessIdentity => {
                write!(f, "cloudfront:DeleteCloudFrontOriginAccessIdentity")
            }
            CloudfrontActions::DeleteConnectionGroup => {
                write!(f, "cloudfront:DeleteConnectionGroup")
            }
            CloudfrontActions::DeleteContinuousDeploymentPolicy => {
                write!(f, "cloudfront:DeleteContinuousDeploymentPolicy")
            }
            CloudfrontActions::DeleteDistribution => write!(f, "cloudfront:DeleteDistribution"),
            CloudfrontActions::DeleteDistributionTenant => {
                write!(f, "cloudfront:DeleteDistributionTenant")
            }
            CloudfrontActions::DeleteFieldLevelEncryptionConfig => {
                write!(f, "cloudfront:DeleteFieldLevelEncryptionConfig")
            }
            CloudfrontActions::DeleteFieldLevelEncryptionProfile => {
                write!(f, "cloudfront:DeleteFieldLevelEncryptionProfile")
            }
            CloudfrontActions::DeleteFunction => write!(f, "cloudfront:DeleteFunction"),
            CloudfrontActions::DeleteKeyGroup => write!(f, "cloudfront:DeleteKeyGroup"),
            CloudfrontActions::DeleteKeyValueStore => write!(f, "cloudfront:DeleteKeyValueStore"),
            CloudfrontActions::DeleteMonitoringSubscription => {
                write!(f, "cloudfront:DeleteMonitoringSubscription")
            }
            CloudfrontActions::DeleteOriginAccessControl => {
                write!(f, "cloudfront:DeleteOriginAccessControl")
            }
            CloudfrontActions::DeleteOriginRequestPolicy => {
                write!(f, "cloudfront:DeleteOriginRequestPolicy")
            }
            CloudfrontActions::DeletePublicKey => write!(f, "cloudfront:DeletePublicKey"),
            CloudfrontActions::DeleteRealtimeLogConfig => {
                write!(f, "cloudfront:DeleteRealtimeLogConfig")
            }
            CloudfrontActions::DeleteResponseHeadersPolicy => {
                write!(f, "cloudfront:DeleteResponseHeadersPolicy")
            }
            CloudfrontActions::DeleteStreamingDistribution => {
                write!(f, "cloudfront:DeleteStreamingDistribution")
            }
            CloudfrontActions::DeleteVpcOrigin => write!(f, "cloudfront:DeleteVpcOrigin"),
            CloudfrontActions::DescribeFunction => write!(f, "cloudfront:DescribeFunction"),
            CloudfrontActions::DescribeKeyValueStore => {
                write!(f, "cloudfront:DescribeKeyValueStore")
            }
            CloudfrontActions::DisassociateDistributionTenantWebAcl => {
                write!(f, "cloudfront:DisassociateDistributionTenantWebACL")
            }
            CloudfrontActions::DisassociateDistributionWebAcl => {
                write!(f, "cloudfront:DisassociateDistributionWebACL")
            }
            CloudfrontActions::GetAnycastIpList => write!(f, "cloudfront:GetAnycastIpList"),
            CloudfrontActions::GetCachePolicy => write!(f, "cloudfront:GetCachePolicy"),
            CloudfrontActions::GetCachePolicyConfig => write!(f, "cloudfront:GetCachePolicyConfig"),
            CloudfrontActions::GetCloudFrontOriginAccessIdentity => {
                write!(f, "cloudfront:GetCloudFrontOriginAccessIdentity")
            }
            CloudfrontActions::GetCloudFrontOriginAccessIdentityConfig => {
                write!(f, "cloudfront:GetCloudFrontOriginAccessIdentityConfig")
            }
            CloudfrontActions::GetConnectionGroup => write!(f, "cloudfront:GetConnectionGroup"),
            CloudfrontActions::GetConnectionGroupByRoutingEndpoint => {
                write!(f, "cloudfront:GetConnectionGroupByRoutingEndpoint")
            }
            CloudfrontActions::GetContinuousDeploymentPolicy => {
                write!(f, "cloudfront:GetContinuousDeploymentPolicy")
            }
            CloudfrontActions::GetContinuousDeploymentPolicyConfig => {
                write!(f, "cloudfront:GetContinuousDeploymentPolicyConfig")
            }
            CloudfrontActions::GetDistribution => write!(f, "cloudfront:GetDistribution"),
            CloudfrontActions::GetDistributionConfig => {
                write!(f, "cloudfront:GetDistributionConfig")
            }
            CloudfrontActions::GetDistributionTenant => {
                write!(f, "cloudfront:GetDistributionTenant")
            }
            CloudfrontActions::GetDistributionTenantByDomain => {
                write!(f, "cloudfront:GetDistributionTenantByDomain")
            }
            CloudfrontActions::GetFieldLevelEncryption => {
                write!(f, "cloudfront:GetFieldLevelEncryption")
            }
            CloudfrontActions::GetFieldLevelEncryptionConfig => {
                write!(f, "cloudfront:GetFieldLevelEncryptionConfig")
            }
            CloudfrontActions::GetFieldLevelEncryptionProfile => {
                write!(f, "cloudfront:GetFieldLevelEncryptionProfile")
            }
            CloudfrontActions::GetFieldLevelEncryptionProfileConfig => {
                write!(f, "cloudfront:GetFieldLevelEncryptionProfileConfig")
            }
            CloudfrontActions::GetFunction => write!(f, "cloudfront:GetFunction"),
            CloudfrontActions::GetInvalidation => write!(f, "cloudfront:GetInvalidation"),
            CloudfrontActions::GetInvalidationForDistributionTenant => {
                write!(f, "cloudfront:GetInvalidationForDistributionTenant")
            }
            CloudfrontActions::GetKeyGroup => write!(f, "cloudfront:GetKeyGroup"),
            CloudfrontActions::GetKeyGroupConfig => write!(f, "cloudfront:GetKeyGroupConfig"),
            CloudfrontActions::GetManagedCertificateDetails => {
                write!(f, "cloudfront:GetManagedCertificateDetails")
            }
            CloudfrontActions::GetMonitoringSubscription => {
                write!(f, "cloudfront:GetMonitoringSubscription")
            }
            CloudfrontActions::GetOriginAccessControl => {
                write!(f, "cloudfront:GetOriginAccessControl")
            }
            CloudfrontActions::GetOriginAccessControlConfig => {
                write!(f, "cloudfront:GetOriginAccessControlConfig")
            }
            CloudfrontActions::GetOriginRequestPolicy => {
                write!(f, "cloudfront:GetOriginRequestPolicy")
            }
            CloudfrontActions::GetOriginRequestPolicyConfig => {
                write!(f, "cloudfront:GetOriginRequestPolicyConfig")
            }
            CloudfrontActions::GetPublicKey => write!(f, "cloudfront:GetPublicKey"),
            CloudfrontActions::GetPublicKeyConfig => write!(f, "cloudfront:GetPublicKeyConfig"),
            CloudfrontActions::GetRealtimeLogConfig => write!(f, "cloudfront:GetRealtimeLogConfig"),
            CloudfrontActions::GetResponseHeadersPolicy => {
                write!(f, "cloudfront:GetResponseHeadersPolicy")
            }
            CloudfrontActions::GetResponseHeadersPolicyConfig => {
                write!(f, "cloudfront:GetResponseHeadersPolicyConfig")
            }
            CloudfrontActions::GetSavingsPlan => write!(f, "cloudfront:GetSavingsPlan"),
            CloudfrontActions::GetStreamingDistribution => {
                write!(f, "cloudfront:GetStreamingDistribution")
            }
            CloudfrontActions::GetStreamingDistributionConfig => {
                write!(f, "cloudfront:GetStreamingDistributionConfig")
            }
            CloudfrontActions::GetVpcOrigin => write!(f, "cloudfront:GetVpcOrigin"),
            CloudfrontActions::ListAnycastIpLists => write!(f, "cloudfront:ListAnycastIpLists"),
            CloudfrontActions::ListCachePolicies => write!(f, "cloudfront:ListCachePolicies"),
            CloudfrontActions::ListCloudFrontOriginAccessIdentities => {
                write!(f, "cloudfront:ListCloudFrontOriginAccessIdentities")
            }
            CloudfrontActions::ListConflictingAliases => {
                write!(f, "cloudfront:ListConflictingAliases")
            }
            CloudfrontActions::ListConnectionGroups => write!(f, "cloudfront:ListConnectionGroups"),
            CloudfrontActions::ListContinuousDeploymentPolicies => {
                write!(f, "cloudfront:ListContinuousDeploymentPolicies")
            }
            CloudfrontActions::ListDistributionTenants => {
                write!(f, "cloudfront:ListDistributionTenants")
            }
            CloudfrontActions::ListDistributionTenantsByCustomization => {
                write!(f, "cloudfront:ListDistributionTenantsByCustomization")
            }
            CloudfrontActions::ListDistributions => write!(f, "cloudfront:ListDistributions"),
            CloudfrontActions::ListDistributionsByAnycastIpListId => {
                write!(f, "cloudfront:ListDistributionsByAnycastIpListId")
            }
            CloudfrontActions::ListDistributionsByCachePolicyId => {
                write!(f, "cloudfront:ListDistributionsByCachePolicyId")
            }
            CloudfrontActions::ListDistributionsByConnectionMode => {
                write!(f, "cloudfront:ListDistributionsByConnectionMode")
            }
            CloudfrontActions::ListDistributionsByKeyGroup => {
                write!(f, "cloudfront:ListDistributionsByKeyGroup")
            }
            CloudfrontActions::ListDistributionsByLambdaFunction => {
                write!(f, "cloudfront:ListDistributionsByLambdaFunction")
            }
            CloudfrontActions::ListDistributionsByOriginRequestPolicyId => {
                write!(f, "cloudfront:ListDistributionsByOriginRequestPolicyId")
            }
            CloudfrontActions::ListDistributionsByRealtimeLogConfig => {
                write!(f, "cloudfront:ListDistributionsByRealtimeLogConfig")
            }
            CloudfrontActions::ListDistributionsByResponseHeadersPolicyId => {
                write!(f, "cloudfront:ListDistributionsByResponseHeadersPolicyId")
            }
            CloudfrontActions::ListDistributionsByVpcOriginId => {
                write!(f, "cloudfront:ListDistributionsByVpcOriginId")
            }
            CloudfrontActions::ListDistributionsByWebAclId => {
                write!(f, "cloudfront:ListDistributionsByWebACLId")
            }
            CloudfrontActions::ListDomainConflicts => write!(f, "cloudfront:ListDomainConflicts"),
            CloudfrontActions::ListFieldLevelEncryptionConfigs => {
                write!(f, "cloudfront:ListFieldLevelEncryptionConfigs")
            }
            CloudfrontActions::ListFieldLevelEncryptionProfiles => {
                write!(f, "cloudfront:ListFieldLevelEncryptionProfiles")
            }
            CloudfrontActions::ListFunctions => write!(f, "cloudfront:ListFunctions"),
            CloudfrontActions::ListInvalidations => write!(f, "cloudfront:ListInvalidations"),
            CloudfrontActions::ListInvalidationsForDistributionTenant => {
                write!(f, "cloudfront:ListInvalidationsForDistributionTenant")
            }
            CloudfrontActions::ListKeyGroups => write!(f, "cloudfront:ListKeyGroups"),
            CloudfrontActions::ListKeyValueStores => write!(f, "cloudfront:ListKeyValueStores"),
            CloudfrontActions::ListOriginAccessControls => {
                write!(f, "cloudfront:ListOriginAccessControls")
            }
            CloudfrontActions::ListOriginRequestPolicies => {
                write!(f, "cloudfront:ListOriginRequestPolicies")
            }
            CloudfrontActions::ListPublicKeys => write!(f, "cloudfront:ListPublicKeys"),
            CloudfrontActions::ListRateCards => write!(f, "cloudfront:ListRateCards"),
            CloudfrontActions::ListRealtimeLogConfigs => {
                write!(f, "cloudfront:ListRealtimeLogConfigs")
            }
            CloudfrontActions::ListResponseHeadersPolicies => {
                write!(f, "cloudfront:ListResponseHeadersPolicies")
            }
            CloudfrontActions::ListSavingsPlans => write!(f, "cloudfront:ListSavingsPlans"),
            CloudfrontActions::ListStreamingDistributions => {
                write!(f, "cloudfront:ListStreamingDistributions")
            }
            CloudfrontActions::ListTagsForResource => write!(f, "cloudfront:ListTagsForResource"),
            CloudfrontActions::ListUsages => write!(f, "cloudfront:ListUsages"),
            CloudfrontActions::ListVpcOrigins => write!(f, "cloudfront:ListVpcOrigins"),
            CloudfrontActions::PublishFunction => write!(f, "cloudfront:PublishFunction"),
            CloudfrontActions::TagResource => write!(f, "cloudfront:TagResource"),
            CloudfrontActions::TestFunction => write!(f, "cloudfront:TestFunction"),
            CloudfrontActions::UntagResource => write!(f, "cloudfront:UntagResource"),
            CloudfrontActions::UpdateCachePolicy => write!(f, "cloudfront:UpdateCachePolicy"),
            CloudfrontActions::UpdateCloudFrontOriginAccessIdentity => {
                write!(f, "cloudfront:UpdateCloudFrontOriginAccessIdentity")
            }
            CloudfrontActions::UpdateConnectionGroup => {
                write!(f, "cloudfront:UpdateConnectionGroup")
            }
            CloudfrontActions::UpdateContinuousDeploymentPolicy => {
                write!(f, "cloudfront:UpdateContinuousDeploymentPolicy")
            }
            CloudfrontActions::UpdateDistribution => write!(f, "cloudfront:UpdateDistribution"),
            CloudfrontActions::UpdateDistributionTenant => {
                write!(f, "cloudfront:UpdateDistributionTenant")
            }
            CloudfrontActions::UpdateDistributionWithStagingConfig => {
                write!(f, "cloudfront:UpdateDistributionWithStagingConfig")
            }
            CloudfrontActions::UpdateDomainAssociation => {
                write!(f, "cloudfront:UpdateDomainAssociation")
            }
            CloudfrontActions::UpdateFieldLevelEncryptionConfig => {
                write!(f, "cloudfront:UpdateFieldLevelEncryptionConfig")
            }
            CloudfrontActions::UpdateFieldLevelEncryptionProfile => {
                write!(f, "cloudfront:UpdateFieldLevelEncryptionProfile")
            }
            CloudfrontActions::UpdateFunction => write!(f, "cloudfront:UpdateFunction"),
            CloudfrontActions::UpdateKeyGroup => write!(f, "cloudfront:UpdateKeyGroup"),
            CloudfrontActions::UpdateKeyValueStore => write!(f, "cloudfront:UpdateKeyValueStore"),
            CloudfrontActions::UpdateOriginAccessControl => {
                write!(f, "cloudfront:UpdateOriginAccessControl")
            }
            CloudfrontActions::UpdateOriginRequestPolicy => {
                write!(f, "cloudfront:UpdateOriginRequestPolicy")
            }
            CloudfrontActions::UpdatePublicKey => write!(f, "cloudfront:UpdatePublicKey"),
            CloudfrontActions::UpdateRealtimeLogConfig => {
                write!(f, "cloudfront:UpdateRealtimeLogConfig")
            }
            CloudfrontActions::UpdateResponseHeadersPolicy => {
                write!(f, "cloudfront:UpdateResponseHeadersPolicy")
            }
            CloudfrontActions::UpdateSavingsPlan => write!(f, "cloudfront:UpdateSavingsPlan"),
            CloudfrontActions::UpdateStreamingDistribution => {
                write!(f, "cloudfront:UpdateStreamingDistribution")
            }
            CloudfrontActions::UpdateVpcOrigin => write!(f, "cloudfront:UpdateVpcOrigin"),
            CloudfrontActions::VerifyDnsConfiguration => {
                write!(f, "cloudfront:VerifyDnsConfiguration")
            }
        }
    }
}
