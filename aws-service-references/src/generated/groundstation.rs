// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum GroundstationActions {
    CancelContact,
    CreateConfig,
    CreateDataflowEndpointGroup,
    CreateEphemeris,
    CreateMissionProfile,
    DeleteConfig,
    DeleteDataflowEndpointGroup,
    DeleteEphemeris,
    DeleteMissionProfile,
    DescribeContact,
    DescribeEphemeris,
    GetAgentConfiguration,
    GetConfig,
    GetDataflowEndpointGroup,
    GetMinuteUsage,
    GetMissionProfile,
    GetSatellite,
    ListConfigs,
    ListContacts,
    ListDataflowEndpointGroups,
    ListEphemerides,
    ListGroundStations,
    ListMissionProfiles,
    ListSatellites,
    ListTagsForResource,
    RegisterAgent,
    ReserveContact,
    TagResource,
    UntagResource,
    UpdateAgentStatus,
    UpdateConfig,
    UpdateEphemeris,
    UpdateMissionProfile,
}
impl std::fmt::Display for GroundstationActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GroundstationActions::CancelContact => write!(f, "groundstation:CancelContact"),
            GroundstationActions::CreateConfig => write!(f, "groundstation:CreateConfig"),
            GroundstationActions::CreateDataflowEndpointGroup => {
                write!(f, "groundstation:CreateDataflowEndpointGroup")
            }
            GroundstationActions::CreateEphemeris => write!(f, "groundstation:CreateEphemeris"),
            GroundstationActions::CreateMissionProfile => {
                write!(f, "groundstation:CreateMissionProfile")
            }
            GroundstationActions::DeleteConfig => write!(f, "groundstation:DeleteConfig"),
            GroundstationActions::DeleteDataflowEndpointGroup => {
                write!(f, "groundstation:DeleteDataflowEndpointGroup")
            }
            GroundstationActions::DeleteEphemeris => write!(f, "groundstation:DeleteEphemeris"),
            GroundstationActions::DeleteMissionProfile => {
                write!(f, "groundstation:DeleteMissionProfile")
            }
            GroundstationActions::DescribeContact => write!(f, "groundstation:DescribeContact"),
            GroundstationActions::DescribeEphemeris => write!(f, "groundstation:DescribeEphemeris"),
            GroundstationActions::GetAgentConfiguration => {
                write!(f, "groundstation:GetAgentConfiguration")
            }
            GroundstationActions::GetConfig => write!(f, "groundstation:GetConfig"),
            GroundstationActions::GetDataflowEndpointGroup => {
                write!(f, "groundstation:GetDataflowEndpointGroup")
            }
            GroundstationActions::GetMinuteUsage => write!(f, "groundstation:GetMinuteUsage"),
            GroundstationActions::GetMissionProfile => write!(f, "groundstation:GetMissionProfile"),
            GroundstationActions::GetSatellite => write!(f, "groundstation:GetSatellite"),
            GroundstationActions::ListConfigs => write!(f, "groundstation:ListConfigs"),
            GroundstationActions::ListContacts => write!(f, "groundstation:ListContacts"),
            GroundstationActions::ListDataflowEndpointGroups => {
                write!(f, "groundstation:ListDataflowEndpointGroups")
            }
            GroundstationActions::ListEphemerides => write!(f, "groundstation:ListEphemerides"),
            GroundstationActions::ListGroundStations => {
                write!(f, "groundstation:ListGroundStations")
            }
            GroundstationActions::ListMissionProfiles => {
                write!(f, "groundstation:ListMissionProfiles")
            }
            GroundstationActions::ListSatellites => write!(f, "groundstation:ListSatellites"),
            GroundstationActions::ListTagsForResource => {
                write!(f, "groundstation:ListTagsForResource")
            }
            GroundstationActions::RegisterAgent => write!(f, "groundstation:RegisterAgent"),
            GroundstationActions::ReserveContact => write!(f, "groundstation:ReserveContact"),
            GroundstationActions::TagResource => write!(f, "groundstation:TagResource"),
            GroundstationActions::UntagResource => write!(f, "groundstation:UntagResource"),
            GroundstationActions::UpdateAgentStatus => write!(f, "groundstation:UpdateAgentStatus"),
            GroundstationActions::UpdateConfig => write!(f, "groundstation:UpdateConfig"),
            GroundstationActions::UpdateEphemeris => write!(f, "groundstation:UpdateEphemeris"),
            GroundstationActions::UpdateMissionProfile => {
                write!(f, "groundstation:UpdateMissionProfile")
            }
        }
    }
}
