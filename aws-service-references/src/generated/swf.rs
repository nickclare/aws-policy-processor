// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum SwfActions {
    CancelTimer,
    CancelWorkflowExecution,
    CompleteWorkflowExecution,
    ContinueAsNewWorkflowExecution,
    CountClosedWorkflowExecutions,
    CountOpenWorkflowExecutions,
    CountPendingActivityTasks,
    CountPendingDecisionTasks,
    DeleteActivityType,
    DeleteWorkflowType,
    DeprecateActivityType,
    DeprecateDomain,
    DeprecateWorkflowType,
    DescribeActivityType,
    DescribeDomain,
    DescribeWorkflowExecution,
    DescribeWorkflowType,
    FailWorkflowExecution,
    GetWorkflowExecutionHistory,
    ListActivityTypes,
    ListClosedWorkflowExecutions,
    ListDomains,
    ListOpenWorkflowExecutions,
    ListTagsForResource,
    ListWorkflowTypes,
    PollForActivityTask,
    PollForDecisionTask,
    RecordActivityTaskHeartbeat,
    RecordMarker,
    RegisterActivityType,
    RegisterDomain,
    RegisterWorkflowType,
    RequestCancelActivityTask,
    RequestCancelExternalWorkflowExecution,
    RequestCancelWorkflowExecution,
    RespondActivityTaskCanceled,
    RespondActivityTaskCompleted,
    RespondActivityTaskFailed,
    RespondDecisionTaskCompleted,
    ScheduleActivityTask,
    SignalExternalWorkflowExecution,
    SignalWorkflowExecution,
    StartChildWorkflowExecution,
    StartTimer,
    StartWorkflowExecution,
    TagResource,
    TerminateWorkflowExecution,
    UndeprecateActivityType,
    UndeprecateDomain,
    UndeprecateWorkflowType,
    UntagResource,
}
impl std::fmt::Display for SwfActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SwfActions::CancelTimer => write!(f, "swf:CancelTimer"),
            SwfActions::CancelWorkflowExecution => write!(f, "swf:CancelWorkflowExecution"),
            SwfActions::CompleteWorkflowExecution => write!(f, "swf:CompleteWorkflowExecution"),
            SwfActions::ContinueAsNewWorkflowExecution => {
                write!(f, "swf:ContinueAsNewWorkflowExecution")
            }
            SwfActions::CountClosedWorkflowExecutions => {
                write!(f, "swf:CountClosedWorkflowExecutions")
            }
            SwfActions::CountOpenWorkflowExecutions => write!(f, "swf:CountOpenWorkflowExecutions"),
            SwfActions::CountPendingActivityTasks => write!(f, "swf:CountPendingActivityTasks"),
            SwfActions::CountPendingDecisionTasks => write!(f, "swf:CountPendingDecisionTasks"),
            SwfActions::DeleteActivityType => write!(f, "swf:DeleteActivityType"),
            SwfActions::DeleteWorkflowType => write!(f, "swf:DeleteWorkflowType"),
            SwfActions::DeprecateActivityType => write!(f, "swf:DeprecateActivityType"),
            SwfActions::DeprecateDomain => write!(f, "swf:DeprecateDomain"),
            SwfActions::DeprecateWorkflowType => write!(f, "swf:DeprecateWorkflowType"),
            SwfActions::DescribeActivityType => write!(f, "swf:DescribeActivityType"),
            SwfActions::DescribeDomain => write!(f, "swf:DescribeDomain"),
            SwfActions::DescribeWorkflowExecution => write!(f, "swf:DescribeWorkflowExecution"),
            SwfActions::DescribeWorkflowType => write!(f, "swf:DescribeWorkflowType"),
            SwfActions::FailWorkflowExecution => write!(f, "swf:FailWorkflowExecution"),
            SwfActions::GetWorkflowExecutionHistory => write!(f, "swf:GetWorkflowExecutionHistory"),
            SwfActions::ListActivityTypes => write!(f, "swf:ListActivityTypes"),
            SwfActions::ListClosedWorkflowExecutions => {
                write!(f, "swf:ListClosedWorkflowExecutions")
            }
            SwfActions::ListDomains => write!(f, "swf:ListDomains"),
            SwfActions::ListOpenWorkflowExecutions => write!(f, "swf:ListOpenWorkflowExecutions"),
            SwfActions::ListTagsForResource => write!(f, "swf:ListTagsForResource"),
            SwfActions::ListWorkflowTypes => write!(f, "swf:ListWorkflowTypes"),
            SwfActions::PollForActivityTask => write!(f, "swf:PollForActivityTask"),
            SwfActions::PollForDecisionTask => write!(f, "swf:PollForDecisionTask"),
            SwfActions::RecordActivityTaskHeartbeat => write!(f, "swf:RecordActivityTaskHeartbeat"),
            SwfActions::RecordMarker => write!(f, "swf:RecordMarker"),
            SwfActions::RegisterActivityType => write!(f, "swf:RegisterActivityType"),
            SwfActions::RegisterDomain => write!(f, "swf:RegisterDomain"),
            SwfActions::RegisterWorkflowType => write!(f, "swf:RegisterWorkflowType"),
            SwfActions::RequestCancelActivityTask => write!(f, "swf:RequestCancelActivityTask"),
            SwfActions::RequestCancelExternalWorkflowExecution => {
                write!(f, "swf:RequestCancelExternalWorkflowExecution")
            }
            SwfActions::RequestCancelWorkflowExecution => {
                write!(f, "swf:RequestCancelWorkflowExecution")
            }
            SwfActions::RespondActivityTaskCanceled => write!(f, "swf:RespondActivityTaskCanceled"),
            SwfActions::RespondActivityTaskCompleted => {
                write!(f, "swf:RespondActivityTaskCompleted")
            }
            SwfActions::RespondActivityTaskFailed => write!(f, "swf:RespondActivityTaskFailed"),
            SwfActions::RespondDecisionTaskCompleted => {
                write!(f, "swf:RespondDecisionTaskCompleted")
            }
            SwfActions::ScheduleActivityTask => write!(f, "swf:ScheduleActivityTask"),
            SwfActions::SignalExternalWorkflowExecution => {
                write!(f, "swf:SignalExternalWorkflowExecution")
            }
            SwfActions::SignalWorkflowExecution => write!(f, "swf:SignalWorkflowExecution"),
            SwfActions::StartChildWorkflowExecution => write!(f, "swf:StartChildWorkflowExecution"),
            SwfActions::StartTimer => write!(f, "swf:StartTimer"),
            SwfActions::StartWorkflowExecution => write!(f, "swf:StartWorkflowExecution"),
            SwfActions::TagResource => write!(f, "swf:TagResource"),
            SwfActions::TerminateWorkflowExecution => write!(f, "swf:TerminateWorkflowExecution"),
            SwfActions::UndeprecateActivityType => write!(f, "swf:UndeprecateActivityType"),
            SwfActions::UndeprecateDomain => write!(f, "swf:UndeprecateDomain"),
            SwfActions::UndeprecateWorkflowType => write!(f, "swf:UndeprecateWorkflowType"),
            SwfActions::UntagResource => write!(f, "swf:UntagResource"),
        }
    }
}
