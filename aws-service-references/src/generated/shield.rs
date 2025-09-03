// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum ShieldActions {
    AssociateDrtLogBucket,
    AssociateDrtRole,
    AssociateHealthCheck,
    AssociateProactiveEngagementDetails,
    CreateProtection,
    CreateProtectionGroup,
    CreateSubscription,
    DeleteProtection,
    DeleteProtectionGroup,
    DeleteSubscription,
    DescribeAttack,
    DescribeAttackContributors,
    DescribeAttackStatistics,
    DescribeDrtAccess,
    DescribeEmergencyContactSettings,
    DescribeProtection,
    DescribeProtectionGroup,
    DescribeSubscription,
    DisableApplicationLayerAutomaticResponse,
    DisableProactiveEngagement,
    DisassociateDrtLogBucket,
    DisassociateDrtRole,
    DisassociateHealthCheck,
    EnableApplicationLayerAutomaticResponse,
    EnableProactiveEngagement,
    GetGlobalThreatData,
    GetSubscriptionState,
    ListAttacks,
    ListMitigations,
    ListProtectionGroups,
    ListProtections,
    ListResourcesInProtectionGroup,
    ListTagsForResource,
    TagResource,
    UntagResource,
    UpdateApplicationLayerAutomaticResponse,
    UpdateEmergencyContactSettings,
    UpdateProtectionGroup,
    UpdateSubscription,
}
impl std::fmt::Display for ShieldActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShieldActions::AssociateDrtLogBucket => write!(f, "shield:AssociateDRTLogBucket"),
            ShieldActions::AssociateDrtRole => write!(f, "shield:AssociateDRTRole"),
            ShieldActions::AssociateHealthCheck => write!(f, "shield:AssociateHealthCheck"),
            ShieldActions::AssociateProactiveEngagementDetails => {
                write!(f, "shield:AssociateProactiveEngagementDetails")
            }
            ShieldActions::CreateProtection => write!(f, "shield:CreateProtection"),
            ShieldActions::CreateProtectionGroup => write!(f, "shield:CreateProtectionGroup"),
            ShieldActions::CreateSubscription => write!(f, "shield:CreateSubscription"),
            ShieldActions::DeleteProtection => write!(f, "shield:DeleteProtection"),
            ShieldActions::DeleteProtectionGroup => write!(f, "shield:DeleteProtectionGroup"),
            ShieldActions::DeleteSubscription => write!(f, "shield:DeleteSubscription"),
            ShieldActions::DescribeAttack => write!(f, "shield:DescribeAttack"),
            ShieldActions::DescribeAttackContributors => {
                write!(f, "shield:DescribeAttackContributors")
            }
            ShieldActions::DescribeAttackStatistics => write!(f, "shield:DescribeAttackStatistics"),
            ShieldActions::DescribeDrtAccess => write!(f, "shield:DescribeDRTAccess"),
            ShieldActions::DescribeEmergencyContactSettings => {
                write!(f, "shield:DescribeEmergencyContactSettings")
            }
            ShieldActions::DescribeProtection => write!(f, "shield:DescribeProtection"),
            ShieldActions::DescribeProtectionGroup => write!(f, "shield:DescribeProtectionGroup"),
            ShieldActions::DescribeSubscription => write!(f, "shield:DescribeSubscription"),
            ShieldActions::DisableApplicationLayerAutomaticResponse => {
                write!(f, "shield:DisableApplicationLayerAutomaticResponse")
            }
            ShieldActions::DisableProactiveEngagement => {
                write!(f, "shield:DisableProactiveEngagement")
            }
            ShieldActions::DisassociateDrtLogBucket => write!(f, "shield:DisassociateDRTLogBucket"),
            ShieldActions::DisassociateDrtRole => write!(f, "shield:DisassociateDRTRole"),
            ShieldActions::DisassociateHealthCheck => write!(f, "shield:DisassociateHealthCheck"),
            ShieldActions::EnableApplicationLayerAutomaticResponse => {
                write!(f, "shield:EnableApplicationLayerAutomaticResponse")
            }
            ShieldActions::EnableProactiveEngagement => {
                write!(f, "shield:EnableProactiveEngagement")
            }
            ShieldActions::GetGlobalThreatData => write!(f, "shield:GetGlobalThreatData"),
            ShieldActions::GetSubscriptionState => write!(f, "shield:GetSubscriptionState"),
            ShieldActions::ListAttacks => write!(f, "shield:ListAttacks"),
            ShieldActions::ListMitigations => write!(f, "shield:ListMitigations"),
            ShieldActions::ListProtectionGroups => write!(f, "shield:ListProtectionGroups"),
            ShieldActions::ListProtections => write!(f, "shield:ListProtections"),
            ShieldActions::ListResourcesInProtectionGroup => {
                write!(f, "shield:ListResourcesInProtectionGroup")
            }
            ShieldActions::ListTagsForResource => write!(f, "shield:ListTagsForResource"),
            ShieldActions::TagResource => write!(f, "shield:TagResource"),
            ShieldActions::UntagResource => write!(f, "shield:UntagResource"),
            ShieldActions::UpdateApplicationLayerAutomaticResponse => {
                write!(f, "shield:UpdateApplicationLayerAutomaticResponse")
            }
            ShieldActions::UpdateEmergencyContactSettings => {
                write!(f, "shield:UpdateEmergencyContactSettings")
            }
            ShieldActions::UpdateProtectionGroup => write!(f, "shield:UpdateProtectionGroup"),
            ShieldActions::UpdateSubscription => write!(f, "shield:UpdateSubscription"),
        }
    }
}
