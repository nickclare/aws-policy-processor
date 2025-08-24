// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum SqsActions {
    AddPermission,
    CancelMessageMoveTask,
    ChangeMessageVisibility,
    CreateQueue,
    DeleteMessage,
    DeleteQueue,
    GetQueueAttributes,
    GetQueueUrl,
    ListDeadLetterSourceQueues,
    ListMessageMoveTasks,
    ListQueueTags,
    ListQueues,
    PurgeQueue,
    ReceiveMessage,
    RemovePermission,
    SendMessage,
    SetQueueAttributes,
    StartMessageMoveTask,
    TagQueue,
    UntagQueue,
}
impl std::fmt::Display for SqsActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SqsActions::AddPermission => write!(f, "sqs:AddPermission"),
            SqsActions::CancelMessageMoveTask => write!(f, "sqs:CancelMessageMoveTask"),
            SqsActions::ChangeMessageVisibility => write!(f, "sqs:ChangeMessageVisibility"),
            SqsActions::CreateQueue => write!(f, "sqs:CreateQueue"),
            SqsActions::DeleteMessage => write!(f, "sqs:DeleteMessage"),
            SqsActions::DeleteQueue => write!(f, "sqs:DeleteQueue"),
            SqsActions::GetQueueAttributes => write!(f, "sqs:GetQueueAttributes"),
            SqsActions::GetQueueUrl => write!(f, "sqs:GetQueueUrl"),
            SqsActions::ListDeadLetterSourceQueues => write!(f, "sqs:ListDeadLetterSourceQueues"),
            SqsActions::ListMessageMoveTasks => write!(f, "sqs:ListMessageMoveTasks"),
            SqsActions::ListQueueTags => write!(f, "sqs:ListQueueTags"),
            SqsActions::ListQueues => write!(f, "sqs:ListQueues"),
            SqsActions::PurgeQueue => write!(f, "sqs:PurgeQueue"),
            SqsActions::ReceiveMessage => write!(f, "sqs:ReceiveMessage"),
            SqsActions::RemovePermission => write!(f, "sqs:RemovePermission"),
            SqsActions::SendMessage => write!(f, "sqs:SendMessage"),
            SqsActions::SetQueueAttributes => write!(f, "sqs:SetQueueAttributes"),
            SqsActions::StartMessageMoveTask => write!(f, "sqs:StartMessageMoveTask"),
            SqsActions::TagQueue => write!(f, "sqs:TagQueue"),
            SqsActions::UntagQueue => write!(f, "sqs:UntagQueue"),
        }
    }
}
