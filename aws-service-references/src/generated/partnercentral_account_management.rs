// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum PartnercentralAccountManagementActions {
    AssociatePartnerAccount,
    AssociatePartnerUser,
    DisassociatePartnerUser,
}
impl std::fmt::Display for PartnercentralAccountManagementActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PartnercentralAccountManagementActions::AssociatePartnerAccount => write!(
                f,
                "partnercentral-account-management:AssociatePartnerAccount"
            ),
            PartnercentralAccountManagementActions::AssociatePartnerUser => {
                write!(f, "partnercentral-account-management:AssociatePartnerUser")
            }
            PartnercentralAccountManagementActions::DisassociatePartnerUser => write!(
                f,
                "partnercentral-account-management:DisassociatePartnerUser"
            ),
        }
    }
}
