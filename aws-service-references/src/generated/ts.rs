// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum TsActions {
    GetExecution,
    GetExecutionOutput,
    GetTool,
    ListExecutions,
    ListTagsForResource,
    ListTools,
    StartExecution,
    TagResource,
    UntagResource,
}
impl std::fmt::Display for TsActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TsActions::GetExecution => write!(f, "ts:GetExecution"),
            TsActions::GetExecutionOutput => write!(f, "ts:GetExecutionOutput"),
            TsActions::GetTool => write!(f, "ts:GetTool"),
            TsActions::ListExecutions => write!(f, "ts:ListExecutions"),
            TsActions::ListTagsForResource => write!(f, "ts:ListTagsForResource"),
            TsActions::ListTools => write!(f, "ts:ListTools"),
            TsActions::StartExecution => write!(f, "ts:StartExecution"),
            TsActions::TagResource => write!(f, "ts:TagResource"),
            TsActions::UntagResource => write!(f, "ts:UntagResource"),
        }
    }
}
