// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum AiopsActions {
    CreateInvestigation,
    CreateInvestigationEvent,
    CreateInvestigationGroup,
    CreateInvestigationResource,
    DeleteInvestigation,
    DeleteInvestigationGroup,
    DeleteInvestigationGroupPolicy,
    GetInvestigation,
    GetInvestigationEvent,
    GetInvestigationGroup,
    GetInvestigationGroupPolicy,
    GetInvestigationResource,
    ListInvestigationEvents,
    ListInvestigationGroups,
    ListInvestigations,
    ListTagsForResource,
    PutInvestigationGroupPolicy,
    TagResource,
    UntagResource,
    UpdateInvestigation,
    UpdateInvestigationEvent,
    UpdateInvestigationGroup,
    ValidateInvestigationGroup,
}
impl std::fmt::Display for AiopsActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AiopsActions::CreateInvestigation => write!(f, "aiops:CreateInvestigation"),
            AiopsActions::CreateInvestigationEvent => write!(f, "aiops:CreateInvestigationEvent"),
            AiopsActions::CreateInvestigationGroup => write!(f, "aiops:CreateInvestigationGroup"),
            AiopsActions::CreateInvestigationResource => {
                write!(f, "aiops:CreateInvestigationResource")
            }
            AiopsActions::DeleteInvestigation => write!(f, "aiops:DeleteInvestigation"),
            AiopsActions::DeleteInvestigationGroup => write!(f, "aiops:DeleteInvestigationGroup"),
            AiopsActions::DeleteInvestigationGroupPolicy => {
                write!(f, "aiops:DeleteInvestigationGroupPolicy")
            }
            AiopsActions::GetInvestigation => write!(f, "aiops:GetInvestigation"),
            AiopsActions::GetInvestigationEvent => write!(f, "aiops:GetInvestigationEvent"),
            AiopsActions::GetInvestigationGroup => write!(f, "aiops:GetInvestigationGroup"),
            AiopsActions::GetInvestigationGroupPolicy => {
                write!(f, "aiops:GetInvestigationGroupPolicy")
            }
            AiopsActions::GetInvestigationResource => write!(f, "aiops:GetInvestigationResource"),
            AiopsActions::ListInvestigationEvents => write!(f, "aiops:ListInvestigationEvents"),
            AiopsActions::ListInvestigationGroups => write!(f, "aiops:ListInvestigationGroups"),
            AiopsActions::ListInvestigations => write!(f, "aiops:ListInvestigations"),
            AiopsActions::ListTagsForResource => write!(f, "aiops:ListTagsForResource"),
            AiopsActions::PutInvestigationGroupPolicy => {
                write!(f, "aiops:PutInvestigationGroupPolicy")
            }
            AiopsActions::TagResource => write!(f, "aiops:TagResource"),
            AiopsActions::UntagResource => write!(f, "aiops:UntagResource"),
            AiopsActions::UpdateInvestigation => write!(f, "aiops:UpdateInvestigation"),
            AiopsActions::UpdateInvestigationEvent => write!(f, "aiops:UpdateInvestigationEvent"),
            AiopsActions::UpdateInvestigationGroup => write!(f, "aiops:UpdateInvestigationGroup"),
            AiopsActions::ValidateInvestigationGroup => {
                write!(f, "aiops:ValidateInvestigationGroup")
            }
        }
    }
}
