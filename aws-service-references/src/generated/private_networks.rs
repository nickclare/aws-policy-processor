// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum PrivateNetworksActions {
    AcknowledgeOrderReceipt,
    ActivateDeviceIdentifier,
    ActivateNetworkSite,
    ConfigureAccessPoint,
    CreateNetwork,
    CreateNetworkSite,
    DeactivateDeviceIdentifier,
    DeleteNetwork,
    DeleteNetworkSite,
    GetDeviceIdentifier,
    GetNetwork,
    GetNetworkResource,
    GetNetworkSite,
    GetOrder,
    ListDeviceIdentifiers,
    ListNetworkResources,
    ListNetworkSites,
    ListNetworks,
    ListOrders,
    ListTagsForResource,
    Ping,
    StartNetworkResourceUpdate,
    TagResource,
    UntagResource,
    UpdateNetworkSite,
    UpdateNetworkSitePlan,
}
impl std::fmt::Display for PrivateNetworksActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PrivateNetworksActions::AcknowledgeOrderReceipt => {
                write!(f, "private-networks:AcknowledgeOrderReceipt")
            }
            PrivateNetworksActions::ActivateDeviceIdentifier => {
                write!(f, "private-networks:ActivateDeviceIdentifier")
            }
            PrivateNetworksActions::ActivateNetworkSite => {
                write!(f, "private-networks:ActivateNetworkSite")
            }
            PrivateNetworksActions::ConfigureAccessPoint => {
                write!(f, "private-networks:ConfigureAccessPoint")
            }
            PrivateNetworksActions::CreateNetwork => write!(f, "private-networks:CreateNetwork"),
            PrivateNetworksActions::CreateNetworkSite => {
                write!(f, "private-networks:CreateNetworkSite")
            }
            PrivateNetworksActions::DeactivateDeviceIdentifier => {
                write!(f, "private-networks:DeactivateDeviceIdentifier")
            }
            PrivateNetworksActions::DeleteNetwork => write!(f, "private-networks:DeleteNetwork"),
            PrivateNetworksActions::DeleteNetworkSite => {
                write!(f, "private-networks:DeleteNetworkSite")
            }
            PrivateNetworksActions::GetDeviceIdentifier => {
                write!(f, "private-networks:GetDeviceIdentifier")
            }
            PrivateNetworksActions::GetNetwork => write!(f, "private-networks:GetNetwork"),
            PrivateNetworksActions::GetNetworkResource => {
                write!(f, "private-networks:GetNetworkResource")
            }
            PrivateNetworksActions::GetNetworkSite => write!(f, "private-networks:GetNetworkSite"),
            PrivateNetworksActions::GetOrder => write!(f, "private-networks:GetOrder"),
            PrivateNetworksActions::ListDeviceIdentifiers => {
                write!(f, "private-networks:ListDeviceIdentifiers")
            }
            PrivateNetworksActions::ListNetworkResources => {
                write!(f, "private-networks:ListNetworkResources")
            }
            PrivateNetworksActions::ListNetworkSites => {
                write!(f, "private-networks:ListNetworkSites")
            }
            PrivateNetworksActions::ListNetworks => write!(f, "private-networks:ListNetworks"),
            PrivateNetworksActions::ListOrders => write!(f, "private-networks:ListOrders"),
            PrivateNetworksActions::ListTagsForResource => {
                write!(f, "private-networks:ListTagsForResource")
            }
            PrivateNetworksActions::Ping => write!(f, "private-networks:Ping"),
            PrivateNetworksActions::StartNetworkResourceUpdate => {
                write!(f, "private-networks:StartNetworkResourceUpdate")
            }
            PrivateNetworksActions::TagResource => write!(f, "private-networks:TagResource"),
            PrivateNetworksActions::UntagResource => write!(f, "private-networks:UntagResource"),
            PrivateNetworksActions::UpdateNetworkSite => {
                write!(f, "private-networks:UpdateNetworkSite")
            }
            PrivateNetworksActions::UpdateNetworkSitePlan => {
                write!(f, "private-networks:UpdateNetworkSitePlan")
            }
        }
    }
}
