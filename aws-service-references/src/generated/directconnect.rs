// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum DirectconnectActions {
    AcceptDirectConnectGatewayAssociationProposal,
    AllocateConnectionOnInterconnect,
    AllocateHostedConnection,
    AllocatePrivateVirtualInterface,
    AllocatePublicVirtualInterface,
    AllocateTransitVirtualInterface,
    AssociateConnectionWithLag,
    AssociateHostedConnection,
    AssociateMacSecKey,
    AssociateVirtualInterface,
    ConfirmConnection,
    ConfirmCustomerAgreement,
    ConfirmPrivateVirtualInterface,
    ConfirmPublicVirtualInterface,
    ConfirmTransitVirtualInterface,
    CreateBgpPeer,
    CreateConnection,
    CreateDirectConnectGateway,
    CreateDirectConnectGatewayAssociation,
    CreateDirectConnectGatewayAssociationProposal,
    CreateInterconnect,
    CreateLag,
    CreatePrivateVirtualInterface,
    CreatePublicVirtualInterface,
    CreateTransitVirtualInterface,
    DeleteBgpPeer,
    DeleteConnection,
    DeleteDirectConnectGateway,
    DeleteDirectConnectGatewayAssociation,
    DeleteDirectConnectGatewayAssociationProposal,
    DeleteInterconnect,
    DeleteLag,
    DeleteVirtualInterface,
    DescribeConnectionLoa,
    DescribeConnections,
    DescribeConnectionsOnInterconnect,
    DescribeCustomerMetadata,
    DescribeDirectConnectGatewayAssociationProposals,
    DescribeDirectConnectGatewayAssociations,
    DescribeDirectConnectGatewayAttachments,
    DescribeDirectConnectGateways,
    DescribeHostedConnections,
    DescribeInterconnectLoa,
    DescribeInterconnects,
    DescribeLags,
    DescribeLoa,
    DescribeLocations,
    DescribeRouterConfiguration,
    DescribeTags,
    DescribeVirtualGateways,
    DescribeVirtualInterfaces,
    DisassociateConnectionFromLag,
    DisassociateMacSecKey,
    ListVirtualInterfaceTestHistory,
    StartBgpFailoverTest,
    StopBgpFailoverTest,
    TagResource,
    UntagResource,
    UpdateConnection,
    UpdateDirectConnectGateway,
    UpdateDirectConnectGatewayAssociation,
    UpdateLag,
    UpdateVirtualInterfaceAttributes,
}
impl std::fmt::Display for DirectconnectActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DirectconnectActions::AcceptDirectConnectGatewayAssociationProposal => write!(
                f,
                "directconnect:AcceptDirectConnectGatewayAssociationProposal"
            ),
            DirectconnectActions::AllocateConnectionOnInterconnect => {
                write!(f, "directconnect:AllocateConnectionOnInterconnect")
            }
            DirectconnectActions::AllocateHostedConnection => {
                write!(f, "directconnect:AllocateHostedConnection")
            }
            DirectconnectActions::AllocatePrivateVirtualInterface => {
                write!(f, "directconnect:AllocatePrivateVirtualInterface")
            }
            DirectconnectActions::AllocatePublicVirtualInterface => {
                write!(f, "directconnect:AllocatePublicVirtualInterface")
            }
            DirectconnectActions::AllocateTransitVirtualInterface => {
                write!(f, "directconnect:AllocateTransitVirtualInterface")
            }
            DirectconnectActions::AssociateConnectionWithLag => {
                write!(f, "directconnect:AssociateConnectionWithLag")
            }
            DirectconnectActions::AssociateHostedConnection => {
                write!(f, "directconnect:AssociateHostedConnection")
            }
            DirectconnectActions::AssociateMacSecKey => {
                write!(f, "directconnect:AssociateMacSecKey")
            }
            DirectconnectActions::AssociateVirtualInterface => {
                write!(f, "directconnect:AssociateVirtualInterface")
            }
            DirectconnectActions::ConfirmConnection => write!(f, "directconnect:ConfirmConnection"),
            DirectconnectActions::ConfirmCustomerAgreement => {
                write!(f, "directconnect:ConfirmCustomerAgreement")
            }
            DirectconnectActions::ConfirmPrivateVirtualInterface => {
                write!(f, "directconnect:ConfirmPrivateVirtualInterface")
            }
            DirectconnectActions::ConfirmPublicVirtualInterface => {
                write!(f, "directconnect:ConfirmPublicVirtualInterface")
            }
            DirectconnectActions::ConfirmTransitVirtualInterface => {
                write!(f, "directconnect:ConfirmTransitVirtualInterface")
            }
            DirectconnectActions::CreateBgpPeer => write!(f, "directconnect:CreateBGPPeer"),
            DirectconnectActions::CreateConnection => write!(f, "directconnect:CreateConnection"),
            DirectconnectActions::CreateDirectConnectGateway => {
                write!(f, "directconnect:CreateDirectConnectGateway")
            }
            DirectconnectActions::CreateDirectConnectGatewayAssociation => {
                write!(f, "directconnect:CreateDirectConnectGatewayAssociation")
            }
            DirectconnectActions::CreateDirectConnectGatewayAssociationProposal => write!(
                f,
                "directconnect:CreateDirectConnectGatewayAssociationProposal"
            ),
            DirectconnectActions::CreateInterconnect => {
                write!(f, "directconnect:CreateInterconnect")
            }
            DirectconnectActions::CreateLag => write!(f, "directconnect:CreateLag"),
            DirectconnectActions::CreatePrivateVirtualInterface => {
                write!(f, "directconnect:CreatePrivateVirtualInterface")
            }
            DirectconnectActions::CreatePublicVirtualInterface => {
                write!(f, "directconnect:CreatePublicVirtualInterface")
            }
            DirectconnectActions::CreateTransitVirtualInterface => {
                write!(f, "directconnect:CreateTransitVirtualInterface")
            }
            DirectconnectActions::DeleteBgpPeer => write!(f, "directconnect:DeleteBGPPeer"),
            DirectconnectActions::DeleteConnection => write!(f, "directconnect:DeleteConnection"),
            DirectconnectActions::DeleteDirectConnectGateway => {
                write!(f, "directconnect:DeleteDirectConnectGateway")
            }
            DirectconnectActions::DeleteDirectConnectGatewayAssociation => {
                write!(f, "directconnect:DeleteDirectConnectGatewayAssociation")
            }
            DirectconnectActions::DeleteDirectConnectGatewayAssociationProposal => write!(
                f,
                "directconnect:DeleteDirectConnectGatewayAssociationProposal"
            ),
            DirectconnectActions::DeleteInterconnect => {
                write!(f, "directconnect:DeleteInterconnect")
            }
            DirectconnectActions::DeleteLag => write!(f, "directconnect:DeleteLag"),
            DirectconnectActions::DeleteVirtualInterface => {
                write!(f, "directconnect:DeleteVirtualInterface")
            }
            DirectconnectActions::DescribeConnectionLoa => {
                write!(f, "directconnect:DescribeConnectionLoa")
            }
            DirectconnectActions::DescribeConnections => {
                write!(f, "directconnect:DescribeConnections")
            }
            DirectconnectActions::DescribeConnectionsOnInterconnect => {
                write!(f, "directconnect:DescribeConnectionsOnInterconnect")
            }
            DirectconnectActions::DescribeCustomerMetadata => {
                write!(f, "directconnect:DescribeCustomerMetadata")
            }
            DirectconnectActions::DescribeDirectConnectGatewayAssociationProposals => write!(
                f,
                "directconnect:DescribeDirectConnectGatewayAssociationProposals"
            ),
            DirectconnectActions::DescribeDirectConnectGatewayAssociations => {
                write!(f, "directconnect:DescribeDirectConnectGatewayAssociations")
            }
            DirectconnectActions::DescribeDirectConnectGatewayAttachments => {
                write!(f, "directconnect:DescribeDirectConnectGatewayAttachments")
            }
            DirectconnectActions::DescribeDirectConnectGateways => {
                write!(f, "directconnect:DescribeDirectConnectGateways")
            }
            DirectconnectActions::DescribeHostedConnections => {
                write!(f, "directconnect:DescribeHostedConnections")
            }
            DirectconnectActions::DescribeInterconnectLoa => {
                write!(f, "directconnect:DescribeInterconnectLoa")
            }
            DirectconnectActions::DescribeInterconnects => {
                write!(f, "directconnect:DescribeInterconnects")
            }
            DirectconnectActions::DescribeLags => write!(f, "directconnect:DescribeLags"),
            DirectconnectActions::DescribeLoa => write!(f, "directconnect:DescribeLoa"),
            DirectconnectActions::DescribeLocations => write!(f, "directconnect:DescribeLocations"),
            DirectconnectActions::DescribeRouterConfiguration => {
                write!(f, "directconnect:DescribeRouterConfiguration")
            }
            DirectconnectActions::DescribeTags => write!(f, "directconnect:DescribeTags"),
            DirectconnectActions::DescribeVirtualGateways => {
                write!(f, "directconnect:DescribeVirtualGateways")
            }
            DirectconnectActions::DescribeVirtualInterfaces => {
                write!(f, "directconnect:DescribeVirtualInterfaces")
            }
            DirectconnectActions::DisassociateConnectionFromLag => {
                write!(f, "directconnect:DisassociateConnectionFromLag")
            }
            DirectconnectActions::DisassociateMacSecKey => {
                write!(f, "directconnect:DisassociateMacSecKey")
            }
            DirectconnectActions::ListVirtualInterfaceTestHistory => {
                write!(f, "directconnect:ListVirtualInterfaceTestHistory")
            }
            DirectconnectActions::StartBgpFailoverTest => {
                write!(f, "directconnect:StartBgpFailoverTest")
            }
            DirectconnectActions::StopBgpFailoverTest => {
                write!(f, "directconnect:StopBgpFailoverTest")
            }
            DirectconnectActions::TagResource => write!(f, "directconnect:TagResource"),
            DirectconnectActions::UntagResource => write!(f, "directconnect:UntagResource"),
            DirectconnectActions::UpdateConnection => write!(f, "directconnect:UpdateConnection"),
            DirectconnectActions::UpdateDirectConnectGateway => {
                write!(f, "directconnect:UpdateDirectConnectGateway")
            }
            DirectconnectActions::UpdateDirectConnectGatewayAssociation => {
                write!(f, "directconnect:UpdateDirectConnectGatewayAssociation")
            }
            DirectconnectActions::UpdateLag => write!(f, "directconnect:UpdateLag"),
            DirectconnectActions::UpdateVirtualInterfaceAttributes => {
                write!(f, "directconnect:UpdateVirtualInterfaceAttributes")
            }
        }
    }
}
