// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum PersonalizeActions {
    CreateBatchInferenceJob,
    CreateBatchSegmentJob,
    CreateCampaign,
    CreateDataDeletionJob,
    CreateDataInsightsJob,
    CreateDataset,
    CreateDatasetExportJob,
    CreateDatasetGroup,
    CreateDatasetImportJob,
    CreateEventTracker,
    CreateFilter,
    CreateMetricAttribution,
    CreateRecommender,
    CreateSchema,
    CreateSolution,
    CreateSolutionVersion,
    DeleteCampaign,
    DeleteDataset,
    DeleteDatasetGroup,
    DeleteEventTracker,
    DeleteFilter,
    DeleteMetricAttribution,
    DeleteRecommender,
    DeleteSchema,
    DeleteSolution,
    DescribeAlgorithm,
    DescribeBatchInferenceJob,
    DescribeBatchSegmentJob,
    DescribeCampaign,
    DescribeDataDeletionJob,
    DescribeDataInsightsJob,
    DescribeDataset,
    DescribeDatasetExportJob,
    DescribeDatasetGroup,
    DescribeDatasetImportJob,
    DescribeEventTracker,
    DescribeFeatureTransformation,
    DescribeFilter,
    DescribeMetricAttribution,
    DescribeRecipe,
    DescribeRecommender,
    DescribeSchema,
    DescribeSolution,
    DescribeSolutionVersion,
    GetActionRecommendations,
    GetDataInsights,
    GetPersonalizedRanking,
    GetRecommendations,
    GetSolutionMetrics,
    ListBatchInferenceJobs,
    ListBatchSegmentJobs,
    ListCampaigns,
    ListDataDeletionJobs,
    ListDataInsightsJobs,
    ListDatasetExportJobs,
    ListDatasetGroups,
    ListDatasetImportJobs,
    ListDatasets,
    ListEventTrackers,
    ListFilters,
    ListMetricAttributionMetrics,
    ListMetricAttributions,
    ListRecipes,
    ListRecommenders,
    ListSchemas,
    ListSolutionVersions,
    ListSolutions,
    ListTagsForResource,
    PutActionInteractions,
    PutActions,
    PutEvents,
    PutItems,
    PutUsers,
    StartRecommender,
    StopRecommender,
    StopSolutionVersionCreation,
    TagResource,
    UntagResource,
    UpdateCampaign,
    UpdateDataset,
    UpdateMetricAttribution,
    UpdateRecommender,
    UpdateSolution,
}
impl std::fmt::Display for PersonalizeActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PersonalizeActions::CreateBatchInferenceJob => {
                write!(f, "personalize:CreateBatchInferenceJob")
            }
            PersonalizeActions::CreateBatchSegmentJob => {
                write!(f, "personalize:CreateBatchSegmentJob")
            }
            PersonalizeActions::CreateCampaign => write!(f, "personalize:CreateCampaign"),
            PersonalizeActions::CreateDataDeletionJob => {
                write!(f, "personalize:CreateDataDeletionJob")
            }
            PersonalizeActions::CreateDataInsightsJob => {
                write!(f, "personalize:CreateDataInsightsJob")
            }
            PersonalizeActions::CreateDataset => write!(f, "personalize:CreateDataset"),
            PersonalizeActions::CreateDatasetExportJob => {
                write!(f, "personalize:CreateDatasetExportJob")
            }
            PersonalizeActions::CreateDatasetGroup => write!(f, "personalize:CreateDatasetGroup"),
            PersonalizeActions::CreateDatasetImportJob => {
                write!(f, "personalize:CreateDatasetImportJob")
            }
            PersonalizeActions::CreateEventTracker => write!(f, "personalize:CreateEventTracker"),
            PersonalizeActions::CreateFilter => write!(f, "personalize:CreateFilter"),
            PersonalizeActions::CreateMetricAttribution => {
                write!(f, "personalize:CreateMetricAttribution")
            }
            PersonalizeActions::CreateRecommender => write!(f, "personalize:CreateRecommender"),
            PersonalizeActions::CreateSchema => write!(f, "personalize:CreateSchema"),
            PersonalizeActions::CreateSolution => write!(f, "personalize:CreateSolution"),
            PersonalizeActions::CreateSolutionVersion => {
                write!(f, "personalize:CreateSolutionVersion")
            }
            PersonalizeActions::DeleteCampaign => write!(f, "personalize:DeleteCampaign"),
            PersonalizeActions::DeleteDataset => write!(f, "personalize:DeleteDataset"),
            PersonalizeActions::DeleteDatasetGroup => write!(f, "personalize:DeleteDatasetGroup"),
            PersonalizeActions::DeleteEventTracker => write!(f, "personalize:DeleteEventTracker"),
            PersonalizeActions::DeleteFilter => write!(f, "personalize:DeleteFilter"),
            PersonalizeActions::DeleteMetricAttribution => {
                write!(f, "personalize:DeleteMetricAttribution")
            }
            PersonalizeActions::DeleteRecommender => write!(f, "personalize:DeleteRecommender"),
            PersonalizeActions::DeleteSchema => write!(f, "personalize:DeleteSchema"),
            PersonalizeActions::DeleteSolution => write!(f, "personalize:DeleteSolution"),
            PersonalizeActions::DescribeAlgorithm => write!(f, "personalize:DescribeAlgorithm"),
            PersonalizeActions::DescribeBatchInferenceJob => {
                write!(f, "personalize:DescribeBatchInferenceJob")
            }
            PersonalizeActions::DescribeBatchSegmentJob => {
                write!(f, "personalize:DescribeBatchSegmentJob")
            }
            PersonalizeActions::DescribeCampaign => write!(f, "personalize:DescribeCampaign"),
            PersonalizeActions::DescribeDataDeletionJob => {
                write!(f, "personalize:DescribeDataDeletionJob")
            }
            PersonalizeActions::DescribeDataInsightsJob => {
                write!(f, "personalize:DescribeDataInsightsJob")
            }
            PersonalizeActions::DescribeDataset => write!(f, "personalize:DescribeDataset"),
            PersonalizeActions::DescribeDatasetExportJob => {
                write!(f, "personalize:DescribeDatasetExportJob")
            }
            PersonalizeActions::DescribeDatasetGroup => {
                write!(f, "personalize:DescribeDatasetGroup")
            }
            PersonalizeActions::DescribeDatasetImportJob => {
                write!(f, "personalize:DescribeDatasetImportJob")
            }
            PersonalizeActions::DescribeEventTracker => {
                write!(f, "personalize:DescribeEventTracker")
            }
            PersonalizeActions::DescribeFeatureTransformation => {
                write!(f, "personalize:DescribeFeatureTransformation")
            }
            PersonalizeActions::DescribeFilter => write!(f, "personalize:DescribeFilter"),
            PersonalizeActions::DescribeMetricAttribution => {
                write!(f, "personalize:DescribeMetricAttribution")
            }
            PersonalizeActions::DescribeRecipe => write!(f, "personalize:DescribeRecipe"),
            PersonalizeActions::DescribeRecommender => write!(f, "personalize:DescribeRecommender"),
            PersonalizeActions::DescribeSchema => write!(f, "personalize:DescribeSchema"),
            PersonalizeActions::DescribeSolution => write!(f, "personalize:DescribeSolution"),
            PersonalizeActions::DescribeSolutionVersion => {
                write!(f, "personalize:DescribeSolutionVersion")
            }
            PersonalizeActions::GetActionRecommendations => {
                write!(f, "personalize:GetActionRecommendations")
            }
            PersonalizeActions::GetDataInsights => write!(f, "personalize:GetDataInsights"),
            PersonalizeActions::GetPersonalizedRanking => {
                write!(f, "personalize:GetPersonalizedRanking")
            }
            PersonalizeActions::GetRecommendations => write!(f, "personalize:GetRecommendations"),
            PersonalizeActions::GetSolutionMetrics => write!(f, "personalize:GetSolutionMetrics"),
            PersonalizeActions::ListBatchInferenceJobs => {
                write!(f, "personalize:ListBatchInferenceJobs")
            }
            PersonalizeActions::ListBatchSegmentJobs => {
                write!(f, "personalize:ListBatchSegmentJobs")
            }
            PersonalizeActions::ListCampaigns => write!(f, "personalize:ListCampaigns"),
            PersonalizeActions::ListDataDeletionJobs => {
                write!(f, "personalize:ListDataDeletionJobs")
            }
            PersonalizeActions::ListDataInsightsJobs => {
                write!(f, "personalize:ListDataInsightsJobs")
            }
            PersonalizeActions::ListDatasetExportJobs => {
                write!(f, "personalize:ListDatasetExportJobs")
            }
            PersonalizeActions::ListDatasetGroups => write!(f, "personalize:ListDatasetGroups"),
            PersonalizeActions::ListDatasetImportJobs => {
                write!(f, "personalize:ListDatasetImportJobs")
            }
            PersonalizeActions::ListDatasets => write!(f, "personalize:ListDatasets"),
            PersonalizeActions::ListEventTrackers => write!(f, "personalize:ListEventTrackers"),
            PersonalizeActions::ListFilters => write!(f, "personalize:ListFilters"),
            PersonalizeActions::ListMetricAttributionMetrics => {
                write!(f, "personalize:ListMetricAttributionMetrics")
            }
            PersonalizeActions::ListMetricAttributions => {
                write!(f, "personalize:ListMetricAttributions")
            }
            PersonalizeActions::ListRecipes => write!(f, "personalize:ListRecipes"),
            PersonalizeActions::ListRecommenders => write!(f, "personalize:ListRecommenders"),
            PersonalizeActions::ListSchemas => write!(f, "personalize:ListSchemas"),
            PersonalizeActions::ListSolutionVersions => {
                write!(f, "personalize:ListSolutionVersions")
            }
            PersonalizeActions::ListSolutions => write!(f, "personalize:ListSolutions"),
            PersonalizeActions::ListTagsForResource => write!(f, "personalize:ListTagsForResource"),
            PersonalizeActions::PutActionInteractions => {
                write!(f, "personalize:PutActionInteractions")
            }
            PersonalizeActions::PutActions => write!(f, "personalize:PutActions"),
            PersonalizeActions::PutEvents => write!(f, "personalize:PutEvents"),
            PersonalizeActions::PutItems => write!(f, "personalize:PutItems"),
            PersonalizeActions::PutUsers => write!(f, "personalize:PutUsers"),
            PersonalizeActions::StartRecommender => write!(f, "personalize:StartRecommender"),
            PersonalizeActions::StopRecommender => write!(f, "personalize:StopRecommender"),
            PersonalizeActions::StopSolutionVersionCreation => {
                write!(f, "personalize:StopSolutionVersionCreation")
            }
            PersonalizeActions::TagResource => write!(f, "personalize:TagResource"),
            PersonalizeActions::UntagResource => write!(f, "personalize:UntagResource"),
            PersonalizeActions::UpdateCampaign => write!(f, "personalize:UpdateCampaign"),
            PersonalizeActions::UpdateDataset => write!(f, "personalize:UpdateDataset"),
            PersonalizeActions::UpdateMetricAttribution => {
                write!(f, "personalize:UpdateMetricAttribution")
            }
            PersonalizeActions::UpdateRecommender => write!(f, "personalize:UpdateRecommender"),
            PersonalizeActions::UpdateSolution => write!(f, "personalize:UpdateSolution"),
        }
    }
}
