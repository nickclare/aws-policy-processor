// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum ComprehendActions {
    BatchDetectDominantLanguage,
    BatchDetectEntities,
    BatchDetectKeyPhrases,
    BatchDetectSentiment,
    BatchDetectSyntax,
    BatchDetectTargetedSentiment,
    ClassifyDocument,
    ContainsPiiEntities,
    CreateDataset,
    CreateDocumentClassifier,
    CreateEndpoint,
    CreateEntityRecognizer,
    CreateFlywheel,
    DeleteDocumentClassifier,
    DeleteEndpoint,
    DeleteEntityRecognizer,
    DeleteFlywheel,
    DeleteResourcePolicy,
    DescribeDataset,
    DescribeDocumentClassificationJob,
    DescribeDocumentClassifier,
    DescribeDominantLanguageDetectionJob,
    DescribeEndpoint,
    DescribeEntitiesDetectionJob,
    DescribeEntityRecognizer,
    DescribeEventsDetectionJob,
    DescribeFlywheel,
    DescribeFlywheelIteration,
    DescribeKeyPhrasesDetectionJob,
    DescribePiiEntitiesDetectionJob,
    DescribeResourcePolicy,
    DescribeSentimentDetectionJob,
    DescribeTargetedSentimentDetectionJob,
    DescribeTopicsDetectionJob,
    DetectDominantLanguage,
    DetectEntities,
    DetectKeyPhrases,
    DetectPiiEntities,
    DetectSentiment,
    DetectSyntax,
    DetectTargetedSentiment,
    DetectToxicContent,
    ImportModel,
    ListDatasets,
    ListDocumentClassificationJobs,
    ListDocumentClassifierSummaries,
    ListDocumentClassifiers,
    ListDominantLanguageDetectionJobs,
    ListEndpoints,
    ListEntitiesDetectionJobs,
    ListEntityRecognizerSummaries,
    ListEntityRecognizers,
    ListEventsDetectionJobs,
    ListFlywheelIterationHistory,
    ListFlywheels,
    ListKeyPhrasesDetectionJobs,
    ListPiiEntitiesDetectionJobs,
    ListSentimentDetectionJobs,
    ListTagsForResource,
    ListTargetedSentimentDetectionJobs,
    ListTopicsDetectionJobs,
    PutResourcePolicy,
    StartDocumentClassificationJob,
    StartDominantLanguageDetectionJob,
    StartEntitiesDetectionJob,
    StartEventsDetectionJob,
    StartFlywheelIteration,
    StartKeyPhrasesDetectionJob,
    StartPiiEntitiesDetectionJob,
    StartSentimentDetectionJob,
    StartTargetedSentimentDetectionJob,
    StartTopicsDetectionJob,
    StopDominantLanguageDetectionJob,
    StopEntitiesDetectionJob,
    StopEventsDetectionJob,
    StopKeyPhrasesDetectionJob,
    StopPiiEntitiesDetectionJob,
    StopSentimentDetectionJob,
    StopTargetedSentimentDetectionJob,
    StopTrainingDocumentClassifier,
    StopTrainingEntityRecognizer,
    TagResource,
    UntagResource,
    UpdateEndpoint,
    UpdateFlywheel,
}
impl std::fmt::Display for ComprehendActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ComprehendActions::BatchDetectDominantLanguage => {
                write!(f, "comprehend:BatchDetectDominantLanguage")
            }
            ComprehendActions::BatchDetectEntities => write!(f, "comprehend:BatchDetectEntities"),
            ComprehendActions::BatchDetectKeyPhrases => {
                write!(f, "comprehend:BatchDetectKeyPhrases")
            }
            ComprehendActions::BatchDetectSentiment => write!(f, "comprehend:BatchDetectSentiment"),
            ComprehendActions::BatchDetectSyntax => write!(f, "comprehend:BatchDetectSyntax"),
            ComprehendActions::BatchDetectTargetedSentiment => {
                write!(f, "comprehend:BatchDetectTargetedSentiment")
            }
            ComprehendActions::ClassifyDocument => write!(f, "comprehend:ClassifyDocument"),
            ComprehendActions::ContainsPiiEntities => write!(f, "comprehend:ContainsPiiEntities"),
            ComprehendActions::CreateDataset => write!(f, "comprehend:CreateDataset"),
            ComprehendActions::CreateDocumentClassifier => {
                write!(f, "comprehend:CreateDocumentClassifier")
            }
            ComprehendActions::CreateEndpoint => write!(f, "comprehend:CreateEndpoint"),
            ComprehendActions::CreateEntityRecognizer => {
                write!(f, "comprehend:CreateEntityRecognizer")
            }
            ComprehendActions::CreateFlywheel => write!(f, "comprehend:CreateFlywheel"),
            ComprehendActions::DeleteDocumentClassifier => {
                write!(f, "comprehend:DeleteDocumentClassifier")
            }
            ComprehendActions::DeleteEndpoint => write!(f, "comprehend:DeleteEndpoint"),
            ComprehendActions::DeleteEntityRecognizer => {
                write!(f, "comprehend:DeleteEntityRecognizer")
            }
            ComprehendActions::DeleteFlywheel => write!(f, "comprehend:DeleteFlywheel"),
            ComprehendActions::DeleteResourcePolicy => write!(f, "comprehend:DeleteResourcePolicy"),
            ComprehendActions::DescribeDataset => write!(f, "comprehend:DescribeDataset"),
            ComprehendActions::DescribeDocumentClassificationJob => {
                write!(f, "comprehend:DescribeDocumentClassificationJob")
            }
            ComprehendActions::DescribeDocumentClassifier => {
                write!(f, "comprehend:DescribeDocumentClassifier")
            }
            ComprehendActions::DescribeDominantLanguageDetectionJob => {
                write!(f, "comprehend:DescribeDominantLanguageDetectionJob")
            }
            ComprehendActions::DescribeEndpoint => write!(f, "comprehend:DescribeEndpoint"),
            ComprehendActions::DescribeEntitiesDetectionJob => {
                write!(f, "comprehend:DescribeEntitiesDetectionJob")
            }
            ComprehendActions::DescribeEntityRecognizer => {
                write!(f, "comprehend:DescribeEntityRecognizer")
            }
            ComprehendActions::DescribeEventsDetectionJob => {
                write!(f, "comprehend:DescribeEventsDetectionJob")
            }
            ComprehendActions::DescribeFlywheel => write!(f, "comprehend:DescribeFlywheel"),
            ComprehendActions::DescribeFlywheelIteration => {
                write!(f, "comprehend:DescribeFlywheelIteration")
            }
            ComprehendActions::DescribeKeyPhrasesDetectionJob => {
                write!(f, "comprehend:DescribeKeyPhrasesDetectionJob")
            }
            ComprehendActions::DescribePiiEntitiesDetectionJob => {
                write!(f, "comprehend:DescribePiiEntitiesDetectionJob")
            }
            ComprehendActions::DescribeResourcePolicy => {
                write!(f, "comprehend:DescribeResourcePolicy")
            }
            ComprehendActions::DescribeSentimentDetectionJob => {
                write!(f, "comprehend:DescribeSentimentDetectionJob")
            }
            ComprehendActions::DescribeTargetedSentimentDetectionJob => {
                write!(f, "comprehend:DescribeTargetedSentimentDetectionJob")
            }
            ComprehendActions::DescribeTopicsDetectionJob => {
                write!(f, "comprehend:DescribeTopicsDetectionJob")
            }
            ComprehendActions::DetectDominantLanguage => {
                write!(f, "comprehend:DetectDominantLanguage")
            }
            ComprehendActions::DetectEntities => write!(f, "comprehend:DetectEntities"),
            ComprehendActions::DetectKeyPhrases => write!(f, "comprehend:DetectKeyPhrases"),
            ComprehendActions::DetectPiiEntities => write!(f, "comprehend:DetectPiiEntities"),
            ComprehendActions::DetectSentiment => write!(f, "comprehend:DetectSentiment"),
            ComprehendActions::DetectSyntax => write!(f, "comprehend:DetectSyntax"),
            ComprehendActions::DetectTargetedSentiment => {
                write!(f, "comprehend:DetectTargetedSentiment")
            }
            ComprehendActions::DetectToxicContent => write!(f, "comprehend:DetectToxicContent"),
            ComprehendActions::ImportModel => write!(f, "comprehend:ImportModel"),
            ComprehendActions::ListDatasets => write!(f, "comprehend:ListDatasets"),
            ComprehendActions::ListDocumentClassificationJobs => {
                write!(f, "comprehend:ListDocumentClassificationJobs")
            }
            ComprehendActions::ListDocumentClassifierSummaries => {
                write!(f, "comprehend:ListDocumentClassifierSummaries")
            }
            ComprehendActions::ListDocumentClassifiers => {
                write!(f, "comprehend:ListDocumentClassifiers")
            }
            ComprehendActions::ListDominantLanguageDetectionJobs => {
                write!(f, "comprehend:ListDominantLanguageDetectionJobs")
            }
            ComprehendActions::ListEndpoints => write!(f, "comprehend:ListEndpoints"),
            ComprehendActions::ListEntitiesDetectionJobs => {
                write!(f, "comprehend:ListEntitiesDetectionJobs")
            }
            ComprehendActions::ListEntityRecognizerSummaries => {
                write!(f, "comprehend:ListEntityRecognizerSummaries")
            }
            ComprehendActions::ListEntityRecognizers => {
                write!(f, "comprehend:ListEntityRecognizers")
            }
            ComprehendActions::ListEventsDetectionJobs => {
                write!(f, "comprehend:ListEventsDetectionJobs")
            }
            ComprehendActions::ListFlywheelIterationHistory => {
                write!(f, "comprehend:ListFlywheelIterationHistory")
            }
            ComprehendActions::ListFlywheels => write!(f, "comprehend:ListFlywheels"),
            ComprehendActions::ListKeyPhrasesDetectionJobs => {
                write!(f, "comprehend:ListKeyPhrasesDetectionJobs")
            }
            ComprehendActions::ListPiiEntitiesDetectionJobs => {
                write!(f, "comprehend:ListPiiEntitiesDetectionJobs")
            }
            ComprehendActions::ListSentimentDetectionJobs => {
                write!(f, "comprehend:ListSentimentDetectionJobs")
            }
            ComprehendActions::ListTagsForResource => write!(f, "comprehend:ListTagsForResource"),
            ComprehendActions::ListTargetedSentimentDetectionJobs => {
                write!(f, "comprehend:ListTargetedSentimentDetectionJobs")
            }
            ComprehendActions::ListTopicsDetectionJobs => {
                write!(f, "comprehend:ListTopicsDetectionJobs")
            }
            ComprehendActions::PutResourcePolicy => write!(f, "comprehend:PutResourcePolicy"),
            ComprehendActions::StartDocumentClassificationJob => {
                write!(f, "comprehend:StartDocumentClassificationJob")
            }
            ComprehendActions::StartDominantLanguageDetectionJob => {
                write!(f, "comprehend:StartDominantLanguageDetectionJob")
            }
            ComprehendActions::StartEntitiesDetectionJob => {
                write!(f, "comprehend:StartEntitiesDetectionJob")
            }
            ComprehendActions::StartEventsDetectionJob => {
                write!(f, "comprehend:StartEventsDetectionJob")
            }
            ComprehendActions::StartFlywheelIteration => {
                write!(f, "comprehend:StartFlywheelIteration")
            }
            ComprehendActions::StartKeyPhrasesDetectionJob => {
                write!(f, "comprehend:StartKeyPhrasesDetectionJob")
            }
            ComprehendActions::StartPiiEntitiesDetectionJob => {
                write!(f, "comprehend:StartPiiEntitiesDetectionJob")
            }
            ComprehendActions::StartSentimentDetectionJob => {
                write!(f, "comprehend:StartSentimentDetectionJob")
            }
            ComprehendActions::StartTargetedSentimentDetectionJob => {
                write!(f, "comprehend:StartTargetedSentimentDetectionJob")
            }
            ComprehendActions::StartTopicsDetectionJob => {
                write!(f, "comprehend:StartTopicsDetectionJob")
            }
            ComprehendActions::StopDominantLanguageDetectionJob => {
                write!(f, "comprehend:StopDominantLanguageDetectionJob")
            }
            ComprehendActions::StopEntitiesDetectionJob => {
                write!(f, "comprehend:StopEntitiesDetectionJob")
            }
            ComprehendActions::StopEventsDetectionJob => {
                write!(f, "comprehend:StopEventsDetectionJob")
            }
            ComprehendActions::StopKeyPhrasesDetectionJob => {
                write!(f, "comprehend:StopKeyPhrasesDetectionJob")
            }
            ComprehendActions::StopPiiEntitiesDetectionJob => {
                write!(f, "comprehend:StopPiiEntitiesDetectionJob")
            }
            ComprehendActions::StopSentimentDetectionJob => {
                write!(f, "comprehend:StopSentimentDetectionJob")
            }
            ComprehendActions::StopTargetedSentimentDetectionJob => {
                write!(f, "comprehend:StopTargetedSentimentDetectionJob")
            }
            ComprehendActions::StopTrainingDocumentClassifier => {
                write!(f, "comprehend:StopTrainingDocumentClassifier")
            }
            ComprehendActions::StopTrainingEntityRecognizer => {
                write!(f, "comprehend:StopTrainingEntityRecognizer")
            }
            ComprehendActions::TagResource => write!(f, "comprehend:TagResource"),
            ComprehendActions::UntagResource => write!(f, "comprehend:UntagResource"),
            ComprehendActions::UpdateEndpoint => write!(f, "comprehend:UpdateEndpoint"),
            ComprehendActions::UpdateFlywheel => write!(f, "comprehend:UpdateFlywheel"),
        }
    }
}
