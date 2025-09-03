// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum PipesActions {
    CreatePipe,
    DeletePipe,
    DescribePipe,
    ListPipes,
    ListTagsForResource,
    StartPipe,
    StopPipe,
    TagResource,
    UntagResource,
    UpdatePipe,
}
impl std::fmt::Display for PipesActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PipesActions::CreatePipe => write!(f, "pipes:CreatePipe"),
            PipesActions::DeletePipe => write!(f, "pipes:DeletePipe"),
            PipesActions::DescribePipe => write!(f, "pipes:DescribePipe"),
            PipesActions::ListPipes => write!(f, "pipes:ListPipes"),
            PipesActions::ListTagsForResource => write!(f, "pipes:ListTagsForResource"),
            PipesActions::StartPipe => write!(f, "pipes:StartPipe"),
            PipesActions::StopPipe => write!(f, "pipes:StopPipe"),
            PipesActions::TagResource => write!(f, "pipes:TagResource"),
            PipesActions::UntagResource => write!(f, "pipes:UntagResource"),
            PipesActions::UpdatePipe => write!(f, "pipes:UpdatePipe"),
        }
    }
}
