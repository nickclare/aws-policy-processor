// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum OsisActions {
    CreatePipeline,
    DeletePipeline,
    GetPipeline,
    GetPipelineBlueprint,
    GetPipelineChangeProgress,
    Ingest,
    ListPipelineBlueprints,
    ListPipelines,
    ListTagsForResource,
    StartPipeline,
    StopPipeline,
    TagResource,
    UntagResource,
    UpdatePipeline,
    ValidatePipeline,
}
impl std::fmt::Display for OsisActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OsisActions::CreatePipeline => write!(f, "osis:CreatePipeline"),
            OsisActions::DeletePipeline => write!(f, "osis:DeletePipeline"),
            OsisActions::GetPipeline => write!(f, "osis:GetPipeline"),
            OsisActions::GetPipelineBlueprint => write!(f, "osis:GetPipelineBlueprint"),
            OsisActions::GetPipelineChangeProgress => write!(f, "osis:GetPipelineChangeProgress"),
            OsisActions::Ingest => write!(f, "osis:Ingest"),
            OsisActions::ListPipelineBlueprints => write!(f, "osis:ListPipelineBlueprints"),
            OsisActions::ListPipelines => write!(f, "osis:ListPipelines"),
            OsisActions::ListTagsForResource => write!(f, "osis:ListTagsForResource"),
            OsisActions::StartPipeline => write!(f, "osis:StartPipeline"),
            OsisActions::StopPipeline => write!(f, "osis:StopPipeline"),
            OsisActions::TagResource => write!(f, "osis:TagResource"),
            OsisActions::UntagResource => write!(f, "osis:UntagResource"),
            OsisActions::UpdatePipeline => write!(f, "osis:UpdatePipeline"),
            OsisActions::ValidatePipeline => write!(f, "osis:ValidatePipeline"),
        }
    }
}
