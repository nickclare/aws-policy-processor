// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum BudgetsActions {
    CreateBudgetAction,
    DeleteBudgetAction,
    DescribeBudgetAction,
    DescribeBudgetActionHistories,
    DescribeBudgetActionsForAccount,
    DescribeBudgetActionsForBudget,
    ExecuteBudgetAction,
    ListTagsForResource,
    ModifyBudget,
    TagResource,
    UntagResource,
    UpdateBudgetAction,
    ViewBudget,
}
impl std::fmt::Display for BudgetsActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BudgetsActions::CreateBudgetAction => write!(f, "budgets:CreateBudgetAction"),
            BudgetsActions::DeleteBudgetAction => write!(f, "budgets:DeleteBudgetAction"),
            BudgetsActions::DescribeBudgetAction => write!(f, "budgets:DescribeBudgetAction"),
            BudgetsActions::DescribeBudgetActionHistories => {
                write!(f, "budgets:DescribeBudgetActionHistories")
            }
            BudgetsActions::DescribeBudgetActionsForAccount => {
                write!(f, "budgets:DescribeBudgetActionsForAccount")
            }
            BudgetsActions::DescribeBudgetActionsForBudget => {
                write!(f, "budgets:DescribeBudgetActionsForBudget")
            }
            BudgetsActions::ExecuteBudgetAction => write!(f, "budgets:ExecuteBudgetAction"),
            BudgetsActions::ListTagsForResource => write!(f, "budgets:ListTagsForResource"),
            BudgetsActions::ModifyBudget => write!(f, "budgets:ModifyBudget"),
            BudgetsActions::TagResource => write!(f, "budgets:TagResource"),
            BudgetsActions::UntagResource => write!(f, "budgets:UntagResource"),
            BudgetsActions::UpdateBudgetAction => write!(f, "budgets:UpdateBudgetAction"),
            BudgetsActions::ViewBudget => write!(f, "budgets:ViewBudget"),
        }
    }
}
