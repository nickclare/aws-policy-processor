// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum Inspector2Actions {
    AssociateMember,
    BatchAssociateCodeSecurityScanConfiguration,
    BatchDisassociateCodeSecurityScanConfiguration,
    BatchGetAccountStatus,
    BatchGetCodeSnippet,
    BatchGetFindingDetails,
    BatchGetFreeTrialInfo,
    BatchGetMemberEc2DeepInspectionStatus,
    BatchUpdateMemberEc2DeepInspectionStatus,
    CancelFindingsReport,
    CancelSbomExport,
    CreateCisScanConfiguration,
    CreateCodeSecurityIntegration,
    CreateCodeSecurityScanConfiguration,
    CreateFilter,
    CreateFindingsReport,
    CreateSbomExport,
    DeleteCisScanConfiguration,
    DeleteCodeSecurityIntegration,
    DeleteCodeSecurityScanConfiguration,
    DeleteFilter,
    DescribeOrganizationConfiguration,
    Disable,
    DisableDelegatedAdminAccount,
    DisassociateMember,
    Enable,
    EnableDelegatedAdminAccount,
    GetCisScanReport,
    GetCisScanResultDetails,
    GetClustersForImage,
    GetCodeSecurityIntegration,
    GetCodeSecurityScan,
    GetCodeSecurityScanConfiguration,
    GetConfiguration,
    GetDelegatedAdminAccount,
    GetEc2DeepInspectionConfiguration,
    GetEncryptionKey,
    GetFindingsReportStatus,
    GetMember,
    GetSbomExport,
    ListAccountPermissions,
    ListCisScanConfigurations,
    ListCisScanResultsAggregatedByChecks,
    ListCisScanResultsAggregatedByTargetResource,
    ListCisScans,
    ListCodeSecurityIntegrations,
    ListCodeSecurityScanConfigurationAssociations,
    ListCodeSecurityScanConfigurations,
    ListCoverage,
    ListCoverageStatistics,
    ListDelegatedAdminAccounts,
    ListFilters,
    ListFindingAggregations,
    ListFindings,
    ListMembers,
    ListTagsForResource,
    ListUsageTotals,
    ResetEncryptionKey,
    SearchVulnerabilities,
    SendCisSessionHealth,
    SendCisSessionTelemetry,
    StartCisSession,
    StartCodeSecurityScan,
    StopCisSession,
    TagResource,
    UntagResource,
    UpdateCisScanConfiguration,
    UpdateCodeSecurityIntegration,
    UpdateCodeSecurityScanConfiguration,
    UpdateConfiguration,
    UpdateEc2DeepInspectionConfiguration,
    UpdateEncryptionKey,
    UpdateFilter,
    UpdateOrgEc2DeepInspectionConfiguration,
    UpdateOrganizationConfiguration,
}
impl std::fmt::Display for Inspector2Actions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Inspector2Actions::AssociateMember => write!(f, "inspector2:AssociateMember"),
            Inspector2Actions::BatchAssociateCodeSecurityScanConfiguration => {
                write!(f, "inspector2:BatchAssociateCodeSecurityScanConfiguration")
            }
            Inspector2Actions::BatchDisassociateCodeSecurityScanConfiguration => write!(
                f,
                "inspector2:BatchDisassociateCodeSecurityScanConfiguration"
            ),
            Inspector2Actions::BatchGetAccountStatus => {
                write!(f, "inspector2:BatchGetAccountStatus")
            }
            Inspector2Actions::BatchGetCodeSnippet => write!(f, "inspector2:BatchGetCodeSnippet"),
            Inspector2Actions::BatchGetFindingDetails => {
                write!(f, "inspector2:BatchGetFindingDetails")
            }
            Inspector2Actions::BatchGetFreeTrialInfo => {
                write!(f, "inspector2:BatchGetFreeTrialInfo")
            }
            Inspector2Actions::BatchGetMemberEc2DeepInspectionStatus => {
                write!(f, "inspector2:BatchGetMemberEc2DeepInspectionStatus")
            }
            Inspector2Actions::BatchUpdateMemberEc2DeepInspectionStatus => {
                write!(f, "inspector2:BatchUpdateMemberEc2DeepInspectionStatus")
            }
            Inspector2Actions::CancelFindingsReport => write!(f, "inspector2:CancelFindingsReport"),
            Inspector2Actions::CancelSbomExport => write!(f, "inspector2:CancelSbomExport"),
            Inspector2Actions::CreateCisScanConfiguration => {
                write!(f, "inspector2:CreateCisScanConfiguration")
            }
            Inspector2Actions::CreateCodeSecurityIntegration => {
                write!(f, "inspector2:CreateCodeSecurityIntegration")
            }
            Inspector2Actions::CreateCodeSecurityScanConfiguration => {
                write!(f, "inspector2:CreateCodeSecurityScanConfiguration")
            }
            Inspector2Actions::CreateFilter => write!(f, "inspector2:CreateFilter"),
            Inspector2Actions::CreateFindingsReport => write!(f, "inspector2:CreateFindingsReport"),
            Inspector2Actions::CreateSbomExport => write!(f, "inspector2:CreateSbomExport"),
            Inspector2Actions::DeleteCisScanConfiguration => {
                write!(f, "inspector2:DeleteCisScanConfiguration")
            }
            Inspector2Actions::DeleteCodeSecurityIntegration => {
                write!(f, "inspector2:DeleteCodeSecurityIntegration")
            }
            Inspector2Actions::DeleteCodeSecurityScanConfiguration => {
                write!(f, "inspector2:DeleteCodeSecurityScanConfiguration")
            }
            Inspector2Actions::DeleteFilter => write!(f, "inspector2:DeleteFilter"),
            Inspector2Actions::DescribeOrganizationConfiguration => {
                write!(f, "inspector2:DescribeOrganizationConfiguration")
            }
            Inspector2Actions::Disable => write!(f, "inspector2:Disable"),
            Inspector2Actions::DisableDelegatedAdminAccount => {
                write!(f, "inspector2:DisableDelegatedAdminAccount")
            }
            Inspector2Actions::DisassociateMember => write!(f, "inspector2:DisassociateMember"),
            Inspector2Actions::Enable => write!(f, "inspector2:Enable"),
            Inspector2Actions::EnableDelegatedAdminAccount => {
                write!(f, "inspector2:EnableDelegatedAdminAccount")
            }
            Inspector2Actions::GetCisScanReport => write!(f, "inspector2:GetCisScanReport"),
            Inspector2Actions::GetCisScanResultDetails => {
                write!(f, "inspector2:GetCisScanResultDetails")
            }
            Inspector2Actions::GetClustersForImage => write!(f, "inspector2:GetClustersForImage"),
            Inspector2Actions::GetCodeSecurityIntegration => {
                write!(f, "inspector2:GetCodeSecurityIntegration")
            }
            Inspector2Actions::GetCodeSecurityScan => write!(f, "inspector2:GetCodeSecurityScan"),
            Inspector2Actions::GetCodeSecurityScanConfiguration => {
                write!(f, "inspector2:GetCodeSecurityScanConfiguration")
            }
            Inspector2Actions::GetConfiguration => write!(f, "inspector2:GetConfiguration"),
            Inspector2Actions::GetDelegatedAdminAccount => {
                write!(f, "inspector2:GetDelegatedAdminAccount")
            }
            Inspector2Actions::GetEc2DeepInspectionConfiguration => {
                write!(f, "inspector2:GetEc2DeepInspectionConfiguration")
            }
            Inspector2Actions::GetEncryptionKey => write!(f, "inspector2:GetEncryptionKey"),
            Inspector2Actions::GetFindingsReportStatus => {
                write!(f, "inspector2:GetFindingsReportStatus")
            }
            Inspector2Actions::GetMember => write!(f, "inspector2:GetMember"),
            Inspector2Actions::GetSbomExport => write!(f, "inspector2:GetSbomExport"),
            Inspector2Actions::ListAccountPermissions => {
                write!(f, "inspector2:ListAccountPermissions")
            }
            Inspector2Actions::ListCisScanConfigurations => {
                write!(f, "inspector2:ListCisScanConfigurations")
            }
            Inspector2Actions::ListCisScanResultsAggregatedByChecks => {
                write!(f, "inspector2:ListCisScanResultsAggregatedByChecks")
            }
            Inspector2Actions::ListCisScanResultsAggregatedByTargetResource => {
                write!(f, "inspector2:ListCisScanResultsAggregatedByTargetResource")
            }
            Inspector2Actions::ListCisScans => write!(f, "inspector2:ListCisScans"),
            Inspector2Actions::ListCodeSecurityIntegrations => {
                write!(f, "inspector2:ListCodeSecurityIntegrations")
            }
            Inspector2Actions::ListCodeSecurityScanConfigurationAssociations => write!(
                f,
                "inspector2:ListCodeSecurityScanConfigurationAssociations"
            ),
            Inspector2Actions::ListCodeSecurityScanConfigurations => {
                write!(f, "inspector2:ListCodeSecurityScanConfigurations")
            }
            Inspector2Actions::ListCoverage => write!(f, "inspector2:ListCoverage"),
            Inspector2Actions::ListCoverageStatistics => {
                write!(f, "inspector2:ListCoverageStatistics")
            }
            Inspector2Actions::ListDelegatedAdminAccounts => {
                write!(f, "inspector2:ListDelegatedAdminAccounts")
            }
            Inspector2Actions::ListFilters => write!(f, "inspector2:ListFilters"),
            Inspector2Actions::ListFindingAggregations => {
                write!(f, "inspector2:ListFindingAggregations")
            }
            Inspector2Actions::ListFindings => write!(f, "inspector2:ListFindings"),
            Inspector2Actions::ListMembers => write!(f, "inspector2:ListMembers"),
            Inspector2Actions::ListTagsForResource => write!(f, "inspector2:ListTagsForResource"),
            Inspector2Actions::ListUsageTotals => write!(f, "inspector2:ListUsageTotals"),
            Inspector2Actions::ResetEncryptionKey => write!(f, "inspector2:ResetEncryptionKey"),
            Inspector2Actions::SearchVulnerabilities => {
                write!(f, "inspector2:SearchVulnerabilities")
            }
            Inspector2Actions::SendCisSessionHealth => write!(f, "inspector2:SendCisSessionHealth"),
            Inspector2Actions::SendCisSessionTelemetry => {
                write!(f, "inspector2:SendCisSessionTelemetry")
            }
            Inspector2Actions::StartCisSession => write!(f, "inspector2:StartCisSession"),
            Inspector2Actions::StartCodeSecurityScan => {
                write!(f, "inspector2:StartCodeSecurityScan")
            }
            Inspector2Actions::StopCisSession => write!(f, "inspector2:StopCisSession"),
            Inspector2Actions::TagResource => write!(f, "inspector2:TagResource"),
            Inspector2Actions::UntagResource => write!(f, "inspector2:UntagResource"),
            Inspector2Actions::UpdateCisScanConfiguration => {
                write!(f, "inspector2:UpdateCisScanConfiguration")
            }
            Inspector2Actions::UpdateCodeSecurityIntegration => {
                write!(f, "inspector2:UpdateCodeSecurityIntegration")
            }
            Inspector2Actions::UpdateCodeSecurityScanConfiguration => {
                write!(f, "inspector2:UpdateCodeSecurityScanConfiguration")
            }
            Inspector2Actions::UpdateConfiguration => write!(f, "inspector2:UpdateConfiguration"),
            Inspector2Actions::UpdateEc2DeepInspectionConfiguration => {
                write!(f, "inspector2:UpdateEc2DeepInspectionConfiguration")
            }
            Inspector2Actions::UpdateEncryptionKey => write!(f, "inspector2:UpdateEncryptionKey"),
            Inspector2Actions::UpdateFilter => write!(f, "inspector2:UpdateFilter"),
            Inspector2Actions::UpdateOrgEc2DeepInspectionConfiguration => {
                write!(f, "inspector2:UpdateOrgEc2DeepInspectionConfiguration")
            }
            Inspector2Actions::UpdateOrganizationConfiguration => {
                write!(f, "inspector2:UpdateOrganizationConfiguration")
            }
        }
    }
}
