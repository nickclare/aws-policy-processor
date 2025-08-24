// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum PaymentCryptographyActions {
    CreateAlias,
    CreateKey,
    DecryptData,
    DeleteAlias,
    DeleteKey,
    EncryptData,
    ExportKey,
    GenerateCardValidationData,
    GenerateMac,
    GenerateMacEmvPinChange,
    GeneratePinData,
    GetAlias,
    GetKey,
    GetParametersForExport,
    GetParametersForImport,
    GetPublicKeyCertificate,
    ImportKey,
    ListAliases,
    ListKeys,
    ListTagsForResource,
    ReEncryptData,
    RestoreKey,
    StartKeyUsage,
    StopKeyUsage,
    TagResource,
    TranslatePinData,
    UntagResource,
    UpdateAlias,
    VerifyAuthRequestCryptogram,
    VerifyCardValidationData,
    VerifyMac,
    VerifyPinData,
}
impl std::fmt::Display for PaymentCryptographyActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PaymentCryptographyActions::CreateAlias => {
                write!(f, "payment-cryptography:CreateAlias")
            }
            PaymentCryptographyActions::CreateKey => write!(f, "payment-cryptography:CreateKey"),
            PaymentCryptographyActions::DecryptData => {
                write!(f, "payment-cryptography:DecryptData")
            }
            PaymentCryptographyActions::DeleteAlias => {
                write!(f, "payment-cryptography:DeleteAlias")
            }
            PaymentCryptographyActions::DeleteKey => write!(f, "payment-cryptography:DeleteKey"),
            PaymentCryptographyActions::EncryptData => {
                write!(f, "payment-cryptography:EncryptData")
            }
            PaymentCryptographyActions::ExportKey => write!(f, "payment-cryptography:ExportKey"),
            PaymentCryptographyActions::GenerateCardValidationData => {
                write!(f, "payment-cryptography:GenerateCardValidationData")
            }
            PaymentCryptographyActions::GenerateMac => {
                write!(f, "payment-cryptography:GenerateMac")
            }
            PaymentCryptographyActions::GenerateMacEmvPinChange => {
                write!(f, "payment-cryptography:GenerateMacEmvPinChange")
            }
            PaymentCryptographyActions::GeneratePinData => {
                write!(f, "payment-cryptography:GeneratePinData")
            }
            PaymentCryptographyActions::GetAlias => write!(f, "payment-cryptography:GetAlias"),
            PaymentCryptographyActions::GetKey => write!(f, "payment-cryptography:GetKey"),
            PaymentCryptographyActions::GetParametersForExport => {
                write!(f, "payment-cryptography:GetParametersForExport")
            }
            PaymentCryptographyActions::GetParametersForImport => {
                write!(f, "payment-cryptography:GetParametersForImport")
            }
            PaymentCryptographyActions::GetPublicKeyCertificate => {
                write!(f, "payment-cryptography:GetPublicKeyCertificate")
            }
            PaymentCryptographyActions::ImportKey => write!(f, "payment-cryptography:ImportKey"),
            PaymentCryptographyActions::ListAliases => {
                write!(f, "payment-cryptography:ListAliases")
            }
            PaymentCryptographyActions::ListKeys => write!(f, "payment-cryptography:ListKeys"),
            PaymentCryptographyActions::ListTagsForResource => {
                write!(f, "payment-cryptography:ListTagsForResource")
            }
            PaymentCryptographyActions::ReEncryptData => {
                write!(f, "payment-cryptography:ReEncryptData")
            }
            PaymentCryptographyActions::RestoreKey => write!(f, "payment-cryptography:RestoreKey"),
            PaymentCryptographyActions::StartKeyUsage => {
                write!(f, "payment-cryptography:StartKeyUsage")
            }
            PaymentCryptographyActions::StopKeyUsage => {
                write!(f, "payment-cryptography:StopKeyUsage")
            }
            PaymentCryptographyActions::TagResource => {
                write!(f, "payment-cryptography:TagResource")
            }
            PaymentCryptographyActions::TranslatePinData => {
                write!(f, "payment-cryptography:TranslatePinData")
            }
            PaymentCryptographyActions::UntagResource => {
                write!(f, "payment-cryptography:UntagResource")
            }
            PaymentCryptographyActions::UpdateAlias => {
                write!(f, "payment-cryptography:UpdateAlias")
            }
            PaymentCryptographyActions::VerifyAuthRequestCryptogram => {
                write!(f, "payment-cryptography:VerifyAuthRequestCryptogram")
            }
            PaymentCryptographyActions::VerifyCardValidationData => {
                write!(f, "payment-cryptography:VerifyCardValidationData")
            }
            PaymentCryptographyActions::VerifyMac => write!(f, "payment-cryptography:VerifyMac"),
            PaymentCryptographyActions::VerifyPinData => {
                write!(f, "payment-cryptography:VerifyPinData")
            }
        }
    }
}
