// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum StatesActions {
    CreateActivity,
    CreateStateMachine,
    CreateStateMachineAlias,
    DeleteActivity,
    DeleteStateMachine,
    DeleteStateMachineAlias,
    DeleteStateMachineVersion,
    DescribeActivity,
    DescribeExecution,
    DescribeMapRun,
    DescribeStateMachine,
    DescribeStateMachineAlias,
    DescribeStateMachineForExecution,
    GetActivityTask,
    GetExecutionHistory,
    InvokeHttpEndpoint,
    ListActivities,
    ListExecutions,
    ListMapRuns,
    ListStateMachineAliases,
    ListStateMachineVersions,
    ListStateMachines,
    ListTagsForResource,
    PublishStateMachineVersion,
    RedriveExecution,
    RevealSecrets,
    SendTaskFailure,
    SendTaskHeartbeat,
    SendTaskSuccess,
    StartExecution,
    StartSyncExecution,
    StopExecution,
    TagResource,
    TestState,
    UntagResource,
    UpdateMapRun,
    UpdateStateMachine,
    UpdateStateMachineAlias,
    ValidateStateMachineDefinition,
}
impl std::fmt::Display for StatesActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StatesActions::CreateActivity => write!(f, "states:CreateActivity"),
            StatesActions::CreateStateMachine => write!(f, "states:CreateStateMachine"),
            StatesActions::CreateStateMachineAlias => write!(f, "states:CreateStateMachineAlias"),
            StatesActions::DeleteActivity => write!(f, "states:DeleteActivity"),
            StatesActions::DeleteStateMachine => write!(f, "states:DeleteStateMachine"),
            StatesActions::DeleteStateMachineAlias => write!(f, "states:DeleteStateMachineAlias"),
            StatesActions::DeleteStateMachineVersion => {
                write!(f, "states:DeleteStateMachineVersion")
            }
            StatesActions::DescribeActivity => write!(f, "states:DescribeActivity"),
            StatesActions::DescribeExecution => write!(f, "states:DescribeExecution"),
            StatesActions::DescribeMapRun => write!(f, "states:DescribeMapRun"),
            StatesActions::DescribeStateMachine => write!(f, "states:DescribeStateMachine"),
            StatesActions::DescribeStateMachineAlias => {
                write!(f, "states:DescribeStateMachineAlias")
            }
            StatesActions::DescribeStateMachineForExecution => {
                write!(f, "states:DescribeStateMachineForExecution")
            }
            StatesActions::GetActivityTask => write!(f, "states:GetActivityTask"),
            StatesActions::GetExecutionHistory => write!(f, "states:GetExecutionHistory"),
            StatesActions::InvokeHttpEndpoint => write!(f, "states:InvokeHTTPEndpoint"),
            StatesActions::ListActivities => write!(f, "states:ListActivities"),
            StatesActions::ListExecutions => write!(f, "states:ListExecutions"),
            StatesActions::ListMapRuns => write!(f, "states:ListMapRuns"),
            StatesActions::ListStateMachineAliases => write!(f, "states:ListStateMachineAliases"),
            StatesActions::ListStateMachineVersions => write!(f, "states:ListStateMachineVersions"),
            StatesActions::ListStateMachines => write!(f, "states:ListStateMachines"),
            StatesActions::ListTagsForResource => write!(f, "states:ListTagsForResource"),
            StatesActions::PublishStateMachineVersion => {
                write!(f, "states:PublishStateMachineVersion")
            }
            StatesActions::RedriveExecution => write!(f, "states:RedriveExecution"),
            StatesActions::RevealSecrets => write!(f, "states:RevealSecrets"),
            StatesActions::SendTaskFailure => write!(f, "states:SendTaskFailure"),
            StatesActions::SendTaskHeartbeat => write!(f, "states:SendTaskHeartbeat"),
            StatesActions::SendTaskSuccess => write!(f, "states:SendTaskSuccess"),
            StatesActions::StartExecution => write!(f, "states:StartExecution"),
            StatesActions::StartSyncExecution => write!(f, "states:StartSyncExecution"),
            StatesActions::StopExecution => write!(f, "states:StopExecution"),
            StatesActions::TagResource => write!(f, "states:TagResource"),
            StatesActions::TestState => write!(f, "states:TestState"),
            StatesActions::UntagResource => write!(f, "states:UntagResource"),
            StatesActions::UpdateMapRun => write!(f, "states:UpdateMapRun"),
            StatesActions::UpdateStateMachine => write!(f, "states:UpdateStateMachine"),
            StatesActions::UpdateStateMachineAlias => write!(f, "states:UpdateStateMachineAlias"),
            StatesActions::ValidateStateMachineDefinition => {
                write!(f, "states:ValidateStateMachineDefinition")
            }
        }
    }
}
