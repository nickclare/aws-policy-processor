// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum WafRegionalActions {
    AssociateWebAcl,
    CreateByteMatchSet,
    CreateGeoMatchSet,
    CreateIpSet,
    CreateRateBasedRule,
    CreateRegexMatchSet,
    CreateRegexPatternSet,
    CreateRule,
    CreateRuleGroup,
    CreateSizeConstraintSet,
    CreateSqlInjectionMatchSet,
    CreateWebAcl,
    CreateWebAclMigrationStack,
    CreateXssMatchSet,
    DeleteByteMatchSet,
    DeleteGeoMatchSet,
    DeleteIpSet,
    DeleteLoggingConfiguration,
    DeletePermissionPolicy,
    DeleteRateBasedRule,
    DeleteRegexMatchSet,
    DeleteRegexPatternSet,
    DeleteRule,
    DeleteRuleGroup,
    DeleteSizeConstraintSet,
    DeleteSqlInjectionMatchSet,
    DeleteWebAcl,
    DeleteXssMatchSet,
    DisassociateWebAcl,
    GetByteMatchSet,
    GetChangeToken,
    GetChangeTokenStatus,
    GetGeoMatchSet,
    GetIpSet,
    GetLoggingConfiguration,
    GetPermissionPolicy,
    GetRateBasedRule,
    GetRateBasedRuleManagedKeys,
    GetRegexMatchSet,
    GetRegexPatternSet,
    GetRule,
    GetRuleGroup,
    GetSampledRequests,
    GetSizeConstraintSet,
    GetSqlInjectionMatchSet,
    GetWebAcl,
    GetWebAclForResource,
    GetXssMatchSet,
    ListActivatedRulesInRuleGroup,
    ListByteMatchSets,
    ListGeoMatchSets,
    ListIpSets,
    ListLoggingConfigurations,
    ListRateBasedRules,
    ListRegexMatchSets,
    ListRegexPatternSets,
    ListResourcesForWebAcl,
    ListRuleGroups,
    ListRules,
    ListSizeConstraintSets,
    ListSqlInjectionMatchSets,
    ListSubscribedRuleGroups,
    ListTagsForResource,
    ListWebAcLs,
    ListXssMatchSets,
    PutLoggingConfiguration,
    PutPermissionPolicy,
    TagResource,
    UntagResource,
    UpdateByteMatchSet,
    UpdateGeoMatchSet,
    UpdateIpSet,
    UpdateRateBasedRule,
    UpdateRegexMatchSet,
    UpdateRegexPatternSet,
    UpdateRule,
    UpdateRuleGroup,
    UpdateSizeConstraintSet,
    UpdateSqlInjectionMatchSet,
    UpdateWebAcl,
    UpdateXssMatchSet,
}
impl std::fmt::Display for WafRegionalActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WafRegionalActions::AssociateWebAcl => write!(f, "waf-regional:AssociateWebACL"),
            WafRegionalActions::CreateByteMatchSet => write!(f, "waf-regional:CreateByteMatchSet"),
            WafRegionalActions::CreateGeoMatchSet => write!(f, "waf-regional:CreateGeoMatchSet"),
            WafRegionalActions::CreateIpSet => write!(f, "waf-regional:CreateIPSet"),
            WafRegionalActions::CreateRateBasedRule => {
                write!(f, "waf-regional:CreateRateBasedRule")
            }
            WafRegionalActions::CreateRegexMatchSet => {
                write!(f, "waf-regional:CreateRegexMatchSet")
            }
            WafRegionalActions::CreateRegexPatternSet => {
                write!(f, "waf-regional:CreateRegexPatternSet")
            }
            WafRegionalActions::CreateRule => write!(f, "waf-regional:CreateRule"),
            WafRegionalActions::CreateRuleGroup => write!(f, "waf-regional:CreateRuleGroup"),
            WafRegionalActions::CreateSizeConstraintSet => {
                write!(f, "waf-regional:CreateSizeConstraintSet")
            }
            WafRegionalActions::CreateSqlInjectionMatchSet => {
                write!(f, "waf-regional:CreateSqlInjectionMatchSet")
            }
            WafRegionalActions::CreateWebAcl => write!(f, "waf-regional:CreateWebACL"),
            WafRegionalActions::CreateWebAclMigrationStack => {
                write!(f, "waf-regional:CreateWebACLMigrationStack")
            }
            WafRegionalActions::CreateXssMatchSet => write!(f, "waf-regional:CreateXssMatchSet"),
            WafRegionalActions::DeleteByteMatchSet => write!(f, "waf-regional:DeleteByteMatchSet"),
            WafRegionalActions::DeleteGeoMatchSet => write!(f, "waf-regional:DeleteGeoMatchSet"),
            WafRegionalActions::DeleteIpSet => write!(f, "waf-regional:DeleteIPSet"),
            WafRegionalActions::DeleteLoggingConfiguration => {
                write!(f, "waf-regional:DeleteLoggingConfiguration")
            }
            WafRegionalActions::DeletePermissionPolicy => {
                write!(f, "waf-regional:DeletePermissionPolicy")
            }
            WafRegionalActions::DeleteRateBasedRule => {
                write!(f, "waf-regional:DeleteRateBasedRule")
            }
            WafRegionalActions::DeleteRegexMatchSet => {
                write!(f, "waf-regional:DeleteRegexMatchSet")
            }
            WafRegionalActions::DeleteRegexPatternSet => {
                write!(f, "waf-regional:DeleteRegexPatternSet")
            }
            WafRegionalActions::DeleteRule => write!(f, "waf-regional:DeleteRule"),
            WafRegionalActions::DeleteRuleGroup => write!(f, "waf-regional:DeleteRuleGroup"),
            WafRegionalActions::DeleteSizeConstraintSet => {
                write!(f, "waf-regional:DeleteSizeConstraintSet")
            }
            WafRegionalActions::DeleteSqlInjectionMatchSet => {
                write!(f, "waf-regional:DeleteSqlInjectionMatchSet")
            }
            WafRegionalActions::DeleteWebAcl => write!(f, "waf-regional:DeleteWebACL"),
            WafRegionalActions::DeleteXssMatchSet => write!(f, "waf-regional:DeleteXssMatchSet"),
            WafRegionalActions::DisassociateWebAcl => write!(f, "waf-regional:DisassociateWebACL"),
            WafRegionalActions::GetByteMatchSet => write!(f, "waf-regional:GetByteMatchSet"),
            WafRegionalActions::GetChangeToken => write!(f, "waf-regional:GetChangeToken"),
            WafRegionalActions::GetChangeTokenStatus => {
                write!(f, "waf-regional:GetChangeTokenStatus")
            }
            WafRegionalActions::GetGeoMatchSet => write!(f, "waf-regional:GetGeoMatchSet"),
            WafRegionalActions::GetIpSet => write!(f, "waf-regional:GetIPSet"),
            WafRegionalActions::GetLoggingConfiguration => {
                write!(f, "waf-regional:GetLoggingConfiguration")
            }
            WafRegionalActions::GetPermissionPolicy => {
                write!(f, "waf-regional:GetPermissionPolicy")
            }
            WafRegionalActions::GetRateBasedRule => write!(f, "waf-regional:GetRateBasedRule"),
            WafRegionalActions::GetRateBasedRuleManagedKeys => {
                write!(f, "waf-regional:GetRateBasedRuleManagedKeys")
            }
            WafRegionalActions::GetRegexMatchSet => write!(f, "waf-regional:GetRegexMatchSet"),
            WafRegionalActions::GetRegexPatternSet => write!(f, "waf-regional:GetRegexPatternSet"),
            WafRegionalActions::GetRule => write!(f, "waf-regional:GetRule"),
            WafRegionalActions::GetRuleGroup => write!(f, "waf-regional:GetRuleGroup"),
            WafRegionalActions::GetSampledRequests => write!(f, "waf-regional:GetSampledRequests"),
            WafRegionalActions::GetSizeConstraintSet => {
                write!(f, "waf-regional:GetSizeConstraintSet")
            }
            WafRegionalActions::GetSqlInjectionMatchSet => {
                write!(f, "waf-regional:GetSqlInjectionMatchSet")
            }
            WafRegionalActions::GetWebAcl => write!(f, "waf-regional:GetWebACL"),
            WafRegionalActions::GetWebAclForResource => {
                write!(f, "waf-regional:GetWebACLForResource")
            }
            WafRegionalActions::GetXssMatchSet => write!(f, "waf-regional:GetXssMatchSet"),
            WafRegionalActions::ListActivatedRulesInRuleGroup => {
                write!(f, "waf-regional:ListActivatedRulesInRuleGroup")
            }
            WafRegionalActions::ListByteMatchSets => write!(f, "waf-regional:ListByteMatchSets"),
            WafRegionalActions::ListGeoMatchSets => write!(f, "waf-regional:ListGeoMatchSets"),
            WafRegionalActions::ListIpSets => write!(f, "waf-regional:ListIPSets"),
            WafRegionalActions::ListLoggingConfigurations => {
                write!(f, "waf-regional:ListLoggingConfigurations")
            }
            WafRegionalActions::ListRateBasedRules => write!(f, "waf-regional:ListRateBasedRules"),
            WafRegionalActions::ListRegexMatchSets => write!(f, "waf-regional:ListRegexMatchSets"),
            WafRegionalActions::ListRegexPatternSets => {
                write!(f, "waf-regional:ListRegexPatternSets")
            }
            WafRegionalActions::ListResourcesForWebAcl => {
                write!(f, "waf-regional:ListResourcesForWebACL")
            }
            WafRegionalActions::ListRuleGroups => write!(f, "waf-regional:ListRuleGroups"),
            WafRegionalActions::ListRules => write!(f, "waf-regional:ListRules"),
            WafRegionalActions::ListSizeConstraintSets => {
                write!(f, "waf-regional:ListSizeConstraintSets")
            }
            WafRegionalActions::ListSqlInjectionMatchSets => {
                write!(f, "waf-regional:ListSqlInjectionMatchSets")
            }
            WafRegionalActions::ListSubscribedRuleGroups => {
                write!(f, "waf-regional:ListSubscribedRuleGroups")
            }
            WafRegionalActions::ListTagsForResource => {
                write!(f, "waf-regional:ListTagsForResource")
            }
            WafRegionalActions::ListWebAcLs => write!(f, "waf-regional:ListWebACLs"),
            WafRegionalActions::ListXssMatchSets => write!(f, "waf-regional:ListXssMatchSets"),
            WafRegionalActions::PutLoggingConfiguration => {
                write!(f, "waf-regional:PutLoggingConfiguration")
            }
            WafRegionalActions::PutPermissionPolicy => {
                write!(f, "waf-regional:PutPermissionPolicy")
            }
            WafRegionalActions::TagResource => write!(f, "waf-regional:TagResource"),
            WafRegionalActions::UntagResource => write!(f, "waf-regional:UntagResource"),
            WafRegionalActions::UpdateByteMatchSet => write!(f, "waf-regional:UpdateByteMatchSet"),
            WafRegionalActions::UpdateGeoMatchSet => write!(f, "waf-regional:UpdateGeoMatchSet"),
            WafRegionalActions::UpdateIpSet => write!(f, "waf-regional:UpdateIPSet"),
            WafRegionalActions::UpdateRateBasedRule => {
                write!(f, "waf-regional:UpdateRateBasedRule")
            }
            WafRegionalActions::UpdateRegexMatchSet => {
                write!(f, "waf-regional:UpdateRegexMatchSet")
            }
            WafRegionalActions::UpdateRegexPatternSet => {
                write!(f, "waf-regional:UpdateRegexPatternSet")
            }
            WafRegionalActions::UpdateRule => write!(f, "waf-regional:UpdateRule"),
            WafRegionalActions::UpdateRuleGroup => write!(f, "waf-regional:UpdateRuleGroup"),
            WafRegionalActions::UpdateSizeConstraintSet => {
                write!(f, "waf-regional:UpdateSizeConstraintSet")
            }
            WafRegionalActions::UpdateSqlInjectionMatchSet => {
                write!(f, "waf-regional:UpdateSqlInjectionMatchSet")
            }
            WafRegionalActions::UpdateWebAcl => write!(f, "waf-regional:UpdateWebACL"),
            WafRegionalActions::UpdateXssMatchSet => write!(f, "waf-regional:UpdateXssMatchSet"),
        }
    }
}
