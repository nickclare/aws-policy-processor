// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum CodeguruReviewerActions {
    AssociateRepository,
    CreateCodeReview,
    CreateConnectionToken,
    DescribeCodeReview,
    DescribeRecommendationFeedback,
    DescribeRepositoryAssociation,
    DisassociateRepository,
    GetMetricsData,
    ListCodeReviews,
    ListRecommendationFeedback,
    ListRecommendations,
    ListRepositoryAssociations,
    ListTagsForResource,
    ListThirdPartyRepositories,
    PutRecommendationFeedback,
    TagResource,
    UnTagResource,
}
impl std::fmt::Display for CodeguruReviewerActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CodeguruReviewerActions::AssociateRepository => {
                write!(f, "codeguru-reviewer:AssociateRepository")
            }
            CodeguruReviewerActions::CreateCodeReview => {
                write!(f, "codeguru-reviewer:CreateCodeReview")
            }
            CodeguruReviewerActions::CreateConnectionToken => {
                write!(f, "codeguru-reviewer:CreateConnectionToken")
            }
            CodeguruReviewerActions::DescribeCodeReview => {
                write!(f, "codeguru-reviewer:DescribeCodeReview")
            }
            CodeguruReviewerActions::DescribeRecommendationFeedback => {
                write!(f, "codeguru-reviewer:DescribeRecommendationFeedback")
            }
            CodeguruReviewerActions::DescribeRepositoryAssociation => {
                write!(f, "codeguru-reviewer:DescribeRepositoryAssociation")
            }
            CodeguruReviewerActions::DisassociateRepository => {
                write!(f, "codeguru-reviewer:DisassociateRepository")
            }
            CodeguruReviewerActions::GetMetricsData => {
                write!(f, "codeguru-reviewer:GetMetricsData")
            }
            CodeguruReviewerActions::ListCodeReviews => {
                write!(f, "codeguru-reviewer:ListCodeReviews")
            }
            CodeguruReviewerActions::ListRecommendationFeedback => {
                write!(f, "codeguru-reviewer:ListRecommendationFeedback")
            }
            CodeguruReviewerActions::ListRecommendations => {
                write!(f, "codeguru-reviewer:ListRecommendations")
            }
            CodeguruReviewerActions::ListRepositoryAssociations => {
                write!(f, "codeguru-reviewer:ListRepositoryAssociations")
            }
            CodeguruReviewerActions::ListTagsForResource => {
                write!(f, "codeguru-reviewer:ListTagsForResource")
            }
            CodeguruReviewerActions::ListThirdPartyRepositories => {
                write!(f, "codeguru-reviewer:ListThirdPartyRepositories")
            }
            CodeguruReviewerActions::PutRecommendationFeedback => {
                write!(f, "codeguru-reviewer:PutRecommendationFeedback")
            }
            CodeguruReviewerActions::TagResource => write!(f, "codeguru-reviewer:TagResource"),
            CodeguruReviewerActions::UnTagResource => write!(f, "codeguru-reviewer:UnTagResource"),
        }
    }
}
