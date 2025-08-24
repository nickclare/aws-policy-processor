// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum KendraActions {
    AssociateEntitiesToExperience,
    AssociatePersonasToEntities,
    BatchDeleteDocument,
    BatchDeleteFeaturedResultsSet,
    BatchGetDocumentStatus,
    BatchPutDocument,
    ClearQuerySuggestions,
    CreateAccessControlConfiguration,
    CreateDataSource,
    CreateExperience,
    CreateFaq,
    CreateFeaturedResultsSet,
    CreateIndex,
    CreateQuerySuggestionsBlockList,
    CreateThesaurus,
    DeleteAccessControlConfiguration,
    DeleteDataSource,
    DeleteExperience,
    DeleteFaq,
    DeleteIndex,
    DeletePrincipalMapping,
    DeleteQuerySuggestionsBlockList,
    DeleteThesaurus,
    DescribeAccessControlConfiguration,
    DescribeDataSource,
    DescribeExperience,
    DescribeFaq,
    DescribeFeaturedResultsSet,
    DescribeIndex,
    DescribePrincipalMapping,
    DescribeQuerySuggestionsBlockList,
    DescribeQuerySuggestionsConfig,
    DescribeThesaurus,
    DisassociateEntitiesFromExperience,
    DisassociatePersonasFromEntities,
    GetQuerySuggestions,
    GetSnapshots,
    ListAccessControlConfigurations,
    ListDataSourceSyncJobs,
    ListDataSources,
    ListEntityPersonas,
    ListExperienceEntities,
    ListExperiences,
    ListFaqs,
    ListFeaturedResultsSets,
    ListGroupsOlderThanOrderingId,
    ListIndices,
    ListQuerySuggestionsBlockLists,
    ListTagsForResource,
    ListThesauri,
    PutPrincipalMapping,
    Query,
    Retrieve,
    StartDataSourceSyncJob,
    StopDataSourceSyncJob,
    SubmitFeedback,
    TagResource,
    UntagResource,
    UpdateAccessControlConfiguration,
    UpdateDataSource,
    UpdateExperience,
    UpdateFeaturedResultsSet,
    UpdateIndex,
    UpdateQuerySuggestionsBlockList,
    UpdateQuerySuggestionsConfig,
    UpdateThesaurus,
}
impl std::fmt::Display for KendraActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KendraActions::AssociateEntitiesToExperience => {
                write!(f, "kendra:AssociateEntitiesToExperience")
            }
            KendraActions::AssociatePersonasToEntities => {
                write!(f, "kendra:AssociatePersonasToEntities")
            }
            KendraActions::BatchDeleteDocument => write!(f, "kendra:BatchDeleteDocument"),
            KendraActions::BatchDeleteFeaturedResultsSet => {
                write!(f, "kendra:BatchDeleteFeaturedResultsSet")
            }
            KendraActions::BatchGetDocumentStatus => write!(f, "kendra:BatchGetDocumentStatus"),
            KendraActions::BatchPutDocument => write!(f, "kendra:BatchPutDocument"),
            KendraActions::ClearQuerySuggestions => write!(f, "kendra:ClearQuerySuggestions"),
            KendraActions::CreateAccessControlConfiguration => {
                write!(f, "kendra:CreateAccessControlConfiguration")
            }
            KendraActions::CreateDataSource => write!(f, "kendra:CreateDataSource"),
            KendraActions::CreateExperience => write!(f, "kendra:CreateExperience"),
            KendraActions::CreateFaq => write!(f, "kendra:CreateFaq"),
            KendraActions::CreateFeaturedResultsSet => write!(f, "kendra:CreateFeaturedResultsSet"),
            KendraActions::CreateIndex => write!(f, "kendra:CreateIndex"),
            KendraActions::CreateQuerySuggestionsBlockList => {
                write!(f, "kendra:CreateQuerySuggestionsBlockList")
            }
            KendraActions::CreateThesaurus => write!(f, "kendra:CreateThesaurus"),
            KendraActions::DeleteAccessControlConfiguration => {
                write!(f, "kendra:DeleteAccessControlConfiguration")
            }
            KendraActions::DeleteDataSource => write!(f, "kendra:DeleteDataSource"),
            KendraActions::DeleteExperience => write!(f, "kendra:DeleteExperience"),
            KendraActions::DeleteFaq => write!(f, "kendra:DeleteFaq"),
            KendraActions::DeleteIndex => write!(f, "kendra:DeleteIndex"),
            KendraActions::DeletePrincipalMapping => write!(f, "kendra:DeletePrincipalMapping"),
            KendraActions::DeleteQuerySuggestionsBlockList => {
                write!(f, "kendra:DeleteQuerySuggestionsBlockList")
            }
            KendraActions::DeleteThesaurus => write!(f, "kendra:DeleteThesaurus"),
            KendraActions::DescribeAccessControlConfiguration => {
                write!(f, "kendra:DescribeAccessControlConfiguration")
            }
            KendraActions::DescribeDataSource => write!(f, "kendra:DescribeDataSource"),
            KendraActions::DescribeExperience => write!(f, "kendra:DescribeExperience"),
            KendraActions::DescribeFaq => write!(f, "kendra:DescribeFaq"),
            KendraActions::DescribeFeaturedResultsSet => {
                write!(f, "kendra:DescribeFeaturedResultsSet")
            }
            KendraActions::DescribeIndex => write!(f, "kendra:DescribeIndex"),
            KendraActions::DescribePrincipalMapping => write!(f, "kendra:DescribePrincipalMapping"),
            KendraActions::DescribeQuerySuggestionsBlockList => {
                write!(f, "kendra:DescribeQuerySuggestionsBlockList")
            }
            KendraActions::DescribeQuerySuggestionsConfig => {
                write!(f, "kendra:DescribeQuerySuggestionsConfig")
            }
            KendraActions::DescribeThesaurus => write!(f, "kendra:DescribeThesaurus"),
            KendraActions::DisassociateEntitiesFromExperience => {
                write!(f, "kendra:DisassociateEntitiesFromExperience")
            }
            KendraActions::DisassociatePersonasFromEntities => {
                write!(f, "kendra:DisassociatePersonasFromEntities")
            }
            KendraActions::GetQuerySuggestions => write!(f, "kendra:GetQuerySuggestions"),
            KendraActions::GetSnapshots => write!(f, "kendra:GetSnapshots"),
            KendraActions::ListAccessControlConfigurations => {
                write!(f, "kendra:ListAccessControlConfigurations")
            }
            KendraActions::ListDataSourceSyncJobs => write!(f, "kendra:ListDataSourceSyncJobs"),
            KendraActions::ListDataSources => write!(f, "kendra:ListDataSources"),
            KendraActions::ListEntityPersonas => write!(f, "kendra:ListEntityPersonas"),
            KendraActions::ListExperienceEntities => write!(f, "kendra:ListExperienceEntities"),
            KendraActions::ListExperiences => write!(f, "kendra:ListExperiences"),
            KendraActions::ListFaqs => write!(f, "kendra:ListFaqs"),
            KendraActions::ListFeaturedResultsSets => write!(f, "kendra:ListFeaturedResultsSets"),
            KendraActions::ListGroupsOlderThanOrderingId => {
                write!(f, "kendra:ListGroupsOlderThanOrderingId")
            }
            KendraActions::ListIndices => write!(f, "kendra:ListIndices"),
            KendraActions::ListQuerySuggestionsBlockLists => {
                write!(f, "kendra:ListQuerySuggestionsBlockLists")
            }
            KendraActions::ListTagsForResource => write!(f, "kendra:ListTagsForResource"),
            KendraActions::ListThesauri => write!(f, "kendra:ListThesauri"),
            KendraActions::PutPrincipalMapping => write!(f, "kendra:PutPrincipalMapping"),
            KendraActions::Query => write!(f, "kendra:Query"),
            KendraActions::Retrieve => write!(f, "kendra:Retrieve"),
            KendraActions::StartDataSourceSyncJob => write!(f, "kendra:StartDataSourceSyncJob"),
            KendraActions::StopDataSourceSyncJob => write!(f, "kendra:StopDataSourceSyncJob"),
            KendraActions::SubmitFeedback => write!(f, "kendra:SubmitFeedback"),
            KendraActions::TagResource => write!(f, "kendra:TagResource"),
            KendraActions::UntagResource => write!(f, "kendra:UntagResource"),
            KendraActions::UpdateAccessControlConfiguration => {
                write!(f, "kendra:UpdateAccessControlConfiguration")
            }
            KendraActions::UpdateDataSource => write!(f, "kendra:UpdateDataSource"),
            KendraActions::UpdateExperience => write!(f, "kendra:UpdateExperience"),
            KendraActions::UpdateFeaturedResultsSet => write!(f, "kendra:UpdateFeaturedResultsSet"),
            KendraActions::UpdateIndex => write!(f, "kendra:UpdateIndex"),
            KendraActions::UpdateQuerySuggestionsBlockList => {
                write!(f, "kendra:UpdateQuerySuggestionsBlockList")
            }
            KendraActions::UpdateQuerySuggestionsConfig => {
                write!(f, "kendra:UpdateQuerySuggestionsConfig")
            }
            KendraActions::UpdateThesaurus => write!(f, "kendra:UpdateThesaurus"),
        }
    }
}
