// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum AcmPcaActions {
    CreateCertificateAuthority,
    CreateCertificateAuthorityAuditReport,
    CreatePermission,
    DeleteCertificateAuthority,
    DeletePermission,
    DeletePolicy,
    DescribeCertificateAuthority,
    DescribeCertificateAuthorityAuditReport,
    GetCertificate,
    GetCertificateAuthorityCertificate,
    GetCertificateAuthorityCsr,
    GetPolicy,
    ImportCertificateAuthorityCertificate,
    IssueCertificate,
    ListCertificateAuthorities,
    ListPermissions,
    ListTags,
    PutPolicy,
    RestoreCertificateAuthority,
    RevokeCertificate,
    TagCertificateAuthority,
    UntagCertificateAuthority,
    UpdateCertificateAuthority,
}
impl std::fmt::Display for AcmPcaActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AcmPcaActions::CreateCertificateAuthority => {
                write!(f, "acm-pca:CreateCertificateAuthority")
            }
            AcmPcaActions::CreateCertificateAuthorityAuditReport => {
                write!(f, "acm-pca:CreateCertificateAuthorityAuditReport")
            }
            AcmPcaActions::CreatePermission => write!(f, "acm-pca:CreatePermission"),
            AcmPcaActions::DeleteCertificateAuthority => {
                write!(f, "acm-pca:DeleteCertificateAuthority")
            }
            AcmPcaActions::DeletePermission => write!(f, "acm-pca:DeletePermission"),
            AcmPcaActions::DeletePolicy => write!(f, "acm-pca:DeletePolicy"),
            AcmPcaActions::DescribeCertificateAuthority => {
                write!(f, "acm-pca:DescribeCertificateAuthority")
            }
            AcmPcaActions::DescribeCertificateAuthorityAuditReport => {
                write!(f, "acm-pca:DescribeCertificateAuthorityAuditReport")
            }
            AcmPcaActions::GetCertificate => write!(f, "acm-pca:GetCertificate"),
            AcmPcaActions::GetCertificateAuthorityCertificate => {
                write!(f, "acm-pca:GetCertificateAuthorityCertificate")
            }
            AcmPcaActions::GetCertificateAuthorityCsr => {
                write!(f, "acm-pca:GetCertificateAuthorityCsr")
            }
            AcmPcaActions::GetPolicy => write!(f, "acm-pca:GetPolicy"),
            AcmPcaActions::ImportCertificateAuthorityCertificate => {
                write!(f, "acm-pca:ImportCertificateAuthorityCertificate")
            }
            AcmPcaActions::IssueCertificate => write!(f, "acm-pca:IssueCertificate"),
            AcmPcaActions::ListCertificateAuthorities => {
                write!(f, "acm-pca:ListCertificateAuthorities")
            }
            AcmPcaActions::ListPermissions => write!(f, "acm-pca:ListPermissions"),
            AcmPcaActions::ListTags => write!(f, "acm-pca:ListTags"),
            AcmPcaActions::PutPolicy => write!(f, "acm-pca:PutPolicy"),
            AcmPcaActions::RestoreCertificateAuthority => {
                write!(f, "acm-pca:RestoreCertificateAuthority")
            }
            AcmPcaActions::RevokeCertificate => write!(f, "acm-pca:RevokeCertificate"),
            AcmPcaActions::TagCertificateAuthority => write!(f, "acm-pca:TagCertificateAuthority"),
            AcmPcaActions::UntagCertificateAuthority => {
                write!(f, "acm-pca:UntagCertificateAuthority")
            }
            AcmPcaActions::UpdateCertificateAuthority => {
                write!(f, "acm-pca:UpdateCertificateAuthority")
            }
        }
    }
}
