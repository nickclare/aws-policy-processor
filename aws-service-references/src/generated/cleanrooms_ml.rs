// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum CleanroomsMlActions {
    CancelTrainedModel,
    CancelTrainedModelInferenceJob,
    CreateAudienceModel,
    CreateConfiguredAudienceModel,
    CreateConfiguredModelAlgorithm,
    CreateConfiguredModelAlgorithmAssociation,
    CreateMlInputChannel,
    CreateTrainedModel,
    CreateTrainingDataset,
    DeleteAudienceGenerationJob,
    DeleteAudienceModel,
    DeleteConfiguredAudienceModel,
    DeleteConfiguredAudienceModelPolicy,
    DeleteConfiguredModelAlgorithm,
    DeleteConfiguredModelAlgorithmAssociation,
    DeleteMlConfiguration,
    DeleteMlInputChannelData,
    DeleteTrainedModelOutput,
    DeleteTrainingDataset,
    GetAudienceGenerationJob,
    GetAudienceModel,
    GetCollaborationConfiguredModelAlgorithmAssociation,
    GetCollaborationMlInputChannel,
    GetCollaborationTrainedModel,
    GetConfiguredAudienceModel,
    GetConfiguredAudienceModelPolicy,
    GetConfiguredModelAlgorithm,
    GetConfiguredModelAlgorithmAssociation,
    GetMlConfiguration,
    GetMlInputChannel,
    GetTrainedModel,
    GetTrainedModelInferenceJob,
    GetTrainingDataset,
    ListAudienceExportJobs,
    ListAudienceGenerationJobs,
    ListAudienceModels,
    ListCollaborationConfiguredModelAlgorithmAssociations,
    ListCollaborationMlInputChannels,
    ListCollaborationTrainedModelExportJobs,
    ListCollaborationTrainedModelInferenceJobs,
    ListCollaborationTrainedModels,
    ListConfiguredAudienceModels,
    ListConfiguredModelAlgorithmAssociations,
    ListConfiguredModelAlgorithms,
    ListMlInputChannels,
    ListTagsForResource,
    ListTrainedModelInferenceJobs,
    ListTrainedModelVersions,
    ListTrainedModels,
    ListTrainingDatasets,
    PutConfiguredAudienceModelPolicy,
    PutMlConfiguration,
    StartAudienceExportJob,
    StartAudienceGenerationJob,
    StartTrainedModelExportJob,
    StartTrainedModelInferenceJob,
    TagResource,
    UnTagResource,
    UpdateConfiguredAudienceModel,
}
impl std::fmt::Display for CleanroomsMlActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CleanroomsMlActions::CancelTrainedModel => {
                write!(f, "cleanrooms-ml:CancelTrainedModel")
            }
            CleanroomsMlActions::CancelTrainedModelInferenceJob => {
                write!(f, "cleanrooms-ml:CancelTrainedModelInferenceJob")
            }
            CleanroomsMlActions::CreateAudienceModel => {
                write!(f, "cleanrooms-ml:CreateAudienceModel")
            }
            CleanroomsMlActions::CreateConfiguredAudienceModel => {
                write!(f, "cleanrooms-ml:CreateConfiguredAudienceModel")
            }
            CleanroomsMlActions::CreateConfiguredModelAlgorithm => {
                write!(f, "cleanrooms-ml:CreateConfiguredModelAlgorithm")
            }
            CleanroomsMlActions::CreateConfiguredModelAlgorithmAssociation => {
                write!(f, "cleanrooms-ml:CreateConfiguredModelAlgorithmAssociation")
            }
            CleanroomsMlActions::CreateMlInputChannel => {
                write!(f, "cleanrooms-ml:CreateMLInputChannel")
            }
            CleanroomsMlActions::CreateTrainedModel => {
                write!(f, "cleanrooms-ml:CreateTrainedModel")
            }
            CleanroomsMlActions::CreateTrainingDataset => {
                write!(f, "cleanrooms-ml:CreateTrainingDataset")
            }
            CleanroomsMlActions::DeleteAudienceGenerationJob => {
                write!(f, "cleanrooms-ml:DeleteAudienceGenerationJob")
            }
            CleanroomsMlActions::DeleteAudienceModel => {
                write!(f, "cleanrooms-ml:DeleteAudienceModel")
            }
            CleanroomsMlActions::DeleteConfiguredAudienceModel => {
                write!(f, "cleanrooms-ml:DeleteConfiguredAudienceModel")
            }
            CleanroomsMlActions::DeleteConfiguredAudienceModelPolicy => {
                write!(f, "cleanrooms-ml:DeleteConfiguredAudienceModelPolicy")
            }
            CleanroomsMlActions::DeleteConfiguredModelAlgorithm => {
                write!(f, "cleanrooms-ml:DeleteConfiguredModelAlgorithm")
            }
            CleanroomsMlActions::DeleteConfiguredModelAlgorithmAssociation => {
                write!(f, "cleanrooms-ml:DeleteConfiguredModelAlgorithmAssociation")
            }
            CleanroomsMlActions::DeleteMlConfiguration => {
                write!(f, "cleanrooms-ml:DeleteMLConfiguration")
            }
            CleanroomsMlActions::DeleteMlInputChannelData => {
                write!(f, "cleanrooms-ml:DeleteMLInputChannelData")
            }
            CleanroomsMlActions::DeleteTrainedModelOutput => {
                write!(f, "cleanrooms-ml:DeleteTrainedModelOutput")
            }
            CleanroomsMlActions::DeleteTrainingDataset => {
                write!(f, "cleanrooms-ml:DeleteTrainingDataset")
            }
            CleanroomsMlActions::GetAudienceGenerationJob => {
                write!(f, "cleanrooms-ml:GetAudienceGenerationJob")
            }
            CleanroomsMlActions::GetAudienceModel => write!(f, "cleanrooms-ml:GetAudienceModel"),
            CleanroomsMlActions::GetCollaborationConfiguredModelAlgorithmAssociation => write!(
                f,
                "cleanrooms-ml:GetCollaborationConfiguredModelAlgorithmAssociation"
            ),
            CleanroomsMlActions::GetCollaborationMlInputChannel => {
                write!(f, "cleanrooms-ml:GetCollaborationMLInputChannel")
            }
            CleanroomsMlActions::GetCollaborationTrainedModel => {
                write!(f, "cleanrooms-ml:GetCollaborationTrainedModel")
            }
            CleanroomsMlActions::GetConfiguredAudienceModel => {
                write!(f, "cleanrooms-ml:GetConfiguredAudienceModel")
            }
            CleanroomsMlActions::GetConfiguredAudienceModelPolicy => {
                write!(f, "cleanrooms-ml:GetConfiguredAudienceModelPolicy")
            }
            CleanroomsMlActions::GetConfiguredModelAlgorithm => {
                write!(f, "cleanrooms-ml:GetConfiguredModelAlgorithm")
            }
            CleanroomsMlActions::GetConfiguredModelAlgorithmAssociation => {
                write!(f, "cleanrooms-ml:GetConfiguredModelAlgorithmAssociation")
            }
            CleanroomsMlActions::GetMlConfiguration => {
                write!(f, "cleanrooms-ml:GetMLConfiguration")
            }
            CleanroomsMlActions::GetMlInputChannel => write!(f, "cleanrooms-ml:GetMLInputChannel"),
            CleanroomsMlActions::GetTrainedModel => write!(f, "cleanrooms-ml:GetTrainedModel"),
            CleanroomsMlActions::GetTrainedModelInferenceJob => {
                write!(f, "cleanrooms-ml:GetTrainedModelInferenceJob")
            }
            CleanroomsMlActions::GetTrainingDataset => {
                write!(f, "cleanrooms-ml:GetTrainingDataset")
            }
            CleanroomsMlActions::ListAudienceExportJobs => {
                write!(f, "cleanrooms-ml:ListAudienceExportJobs")
            }
            CleanroomsMlActions::ListAudienceGenerationJobs => {
                write!(f, "cleanrooms-ml:ListAudienceGenerationJobs")
            }
            CleanroomsMlActions::ListAudienceModels => {
                write!(f, "cleanrooms-ml:ListAudienceModels")
            }
            CleanroomsMlActions::ListCollaborationConfiguredModelAlgorithmAssociations => write!(
                f,
                "cleanrooms-ml:ListCollaborationConfiguredModelAlgorithmAssociations"
            ),
            CleanroomsMlActions::ListCollaborationMlInputChannels => {
                write!(f, "cleanrooms-ml:ListCollaborationMLInputChannels")
            }
            CleanroomsMlActions::ListCollaborationTrainedModelExportJobs => {
                write!(f, "cleanrooms-ml:ListCollaborationTrainedModelExportJobs")
            }
            CleanroomsMlActions::ListCollaborationTrainedModelInferenceJobs => write!(
                f,
                "cleanrooms-ml:ListCollaborationTrainedModelInferenceJobs"
            ),
            CleanroomsMlActions::ListCollaborationTrainedModels => {
                write!(f, "cleanrooms-ml:ListCollaborationTrainedModels")
            }
            CleanroomsMlActions::ListConfiguredAudienceModels => {
                write!(f, "cleanrooms-ml:ListConfiguredAudienceModels")
            }
            CleanroomsMlActions::ListConfiguredModelAlgorithmAssociations => {
                write!(f, "cleanrooms-ml:ListConfiguredModelAlgorithmAssociations")
            }
            CleanroomsMlActions::ListConfiguredModelAlgorithms => {
                write!(f, "cleanrooms-ml:ListConfiguredModelAlgorithms")
            }
            CleanroomsMlActions::ListMlInputChannels => {
                write!(f, "cleanrooms-ml:ListMLInputChannels")
            }
            CleanroomsMlActions::ListTagsForResource => {
                write!(f, "cleanrooms-ml:ListTagsForResource")
            }
            CleanroomsMlActions::ListTrainedModelInferenceJobs => {
                write!(f, "cleanrooms-ml:ListTrainedModelInferenceJobs")
            }
            CleanroomsMlActions::ListTrainedModelVersions => {
                write!(f, "cleanrooms-ml:ListTrainedModelVersions")
            }
            CleanroomsMlActions::ListTrainedModels => write!(f, "cleanrooms-ml:ListTrainedModels"),
            CleanroomsMlActions::ListTrainingDatasets => {
                write!(f, "cleanrooms-ml:ListTrainingDatasets")
            }
            CleanroomsMlActions::PutConfiguredAudienceModelPolicy => {
                write!(f, "cleanrooms-ml:PutConfiguredAudienceModelPolicy")
            }
            CleanroomsMlActions::PutMlConfiguration => {
                write!(f, "cleanrooms-ml:PutMLConfiguration")
            }
            CleanroomsMlActions::StartAudienceExportJob => {
                write!(f, "cleanrooms-ml:StartAudienceExportJob")
            }
            CleanroomsMlActions::StartAudienceGenerationJob => {
                write!(f, "cleanrooms-ml:StartAudienceGenerationJob")
            }
            CleanroomsMlActions::StartTrainedModelExportJob => {
                write!(f, "cleanrooms-ml:StartTrainedModelExportJob")
            }
            CleanroomsMlActions::StartTrainedModelInferenceJob => {
                write!(f, "cleanrooms-ml:StartTrainedModelInferenceJob")
            }
            CleanroomsMlActions::TagResource => write!(f, "cleanrooms-ml:TagResource"),
            CleanroomsMlActions::UnTagResource => write!(f, "cleanrooms-ml:UnTagResource"),
            CleanroomsMlActions::UpdateConfiguredAudienceModel => {
                write!(f, "cleanrooms-ml:UpdateConfiguredAudienceModel")
            }
        }
    }
}
