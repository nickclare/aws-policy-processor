// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum IdentitySyncActions {
    AllowVendedLogDeliveryForResource,
    CreateSyncFilter,
    CreateSyncProfile,
    CreateSyncTarget,
    DeleteSyncFilter,
    DeleteSyncProfile,
    DeleteSyncTarget,
    GetSyncProfile,
    GetSyncTarget,
    ListSyncFilters,
    StartSync,
    StopSync,
    UpdateSyncTarget,
}
impl std::fmt::Display for IdentitySyncActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IdentitySyncActions::AllowVendedLogDeliveryForResource => {
                write!(f, "identity-sync:AllowVendedLogDeliveryForResource")
            }
            IdentitySyncActions::CreateSyncFilter => write!(f, "identity-sync:CreateSyncFilter"),
            IdentitySyncActions::CreateSyncProfile => write!(f, "identity-sync:CreateSyncProfile"),
            IdentitySyncActions::CreateSyncTarget => write!(f, "identity-sync:CreateSyncTarget"),
            IdentitySyncActions::DeleteSyncFilter => write!(f, "identity-sync:DeleteSyncFilter"),
            IdentitySyncActions::DeleteSyncProfile => write!(f, "identity-sync:DeleteSyncProfile"),
            IdentitySyncActions::DeleteSyncTarget => write!(f, "identity-sync:DeleteSyncTarget"),
            IdentitySyncActions::GetSyncProfile => write!(f, "identity-sync:GetSyncProfile"),
            IdentitySyncActions::GetSyncTarget => write!(f, "identity-sync:GetSyncTarget"),
            IdentitySyncActions::ListSyncFilters => write!(f, "identity-sync:ListSyncFilters"),
            IdentitySyncActions::StartSync => write!(f, "identity-sync:StartSync"),
            IdentitySyncActions::StopSync => write!(f, "identity-sync:StopSync"),
            IdentitySyncActions::UpdateSyncTarget => write!(f, "identity-sync:UpdateSyncTarget"),
        }
    }
}
