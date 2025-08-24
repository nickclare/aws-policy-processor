// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum QdeveloperActions {
    ExportArtifact,
    ImportArtifact,
    ListTagsForResource,
    StartAgentSession,
    TagResource,
    TransformCode,
    UntagResource,
}
impl std::fmt::Display for QdeveloperActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QdeveloperActions::ExportArtifact => write!(f, "qdeveloper:ExportArtifact"),
            QdeveloperActions::ImportArtifact => write!(f, "qdeveloper:ImportArtifact"),
            QdeveloperActions::ListTagsForResource => write!(f, "qdeveloper:ListTagsForResource"),
            QdeveloperActions::StartAgentSession => write!(f, "qdeveloper:StartAgentSession"),
            QdeveloperActions::TagResource => write!(f, "qdeveloper:TagResource"),
            QdeveloperActions::TransformCode => write!(f, "qdeveloper:TransformCode"),
            QdeveloperActions::UntagResource => write!(f, "qdeveloper:UntagResource"),
        }
    }
}
