// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum NimbleActions {
    AcceptEulas,
    CreateLaunchProfile,
    CreateStreamingImage,
    CreateStreamingSession,
    CreateStreamingSessionStream,
    CreateStudio,
    CreateStudioComponent,
    DeleteLaunchProfile,
    DeleteLaunchProfileMember,
    DeleteStreamingImage,
    DeleteStreamingSession,
    DeleteStudio,
    DeleteStudioComponent,
    DeleteStudioMember,
    GetEula,
    GetFeatureMap,
    GetLaunchProfile,
    GetLaunchProfileDetails,
    GetLaunchProfileInitialization,
    GetLaunchProfileMember,
    GetStreamingImage,
    GetStreamingSession,
    GetStreamingSessionBackup,
    GetStreamingSessionStream,
    GetStudio,
    GetStudioComponent,
    GetStudioMember,
    ListEulaAcceptances,
    ListEulas,
    ListLaunchProfileMembers,
    ListLaunchProfiles,
    ListStreamingImages,
    ListStreamingSessionBackups,
    ListStreamingSessions,
    ListStudioComponents,
    ListStudioMembers,
    ListStudios,
    ListTagsForResource,
    PutLaunchProfileMembers,
    PutStudioLogEvents,
    PutStudioMembers,
    StartStreamingSession,
    StartStudioSsoConfigurationRepair,
    StopStreamingSession,
    TagResource,
    UntagResource,
    UpdateLaunchProfile,
    UpdateLaunchProfileMember,
    UpdateStreamingImage,
    UpdateStudio,
    UpdateStudioComponent,
}
impl std::fmt::Display for NimbleActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NimbleActions::AcceptEulas => write!(f, "nimble:AcceptEulas"),
            NimbleActions::CreateLaunchProfile => write!(f, "nimble:CreateLaunchProfile"),
            NimbleActions::CreateStreamingImage => write!(f, "nimble:CreateStreamingImage"),
            NimbleActions::CreateStreamingSession => write!(f, "nimble:CreateStreamingSession"),
            NimbleActions::CreateStreamingSessionStream => {
                write!(f, "nimble:CreateStreamingSessionStream")
            }
            NimbleActions::CreateStudio => write!(f, "nimble:CreateStudio"),
            NimbleActions::CreateStudioComponent => write!(f, "nimble:CreateStudioComponent"),
            NimbleActions::DeleteLaunchProfile => write!(f, "nimble:DeleteLaunchProfile"),
            NimbleActions::DeleteLaunchProfileMember => {
                write!(f, "nimble:DeleteLaunchProfileMember")
            }
            NimbleActions::DeleteStreamingImage => write!(f, "nimble:DeleteStreamingImage"),
            NimbleActions::DeleteStreamingSession => write!(f, "nimble:DeleteStreamingSession"),
            NimbleActions::DeleteStudio => write!(f, "nimble:DeleteStudio"),
            NimbleActions::DeleteStudioComponent => write!(f, "nimble:DeleteStudioComponent"),
            NimbleActions::DeleteStudioMember => write!(f, "nimble:DeleteStudioMember"),
            NimbleActions::GetEula => write!(f, "nimble:GetEula"),
            NimbleActions::GetFeatureMap => write!(f, "nimble:GetFeatureMap"),
            NimbleActions::GetLaunchProfile => write!(f, "nimble:GetLaunchProfile"),
            NimbleActions::GetLaunchProfileDetails => write!(f, "nimble:GetLaunchProfileDetails"),
            NimbleActions::GetLaunchProfileInitialization => {
                write!(f, "nimble:GetLaunchProfileInitialization")
            }
            NimbleActions::GetLaunchProfileMember => write!(f, "nimble:GetLaunchProfileMember"),
            NimbleActions::GetStreamingImage => write!(f, "nimble:GetStreamingImage"),
            NimbleActions::GetStreamingSession => write!(f, "nimble:GetStreamingSession"),
            NimbleActions::GetStreamingSessionBackup => {
                write!(f, "nimble:GetStreamingSessionBackup")
            }
            NimbleActions::GetStreamingSessionStream => {
                write!(f, "nimble:GetStreamingSessionStream")
            }
            NimbleActions::GetStudio => write!(f, "nimble:GetStudio"),
            NimbleActions::GetStudioComponent => write!(f, "nimble:GetStudioComponent"),
            NimbleActions::GetStudioMember => write!(f, "nimble:GetStudioMember"),
            NimbleActions::ListEulaAcceptances => write!(f, "nimble:ListEulaAcceptances"),
            NimbleActions::ListEulas => write!(f, "nimble:ListEulas"),
            NimbleActions::ListLaunchProfileMembers => write!(f, "nimble:ListLaunchProfileMembers"),
            NimbleActions::ListLaunchProfiles => write!(f, "nimble:ListLaunchProfiles"),
            NimbleActions::ListStreamingImages => write!(f, "nimble:ListStreamingImages"),
            NimbleActions::ListStreamingSessionBackups => {
                write!(f, "nimble:ListStreamingSessionBackups")
            }
            NimbleActions::ListStreamingSessions => write!(f, "nimble:ListStreamingSessions"),
            NimbleActions::ListStudioComponents => write!(f, "nimble:ListStudioComponents"),
            NimbleActions::ListStudioMembers => write!(f, "nimble:ListStudioMembers"),
            NimbleActions::ListStudios => write!(f, "nimble:ListStudios"),
            NimbleActions::ListTagsForResource => write!(f, "nimble:ListTagsForResource"),
            NimbleActions::PutLaunchProfileMembers => write!(f, "nimble:PutLaunchProfileMembers"),
            NimbleActions::PutStudioLogEvents => write!(f, "nimble:PutStudioLogEvents"),
            NimbleActions::PutStudioMembers => write!(f, "nimble:PutStudioMembers"),
            NimbleActions::StartStreamingSession => write!(f, "nimble:StartStreamingSession"),
            NimbleActions::StartStudioSsoConfigurationRepair => {
                write!(f, "nimble:StartStudioSSOConfigurationRepair")
            }
            NimbleActions::StopStreamingSession => write!(f, "nimble:StopStreamingSession"),
            NimbleActions::TagResource => write!(f, "nimble:TagResource"),
            NimbleActions::UntagResource => write!(f, "nimble:UntagResource"),
            NimbleActions::UpdateLaunchProfile => write!(f, "nimble:UpdateLaunchProfile"),
            NimbleActions::UpdateLaunchProfileMember => {
                write!(f, "nimble:UpdateLaunchProfileMember")
            }
            NimbleActions::UpdateStreamingImage => write!(f, "nimble:UpdateStreamingImage"),
            NimbleActions::UpdateStudio => write!(f, "nimble:UpdateStudio"),
            NimbleActions::UpdateStudioComponent => write!(f, "nimble:UpdateStudioComponent"),
        }
    }
}
