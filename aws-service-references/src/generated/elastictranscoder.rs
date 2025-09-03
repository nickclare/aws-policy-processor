// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum ElastictranscoderActions {
    CancelJob,
    CreateJob,
    CreatePipeline,
    CreatePreset,
    DeletePipeline,
    DeletePreset,
    ListJobsByPipeline,
    ListJobsByStatus,
    ListPipelines,
    ListPresets,
    ReadJob,
    ReadPipeline,
    ReadPreset,
    TestRole,
    UpdatePipeline,
    UpdatePipelineNotifications,
    UpdatePipelineStatus,
}
impl std::fmt::Display for ElastictranscoderActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ElastictranscoderActions::CancelJob => write!(f, "elastictranscoder:CancelJob"),
            ElastictranscoderActions::CreateJob => write!(f, "elastictranscoder:CreateJob"),
            ElastictranscoderActions::CreatePipeline => {
                write!(f, "elastictranscoder:CreatePipeline")
            }
            ElastictranscoderActions::CreatePreset => write!(f, "elastictranscoder:CreatePreset"),
            ElastictranscoderActions::DeletePipeline => {
                write!(f, "elastictranscoder:DeletePipeline")
            }
            ElastictranscoderActions::DeletePreset => write!(f, "elastictranscoder:DeletePreset"),
            ElastictranscoderActions::ListJobsByPipeline => {
                write!(f, "elastictranscoder:ListJobsByPipeline")
            }
            ElastictranscoderActions::ListJobsByStatus => {
                write!(f, "elastictranscoder:ListJobsByStatus")
            }
            ElastictranscoderActions::ListPipelines => write!(f, "elastictranscoder:ListPipelines"),
            ElastictranscoderActions::ListPresets => write!(f, "elastictranscoder:ListPresets"),
            ElastictranscoderActions::ReadJob => write!(f, "elastictranscoder:ReadJob"),
            ElastictranscoderActions::ReadPipeline => write!(f, "elastictranscoder:ReadPipeline"),
            ElastictranscoderActions::ReadPreset => write!(f, "elastictranscoder:ReadPreset"),
            ElastictranscoderActions::TestRole => write!(f, "elastictranscoder:TestRole"),
            ElastictranscoderActions::UpdatePipeline => {
                write!(f, "elastictranscoder:UpdatePipeline")
            }
            ElastictranscoderActions::UpdatePipelineNotifications => {
                write!(f, "elastictranscoder:UpdatePipelineNotifications")
            }
            ElastictranscoderActions::UpdatePipelineStatus => {
                write!(f, "elastictranscoder:UpdatePipelineStatus")
            }
        }
    }
}
