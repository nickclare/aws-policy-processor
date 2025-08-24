// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum IvsActions {
    BatchGetChannel,
    BatchGetStreamKey,
    BatchStartViewerSessionRevocation,
    CreateChannel,
    CreateEncoderConfiguration,
    CreateIngestConfiguration,
    CreateParticipantToken,
    CreatePlaybackRestrictionPolicy,
    CreateRecordingConfiguration,
    CreateStage,
    CreateStorageConfiguration,
    CreateStreamKey,
    DeleteChannel,
    DeleteEncoderConfiguration,
    DeleteIngestConfiguration,
    DeletePlaybackKeyPair,
    DeletePlaybackRestrictionPolicy,
    DeletePublicKey,
    DeleteRecordingConfiguration,
    DeleteStage,
    DeleteStorageConfiguration,
    DeleteStreamKey,
    DisconnectParticipant,
    GetChannel,
    GetComposition,
    GetEncoderConfiguration,
    GetIngestConfiguration,
    GetParticipant,
    GetPlaybackKeyPair,
    GetPlaybackRestrictionPolicy,
    GetPublicKey,
    GetRecordingConfiguration,
    GetStage,
    GetStageSession,
    GetStorageConfiguration,
    GetStream,
    GetStreamKey,
    GetStreamSession,
    ImportPlaybackKeyPair,
    ImportPublicKey,
    ListChannels,
    ListCompositions,
    ListEncoderConfigurations,
    ListIngestConfigurations,
    ListParticipantEvents,
    ListParticipantReplicas,
    ListParticipants,
    ListPlaybackKeyPairs,
    ListPlaybackRestrictionPolicies,
    ListPublicKeys,
    ListRecordingConfigurations,
    ListStageSessions,
    ListStages,
    ListStorageConfigurations,
    ListStreamKeys,
    ListStreamSessions,
    ListStreams,
    ListTagsForResource,
    PutMetadata,
    StartComposition,
    StartParticipantReplication,
    StartViewerSessionRevocation,
    StopComposition,
    StopParticipantReplication,
    StopStream,
    TagResource,
    UntagResource,
    UpdateChannel,
    UpdateIngestConfiguration,
    UpdatePlaybackRestrictionPolicy,
    UpdateStage,
}
impl std::fmt::Display for IvsActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IvsActions::BatchGetChannel => write!(f, "ivs:BatchGetChannel"),
            IvsActions::BatchGetStreamKey => write!(f, "ivs:BatchGetStreamKey"),
            IvsActions::BatchStartViewerSessionRevocation => {
                write!(f, "ivs:BatchStartViewerSessionRevocation")
            }
            IvsActions::CreateChannel => write!(f, "ivs:CreateChannel"),
            IvsActions::CreateEncoderConfiguration => write!(f, "ivs:CreateEncoderConfiguration"),
            IvsActions::CreateIngestConfiguration => write!(f, "ivs:CreateIngestConfiguration"),
            IvsActions::CreateParticipantToken => write!(f, "ivs:CreateParticipantToken"),
            IvsActions::CreatePlaybackRestrictionPolicy => {
                write!(f, "ivs:CreatePlaybackRestrictionPolicy")
            }
            IvsActions::CreateRecordingConfiguration => {
                write!(f, "ivs:CreateRecordingConfiguration")
            }
            IvsActions::CreateStage => write!(f, "ivs:CreateStage"),
            IvsActions::CreateStorageConfiguration => write!(f, "ivs:CreateStorageConfiguration"),
            IvsActions::CreateStreamKey => write!(f, "ivs:CreateStreamKey"),
            IvsActions::DeleteChannel => write!(f, "ivs:DeleteChannel"),
            IvsActions::DeleteEncoderConfiguration => write!(f, "ivs:DeleteEncoderConfiguration"),
            IvsActions::DeleteIngestConfiguration => write!(f, "ivs:DeleteIngestConfiguration"),
            IvsActions::DeletePlaybackKeyPair => write!(f, "ivs:DeletePlaybackKeyPair"),
            IvsActions::DeletePlaybackRestrictionPolicy => {
                write!(f, "ivs:DeletePlaybackRestrictionPolicy")
            }
            IvsActions::DeletePublicKey => write!(f, "ivs:DeletePublicKey"),
            IvsActions::DeleteRecordingConfiguration => {
                write!(f, "ivs:DeleteRecordingConfiguration")
            }
            IvsActions::DeleteStage => write!(f, "ivs:DeleteStage"),
            IvsActions::DeleteStorageConfiguration => write!(f, "ivs:DeleteStorageConfiguration"),
            IvsActions::DeleteStreamKey => write!(f, "ivs:DeleteStreamKey"),
            IvsActions::DisconnectParticipant => write!(f, "ivs:DisconnectParticipant"),
            IvsActions::GetChannel => write!(f, "ivs:GetChannel"),
            IvsActions::GetComposition => write!(f, "ivs:GetComposition"),
            IvsActions::GetEncoderConfiguration => write!(f, "ivs:GetEncoderConfiguration"),
            IvsActions::GetIngestConfiguration => write!(f, "ivs:GetIngestConfiguration"),
            IvsActions::GetParticipant => write!(f, "ivs:GetParticipant"),
            IvsActions::GetPlaybackKeyPair => write!(f, "ivs:GetPlaybackKeyPair"),
            IvsActions::GetPlaybackRestrictionPolicy => {
                write!(f, "ivs:GetPlaybackRestrictionPolicy")
            }
            IvsActions::GetPublicKey => write!(f, "ivs:GetPublicKey"),
            IvsActions::GetRecordingConfiguration => write!(f, "ivs:GetRecordingConfiguration"),
            IvsActions::GetStage => write!(f, "ivs:GetStage"),
            IvsActions::GetStageSession => write!(f, "ivs:GetStageSession"),
            IvsActions::GetStorageConfiguration => write!(f, "ivs:GetStorageConfiguration"),
            IvsActions::GetStream => write!(f, "ivs:GetStream"),
            IvsActions::GetStreamKey => write!(f, "ivs:GetStreamKey"),
            IvsActions::GetStreamSession => write!(f, "ivs:GetStreamSession"),
            IvsActions::ImportPlaybackKeyPair => write!(f, "ivs:ImportPlaybackKeyPair"),
            IvsActions::ImportPublicKey => write!(f, "ivs:ImportPublicKey"),
            IvsActions::ListChannels => write!(f, "ivs:ListChannels"),
            IvsActions::ListCompositions => write!(f, "ivs:ListCompositions"),
            IvsActions::ListEncoderConfigurations => write!(f, "ivs:ListEncoderConfigurations"),
            IvsActions::ListIngestConfigurations => write!(f, "ivs:ListIngestConfigurations"),
            IvsActions::ListParticipantEvents => write!(f, "ivs:ListParticipantEvents"),
            IvsActions::ListParticipantReplicas => write!(f, "ivs:ListParticipantReplicas"),
            IvsActions::ListParticipants => write!(f, "ivs:ListParticipants"),
            IvsActions::ListPlaybackKeyPairs => write!(f, "ivs:ListPlaybackKeyPairs"),
            IvsActions::ListPlaybackRestrictionPolicies => {
                write!(f, "ivs:ListPlaybackRestrictionPolicies")
            }
            IvsActions::ListPublicKeys => write!(f, "ivs:ListPublicKeys"),
            IvsActions::ListRecordingConfigurations => write!(f, "ivs:ListRecordingConfigurations"),
            IvsActions::ListStageSessions => write!(f, "ivs:ListStageSessions"),
            IvsActions::ListStages => write!(f, "ivs:ListStages"),
            IvsActions::ListStorageConfigurations => write!(f, "ivs:ListStorageConfigurations"),
            IvsActions::ListStreamKeys => write!(f, "ivs:ListStreamKeys"),
            IvsActions::ListStreamSessions => write!(f, "ivs:ListStreamSessions"),
            IvsActions::ListStreams => write!(f, "ivs:ListStreams"),
            IvsActions::ListTagsForResource => write!(f, "ivs:ListTagsForResource"),
            IvsActions::PutMetadata => write!(f, "ivs:PutMetadata"),
            IvsActions::StartComposition => write!(f, "ivs:StartComposition"),
            IvsActions::StartParticipantReplication => write!(f, "ivs:StartParticipantReplication"),
            IvsActions::StartViewerSessionRevocation => {
                write!(f, "ivs:StartViewerSessionRevocation")
            }
            IvsActions::StopComposition => write!(f, "ivs:StopComposition"),
            IvsActions::StopParticipantReplication => write!(f, "ivs:StopParticipantReplication"),
            IvsActions::StopStream => write!(f, "ivs:StopStream"),
            IvsActions::TagResource => write!(f, "ivs:TagResource"),
            IvsActions::UntagResource => write!(f, "ivs:UntagResource"),
            IvsActions::UpdateChannel => write!(f, "ivs:UpdateChannel"),
            IvsActions::UpdateIngestConfiguration => write!(f, "ivs:UpdateIngestConfiguration"),
            IvsActions::UpdatePlaybackRestrictionPolicy => {
                write!(f, "ivs:UpdatePlaybackRestrictionPolicy")
            }
            IvsActions::UpdateStage => write!(f, "ivs:UpdateStage"),
        }
    }
}
