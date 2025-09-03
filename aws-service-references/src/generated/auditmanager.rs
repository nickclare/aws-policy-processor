// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum AuditmanagerActions {
    AssociateAssessmentReportEvidenceFolder,
    BatchAssociateAssessmentReportEvidence,
    BatchCreateDelegationByAssessment,
    BatchDeleteDelegationByAssessment,
    BatchDisassociateAssessmentReportEvidence,
    BatchImportEvidenceToAssessmentControl,
    CreateAssessment,
    CreateAssessmentFramework,
    CreateAssessmentReport,
    CreateControl,
    DeleteAssessment,
    DeleteAssessmentFramework,
    DeleteAssessmentFrameworkShare,
    DeleteAssessmentReport,
    DeleteControl,
    DeregisterAccount,
    DeregisterOrganizationAdminAccount,
    DisassociateAssessmentReportEvidenceFolder,
    GetAccountStatus,
    GetAssessment,
    GetAssessmentFramework,
    GetAssessmentReportUrl,
    GetChangeLogs,
    GetControl,
    GetDelegations,
    GetEvidence,
    GetEvidenceByEvidenceFolder,
    GetEvidenceFileUploadUrl,
    GetEvidenceFolder,
    GetEvidenceFoldersByAssessment,
    GetEvidenceFoldersByAssessmentControl,
    GetInsights,
    GetInsightsByAssessment,
    GetOrganizationAdminAccount,
    GetServicesInScope,
    GetSettings,
    ListAssessmentControlInsightsByControlDomain,
    ListAssessmentFrameworkShareRequests,
    ListAssessmentFrameworks,
    ListAssessmentReports,
    ListAssessments,
    ListControlDomainInsights,
    ListControlDomainInsightsByAssessment,
    ListControlInsightsByControlDomain,
    ListControls,
    ListKeywordsForDataSource,
    ListNotifications,
    ListTagsForResource,
    RegisterAccount,
    RegisterOrganizationAdminAccount,
    StartAssessmentFrameworkShare,
    TagResource,
    UntagResource,
    UpdateAssessment,
    UpdateAssessmentControl,
    UpdateAssessmentControlSetStatus,
    UpdateAssessmentFramework,
    UpdateAssessmentFrameworkShare,
    UpdateAssessmentStatus,
    UpdateControl,
    UpdateSettings,
    ValidateAssessmentReportIntegrity,
}
impl std::fmt::Display for AuditmanagerActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuditmanagerActions::AssociateAssessmentReportEvidenceFolder => {
                write!(f, "auditmanager:AssociateAssessmentReportEvidenceFolder")
            }
            AuditmanagerActions::BatchAssociateAssessmentReportEvidence => {
                write!(f, "auditmanager:BatchAssociateAssessmentReportEvidence")
            }
            AuditmanagerActions::BatchCreateDelegationByAssessment => {
                write!(f, "auditmanager:BatchCreateDelegationByAssessment")
            }
            AuditmanagerActions::BatchDeleteDelegationByAssessment => {
                write!(f, "auditmanager:BatchDeleteDelegationByAssessment")
            }
            AuditmanagerActions::BatchDisassociateAssessmentReportEvidence => {
                write!(f, "auditmanager:BatchDisassociateAssessmentReportEvidence")
            }
            AuditmanagerActions::BatchImportEvidenceToAssessmentControl => {
                write!(f, "auditmanager:BatchImportEvidenceToAssessmentControl")
            }
            AuditmanagerActions::CreateAssessment => write!(f, "auditmanager:CreateAssessment"),
            AuditmanagerActions::CreateAssessmentFramework => {
                write!(f, "auditmanager:CreateAssessmentFramework")
            }
            AuditmanagerActions::CreateAssessmentReport => {
                write!(f, "auditmanager:CreateAssessmentReport")
            }
            AuditmanagerActions::CreateControl => write!(f, "auditmanager:CreateControl"),
            AuditmanagerActions::DeleteAssessment => write!(f, "auditmanager:DeleteAssessment"),
            AuditmanagerActions::DeleteAssessmentFramework => {
                write!(f, "auditmanager:DeleteAssessmentFramework")
            }
            AuditmanagerActions::DeleteAssessmentFrameworkShare => {
                write!(f, "auditmanager:DeleteAssessmentFrameworkShare")
            }
            AuditmanagerActions::DeleteAssessmentReport => {
                write!(f, "auditmanager:DeleteAssessmentReport")
            }
            AuditmanagerActions::DeleteControl => write!(f, "auditmanager:DeleteControl"),
            AuditmanagerActions::DeregisterAccount => write!(f, "auditmanager:DeregisterAccount"),
            AuditmanagerActions::DeregisterOrganizationAdminAccount => {
                write!(f, "auditmanager:DeregisterOrganizationAdminAccount")
            }
            AuditmanagerActions::DisassociateAssessmentReportEvidenceFolder => {
                write!(f, "auditmanager:DisassociateAssessmentReportEvidenceFolder")
            }
            AuditmanagerActions::GetAccountStatus => write!(f, "auditmanager:GetAccountStatus"),
            AuditmanagerActions::GetAssessment => write!(f, "auditmanager:GetAssessment"),
            AuditmanagerActions::GetAssessmentFramework => {
                write!(f, "auditmanager:GetAssessmentFramework")
            }
            AuditmanagerActions::GetAssessmentReportUrl => {
                write!(f, "auditmanager:GetAssessmentReportUrl")
            }
            AuditmanagerActions::GetChangeLogs => write!(f, "auditmanager:GetChangeLogs"),
            AuditmanagerActions::GetControl => write!(f, "auditmanager:GetControl"),
            AuditmanagerActions::GetDelegations => write!(f, "auditmanager:GetDelegations"),
            AuditmanagerActions::GetEvidence => write!(f, "auditmanager:GetEvidence"),
            AuditmanagerActions::GetEvidenceByEvidenceFolder => {
                write!(f, "auditmanager:GetEvidenceByEvidenceFolder")
            }
            AuditmanagerActions::GetEvidenceFileUploadUrl => {
                write!(f, "auditmanager:GetEvidenceFileUploadUrl")
            }
            AuditmanagerActions::GetEvidenceFolder => write!(f, "auditmanager:GetEvidenceFolder"),
            AuditmanagerActions::GetEvidenceFoldersByAssessment => {
                write!(f, "auditmanager:GetEvidenceFoldersByAssessment")
            }
            AuditmanagerActions::GetEvidenceFoldersByAssessmentControl => {
                write!(f, "auditmanager:GetEvidenceFoldersByAssessmentControl")
            }
            AuditmanagerActions::GetInsights => write!(f, "auditmanager:GetInsights"),
            AuditmanagerActions::GetInsightsByAssessment => {
                write!(f, "auditmanager:GetInsightsByAssessment")
            }
            AuditmanagerActions::GetOrganizationAdminAccount => {
                write!(f, "auditmanager:GetOrganizationAdminAccount")
            }
            AuditmanagerActions::GetServicesInScope => write!(f, "auditmanager:GetServicesInScope"),
            AuditmanagerActions::GetSettings => write!(f, "auditmanager:GetSettings"),
            AuditmanagerActions::ListAssessmentControlInsightsByControlDomain => write!(
                f,
                "auditmanager:ListAssessmentControlInsightsByControlDomain"
            ),
            AuditmanagerActions::ListAssessmentFrameworkShareRequests => {
                write!(f, "auditmanager:ListAssessmentFrameworkShareRequests")
            }
            AuditmanagerActions::ListAssessmentFrameworks => {
                write!(f, "auditmanager:ListAssessmentFrameworks")
            }
            AuditmanagerActions::ListAssessmentReports => {
                write!(f, "auditmanager:ListAssessmentReports")
            }
            AuditmanagerActions::ListAssessments => write!(f, "auditmanager:ListAssessments"),
            AuditmanagerActions::ListControlDomainInsights => {
                write!(f, "auditmanager:ListControlDomainInsights")
            }
            AuditmanagerActions::ListControlDomainInsightsByAssessment => {
                write!(f, "auditmanager:ListControlDomainInsightsByAssessment")
            }
            AuditmanagerActions::ListControlInsightsByControlDomain => {
                write!(f, "auditmanager:ListControlInsightsByControlDomain")
            }
            AuditmanagerActions::ListControls => write!(f, "auditmanager:ListControls"),
            AuditmanagerActions::ListKeywordsForDataSource => {
                write!(f, "auditmanager:ListKeywordsForDataSource")
            }
            AuditmanagerActions::ListNotifications => write!(f, "auditmanager:ListNotifications"),
            AuditmanagerActions::ListTagsForResource => {
                write!(f, "auditmanager:ListTagsForResource")
            }
            AuditmanagerActions::RegisterAccount => write!(f, "auditmanager:RegisterAccount"),
            AuditmanagerActions::RegisterOrganizationAdminAccount => {
                write!(f, "auditmanager:RegisterOrganizationAdminAccount")
            }
            AuditmanagerActions::StartAssessmentFrameworkShare => {
                write!(f, "auditmanager:StartAssessmentFrameworkShare")
            }
            AuditmanagerActions::TagResource => write!(f, "auditmanager:TagResource"),
            AuditmanagerActions::UntagResource => write!(f, "auditmanager:UntagResource"),
            AuditmanagerActions::UpdateAssessment => write!(f, "auditmanager:UpdateAssessment"),
            AuditmanagerActions::UpdateAssessmentControl => {
                write!(f, "auditmanager:UpdateAssessmentControl")
            }
            AuditmanagerActions::UpdateAssessmentControlSetStatus => {
                write!(f, "auditmanager:UpdateAssessmentControlSetStatus")
            }
            AuditmanagerActions::UpdateAssessmentFramework => {
                write!(f, "auditmanager:UpdateAssessmentFramework")
            }
            AuditmanagerActions::UpdateAssessmentFrameworkShare => {
                write!(f, "auditmanager:UpdateAssessmentFrameworkShare")
            }
            AuditmanagerActions::UpdateAssessmentStatus => {
                write!(f, "auditmanager:UpdateAssessmentStatus")
            }
            AuditmanagerActions::UpdateControl => write!(f, "auditmanager:UpdateControl"),
            AuditmanagerActions::UpdateSettings => write!(f, "auditmanager:UpdateSettings"),
            AuditmanagerActions::ValidateAssessmentReportIntegrity => {
                write!(f, "auditmanager:ValidateAssessmentReportIntegrity")
            }
        }
    }
}
