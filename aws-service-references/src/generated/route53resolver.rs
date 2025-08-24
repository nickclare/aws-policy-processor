// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum Route53resolverActions {
    AssociateFirewallRuleGroup,
    AssociateResolverEndpointIpAddress,
    AssociateResolverQueryLogConfig,
    AssociateResolverRule,
    CreateFirewallDomainList,
    CreateFirewallRule,
    CreateFirewallRuleGroup,
    CreateOutpostResolver,
    CreateResolverEndpoint,
    CreateResolverQueryLogConfig,
    CreateResolverRule,
    DeleteFirewallDomainList,
    DeleteFirewallRule,
    DeleteFirewallRuleGroup,
    DeleteOutpostResolver,
    DeleteResolverEndpoint,
    DeleteResolverQueryLogConfig,
    DeleteResolverRule,
    DisassociateFirewallRuleGroup,
    DisassociateResolverEndpointIpAddress,
    DisassociateResolverQueryLogConfig,
    DisassociateResolverRule,
    GetFirewallConfig,
    GetFirewallDomainList,
    GetFirewallRuleGroup,
    GetFirewallRuleGroupAssociation,
    GetFirewallRuleGroupPolicy,
    GetOutpostResolver,
    GetResolverConfig,
    GetResolverDnssecConfig,
    GetResolverEndpoint,
    GetResolverQueryLogConfig,
    GetResolverQueryLogConfigAssociation,
    GetResolverQueryLogConfigPolicy,
    GetResolverRule,
    GetResolverRuleAssociation,
    GetResolverRulePolicy,
    ImportFirewallDomains,
    ListFirewallConfigs,
    ListFirewallDomainLists,
    ListFirewallDomains,
    ListFirewallRuleGroupAssociations,
    ListFirewallRuleGroups,
    ListFirewallRules,
    ListOutpostResolvers,
    ListResolverConfigs,
    ListResolverDnssecConfigs,
    ListResolverEndpointIpAddresses,
    ListResolverEndpoints,
    ListResolverQueryLogConfigAssociations,
    ListResolverQueryLogConfigs,
    ListResolverRuleAssociations,
    ListResolverRules,
    ListTagsForResource,
    PutFirewallRuleGroupPolicy,
    PutResolverQueryLogConfigPolicy,
    PutResolverRulePolicy,
    TagResource,
    UntagResource,
    UpdateFirewallConfig,
    UpdateFirewallDomains,
    UpdateFirewallRule,
    UpdateFirewallRuleGroupAssociation,
    UpdateOutpostResolver,
    UpdateResolverConfig,
    UpdateResolverDnssecConfig,
    UpdateResolverEndpoint,
    UpdateResolverRule,
}
impl std::fmt::Display for Route53resolverActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Route53resolverActions::AssociateFirewallRuleGroup => {
                write!(f, "route53resolver:AssociateFirewallRuleGroup")
            }
            Route53resolverActions::AssociateResolverEndpointIpAddress => {
                write!(f, "route53resolver:AssociateResolverEndpointIpAddress")
            }
            Route53resolverActions::AssociateResolverQueryLogConfig => {
                write!(f, "route53resolver:AssociateResolverQueryLogConfig")
            }
            Route53resolverActions::AssociateResolverRule => {
                write!(f, "route53resolver:AssociateResolverRule")
            }
            Route53resolverActions::CreateFirewallDomainList => {
                write!(f, "route53resolver:CreateFirewallDomainList")
            }
            Route53resolverActions::CreateFirewallRule => {
                write!(f, "route53resolver:CreateFirewallRule")
            }
            Route53resolverActions::CreateFirewallRuleGroup => {
                write!(f, "route53resolver:CreateFirewallRuleGroup")
            }
            Route53resolverActions::CreateOutpostResolver => {
                write!(f, "route53resolver:CreateOutpostResolver")
            }
            Route53resolverActions::CreateResolverEndpoint => {
                write!(f, "route53resolver:CreateResolverEndpoint")
            }
            Route53resolverActions::CreateResolverQueryLogConfig => {
                write!(f, "route53resolver:CreateResolverQueryLogConfig")
            }
            Route53resolverActions::CreateResolverRule => {
                write!(f, "route53resolver:CreateResolverRule")
            }
            Route53resolverActions::DeleteFirewallDomainList => {
                write!(f, "route53resolver:DeleteFirewallDomainList")
            }
            Route53resolverActions::DeleteFirewallRule => {
                write!(f, "route53resolver:DeleteFirewallRule")
            }
            Route53resolverActions::DeleteFirewallRuleGroup => {
                write!(f, "route53resolver:DeleteFirewallRuleGroup")
            }
            Route53resolverActions::DeleteOutpostResolver => {
                write!(f, "route53resolver:DeleteOutpostResolver")
            }
            Route53resolverActions::DeleteResolverEndpoint => {
                write!(f, "route53resolver:DeleteResolverEndpoint")
            }
            Route53resolverActions::DeleteResolverQueryLogConfig => {
                write!(f, "route53resolver:DeleteResolverQueryLogConfig")
            }
            Route53resolverActions::DeleteResolverRule => {
                write!(f, "route53resolver:DeleteResolverRule")
            }
            Route53resolverActions::DisassociateFirewallRuleGroup => {
                write!(f, "route53resolver:DisassociateFirewallRuleGroup")
            }
            Route53resolverActions::DisassociateResolverEndpointIpAddress => {
                write!(f, "route53resolver:DisassociateResolverEndpointIpAddress")
            }
            Route53resolverActions::DisassociateResolverQueryLogConfig => {
                write!(f, "route53resolver:DisassociateResolverQueryLogConfig")
            }
            Route53resolverActions::DisassociateResolverRule => {
                write!(f, "route53resolver:DisassociateResolverRule")
            }
            Route53resolverActions::GetFirewallConfig => {
                write!(f, "route53resolver:GetFirewallConfig")
            }
            Route53resolverActions::GetFirewallDomainList => {
                write!(f, "route53resolver:GetFirewallDomainList")
            }
            Route53resolverActions::GetFirewallRuleGroup => {
                write!(f, "route53resolver:GetFirewallRuleGroup")
            }
            Route53resolverActions::GetFirewallRuleGroupAssociation => {
                write!(f, "route53resolver:GetFirewallRuleGroupAssociation")
            }
            Route53resolverActions::GetFirewallRuleGroupPolicy => {
                write!(f, "route53resolver:GetFirewallRuleGroupPolicy")
            }
            Route53resolverActions::GetOutpostResolver => {
                write!(f, "route53resolver:GetOutpostResolver")
            }
            Route53resolverActions::GetResolverConfig => {
                write!(f, "route53resolver:GetResolverConfig")
            }
            Route53resolverActions::GetResolverDnssecConfig => {
                write!(f, "route53resolver:GetResolverDnssecConfig")
            }
            Route53resolverActions::GetResolverEndpoint => {
                write!(f, "route53resolver:GetResolverEndpoint")
            }
            Route53resolverActions::GetResolverQueryLogConfig => {
                write!(f, "route53resolver:GetResolverQueryLogConfig")
            }
            Route53resolverActions::GetResolverQueryLogConfigAssociation => {
                write!(f, "route53resolver:GetResolverQueryLogConfigAssociation")
            }
            Route53resolverActions::GetResolverQueryLogConfigPolicy => {
                write!(f, "route53resolver:GetResolverQueryLogConfigPolicy")
            }
            Route53resolverActions::GetResolverRule => write!(f, "route53resolver:GetResolverRule"),
            Route53resolverActions::GetResolverRuleAssociation => {
                write!(f, "route53resolver:GetResolverRuleAssociation")
            }
            Route53resolverActions::GetResolverRulePolicy => {
                write!(f, "route53resolver:GetResolverRulePolicy")
            }
            Route53resolverActions::ImportFirewallDomains => {
                write!(f, "route53resolver:ImportFirewallDomains")
            }
            Route53resolverActions::ListFirewallConfigs => {
                write!(f, "route53resolver:ListFirewallConfigs")
            }
            Route53resolverActions::ListFirewallDomainLists => {
                write!(f, "route53resolver:ListFirewallDomainLists")
            }
            Route53resolverActions::ListFirewallDomains => {
                write!(f, "route53resolver:ListFirewallDomains")
            }
            Route53resolverActions::ListFirewallRuleGroupAssociations => {
                write!(f, "route53resolver:ListFirewallRuleGroupAssociations")
            }
            Route53resolverActions::ListFirewallRuleGroups => {
                write!(f, "route53resolver:ListFirewallRuleGroups")
            }
            Route53resolverActions::ListFirewallRules => {
                write!(f, "route53resolver:ListFirewallRules")
            }
            Route53resolverActions::ListOutpostResolvers => {
                write!(f, "route53resolver:ListOutpostResolvers")
            }
            Route53resolverActions::ListResolverConfigs => {
                write!(f, "route53resolver:ListResolverConfigs")
            }
            Route53resolverActions::ListResolverDnssecConfigs => {
                write!(f, "route53resolver:ListResolverDnssecConfigs")
            }
            Route53resolverActions::ListResolverEndpointIpAddresses => {
                write!(f, "route53resolver:ListResolverEndpointIpAddresses")
            }
            Route53resolverActions::ListResolverEndpoints => {
                write!(f, "route53resolver:ListResolverEndpoints")
            }
            Route53resolverActions::ListResolverQueryLogConfigAssociations => {
                write!(f, "route53resolver:ListResolverQueryLogConfigAssociations")
            }
            Route53resolverActions::ListResolverQueryLogConfigs => {
                write!(f, "route53resolver:ListResolverQueryLogConfigs")
            }
            Route53resolverActions::ListResolverRuleAssociations => {
                write!(f, "route53resolver:ListResolverRuleAssociations")
            }
            Route53resolverActions::ListResolverRules => {
                write!(f, "route53resolver:ListResolverRules")
            }
            Route53resolverActions::ListTagsForResource => {
                write!(f, "route53resolver:ListTagsForResource")
            }
            Route53resolverActions::PutFirewallRuleGroupPolicy => {
                write!(f, "route53resolver:PutFirewallRuleGroupPolicy")
            }
            Route53resolverActions::PutResolverQueryLogConfigPolicy => {
                write!(f, "route53resolver:PutResolverQueryLogConfigPolicy")
            }
            Route53resolverActions::PutResolverRulePolicy => {
                write!(f, "route53resolver:PutResolverRulePolicy")
            }
            Route53resolverActions::TagResource => write!(f, "route53resolver:TagResource"),
            Route53resolverActions::UntagResource => write!(f, "route53resolver:UntagResource"),
            Route53resolverActions::UpdateFirewallConfig => {
                write!(f, "route53resolver:UpdateFirewallConfig")
            }
            Route53resolverActions::UpdateFirewallDomains => {
                write!(f, "route53resolver:UpdateFirewallDomains")
            }
            Route53resolverActions::UpdateFirewallRule => {
                write!(f, "route53resolver:UpdateFirewallRule")
            }
            Route53resolverActions::UpdateFirewallRuleGroupAssociation => {
                write!(f, "route53resolver:UpdateFirewallRuleGroupAssociation")
            }
            Route53resolverActions::UpdateOutpostResolver => {
                write!(f, "route53resolver:UpdateOutpostResolver")
            }
            Route53resolverActions::UpdateResolverConfig => {
                write!(f, "route53resolver:UpdateResolverConfig")
            }
            Route53resolverActions::UpdateResolverDnssecConfig => {
                write!(f, "route53resolver:UpdateResolverDnssecConfig")
            }
            Route53resolverActions::UpdateResolverEndpoint => {
                write!(f, "route53resolver:UpdateResolverEndpoint")
            }
            Route53resolverActions::UpdateResolverRule => {
                write!(f, "route53resolver:UpdateResolverRule")
            }
        }
    }
}
