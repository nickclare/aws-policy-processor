// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum GuarddutyActions {
    AcceptAdministratorInvitation,
    AcceptInvitation,
    ArchiveFindings,
    CreateDetector,
    CreateFilter,
    CreateIpSet,
    CreateMalwareProtectionPlan,
    CreateMembers,
    CreatePublishingDestination,
    CreateSampleFindings,
    CreateThreatEntitySet,
    CreateThreatIntelSet,
    CreateTrustedEntitySet,
    DeclineInvitations,
    DeleteDetector,
    DeleteFilter,
    DeleteInvitations,
    DeleteIpSet,
    DeleteMalwareProtectionPlan,
    DeleteMembers,
    DeletePublishingDestination,
    DeleteThreatEntitySet,
    DeleteThreatIntelSet,
    DeleteTrustedEntitySet,
    DescribeMalwareScans,
    DescribeOrganizationConfiguration,
    DescribePublishingDestination,
    DisableOrganizationAdminAccount,
    DisassociateFromAdministratorAccount,
    DisassociateFromMasterAccount,
    DisassociateMembers,
    EnableOrganizationAdminAccount,
    GetAdministratorAccount,
    GetCoverageStatistics,
    GetDetector,
    GetFilter,
    GetFindings,
    GetFindingsStatistics,
    GetInvitationsCount,
    GetIpSet,
    GetMalwareProtectionPlan,
    GetMalwareScanSettings,
    GetMasterAccount,
    GetMemberDetectors,
    GetMembers,
    GetOrganizationStatistics,
    GetRemainingFreeTrialDays,
    GetThreatEntitySet,
    GetThreatIntelSet,
    GetTrustedEntitySet,
    GetUsageStatistics,
    InviteMembers,
    ListCoverage,
    ListDetectors,
    ListFilters,
    ListFindings,
    ListInvitations,
    ListIpSets,
    ListMalwareProtectionPlans,
    ListMembers,
    ListOrganizationAdminAccounts,
    ListPublishingDestinations,
    ListTagsForResource,
    ListThreatEntitySets,
    ListThreatIntelSets,
    ListTrustedEntitySets,
    SendSecurityTelemetry,
    StartMalwareScan,
    StartMonitoringMembers,
    StopMonitoringMembers,
    TagResource,
    UnarchiveFindings,
    UntagResource,
    UpdateDetector,
    UpdateFilter,
    UpdateFindingsFeedback,
    UpdateIpSet,
    UpdateMalwareProtectionPlan,
    UpdateMalwareScanSettings,
    UpdateMemberDetectors,
    UpdateOrganizationConfiguration,
    UpdatePublishingDestination,
    UpdateThreatEntitySet,
    UpdateThreatIntelSet,
    UpdateTrustedEntitySet,
}
impl std::fmt::Display for GuarddutyActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GuarddutyActions::AcceptAdministratorInvitation => {
                write!(f, "guardduty:AcceptAdministratorInvitation")
            }
            GuarddutyActions::AcceptInvitation => write!(f, "guardduty:AcceptInvitation"),
            GuarddutyActions::ArchiveFindings => write!(f, "guardduty:ArchiveFindings"),
            GuarddutyActions::CreateDetector => write!(f, "guardduty:CreateDetector"),
            GuarddutyActions::CreateFilter => write!(f, "guardduty:CreateFilter"),
            GuarddutyActions::CreateIpSet => write!(f, "guardduty:CreateIPSet"),
            GuarddutyActions::CreateMalwareProtectionPlan => {
                write!(f, "guardduty:CreateMalwareProtectionPlan")
            }
            GuarddutyActions::CreateMembers => write!(f, "guardduty:CreateMembers"),
            GuarddutyActions::CreatePublishingDestination => {
                write!(f, "guardduty:CreatePublishingDestination")
            }
            GuarddutyActions::CreateSampleFindings => write!(f, "guardduty:CreateSampleFindings"),
            GuarddutyActions::CreateThreatEntitySet => write!(f, "guardduty:CreateThreatEntitySet"),
            GuarddutyActions::CreateThreatIntelSet => write!(f, "guardduty:CreateThreatIntelSet"),
            GuarddutyActions::CreateTrustedEntitySet => {
                write!(f, "guardduty:CreateTrustedEntitySet")
            }
            GuarddutyActions::DeclineInvitations => write!(f, "guardduty:DeclineInvitations"),
            GuarddutyActions::DeleteDetector => write!(f, "guardduty:DeleteDetector"),
            GuarddutyActions::DeleteFilter => write!(f, "guardduty:DeleteFilter"),
            GuarddutyActions::DeleteInvitations => write!(f, "guardduty:DeleteInvitations"),
            GuarddutyActions::DeleteIpSet => write!(f, "guardduty:DeleteIPSet"),
            GuarddutyActions::DeleteMalwareProtectionPlan => {
                write!(f, "guardduty:DeleteMalwareProtectionPlan")
            }
            GuarddutyActions::DeleteMembers => write!(f, "guardduty:DeleteMembers"),
            GuarddutyActions::DeletePublishingDestination => {
                write!(f, "guardduty:DeletePublishingDestination")
            }
            GuarddutyActions::DeleteThreatEntitySet => write!(f, "guardduty:DeleteThreatEntitySet"),
            GuarddutyActions::DeleteThreatIntelSet => write!(f, "guardduty:DeleteThreatIntelSet"),
            GuarddutyActions::DeleteTrustedEntitySet => {
                write!(f, "guardduty:DeleteTrustedEntitySet")
            }
            GuarddutyActions::DescribeMalwareScans => write!(f, "guardduty:DescribeMalwareScans"),
            GuarddutyActions::DescribeOrganizationConfiguration => {
                write!(f, "guardduty:DescribeOrganizationConfiguration")
            }
            GuarddutyActions::DescribePublishingDestination => {
                write!(f, "guardduty:DescribePublishingDestination")
            }
            GuarddutyActions::DisableOrganizationAdminAccount => {
                write!(f, "guardduty:DisableOrganizationAdminAccount")
            }
            GuarddutyActions::DisassociateFromAdministratorAccount => {
                write!(f, "guardduty:DisassociateFromAdministratorAccount")
            }
            GuarddutyActions::DisassociateFromMasterAccount => {
                write!(f, "guardduty:DisassociateFromMasterAccount")
            }
            GuarddutyActions::DisassociateMembers => write!(f, "guardduty:DisassociateMembers"),
            GuarddutyActions::EnableOrganizationAdminAccount => {
                write!(f, "guardduty:EnableOrganizationAdminAccount")
            }
            GuarddutyActions::GetAdministratorAccount => {
                write!(f, "guardduty:GetAdministratorAccount")
            }
            GuarddutyActions::GetCoverageStatistics => write!(f, "guardduty:GetCoverageStatistics"),
            GuarddutyActions::GetDetector => write!(f, "guardduty:GetDetector"),
            GuarddutyActions::GetFilter => write!(f, "guardduty:GetFilter"),
            GuarddutyActions::GetFindings => write!(f, "guardduty:GetFindings"),
            GuarddutyActions::GetFindingsStatistics => write!(f, "guardduty:GetFindingsStatistics"),
            GuarddutyActions::GetInvitationsCount => write!(f, "guardduty:GetInvitationsCount"),
            GuarddutyActions::GetIpSet => write!(f, "guardduty:GetIPSet"),
            GuarddutyActions::GetMalwareProtectionPlan => {
                write!(f, "guardduty:GetMalwareProtectionPlan")
            }
            GuarddutyActions::GetMalwareScanSettings => {
                write!(f, "guardduty:GetMalwareScanSettings")
            }
            GuarddutyActions::GetMasterAccount => write!(f, "guardduty:GetMasterAccount"),
            GuarddutyActions::GetMemberDetectors => write!(f, "guardduty:GetMemberDetectors"),
            GuarddutyActions::GetMembers => write!(f, "guardduty:GetMembers"),
            GuarddutyActions::GetOrganizationStatistics => {
                write!(f, "guardduty:GetOrganizationStatistics")
            }
            GuarddutyActions::GetRemainingFreeTrialDays => {
                write!(f, "guardduty:GetRemainingFreeTrialDays")
            }
            GuarddutyActions::GetThreatEntitySet => write!(f, "guardduty:GetThreatEntitySet"),
            GuarddutyActions::GetThreatIntelSet => write!(f, "guardduty:GetThreatIntelSet"),
            GuarddutyActions::GetTrustedEntitySet => write!(f, "guardduty:GetTrustedEntitySet"),
            GuarddutyActions::GetUsageStatistics => write!(f, "guardduty:GetUsageStatistics"),
            GuarddutyActions::InviteMembers => write!(f, "guardduty:InviteMembers"),
            GuarddutyActions::ListCoverage => write!(f, "guardduty:ListCoverage"),
            GuarddutyActions::ListDetectors => write!(f, "guardduty:ListDetectors"),
            GuarddutyActions::ListFilters => write!(f, "guardduty:ListFilters"),
            GuarddutyActions::ListFindings => write!(f, "guardduty:ListFindings"),
            GuarddutyActions::ListInvitations => write!(f, "guardduty:ListInvitations"),
            GuarddutyActions::ListIpSets => write!(f, "guardduty:ListIPSets"),
            GuarddutyActions::ListMalwareProtectionPlans => {
                write!(f, "guardduty:ListMalwareProtectionPlans")
            }
            GuarddutyActions::ListMembers => write!(f, "guardduty:ListMembers"),
            GuarddutyActions::ListOrganizationAdminAccounts => {
                write!(f, "guardduty:ListOrganizationAdminAccounts")
            }
            GuarddutyActions::ListPublishingDestinations => {
                write!(f, "guardduty:ListPublishingDestinations")
            }
            GuarddutyActions::ListTagsForResource => write!(f, "guardduty:ListTagsForResource"),
            GuarddutyActions::ListThreatEntitySets => write!(f, "guardduty:ListThreatEntitySets"),
            GuarddutyActions::ListThreatIntelSets => write!(f, "guardduty:ListThreatIntelSets"),
            GuarddutyActions::ListTrustedEntitySets => write!(f, "guardduty:ListTrustedEntitySets"),
            GuarddutyActions::SendSecurityTelemetry => write!(f, "guardduty:SendSecurityTelemetry"),
            GuarddutyActions::StartMalwareScan => write!(f, "guardduty:StartMalwareScan"),
            GuarddutyActions::StartMonitoringMembers => {
                write!(f, "guardduty:StartMonitoringMembers")
            }
            GuarddutyActions::StopMonitoringMembers => write!(f, "guardduty:StopMonitoringMembers"),
            GuarddutyActions::TagResource => write!(f, "guardduty:TagResource"),
            GuarddutyActions::UnarchiveFindings => write!(f, "guardduty:UnarchiveFindings"),
            GuarddutyActions::UntagResource => write!(f, "guardduty:UntagResource"),
            GuarddutyActions::UpdateDetector => write!(f, "guardduty:UpdateDetector"),
            GuarddutyActions::UpdateFilter => write!(f, "guardduty:UpdateFilter"),
            GuarddutyActions::UpdateFindingsFeedback => {
                write!(f, "guardduty:UpdateFindingsFeedback")
            }
            GuarddutyActions::UpdateIpSet => write!(f, "guardduty:UpdateIPSet"),
            GuarddutyActions::UpdateMalwareProtectionPlan => {
                write!(f, "guardduty:UpdateMalwareProtectionPlan")
            }
            GuarddutyActions::UpdateMalwareScanSettings => {
                write!(f, "guardduty:UpdateMalwareScanSettings")
            }
            GuarddutyActions::UpdateMemberDetectors => write!(f, "guardduty:UpdateMemberDetectors"),
            GuarddutyActions::UpdateOrganizationConfiguration => {
                write!(f, "guardduty:UpdateOrganizationConfiguration")
            }
            GuarddutyActions::UpdatePublishingDestination => {
                write!(f, "guardduty:UpdatePublishingDestination")
            }
            GuarddutyActions::UpdateThreatEntitySet => write!(f, "guardduty:UpdateThreatEntitySet"),
            GuarddutyActions::UpdateThreatIntelSet => write!(f, "guardduty:UpdateThreatIntelSet"),
            GuarddutyActions::UpdateTrustedEntitySet => {
                write!(f, "guardduty:UpdateTrustedEntitySet")
            }
        }
    }
}
