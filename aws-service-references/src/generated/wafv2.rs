// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum Wafv2Actions {
    AssociateWebAcl,
    CheckCapacity,
    CreateApiKey,
    CreateIpSet,
    CreateRegexPatternSet,
    CreateRuleGroup,
    CreateWebAcl,
    DeleteApiKey,
    DeleteFirewallManagerRuleGroups,
    DeleteIpSet,
    DeleteLoggingConfiguration,
    DeletePermissionPolicy,
    DeleteRegexPatternSet,
    DeleteRuleGroup,
    DeleteWebAcl,
    DescribeAllManagedProducts,
    DescribeManagedProductsByVendor,
    DescribeManagedRuleGroup,
    DisassociateFirewallManager,
    DisassociateWebAcl,
    GenerateMobileSdkReleaseUrl,
    GetDecryptedApiKey,
    GetIpSet,
    GetLoggingConfiguration,
    GetManagedRuleSet,
    GetMobileSdkRelease,
    GetPermissionPolicy,
    GetRateBasedStatementManagedKeys,
    GetRegexPatternSet,
    GetRuleGroup,
    GetSampledRequests,
    GetWebAcl,
    GetWebAclForResource,
    ListApiKeys,
    ListAvailableManagedRuleGroupVersions,
    ListAvailableManagedRuleGroups,
    ListIpSets,
    ListLoggingConfigurations,
    ListManagedRuleSets,
    ListMobileSdkReleases,
    ListRegexPatternSets,
    ListResourcesForWebAcl,
    ListRuleGroups,
    ListTagsForResource,
    ListWebAcLs,
    PutFirewallManagerRuleGroups,
    PutLoggingConfiguration,
    PutManagedRuleSetVersions,
    PutPermissionPolicy,
    TagResource,
    UntagResource,
    UpdateIpSet,
    UpdateManagedRuleSetVersionExpiryDate,
    UpdateRegexPatternSet,
    UpdateRuleGroup,
    UpdateWebAcl,
}
impl std::fmt::Display for Wafv2Actions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Wafv2Actions::AssociateWebAcl => write!(f, "wafv2:AssociateWebACL"),
            Wafv2Actions::CheckCapacity => write!(f, "wafv2:CheckCapacity"),
            Wafv2Actions::CreateApiKey => write!(f, "wafv2:CreateAPIKey"),
            Wafv2Actions::CreateIpSet => write!(f, "wafv2:CreateIPSet"),
            Wafv2Actions::CreateRegexPatternSet => write!(f, "wafv2:CreateRegexPatternSet"),
            Wafv2Actions::CreateRuleGroup => write!(f, "wafv2:CreateRuleGroup"),
            Wafv2Actions::CreateWebAcl => write!(f, "wafv2:CreateWebACL"),
            Wafv2Actions::DeleteApiKey => write!(f, "wafv2:DeleteAPIKey"),
            Wafv2Actions::DeleteFirewallManagerRuleGroups => {
                write!(f, "wafv2:DeleteFirewallManagerRuleGroups")
            }
            Wafv2Actions::DeleteIpSet => write!(f, "wafv2:DeleteIPSet"),
            Wafv2Actions::DeleteLoggingConfiguration => {
                write!(f, "wafv2:DeleteLoggingConfiguration")
            }
            Wafv2Actions::DeletePermissionPolicy => write!(f, "wafv2:DeletePermissionPolicy"),
            Wafv2Actions::DeleteRegexPatternSet => write!(f, "wafv2:DeleteRegexPatternSet"),
            Wafv2Actions::DeleteRuleGroup => write!(f, "wafv2:DeleteRuleGroup"),
            Wafv2Actions::DeleteWebAcl => write!(f, "wafv2:DeleteWebACL"),
            Wafv2Actions::DescribeAllManagedProducts => {
                write!(f, "wafv2:DescribeAllManagedProducts")
            }
            Wafv2Actions::DescribeManagedProductsByVendor => {
                write!(f, "wafv2:DescribeManagedProductsByVendor")
            }
            Wafv2Actions::DescribeManagedRuleGroup => write!(f, "wafv2:DescribeManagedRuleGroup"),
            Wafv2Actions::DisassociateFirewallManager => {
                write!(f, "wafv2:DisassociateFirewallManager")
            }
            Wafv2Actions::DisassociateWebAcl => write!(f, "wafv2:DisassociateWebACL"),
            Wafv2Actions::GenerateMobileSdkReleaseUrl => {
                write!(f, "wafv2:GenerateMobileSdkReleaseUrl")
            }
            Wafv2Actions::GetDecryptedApiKey => write!(f, "wafv2:GetDecryptedAPIKey"),
            Wafv2Actions::GetIpSet => write!(f, "wafv2:GetIPSet"),
            Wafv2Actions::GetLoggingConfiguration => write!(f, "wafv2:GetLoggingConfiguration"),
            Wafv2Actions::GetManagedRuleSet => write!(f, "wafv2:GetManagedRuleSet"),
            Wafv2Actions::GetMobileSdkRelease => write!(f, "wafv2:GetMobileSdkRelease"),
            Wafv2Actions::GetPermissionPolicy => write!(f, "wafv2:GetPermissionPolicy"),
            Wafv2Actions::GetRateBasedStatementManagedKeys => {
                write!(f, "wafv2:GetRateBasedStatementManagedKeys")
            }
            Wafv2Actions::GetRegexPatternSet => write!(f, "wafv2:GetRegexPatternSet"),
            Wafv2Actions::GetRuleGroup => write!(f, "wafv2:GetRuleGroup"),
            Wafv2Actions::GetSampledRequests => write!(f, "wafv2:GetSampledRequests"),
            Wafv2Actions::GetWebAcl => write!(f, "wafv2:GetWebACL"),
            Wafv2Actions::GetWebAclForResource => write!(f, "wafv2:GetWebACLForResource"),
            Wafv2Actions::ListApiKeys => write!(f, "wafv2:ListAPIKeys"),
            Wafv2Actions::ListAvailableManagedRuleGroupVersions => {
                write!(f, "wafv2:ListAvailableManagedRuleGroupVersions")
            }
            Wafv2Actions::ListAvailableManagedRuleGroups => {
                write!(f, "wafv2:ListAvailableManagedRuleGroups")
            }
            Wafv2Actions::ListIpSets => write!(f, "wafv2:ListIPSets"),
            Wafv2Actions::ListLoggingConfigurations => write!(f, "wafv2:ListLoggingConfigurations"),
            Wafv2Actions::ListManagedRuleSets => write!(f, "wafv2:ListManagedRuleSets"),
            Wafv2Actions::ListMobileSdkReleases => write!(f, "wafv2:ListMobileSdkReleases"),
            Wafv2Actions::ListRegexPatternSets => write!(f, "wafv2:ListRegexPatternSets"),
            Wafv2Actions::ListResourcesForWebAcl => write!(f, "wafv2:ListResourcesForWebACL"),
            Wafv2Actions::ListRuleGroups => write!(f, "wafv2:ListRuleGroups"),
            Wafv2Actions::ListTagsForResource => write!(f, "wafv2:ListTagsForResource"),
            Wafv2Actions::ListWebAcLs => write!(f, "wafv2:ListWebACLs"),
            Wafv2Actions::PutFirewallManagerRuleGroups => {
                write!(f, "wafv2:PutFirewallManagerRuleGroups")
            }
            Wafv2Actions::PutLoggingConfiguration => write!(f, "wafv2:PutLoggingConfiguration"),
            Wafv2Actions::PutManagedRuleSetVersions => write!(f, "wafv2:PutManagedRuleSetVersions"),
            Wafv2Actions::PutPermissionPolicy => write!(f, "wafv2:PutPermissionPolicy"),
            Wafv2Actions::TagResource => write!(f, "wafv2:TagResource"),
            Wafv2Actions::UntagResource => write!(f, "wafv2:UntagResource"),
            Wafv2Actions::UpdateIpSet => write!(f, "wafv2:UpdateIPSet"),
            Wafv2Actions::UpdateManagedRuleSetVersionExpiryDate => {
                write!(f, "wafv2:UpdateManagedRuleSetVersionExpiryDate")
            }
            Wafv2Actions::UpdateRegexPatternSet => write!(f, "wafv2:UpdateRegexPatternSet"),
            Wafv2Actions::UpdateRuleGroup => write!(f, "wafv2:UpdateRuleGroup"),
            Wafv2Actions::UpdateWebAcl => write!(f, "wafv2:UpdateWebACL"),
        }
    }
}
