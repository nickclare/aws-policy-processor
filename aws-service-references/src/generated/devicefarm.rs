// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum DevicefarmActions {
    CreateDevicePool,
    CreateInstanceProfile,
    CreateNetworkProfile,
    CreateProject,
    CreateRemoteAccessSession,
    CreateTestGridProject,
    CreateTestGridUrl,
    CreateUpload,
    CreateVpceConfiguration,
    DeleteDevicePool,
    DeleteInstanceProfile,
    DeleteNetworkProfile,
    DeleteProject,
    DeleteRemoteAccessSession,
    DeleteRun,
    DeleteTestGridProject,
    DeleteUpload,
    DeleteVpceConfiguration,
    GetAccountSettings,
    GetDevice,
    GetDeviceInstance,
    GetDevicePool,
    GetDevicePoolCompatibility,
    GetInstanceProfile,
    GetJob,
    GetNetworkProfile,
    GetOfferingStatus,
    GetProject,
    GetRemoteAccessSession,
    GetRun,
    GetSuite,
    GetTest,
    GetTestGridProject,
    GetTestGridSession,
    GetUpload,
    GetVpceConfiguration,
    InstallToRemoteAccessSession,
    ListArtifacts,
    ListDeviceInstances,
    ListDevicePools,
    ListDevices,
    ListInstanceProfiles,
    ListJobs,
    ListNetworkProfiles,
    ListOfferingPromotions,
    ListOfferingTransactions,
    ListOfferings,
    ListProjects,
    ListRemoteAccessSessions,
    ListRuns,
    ListSamples,
    ListSuites,
    ListTagsForResource,
    ListTestGridProjects,
    ListTestGridSessionActions,
    ListTestGridSessionArtifacts,
    ListTestGridSessions,
    ListTests,
    ListUniqueProblems,
    ListUploads,
    ListVpceConfigurations,
    PurchaseOffering,
    RenewOffering,
    ScheduleRun,
    StopJob,
    StopRemoteAccessSession,
    StopRun,
    TagResource,
    UntagResource,
    UpdateDeviceInstance,
    UpdateDevicePool,
    UpdateInstanceProfile,
    UpdateNetworkProfile,
    UpdateProject,
    UpdateTestGridProject,
    UpdateUpload,
    UpdateVpceConfiguration,
}
impl std::fmt::Display for DevicefarmActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DevicefarmActions::CreateDevicePool => write!(f, "devicefarm:CreateDevicePool"),
            DevicefarmActions::CreateInstanceProfile => {
                write!(f, "devicefarm:CreateInstanceProfile")
            }
            DevicefarmActions::CreateNetworkProfile => write!(f, "devicefarm:CreateNetworkProfile"),
            DevicefarmActions::CreateProject => write!(f, "devicefarm:CreateProject"),
            DevicefarmActions::CreateRemoteAccessSession => {
                write!(f, "devicefarm:CreateRemoteAccessSession")
            }
            DevicefarmActions::CreateTestGridProject => {
                write!(f, "devicefarm:CreateTestGridProject")
            }
            DevicefarmActions::CreateTestGridUrl => write!(f, "devicefarm:CreateTestGridUrl"),
            DevicefarmActions::CreateUpload => write!(f, "devicefarm:CreateUpload"),
            DevicefarmActions::CreateVpceConfiguration => {
                write!(f, "devicefarm:CreateVPCEConfiguration")
            }
            DevicefarmActions::DeleteDevicePool => write!(f, "devicefarm:DeleteDevicePool"),
            DevicefarmActions::DeleteInstanceProfile => {
                write!(f, "devicefarm:DeleteInstanceProfile")
            }
            DevicefarmActions::DeleteNetworkProfile => write!(f, "devicefarm:DeleteNetworkProfile"),
            DevicefarmActions::DeleteProject => write!(f, "devicefarm:DeleteProject"),
            DevicefarmActions::DeleteRemoteAccessSession => {
                write!(f, "devicefarm:DeleteRemoteAccessSession")
            }
            DevicefarmActions::DeleteRun => write!(f, "devicefarm:DeleteRun"),
            DevicefarmActions::DeleteTestGridProject => {
                write!(f, "devicefarm:DeleteTestGridProject")
            }
            DevicefarmActions::DeleteUpload => write!(f, "devicefarm:DeleteUpload"),
            DevicefarmActions::DeleteVpceConfiguration => {
                write!(f, "devicefarm:DeleteVPCEConfiguration")
            }
            DevicefarmActions::GetAccountSettings => write!(f, "devicefarm:GetAccountSettings"),
            DevicefarmActions::GetDevice => write!(f, "devicefarm:GetDevice"),
            DevicefarmActions::GetDeviceInstance => write!(f, "devicefarm:GetDeviceInstance"),
            DevicefarmActions::GetDevicePool => write!(f, "devicefarm:GetDevicePool"),
            DevicefarmActions::GetDevicePoolCompatibility => {
                write!(f, "devicefarm:GetDevicePoolCompatibility")
            }
            DevicefarmActions::GetInstanceProfile => write!(f, "devicefarm:GetInstanceProfile"),
            DevicefarmActions::GetJob => write!(f, "devicefarm:GetJob"),
            DevicefarmActions::GetNetworkProfile => write!(f, "devicefarm:GetNetworkProfile"),
            DevicefarmActions::GetOfferingStatus => write!(f, "devicefarm:GetOfferingStatus"),
            DevicefarmActions::GetProject => write!(f, "devicefarm:GetProject"),
            DevicefarmActions::GetRemoteAccessSession => {
                write!(f, "devicefarm:GetRemoteAccessSession")
            }
            DevicefarmActions::GetRun => write!(f, "devicefarm:GetRun"),
            DevicefarmActions::GetSuite => write!(f, "devicefarm:GetSuite"),
            DevicefarmActions::GetTest => write!(f, "devicefarm:GetTest"),
            DevicefarmActions::GetTestGridProject => write!(f, "devicefarm:GetTestGridProject"),
            DevicefarmActions::GetTestGridSession => write!(f, "devicefarm:GetTestGridSession"),
            DevicefarmActions::GetUpload => write!(f, "devicefarm:GetUpload"),
            DevicefarmActions::GetVpceConfiguration => write!(f, "devicefarm:GetVPCEConfiguration"),
            DevicefarmActions::InstallToRemoteAccessSession => {
                write!(f, "devicefarm:InstallToRemoteAccessSession")
            }
            DevicefarmActions::ListArtifacts => write!(f, "devicefarm:ListArtifacts"),
            DevicefarmActions::ListDeviceInstances => write!(f, "devicefarm:ListDeviceInstances"),
            DevicefarmActions::ListDevicePools => write!(f, "devicefarm:ListDevicePools"),
            DevicefarmActions::ListDevices => write!(f, "devicefarm:ListDevices"),
            DevicefarmActions::ListInstanceProfiles => write!(f, "devicefarm:ListInstanceProfiles"),
            DevicefarmActions::ListJobs => write!(f, "devicefarm:ListJobs"),
            DevicefarmActions::ListNetworkProfiles => write!(f, "devicefarm:ListNetworkProfiles"),
            DevicefarmActions::ListOfferingPromotions => {
                write!(f, "devicefarm:ListOfferingPromotions")
            }
            DevicefarmActions::ListOfferingTransactions => {
                write!(f, "devicefarm:ListOfferingTransactions")
            }
            DevicefarmActions::ListOfferings => write!(f, "devicefarm:ListOfferings"),
            DevicefarmActions::ListProjects => write!(f, "devicefarm:ListProjects"),
            DevicefarmActions::ListRemoteAccessSessions => {
                write!(f, "devicefarm:ListRemoteAccessSessions")
            }
            DevicefarmActions::ListRuns => write!(f, "devicefarm:ListRuns"),
            DevicefarmActions::ListSamples => write!(f, "devicefarm:ListSamples"),
            DevicefarmActions::ListSuites => write!(f, "devicefarm:ListSuites"),
            DevicefarmActions::ListTagsForResource => write!(f, "devicefarm:ListTagsForResource"),
            DevicefarmActions::ListTestGridProjects => write!(f, "devicefarm:ListTestGridProjects"),
            DevicefarmActions::ListTestGridSessionActions => {
                write!(f, "devicefarm:ListTestGridSessionActions")
            }
            DevicefarmActions::ListTestGridSessionArtifacts => {
                write!(f, "devicefarm:ListTestGridSessionArtifacts")
            }
            DevicefarmActions::ListTestGridSessions => write!(f, "devicefarm:ListTestGridSessions"),
            DevicefarmActions::ListTests => write!(f, "devicefarm:ListTests"),
            DevicefarmActions::ListUniqueProblems => write!(f, "devicefarm:ListUniqueProblems"),
            DevicefarmActions::ListUploads => write!(f, "devicefarm:ListUploads"),
            DevicefarmActions::ListVpceConfigurations => {
                write!(f, "devicefarm:ListVPCEConfigurations")
            }
            DevicefarmActions::PurchaseOffering => write!(f, "devicefarm:PurchaseOffering"),
            DevicefarmActions::RenewOffering => write!(f, "devicefarm:RenewOffering"),
            DevicefarmActions::ScheduleRun => write!(f, "devicefarm:ScheduleRun"),
            DevicefarmActions::StopJob => write!(f, "devicefarm:StopJob"),
            DevicefarmActions::StopRemoteAccessSession => {
                write!(f, "devicefarm:StopRemoteAccessSession")
            }
            DevicefarmActions::StopRun => write!(f, "devicefarm:StopRun"),
            DevicefarmActions::TagResource => write!(f, "devicefarm:TagResource"),
            DevicefarmActions::UntagResource => write!(f, "devicefarm:UntagResource"),
            DevicefarmActions::UpdateDeviceInstance => write!(f, "devicefarm:UpdateDeviceInstance"),
            DevicefarmActions::UpdateDevicePool => write!(f, "devicefarm:UpdateDevicePool"),
            DevicefarmActions::UpdateInstanceProfile => {
                write!(f, "devicefarm:UpdateInstanceProfile")
            }
            DevicefarmActions::UpdateNetworkProfile => write!(f, "devicefarm:UpdateNetworkProfile"),
            DevicefarmActions::UpdateProject => write!(f, "devicefarm:UpdateProject"),
            DevicefarmActions::UpdateTestGridProject => {
                write!(f, "devicefarm:UpdateTestGridProject")
            }
            DevicefarmActions::UpdateUpload => write!(f, "devicefarm:UpdateUpload"),
            DevicefarmActions::UpdateVpceConfiguration => {
                write!(f, "devicefarm:UpdateVPCEConfiguration")
            }
        }
    }
}
