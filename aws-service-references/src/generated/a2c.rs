// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum A2cActions {
    GetContainerizationJobDetails,
    GetDeploymentJobDetails,
    StartContainerizationJob,
    StartDeploymentJob,
}
impl std::fmt::Display for A2cActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            A2cActions::GetContainerizationJobDetails => {
                write!(f, "a2c:GetContainerizationJobDetails")
            }
            A2cActions::GetDeploymentJobDetails => write!(f, "a2c:GetDeploymentJobDetails"),
            A2cActions::StartContainerizationJob => write!(f, "a2c:StartContainerizationJob"),
            A2cActions::StartDeploymentJob => write!(f, "a2c:StartDeploymentJob"),
        }
    }
}
