// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum BillingconductorActions {
    AssociateAccounts,
    AssociatePricingRules,
    BatchAssociateResourcesToCustomLineItem,
    BatchDisassociateResourcesFromCustomLineItem,
    CreateBillingGroup,
    CreateCustomLineItem,
    CreatePricingPlan,
    CreatePricingRule,
    DeleteBillingGroup,
    DeleteCustomLineItem,
    DeletePricingPlan,
    DeletePricingRule,
    DisassociateAccounts,
    DisassociatePricingRules,
    GetBillingGroupCostReport,
    ListAccountAssociations,
    ListBillingGroupCostReports,
    ListBillingGroups,
    ListCustomLineItemVersions,
    ListCustomLineItems,
    ListPricingPlans,
    ListPricingPlansAssociatedWithPricingRule,
    ListPricingRules,
    ListPricingRulesAssociatedToPricingPlan,
    ListResourcesAssociatedToCustomLineItem,
    ListTagsForResource,
    TagResource,
    UntagResource,
    UpdateBillingGroup,
    UpdateCustomLineItem,
    UpdatePricingPlan,
    UpdatePricingRule,
}
impl std::fmt::Display for BillingconductorActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BillingconductorActions::AssociateAccounts => {
                write!(f, "billingconductor:AssociateAccounts")
            }
            BillingconductorActions::AssociatePricingRules => {
                write!(f, "billingconductor:AssociatePricingRules")
            }
            BillingconductorActions::BatchAssociateResourcesToCustomLineItem => write!(
                f,
                "billingconductor:BatchAssociateResourcesToCustomLineItem"
            ),
            BillingconductorActions::BatchDisassociateResourcesFromCustomLineItem => write!(
                f,
                "billingconductor:BatchDisassociateResourcesFromCustomLineItem"
            ),
            BillingconductorActions::CreateBillingGroup => {
                write!(f, "billingconductor:CreateBillingGroup")
            }
            BillingconductorActions::CreateCustomLineItem => {
                write!(f, "billingconductor:CreateCustomLineItem")
            }
            BillingconductorActions::CreatePricingPlan => {
                write!(f, "billingconductor:CreatePricingPlan")
            }
            BillingconductorActions::CreatePricingRule => {
                write!(f, "billingconductor:CreatePricingRule")
            }
            BillingconductorActions::DeleteBillingGroup => {
                write!(f, "billingconductor:DeleteBillingGroup")
            }
            BillingconductorActions::DeleteCustomLineItem => {
                write!(f, "billingconductor:DeleteCustomLineItem")
            }
            BillingconductorActions::DeletePricingPlan => {
                write!(f, "billingconductor:DeletePricingPlan")
            }
            BillingconductorActions::DeletePricingRule => {
                write!(f, "billingconductor:DeletePricingRule")
            }
            BillingconductorActions::DisassociateAccounts => {
                write!(f, "billingconductor:DisassociateAccounts")
            }
            BillingconductorActions::DisassociatePricingRules => {
                write!(f, "billingconductor:DisassociatePricingRules")
            }
            BillingconductorActions::GetBillingGroupCostReport => {
                write!(f, "billingconductor:GetBillingGroupCostReport")
            }
            BillingconductorActions::ListAccountAssociations => {
                write!(f, "billingconductor:ListAccountAssociations")
            }
            BillingconductorActions::ListBillingGroupCostReports => {
                write!(f, "billingconductor:ListBillingGroupCostReports")
            }
            BillingconductorActions::ListBillingGroups => {
                write!(f, "billingconductor:ListBillingGroups")
            }
            BillingconductorActions::ListCustomLineItemVersions => {
                write!(f, "billingconductor:ListCustomLineItemVersions")
            }
            BillingconductorActions::ListCustomLineItems => {
                write!(f, "billingconductor:ListCustomLineItems")
            }
            BillingconductorActions::ListPricingPlans => {
                write!(f, "billingconductor:ListPricingPlans")
            }
            BillingconductorActions::ListPricingPlansAssociatedWithPricingRule => write!(
                f,
                "billingconductor:ListPricingPlansAssociatedWithPricingRule"
            ),
            BillingconductorActions::ListPricingRules => {
                write!(f, "billingconductor:ListPricingRules")
            }
            BillingconductorActions::ListPricingRulesAssociatedToPricingPlan => write!(
                f,
                "billingconductor:ListPricingRulesAssociatedToPricingPlan"
            ),
            BillingconductorActions::ListResourcesAssociatedToCustomLineItem => write!(
                f,
                "billingconductor:ListResourcesAssociatedToCustomLineItem"
            ),
            BillingconductorActions::ListTagsForResource => {
                write!(f, "billingconductor:ListTagsForResource")
            }
            BillingconductorActions::TagResource => write!(f, "billingconductor:TagResource"),
            BillingconductorActions::UntagResource => write!(f, "billingconductor:UntagResource"),
            BillingconductorActions::UpdateBillingGroup => {
                write!(f, "billingconductor:UpdateBillingGroup")
            }
            BillingconductorActions::UpdateCustomLineItem => {
                write!(f, "billingconductor:UpdateCustomLineItem")
            }
            BillingconductorActions::UpdatePricingPlan => {
                write!(f, "billingconductor:UpdatePricingPlan")
            }
            BillingconductorActions::UpdatePricingRule => {
                write!(f, "billingconductor:UpdatePricingRule")
            }
        }
    }
}
