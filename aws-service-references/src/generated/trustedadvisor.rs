// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum TrustedadvisorActions {
    BatchUpdateRecommendationResourceExclusion,
    CreateEngagement,
    CreateEngagementAttachment,
    CreateEngagementCommunication,
    DeleteNotificationConfigurationForDelegatedAdmin,
    DescribeAccount,
    DescribeAccountAccess,
    DescribeCheckItems,
    DescribeCheckRefreshStatuses,
    DescribeCheckStatusHistoryChanges,
    DescribeCheckSummaries,
    DescribeChecks,
    DescribeNotificationConfigurations,
    DescribeNotificationPreferences,
    DescribeOrganization,
    DescribeOrganizationAccounts,
    DescribeReports,
    DescribeRisk,
    DescribeRiskResources,
    DescribeRisks,
    DescribeServiceMetadata,
    DownloadRisk,
    ExcludeCheckItems,
    GenerateReport,
    GetEngagement,
    GetEngagementAttachment,
    GetEngagementType,
    GetOrganizationRecommendation,
    GetRecommendation,
    IncludeCheckItems,
    ListAccountsForParent,
    ListChecks,
    ListEngagementCommunications,
    ListEngagementTypes,
    ListEngagements,
    ListOrganizationRecommendationAccounts,
    ListOrganizationRecommendationResources,
    ListOrganizationRecommendations,
    ListOrganizationalUnitsForParent,
    ListRecommendationResources,
    ListRecommendations,
    ListRoots,
    RefreshCheck,
    SetAccountAccess,
    SetOrganizationAccess,
    UpdateEngagement,
    UpdateEngagementStatus,
    UpdateNotificationConfigurations,
    UpdateNotificationPreferences,
    UpdateOrganizationRecommendationLifecycle,
    UpdateRecommendationLifecycle,
    UpdateRiskStatus,
}
impl std::fmt::Display for TrustedadvisorActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TrustedadvisorActions::BatchUpdateRecommendationResourceExclusion => write!(
                f,
                "trustedadvisor:BatchUpdateRecommendationResourceExclusion"
            ),
            TrustedadvisorActions::CreateEngagement => write!(f, "trustedadvisor:CreateEngagement"),
            TrustedadvisorActions::CreateEngagementAttachment => {
                write!(f, "trustedadvisor:CreateEngagementAttachment")
            }
            TrustedadvisorActions::CreateEngagementCommunication => {
                write!(f, "trustedadvisor:CreateEngagementCommunication")
            }
            TrustedadvisorActions::DeleteNotificationConfigurationForDelegatedAdmin => write!(
                f,
                "trustedadvisor:DeleteNotificationConfigurationForDelegatedAdmin"
            ),
            TrustedadvisorActions::DescribeAccount => write!(f, "trustedadvisor:DescribeAccount"),
            TrustedadvisorActions::DescribeAccountAccess => {
                write!(f, "trustedadvisor:DescribeAccountAccess")
            }
            TrustedadvisorActions::DescribeCheckItems => {
                write!(f, "trustedadvisor:DescribeCheckItems")
            }
            TrustedadvisorActions::DescribeCheckRefreshStatuses => {
                write!(f, "trustedadvisor:DescribeCheckRefreshStatuses")
            }
            TrustedadvisorActions::DescribeCheckStatusHistoryChanges => {
                write!(f, "trustedadvisor:DescribeCheckStatusHistoryChanges")
            }
            TrustedadvisorActions::DescribeCheckSummaries => {
                write!(f, "trustedadvisor:DescribeCheckSummaries")
            }
            TrustedadvisorActions::DescribeChecks => write!(f, "trustedadvisor:DescribeChecks"),
            TrustedadvisorActions::DescribeNotificationConfigurations => {
                write!(f, "trustedadvisor:DescribeNotificationConfigurations")
            }
            TrustedadvisorActions::DescribeNotificationPreferences => {
                write!(f, "trustedadvisor:DescribeNotificationPreferences")
            }
            TrustedadvisorActions::DescribeOrganization => {
                write!(f, "trustedadvisor:DescribeOrganization")
            }
            TrustedadvisorActions::DescribeOrganizationAccounts => {
                write!(f, "trustedadvisor:DescribeOrganizationAccounts")
            }
            TrustedadvisorActions::DescribeReports => write!(f, "trustedadvisor:DescribeReports"),
            TrustedadvisorActions::DescribeRisk => write!(f, "trustedadvisor:DescribeRisk"),
            TrustedadvisorActions::DescribeRiskResources => {
                write!(f, "trustedadvisor:DescribeRiskResources")
            }
            TrustedadvisorActions::DescribeRisks => write!(f, "trustedadvisor:DescribeRisks"),
            TrustedadvisorActions::DescribeServiceMetadata => {
                write!(f, "trustedadvisor:DescribeServiceMetadata")
            }
            TrustedadvisorActions::DownloadRisk => write!(f, "trustedadvisor:DownloadRisk"),
            TrustedadvisorActions::ExcludeCheckItems => {
                write!(f, "trustedadvisor:ExcludeCheckItems")
            }
            TrustedadvisorActions::GenerateReport => write!(f, "trustedadvisor:GenerateReport"),
            TrustedadvisorActions::GetEngagement => write!(f, "trustedadvisor:GetEngagement"),
            TrustedadvisorActions::GetEngagementAttachment => {
                write!(f, "trustedadvisor:GetEngagementAttachment")
            }
            TrustedadvisorActions::GetEngagementType => {
                write!(f, "trustedadvisor:GetEngagementType")
            }
            TrustedadvisorActions::GetOrganizationRecommendation => {
                write!(f, "trustedadvisor:GetOrganizationRecommendation")
            }
            TrustedadvisorActions::GetRecommendation => {
                write!(f, "trustedadvisor:GetRecommendation")
            }
            TrustedadvisorActions::IncludeCheckItems => {
                write!(f, "trustedadvisor:IncludeCheckItems")
            }
            TrustedadvisorActions::ListAccountsForParent => {
                write!(f, "trustedadvisor:ListAccountsForParent")
            }
            TrustedadvisorActions::ListChecks => write!(f, "trustedadvisor:ListChecks"),
            TrustedadvisorActions::ListEngagementCommunications => {
                write!(f, "trustedadvisor:ListEngagementCommunications")
            }
            TrustedadvisorActions::ListEngagementTypes => {
                write!(f, "trustedadvisor:ListEngagementTypes")
            }
            TrustedadvisorActions::ListEngagements => write!(f, "trustedadvisor:ListEngagements"),
            TrustedadvisorActions::ListOrganizationRecommendationAccounts => {
                write!(f, "trustedadvisor:ListOrganizationRecommendationAccounts")
            }
            TrustedadvisorActions::ListOrganizationRecommendationResources => {
                write!(f, "trustedadvisor:ListOrganizationRecommendationResources")
            }
            TrustedadvisorActions::ListOrganizationRecommendations => {
                write!(f, "trustedadvisor:ListOrganizationRecommendations")
            }
            TrustedadvisorActions::ListOrganizationalUnitsForParent => {
                write!(f, "trustedadvisor:ListOrganizationalUnitsForParent")
            }
            TrustedadvisorActions::ListRecommendationResources => {
                write!(f, "trustedadvisor:ListRecommendationResources")
            }
            TrustedadvisorActions::ListRecommendations => {
                write!(f, "trustedadvisor:ListRecommendations")
            }
            TrustedadvisorActions::ListRoots => write!(f, "trustedadvisor:ListRoots"),
            TrustedadvisorActions::RefreshCheck => write!(f, "trustedadvisor:RefreshCheck"),
            TrustedadvisorActions::SetAccountAccess => write!(f, "trustedadvisor:SetAccountAccess"),
            TrustedadvisorActions::SetOrganizationAccess => {
                write!(f, "trustedadvisor:SetOrganizationAccess")
            }
            TrustedadvisorActions::UpdateEngagement => write!(f, "trustedadvisor:UpdateEngagement"),
            TrustedadvisorActions::UpdateEngagementStatus => {
                write!(f, "trustedadvisor:UpdateEngagementStatus")
            }
            TrustedadvisorActions::UpdateNotificationConfigurations => {
                write!(f, "trustedadvisor:UpdateNotificationConfigurations")
            }
            TrustedadvisorActions::UpdateNotificationPreferences => {
                write!(f, "trustedadvisor:UpdateNotificationPreferences")
            }
            TrustedadvisorActions::UpdateOrganizationRecommendationLifecycle => write!(
                f,
                "trustedadvisor:UpdateOrganizationRecommendationLifecycle"
            ),
            TrustedadvisorActions::UpdateRecommendationLifecycle => {
                write!(f, "trustedadvisor:UpdateRecommendationLifecycle")
            }
            TrustedadvisorActions::UpdateRiskStatus => write!(f, "trustedadvisor:UpdateRiskStatus"),
        }
    }
}
