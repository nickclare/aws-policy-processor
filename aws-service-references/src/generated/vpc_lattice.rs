// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum VpcLatticeActions {
    AssociateViaAwsService,
    AssociateViaAwsServiceEventsAndStates,
    CreateAccessLogSubscription,
    CreateListener,
    CreateResourceConfiguration,
    CreateResourceGateway,
    CreateRule,
    CreateService,
    CreateServiceNetwork,
    CreateServiceNetworkResourceAssociation,
    CreateServiceNetworkServiceAssociation,
    CreateServiceNetworkVpcAssociation,
    CreateServiceNetworkVpcEndpointAssociation,
    CreateTargetGroup,
    DeleteAccessLogSubscription,
    DeleteAuthPolicy,
    DeleteListener,
    DeleteResourceConfiguration,
    DeleteResourceEndpointAssociation,
    DeleteResourceGateway,
    DeleteResourcePolicy,
    DeleteRule,
    DeleteService,
    DeleteServiceNetwork,
    DeleteServiceNetworkResourceAssociation,
    DeleteServiceNetworkServiceAssociation,
    DeleteServiceNetworkVpcAssociation,
    DeleteTargetGroup,
    DeregisterTargets,
    GetAccessLogSubscription,
    GetAuthPolicy,
    GetListener,
    GetResourceConfiguration,
    GetResourceGateway,
    GetResourcePolicy,
    GetRule,
    GetService,
    GetServiceNetwork,
    GetServiceNetworkResourceAssociation,
    GetServiceNetworkServiceAssociation,
    GetServiceNetworkVpcAssociation,
    GetTargetGroup,
    ListAccessLogSubscriptions,
    ListListeners,
    ListResourceConfigurations,
    ListResourceEndpointAssociations,
    ListResourceGateways,
    ListRules,
    ListServiceNetworkResourceAssociations,
    ListServiceNetworkServiceAssociations,
    ListServiceNetworkVpcAssociations,
    ListServiceNetworkVpcEndpointAssociations,
    ListServiceNetworks,
    ListServices,
    ListTagsForResource,
    ListTargetGroups,
    ListTargets,
    PutAuthPolicy,
    PutResourcePolicy,
    RegisterTargets,
    TagResource,
    UntagResource,
    UpdateAccessLogSubscription,
    UpdateListener,
    UpdateResourceConfiguration,
    UpdateResourceGateway,
    UpdateRule,
    UpdateService,
    UpdateServiceNetwork,
    UpdateServiceNetworkVpcAssociation,
    UpdateTargetGroup,
}
impl std::fmt::Display for VpcLatticeActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VpcLatticeActions::AssociateViaAwsService => {
                write!(f, "vpc-lattice:AssociateViaAWSService")
            }
            VpcLatticeActions::AssociateViaAwsServiceEventsAndStates => {
                write!(f, "vpc-lattice:AssociateViaAWSService-EventsAndStates")
            }
            VpcLatticeActions::CreateAccessLogSubscription => {
                write!(f, "vpc-lattice:CreateAccessLogSubscription")
            }
            VpcLatticeActions::CreateListener => write!(f, "vpc-lattice:CreateListener"),
            VpcLatticeActions::CreateResourceConfiguration => {
                write!(f, "vpc-lattice:CreateResourceConfiguration")
            }
            VpcLatticeActions::CreateResourceGateway => {
                write!(f, "vpc-lattice:CreateResourceGateway")
            }
            VpcLatticeActions::CreateRule => write!(f, "vpc-lattice:CreateRule"),
            VpcLatticeActions::CreateService => write!(f, "vpc-lattice:CreateService"),
            VpcLatticeActions::CreateServiceNetwork => {
                write!(f, "vpc-lattice:CreateServiceNetwork")
            }
            VpcLatticeActions::CreateServiceNetworkResourceAssociation => {
                write!(f, "vpc-lattice:CreateServiceNetworkResourceAssociation")
            }
            VpcLatticeActions::CreateServiceNetworkServiceAssociation => {
                write!(f, "vpc-lattice:CreateServiceNetworkServiceAssociation")
            }
            VpcLatticeActions::CreateServiceNetworkVpcAssociation => {
                write!(f, "vpc-lattice:CreateServiceNetworkVpcAssociation")
            }
            VpcLatticeActions::CreateServiceNetworkVpcEndpointAssociation => {
                write!(f, "vpc-lattice:CreateServiceNetworkVpcEndpointAssociation")
            }
            VpcLatticeActions::CreateTargetGroup => write!(f, "vpc-lattice:CreateTargetGroup"),
            VpcLatticeActions::DeleteAccessLogSubscription => {
                write!(f, "vpc-lattice:DeleteAccessLogSubscription")
            }
            VpcLatticeActions::DeleteAuthPolicy => write!(f, "vpc-lattice:DeleteAuthPolicy"),
            VpcLatticeActions::DeleteListener => write!(f, "vpc-lattice:DeleteListener"),
            VpcLatticeActions::DeleteResourceConfiguration => {
                write!(f, "vpc-lattice:DeleteResourceConfiguration")
            }
            VpcLatticeActions::DeleteResourceEndpointAssociation => {
                write!(f, "vpc-lattice:DeleteResourceEndpointAssociation")
            }
            VpcLatticeActions::DeleteResourceGateway => {
                write!(f, "vpc-lattice:DeleteResourceGateway")
            }
            VpcLatticeActions::DeleteResourcePolicy => {
                write!(f, "vpc-lattice:DeleteResourcePolicy")
            }
            VpcLatticeActions::DeleteRule => write!(f, "vpc-lattice:DeleteRule"),
            VpcLatticeActions::DeleteService => write!(f, "vpc-lattice:DeleteService"),
            VpcLatticeActions::DeleteServiceNetwork => {
                write!(f, "vpc-lattice:DeleteServiceNetwork")
            }
            VpcLatticeActions::DeleteServiceNetworkResourceAssociation => {
                write!(f, "vpc-lattice:DeleteServiceNetworkResourceAssociation")
            }
            VpcLatticeActions::DeleteServiceNetworkServiceAssociation => {
                write!(f, "vpc-lattice:DeleteServiceNetworkServiceAssociation")
            }
            VpcLatticeActions::DeleteServiceNetworkVpcAssociation => {
                write!(f, "vpc-lattice:DeleteServiceNetworkVpcAssociation")
            }
            VpcLatticeActions::DeleteTargetGroup => write!(f, "vpc-lattice:DeleteTargetGroup"),
            VpcLatticeActions::DeregisterTargets => write!(f, "vpc-lattice:DeregisterTargets"),
            VpcLatticeActions::GetAccessLogSubscription => {
                write!(f, "vpc-lattice:GetAccessLogSubscription")
            }
            VpcLatticeActions::GetAuthPolicy => write!(f, "vpc-lattice:GetAuthPolicy"),
            VpcLatticeActions::GetListener => write!(f, "vpc-lattice:GetListener"),
            VpcLatticeActions::GetResourceConfiguration => {
                write!(f, "vpc-lattice:GetResourceConfiguration")
            }
            VpcLatticeActions::GetResourceGateway => write!(f, "vpc-lattice:GetResourceGateway"),
            VpcLatticeActions::GetResourcePolicy => write!(f, "vpc-lattice:GetResourcePolicy"),
            VpcLatticeActions::GetRule => write!(f, "vpc-lattice:GetRule"),
            VpcLatticeActions::GetService => write!(f, "vpc-lattice:GetService"),
            VpcLatticeActions::GetServiceNetwork => write!(f, "vpc-lattice:GetServiceNetwork"),
            VpcLatticeActions::GetServiceNetworkResourceAssociation => {
                write!(f, "vpc-lattice:GetServiceNetworkResourceAssociation")
            }
            VpcLatticeActions::GetServiceNetworkServiceAssociation => {
                write!(f, "vpc-lattice:GetServiceNetworkServiceAssociation")
            }
            VpcLatticeActions::GetServiceNetworkVpcAssociation => {
                write!(f, "vpc-lattice:GetServiceNetworkVpcAssociation")
            }
            VpcLatticeActions::GetTargetGroup => write!(f, "vpc-lattice:GetTargetGroup"),
            VpcLatticeActions::ListAccessLogSubscriptions => {
                write!(f, "vpc-lattice:ListAccessLogSubscriptions")
            }
            VpcLatticeActions::ListListeners => write!(f, "vpc-lattice:ListListeners"),
            VpcLatticeActions::ListResourceConfigurations => {
                write!(f, "vpc-lattice:ListResourceConfigurations")
            }
            VpcLatticeActions::ListResourceEndpointAssociations => {
                write!(f, "vpc-lattice:ListResourceEndpointAssociations")
            }
            VpcLatticeActions::ListResourceGateways => {
                write!(f, "vpc-lattice:ListResourceGateways")
            }
            VpcLatticeActions::ListRules => write!(f, "vpc-lattice:ListRules"),
            VpcLatticeActions::ListServiceNetworkResourceAssociations => {
                write!(f, "vpc-lattice:ListServiceNetworkResourceAssociations")
            }
            VpcLatticeActions::ListServiceNetworkServiceAssociations => {
                write!(f, "vpc-lattice:ListServiceNetworkServiceAssociations")
            }
            VpcLatticeActions::ListServiceNetworkVpcAssociations => {
                write!(f, "vpc-lattice:ListServiceNetworkVpcAssociations")
            }
            VpcLatticeActions::ListServiceNetworkVpcEndpointAssociations => {
                write!(f, "vpc-lattice:ListServiceNetworkVpcEndpointAssociations")
            }
            VpcLatticeActions::ListServiceNetworks => write!(f, "vpc-lattice:ListServiceNetworks"),
            VpcLatticeActions::ListServices => write!(f, "vpc-lattice:ListServices"),
            VpcLatticeActions::ListTagsForResource => write!(f, "vpc-lattice:ListTagsForResource"),
            VpcLatticeActions::ListTargetGroups => write!(f, "vpc-lattice:ListTargetGroups"),
            VpcLatticeActions::ListTargets => write!(f, "vpc-lattice:ListTargets"),
            VpcLatticeActions::PutAuthPolicy => write!(f, "vpc-lattice:PutAuthPolicy"),
            VpcLatticeActions::PutResourcePolicy => write!(f, "vpc-lattice:PutResourcePolicy"),
            VpcLatticeActions::RegisterTargets => write!(f, "vpc-lattice:RegisterTargets"),
            VpcLatticeActions::TagResource => write!(f, "vpc-lattice:TagResource"),
            VpcLatticeActions::UntagResource => write!(f, "vpc-lattice:UntagResource"),
            VpcLatticeActions::UpdateAccessLogSubscription => {
                write!(f, "vpc-lattice:UpdateAccessLogSubscription")
            }
            VpcLatticeActions::UpdateListener => write!(f, "vpc-lattice:UpdateListener"),
            VpcLatticeActions::UpdateResourceConfiguration => {
                write!(f, "vpc-lattice:UpdateResourceConfiguration")
            }
            VpcLatticeActions::UpdateResourceGateway => {
                write!(f, "vpc-lattice:UpdateResourceGateway")
            }
            VpcLatticeActions::UpdateRule => write!(f, "vpc-lattice:UpdateRule"),
            VpcLatticeActions::UpdateService => write!(f, "vpc-lattice:UpdateService"),
            VpcLatticeActions::UpdateServiceNetwork => {
                write!(f, "vpc-lattice:UpdateServiceNetwork")
            }
            VpcLatticeActions::UpdateServiceNetworkVpcAssociation => {
                write!(f, "vpc-lattice:UpdateServiceNetworkVpcAssociation")
            }
            VpcLatticeActions::UpdateTargetGroup => write!(f, "vpc-lattice:UpdateTargetGroup"),
        }
    }
}
