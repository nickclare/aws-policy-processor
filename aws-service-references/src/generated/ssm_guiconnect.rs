// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum SsmGuiconnectActions {
    CancelConnection,
    DeleteConnectionRecordingPreferences,
    GetConnection,
    GetConnectionRecordingPreferences,
    ListConnections,
    StartConnection,
    UpdateConnectionRecordingPreferences,
}
impl std::fmt::Display for SsmGuiconnectActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SsmGuiconnectActions::CancelConnection => write!(f, "ssm-guiconnect:CancelConnection"),
            SsmGuiconnectActions::DeleteConnectionRecordingPreferences => {
                write!(f, "ssm-guiconnect:DeleteConnectionRecordingPreferences")
            }
            SsmGuiconnectActions::GetConnection => write!(f, "ssm-guiconnect:GetConnection"),
            SsmGuiconnectActions::GetConnectionRecordingPreferences => {
                write!(f, "ssm-guiconnect:GetConnectionRecordingPreferences")
            }
            SsmGuiconnectActions::ListConnections => write!(f, "ssm-guiconnect:ListConnections"),
            SsmGuiconnectActions::StartConnection => write!(f, "ssm-guiconnect:StartConnection"),
            SsmGuiconnectActions::UpdateConnectionRecordingPreferences => {
                write!(f, "ssm-guiconnect:UpdateConnectionRecordingPreferences")
            }
        }
    }
}
