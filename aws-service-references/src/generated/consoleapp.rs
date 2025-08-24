// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum ConsoleappActions {
    GetDeviceIdentity,
    ListDeviceIdentities,
}
impl std::fmt::Display for ConsoleappActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConsoleappActions::GetDeviceIdentity => write!(f, "consoleapp:GetDeviceIdentity"),
            ConsoleappActions::ListDeviceIdentities => write!(f, "consoleapp:ListDeviceIdentities"),
        }
    }
}
