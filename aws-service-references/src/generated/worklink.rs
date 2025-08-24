// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum WorklinkActions {
    AssociateDomain,
    AssociateWebsiteAuthorizationProvider,
    AssociateWebsiteCertificateAuthority,
    CreateFleet,
    DeleteFleet,
    DescribeAuditStreamConfiguration,
    DescribeCompanyNetworkConfiguration,
    DescribeDevice,
    DescribeDevicePolicyConfiguration,
    DescribeDomain,
    DescribeFleetMetadata,
    DescribeIdentityProviderConfiguration,
    DescribeWebsiteCertificateAuthority,
    DisassociateDomain,
    DisassociateWebsiteAuthorizationProvider,
    DisassociateWebsiteCertificateAuthority,
    ListDevices,
    ListDomains,
    ListFleets,
    ListTagsForResource,
    ListWebsiteAuthorizationProviders,
    ListWebsiteCertificateAuthorities,
    RestoreDomainAccess,
    RevokeDomainAccess,
    SearchEntity,
    SignOutUser,
    TagResource,
    UntagResource,
    UpdateAuditStreamConfiguration,
    UpdateCompanyNetworkConfiguration,
    UpdateDevicePolicyConfiguration,
    UpdateDomainMetadata,
    UpdateFleetMetadata,
    UpdateIdentityProviderConfiguration,
}
impl std::fmt::Display for WorklinkActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WorklinkActions::AssociateDomain => write!(f, "worklink:AssociateDomain"),
            WorklinkActions::AssociateWebsiteAuthorizationProvider => {
                write!(f, "worklink:AssociateWebsiteAuthorizationProvider")
            }
            WorklinkActions::AssociateWebsiteCertificateAuthority => {
                write!(f, "worklink:AssociateWebsiteCertificateAuthority")
            }
            WorklinkActions::CreateFleet => write!(f, "worklink:CreateFleet"),
            WorklinkActions::DeleteFleet => write!(f, "worklink:DeleteFleet"),
            WorklinkActions::DescribeAuditStreamConfiguration => {
                write!(f, "worklink:DescribeAuditStreamConfiguration")
            }
            WorklinkActions::DescribeCompanyNetworkConfiguration => {
                write!(f, "worklink:DescribeCompanyNetworkConfiguration")
            }
            WorklinkActions::DescribeDevice => write!(f, "worklink:DescribeDevice"),
            WorklinkActions::DescribeDevicePolicyConfiguration => {
                write!(f, "worklink:DescribeDevicePolicyConfiguration")
            }
            WorklinkActions::DescribeDomain => write!(f, "worklink:DescribeDomain"),
            WorklinkActions::DescribeFleetMetadata => write!(f, "worklink:DescribeFleetMetadata"),
            WorklinkActions::DescribeIdentityProviderConfiguration => {
                write!(f, "worklink:DescribeIdentityProviderConfiguration")
            }
            WorklinkActions::DescribeWebsiteCertificateAuthority => {
                write!(f, "worklink:DescribeWebsiteCertificateAuthority")
            }
            WorklinkActions::DisassociateDomain => write!(f, "worklink:DisassociateDomain"),
            WorklinkActions::DisassociateWebsiteAuthorizationProvider => {
                write!(f, "worklink:DisassociateWebsiteAuthorizationProvider")
            }
            WorklinkActions::DisassociateWebsiteCertificateAuthority => {
                write!(f, "worklink:DisassociateWebsiteCertificateAuthority")
            }
            WorklinkActions::ListDevices => write!(f, "worklink:ListDevices"),
            WorklinkActions::ListDomains => write!(f, "worklink:ListDomains"),
            WorklinkActions::ListFleets => write!(f, "worklink:ListFleets"),
            WorklinkActions::ListTagsForResource => write!(f, "worklink:ListTagsForResource"),
            WorklinkActions::ListWebsiteAuthorizationProviders => {
                write!(f, "worklink:ListWebsiteAuthorizationProviders")
            }
            WorklinkActions::ListWebsiteCertificateAuthorities => {
                write!(f, "worklink:ListWebsiteCertificateAuthorities")
            }
            WorklinkActions::RestoreDomainAccess => write!(f, "worklink:RestoreDomainAccess"),
            WorklinkActions::RevokeDomainAccess => write!(f, "worklink:RevokeDomainAccess"),
            WorklinkActions::SearchEntity => write!(f, "worklink:SearchEntity"),
            WorklinkActions::SignOutUser => write!(f, "worklink:SignOutUser"),
            WorklinkActions::TagResource => write!(f, "worklink:TagResource"),
            WorklinkActions::UntagResource => write!(f, "worklink:UntagResource"),
            WorklinkActions::UpdateAuditStreamConfiguration => {
                write!(f, "worklink:UpdateAuditStreamConfiguration")
            }
            WorklinkActions::UpdateCompanyNetworkConfiguration => {
                write!(f, "worklink:UpdateCompanyNetworkConfiguration")
            }
            WorklinkActions::UpdateDevicePolicyConfiguration => {
                write!(f, "worklink:UpdateDevicePolicyConfiguration")
            }
            WorklinkActions::UpdateDomainMetadata => write!(f, "worklink:UpdateDomainMetadata"),
            WorklinkActions::UpdateFleetMetadata => write!(f, "worklink:UpdateFleetMetadata"),
            WorklinkActions::UpdateIdentityProviderConfiguration => {
                write!(f, "worklink:UpdateIdentityProviderConfiguration")
            }
        }
    }
}
