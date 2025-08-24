// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum NetworkmanagerActions {
    AcceptAttachment,
    AssociateConnectPeer,
    AssociateCustomerGateway,
    AssociateLink,
    AssociateTransitGatewayConnectPeer,
    CreateConnectAttachment,
    CreateConnectPeer,
    CreateConnection,
    CreateCoreNetwork,
    CreateDevice,
    CreateDirectConnectGatewayAttachment,
    CreateGlobalNetwork,
    CreateLink,
    CreateSite,
    CreateSiteToSiteVpnAttachment,
    CreateTransitGatewayPeering,
    CreateTransitGatewayRouteTableAttachment,
    CreateVpcAttachment,
    DeleteAttachment,
    DeleteConnectPeer,
    DeleteConnection,
    DeleteCoreNetwork,
    DeleteCoreNetworkPolicyVersion,
    DeleteDevice,
    DeleteGlobalNetwork,
    DeleteLink,
    DeletePeering,
    DeleteResourcePolicy,
    DeleteSite,
    DeregisterTransitGateway,
    DescribeGlobalNetworks,
    DisassociateConnectPeer,
    DisassociateCustomerGateway,
    DisassociateLink,
    DisassociateTransitGatewayConnectPeer,
    ExecuteCoreNetworkChangeSet,
    GetConnectAttachment,
    GetConnectPeer,
    GetConnectPeerAssociations,
    GetConnections,
    GetCoreNetwork,
    GetCoreNetworkChangeEvents,
    GetCoreNetworkChangeSet,
    GetCoreNetworkPolicy,
    GetCustomerGatewayAssociations,
    GetDevices,
    GetDirectConnectGatewayAttachment,
    GetLinkAssociations,
    GetLinks,
    GetNetworkResourceCounts,
    GetNetworkResourceRelationships,
    GetNetworkResources,
    GetNetworkRoutes,
    GetNetworkTelemetry,
    GetResourcePolicy,
    GetRouteAnalysis,
    GetSiteToSiteVpnAttachment,
    GetSites,
    GetTransitGatewayConnectPeerAssociations,
    GetTransitGatewayPeering,
    GetTransitGatewayRegistrations,
    GetTransitGatewayRouteTableAttachment,
    GetVpcAttachment,
    ListAttachments,
    ListConnectPeers,
    ListCoreNetworkPolicyVersions,
    ListCoreNetworks,
    ListOrganizationServiceAccessStatus,
    ListPeerings,
    ListTagsForResource,
    PutCoreNetworkPolicy,
    PutResourcePolicy,
    RegisterTransitGateway,
    RejectAttachment,
    RestoreCoreNetworkPolicyVersion,
    StartOrganizationServiceAccessUpdate,
    StartRouteAnalysis,
    TagResource,
    UntagResource,
    UpdateConnection,
    UpdateCoreNetwork,
    UpdateDevice,
    UpdateDirectConnectGatewayAttachment,
    UpdateGlobalNetwork,
    UpdateLink,
    UpdateNetworkResourceMetadata,
    UpdateSite,
    UpdateVpcAttachment,
}
impl std::fmt::Display for NetworkmanagerActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NetworkmanagerActions::AcceptAttachment => write!(f, "networkmanager:AcceptAttachment"),
            NetworkmanagerActions::AssociateConnectPeer => {
                write!(f, "networkmanager:AssociateConnectPeer")
            }
            NetworkmanagerActions::AssociateCustomerGateway => {
                write!(f, "networkmanager:AssociateCustomerGateway")
            }
            NetworkmanagerActions::AssociateLink => write!(f, "networkmanager:AssociateLink"),
            NetworkmanagerActions::AssociateTransitGatewayConnectPeer => {
                write!(f, "networkmanager:AssociateTransitGatewayConnectPeer")
            }
            NetworkmanagerActions::CreateConnectAttachment => {
                write!(f, "networkmanager:CreateConnectAttachment")
            }
            NetworkmanagerActions::CreateConnectPeer => {
                write!(f, "networkmanager:CreateConnectPeer")
            }
            NetworkmanagerActions::CreateConnection => write!(f, "networkmanager:CreateConnection"),
            NetworkmanagerActions::CreateCoreNetwork => {
                write!(f, "networkmanager:CreateCoreNetwork")
            }
            NetworkmanagerActions::CreateDevice => write!(f, "networkmanager:CreateDevice"),
            NetworkmanagerActions::CreateDirectConnectGatewayAttachment => {
                write!(f, "networkmanager:CreateDirectConnectGatewayAttachment")
            }
            NetworkmanagerActions::CreateGlobalNetwork => {
                write!(f, "networkmanager:CreateGlobalNetwork")
            }
            NetworkmanagerActions::CreateLink => write!(f, "networkmanager:CreateLink"),
            NetworkmanagerActions::CreateSite => write!(f, "networkmanager:CreateSite"),
            NetworkmanagerActions::CreateSiteToSiteVpnAttachment => {
                write!(f, "networkmanager:CreateSiteToSiteVpnAttachment")
            }
            NetworkmanagerActions::CreateTransitGatewayPeering => {
                write!(f, "networkmanager:CreateTransitGatewayPeering")
            }
            NetworkmanagerActions::CreateTransitGatewayRouteTableAttachment => {
                write!(f, "networkmanager:CreateTransitGatewayRouteTableAttachment")
            }
            NetworkmanagerActions::CreateVpcAttachment => {
                write!(f, "networkmanager:CreateVpcAttachment")
            }
            NetworkmanagerActions::DeleteAttachment => write!(f, "networkmanager:DeleteAttachment"),
            NetworkmanagerActions::DeleteConnectPeer => {
                write!(f, "networkmanager:DeleteConnectPeer")
            }
            NetworkmanagerActions::DeleteConnection => write!(f, "networkmanager:DeleteConnection"),
            NetworkmanagerActions::DeleteCoreNetwork => {
                write!(f, "networkmanager:DeleteCoreNetwork")
            }
            NetworkmanagerActions::DeleteCoreNetworkPolicyVersion => {
                write!(f, "networkmanager:DeleteCoreNetworkPolicyVersion")
            }
            NetworkmanagerActions::DeleteDevice => write!(f, "networkmanager:DeleteDevice"),
            NetworkmanagerActions::DeleteGlobalNetwork => {
                write!(f, "networkmanager:DeleteGlobalNetwork")
            }
            NetworkmanagerActions::DeleteLink => write!(f, "networkmanager:DeleteLink"),
            NetworkmanagerActions::DeletePeering => write!(f, "networkmanager:DeletePeering"),
            NetworkmanagerActions::DeleteResourcePolicy => {
                write!(f, "networkmanager:DeleteResourcePolicy")
            }
            NetworkmanagerActions::DeleteSite => write!(f, "networkmanager:DeleteSite"),
            NetworkmanagerActions::DeregisterTransitGateway => {
                write!(f, "networkmanager:DeregisterTransitGateway")
            }
            NetworkmanagerActions::DescribeGlobalNetworks => {
                write!(f, "networkmanager:DescribeGlobalNetworks")
            }
            NetworkmanagerActions::DisassociateConnectPeer => {
                write!(f, "networkmanager:DisassociateConnectPeer")
            }
            NetworkmanagerActions::DisassociateCustomerGateway => {
                write!(f, "networkmanager:DisassociateCustomerGateway")
            }
            NetworkmanagerActions::DisassociateLink => write!(f, "networkmanager:DisassociateLink"),
            NetworkmanagerActions::DisassociateTransitGatewayConnectPeer => {
                write!(f, "networkmanager:DisassociateTransitGatewayConnectPeer")
            }
            NetworkmanagerActions::ExecuteCoreNetworkChangeSet => {
                write!(f, "networkmanager:ExecuteCoreNetworkChangeSet")
            }
            NetworkmanagerActions::GetConnectAttachment => {
                write!(f, "networkmanager:GetConnectAttachment")
            }
            NetworkmanagerActions::GetConnectPeer => write!(f, "networkmanager:GetConnectPeer"),
            NetworkmanagerActions::GetConnectPeerAssociations => {
                write!(f, "networkmanager:GetConnectPeerAssociations")
            }
            NetworkmanagerActions::GetConnections => write!(f, "networkmanager:GetConnections"),
            NetworkmanagerActions::GetCoreNetwork => write!(f, "networkmanager:GetCoreNetwork"),
            NetworkmanagerActions::GetCoreNetworkChangeEvents => {
                write!(f, "networkmanager:GetCoreNetworkChangeEvents")
            }
            NetworkmanagerActions::GetCoreNetworkChangeSet => {
                write!(f, "networkmanager:GetCoreNetworkChangeSet")
            }
            NetworkmanagerActions::GetCoreNetworkPolicy => {
                write!(f, "networkmanager:GetCoreNetworkPolicy")
            }
            NetworkmanagerActions::GetCustomerGatewayAssociations => {
                write!(f, "networkmanager:GetCustomerGatewayAssociations")
            }
            NetworkmanagerActions::GetDevices => write!(f, "networkmanager:GetDevices"),
            NetworkmanagerActions::GetDirectConnectGatewayAttachment => {
                write!(f, "networkmanager:GetDirectConnectGatewayAttachment")
            }
            NetworkmanagerActions::GetLinkAssociations => {
                write!(f, "networkmanager:GetLinkAssociations")
            }
            NetworkmanagerActions::GetLinks => write!(f, "networkmanager:GetLinks"),
            NetworkmanagerActions::GetNetworkResourceCounts => {
                write!(f, "networkmanager:GetNetworkResourceCounts")
            }
            NetworkmanagerActions::GetNetworkResourceRelationships => {
                write!(f, "networkmanager:GetNetworkResourceRelationships")
            }
            NetworkmanagerActions::GetNetworkResources => {
                write!(f, "networkmanager:GetNetworkResources")
            }
            NetworkmanagerActions::GetNetworkRoutes => write!(f, "networkmanager:GetNetworkRoutes"),
            NetworkmanagerActions::GetNetworkTelemetry => {
                write!(f, "networkmanager:GetNetworkTelemetry")
            }
            NetworkmanagerActions::GetResourcePolicy => {
                write!(f, "networkmanager:GetResourcePolicy")
            }
            NetworkmanagerActions::GetRouteAnalysis => write!(f, "networkmanager:GetRouteAnalysis"),
            NetworkmanagerActions::GetSiteToSiteVpnAttachment => {
                write!(f, "networkmanager:GetSiteToSiteVpnAttachment")
            }
            NetworkmanagerActions::GetSites => write!(f, "networkmanager:GetSites"),
            NetworkmanagerActions::GetTransitGatewayConnectPeerAssociations => {
                write!(f, "networkmanager:GetTransitGatewayConnectPeerAssociations")
            }
            NetworkmanagerActions::GetTransitGatewayPeering => {
                write!(f, "networkmanager:GetTransitGatewayPeering")
            }
            NetworkmanagerActions::GetTransitGatewayRegistrations => {
                write!(f, "networkmanager:GetTransitGatewayRegistrations")
            }
            NetworkmanagerActions::GetTransitGatewayRouteTableAttachment => {
                write!(f, "networkmanager:GetTransitGatewayRouteTableAttachment")
            }
            NetworkmanagerActions::GetVpcAttachment => write!(f, "networkmanager:GetVpcAttachment"),
            NetworkmanagerActions::ListAttachments => write!(f, "networkmanager:ListAttachments"),
            NetworkmanagerActions::ListConnectPeers => write!(f, "networkmanager:ListConnectPeers"),
            NetworkmanagerActions::ListCoreNetworkPolicyVersions => {
                write!(f, "networkmanager:ListCoreNetworkPolicyVersions")
            }
            NetworkmanagerActions::ListCoreNetworks => write!(f, "networkmanager:ListCoreNetworks"),
            NetworkmanagerActions::ListOrganizationServiceAccessStatus => {
                write!(f, "networkmanager:ListOrganizationServiceAccessStatus")
            }
            NetworkmanagerActions::ListPeerings => write!(f, "networkmanager:ListPeerings"),
            NetworkmanagerActions::ListTagsForResource => {
                write!(f, "networkmanager:ListTagsForResource")
            }
            NetworkmanagerActions::PutCoreNetworkPolicy => {
                write!(f, "networkmanager:PutCoreNetworkPolicy")
            }
            NetworkmanagerActions::PutResourcePolicy => {
                write!(f, "networkmanager:PutResourcePolicy")
            }
            NetworkmanagerActions::RegisterTransitGateway => {
                write!(f, "networkmanager:RegisterTransitGateway")
            }
            NetworkmanagerActions::RejectAttachment => write!(f, "networkmanager:RejectAttachment"),
            NetworkmanagerActions::RestoreCoreNetworkPolicyVersion => {
                write!(f, "networkmanager:RestoreCoreNetworkPolicyVersion")
            }
            NetworkmanagerActions::StartOrganizationServiceAccessUpdate => {
                write!(f, "networkmanager:StartOrganizationServiceAccessUpdate")
            }
            NetworkmanagerActions::StartRouteAnalysis => {
                write!(f, "networkmanager:StartRouteAnalysis")
            }
            NetworkmanagerActions::TagResource => write!(f, "networkmanager:TagResource"),
            NetworkmanagerActions::UntagResource => write!(f, "networkmanager:UntagResource"),
            NetworkmanagerActions::UpdateConnection => write!(f, "networkmanager:UpdateConnection"),
            NetworkmanagerActions::UpdateCoreNetwork => {
                write!(f, "networkmanager:UpdateCoreNetwork")
            }
            NetworkmanagerActions::UpdateDevice => write!(f, "networkmanager:UpdateDevice"),
            NetworkmanagerActions::UpdateDirectConnectGatewayAttachment => {
                write!(f, "networkmanager:UpdateDirectConnectGatewayAttachment")
            }
            NetworkmanagerActions::UpdateGlobalNetwork => {
                write!(f, "networkmanager:UpdateGlobalNetwork")
            }
            NetworkmanagerActions::UpdateLink => write!(f, "networkmanager:UpdateLink"),
            NetworkmanagerActions::UpdateNetworkResourceMetadata => {
                write!(f, "networkmanager:UpdateNetworkResourceMetadata")
            }
            NetworkmanagerActions::UpdateSite => write!(f, "networkmanager:UpdateSite"),
            NetworkmanagerActions::UpdateVpcAttachment => {
                write!(f, "networkmanager:UpdateVpcAttachment")
            }
        }
    }
}
