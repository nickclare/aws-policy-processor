// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum CodecommitActions {
    AssociateApprovalRuleTemplateWithRepository,
    BatchAssociateApprovalRuleTemplateWithRepositories,
    BatchDescribeMergeConflicts,
    BatchDisassociateApprovalRuleTemplateFromRepositories,
    BatchGetCommits,
    BatchGetPullRequests,
    BatchGetRepositories,
    CancelUploadArchive,
    CreateApprovalRuleTemplate,
    CreateBranch,
    CreateCommit,
    CreatePullRequest,
    CreatePullRequestApprovalRule,
    CreateRepository,
    CreateUnreferencedMergeCommit,
    DeleteApprovalRuleTemplate,
    DeleteBranch,
    DeleteCommentContent,
    DeleteFile,
    DeletePullRequestApprovalRule,
    DeleteRepository,
    DescribeMergeConflicts,
    DescribePullRequestEvents,
    DisassociateApprovalRuleTemplateFromRepository,
    EvaluatePullRequestApprovalRules,
    GetApprovalRuleTemplate,
    GetBlob,
    GetBranch,
    GetComment,
    GetCommentReactions,
    GetCommentsForComparedCommit,
    GetCommentsForPullRequest,
    GetCommit,
    GetCommitHistory,
    GetCommitsFromMergeBase,
    GetDifferences,
    GetFile,
    GetFolder,
    GetMergeCommit,
    GetMergeConflicts,
    GetMergeOptions,
    GetObjectIdentifier,
    GetPullRequest,
    GetPullRequestApprovalStates,
    GetPullRequestOverrideState,
    GetReferences,
    GetRepository,
    GetRepositoryTriggers,
    GetTree,
    GetUploadArchiveStatus,
    GitPull,
    GitPush,
    ListApprovalRuleTemplates,
    ListAssociatedApprovalRuleTemplatesForRepository,
    ListBranches,
    ListFileCommitHistory,
    ListPullRequests,
    ListRepositories,
    ListRepositoriesForApprovalRuleTemplate,
    ListTagsForResource,
    MergeBranchesByFastForward,
    MergeBranchesBySquash,
    MergeBranchesByThreeWay,
    MergePullRequestByFastForward,
    MergePullRequestBySquash,
    MergePullRequestByThreeWay,
    OverridePullRequestApprovalRules,
    PostCommentForComparedCommit,
    PostCommentForPullRequest,
    PostCommentReply,
    PutCommentReaction,
    PutFile,
    PutRepositoryTriggers,
    TagResource,
    TestRepositoryTriggers,
    UntagResource,
    UpdateApprovalRuleTemplateContent,
    UpdateApprovalRuleTemplateDescription,
    UpdateApprovalRuleTemplateName,
    UpdateComment,
    UpdateDefaultBranch,
    UpdatePullRequestApprovalRuleContent,
    UpdatePullRequestApprovalState,
    UpdatePullRequestDescription,
    UpdatePullRequestStatus,
    UpdatePullRequestTitle,
    UpdateRepositoryDescription,
    UpdateRepositoryEncryptionKey,
    UpdateRepositoryName,
    UploadArchive,
}
impl std::fmt::Display for CodecommitActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CodecommitActions::AssociateApprovalRuleTemplateWithRepository => {
                write!(f, "codecommit:AssociateApprovalRuleTemplateWithRepository")
            }
            CodecommitActions::BatchAssociateApprovalRuleTemplateWithRepositories => write!(
                f,
                "codecommit:BatchAssociateApprovalRuleTemplateWithRepositories"
            ),
            CodecommitActions::BatchDescribeMergeConflicts => {
                write!(f, "codecommit:BatchDescribeMergeConflicts")
            }
            CodecommitActions::BatchDisassociateApprovalRuleTemplateFromRepositories => write!(
                f,
                "codecommit:BatchDisassociateApprovalRuleTemplateFromRepositories"
            ),
            CodecommitActions::BatchGetCommits => write!(f, "codecommit:BatchGetCommits"),
            CodecommitActions::BatchGetPullRequests => write!(f, "codecommit:BatchGetPullRequests"),
            CodecommitActions::BatchGetRepositories => write!(f, "codecommit:BatchGetRepositories"),
            CodecommitActions::CancelUploadArchive => write!(f, "codecommit:CancelUploadArchive"),
            CodecommitActions::CreateApprovalRuleTemplate => {
                write!(f, "codecommit:CreateApprovalRuleTemplate")
            }
            CodecommitActions::CreateBranch => write!(f, "codecommit:CreateBranch"),
            CodecommitActions::CreateCommit => write!(f, "codecommit:CreateCommit"),
            CodecommitActions::CreatePullRequest => write!(f, "codecommit:CreatePullRequest"),
            CodecommitActions::CreatePullRequestApprovalRule => {
                write!(f, "codecommit:CreatePullRequestApprovalRule")
            }
            CodecommitActions::CreateRepository => write!(f, "codecommit:CreateRepository"),
            CodecommitActions::CreateUnreferencedMergeCommit => {
                write!(f, "codecommit:CreateUnreferencedMergeCommit")
            }
            CodecommitActions::DeleteApprovalRuleTemplate => {
                write!(f, "codecommit:DeleteApprovalRuleTemplate")
            }
            CodecommitActions::DeleteBranch => write!(f, "codecommit:DeleteBranch"),
            CodecommitActions::DeleteCommentContent => write!(f, "codecommit:DeleteCommentContent"),
            CodecommitActions::DeleteFile => write!(f, "codecommit:DeleteFile"),
            CodecommitActions::DeletePullRequestApprovalRule => {
                write!(f, "codecommit:DeletePullRequestApprovalRule")
            }
            CodecommitActions::DeleteRepository => write!(f, "codecommit:DeleteRepository"),
            CodecommitActions::DescribeMergeConflicts => {
                write!(f, "codecommit:DescribeMergeConflicts")
            }
            CodecommitActions::DescribePullRequestEvents => {
                write!(f, "codecommit:DescribePullRequestEvents")
            }
            CodecommitActions::DisassociateApprovalRuleTemplateFromRepository => write!(
                f,
                "codecommit:DisassociateApprovalRuleTemplateFromRepository"
            ),
            CodecommitActions::EvaluatePullRequestApprovalRules => {
                write!(f, "codecommit:EvaluatePullRequestApprovalRules")
            }
            CodecommitActions::GetApprovalRuleTemplate => {
                write!(f, "codecommit:GetApprovalRuleTemplate")
            }
            CodecommitActions::GetBlob => write!(f, "codecommit:GetBlob"),
            CodecommitActions::GetBranch => write!(f, "codecommit:GetBranch"),
            CodecommitActions::GetComment => write!(f, "codecommit:GetComment"),
            CodecommitActions::GetCommentReactions => write!(f, "codecommit:GetCommentReactions"),
            CodecommitActions::GetCommentsForComparedCommit => {
                write!(f, "codecommit:GetCommentsForComparedCommit")
            }
            CodecommitActions::GetCommentsForPullRequest => {
                write!(f, "codecommit:GetCommentsForPullRequest")
            }
            CodecommitActions::GetCommit => write!(f, "codecommit:GetCommit"),
            CodecommitActions::GetCommitHistory => write!(f, "codecommit:GetCommitHistory"),
            CodecommitActions::GetCommitsFromMergeBase => {
                write!(f, "codecommit:GetCommitsFromMergeBase")
            }
            CodecommitActions::GetDifferences => write!(f, "codecommit:GetDifferences"),
            CodecommitActions::GetFile => write!(f, "codecommit:GetFile"),
            CodecommitActions::GetFolder => write!(f, "codecommit:GetFolder"),
            CodecommitActions::GetMergeCommit => write!(f, "codecommit:GetMergeCommit"),
            CodecommitActions::GetMergeConflicts => write!(f, "codecommit:GetMergeConflicts"),
            CodecommitActions::GetMergeOptions => write!(f, "codecommit:GetMergeOptions"),
            CodecommitActions::GetObjectIdentifier => write!(f, "codecommit:GetObjectIdentifier"),
            CodecommitActions::GetPullRequest => write!(f, "codecommit:GetPullRequest"),
            CodecommitActions::GetPullRequestApprovalStates => {
                write!(f, "codecommit:GetPullRequestApprovalStates")
            }
            CodecommitActions::GetPullRequestOverrideState => {
                write!(f, "codecommit:GetPullRequestOverrideState")
            }
            CodecommitActions::GetReferences => write!(f, "codecommit:GetReferences"),
            CodecommitActions::GetRepository => write!(f, "codecommit:GetRepository"),
            CodecommitActions::GetRepositoryTriggers => {
                write!(f, "codecommit:GetRepositoryTriggers")
            }
            CodecommitActions::GetTree => write!(f, "codecommit:GetTree"),
            CodecommitActions::GetUploadArchiveStatus => {
                write!(f, "codecommit:GetUploadArchiveStatus")
            }
            CodecommitActions::GitPull => write!(f, "codecommit:GitPull"),
            CodecommitActions::GitPush => write!(f, "codecommit:GitPush"),
            CodecommitActions::ListApprovalRuleTemplates => {
                write!(f, "codecommit:ListApprovalRuleTemplates")
            }
            CodecommitActions::ListAssociatedApprovalRuleTemplatesForRepository => write!(
                f,
                "codecommit:ListAssociatedApprovalRuleTemplatesForRepository"
            ),
            CodecommitActions::ListBranches => write!(f, "codecommit:ListBranches"),
            CodecommitActions::ListFileCommitHistory => {
                write!(f, "codecommit:ListFileCommitHistory")
            }
            CodecommitActions::ListPullRequests => write!(f, "codecommit:ListPullRequests"),
            CodecommitActions::ListRepositories => write!(f, "codecommit:ListRepositories"),
            CodecommitActions::ListRepositoriesForApprovalRuleTemplate => {
                write!(f, "codecommit:ListRepositoriesForApprovalRuleTemplate")
            }
            CodecommitActions::ListTagsForResource => write!(f, "codecommit:ListTagsForResource"),
            CodecommitActions::MergeBranchesByFastForward => {
                write!(f, "codecommit:MergeBranchesByFastForward")
            }
            CodecommitActions::MergeBranchesBySquash => {
                write!(f, "codecommit:MergeBranchesBySquash")
            }
            CodecommitActions::MergeBranchesByThreeWay => {
                write!(f, "codecommit:MergeBranchesByThreeWay")
            }
            CodecommitActions::MergePullRequestByFastForward => {
                write!(f, "codecommit:MergePullRequestByFastForward")
            }
            CodecommitActions::MergePullRequestBySquash => {
                write!(f, "codecommit:MergePullRequestBySquash")
            }
            CodecommitActions::MergePullRequestByThreeWay => {
                write!(f, "codecommit:MergePullRequestByThreeWay")
            }
            CodecommitActions::OverridePullRequestApprovalRules => {
                write!(f, "codecommit:OverridePullRequestApprovalRules")
            }
            CodecommitActions::PostCommentForComparedCommit => {
                write!(f, "codecommit:PostCommentForComparedCommit")
            }
            CodecommitActions::PostCommentForPullRequest => {
                write!(f, "codecommit:PostCommentForPullRequest")
            }
            CodecommitActions::PostCommentReply => write!(f, "codecommit:PostCommentReply"),
            CodecommitActions::PutCommentReaction => write!(f, "codecommit:PutCommentReaction"),
            CodecommitActions::PutFile => write!(f, "codecommit:PutFile"),
            CodecommitActions::PutRepositoryTriggers => {
                write!(f, "codecommit:PutRepositoryTriggers")
            }
            CodecommitActions::TagResource => write!(f, "codecommit:TagResource"),
            CodecommitActions::TestRepositoryTriggers => {
                write!(f, "codecommit:TestRepositoryTriggers")
            }
            CodecommitActions::UntagResource => write!(f, "codecommit:UntagResource"),
            CodecommitActions::UpdateApprovalRuleTemplateContent => {
                write!(f, "codecommit:UpdateApprovalRuleTemplateContent")
            }
            CodecommitActions::UpdateApprovalRuleTemplateDescription => {
                write!(f, "codecommit:UpdateApprovalRuleTemplateDescription")
            }
            CodecommitActions::UpdateApprovalRuleTemplateName => {
                write!(f, "codecommit:UpdateApprovalRuleTemplateName")
            }
            CodecommitActions::UpdateComment => write!(f, "codecommit:UpdateComment"),
            CodecommitActions::UpdateDefaultBranch => write!(f, "codecommit:UpdateDefaultBranch"),
            CodecommitActions::UpdatePullRequestApprovalRuleContent => {
                write!(f, "codecommit:UpdatePullRequestApprovalRuleContent")
            }
            CodecommitActions::UpdatePullRequestApprovalState => {
                write!(f, "codecommit:UpdatePullRequestApprovalState")
            }
            CodecommitActions::UpdatePullRequestDescription => {
                write!(f, "codecommit:UpdatePullRequestDescription")
            }
            CodecommitActions::UpdatePullRequestStatus => {
                write!(f, "codecommit:UpdatePullRequestStatus")
            }
            CodecommitActions::UpdatePullRequestTitle => {
                write!(f, "codecommit:UpdatePullRequestTitle")
            }
            CodecommitActions::UpdateRepositoryDescription => {
                write!(f, "codecommit:UpdateRepositoryDescription")
            }
            CodecommitActions::UpdateRepositoryEncryptionKey => {
                write!(f, "codecommit:UpdateRepositoryEncryptionKey")
            }
            CodecommitActions::UpdateRepositoryName => write!(f, "codecommit:UpdateRepositoryName"),
            CodecommitActions::UploadArchive => write!(f, "codecommit:UploadArchive"),
        }
    }
}
