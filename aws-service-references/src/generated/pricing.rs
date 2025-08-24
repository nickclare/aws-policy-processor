// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum PricingActions {
    DescribeServices,
    GetAttributeValues,
    GetPriceListFileUrl,
    GetProducts,
    ListPriceLists,
}
impl std::fmt::Display for PricingActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PricingActions::DescribeServices => write!(f, "pricing:DescribeServices"),
            PricingActions::GetAttributeValues => write!(f, "pricing:GetAttributeValues"),
            PricingActions::GetPriceListFileUrl => write!(f, "pricing:GetPriceListFileUrl"),
            PricingActions::GetProducts => write!(f, "pricing:GetProducts"),
            PricingActions::ListPriceLists => write!(f, "pricing:ListPriceLists"),
        }
    }
}
