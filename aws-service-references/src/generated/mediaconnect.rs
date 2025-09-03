// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum MediaconnectActions {
    AddBridgeOutputs,
    AddBridgeSources,
    AddFlowMediaStreams,
    AddFlowOutputs,
    AddFlowSources,
    AddFlowVpcInterfaces,
    CreateBridge,
    CreateFlow,
    CreateGateway,
    DeleteBridge,
    DeleteFlow,
    DeleteGateway,
    DeregisterGatewayInstance,
    DescribeBridge,
    DescribeFlow,
    DescribeFlowSourceMetadata,
    DescribeFlowSourceThumbnail,
    DescribeGateway,
    DescribeGatewayInstance,
    DescribeOffering,
    DescribeReservation,
    DiscoverGatewayPollEndpoint,
    GrantFlowEntitlements,
    ListBridges,
    ListEntitlements,
    ListFlows,
    ListGatewayInstances,
    ListGateways,
    ListOfferings,
    ListReservations,
    ListTagsForResource,
    PollGateway,
    PurchaseOffering,
    RemoveBridgeOutput,
    RemoveBridgeSource,
    RemoveFlowMediaStream,
    RemoveFlowOutput,
    RemoveFlowSource,
    RemoveFlowVpcInterface,
    RevokeFlowEntitlement,
    StartFlow,
    StopFlow,
    SubmitGatewayStateChange,
    TagResource,
    UntagResource,
    UpdateBridge,
    UpdateBridgeOutput,
    UpdateBridgeSource,
    UpdateBridgeState,
    UpdateFlow,
    UpdateFlowEntitlement,
    UpdateFlowMediaStream,
    UpdateFlowOutput,
    UpdateFlowSource,
    UpdateGatewayInstance,
}
impl std::fmt::Display for MediaconnectActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MediaconnectActions::AddBridgeOutputs => write!(f, "mediaconnect:AddBridgeOutputs"),
            MediaconnectActions::AddBridgeSources => write!(f, "mediaconnect:AddBridgeSources"),
            MediaconnectActions::AddFlowMediaStreams => {
                write!(f, "mediaconnect:AddFlowMediaStreams")
            }
            MediaconnectActions::AddFlowOutputs => write!(f, "mediaconnect:AddFlowOutputs"),
            MediaconnectActions::AddFlowSources => write!(f, "mediaconnect:AddFlowSources"),
            MediaconnectActions::AddFlowVpcInterfaces => {
                write!(f, "mediaconnect:AddFlowVpcInterfaces")
            }
            MediaconnectActions::CreateBridge => write!(f, "mediaconnect:CreateBridge"),
            MediaconnectActions::CreateFlow => write!(f, "mediaconnect:CreateFlow"),
            MediaconnectActions::CreateGateway => write!(f, "mediaconnect:CreateGateway"),
            MediaconnectActions::DeleteBridge => write!(f, "mediaconnect:DeleteBridge"),
            MediaconnectActions::DeleteFlow => write!(f, "mediaconnect:DeleteFlow"),
            MediaconnectActions::DeleteGateway => write!(f, "mediaconnect:DeleteGateway"),
            MediaconnectActions::DeregisterGatewayInstance => {
                write!(f, "mediaconnect:DeregisterGatewayInstance")
            }
            MediaconnectActions::DescribeBridge => write!(f, "mediaconnect:DescribeBridge"),
            MediaconnectActions::DescribeFlow => write!(f, "mediaconnect:DescribeFlow"),
            MediaconnectActions::DescribeFlowSourceMetadata => {
                write!(f, "mediaconnect:DescribeFlowSourceMetadata")
            }
            MediaconnectActions::DescribeFlowSourceThumbnail => {
                write!(f, "mediaconnect:DescribeFlowSourceThumbnail")
            }
            MediaconnectActions::DescribeGateway => write!(f, "mediaconnect:DescribeGateway"),
            MediaconnectActions::DescribeGatewayInstance => {
                write!(f, "mediaconnect:DescribeGatewayInstance")
            }
            MediaconnectActions::DescribeOffering => write!(f, "mediaconnect:DescribeOffering"),
            MediaconnectActions::DescribeReservation => {
                write!(f, "mediaconnect:DescribeReservation")
            }
            MediaconnectActions::DiscoverGatewayPollEndpoint => {
                write!(f, "mediaconnect:DiscoverGatewayPollEndpoint")
            }
            MediaconnectActions::GrantFlowEntitlements => {
                write!(f, "mediaconnect:GrantFlowEntitlements")
            }
            MediaconnectActions::ListBridges => write!(f, "mediaconnect:ListBridges"),
            MediaconnectActions::ListEntitlements => write!(f, "mediaconnect:ListEntitlements"),
            MediaconnectActions::ListFlows => write!(f, "mediaconnect:ListFlows"),
            MediaconnectActions::ListGatewayInstances => {
                write!(f, "mediaconnect:ListGatewayInstances")
            }
            MediaconnectActions::ListGateways => write!(f, "mediaconnect:ListGateways"),
            MediaconnectActions::ListOfferings => write!(f, "mediaconnect:ListOfferings"),
            MediaconnectActions::ListReservations => write!(f, "mediaconnect:ListReservations"),
            MediaconnectActions::ListTagsForResource => {
                write!(f, "mediaconnect:ListTagsForResource")
            }
            MediaconnectActions::PollGateway => write!(f, "mediaconnect:PollGateway"),
            MediaconnectActions::PurchaseOffering => write!(f, "mediaconnect:PurchaseOffering"),
            MediaconnectActions::RemoveBridgeOutput => write!(f, "mediaconnect:RemoveBridgeOutput"),
            MediaconnectActions::RemoveBridgeSource => write!(f, "mediaconnect:RemoveBridgeSource"),
            MediaconnectActions::RemoveFlowMediaStream => {
                write!(f, "mediaconnect:RemoveFlowMediaStream")
            }
            MediaconnectActions::RemoveFlowOutput => write!(f, "mediaconnect:RemoveFlowOutput"),
            MediaconnectActions::RemoveFlowSource => write!(f, "mediaconnect:RemoveFlowSource"),
            MediaconnectActions::RemoveFlowVpcInterface => {
                write!(f, "mediaconnect:RemoveFlowVpcInterface")
            }
            MediaconnectActions::RevokeFlowEntitlement => {
                write!(f, "mediaconnect:RevokeFlowEntitlement")
            }
            MediaconnectActions::StartFlow => write!(f, "mediaconnect:StartFlow"),
            MediaconnectActions::StopFlow => write!(f, "mediaconnect:StopFlow"),
            MediaconnectActions::SubmitGatewayStateChange => {
                write!(f, "mediaconnect:SubmitGatewayStateChange")
            }
            MediaconnectActions::TagResource => write!(f, "mediaconnect:TagResource"),
            MediaconnectActions::UntagResource => write!(f, "mediaconnect:UntagResource"),
            MediaconnectActions::UpdateBridge => write!(f, "mediaconnect:UpdateBridge"),
            MediaconnectActions::UpdateBridgeOutput => write!(f, "mediaconnect:UpdateBridgeOutput"),
            MediaconnectActions::UpdateBridgeSource => write!(f, "mediaconnect:UpdateBridgeSource"),
            MediaconnectActions::UpdateBridgeState => write!(f, "mediaconnect:UpdateBridgeState"),
            MediaconnectActions::UpdateFlow => write!(f, "mediaconnect:UpdateFlow"),
            MediaconnectActions::UpdateFlowEntitlement => {
                write!(f, "mediaconnect:UpdateFlowEntitlement")
            }
            MediaconnectActions::UpdateFlowMediaStream => {
                write!(f, "mediaconnect:UpdateFlowMediaStream")
            }
            MediaconnectActions::UpdateFlowOutput => write!(f, "mediaconnect:UpdateFlowOutput"),
            MediaconnectActions::UpdateFlowSource => write!(f, "mediaconnect:UpdateFlowSource"),
            MediaconnectActions::UpdateGatewayInstance => {
                write!(f, "mediaconnect:UpdateGatewayInstance")
            }
        }
    }
}
