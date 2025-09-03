// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum ServicediscoveryActions {
    CreateHttpNamespace,
    CreatePrivateDnsNamespace,
    CreatePublicDnsNamespace,
    CreateService,
    DeleteNamespace,
    DeleteResourcePolicy,
    DeleteService,
    DeleteServiceAttributes,
    DeregisterInstance,
    DiscoverInstances,
    DiscoverInstancesRevision,
    GetInstance,
    GetInstancesHealthStatus,
    GetNamespace,
    GetOperation,
    GetResourcePolicy,
    GetService,
    GetServiceAttributes,
    ListInstances,
    ListNamespaces,
    ListOperations,
    ListServices,
    ListTagsForResource,
    PutResourcePolicy,
    RegisterInstance,
    TagResource,
    UntagResource,
    UpdateHttpNamespace,
    UpdateInstanceCustomHealthStatus,
    UpdatePrivateDnsNamespace,
    UpdatePublicDnsNamespace,
    UpdateService,
    UpdateServiceAttributes,
}
impl std::fmt::Display for ServicediscoveryActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServicediscoveryActions::CreateHttpNamespace => {
                write!(f, "servicediscovery:CreateHttpNamespace")
            }
            ServicediscoveryActions::CreatePrivateDnsNamespace => {
                write!(f, "servicediscovery:CreatePrivateDnsNamespace")
            }
            ServicediscoveryActions::CreatePublicDnsNamespace => {
                write!(f, "servicediscovery:CreatePublicDnsNamespace")
            }
            ServicediscoveryActions::CreateService => write!(f, "servicediscovery:CreateService"),
            ServicediscoveryActions::DeleteNamespace => {
                write!(f, "servicediscovery:DeleteNamespace")
            }
            ServicediscoveryActions::DeleteResourcePolicy => {
                write!(f, "servicediscovery:DeleteResourcePolicy")
            }
            ServicediscoveryActions::DeleteService => write!(f, "servicediscovery:DeleteService"),
            ServicediscoveryActions::DeleteServiceAttributes => {
                write!(f, "servicediscovery:DeleteServiceAttributes")
            }
            ServicediscoveryActions::DeregisterInstance => {
                write!(f, "servicediscovery:DeregisterInstance")
            }
            ServicediscoveryActions::DiscoverInstances => {
                write!(f, "servicediscovery:DiscoverInstances")
            }
            ServicediscoveryActions::DiscoverInstancesRevision => {
                write!(f, "servicediscovery:DiscoverInstancesRevision")
            }
            ServicediscoveryActions::GetInstance => write!(f, "servicediscovery:GetInstance"),
            ServicediscoveryActions::GetInstancesHealthStatus => {
                write!(f, "servicediscovery:GetInstancesHealthStatus")
            }
            ServicediscoveryActions::GetNamespace => write!(f, "servicediscovery:GetNamespace"),
            ServicediscoveryActions::GetOperation => write!(f, "servicediscovery:GetOperation"),
            ServicediscoveryActions::GetResourcePolicy => {
                write!(f, "servicediscovery:GetResourcePolicy")
            }
            ServicediscoveryActions::GetService => write!(f, "servicediscovery:GetService"),
            ServicediscoveryActions::GetServiceAttributes => {
                write!(f, "servicediscovery:GetServiceAttributes")
            }
            ServicediscoveryActions::ListInstances => write!(f, "servicediscovery:ListInstances"),
            ServicediscoveryActions::ListNamespaces => write!(f, "servicediscovery:ListNamespaces"),
            ServicediscoveryActions::ListOperations => write!(f, "servicediscovery:ListOperations"),
            ServicediscoveryActions::ListServices => write!(f, "servicediscovery:ListServices"),
            ServicediscoveryActions::ListTagsForResource => {
                write!(f, "servicediscovery:ListTagsForResource")
            }
            ServicediscoveryActions::PutResourcePolicy => {
                write!(f, "servicediscovery:PutResourcePolicy")
            }
            ServicediscoveryActions::RegisterInstance => {
                write!(f, "servicediscovery:RegisterInstance")
            }
            ServicediscoveryActions::TagResource => write!(f, "servicediscovery:TagResource"),
            ServicediscoveryActions::UntagResource => write!(f, "servicediscovery:UntagResource"),
            ServicediscoveryActions::UpdateHttpNamespace => {
                write!(f, "servicediscovery:UpdateHttpNamespace")
            }
            ServicediscoveryActions::UpdateInstanceCustomHealthStatus => {
                write!(f, "servicediscovery:UpdateInstanceCustomHealthStatus")
            }
            ServicediscoveryActions::UpdatePrivateDnsNamespace => {
                write!(f, "servicediscovery:UpdatePrivateDnsNamespace")
            }
            ServicediscoveryActions::UpdatePublicDnsNamespace => {
                write!(f, "servicediscovery:UpdatePublicDnsNamespace")
            }
            ServicediscoveryActions::UpdateService => write!(f, "servicediscovery:UpdateService"),
            ServicediscoveryActions::UpdateServiceAttributes => {
                write!(f, "servicediscovery:UpdateServiceAttributes")
            }
        }
    }
}
