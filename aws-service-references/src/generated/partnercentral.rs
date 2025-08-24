// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum PartnercentralActions {
    AcceptEngagementInvitation,
    AssignOpportunity,
    AssociateOpportunity,
    CreateEngagement,
    CreateEngagementInvitation,
    CreateOpportunity,
    CreateResourceSnapshot,
    CreateResourceSnapshotJob,
    DeleteResourceSnapshotJob,
    DisassociateOpportunity,
    GetAwsOpportunitySummary,
    GetEngagement,
    GetEngagementInvitation,
    GetOpportunity,
    GetResourceSnapshot,
    GetResourceSnapshotJob,
    GetSellingSystemSettings,
    ListEngagementByAcceptingInvitationTasks,
    ListEngagementFromOpportunityTasks,
    ListEngagementInvitations,
    ListEngagementMembers,
    ListEngagementResourceAssociations,
    ListEngagements,
    ListOpportunities,
    ListResourceSnapshotJobs,
    ListResourceSnapshots,
    ListSolutions,
    ListTagsForResource,
    PutSellingSystemSettings,
    RejectEngagementInvitation,
    StartEngagementByAcceptingInvitationTask,
    StartEngagementFromOpportunityTask,
    StartResourceSnapshotJob,
    StopResourceSnapshotJob,
    SubmitOpportunity,
    TagResource,
    UntagResource,
    UpdateOpportunity,
}
impl std::fmt::Display for PartnercentralActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PartnercentralActions::AcceptEngagementInvitation => {
                write!(f, "partnercentral:AcceptEngagementInvitation")
            }
            PartnercentralActions::AssignOpportunity => {
                write!(f, "partnercentral:AssignOpportunity")
            }
            PartnercentralActions::AssociateOpportunity => {
                write!(f, "partnercentral:AssociateOpportunity")
            }
            PartnercentralActions::CreateEngagement => write!(f, "partnercentral:CreateEngagement"),
            PartnercentralActions::CreateEngagementInvitation => {
                write!(f, "partnercentral:CreateEngagementInvitation")
            }
            PartnercentralActions::CreateOpportunity => {
                write!(f, "partnercentral:CreateOpportunity")
            }
            PartnercentralActions::CreateResourceSnapshot => {
                write!(f, "partnercentral:CreateResourceSnapshot")
            }
            PartnercentralActions::CreateResourceSnapshotJob => {
                write!(f, "partnercentral:CreateResourceSnapshotJob")
            }
            PartnercentralActions::DeleteResourceSnapshotJob => {
                write!(f, "partnercentral:DeleteResourceSnapshotJob")
            }
            PartnercentralActions::DisassociateOpportunity => {
                write!(f, "partnercentral:DisassociateOpportunity")
            }
            PartnercentralActions::GetAwsOpportunitySummary => {
                write!(f, "partnercentral:GetAwsOpportunitySummary")
            }
            PartnercentralActions::GetEngagement => write!(f, "partnercentral:GetEngagement"),
            PartnercentralActions::GetEngagementInvitation => {
                write!(f, "partnercentral:GetEngagementInvitation")
            }
            PartnercentralActions::GetOpportunity => write!(f, "partnercentral:GetOpportunity"),
            PartnercentralActions::GetResourceSnapshot => {
                write!(f, "partnercentral:GetResourceSnapshot")
            }
            PartnercentralActions::GetResourceSnapshotJob => {
                write!(f, "partnercentral:GetResourceSnapshotJob")
            }
            PartnercentralActions::GetSellingSystemSettings => {
                write!(f, "partnercentral:GetSellingSystemSettings")
            }
            PartnercentralActions::ListEngagementByAcceptingInvitationTasks => {
                write!(f, "partnercentral:ListEngagementByAcceptingInvitationTasks")
            }
            PartnercentralActions::ListEngagementFromOpportunityTasks => {
                write!(f, "partnercentral:ListEngagementFromOpportunityTasks")
            }
            PartnercentralActions::ListEngagementInvitations => {
                write!(f, "partnercentral:ListEngagementInvitations")
            }
            PartnercentralActions::ListEngagementMembers => {
                write!(f, "partnercentral:ListEngagementMembers")
            }
            PartnercentralActions::ListEngagementResourceAssociations => {
                write!(f, "partnercentral:ListEngagementResourceAssociations")
            }
            PartnercentralActions::ListEngagements => write!(f, "partnercentral:ListEngagements"),
            PartnercentralActions::ListOpportunities => {
                write!(f, "partnercentral:ListOpportunities")
            }
            PartnercentralActions::ListResourceSnapshotJobs => {
                write!(f, "partnercentral:ListResourceSnapshotJobs")
            }
            PartnercentralActions::ListResourceSnapshots => {
                write!(f, "partnercentral:ListResourceSnapshots")
            }
            PartnercentralActions::ListSolutions => write!(f, "partnercentral:ListSolutions"),
            PartnercentralActions::ListTagsForResource => {
                write!(f, "partnercentral:ListTagsForResource")
            }
            PartnercentralActions::PutSellingSystemSettings => {
                write!(f, "partnercentral:PutSellingSystemSettings")
            }
            PartnercentralActions::RejectEngagementInvitation => {
                write!(f, "partnercentral:RejectEngagementInvitation")
            }
            PartnercentralActions::StartEngagementByAcceptingInvitationTask => {
                write!(f, "partnercentral:StartEngagementByAcceptingInvitationTask")
            }
            PartnercentralActions::StartEngagementFromOpportunityTask => {
                write!(f, "partnercentral:StartEngagementFromOpportunityTask")
            }
            PartnercentralActions::StartResourceSnapshotJob => {
                write!(f, "partnercentral:StartResourceSnapshotJob")
            }
            PartnercentralActions::StopResourceSnapshotJob => {
                write!(f, "partnercentral:StopResourceSnapshotJob")
            }
            PartnercentralActions::SubmitOpportunity => {
                write!(f, "partnercentral:SubmitOpportunity")
            }
            PartnercentralActions::TagResource => write!(f, "partnercentral:TagResource"),
            PartnercentralActions::UntagResource => write!(f, "partnercentral:UntagResource"),
            PartnercentralActions::UpdateOpportunity => {
                write!(f, "partnercentral:UpdateOpportunity")
            }
        }
    }
}
