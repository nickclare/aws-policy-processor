// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum IqPermissionActions {
    ApproveAccessGrant,
    ApprovePermissionRequest,
    AssumePermissionRole,
    CreatePermissionRequest,
    GetPermissionRequest,
    ListPermissionRequests,
    RejectPermissionRequest,
    RevokePermissionRequest,
    WithdrawPermissionRequest,
}
impl std::fmt::Display for IqPermissionActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IqPermissionActions::ApproveAccessGrant => {
                write!(f, "iq-permission:ApproveAccessGrant")
            }
            IqPermissionActions::ApprovePermissionRequest => {
                write!(f, "iq-permission:ApprovePermissionRequest")
            }
            IqPermissionActions::AssumePermissionRole => {
                write!(f, "iq-permission:AssumePermissionRole")
            }
            IqPermissionActions::CreatePermissionRequest => {
                write!(f, "iq-permission:CreatePermissionRequest")
            }
            IqPermissionActions::GetPermissionRequest => {
                write!(f, "iq-permission:GetPermissionRequest")
            }
            IqPermissionActions::ListPermissionRequests => {
                write!(f, "iq-permission:ListPermissionRequests")
            }
            IqPermissionActions::RejectPermissionRequest => {
                write!(f, "iq-permission:RejectPermissionRequest")
            }
            IqPermissionActions::RevokePermissionRequest => {
                write!(f, "iq-permission:RevokePermissionRequest")
            }
            IqPermissionActions::WithdrawPermissionRequest => {
                write!(f, "iq-permission:WithdrawPermissionRequest")
            }
        }
    }
}
