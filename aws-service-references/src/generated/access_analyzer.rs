// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum AccessAnalyzerActions {
    ApplyArchiveRule,
    CancelPolicyGeneration,
    CheckAccessNotGranted,
    CheckNoNewAccess,
    CheckNoPublicAccess,
    CreateAccessPreview,
    CreateAnalyzer,
    CreateArchiveRule,
    DeleteAnalyzer,
    DeleteArchiveRule,
    GenerateFindingRecommendation,
    GetAccessPreview,
    GetAnalyzedResource,
    GetAnalyzer,
    GetArchiveRule,
    GetFinding,
    GetFindingRecommendation,
    GetFindingsStatistics,
    GetGeneratedPolicy,
    ListAccessPreviewFindings,
    ListAccessPreviews,
    ListAnalyzedResources,
    ListAnalyzers,
    ListArchiveRules,
    ListFindings,
    ListPolicyGenerations,
    ListTagsForResource,
    StartPolicyGeneration,
    StartResourceScan,
    TagResource,
    UntagResource,
    UpdateAnalyzer,
    UpdateArchiveRule,
    UpdateFindings,
    ValidatePolicy,
}
impl std::fmt::Display for AccessAnalyzerActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AccessAnalyzerActions::ApplyArchiveRule => {
                write!(f, "access-analyzer:ApplyArchiveRule")
            }
            AccessAnalyzerActions::CancelPolicyGeneration => {
                write!(f, "access-analyzer:CancelPolicyGeneration")
            }
            AccessAnalyzerActions::CheckAccessNotGranted => {
                write!(f, "access-analyzer:CheckAccessNotGranted")
            }
            AccessAnalyzerActions::CheckNoNewAccess => {
                write!(f, "access-analyzer:CheckNoNewAccess")
            }
            AccessAnalyzerActions::CheckNoPublicAccess => {
                write!(f, "access-analyzer:CheckNoPublicAccess")
            }
            AccessAnalyzerActions::CreateAccessPreview => {
                write!(f, "access-analyzer:CreateAccessPreview")
            }
            AccessAnalyzerActions::CreateAnalyzer => write!(f, "access-analyzer:CreateAnalyzer"),
            AccessAnalyzerActions::CreateArchiveRule => {
                write!(f, "access-analyzer:CreateArchiveRule")
            }
            AccessAnalyzerActions::DeleteAnalyzer => write!(f, "access-analyzer:DeleteAnalyzer"),
            AccessAnalyzerActions::DeleteArchiveRule => {
                write!(f, "access-analyzer:DeleteArchiveRule")
            }
            AccessAnalyzerActions::GenerateFindingRecommendation => {
                write!(f, "access-analyzer:GenerateFindingRecommendation")
            }
            AccessAnalyzerActions::GetAccessPreview => {
                write!(f, "access-analyzer:GetAccessPreview")
            }
            AccessAnalyzerActions::GetAnalyzedResource => {
                write!(f, "access-analyzer:GetAnalyzedResource")
            }
            AccessAnalyzerActions::GetAnalyzer => write!(f, "access-analyzer:GetAnalyzer"),
            AccessAnalyzerActions::GetArchiveRule => write!(f, "access-analyzer:GetArchiveRule"),
            AccessAnalyzerActions::GetFinding => write!(f, "access-analyzer:GetFinding"),
            AccessAnalyzerActions::GetFindingRecommendation => {
                write!(f, "access-analyzer:GetFindingRecommendation")
            }
            AccessAnalyzerActions::GetFindingsStatistics => {
                write!(f, "access-analyzer:GetFindingsStatistics")
            }
            AccessAnalyzerActions::GetGeneratedPolicy => {
                write!(f, "access-analyzer:GetGeneratedPolicy")
            }
            AccessAnalyzerActions::ListAccessPreviewFindings => {
                write!(f, "access-analyzer:ListAccessPreviewFindings")
            }
            AccessAnalyzerActions::ListAccessPreviews => {
                write!(f, "access-analyzer:ListAccessPreviews")
            }
            AccessAnalyzerActions::ListAnalyzedResources => {
                write!(f, "access-analyzer:ListAnalyzedResources")
            }
            AccessAnalyzerActions::ListAnalyzers => write!(f, "access-analyzer:ListAnalyzers"),
            AccessAnalyzerActions::ListArchiveRules => {
                write!(f, "access-analyzer:ListArchiveRules")
            }
            AccessAnalyzerActions::ListFindings => write!(f, "access-analyzer:ListFindings"),
            AccessAnalyzerActions::ListPolicyGenerations => {
                write!(f, "access-analyzer:ListPolicyGenerations")
            }
            AccessAnalyzerActions::ListTagsForResource => {
                write!(f, "access-analyzer:ListTagsForResource")
            }
            AccessAnalyzerActions::StartPolicyGeneration => {
                write!(f, "access-analyzer:StartPolicyGeneration")
            }
            AccessAnalyzerActions::StartResourceScan => {
                write!(f, "access-analyzer:StartResourceScan")
            }
            AccessAnalyzerActions::TagResource => write!(f, "access-analyzer:TagResource"),
            AccessAnalyzerActions::UntagResource => write!(f, "access-analyzer:UntagResource"),
            AccessAnalyzerActions::UpdateAnalyzer => write!(f, "access-analyzer:UpdateAnalyzer"),
            AccessAnalyzerActions::UpdateArchiveRule => {
                write!(f, "access-analyzer:UpdateArchiveRule")
            }
            AccessAnalyzerActions::UpdateFindings => write!(f, "access-analyzer:UpdateFindings"),
            AccessAnalyzerActions::ValidatePolicy => write!(f, "access-analyzer:ValidatePolicy"),
        }
    }
}
