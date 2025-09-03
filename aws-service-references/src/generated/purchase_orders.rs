// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum PurchaseOrdersActions {
    AddPurchaseOrder,
    DeletePurchaseOrder,
    GetConsoleActionSetEnforced,
    GetPurchaseOrder,
    ListPurchaseOrderInvoices,
    ListPurchaseOrders,
    ListTagsForResource,
    ModifyPurchaseOrders,
    TagResource,
    UntagResource,
    UpdateConsoleActionSetEnforced,
    UpdatePurchaseOrder,
    UpdatePurchaseOrderStatus,
    ViewPurchaseOrders,
}
impl std::fmt::Display for PurchaseOrdersActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PurchaseOrdersActions::AddPurchaseOrder => {
                write!(f, "purchase-orders:AddPurchaseOrder")
            }
            PurchaseOrdersActions::DeletePurchaseOrder => {
                write!(f, "purchase-orders:DeletePurchaseOrder")
            }
            PurchaseOrdersActions::GetConsoleActionSetEnforced => {
                write!(f, "purchase-orders:GetConsoleActionSetEnforced")
            }
            PurchaseOrdersActions::GetPurchaseOrder => {
                write!(f, "purchase-orders:GetPurchaseOrder")
            }
            PurchaseOrdersActions::ListPurchaseOrderInvoices => {
                write!(f, "purchase-orders:ListPurchaseOrderInvoices")
            }
            PurchaseOrdersActions::ListPurchaseOrders => {
                write!(f, "purchase-orders:ListPurchaseOrders")
            }
            PurchaseOrdersActions::ListTagsForResource => {
                write!(f, "purchase-orders:ListTagsForResource")
            }
            PurchaseOrdersActions::ModifyPurchaseOrders => {
                write!(f, "purchase-orders:ModifyPurchaseOrders")
            }
            PurchaseOrdersActions::TagResource => write!(f, "purchase-orders:TagResource"),
            PurchaseOrdersActions::UntagResource => write!(f, "purchase-orders:UntagResource"),
            PurchaseOrdersActions::UpdateConsoleActionSetEnforced => {
                write!(f, "purchase-orders:UpdateConsoleActionSetEnforced")
            }
            PurchaseOrdersActions::UpdatePurchaseOrder => {
                write!(f, "purchase-orders:UpdatePurchaseOrder")
            }
            PurchaseOrdersActions::UpdatePurchaseOrderStatus => {
                write!(f, "purchase-orders:UpdatePurchaseOrderStatus")
            }
            PurchaseOrdersActions::ViewPurchaseOrders => {
                write!(f, "purchase-orders:ViewPurchaseOrders")
            }
        }
    }
}
