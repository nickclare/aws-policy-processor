// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum TaxActions {
    BatchDeleteTaxRegistration,
    BatchPutTaxRegistration,
    DeleteSupplementalTaxRegistration,
    DeleteTaxRegistration,
    GetExemptions,
    GetTaxInfoReportingDocument,
    GetTaxInheritance,
    GetTaxInterview,
    GetTaxRegistration,
    GetTaxRegistrationDocument,
    ListSupplementalTaxRegistrations,
    ListTaxRegistrations,
    PutSupplementalTaxRegistration,
    PutTaxInheritance,
    PutTaxInterview,
    PutTaxRegistration,
    UpdateExemptions,
}
impl std::fmt::Display for TaxActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaxActions::BatchDeleteTaxRegistration => write!(f, "tax:BatchDeleteTaxRegistration"),
            TaxActions::BatchPutTaxRegistration => write!(f, "tax:BatchPutTaxRegistration"),
            TaxActions::DeleteSupplementalTaxRegistration => {
                write!(f, "tax:DeleteSupplementalTaxRegistration")
            }
            TaxActions::DeleteTaxRegistration => write!(f, "tax:DeleteTaxRegistration"),
            TaxActions::GetExemptions => write!(f, "tax:GetExemptions"),
            TaxActions::GetTaxInfoReportingDocument => write!(f, "tax:GetTaxInfoReportingDocument"),
            TaxActions::GetTaxInheritance => write!(f, "tax:GetTaxInheritance"),
            TaxActions::GetTaxInterview => write!(f, "tax:GetTaxInterview"),
            TaxActions::GetTaxRegistration => write!(f, "tax:GetTaxRegistration"),
            TaxActions::GetTaxRegistrationDocument => write!(f, "tax:GetTaxRegistrationDocument"),
            TaxActions::ListSupplementalTaxRegistrations => {
                write!(f, "tax:ListSupplementalTaxRegistrations")
            }
            TaxActions::ListTaxRegistrations => write!(f, "tax:ListTaxRegistrations"),
            TaxActions::PutSupplementalTaxRegistration => {
                write!(f, "tax:PutSupplementalTaxRegistration")
            }
            TaxActions::PutTaxInheritance => write!(f, "tax:PutTaxInheritance"),
            TaxActions::PutTaxInterview => write!(f, "tax:PutTaxInterview"),
            TaxActions::PutTaxRegistration => write!(f, "tax:PutTaxRegistration"),
            TaxActions::UpdateExemptions => write!(f, "tax:UpdateExemptions"),
        }
    }
}
