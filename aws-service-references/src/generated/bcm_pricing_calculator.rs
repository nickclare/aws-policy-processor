// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum BcmPricingCalculatorActions {
    CreateBillEstimate,
    CreateBillScenario,
    CreateBillScenarioCommitmentModification,
    CreateBillScenarioUsageModification,
    CreateWorkloadEstimate,
    CreateWorkloadEstimateUsage,
    DeleteBillEstimate,
    DeleteBillScenario,
    DeleteBillScenarioCommitmentModification,
    DeleteBillScenarioUsageModification,
    DeleteWorkloadEstimate,
    DeleteWorkloadEstimateUsage,
    GetBillEstimate,
    GetBillScenario,
    GetPreferences,
    GetWorkloadEstimate,
    ListBillEstimateCommitments,
    ListBillEstimateInputCommitmentModifications,
    ListBillEstimateInputUsageModifications,
    ListBillEstimateLineItems,
    ListBillEstimates,
    ListBillScenarioCommitmentModifications,
    ListBillScenarioUsageModifications,
    ListBillScenarios,
    ListTagsForResource,
    ListWorkloadEstimateUsage,
    ListWorkloadEstimates,
    TagResource,
    UntagResource,
    UpdateBillEstimate,
    UpdateBillScenario,
    UpdateBillScenarioCommitmentModification,
    UpdateBillScenarioUsageModification,
    UpdatePreferences,
    UpdateWorkloadEstimate,
    UpdateWorkloadEstimateUsage,
}
impl std::fmt::Display for BcmPricingCalculatorActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BcmPricingCalculatorActions::CreateBillEstimate => {
                write!(f, "bcm-pricing-calculator:CreateBillEstimate")
            }
            BcmPricingCalculatorActions::CreateBillScenario => {
                write!(f, "bcm-pricing-calculator:CreateBillScenario")
            }
            BcmPricingCalculatorActions::CreateBillScenarioCommitmentModification => write!(
                f,
                "bcm-pricing-calculator:CreateBillScenarioCommitmentModification"
            ),
            BcmPricingCalculatorActions::CreateBillScenarioUsageModification => write!(
                f,
                "bcm-pricing-calculator:CreateBillScenarioUsageModification"
            ),
            BcmPricingCalculatorActions::CreateWorkloadEstimate => {
                write!(f, "bcm-pricing-calculator:CreateWorkloadEstimate")
            }
            BcmPricingCalculatorActions::CreateWorkloadEstimateUsage => {
                write!(f, "bcm-pricing-calculator:CreateWorkloadEstimateUsage")
            }
            BcmPricingCalculatorActions::DeleteBillEstimate => {
                write!(f, "bcm-pricing-calculator:DeleteBillEstimate")
            }
            BcmPricingCalculatorActions::DeleteBillScenario => {
                write!(f, "bcm-pricing-calculator:DeleteBillScenario")
            }
            BcmPricingCalculatorActions::DeleteBillScenarioCommitmentModification => write!(
                f,
                "bcm-pricing-calculator:DeleteBillScenarioCommitmentModification"
            ),
            BcmPricingCalculatorActions::DeleteBillScenarioUsageModification => write!(
                f,
                "bcm-pricing-calculator:DeleteBillScenarioUsageModification"
            ),
            BcmPricingCalculatorActions::DeleteWorkloadEstimate => {
                write!(f, "bcm-pricing-calculator:DeleteWorkloadEstimate")
            }
            BcmPricingCalculatorActions::DeleteWorkloadEstimateUsage => {
                write!(f, "bcm-pricing-calculator:DeleteWorkloadEstimateUsage")
            }
            BcmPricingCalculatorActions::GetBillEstimate => {
                write!(f, "bcm-pricing-calculator:GetBillEstimate")
            }
            BcmPricingCalculatorActions::GetBillScenario => {
                write!(f, "bcm-pricing-calculator:GetBillScenario")
            }
            BcmPricingCalculatorActions::GetPreferences => {
                write!(f, "bcm-pricing-calculator:GetPreferences")
            }
            BcmPricingCalculatorActions::GetWorkloadEstimate => {
                write!(f, "bcm-pricing-calculator:GetWorkloadEstimate")
            }
            BcmPricingCalculatorActions::ListBillEstimateCommitments => {
                write!(f, "bcm-pricing-calculator:ListBillEstimateCommitments")
            }
            BcmPricingCalculatorActions::ListBillEstimateInputCommitmentModifications => write!(
                f,
                "bcm-pricing-calculator:ListBillEstimateInputCommitmentModifications"
            ),
            BcmPricingCalculatorActions::ListBillEstimateInputUsageModifications => write!(
                f,
                "bcm-pricing-calculator:ListBillEstimateInputUsageModifications"
            ),
            BcmPricingCalculatorActions::ListBillEstimateLineItems => {
                write!(f, "bcm-pricing-calculator:ListBillEstimateLineItems")
            }
            BcmPricingCalculatorActions::ListBillEstimates => {
                write!(f, "bcm-pricing-calculator:ListBillEstimates")
            }
            BcmPricingCalculatorActions::ListBillScenarioCommitmentModifications => write!(
                f,
                "bcm-pricing-calculator:ListBillScenarioCommitmentModifications"
            ),
            BcmPricingCalculatorActions::ListBillScenarioUsageModifications => write!(
                f,
                "bcm-pricing-calculator:ListBillScenarioUsageModifications"
            ),
            BcmPricingCalculatorActions::ListBillScenarios => {
                write!(f, "bcm-pricing-calculator:ListBillScenarios")
            }
            BcmPricingCalculatorActions::ListTagsForResource => {
                write!(f, "bcm-pricing-calculator:ListTagsForResource")
            }
            BcmPricingCalculatorActions::ListWorkloadEstimateUsage => {
                write!(f, "bcm-pricing-calculator:ListWorkloadEstimateUsage")
            }
            BcmPricingCalculatorActions::ListWorkloadEstimates => {
                write!(f, "bcm-pricing-calculator:ListWorkloadEstimates")
            }
            BcmPricingCalculatorActions::TagResource => {
                write!(f, "bcm-pricing-calculator:TagResource")
            }
            BcmPricingCalculatorActions::UntagResource => {
                write!(f, "bcm-pricing-calculator:UntagResource")
            }
            BcmPricingCalculatorActions::UpdateBillEstimate => {
                write!(f, "bcm-pricing-calculator:UpdateBillEstimate")
            }
            BcmPricingCalculatorActions::UpdateBillScenario => {
                write!(f, "bcm-pricing-calculator:UpdateBillScenario")
            }
            BcmPricingCalculatorActions::UpdateBillScenarioCommitmentModification => write!(
                f,
                "bcm-pricing-calculator:UpdateBillScenarioCommitmentModification"
            ),
            BcmPricingCalculatorActions::UpdateBillScenarioUsageModification => write!(
                f,
                "bcm-pricing-calculator:UpdateBillScenarioUsageModification"
            ),
            BcmPricingCalculatorActions::UpdatePreferences => {
                write!(f, "bcm-pricing-calculator:UpdatePreferences")
            }
            BcmPricingCalculatorActions::UpdateWorkloadEstimate => {
                write!(f, "bcm-pricing-calculator:UpdateWorkloadEstimate")
            }
            BcmPricingCalculatorActions::UpdateWorkloadEstimateUsage => {
                write!(f, "bcm-pricing-calculator:UpdateWorkloadEstimateUsage")
            }
        }
    }
}
