// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum AcmActions {
    AddTagsToCertificate,
    DeleteCertificate,
    DescribeCertificate,
    ExportCertificate,
    GetAccountConfiguration,
    GetCertificate,
    ImportCertificate,
    ListCertificates,
    ListTagsForCertificate,
    PutAccountConfiguration,
    RemoveTagsFromCertificate,
    RenewCertificate,
    RequestCertificate,
    ResendValidationEmail,
    RevokeCertificate,
    UpdateCertificateOptions,
}
impl std::fmt::Display for AcmActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AcmActions::AddTagsToCertificate => write!(f, "acm:AddTagsToCertificate"),
            AcmActions::DeleteCertificate => write!(f, "acm:DeleteCertificate"),
            AcmActions::DescribeCertificate => write!(f, "acm:DescribeCertificate"),
            AcmActions::ExportCertificate => write!(f, "acm:ExportCertificate"),
            AcmActions::GetAccountConfiguration => write!(f, "acm:GetAccountConfiguration"),
            AcmActions::GetCertificate => write!(f, "acm:GetCertificate"),
            AcmActions::ImportCertificate => write!(f, "acm:ImportCertificate"),
            AcmActions::ListCertificates => write!(f, "acm:ListCertificates"),
            AcmActions::ListTagsForCertificate => write!(f, "acm:ListTagsForCertificate"),
            AcmActions::PutAccountConfiguration => write!(f, "acm:PutAccountConfiguration"),
            AcmActions::RemoveTagsFromCertificate => write!(f, "acm:RemoveTagsFromCertificate"),
            AcmActions::RenewCertificate => write!(f, "acm:RenewCertificate"),
            AcmActions::RequestCertificate => write!(f, "acm:RequestCertificate"),
            AcmActions::ResendValidationEmail => write!(f, "acm:ResendValidationEmail"),
            AcmActions::RevokeCertificate => write!(f, "acm:RevokeCertificate"),
            AcmActions::UpdateCertificateOptions => write!(f, "acm:UpdateCertificateOptions"),
        }
    }
}
