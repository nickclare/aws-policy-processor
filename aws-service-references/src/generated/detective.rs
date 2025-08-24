// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum DetectiveActions {
    AcceptInvitation,
    BatchGetGraphMemberDatasources,
    BatchGetMembershipDatasources,
    CreateGraph,
    CreateMembers,
    DeleteGraph,
    DeleteMembers,
    DescribeOrganizationConfiguration,
    DisableOrganizationAdminAccount,
    DisassociateMembership,
    EnableOrganizationAdminAccount,
    GetFreeTrialEligibility,
    GetGraphIngestState,
    GetInvestigation,
    GetMembers,
    GetPricingInformation,
    GetUsageInformation,
    InvokeAssistant,
    ListDatasourcePackages,
    ListGraphs,
    ListHighDegreeEntities,
    ListIndicators,
    ListInvestigations,
    ListInvitations,
    ListMembers,
    ListOrganizationAdminAccount,
    ListTagsForResource,
    RejectInvitation,
    SearchGraph,
    StartInvestigation,
    StartMonitoringMember,
    TagResource,
    UntagResource,
    UpdateDatasourcePackages,
    UpdateInvestigationState,
    UpdateOrganizationConfiguration,
}
impl std::fmt::Display for DetectiveActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DetectiveActions::AcceptInvitation => write!(f, "detective:AcceptInvitation"),
            DetectiveActions::BatchGetGraphMemberDatasources => {
                write!(f, "detective:BatchGetGraphMemberDatasources")
            }
            DetectiveActions::BatchGetMembershipDatasources => {
                write!(f, "detective:BatchGetMembershipDatasources")
            }
            DetectiveActions::CreateGraph => write!(f, "detective:CreateGraph"),
            DetectiveActions::CreateMembers => write!(f, "detective:CreateMembers"),
            DetectiveActions::DeleteGraph => write!(f, "detective:DeleteGraph"),
            DetectiveActions::DeleteMembers => write!(f, "detective:DeleteMembers"),
            DetectiveActions::DescribeOrganizationConfiguration => {
                write!(f, "detective:DescribeOrganizationConfiguration")
            }
            DetectiveActions::DisableOrganizationAdminAccount => {
                write!(f, "detective:DisableOrganizationAdminAccount")
            }
            DetectiveActions::DisassociateMembership => {
                write!(f, "detective:DisassociateMembership")
            }
            DetectiveActions::EnableOrganizationAdminAccount => {
                write!(f, "detective:EnableOrganizationAdminAccount")
            }
            DetectiveActions::GetFreeTrialEligibility => {
                write!(f, "detective:GetFreeTrialEligibility")
            }
            DetectiveActions::GetGraphIngestState => write!(f, "detective:GetGraphIngestState"),
            DetectiveActions::GetInvestigation => write!(f, "detective:GetInvestigation"),
            DetectiveActions::GetMembers => write!(f, "detective:GetMembers"),
            DetectiveActions::GetPricingInformation => write!(f, "detective:GetPricingInformation"),
            DetectiveActions::GetUsageInformation => write!(f, "detective:GetUsageInformation"),
            DetectiveActions::InvokeAssistant => write!(f, "detective:InvokeAssistant"),
            DetectiveActions::ListDatasourcePackages => {
                write!(f, "detective:ListDatasourcePackages")
            }
            DetectiveActions::ListGraphs => write!(f, "detective:ListGraphs"),
            DetectiveActions::ListHighDegreeEntities => {
                write!(f, "detective:ListHighDegreeEntities")
            }
            DetectiveActions::ListIndicators => write!(f, "detective:ListIndicators"),
            DetectiveActions::ListInvestigations => write!(f, "detective:ListInvestigations"),
            DetectiveActions::ListInvitations => write!(f, "detective:ListInvitations"),
            DetectiveActions::ListMembers => write!(f, "detective:ListMembers"),
            DetectiveActions::ListOrganizationAdminAccount => {
                write!(f, "detective:ListOrganizationAdminAccount")
            }
            DetectiveActions::ListTagsForResource => write!(f, "detective:ListTagsForResource"),
            DetectiveActions::RejectInvitation => write!(f, "detective:RejectInvitation"),
            DetectiveActions::SearchGraph => write!(f, "detective:SearchGraph"),
            DetectiveActions::StartInvestigation => write!(f, "detective:StartInvestigation"),
            DetectiveActions::StartMonitoringMember => write!(f, "detective:StartMonitoringMember"),
            DetectiveActions::TagResource => write!(f, "detective:TagResource"),
            DetectiveActions::UntagResource => write!(f, "detective:UntagResource"),
            DetectiveActions::UpdateDatasourcePackages => {
                write!(f, "detective:UpdateDatasourcePackages")
            }
            DetectiveActions::UpdateInvestigationState => {
                write!(f, "detective:UpdateInvestigationState")
            }
            DetectiveActions::UpdateOrganizationConfiguration => {
                write!(f, "detective:UpdateOrganizationConfiguration")
            }
        }
    }
}
