// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum GameliftstreamsActions {
    AddStreamGroupLocations,
    AssociateApplications,
    CreateApplication,
    CreateStreamGroup,
    CreateStreamSessionConnection,
    DeleteApplication,
    DeleteStreamGroup,
    DisassociateApplications,
    ExportStreamSessionFiles,
    GetApplication,
    GetStreamGroup,
    GetStreamSession,
    ListApplications,
    ListStreamGroups,
    ListStreamSessions,
    ListStreamSessionsByAccount,
    ListTagsForResource,
    RemoveStreamGroupLocations,
    StartStreamSession,
    TagResource,
    TerminateStreamSession,
    UntagResource,
    UpdateApplication,
    UpdateStreamGroup,
}
impl std::fmt::Display for GameliftstreamsActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GameliftstreamsActions::AddStreamGroupLocations => {
                write!(f, "gameliftstreams:AddStreamGroupLocations")
            }
            GameliftstreamsActions::AssociateApplications => {
                write!(f, "gameliftstreams:AssociateApplications")
            }
            GameliftstreamsActions::CreateApplication => {
                write!(f, "gameliftstreams:CreateApplication")
            }
            GameliftstreamsActions::CreateStreamGroup => {
                write!(f, "gameliftstreams:CreateStreamGroup")
            }
            GameliftstreamsActions::CreateStreamSessionConnection => {
                write!(f, "gameliftstreams:CreateStreamSessionConnection")
            }
            GameliftstreamsActions::DeleteApplication => {
                write!(f, "gameliftstreams:DeleteApplication")
            }
            GameliftstreamsActions::DeleteStreamGroup => {
                write!(f, "gameliftstreams:DeleteStreamGroup")
            }
            GameliftstreamsActions::DisassociateApplications => {
                write!(f, "gameliftstreams:DisassociateApplications")
            }
            GameliftstreamsActions::ExportStreamSessionFiles => {
                write!(f, "gameliftstreams:ExportStreamSessionFiles")
            }
            GameliftstreamsActions::GetApplication => write!(f, "gameliftstreams:GetApplication"),
            GameliftstreamsActions::GetStreamGroup => write!(f, "gameliftstreams:GetStreamGroup"),
            GameliftstreamsActions::GetStreamSession => {
                write!(f, "gameliftstreams:GetStreamSession")
            }
            GameliftstreamsActions::ListApplications => {
                write!(f, "gameliftstreams:ListApplications")
            }
            GameliftstreamsActions::ListStreamGroups => {
                write!(f, "gameliftstreams:ListStreamGroups")
            }
            GameliftstreamsActions::ListStreamSessions => {
                write!(f, "gameliftstreams:ListStreamSessions")
            }
            GameliftstreamsActions::ListStreamSessionsByAccount => {
                write!(f, "gameliftstreams:ListStreamSessionsByAccount")
            }
            GameliftstreamsActions::ListTagsForResource => {
                write!(f, "gameliftstreams:ListTagsForResource")
            }
            GameliftstreamsActions::RemoveStreamGroupLocations => {
                write!(f, "gameliftstreams:RemoveStreamGroupLocations")
            }
            GameliftstreamsActions::StartStreamSession => {
                write!(f, "gameliftstreams:StartStreamSession")
            }
            GameliftstreamsActions::TagResource => write!(f, "gameliftstreams:TagResource"),
            GameliftstreamsActions::TerminateStreamSession => {
                write!(f, "gameliftstreams:TerminateStreamSession")
            }
            GameliftstreamsActions::UntagResource => write!(f, "gameliftstreams:UntagResource"),
            GameliftstreamsActions::UpdateApplication => {
                write!(f, "gameliftstreams:UpdateApplication")
            }
            GameliftstreamsActions::UpdateStreamGroup => {
                write!(f, "gameliftstreams:UpdateStreamGroup")
            }
        }
    }
}
