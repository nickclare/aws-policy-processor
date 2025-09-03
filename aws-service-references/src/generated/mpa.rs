// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum MpaActions {
    CancelSession,
    CreateApprovalTeam,
    CreateIdentitySource,
    DeleteIdentitySource,
    DeleteInactiveApprovalTeamVersion,
    DeleteResourcePolicy,
    GetApprovalTeam,
    GetIdentitySource,
    GetPolicyVersion,
    GetResourcePolicy,
    GetSession,
    ListApprovalTeams,
    ListIdentitySources,
    ListPolicies,
    ListPolicyVersions,
    ListResourcePolicies,
    ListSessions,
    ListTagsForResource,
    PutResourcePolicy,
    StartActiveApprovalTeamDeletion,
    StartSession,
    TagResource,
    UntagResource,
    UpdateApprovalTeam,
}
impl std::fmt::Display for MpaActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MpaActions::CancelSession => write!(f, "mpa:CancelSession"),
            MpaActions::CreateApprovalTeam => write!(f, "mpa:CreateApprovalTeam"),
            MpaActions::CreateIdentitySource => write!(f, "mpa:CreateIdentitySource"),
            MpaActions::DeleteIdentitySource => write!(f, "mpa:DeleteIdentitySource"),
            MpaActions::DeleteInactiveApprovalTeamVersion => {
                write!(f, "mpa:DeleteInactiveApprovalTeamVersion")
            }
            MpaActions::DeleteResourcePolicy => write!(f, "mpa:DeleteResourcePolicy"),
            MpaActions::GetApprovalTeam => write!(f, "mpa:GetApprovalTeam"),
            MpaActions::GetIdentitySource => write!(f, "mpa:GetIdentitySource"),
            MpaActions::GetPolicyVersion => write!(f, "mpa:GetPolicyVersion"),
            MpaActions::GetResourcePolicy => write!(f, "mpa:GetResourcePolicy"),
            MpaActions::GetSession => write!(f, "mpa:GetSession"),
            MpaActions::ListApprovalTeams => write!(f, "mpa:ListApprovalTeams"),
            MpaActions::ListIdentitySources => write!(f, "mpa:ListIdentitySources"),
            MpaActions::ListPolicies => write!(f, "mpa:ListPolicies"),
            MpaActions::ListPolicyVersions => write!(f, "mpa:ListPolicyVersions"),
            MpaActions::ListResourcePolicies => write!(f, "mpa:ListResourcePolicies"),
            MpaActions::ListSessions => write!(f, "mpa:ListSessions"),
            MpaActions::ListTagsForResource => write!(f, "mpa:ListTagsForResource"),
            MpaActions::PutResourcePolicy => write!(f, "mpa:PutResourcePolicy"),
            MpaActions::StartActiveApprovalTeamDeletion => {
                write!(f, "mpa:StartActiveApprovalTeamDeletion")
            }
            MpaActions::StartSession => write!(f, "mpa:StartSession"),
            MpaActions::TagResource => write!(f, "mpa:TagResource"),
            MpaActions::UntagResource => write!(f, "mpa:UntagResource"),
            MpaActions::UpdateApprovalTeam => write!(f, "mpa:UpdateApprovalTeam"),
        }
    }
}
