// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum AwsconnectorActions {
    GetConnectorHealth,
    RegisterConnector,
    ValidateConnectorId,
}
impl std::fmt::Display for AwsconnectorActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AwsconnectorActions::GetConnectorHealth => write!(f, "awsconnector:GetConnectorHealth"),
            AwsconnectorActions::RegisterConnector => write!(f, "awsconnector:RegisterConnector"),
            AwsconnectorActions::ValidateConnectorId => {
                write!(f, "awsconnector:ValidateConnectorId")
            }
        }
    }
}
