// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum DeepcomposerActions {
    AssociateCoupon,
    CreateAudio,
    CreateComposition,
    CreateModel,
    DeleteComposition,
    DeleteModel,
    GetComposition,
    GetModel,
    GetSampleModel,
    ListCompositions,
    ListModels,
    ListSampleModels,
    ListTagsForResource,
    ListTrainingTopics,
    TagResource,
    UntagResource,
    UpdateComposition,
    UpdateModel,
}
impl std::fmt::Display for DeepcomposerActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeepcomposerActions::AssociateCoupon => write!(f, "deepcomposer:AssociateCoupon"),
            DeepcomposerActions::CreateAudio => write!(f, "deepcomposer:CreateAudio"),
            DeepcomposerActions::CreateComposition => write!(f, "deepcomposer:CreateComposition"),
            DeepcomposerActions::CreateModel => write!(f, "deepcomposer:CreateModel"),
            DeepcomposerActions::DeleteComposition => write!(f, "deepcomposer:DeleteComposition"),
            DeepcomposerActions::DeleteModel => write!(f, "deepcomposer:DeleteModel"),
            DeepcomposerActions::GetComposition => write!(f, "deepcomposer:GetComposition"),
            DeepcomposerActions::GetModel => write!(f, "deepcomposer:GetModel"),
            DeepcomposerActions::GetSampleModel => write!(f, "deepcomposer:GetSampleModel"),
            DeepcomposerActions::ListCompositions => write!(f, "deepcomposer:ListCompositions"),
            DeepcomposerActions::ListModels => write!(f, "deepcomposer:ListModels"),
            DeepcomposerActions::ListSampleModels => write!(f, "deepcomposer:ListSampleModels"),
            DeepcomposerActions::ListTagsForResource => {
                write!(f, "deepcomposer:ListTagsForResource")
            }
            DeepcomposerActions::ListTrainingTopics => write!(f, "deepcomposer:ListTrainingTopics"),
            DeepcomposerActions::TagResource => write!(f, "deepcomposer:TagResource"),
            DeepcomposerActions::UntagResource => write!(f, "deepcomposer:UntagResource"),
            DeepcomposerActions::UpdateComposition => write!(f, "deepcomposer:UpdateComposition"),
            DeepcomposerActions::UpdateModel => write!(f, "deepcomposer:UpdateModel"),
        }
    }
}
