// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum InspectorActions {
    AddAttributesToFindings,
    CreateAssessmentTarget,
    CreateAssessmentTemplate,
    CreateExclusionsPreview,
    CreateResourceGroup,
    DeleteAssessmentRun,
    DeleteAssessmentTarget,
    DeleteAssessmentTemplate,
    DescribeAssessmentRuns,
    DescribeAssessmentTargets,
    DescribeAssessmentTemplates,
    DescribeCrossAccountAccessRole,
    DescribeExclusions,
    DescribeFindings,
    DescribeResourceGroups,
    DescribeRulesPackages,
    GetAssessmentReport,
    GetExclusionsPreview,
    GetTelemetryMetadata,
    ListAssessmentRunAgents,
    ListAssessmentRuns,
    ListAssessmentTargets,
    ListAssessmentTemplates,
    ListEventSubscriptions,
    ListExclusions,
    ListFindings,
    ListRulesPackages,
    ListTagsForResource,
    PreviewAgents,
    RegisterCrossAccountAccessRole,
    RemoveAttributesFromFindings,
    SetTagsForResource,
    StartAssessmentRun,
    StopAssessmentRun,
    SubscribeToEvent,
    UnsubscribeFromEvent,
    UpdateAssessmentTarget,
}
impl std::fmt::Display for InspectorActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InspectorActions::AddAttributesToFindings => {
                write!(f, "inspector:AddAttributesToFindings")
            }
            InspectorActions::CreateAssessmentTarget => {
                write!(f, "inspector:CreateAssessmentTarget")
            }
            InspectorActions::CreateAssessmentTemplate => {
                write!(f, "inspector:CreateAssessmentTemplate")
            }
            InspectorActions::CreateExclusionsPreview => {
                write!(f, "inspector:CreateExclusionsPreview")
            }
            InspectorActions::CreateResourceGroup => write!(f, "inspector:CreateResourceGroup"),
            InspectorActions::DeleteAssessmentRun => write!(f, "inspector:DeleteAssessmentRun"),
            InspectorActions::DeleteAssessmentTarget => {
                write!(f, "inspector:DeleteAssessmentTarget")
            }
            InspectorActions::DeleteAssessmentTemplate => {
                write!(f, "inspector:DeleteAssessmentTemplate")
            }
            InspectorActions::DescribeAssessmentRuns => {
                write!(f, "inspector:DescribeAssessmentRuns")
            }
            InspectorActions::DescribeAssessmentTargets => {
                write!(f, "inspector:DescribeAssessmentTargets")
            }
            InspectorActions::DescribeAssessmentTemplates => {
                write!(f, "inspector:DescribeAssessmentTemplates")
            }
            InspectorActions::DescribeCrossAccountAccessRole => {
                write!(f, "inspector:DescribeCrossAccountAccessRole")
            }
            InspectorActions::DescribeExclusions => write!(f, "inspector:DescribeExclusions"),
            InspectorActions::DescribeFindings => write!(f, "inspector:DescribeFindings"),
            InspectorActions::DescribeResourceGroups => {
                write!(f, "inspector:DescribeResourceGroups")
            }
            InspectorActions::DescribeRulesPackages => write!(f, "inspector:DescribeRulesPackages"),
            InspectorActions::GetAssessmentReport => write!(f, "inspector:GetAssessmentReport"),
            InspectorActions::GetExclusionsPreview => write!(f, "inspector:GetExclusionsPreview"),
            InspectorActions::GetTelemetryMetadata => write!(f, "inspector:GetTelemetryMetadata"),
            InspectorActions::ListAssessmentRunAgents => {
                write!(f, "inspector:ListAssessmentRunAgents")
            }
            InspectorActions::ListAssessmentRuns => write!(f, "inspector:ListAssessmentRuns"),
            InspectorActions::ListAssessmentTargets => write!(f, "inspector:ListAssessmentTargets"),
            InspectorActions::ListAssessmentTemplates => {
                write!(f, "inspector:ListAssessmentTemplates")
            }
            InspectorActions::ListEventSubscriptions => {
                write!(f, "inspector:ListEventSubscriptions")
            }
            InspectorActions::ListExclusions => write!(f, "inspector:ListExclusions"),
            InspectorActions::ListFindings => write!(f, "inspector:ListFindings"),
            InspectorActions::ListRulesPackages => write!(f, "inspector:ListRulesPackages"),
            InspectorActions::ListTagsForResource => write!(f, "inspector:ListTagsForResource"),
            InspectorActions::PreviewAgents => write!(f, "inspector:PreviewAgents"),
            InspectorActions::RegisterCrossAccountAccessRole => {
                write!(f, "inspector:RegisterCrossAccountAccessRole")
            }
            InspectorActions::RemoveAttributesFromFindings => {
                write!(f, "inspector:RemoveAttributesFromFindings")
            }
            InspectorActions::SetTagsForResource => write!(f, "inspector:SetTagsForResource"),
            InspectorActions::StartAssessmentRun => write!(f, "inspector:StartAssessmentRun"),
            InspectorActions::StopAssessmentRun => write!(f, "inspector:StopAssessmentRun"),
            InspectorActions::SubscribeToEvent => write!(f, "inspector:SubscribeToEvent"),
            InspectorActions::UnsubscribeFromEvent => write!(f, "inspector:UnsubscribeFromEvent"),
            InspectorActions::UpdateAssessmentTarget => {
                write!(f, "inspector:UpdateAssessmentTarget")
            }
        }
    }
}
