// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum TranscribeActions {
    CreateCallAnalyticsCategory,
    CreateLanguageModel,
    CreateMedicalVocabulary,
    CreateVocabulary,
    CreateVocabularyFilter,
    DeleteCallAnalyticsCategory,
    DeleteCallAnalyticsJob,
    DeleteLanguageModel,
    DeleteMedicalScribeJob,
    DeleteMedicalTranscriptionJob,
    DeleteMedicalVocabulary,
    DeleteTranscriptionJob,
    DeleteVocabulary,
    DeleteVocabularyFilter,
    DescribeLanguageModel,
    GetCallAnalyticsCategory,
    GetCallAnalyticsJob,
    GetMedicalScribeJob,
    GetMedicalScribeStream,
    GetMedicalTranscriptionJob,
    GetMedicalVocabulary,
    GetTranscriptionJob,
    GetVocabulary,
    GetVocabularyFilter,
    ListCallAnalyticsCategories,
    ListCallAnalyticsJobs,
    ListLanguageModels,
    ListMedicalScribeJobs,
    ListMedicalTranscriptionJobs,
    ListMedicalVocabularies,
    ListTagsForResource,
    ListTranscriptionJobs,
    ListVocabularies,
    ListVocabularyFilters,
    StartCallAnalyticsJob,
    StartCallAnalyticsStreamTranscription,
    StartCallAnalyticsStreamTranscriptionWebSocket,
    StartMedicalScribeJob,
    StartMedicalScribeStream,
    StartMedicalStreamTranscription,
    StartMedicalStreamTranscriptionWebSocket,
    StartMedicalTranscriptionJob,
    StartStreamTranscription,
    StartStreamTranscriptionWebSocket,
    StartTranscriptionJob,
    TagResource,
    UntagResource,
    UpdateCallAnalyticsCategory,
    UpdateMedicalVocabulary,
    UpdateVocabulary,
    UpdateVocabularyFilter,
}
impl std::fmt::Display for TranscribeActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TranscribeActions::CreateCallAnalyticsCategory => {
                write!(f, "transcribe:CreateCallAnalyticsCategory")
            }
            TranscribeActions::CreateLanguageModel => write!(f, "transcribe:CreateLanguageModel"),
            TranscribeActions::CreateMedicalVocabulary => {
                write!(f, "transcribe:CreateMedicalVocabulary")
            }
            TranscribeActions::CreateVocabulary => write!(f, "transcribe:CreateVocabulary"),
            TranscribeActions::CreateVocabularyFilter => {
                write!(f, "transcribe:CreateVocabularyFilter")
            }
            TranscribeActions::DeleteCallAnalyticsCategory => {
                write!(f, "transcribe:DeleteCallAnalyticsCategory")
            }
            TranscribeActions::DeleteCallAnalyticsJob => {
                write!(f, "transcribe:DeleteCallAnalyticsJob")
            }
            TranscribeActions::DeleteLanguageModel => write!(f, "transcribe:DeleteLanguageModel"),
            TranscribeActions::DeleteMedicalScribeJob => {
                write!(f, "transcribe:DeleteMedicalScribeJob")
            }
            TranscribeActions::DeleteMedicalTranscriptionJob => {
                write!(f, "transcribe:DeleteMedicalTranscriptionJob")
            }
            TranscribeActions::DeleteMedicalVocabulary => {
                write!(f, "transcribe:DeleteMedicalVocabulary")
            }
            TranscribeActions::DeleteTranscriptionJob => {
                write!(f, "transcribe:DeleteTranscriptionJob")
            }
            TranscribeActions::DeleteVocabulary => write!(f, "transcribe:DeleteVocabulary"),
            TranscribeActions::DeleteVocabularyFilter => {
                write!(f, "transcribe:DeleteVocabularyFilter")
            }
            TranscribeActions::DescribeLanguageModel => {
                write!(f, "transcribe:DescribeLanguageModel")
            }
            TranscribeActions::GetCallAnalyticsCategory => {
                write!(f, "transcribe:GetCallAnalyticsCategory")
            }
            TranscribeActions::GetCallAnalyticsJob => write!(f, "transcribe:GetCallAnalyticsJob"),
            TranscribeActions::GetMedicalScribeJob => write!(f, "transcribe:GetMedicalScribeJob"),
            TranscribeActions::GetMedicalScribeStream => {
                write!(f, "transcribe:GetMedicalScribeStream")
            }
            TranscribeActions::GetMedicalTranscriptionJob => {
                write!(f, "transcribe:GetMedicalTranscriptionJob")
            }
            TranscribeActions::GetMedicalVocabulary => write!(f, "transcribe:GetMedicalVocabulary"),
            TranscribeActions::GetTranscriptionJob => write!(f, "transcribe:GetTranscriptionJob"),
            TranscribeActions::GetVocabulary => write!(f, "transcribe:GetVocabulary"),
            TranscribeActions::GetVocabularyFilter => write!(f, "transcribe:GetVocabularyFilter"),
            TranscribeActions::ListCallAnalyticsCategories => {
                write!(f, "transcribe:ListCallAnalyticsCategories")
            }
            TranscribeActions::ListCallAnalyticsJobs => {
                write!(f, "transcribe:ListCallAnalyticsJobs")
            }
            TranscribeActions::ListLanguageModels => write!(f, "transcribe:ListLanguageModels"),
            TranscribeActions::ListMedicalScribeJobs => {
                write!(f, "transcribe:ListMedicalScribeJobs")
            }
            TranscribeActions::ListMedicalTranscriptionJobs => {
                write!(f, "transcribe:ListMedicalTranscriptionJobs")
            }
            TranscribeActions::ListMedicalVocabularies => {
                write!(f, "transcribe:ListMedicalVocabularies")
            }
            TranscribeActions::ListTagsForResource => write!(f, "transcribe:ListTagsForResource"),
            TranscribeActions::ListTranscriptionJobs => {
                write!(f, "transcribe:ListTranscriptionJobs")
            }
            TranscribeActions::ListVocabularies => write!(f, "transcribe:ListVocabularies"),
            TranscribeActions::ListVocabularyFilters => {
                write!(f, "transcribe:ListVocabularyFilters")
            }
            TranscribeActions::StartCallAnalyticsJob => {
                write!(f, "transcribe:StartCallAnalyticsJob")
            }
            TranscribeActions::StartCallAnalyticsStreamTranscription => {
                write!(f, "transcribe:StartCallAnalyticsStreamTranscription")
            }
            TranscribeActions::StartCallAnalyticsStreamTranscriptionWebSocket => write!(
                f,
                "transcribe:StartCallAnalyticsStreamTranscriptionWebSocket"
            ),
            TranscribeActions::StartMedicalScribeJob => {
                write!(f, "transcribe:StartMedicalScribeJob")
            }
            TranscribeActions::StartMedicalScribeStream => {
                write!(f, "transcribe:StartMedicalScribeStream")
            }
            TranscribeActions::StartMedicalStreamTranscription => {
                write!(f, "transcribe:StartMedicalStreamTranscription")
            }
            TranscribeActions::StartMedicalStreamTranscriptionWebSocket => {
                write!(f, "transcribe:StartMedicalStreamTranscriptionWebSocket")
            }
            TranscribeActions::StartMedicalTranscriptionJob => {
                write!(f, "transcribe:StartMedicalTranscriptionJob")
            }
            TranscribeActions::StartStreamTranscription => {
                write!(f, "transcribe:StartStreamTranscription")
            }
            TranscribeActions::StartStreamTranscriptionWebSocket => {
                write!(f, "transcribe:StartStreamTranscriptionWebSocket")
            }
            TranscribeActions::StartTranscriptionJob => {
                write!(f, "transcribe:StartTranscriptionJob")
            }
            TranscribeActions::TagResource => write!(f, "transcribe:TagResource"),
            TranscribeActions::UntagResource => write!(f, "transcribe:UntagResource"),
            TranscribeActions::UpdateCallAnalyticsCategory => {
                write!(f, "transcribe:UpdateCallAnalyticsCategory")
            }
            TranscribeActions::UpdateMedicalVocabulary => {
                write!(f, "transcribe:UpdateMedicalVocabulary")
            }
            TranscribeActions::UpdateVocabulary => write!(f, "transcribe:UpdateVocabulary"),
            TranscribeActions::UpdateVocabularyFilter => {
                write!(f, "transcribe:UpdateVocabularyFilter")
            }
        }
    }
}
