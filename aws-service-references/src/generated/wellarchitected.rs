// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum WellarchitectedActions {
    AssociateLenses,
    AssociateProfiles,
    ConfigureIntegration,
    CreateLensShare,
    CreateLensVersion,
    CreateMilestone,
    CreateProfile,
    CreateProfileShare,
    CreateReviewTemplate,
    CreateTemplateShare,
    CreateWorkload,
    CreateWorkloadShare,
    DeleteLens,
    DeleteLensShare,
    DeleteProfile,
    DeleteProfileShare,
    DeleteReviewTemplate,
    DeleteTemplateShare,
    DeleteWorkload,
    DeleteWorkloadShare,
    DisassociateLenses,
    DisassociateProfiles,
    ExportLens,
    GetAnswer,
    GetConsolidatedReport,
    GetGlobalSettings,
    GetLens,
    GetLensReview,
    GetLensReviewReport,
    GetLensVersionDifference,
    GetMilestone,
    GetProfile,
    GetProfileTemplate,
    GetReviewTemplate,
    GetReviewTemplateAnswer,
    GetReviewTemplateLensReview,
    GetWorkload,
    ImportLens,
    ListAnswers,
    ListCheckDetails,
    ListCheckSummaries,
    ListLensReviewImprovements,
    ListLensReviews,
    ListLensShares,
    ListLenses,
    ListMilestones,
    ListNotifications,
    ListProfileNotifications,
    ListProfileShares,
    ListProfiles,
    ListReviewTemplateAnswers,
    ListReviewTemplates,
    ListShareInvitations,
    ListTagsForResource,
    ListTemplateShares,
    ListWorkloadShares,
    ListWorkloads,
    TagResource,
    UntagResource,
    UpdateAnswer,
    UpdateGlobalSettings,
    UpdateIntegration,
    UpdateLensReview,
    UpdateProfile,
    UpdateReviewTemplate,
    UpdateReviewTemplateAnswer,
    UpdateReviewTemplateLensReview,
    UpdateShareInvitation,
    UpdateWorkload,
    UpdateWorkloadShare,
    UpgradeLensReview,
    UpgradeProfileVersion,
    UpgradeReviewTemplateLensReview,
}
impl std::fmt::Display for WellarchitectedActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WellarchitectedActions::AssociateLenses => write!(f, "wellarchitected:AssociateLenses"),
            WellarchitectedActions::AssociateProfiles => {
                write!(f, "wellarchitected:AssociateProfiles")
            }
            WellarchitectedActions::ConfigureIntegration => {
                write!(f, "wellarchitected:ConfigureIntegration")
            }
            WellarchitectedActions::CreateLensShare => write!(f, "wellarchitected:CreateLensShare"),
            WellarchitectedActions::CreateLensVersion => {
                write!(f, "wellarchitected:CreateLensVersion")
            }
            WellarchitectedActions::CreateMilestone => write!(f, "wellarchitected:CreateMilestone"),
            WellarchitectedActions::CreateProfile => write!(f, "wellarchitected:CreateProfile"),
            WellarchitectedActions::CreateProfileShare => {
                write!(f, "wellarchitected:CreateProfileShare")
            }
            WellarchitectedActions::CreateReviewTemplate => {
                write!(f, "wellarchitected:CreateReviewTemplate")
            }
            WellarchitectedActions::CreateTemplateShare => {
                write!(f, "wellarchitected:CreateTemplateShare")
            }
            WellarchitectedActions::CreateWorkload => write!(f, "wellarchitected:CreateWorkload"),
            WellarchitectedActions::CreateWorkloadShare => {
                write!(f, "wellarchitected:CreateWorkloadShare")
            }
            WellarchitectedActions::DeleteLens => write!(f, "wellarchitected:DeleteLens"),
            WellarchitectedActions::DeleteLensShare => write!(f, "wellarchitected:DeleteLensShare"),
            WellarchitectedActions::DeleteProfile => write!(f, "wellarchitected:DeleteProfile"),
            WellarchitectedActions::DeleteProfileShare => {
                write!(f, "wellarchitected:DeleteProfileShare")
            }
            WellarchitectedActions::DeleteReviewTemplate => {
                write!(f, "wellarchitected:DeleteReviewTemplate")
            }
            WellarchitectedActions::DeleteTemplateShare => {
                write!(f, "wellarchitected:DeleteTemplateShare")
            }
            WellarchitectedActions::DeleteWorkload => write!(f, "wellarchitected:DeleteWorkload"),
            WellarchitectedActions::DeleteWorkloadShare => {
                write!(f, "wellarchitected:DeleteWorkloadShare")
            }
            WellarchitectedActions::DisassociateLenses => {
                write!(f, "wellarchitected:DisassociateLenses")
            }
            WellarchitectedActions::DisassociateProfiles => {
                write!(f, "wellarchitected:DisassociateProfiles")
            }
            WellarchitectedActions::ExportLens => write!(f, "wellarchitected:ExportLens"),
            WellarchitectedActions::GetAnswer => write!(f, "wellarchitected:GetAnswer"),
            WellarchitectedActions::GetConsolidatedReport => {
                write!(f, "wellarchitected:GetConsolidatedReport")
            }
            WellarchitectedActions::GetGlobalSettings => {
                write!(f, "wellarchitected:GetGlobalSettings")
            }
            WellarchitectedActions::GetLens => write!(f, "wellarchitected:GetLens"),
            WellarchitectedActions::GetLensReview => write!(f, "wellarchitected:GetLensReview"),
            WellarchitectedActions::GetLensReviewReport => {
                write!(f, "wellarchitected:GetLensReviewReport")
            }
            WellarchitectedActions::GetLensVersionDifference => {
                write!(f, "wellarchitected:GetLensVersionDifference")
            }
            WellarchitectedActions::GetMilestone => write!(f, "wellarchitected:GetMilestone"),
            WellarchitectedActions::GetProfile => write!(f, "wellarchitected:GetProfile"),
            WellarchitectedActions::GetProfileTemplate => {
                write!(f, "wellarchitected:GetProfileTemplate")
            }
            WellarchitectedActions::GetReviewTemplate => {
                write!(f, "wellarchitected:GetReviewTemplate")
            }
            WellarchitectedActions::GetReviewTemplateAnswer => {
                write!(f, "wellarchitected:GetReviewTemplateAnswer")
            }
            WellarchitectedActions::GetReviewTemplateLensReview => {
                write!(f, "wellarchitected:GetReviewTemplateLensReview")
            }
            WellarchitectedActions::GetWorkload => write!(f, "wellarchitected:GetWorkload"),
            WellarchitectedActions::ImportLens => write!(f, "wellarchitected:ImportLens"),
            WellarchitectedActions::ListAnswers => write!(f, "wellarchitected:ListAnswers"),
            WellarchitectedActions::ListCheckDetails => {
                write!(f, "wellarchitected:ListCheckDetails")
            }
            WellarchitectedActions::ListCheckSummaries => {
                write!(f, "wellarchitected:ListCheckSummaries")
            }
            WellarchitectedActions::ListLensReviewImprovements => {
                write!(f, "wellarchitected:ListLensReviewImprovements")
            }
            WellarchitectedActions::ListLensReviews => write!(f, "wellarchitected:ListLensReviews"),
            WellarchitectedActions::ListLensShares => write!(f, "wellarchitected:ListLensShares"),
            WellarchitectedActions::ListLenses => write!(f, "wellarchitected:ListLenses"),
            WellarchitectedActions::ListMilestones => write!(f, "wellarchitected:ListMilestones"),
            WellarchitectedActions::ListNotifications => {
                write!(f, "wellarchitected:ListNotifications")
            }
            WellarchitectedActions::ListProfileNotifications => {
                write!(f, "wellarchitected:ListProfileNotifications")
            }
            WellarchitectedActions::ListProfileShares => {
                write!(f, "wellarchitected:ListProfileShares")
            }
            WellarchitectedActions::ListProfiles => write!(f, "wellarchitected:ListProfiles"),
            WellarchitectedActions::ListReviewTemplateAnswers => {
                write!(f, "wellarchitected:ListReviewTemplateAnswers")
            }
            WellarchitectedActions::ListReviewTemplates => {
                write!(f, "wellarchitected:ListReviewTemplates")
            }
            WellarchitectedActions::ListShareInvitations => {
                write!(f, "wellarchitected:ListShareInvitations")
            }
            WellarchitectedActions::ListTagsForResource => {
                write!(f, "wellarchitected:ListTagsForResource")
            }
            WellarchitectedActions::ListTemplateShares => {
                write!(f, "wellarchitected:ListTemplateShares")
            }
            WellarchitectedActions::ListWorkloadShares => {
                write!(f, "wellarchitected:ListWorkloadShares")
            }
            WellarchitectedActions::ListWorkloads => write!(f, "wellarchitected:ListWorkloads"),
            WellarchitectedActions::TagResource => write!(f, "wellarchitected:TagResource"),
            WellarchitectedActions::UntagResource => write!(f, "wellarchitected:UntagResource"),
            WellarchitectedActions::UpdateAnswer => write!(f, "wellarchitected:UpdateAnswer"),
            WellarchitectedActions::UpdateGlobalSettings => {
                write!(f, "wellarchitected:UpdateGlobalSettings")
            }
            WellarchitectedActions::UpdateIntegration => {
                write!(f, "wellarchitected:UpdateIntegration")
            }
            WellarchitectedActions::UpdateLensReview => {
                write!(f, "wellarchitected:UpdateLensReview")
            }
            WellarchitectedActions::UpdateProfile => write!(f, "wellarchitected:UpdateProfile"),
            WellarchitectedActions::UpdateReviewTemplate => {
                write!(f, "wellarchitected:UpdateReviewTemplate")
            }
            WellarchitectedActions::UpdateReviewTemplateAnswer => {
                write!(f, "wellarchitected:UpdateReviewTemplateAnswer")
            }
            WellarchitectedActions::UpdateReviewTemplateLensReview => {
                write!(f, "wellarchitected:UpdateReviewTemplateLensReview")
            }
            WellarchitectedActions::UpdateShareInvitation => {
                write!(f, "wellarchitected:UpdateShareInvitation")
            }
            WellarchitectedActions::UpdateWorkload => write!(f, "wellarchitected:UpdateWorkload"),
            WellarchitectedActions::UpdateWorkloadShare => {
                write!(f, "wellarchitected:UpdateWorkloadShare")
            }
            WellarchitectedActions::UpgradeLensReview => {
                write!(f, "wellarchitected:UpgradeLensReview")
            }
            WellarchitectedActions::UpgradeProfileVersion => {
                write!(f, "wellarchitected:UpgradeProfileVersion")
            }
            WellarchitectedActions::UpgradeReviewTemplateLensReview => {
                write!(f, "wellarchitected:UpgradeReviewTemplateLensReview")
            }
        }
    }
}
