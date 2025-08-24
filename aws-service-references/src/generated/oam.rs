// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum OamActions {
    CreateLink,
    CreateSink,
    DeleteLink,
    DeleteSink,
    GetLink,
    GetSink,
    GetSinkPolicy,
    ListAttachedLinks,
    ListLinks,
    ListSinks,
    ListTagsForResource,
    PutSinkPolicy,
    TagResource,
    UntagResource,
    UpdateLink,
}
impl std::fmt::Display for OamActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OamActions::CreateLink => write!(f, "oam:CreateLink"),
            OamActions::CreateSink => write!(f, "oam:CreateSink"),
            OamActions::DeleteLink => write!(f, "oam:DeleteLink"),
            OamActions::DeleteSink => write!(f, "oam:DeleteSink"),
            OamActions::GetLink => write!(f, "oam:GetLink"),
            OamActions::GetSink => write!(f, "oam:GetSink"),
            OamActions::GetSinkPolicy => write!(f, "oam:GetSinkPolicy"),
            OamActions::ListAttachedLinks => write!(f, "oam:ListAttachedLinks"),
            OamActions::ListLinks => write!(f, "oam:ListLinks"),
            OamActions::ListSinks => write!(f, "oam:ListSinks"),
            OamActions::ListTagsForResource => write!(f, "oam:ListTagsForResource"),
            OamActions::PutSinkPolicy => write!(f, "oam:PutSinkPolicy"),
            OamActions::TagResource => write!(f, "oam:TagResource"),
            OamActions::UntagResource => write!(f, "oam:UntagResource"),
            OamActions::UpdateLink => write!(f, "oam:UpdateLink"),
        }
    }
}
