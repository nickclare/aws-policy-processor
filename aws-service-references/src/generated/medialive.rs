// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum MedialiveActions {
    AcceptInputDeviceTransfer,
    BatchDelete,
    BatchStart,
    BatchStop,
    BatchUpdateSchedule,
    CancelInputDeviceTransfer,
    ClaimDevice,
    CreateChannel,
    CreateChannelPlacementGroup,
    CreateCloudWatchAlarmTemplate,
    CreateCloudWatchAlarmTemplateGroup,
    CreateCluster,
    CreateEventBridgeRuleTemplate,
    CreateEventBridgeRuleTemplateGroup,
    CreateInput,
    CreateInputSecurityGroup,
    CreateMultiplex,
    CreateMultiplexProgram,
    CreateNetwork,
    CreateNode,
    CreateNodeRegistrationScript,
    CreatePartnerInput,
    CreateSdiSource,
    CreateSignalMap,
    CreateTags,
    DeleteChannel,
    DeleteChannelPlacementGroup,
    DeleteCloudWatchAlarmTemplate,
    DeleteCloudWatchAlarmTemplateGroup,
    DeleteCluster,
    DeleteEventBridgeRuleTemplate,
    DeleteEventBridgeRuleTemplateGroup,
    DeleteInput,
    DeleteInputSecurityGroup,
    DeleteMultiplex,
    DeleteMultiplexProgram,
    DeleteNetwork,
    DeleteNode,
    DeleteReservation,
    DeleteSchedule,
    DeleteSdiSource,
    DeleteSignalMap,
    DeleteTags,
    DescribeAccountConfiguration,
    DescribeChannel,
    DescribeChannelPlacementGroup,
    DescribeCluster,
    DescribeInput,
    DescribeInputDevice,
    DescribeInputDeviceThumbnail,
    DescribeInputSecurityGroup,
    DescribeMultiplex,
    DescribeMultiplexProgram,
    DescribeNetwork,
    DescribeNode,
    DescribeOffering,
    DescribeReservation,
    DescribeSchedule,
    DescribeSdiSource,
    DescribeThumbnails,
    GetCloudWatchAlarmTemplate,
    GetCloudWatchAlarmTemplateGroup,
    GetEventBridgeRuleTemplate,
    GetEventBridgeRuleTemplateGroup,
    GetSignalMap,
    ListChannelPlacementGroups,
    ListChannels,
    ListCloudWatchAlarmTemplateGroups,
    ListCloudWatchAlarmTemplates,
    ListClusters,
    ListEventBridgeRuleTemplateGroups,
    ListEventBridgeRuleTemplates,
    ListInputDeviceTransfers,
    ListInputDevices,
    ListInputSecurityGroups,
    ListInputs,
    ListMultiplexPrograms,
    ListMultiplexes,
    ListNetworks,
    ListNodes,
    ListOfferings,
    ListReservations,
    ListSdiSources,
    ListSignalMaps,
    ListTagsForResource,
    ListVersions,
    PollAnywhere,
    PurchaseOffering,
    RebootInputDevice,
    RejectInputDeviceTransfer,
    RestartChannelPipelines,
    StartChannel,
    StartDeleteMonitorDeployment,
    StartInputDevice,
    StartInputDeviceMaintenanceWindow,
    StartMonitorDeployment,
    StartMultiplex,
    StartUpdateSignalMap,
    StopChannel,
    StopInputDevice,
    StopMultiplex,
    SubmitAnywhereStateChange,
    TransferInputDevice,
    UpdateAccountConfiguration,
    UpdateChannel,
    UpdateChannelClass,
    UpdateChannelPlacementGroup,
    UpdateCloudWatchAlarmTemplate,
    UpdateCloudWatchAlarmTemplateGroup,
    UpdateCluster,
    UpdateEventBridgeRuleTemplate,
    UpdateEventBridgeRuleTemplateGroup,
    UpdateInput,
    UpdateInputDevice,
    UpdateInputSecurityGroup,
    UpdateMultiplex,
    UpdateMultiplexProgram,
    UpdateNetwork,
    UpdateNode,
    UpdateNodeState,
    UpdateReservation,
    UpdateSdiSource,
}
impl std::fmt::Display for MedialiveActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MedialiveActions::AcceptInputDeviceTransfer => {
                write!(f, "medialive:AcceptInputDeviceTransfer")
            }
            MedialiveActions::BatchDelete => write!(f, "medialive:BatchDelete"),
            MedialiveActions::BatchStart => write!(f, "medialive:BatchStart"),
            MedialiveActions::BatchStop => write!(f, "medialive:BatchStop"),
            MedialiveActions::BatchUpdateSchedule => write!(f, "medialive:BatchUpdateSchedule"),
            MedialiveActions::CancelInputDeviceTransfer => {
                write!(f, "medialive:CancelInputDeviceTransfer")
            }
            MedialiveActions::ClaimDevice => write!(f, "medialive:ClaimDevice"),
            MedialiveActions::CreateChannel => write!(f, "medialive:CreateChannel"),
            MedialiveActions::CreateChannelPlacementGroup => {
                write!(f, "medialive:CreateChannelPlacementGroup")
            }
            MedialiveActions::CreateCloudWatchAlarmTemplate => {
                write!(f, "medialive:CreateCloudWatchAlarmTemplate")
            }
            MedialiveActions::CreateCloudWatchAlarmTemplateGroup => {
                write!(f, "medialive:CreateCloudWatchAlarmTemplateGroup")
            }
            MedialiveActions::CreateCluster => write!(f, "medialive:CreateCluster"),
            MedialiveActions::CreateEventBridgeRuleTemplate => {
                write!(f, "medialive:CreateEventBridgeRuleTemplate")
            }
            MedialiveActions::CreateEventBridgeRuleTemplateGroup => {
                write!(f, "medialive:CreateEventBridgeRuleTemplateGroup")
            }
            MedialiveActions::CreateInput => write!(f, "medialive:CreateInput"),
            MedialiveActions::CreateInputSecurityGroup => {
                write!(f, "medialive:CreateInputSecurityGroup")
            }
            MedialiveActions::CreateMultiplex => write!(f, "medialive:CreateMultiplex"),
            MedialiveActions::CreateMultiplexProgram => {
                write!(f, "medialive:CreateMultiplexProgram")
            }
            MedialiveActions::CreateNetwork => write!(f, "medialive:CreateNetwork"),
            MedialiveActions::CreateNode => write!(f, "medialive:CreateNode"),
            MedialiveActions::CreateNodeRegistrationScript => {
                write!(f, "medialive:CreateNodeRegistrationScript")
            }
            MedialiveActions::CreatePartnerInput => write!(f, "medialive:CreatePartnerInput"),
            MedialiveActions::CreateSdiSource => write!(f, "medialive:CreateSdiSource"),
            MedialiveActions::CreateSignalMap => write!(f, "medialive:CreateSignalMap"),
            MedialiveActions::CreateTags => write!(f, "medialive:CreateTags"),
            MedialiveActions::DeleteChannel => write!(f, "medialive:DeleteChannel"),
            MedialiveActions::DeleteChannelPlacementGroup => {
                write!(f, "medialive:DeleteChannelPlacementGroup")
            }
            MedialiveActions::DeleteCloudWatchAlarmTemplate => {
                write!(f, "medialive:DeleteCloudWatchAlarmTemplate")
            }
            MedialiveActions::DeleteCloudWatchAlarmTemplateGroup => {
                write!(f, "medialive:DeleteCloudWatchAlarmTemplateGroup")
            }
            MedialiveActions::DeleteCluster => write!(f, "medialive:DeleteCluster"),
            MedialiveActions::DeleteEventBridgeRuleTemplate => {
                write!(f, "medialive:DeleteEventBridgeRuleTemplate")
            }
            MedialiveActions::DeleteEventBridgeRuleTemplateGroup => {
                write!(f, "medialive:DeleteEventBridgeRuleTemplateGroup")
            }
            MedialiveActions::DeleteInput => write!(f, "medialive:DeleteInput"),
            MedialiveActions::DeleteInputSecurityGroup => {
                write!(f, "medialive:DeleteInputSecurityGroup")
            }
            MedialiveActions::DeleteMultiplex => write!(f, "medialive:DeleteMultiplex"),
            MedialiveActions::DeleteMultiplexProgram => {
                write!(f, "medialive:DeleteMultiplexProgram")
            }
            MedialiveActions::DeleteNetwork => write!(f, "medialive:DeleteNetwork"),
            MedialiveActions::DeleteNode => write!(f, "medialive:DeleteNode"),
            MedialiveActions::DeleteReservation => write!(f, "medialive:DeleteReservation"),
            MedialiveActions::DeleteSchedule => write!(f, "medialive:DeleteSchedule"),
            MedialiveActions::DeleteSdiSource => write!(f, "medialive:DeleteSdiSource"),
            MedialiveActions::DeleteSignalMap => write!(f, "medialive:DeleteSignalMap"),
            MedialiveActions::DeleteTags => write!(f, "medialive:DeleteTags"),
            MedialiveActions::DescribeAccountConfiguration => {
                write!(f, "medialive:DescribeAccountConfiguration")
            }
            MedialiveActions::DescribeChannel => write!(f, "medialive:DescribeChannel"),
            MedialiveActions::DescribeChannelPlacementGroup => {
                write!(f, "medialive:DescribeChannelPlacementGroup")
            }
            MedialiveActions::DescribeCluster => write!(f, "medialive:DescribeCluster"),
            MedialiveActions::DescribeInput => write!(f, "medialive:DescribeInput"),
            MedialiveActions::DescribeInputDevice => write!(f, "medialive:DescribeInputDevice"),
            MedialiveActions::DescribeInputDeviceThumbnail => {
                write!(f, "medialive:DescribeInputDeviceThumbnail")
            }
            MedialiveActions::DescribeInputSecurityGroup => {
                write!(f, "medialive:DescribeInputSecurityGroup")
            }
            MedialiveActions::DescribeMultiplex => write!(f, "medialive:DescribeMultiplex"),
            MedialiveActions::DescribeMultiplexProgram => {
                write!(f, "medialive:DescribeMultiplexProgram")
            }
            MedialiveActions::DescribeNetwork => write!(f, "medialive:DescribeNetwork"),
            MedialiveActions::DescribeNode => write!(f, "medialive:DescribeNode"),
            MedialiveActions::DescribeOffering => write!(f, "medialive:DescribeOffering"),
            MedialiveActions::DescribeReservation => write!(f, "medialive:DescribeReservation"),
            MedialiveActions::DescribeSchedule => write!(f, "medialive:DescribeSchedule"),
            MedialiveActions::DescribeSdiSource => write!(f, "medialive:DescribeSdiSource"),
            MedialiveActions::DescribeThumbnails => write!(f, "medialive:DescribeThumbnails"),
            MedialiveActions::GetCloudWatchAlarmTemplate => {
                write!(f, "medialive:GetCloudWatchAlarmTemplate")
            }
            MedialiveActions::GetCloudWatchAlarmTemplateGroup => {
                write!(f, "medialive:GetCloudWatchAlarmTemplateGroup")
            }
            MedialiveActions::GetEventBridgeRuleTemplate => {
                write!(f, "medialive:GetEventBridgeRuleTemplate")
            }
            MedialiveActions::GetEventBridgeRuleTemplateGroup => {
                write!(f, "medialive:GetEventBridgeRuleTemplateGroup")
            }
            MedialiveActions::GetSignalMap => write!(f, "medialive:GetSignalMap"),
            MedialiveActions::ListChannelPlacementGroups => {
                write!(f, "medialive:ListChannelPlacementGroups")
            }
            MedialiveActions::ListChannels => write!(f, "medialive:ListChannels"),
            MedialiveActions::ListCloudWatchAlarmTemplateGroups => {
                write!(f, "medialive:ListCloudWatchAlarmTemplateGroups")
            }
            MedialiveActions::ListCloudWatchAlarmTemplates => {
                write!(f, "medialive:ListCloudWatchAlarmTemplates")
            }
            MedialiveActions::ListClusters => write!(f, "medialive:ListClusters"),
            MedialiveActions::ListEventBridgeRuleTemplateGroups => {
                write!(f, "medialive:ListEventBridgeRuleTemplateGroups")
            }
            MedialiveActions::ListEventBridgeRuleTemplates => {
                write!(f, "medialive:ListEventBridgeRuleTemplates")
            }
            MedialiveActions::ListInputDeviceTransfers => {
                write!(f, "medialive:ListInputDeviceTransfers")
            }
            MedialiveActions::ListInputDevices => write!(f, "medialive:ListInputDevices"),
            MedialiveActions::ListInputSecurityGroups => {
                write!(f, "medialive:ListInputSecurityGroups")
            }
            MedialiveActions::ListInputs => write!(f, "medialive:ListInputs"),
            MedialiveActions::ListMultiplexPrograms => write!(f, "medialive:ListMultiplexPrograms"),
            MedialiveActions::ListMultiplexes => write!(f, "medialive:ListMultiplexes"),
            MedialiveActions::ListNetworks => write!(f, "medialive:ListNetworks"),
            MedialiveActions::ListNodes => write!(f, "medialive:ListNodes"),
            MedialiveActions::ListOfferings => write!(f, "medialive:ListOfferings"),
            MedialiveActions::ListReservations => write!(f, "medialive:ListReservations"),
            MedialiveActions::ListSdiSources => write!(f, "medialive:ListSdiSources"),
            MedialiveActions::ListSignalMaps => write!(f, "medialive:ListSignalMaps"),
            MedialiveActions::ListTagsForResource => write!(f, "medialive:ListTagsForResource"),
            MedialiveActions::ListVersions => write!(f, "medialive:ListVersions"),
            MedialiveActions::PollAnywhere => write!(f, "medialive:PollAnywhere"),
            MedialiveActions::PurchaseOffering => write!(f, "medialive:PurchaseOffering"),
            MedialiveActions::RebootInputDevice => write!(f, "medialive:RebootInputDevice"),
            MedialiveActions::RejectInputDeviceTransfer => {
                write!(f, "medialive:RejectInputDeviceTransfer")
            }
            MedialiveActions::RestartChannelPipelines => {
                write!(f, "medialive:RestartChannelPipelines")
            }
            MedialiveActions::StartChannel => write!(f, "medialive:StartChannel"),
            MedialiveActions::StartDeleteMonitorDeployment => {
                write!(f, "medialive:StartDeleteMonitorDeployment")
            }
            MedialiveActions::StartInputDevice => write!(f, "medialive:StartInputDevice"),
            MedialiveActions::StartInputDeviceMaintenanceWindow => {
                write!(f, "medialive:StartInputDeviceMaintenanceWindow")
            }
            MedialiveActions::StartMonitorDeployment => {
                write!(f, "medialive:StartMonitorDeployment")
            }
            MedialiveActions::StartMultiplex => write!(f, "medialive:StartMultiplex"),
            MedialiveActions::StartUpdateSignalMap => write!(f, "medialive:StartUpdateSignalMap"),
            MedialiveActions::StopChannel => write!(f, "medialive:StopChannel"),
            MedialiveActions::StopInputDevice => write!(f, "medialive:StopInputDevice"),
            MedialiveActions::StopMultiplex => write!(f, "medialive:StopMultiplex"),
            MedialiveActions::SubmitAnywhereStateChange => {
                write!(f, "medialive:SubmitAnywhereStateChange")
            }
            MedialiveActions::TransferInputDevice => write!(f, "medialive:TransferInputDevice"),
            MedialiveActions::UpdateAccountConfiguration => {
                write!(f, "medialive:UpdateAccountConfiguration")
            }
            MedialiveActions::UpdateChannel => write!(f, "medialive:UpdateChannel"),
            MedialiveActions::UpdateChannelClass => write!(f, "medialive:UpdateChannelClass"),
            MedialiveActions::UpdateChannelPlacementGroup => {
                write!(f, "medialive:UpdateChannelPlacementGroup")
            }
            MedialiveActions::UpdateCloudWatchAlarmTemplate => {
                write!(f, "medialive:UpdateCloudWatchAlarmTemplate")
            }
            MedialiveActions::UpdateCloudWatchAlarmTemplateGroup => {
                write!(f, "medialive:UpdateCloudWatchAlarmTemplateGroup")
            }
            MedialiveActions::UpdateCluster => write!(f, "medialive:UpdateCluster"),
            MedialiveActions::UpdateEventBridgeRuleTemplate => {
                write!(f, "medialive:UpdateEventBridgeRuleTemplate")
            }
            MedialiveActions::UpdateEventBridgeRuleTemplateGroup => {
                write!(f, "medialive:UpdateEventBridgeRuleTemplateGroup")
            }
            MedialiveActions::UpdateInput => write!(f, "medialive:UpdateInput"),
            MedialiveActions::UpdateInputDevice => write!(f, "medialive:UpdateInputDevice"),
            MedialiveActions::UpdateInputSecurityGroup => {
                write!(f, "medialive:UpdateInputSecurityGroup")
            }
            MedialiveActions::UpdateMultiplex => write!(f, "medialive:UpdateMultiplex"),
            MedialiveActions::UpdateMultiplexProgram => {
                write!(f, "medialive:UpdateMultiplexProgram")
            }
            MedialiveActions::UpdateNetwork => write!(f, "medialive:UpdateNetwork"),
            MedialiveActions::UpdateNode => write!(f, "medialive:UpdateNode"),
            MedialiveActions::UpdateNodeState => write!(f, "medialive:UpdateNodeState"),
            MedialiveActions::UpdateReservation => write!(f, "medialive:UpdateReservation"),
            MedialiveActions::UpdateSdiSource => write!(f, "medialive:UpdateSdiSource"),
        }
    }
}
