// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum DeepracerActions {
    AddLeaderboardAccessPermission,
    AdminDescribeAccountKey,
    AdminGetAccountConfig,
    AdminListAssociatedResources,
    AdminListAssociatedUsers,
    AdminManageUser,
    AdminSetAccountConfig,
    AdminUpdateAccountKey,
    CloneReinforcementLearningModel,
    CreateCar,
    CreateLeaderboard,
    CreateLeaderboardAccessToken,
    CreateLeaderboardSubmission,
    CreateReinforcementLearningModel,
    DeleteLeaderboard,
    DeleteModel,
    EditLeaderboard,
    GetAccountConfig,
    GetAlias,
    GetAssetUrl,
    GetCar,
    GetCars,
    GetEvaluation,
    GetLatestUserSubmission,
    GetLeaderboard,
    GetModel,
    GetPrivateLeaderboard,
    GetRankedUserSubmission,
    GetTrack,
    GetTrainingJob,
    ImportModel,
    ListEvaluations,
    ListLeaderboardEvaluations,
    ListLeaderboardSubmissions,
    ListLeaderboards,
    ListModels,
    ListPrivateLeaderboardParticipants,
    ListPrivateLeaderboards,
    ListSubscribedPrivateLeaderboards,
    ListTagsForResource,
    ListTracks,
    ListTrainingJobs,
    MigrateModels,
    PerformLeaderboardOperation,
    RemoveLeaderboardAccessPermission,
    SetAlias,
    StartEvaluation,
    StopEvaluation,
    StopTrainingReinforcementLearningModel,
    TagResource,
    TestRewardFunction,
    UntagResource,
    UpdateCar,
}
impl std::fmt::Display for DeepracerActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeepracerActions::AddLeaderboardAccessPermission => {
                write!(f, "deepracer:AddLeaderboardAccessPermission")
            }
            DeepracerActions::AdminDescribeAccountKey => {
                write!(f, "deepracer:AdminDescribeAccountKey")
            }
            DeepracerActions::AdminGetAccountConfig => write!(f, "deepracer:AdminGetAccountConfig"),
            DeepracerActions::AdminListAssociatedResources => {
                write!(f, "deepracer:AdminListAssociatedResources")
            }
            DeepracerActions::AdminListAssociatedUsers => {
                write!(f, "deepracer:AdminListAssociatedUsers")
            }
            DeepracerActions::AdminManageUser => write!(f, "deepracer:AdminManageUser"),
            DeepracerActions::AdminSetAccountConfig => write!(f, "deepracer:AdminSetAccountConfig"),
            DeepracerActions::AdminUpdateAccountKey => write!(f, "deepracer:AdminUpdateAccountKey"),
            DeepracerActions::CloneReinforcementLearningModel => {
                write!(f, "deepracer:CloneReinforcementLearningModel")
            }
            DeepracerActions::CreateCar => write!(f, "deepracer:CreateCar"),
            DeepracerActions::CreateLeaderboard => write!(f, "deepracer:CreateLeaderboard"),
            DeepracerActions::CreateLeaderboardAccessToken => {
                write!(f, "deepracer:CreateLeaderboardAccessToken")
            }
            DeepracerActions::CreateLeaderboardSubmission => {
                write!(f, "deepracer:CreateLeaderboardSubmission")
            }
            DeepracerActions::CreateReinforcementLearningModel => {
                write!(f, "deepracer:CreateReinforcementLearningModel")
            }
            DeepracerActions::DeleteLeaderboard => write!(f, "deepracer:DeleteLeaderboard"),
            DeepracerActions::DeleteModel => write!(f, "deepracer:DeleteModel"),
            DeepracerActions::EditLeaderboard => write!(f, "deepracer:EditLeaderboard"),
            DeepracerActions::GetAccountConfig => write!(f, "deepracer:GetAccountConfig"),
            DeepracerActions::GetAlias => write!(f, "deepracer:GetAlias"),
            DeepracerActions::GetAssetUrl => write!(f, "deepracer:GetAssetUrl"),
            DeepracerActions::GetCar => write!(f, "deepracer:GetCar"),
            DeepracerActions::GetCars => write!(f, "deepracer:GetCars"),
            DeepracerActions::GetEvaluation => write!(f, "deepracer:GetEvaluation"),
            DeepracerActions::GetLatestUserSubmission => {
                write!(f, "deepracer:GetLatestUserSubmission")
            }
            DeepracerActions::GetLeaderboard => write!(f, "deepracer:GetLeaderboard"),
            DeepracerActions::GetModel => write!(f, "deepracer:GetModel"),
            DeepracerActions::GetPrivateLeaderboard => write!(f, "deepracer:GetPrivateLeaderboard"),
            DeepracerActions::GetRankedUserSubmission => {
                write!(f, "deepracer:GetRankedUserSubmission")
            }
            DeepracerActions::GetTrack => write!(f, "deepracer:GetTrack"),
            DeepracerActions::GetTrainingJob => write!(f, "deepracer:GetTrainingJob"),
            DeepracerActions::ImportModel => write!(f, "deepracer:ImportModel"),
            DeepracerActions::ListEvaluations => write!(f, "deepracer:ListEvaluations"),
            DeepracerActions::ListLeaderboardEvaluations => {
                write!(f, "deepracer:ListLeaderboardEvaluations")
            }
            DeepracerActions::ListLeaderboardSubmissions => {
                write!(f, "deepracer:ListLeaderboardSubmissions")
            }
            DeepracerActions::ListLeaderboards => write!(f, "deepracer:ListLeaderboards"),
            DeepracerActions::ListModels => write!(f, "deepracer:ListModels"),
            DeepracerActions::ListPrivateLeaderboardParticipants => {
                write!(f, "deepracer:ListPrivateLeaderboardParticipants")
            }
            DeepracerActions::ListPrivateLeaderboards => {
                write!(f, "deepracer:ListPrivateLeaderboards")
            }
            DeepracerActions::ListSubscribedPrivateLeaderboards => {
                write!(f, "deepracer:ListSubscribedPrivateLeaderboards")
            }
            DeepracerActions::ListTagsForResource => write!(f, "deepracer:ListTagsForResource"),
            DeepracerActions::ListTracks => write!(f, "deepracer:ListTracks"),
            DeepracerActions::ListTrainingJobs => write!(f, "deepracer:ListTrainingJobs"),
            DeepracerActions::MigrateModels => write!(f, "deepracer:MigrateModels"),
            DeepracerActions::PerformLeaderboardOperation => {
                write!(f, "deepracer:PerformLeaderboardOperation")
            }
            DeepracerActions::RemoveLeaderboardAccessPermission => {
                write!(f, "deepracer:RemoveLeaderboardAccessPermission")
            }
            DeepracerActions::SetAlias => write!(f, "deepracer:SetAlias"),
            DeepracerActions::StartEvaluation => write!(f, "deepracer:StartEvaluation"),
            DeepracerActions::StopEvaluation => write!(f, "deepracer:StopEvaluation"),
            DeepracerActions::StopTrainingReinforcementLearningModel => {
                write!(f, "deepracer:StopTrainingReinforcementLearningModel")
            }
            DeepracerActions::TagResource => write!(f, "deepracer:TagResource"),
            DeepracerActions::TestRewardFunction => write!(f, "deepracer:TestRewardFunction"),
            DeepracerActions::UntagResource => write!(f, "deepracer:UntagResource"),
            DeepracerActions::UpdateCar => write!(f, "deepracer:UpdateCar"),
        }
    }
}
