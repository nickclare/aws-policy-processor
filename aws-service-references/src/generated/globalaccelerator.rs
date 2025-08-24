// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum GlobalacceleratorActions {
    AddCustomRoutingEndpoints,
    AddEndpoints,
    AdvertiseByoipCidr,
    AllowCustomRoutingTraffic,
    CreateAccelerator,
    CreateCrossAccountAttachment,
    CreateCustomRoutingAccelerator,
    CreateCustomRoutingEndpointGroup,
    CreateCustomRoutingListener,
    CreateEndpointGroup,
    CreateListener,
    DeleteAccelerator,
    DeleteCrossAccountAttachment,
    DeleteCustomRoutingAccelerator,
    DeleteCustomRoutingEndpointGroup,
    DeleteCustomRoutingListener,
    DeleteEndpointGroup,
    DeleteListener,
    DenyCustomRoutingTraffic,
    DeprovisionByoipCidr,
    DescribeAccelerator,
    DescribeAcceleratorAttributes,
    DescribeCrossAccountAttachment,
    DescribeCustomRoutingAccelerator,
    DescribeCustomRoutingAcceleratorAttributes,
    DescribeCustomRoutingEndpointGroup,
    DescribeCustomRoutingListener,
    DescribeEndpointGroup,
    DescribeListener,
    ListAccelerators,
    ListByoipCidrs,
    ListCrossAccountAttachments,
    ListCrossAccountResourceAccounts,
    ListCrossAccountResources,
    ListCustomRoutingAccelerators,
    ListCustomRoutingEndpointGroups,
    ListCustomRoutingListeners,
    ListCustomRoutingPortMappings,
    ListCustomRoutingPortMappingsByDestination,
    ListEndpointGroups,
    ListListeners,
    ListTagsForResource,
    ProvisionByoipCidr,
    RemoveCustomRoutingEndpoints,
    RemoveEndpoints,
    TagResource,
    UntagResource,
    UpdateAccelerator,
    UpdateAcceleratorAttributes,
    UpdateCrossAccountAttachment,
    UpdateCustomRoutingAccelerator,
    UpdateCustomRoutingAcceleratorAttributes,
    UpdateCustomRoutingListener,
    UpdateEndpointGroup,
    UpdateListener,
    WithdrawByoipCidr,
}
impl std::fmt::Display for GlobalacceleratorActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GlobalacceleratorActions::AddCustomRoutingEndpoints => {
                write!(f, "globalaccelerator:AddCustomRoutingEndpoints")
            }
            GlobalacceleratorActions::AddEndpoints => write!(f, "globalaccelerator:AddEndpoints"),
            GlobalacceleratorActions::AdvertiseByoipCidr => {
                write!(f, "globalaccelerator:AdvertiseByoipCidr")
            }
            GlobalacceleratorActions::AllowCustomRoutingTraffic => {
                write!(f, "globalaccelerator:AllowCustomRoutingTraffic")
            }
            GlobalacceleratorActions::CreateAccelerator => {
                write!(f, "globalaccelerator:CreateAccelerator")
            }
            GlobalacceleratorActions::CreateCrossAccountAttachment => {
                write!(f, "globalaccelerator:CreateCrossAccountAttachment")
            }
            GlobalacceleratorActions::CreateCustomRoutingAccelerator => {
                write!(f, "globalaccelerator:CreateCustomRoutingAccelerator")
            }
            GlobalacceleratorActions::CreateCustomRoutingEndpointGroup => {
                write!(f, "globalaccelerator:CreateCustomRoutingEndpointGroup")
            }
            GlobalacceleratorActions::CreateCustomRoutingListener => {
                write!(f, "globalaccelerator:CreateCustomRoutingListener")
            }
            GlobalacceleratorActions::CreateEndpointGroup => {
                write!(f, "globalaccelerator:CreateEndpointGroup")
            }
            GlobalacceleratorActions::CreateListener => {
                write!(f, "globalaccelerator:CreateListener")
            }
            GlobalacceleratorActions::DeleteAccelerator => {
                write!(f, "globalaccelerator:DeleteAccelerator")
            }
            GlobalacceleratorActions::DeleteCrossAccountAttachment => {
                write!(f, "globalaccelerator:DeleteCrossAccountAttachment")
            }
            GlobalacceleratorActions::DeleteCustomRoutingAccelerator => {
                write!(f, "globalaccelerator:DeleteCustomRoutingAccelerator")
            }
            GlobalacceleratorActions::DeleteCustomRoutingEndpointGroup => {
                write!(f, "globalaccelerator:DeleteCustomRoutingEndpointGroup")
            }
            GlobalacceleratorActions::DeleteCustomRoutingListener => {
                write!(f, "globalaccelerator:DeleteCustomRoutingListener")
            }
            GlobalacceleratorActions::DeleteEndpointGroup => {
                write!(f, "globalaccelerator:DeleteEndpointGroup")
            }
            GlobalacceleratorActions::DeleteListener => {
                write!(f, "globalaccelerator:DeleteListener")
            }
            GlobalacceleratorActions::DenyCustomRoutingTraffic => {
                write!(f, "globalaccelerator:DenyCustomRoutingTraffic")
            }
            GlobalacceleratorActions::DeprovisionByoipCidr => {
                write!(f, "globalaccelerator:DeprovisionByoipCidr")
            }
            GlobalacceleratorActions::DescribeAccelerator => {
                write!(f, "globalaccelerator:DescribeAccelerator")
            }
            GlobalacceleratorActions::DescribeAcceleratorAttributes => {
                write!(f, "globalaccelerator:DescribeAcceleratorAttributes")
            }
            GlobalacceleratorActions::DescribeCrossAccountAttachment => {
                write!(f, "globalaccelerator:DescribeCrossAccountAttachment")
            }
            GlobalacceleratorActions::DescribeCustomRoutingAccelerator => {
                write!(f, "globalaccelerator:DescribeCustomRoutingAccelerator")
            }
            GlobalacceleratorActions::DescribeCustomRoutingAcceleratorAttributes => write!(
                f,
                "globalaccelerator:DescribeCustomRoutingAcceleratorAttributes"
            ),
            GlobalacceleratorActions::DescribeCustomRoutingEndpointGroup => {
                write!(f, "globalaccelerator:DescribeCustomRoutingEndpointGroup")
            }
            GlobalacceleratorActions::DescribeCustomRoutingListener => {
                write!(f, "globalaccelerator:DescribeCustomRoutingListener")
            }
            GlobalacceleratorActions::DescribeEndpointGroup => {
                write!(f, "globalaccelerator:DescribeEndpointGroup")
            }
            GlobalacceleratorActions::DescribeListener => {
                write!(f, "globalaccelerator:DescribeListener")
            }
            GlobalacceleratorActions::ListAccelerators => {
                write!(f, "globalaccelerator:ListAccelerators")
            }
            GlobalacceleratorActions::ListByoipCidrs => {
                write!(f, "globalaccelerator:ListByoipCidrs")
            }
            GlobalacceleratorActions::ListCrossAccountAttachments => {
                write!(f, "globalaccelerator:ListCrossAccountAttachments")
            }
            GlobalacceleratorActions::ListCrossAccountResourceAccounts => {
                write!(f, "globalaccelerator:ListCrossAccountResourceAccounts")
            }
            GlobalacceleratorActions::ListCrossAccountResources => {
                write!(f, "globalaccelerator:ListCrossAccountResources")
            }
            GlobalacceleratorActions::ListCustomRoutingAccelerators => {
                write!(f, "globalaccelerator:ListCustomRoutingAccelerators")
            }
            GlobalacceleratorActions::ListCustomRoutingEndpointGroups => {
                write!(f, "globalaccelerator:ListCustomRoutingEndpointGroups")
            }
            GlobalacceleratorActions::ListCustomRoutingListeners => {
                write!(f, "globalaccelerator:ListCustomRoutingListeners")
            }
            GlobalacceleratorActions::ListCustomRoutingPortMappings => {
                write!(f, "globalaccelerator:ListCustomRoutingPortMappings")
            }
            GlobalacceleratorActions::ListCustomRoutingPortMappingsByDestination => write!(
                f,
                "globalaccelerator:ListCustomRoutingPortMappingsByDestination"
            ),
            GlobalacceleratorActions::ListEndpointGroups => {
                write!(f, "globalaccelerator:ListEndpointGroups")
            }
            GlobalacceleratorActions::ListListeners => write!(f, "globalaccelerator:ListListeners"),
            GlobalacceleratorActions::ListTagsForResource => {
                write!(f, "globalaccelerator:ListTagsForResource")
            }
            GlobalacceleratorActions::ProvisionByoipCidr => {
                write!(f, "globalaccelerator:ProvisionByoipCidr")
            }
            GlobalacceleratorActions::RemoveCustomRoutingEndpoints => {
                write!(f, "globalaccelerator:RemoveCustomRoutingEndpoints")
            }
            GlobalacceleratorActions::RemoveEndpoints => {
                write!(f, "globalaccelerator:RemoveEndpoints")
            }
            GlobalacceleratorActions::TagResource => write!(f, "globalaccelerator:TagResource"),
            GlobalacceleratorActions::UntagResource => write!(f, "globalaccelerator:UntagResource"),
            GlobalacceleratorActions::UpdateAccelerator => {
                write!(f, "globalaccelerator:UpdateAccelerator")
            }
            GlobalacceleratorActions::UpdateAcceleratorAttributes => {
                write!(f, "globalaccelerator:UpdateAcceleratorAttributes")
            }
            GlobalacceleratorActions::UpdateCrossAccountAttachment => {
                write!(f, "globalaccelerator:UpdateCrossAccountAttachment")
            }
            GlobalacceleratorActions::UpdateCustomRoutingAccelerator => {
                write!(f, "globalaccelerator:UpdateCustomRoutingAccelerator")
            }
            GlobalacceleratorActions::UpdateCustomRoutingAcceleratorAttributes => write!(
                f,
                "globalaccelerator:UpdateCustomRoutingAcceleratorAttributes"
            ),
            GlobalacceleratorActions::UpdateCustomRoutingListener => {
                write!(f, "globalaccelerator:UpdateCustomRoutingListener")
            }
            GlobalacceleratorActions::UpdateEndpointGroup => {
                write!(f, "globalaccelerator:UpdateEndpointGroup")
            }
            GlobalacceleratorActions::UpdateListener => {
                write!(f, "globalaccelerator:UpdateListener")
            }
            GlobalacceleratorActions::WithdrawByoipCidr => {
                write!(f, "globalaccelerator:WithdrawByoipCidr")
            }
        }
    }
}
