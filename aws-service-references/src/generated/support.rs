// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum SupportActions {
    AddAttachmentsToSet,
    AddCommunicationToCase,
    CreateCase,
    DescribeAttachment,
    DescribeCaseAttributes,
    DescribeCaseOptions,
    DescribeCases,
    DescribeCommunication,
    DescribeCommunications,
    DescribeCreateCaseOptions,
    DescribeIssueTypes,
    DescribeServices,
    DescribeSeverityLevels,
    DescribeSupportLevel,
    DescribeSupportedLanguages,
    DescribeTrustedAdvisorCheckRefreshStatuses,
    DescribeTrustedAdvisorCheckResult,
    DescribeTrustedAdvisorCheckSummaries,
    DescribeTrustedAdvisorChecks,
    GetInteraction,
    InitiateCallForCase,
    InitiateChatForCase,
    PutCaseAttributes,
    RateCaseCommunication,
    RefreshTrustedAdvisorCheck,
    ResolveCase,
    SearchForCases,
    StartInteraction,
    UpdateCaseSeverity,
    UpdateInteraction,
}
impl std::fmt::Display for SupportActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SupportActions::AddAttachmentsToSet => write!(f, "support:AddAttachmentsToSet"),
            SupportActions::AddCommunicationToCase => write!(f, "support:AddCommunicationToCase"),
            SupportActions::CreateCase => write!(f, "support:CreateCase"),
            SupportActions::DescribeAttachment => write!(f, "support:DescribeAttachment"),
            SupportActions::DescribeCaseAttributes => write!(f, "support:DescribeCaseAttributes"),
            SupportActions::DescribeCaseOptions => write!(f, "support:DescribeCaseOptions"),
            SupportActions::DescribeCases => write!(f, "support:DescribeCases"),
            SupportActions::DescribeCommunication => write!(f, "support:DescribeCommunication"),
            SupportActions::DescribeCommunications => write!(f, "support:DescribeCommunications"),
            SupportActions::DescribeCreateCaseOptions => {
                write!(f, "support:DescribeCreateCaseOptions")
            }
            SupportActions::DescribeIssueTypes => write!(f, "support:DescribeIssueTypes"),
            SupportActions::DescribeServices => write!(f, "support:DescribeServices"),
            SupportActions::DescribeSeverityLevels => write!(f, "support:DescribeSeverityLevels"),
            SupportActions::DescribeSupportLevel => write!(f, "support:DescribeSupportLevel"),
            SupportActions::DescribeSupportedLanguages => {
                write!(f, "support:DescribeSupportedLanguages")
            }
            SupportActions::DescribeTrustedAdvisorCheckRefreshStatuses => {
                write!(f, "support:DescribeTrustedAdvisorCheckRefreshStatuses")
            }
            SupportActions::DescribeTrustedAdvisorCheckResult => {
                write!(f, "support:DescribeTrustedAdvisorCheckResult")
            }
            SupportActions::DescribeTrustedAdvisorCheckSummaries => {
                write!(f, "support:DescribeTrustedAdvisorCheckSummaries")
            }
            SupportActions::DescribeTrustedAdvisorChecks => {
                write!(f, "support:DescribeTrustedAdvisorChecks")
            }
            SupportActions::GetInteraction => write!(f, "support:GetInteraction"),
            SupportActions::InitiateCallForCase => write!(f, "support:InitiateCallForCase"),
            SupportActions::InitiateChatForCase => write!(f, "support:InitiateChatForCase"),
            SupportActions::PutCaseAttributes => write!(f, "support:PutCaseAttributes"),
            SupportActions::RateCaseCommunication => write!(f, "support:RateCaseCommunication"),
            SupportActions::RefreshTrustedAdvisorCheck => {
                write!(f, "support:RefreshTrustedAdvisorCheck")
            }
            SupportActions::ResolveCase => write!(f, "support:ResolveCase"),
            SupportActions::SearchForCases => write!(f, "support:SearchForCases"),
            SupportActions::StartInteraction => write!(f, "support:StartInteraction"),
            SupportActions::UpdateCaseSeverity => write!(f, "support:UpdateCaseSeverity"),
            SupportActions::UpdateInteraction => write!(f, "support:UpdateInteraction"),
        }
    }
}
