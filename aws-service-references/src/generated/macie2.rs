// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum Macie2Actions {
    AcceptInvitation,
    BatchGetCustomDataIdentifiers,
    BatchUpdateAutomatedDiscoveryAccounts,
    CreateAllowList,
    CreateClassificationJob,
    CreateCustomDataIdentifier,
    CreateFindingsFilter,
    CreateInvitations,
    CreateMember,
    CreateSampleFindings,
    DeclineInvitations,
    DeleteAllowList,
    DeleteCustomDataIdentifier,
    DeleteFindingsFilter,
    DeleteInvitations,
    DeleteMember,
    DescribeBuckets,
    DescribeClassificationJob,
    DescribeOrganizationConfiguration,
    DisableMacie,
    DisableOrganizationAdminAccount,
    DisassociateFromAdministratorAccount,
    DisassociateFromMasterAccount,
    DisassociateMember,
    EnableMacie,
    EnableOrganizationAdminAccount,
    GetAdministratorAccount,
    GetAllowList,
    GetAutomatedDiscoveryConfiguration,
    GetBucketStatistics,
    GetClassificationExportConfiguration,
    GetClassificationScope,
    GetCustomDataIdentifier,
    GetFindingStatistics,
    GetFindings,
    GetFindingsFilter,
    GetFindingsPublicationConfiguration,
    GetInvitationsCount,
    GetMacieSession,
    GetMasterAccount,
    GetMember,
    GetResourceProfile,
    GetRevealConfiguration,
    GetSensitiveDataOccurrences,
    GetSensitiveDataOccurrencesAvailability,
    GetSensitivityInspectionTemplate,
    GetUsageStatistics,
    GetUsageTotals,
    ListAllowLists,
    ListAutomatedDiscoveryAccounts,
    ListClassificationJobs,
    ListClassificationScopes,
    ListCustomDataIdentifiers,
    ListFindings,
    ListFindingsFilters,
    ListInvitations,
    ListManagedDataIdentifiers,
    ListMembers,
    ListOrganizationAdminAccounts,
    ListResourceProfileArtifacts,
    ListResourceProfileDetections,
    ListSensitivityInspectionTemplates,
    ListTagsForResource,
    PutClassificationExportConfiguration,
    PutFindingsPublicationConfiguration,
    SearchResources,
    TagResource,
    TestCustomDataIdentifier,
    UntagResource,
    UpdateAllowList,
    UpdateAutomatedDiscoveryConfiguration,
    UpdateClassificationJob,
    UpdateClassificationScope,
    UpdateFindingsFilter,
    UpdateMacieSession,
    UpdateMemberSession,
    UpdateOrganizationConfiguration,
    UpdateResourceProfile,
    UpdateResourceProfileDetections,
    UpdateRevealConfiguration,
    UpdateSensitivityInspectionTemplate,
}
impl std::fmt::Display for Macie2Actions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Macie2Actions::AcceptInvitation => write!(f, "macie2:AcceptInvitation"),
            Macie2Actions::BatchGetCustomDataIdentifiers => {
                write!(f, "macie2:BatchGetCustomDataIdentifiers")
            }
            Macie2Actions::BatchUpdateAutomatedDiscoveryAccounts => {
                write!(f, "macie2:BatchUpdateAutomatedDiscoveryAccounts")
            }
            Macie2Actions::CreateAllowList => write!(f, "macie2:CreateAllowList"),
            Macie2Actions::CreateClassificationJob => write!(f, "macie2:CreateClassificationJob"),
            Macie2Actions::CreateCustomDataIdentifier => {
                write!(f, "macie2:CreateCustomDataIdentifier")
            }
            Macie2Actions::CreateFindingsFilter => write!(f, "macie2:CreateFindingsFilter"),
            Macie2Actions::CreateInvitations => write!(f, "macie2:CreateInvitations"),
            Macie2Actions::CreateMember => write!(f, "macie2:CreateMember"),
            Macie2Actions::CreateSampleFindings => write!(f, "macie2:CreateSampleFindings"),
            Macie2Actions::DeclineInvitations => write!(f, "macie2:DeclineInvitations"),
            Macie2Actions::DeleteAllowList => write!(f, "macie2:DeleteAllowList"),
            Macie2Actions::DeleteCustomDataIdentifier => {
                write!(f, "macie2:DeleteCustomDataIdentifier")
            }
            Macie2Actions::DeleteFindingsFilter => write!(f, "macie2:DeleteFindingsFilter"),
            Macie2Actions::DeleteInvitations => write!(f, "macie2:DeleteInvitations"),
            Macie2Actions::DeleteMember => write!(f, "macie2:DeleteMember"),
            Macie2Actions::DescribeBuckets => write!(f, "macie2:DescribeBuckets"),
            Macie2Actions::DescribeClassificationJob => {
                write!(f, "macie2:DescribeClassificationJob")
            }
            Macie2Actions::DescribeOrganizationConfiguration => {
                write!(f, "macie2:DescribeOrganizationConfiguration")
            }
            Macie2Actions::DisableMacie => write!(f, "macie2:DisableMacie"),
            Macie2Actions::DisableOrganizationAdminAccount => {
                write!(f, "macie2:DisableOrganizationAdminAccount")
            }
            Macie2Actions::DisassociateFromAdministratorAccount => {
                write!(f, "macie2:DisassociateFromAdministratorAccount")
            }
            Macie2Actions::DisassociateFromMasterAccount => {
                write!(f, "macie2:DisassociateFromMasterAccount")
            }
            Macie2Actions::DisassociateMember => write!(f, "macie2:DisassociateMember"),
            Macie2Actions::EnableMacie => write!(f, "macie2:EnableMacie"),
            Macie2Actions::EnableOrganizationAdminAccount => {
                write!(f, "macie2:EnableOrganizationAdminAccount")
            }
            Macie2Actions::GetAdministratorAccount => write!(f, "macie2:GetAdministratorAccount"),
            Macie2Actions::GetAllowList => write!(f, "macie2:GetAllowList"),
            Macie2Actions::GetAutomatedDiscoveryConfiguration => {
                write!(f, "macie2:GetAutomatedDiscoveryConfiguration")
            }
            Macie2Actions::GetBucketStatistics => write!(f, "macie2:GetBucketStatistics"),
            Macie2Actions::GetClassificationExportConfiguration => {
                write!(f, "macie2:GetClassificationExportConfiguration")
            }
            Macie2Actions::GetClassificationScope => write!(f, "macie2:GetClassificationScope"),
            Macie2Actions::GetCustomDataIdentifier => write!(f, "macie2:GetCustomDataIdentifier"),
            Macie2Actions::GetFindingStatistics => write!(f, "macie2:GetFindingStatistics"),
            Macie2Actions::GetFindings => write!(f, "macie2:GetFindings"),
            Macie2Actions::GetFindingsFilter => write!(f, "macie2:GetFindingsFilter"),
            Macie2Actions::GetFindingsPublicationConfiguration => {
                write!(f, "macie2:GetFindingsPublicationConfiguration")
            }
            Macie2Actions::GetInvitationsCount => write!(f, "macie2:GetInvitationsCount"),
            Macie2Actions::GetMacieSession => write!(f, "macie2:GetMacieSession"),
            Macie2Actions::GetMasterAccount => write!(f, "macie2:GetMasterAccount"),
            Macie2Actions::GetMember => write!(f, "macie2:GetMember"),
            Macie2Actions::GetResourceProfile => write!(f, "macie2:GetResourceProfile"),
            Macie2Actions::GetRevealConfiguration => write!(f, "macie2:GetRevealConfiguration"),
            Macie2Actions::GetSensitiveDataOccurrences => {
                write!(f, "macie2:GetSensitiveDataOccurrences")
            }
            Macie2Actions::GetSensitiveDataOccurrencesAvailability => {
                write!(f, "macie2:GetSensitiveDataOccurrencesAvailability")
            }
            Macie2Actions::GetSensitivityInspectionTemplate => {
                write!(f, "macie2:GetSensitivityInspectionTemplate")
            }
            Macie2Actions::GetUsageStatistics => write!(f, "macie2:GetUsageStatistics"),
            Macie2Actions::GetUsageTotals => write!(f, "macie2:GetUsageTotals"),
            Macie2Actions::ListAllowLists => write!(f, "macie2:ListAllowLists"),
            Macie2Actions::ListAutomatedDiscoveryAccounts => {
                write!(f, "macie2:ListAutomatedDiscoveryAccounts")
            }
            Macie2Actions::ListClassificationJobs => write!(f, "macie2:ListClassificationJobs"),
            Macie2Actions::ListClassificationScopes => write!(f, "macie2:ListClassificationScopes"),
            Macie2Actions::ListCustomDataIdentifiers => {
                write!(f, "macie2:ListCustomDataIdentifiers")
            }
            Macie2Actions::ListFindings => write!(f, "macie2:ListFindings"),
            Macie2Actions::ListFindingsFilters => write!(f, "macie2:ListFindingsFilters"),
            Macie2Actions::ListInvitations => write!(f, "macie2:ListInvitations"),
            Macie2Actions::ListManagedDataIdentifiers => {
                write!(f, "macie2:ListManagedDataIdentifiers")
            }
            Macie2Actions::ListMembers => write!(f, "macie2:ListMembers"),
            Macie2Actions::ListOrganizationAdminAccounts => {
                write!(f, "macie2:ListOrganizationAdminAccounts")
            }
            Macie2Actions::ListResourceProfileArtifacts => {
                write!(f, "macie2:ListResourceProfileArtifacts")
            }
            Macie2Actions::ListResourceProfileDetections => {
                write!(f, "macie2:ListResourceProfileDetections")
            }
            Macie2Actions::ListSensitivityInspectionTemplates => {
                write!(f, "macie2:ListSensitivityInspectionTemplates")
            }
            Macie2Actions::ListTagsForResource => write!(f, "macie2:ListTagsForResource"),
            Macie2Actions::PutClassificationExportConfiguration => {
                write!(f, "macie2:PutClassificationExportConfiguration")
            }
            Macie2Actions::PutFindingsPublicationConfiguration => {
                write!(f, "macie2:PutFindingsPublicationConfiguration")
            }
            Macie2Actions::SearchResources => write!(f, "macie2:SearchResources"),
            Macie2Actions::TagResource => write!(f, "macie2:TagResource"),
            Macie2Actions::TestCustomDataIdentifier => write!(f, "macie2:TestCustomDataIdentifier"),
            Macie2Actions::UntagResource => write!(f, "macie2:UntagResource"),
            Macie2Actions::UpdateAllowList => write!(f, "macie2:UpdateAllowList"),
            Macie2Actions::UpdateAutomatedDiscoveryConfiguration => {
                write!(f, "macie2:UpdateAutomatedDiscoveryConfiguration")
            }
            Macie2Actions::UpdateClassificationJob => write!(f, "macie2:UpdateClassificationJob"),
            Macie2Actions::UpdateClassificationScope => {
                write!(f, "macie2:UpdateClassificationScope")
            }
            Macie2Actions::UpdateFindingsFilter => write!(f, "macie2:UpdateFindingsFilter"),
            Macie2Actions::UpdateMacieSession => write!(f, "macie2:UpdateMacieSession"),
            Macie2Actions::UpdateMemberSession => write!(f, "macie2:UpdateMemberSession"),
            Macie2Actions::UpdateOrganizationConfiguration => {
                write!(f, "macie2:UpdateOrganizationConfiguration")
            }
            Macie2Actions::UpdateResourceProfile => write!(f, "macie2:UpdateResourceProfile"),
            Macie2Actions::UpdateResourceProfileDetections => {
                write!(f, "macie2:UpdateResourceProfileDetections")
            }
            Macie2Actions::UpdateRevealConfiguration => {
                write!(f, "macie2:UpdateRevealConfiguration")
            }
            Macie2Actions::UpdateSensitivityInspectionTemplate => {
                write!(f, "macie2:UpdateSensitivityInspectionTemplate")
            }
        }
    }
}
