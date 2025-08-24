// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum CodedeployCommandsSecureActions {
    GetDeploymentSpecification,
    PollHostCommand,
    PutHostCommandAcknowledgement,
    PutHostCommandComplete,
}
impl std::fmt::Display for CodedeployCommandsSecureActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CodedeployCommandsSecureActions::GetDeploymentSpecification => {
                write!(f, "codedeploy-commands-secure:GetDeploymentSpecification")
            }
            CodedeployCommandsSecureActions::PollHostCommand => {
                write!(f, "codedeploy-commands-secure:PollHostCommand")
            }
            CodedeployCommandsSecureActions::PutHostCommandAcknowledgement => write!(
                f,
                "codedeploy-commands-secure:PutHostCommandAcknowledgement"
            ),
            CodedeployCommandsSecureActions::PutHostCommandComplete => {
                write!(f, "codedeploy-commands-secure:PutHostCommandComplete")
            }
        }
    }
}
