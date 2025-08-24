// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum TranslateActions {
    CreateParallelData,
    DeleteParallelData,
    DeleteTerminology,
    DescribeTextTranslationJob,
    GetParallelData,
    GetTerminology,
    ImportTerminology,
    ListLanguages,
    ListParallelData,
    ListTagsForResource,
    ListTerminologies,
    ListTextTranslationJobs,
    StartTextTranslationJob,
    StopTextTranslationJob,
    TagResource,
    TranslateDocument,
    TranslateText,
    UntagResource,
    UpdateParallelData,
}
impl std::fmt::Display for TranslateActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TranslateActions::CreateParallelData => write!(f, "translate:CreateParallelData"),
            TranslateActions::DeleteParallelData => write!(f, "translate:DeleteParallelData"),
            TranslateActions::DeleteTerminology => write!(f, "translate:DeleteTerminology"),
            TranslateActions::DescribeTextTranslationJob => {
                write!(f, "translate:DescribeTextTranslationJob")
            }
            TranslateActions::GetParallelData => write!(f, "translate:GetParallelData"),
            TranslateActions::GetTerminology => write!(f, "translate:GetTerminology"),
            TranslateActions::ImportTerminology => write!(f, "translate:ImportTerminology"),
            TranslateActions::ListLanguages => write!(f, "translate:ListLanguages"),
            TranslateActions::ListParallelData => write!(f, "translate:ListParallelData"),
            TranslateActions::ListTagsForResource => write!(f, "translate:ListTagsForResource"),
            TranslateActions::ListTerminologies => write!(f, "translate:ListTerminologies"),
            TranslateActions::ListTextTranslationJobs => {
                write!(f, "translate:ListTextTranslationJobs")
            }
            TranslateActions::StartTextTranslationJob => {
                write!(f, "translate:StartTextTranslationJob")
            }
            TranslateActions::StopTextTranslationJob => {
                write!(f, "translate:StopTextTranslationJob")
            }
            TranslateActions::TagResource => write!(f, "translate:TagResource"),
            TranslateActions::TranslateDocument => write!(f, "translate:TranslateDocument"),
            TranslateActions::TranslateText => write!(f, "translate:TranslateText"),
            TranslateActions::UntagResource => write!(f, "translate:UntagResource"),
            TranslateActions::UpdateParallelData => write!(f, "translate:UpdateParallelData"),
        }
    }
}
