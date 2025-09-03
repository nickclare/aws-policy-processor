// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum NetworkSecurityDirectorActions {
    GetFinding,
    GetNetworkSecurityScan,
    GetResource,
    ListFindings,
    ListInsights,
    ListRemediations,
    ListResources,
    StartNetworkSecurityScan,
    UpdateFinding,
}
impl std::fmt::Display for NetworkSecurityDirectorActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NetworkSecurityDirectorActions::GetFinding => {
                write!(f, "network-security-director:GetFinding")
            }
            NetworkSecurityDirectorActions::GetNetworkSecurityScan => {
                write!(f, "network-security-director:GetNetworkSecurityScan")
            }
            NetworkSecurityDirectorActions::GetResource => {
                write!(f, "network-security-director:GetResource")
            }
            NetworkSecurityDirectorActions::ListFindings => {
                write!(f, "network-security-director:ListFindings")
            }
            NetworkSecurityDirectorActions::ListInsights => {
                write!(f, "network-security-director:ListInsights")
            }
            NetworkSecurityDirectorActions::ListRemediations => {
                write!(f, "network-security-director:ListRemediations")
            }
            NetworkSecurityDirectorActions::ListResources => {
                write!(f, "network-security-director:ListResources")
            }
            NetworkSecurityDirectorActions::StartNetworkSecurityScan => {
                write!(f, "network-security-director:StartNetworkSecurityScan")
            }
            NetworkSecurityDirectorActions::UpdateFinding => {
                write!(f, "network-security-director:UpdateFinding")
            }
        }
    }
}
