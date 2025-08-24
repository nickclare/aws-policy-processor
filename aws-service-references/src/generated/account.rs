// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum AccountActions {
    AcceptPrimaryEmailUpdate,
    CloseAccount,
    DeleteAlternateContact,
    DisableRegion,
    EnableRegion,
    GetAccountInformation,
    GetAlternateContact,
    GetContactInformation,
    GetPrimaryEmail,
    GetRegionOptStatus,
    ListRegions,
    PutAccountName,
    PutAlternateContact,
    PutContactInformation,
    StartPrimaryEmailUpdate,
}
impl std::fmt::Display for AccountActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AccountActions::AcceptPrimaryEmailUpdate => {
                write!(f, "account:AcceptPrimaryEmailUpdate")
            }
            AccountActions::CloseAccount => write!(f, "account:CloseAccount"),
            AccountActions::DeleteAlternateContact => write!(f, "account:DeleteAlternateContact"),
            AccountActions::DisableRegion => write!(f, "account:DisableRegion"),
            AccountActions::EnableRegion => write!(f, "account:EnableRegion"),
            AccountActions::GetAccountInformation => write!(f, "account:GetAccountInformation"),
            AccountActions::GetAlternateContact => write!(f, "account:GetAlternateContact"),
            AccountActions::GetContactInformation => write!(f, "account:GetContactInformation"),
            AccountActions::GetPrimaryEmail => write!(f, "account:GetPrimaryEmail"),
            AccountActions::GetRegionOptStatus => write!(f, "account:GetRegionOptStatus"),
            AccountActions::ListRegions => write!(f, "account:ListRegions"),
            AccountActions::PutAccountName => write!(f, "account:PutAccountName"),
            AccountActions::PutAlternateContact => write!(f, "account:PutAlternateContact"),
            AccountActions::PutContactInformation => write!(f, "account:PutContactInformation"),
            AccountActions::StartPrimaryEmailUpdate => write!(f, "account:StartPrimaryEmailUpdate"),
        }
    }
}
