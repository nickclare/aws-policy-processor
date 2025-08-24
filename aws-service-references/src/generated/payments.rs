// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum PaymentsActions {
    AcceptFinancingApplicationTerms,
    CreateFinancingApplication,
    CreatePaymentInstrument,
    DeletePaymentInstrument,
    GetFinancingApplication,
    GetFinancingLine,
    GetFinancingLineWithdrawal,
    GetFinancingOption,
    GetPaymentInstrument,
    GetPaymentStatus,
    ListFinancingApplications,
    ListFinancingLineWithdrawals,
    ListFinancingLines,
    ListPaymentInstruments,
    ListPaymentPreferences,
    ListPaymentProgramOptions,
    ListPaymentProgramStatus,
    ListTagsForResource,
    MakePayment,
    TagResource,
    UntagResource,
    UpdateFinancingApplication,
    UpdatePaymentInstrument,
    UpdatePaymentPreferences,
}
impl std::fmt::Display for PaymentsActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PaymentsActions::AcceptFinancingApplicationTerms => {
                write!(f, "payments:AcceptFinancingApplicationTerms")
            }
            PaymentsActions::CreateFinancingApplication => {
                write!(f, "payments:CreateFinancingApplication")
            }
            PaymentsActions::CreatePaymentInstrument => {
                write!(f, "payments:CreatePaymentInstrument")
            }
            PaymentsActions::DeletePaymentInstrument => {
                write!(f, "payments:DeletePaymentInstrument")
            }
            PaymentsActions::GetFinancingApplication => {
                write!(f, "payments:GetFinancingApplication")
            }
            PaymentsActions::GetFinancingLine => write!(f, "payments:GetFinancingLine"),
            PaymentsActions::GetFinancingLineWithdrawal => {
                write!(f, "payments:GetFinancingLineWithdrawal")
            }
            PaymentsActions::GetFinancingOption => write!(f, "payments:GetFinancingOption"),
            PaymentsActions::GetPaymentInstrument => write!(f, "payments:GetPaymentInstrument"),
            PaymentsActions::GetPaymentStatus => write!(f, "payments:GetPaymentStatus"),
            PaymentsActions::ListFinancingApplications => {
                write!(f, "payments:ListFinancingApplications")
            }
            PaymentsActions::ListFinancingLineWithdrawals => {
                write!(f, "payments:ListFinancingLineWithdrawals")
            }
            PaymentsActions::ListFinancingLines => write!(f, "payments:ListFinancingLines"),
            PaymentsActions::ListPaymentInstruments => write!(f, "payments:ListPaymentInstruments"),
            PaymentsActions::ListPaymentPreferences => write!(f, "payments:ListPaymentPreferences"),
            PaymentsActions::ListPaymentProgramOptions => {
                write!(f, "payments:ListPaymentProgramOptions")
            }
            PaymentsActions::ListPaymentProgramStatus => {
                write!(f, "payments:ListPaymentProgramStatus")
            }
            PaymentsActions::ListTagsForResource => write!(f, "payments:ListTagsForResource"),
            PaymentsActions::MakePayment => write!(f, "payments:MakePayment"),
            PaymentsActions::TagResource => write!(f, "payments:TagResource"),
            PaymentsActions::UntagResource => write!(f, "payments:UntagResource"),
            PaymentsActions::UpdateFinancingApplication => {
                write!(f, "payments:UpdateFinancingApplication")
            }
            PaymentsActions::UpdatePaymentInstrument => {
                write!(f, "payments:UpdatePaymentInstrument")
            }
            PaymentsActions::UpdatePaymentPreferences => {
                write!(f, "payments:UpdatePaymentPreferences")
            }
        }
    }
}
