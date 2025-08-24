// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum InvoicingActions {
    BatchGetInvoiceProfile,
    CreateInvoiceUnit,
    DeleteInvoiceUnit,
    GetInvoiceEmailDeliveryPreferences,
    GetInvoicePdf,
    GetInvoiceUnit,
    ListInvoiceSummaries,
    ListInvoiceUnits,
    ListTagsForResource,
    PutInvoiceEmailDeliveryPreferences,
    TagResource,
    UntagResource,
    UpdateInvoiceUnit,
}
impl std::fmt::Display for InvoicingActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InvoicingActions::BatchGetInvoiceProfile => {
                write!(f, "invoicing:BatchGetInvoiceProfile")
            }
            InvoicingActions::CreateInvoiceUnit => write!(f, "invoicing:CreateInvoiceUnit"),
            InvoicingActions::DeleteInvoiceUnit => write!(f, "invoicing:DeleteInvoiceUnit"),
            InvoicingActions::GetInvoiceEmailDeliveryPreferences => {
                write!(f, "invoicing:GetInvoiceEmailDeliveryPreferences")
            }
            InvoicingActions::GetInvoicePdf => write!(f, "invoicing:GetInvoicePDF"),
            InvoicingActions::GetInvoiceUnit => write!(f, "invoicing:GetInvoiceUnit"),
            InvoicingActions::ListInvoiceSummaries => write!(f, "invoicing:ListInvoiceSummaries"),
            InvoicingActions::ListInvoiceUnits => write!(f, "invoicing:ListInvoiceUnits"),
            InvoicingActions::ListTagsForResource => write!(f, "invoicing:ListTagsForResource"),
            InvoicingActions::PutInvoiceEmailDeliveryPreferences => {
                write!(f, "invoicing:PutInvoiceEmailDeliveryPreferences")
            }
            InvoicingActions::TagResource => write!(f, "invoicing:TagResource"),
            InvoicingActions::UntagResource => write!(f, "invoicing:UntagResource"),
            InvoicingActions::UpdateInvoiceUnit => write!(f, "invoicing:UpdateInvoiceUnit"),
        }
    }
}
