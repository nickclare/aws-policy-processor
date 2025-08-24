// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum ElementalAppliancesSoftwareActions {
    CompleteUpload,
    CreateOrderV1,
    CreateQuote,
    GetAvsCorrectAddress,
    GetBillingAddresses,
    GetDeliveryAddressesV2,
    GetOrder,
    GetOrdersV2,
    GetQuote,
    GetTaxes,
    ListQuotes,
    StartUpload,
    SubmitOrderV1,
    UpdateQuote,
}
impl std::fmt::Display for ElementalAppliancesSoftwareActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ElementalAppliancesSoftwareActions::CompleteUpload => {
                write!(f, "elemental-appliances-software:CompleteUpload")
            }
            ElementalAppliancesSoftwareActions::CreateOrderV1 => {
                write!(f, "elemental-appliances-software:CreateOrderV1")
            }
            ElementalAppliancesSoftwareActions::CreateQuote => {
                write!(f, "elemental-appliances-software:CreateQuote")
            }
            ElementalAppliancesSoftwareActions::GetAvsCorrectAddress => {
                write!(f, "elemental-appliances-software:GetAvsCorrectAddress")
            }
            ElementalAppliancesSoftwareActions::GetBillingAddresses => {
                write!(f, "elemental-appliances-software:GetBillingAddresses")
            }
            ElementalAppliancesSoftwareActions::GetDeliveryAddressesV2 => {
                write!(f, "elemental-appliances-software:GetDeliveryAddressesV2")
            }
            ElementalAppliancesSoftwareActions::GetOrder => {
                write!(f, "elemental-appliances-software:GetOrder")
            }
            ElementalAppliancesSoftwareActions::GetOrdersV2 => {
                write!(f, "elemental-appliances-software:GetOrdersV2")
            }
            ElementalAppliancesSoftwareActions::GetQuote => {
                write!(f, "elemental-appliances-software:GetQuote")
            }
            ElementalAppliancesSoftwareActions::GetTaxes => {
                write!(f, "elemental-appliances-software:GetTaxes")
            }
            ElementalAppliancesSoftwareActions::ListQuotes => {
                write!(f, "elemental-appliances-software:ListQuotes")
            }
            ElementalAppliancesSoftwareActions::StartUpload => {
                write!(f, "elemental-appliances-software:StartUpload")
            }
            ElementalAppliancesSoftwareActions::SubmitOrderV1 => {
                write!(f, "elemental-appliances-software:SubmitOrderV1")
            }
            ElementalAppliancesSoftwareActions::UpdateQuote => {
                write!(f, "elemental-appliances-software:UpdateQuote")
            }
        }
    }
}
