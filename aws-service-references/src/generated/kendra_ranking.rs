// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum KendraRankingActions {
    CreateRescoreExecutionPlan,
    DeleteRescoreExecutionPlan,
    DescribeRescoreExecutionPlan,
    ListRescoreExecutionPlans,
    ListTagsForResource,
    Rescore,
    TagResource,
    UntagResource,
    UpdateRescoreExecutionPlan,
}
impl std::fmt::Display for KendraRankingActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KendraRankingActions::CreateRescoreExecutionPlan => {
                write!(f, "kendra-ranking:CreateRescoreExecutionPlan")
            }
            KendraRankingActions::DeleteRescoreExecutionPlan => {
                write!(f, "kendra-ranking:DeleteRescoreExecutionPlan")
            }
            KendraRankingActions::DescribeRescoreExecutionPlan => {
                write!(f, "kendra-ranking:DescribeRescoreExecutionPlan")
            }
            KendraRankingActions::ListRescoreExecutionPlans => {
                write!(f, "kendra-ranking:ListRescoreExecutionPlans")
            }
            KendraRankingActions::ListTagsForResource => {
                write!(f, "kendra-ranking:ListTagsForResource")
            }
            KendraRankingActions::Rescore => write!(f, "kendra-ranking:Rescore"),
            KendraRankingActions::TagResource => write!(f, "kendra-ranking:TagResource"),
            KendraRankingActions::UntagResource => write!(f, "kendra-ranking:UntagResource"),
            KendraRankingActions::UpdateRescoreExecutionPlan => {
                write!(f, "kendra-ranking:UpdateRescoreExecutionPlan")
            }
        }
    }
}
