// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum SavingsplansActions {
    CreateSavingsPlan,
    DeleteQueuedSavingsPlan,
    DescribeSavingsPlanRates,
    DescribeSavingsPlans,
    DescribeSavingsPlansOfferingRates,
    DescribeSavingsPlansOfferings,
    ListTagsForResource,
    ReturnSavingsPlan,
    TagResource,
    UntagResource,
}
impl std::fmt::Display for SavingsplansActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SavingsplansActions::CreateSavingsPlan => write!(f, "savingsplans:CreateSavingsPlan"),
            SavingsplansActions::DeleteQueuedSavingsPlan => {
                write!(f, "savingsplans:DeleteQueuedSavingsPlan")
            }
            SavingsplansActions::DescribeSavingsPlanRates => {
                write!(f, "savingsplans:DescribeSavingsPlanRates")
            }
            SavingsplansActions::DescribeSavingsPlans => {
                write!(f, "savingsplans:DescribeSavingsPlans")
            }
            SavingsplansActions::DescribeSavingsPlansOfferingRates => {
                write!(f, "savingsplans:DescribeSavingsPlansOfferingRates")
            }
            SavingsplansActions::DescribeSavingsPlansOfferings => {
                write!(f, "savingsplans:DescribeSavingsPlansOfferings")
            }
            SavingsplansActions::ListTagsForResource => {
                write!(f, "savingsplans:ListTagsForResource")
            }
            SavingsplansActions::ReturnSavingsPlan => write!(f, "savingsplans:ReturnSavingsPlan"),
            SavingsplansActions::TagResource => write!(f, "savingsplans:TagResource"),
            SavingsplansActions::UntagResource => write!(f, "savingsplans:UntagResource"),
        }
    }
}
