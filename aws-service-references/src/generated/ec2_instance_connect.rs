// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum Ec2InstanceConnectActions {
    OpenTunnel,
    SendSerialConsoleSshPublicKey,
    SendSshPublicKey,
}
impl std::fmt::Display for Ec2InstanceConnectActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Ec2InstanceConnectActions::OpenTunnel => write!(f, "ec2-instance-connect:OpenTunnel"),
            Ec2InstanceConnectActions::SendSerialConsoleSshPublicKey => {
                write!(f, "ec2-instance-connect:SendSerialConsoleSSHPublicKey")
            }
            Ec2InstanceConnectActions::SendSshPublicKey => {
                write!(f, "ec2-instance-connect:SendSSHPublicKey")
            }
        }
    }
}
