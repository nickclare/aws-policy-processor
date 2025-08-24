// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum NetworkFirewallActions {
    AcceptNetworkFirewallTransitGatewayAttachment,
    AssociateAvailabilityZones,
    AssociateFirewallPolicy,
    AssociateSubnets,
    CreateFirewall,
    CreateFirewallPolicy,
    CreateRuleGroup,
    CreateTlsInspectionConfiguration,
    CreateVpcEndpointAssociation,
    DeleteFirewall,
    DeleteFirewallPolicy,
    DeleteNetworkFirewallTransitGatewayAttachment,
    DeleteResourcePolicy,
    DeleteRuleGroup,
    DeleteTlsInspectionConfiguration,
    DeleteVpcEndpointAssociation,
    DescribeFirewall,
    DescribeFirewallMetadata,
    DescribeFirewallPolicy,
    DescribeFlowOperation,
    DescribeLoggingConfiguration,
    DescribeResourcePolicy,
    DescribeRuleGroup,
    DescribeRuleGroupMetadata,
    DescribeRuleGroupSummary,
    DescribeTlsInspectionConfiguration,
    DescribeVpcEndpointAssociation,
    DisassociateAvailabilityZones,
    DisassociateSubnets,
    GetAnalysisReportResults,
    ListAnalysisReports,
    ListFirewallPolicies,
    ListFirewalls,
    ListFlowOperationResults,
    ListFlowOperations,
    ListRuleGroups,
    ListTagsForResource,
    ListTlsInspectionConfigurations,
    ListVpcEndpointAssociations,
    PutResourcePolicy,
    RejectNetworkFirewallTransitGatewayAttachment,
    StartAnalysisReport,
    StartFlowCapture,
    StartFlowFlush,
    TagResource,
    UntagResource,
    UpdateAvailabilityZoneChangeProtection,
    UpdateFirewallAnalysisSettings,
    UpdateFirewallDeleteProtection,
    UpdateFirewallDescription,
    UpdateFirewallEncryptionConfiguration,
    UpdateFirewallPolicy,
    UpdateFirewallPolicyChangeProtection,
    UpdateLoggingConfiguration,
    UpdateRuleGroup,
    UpdateSubnetChangeProtection,
    UpdateTlsInspectionConfiguration,
}
impl std::fmt::Display for NetworkFirewallActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NetworkFirewallActions::AcceptNetworkFirewallTransitGatewayAttachment => write!(
                f,
                "network-firewall:AcceptNetworkFirewallTransitGatewayAttachment"
            ),
            NetworkFirewallActions::AssociateAvailabilityZones => {
                write!(f, "network-firewall:AssociateAvailabilityZones")
            }
            NetworkFirewallActions::AssociateFirewallPolicy => {
                write!(f, "network-firewall:AssociateFirewallPolicy")
            }
            NetworkFirewallActions::AssociateSubnets => {
                write!(f, "network-firewall:AssociateSubnets")
            }
            NetworkFirewallActions::CreateFirewall => write!(f, "network-firewall:CreateFirewall"),
            NetworkFirewallActions::CreateFirewallPolicy => {
                write!(f, "network-firewall:CreateFirewallPolicy")
            }
            NetworkFirewallActions::CreateRuleGroup => {
                write!(f, "network-firewall:CreateRuleGroup")
            }
            NetworkFirewallActions::CreateTlsInspectionConfiguration => {
                write!(f, "network-firewall:CreateTLSInspectionConfiguration")
            }
            NetworkFirewallActions::CreateVpcEndpointAssociation => {
                write!(f, "network-firewall:CreateVpcEndpointAssociation")
            }
            NetworkFirewallActions::DeleteFirewall => write!(f, "network-firewall:DeleteFirewall"),
            NetworkFirewallActions::DeleteFirewallPolicy => {
                write!(f, "network-firewall:DeleteFirewallPolicy")
            }
            NetworkFirewallActions::DeleteNetworkFirewallTransitGatewayAttachment => write!(
                f,
                "network-firewall:DeleteNetworkFirewallTransitGatewayAttachment"
            ),
            NetworkFirewallActions::DeleteResourcePolicy => {
                write!(f, "network-firewall:DeleteResourcePolicy")
            }
            NetworkFirewallActions::DeleteRuleGroup => {
                write!(f, "network-firewall:DeleteRuleGroup")
            }
            NetworkFirewallActions::DeleteTlsInspectionConfiguration => {
                write!(f, "network-firewall:DeleteTLSInspectionConfiguration")
            }
            NetworkFirewallActions::DeleteVpcEndpointAssociation => {
                write!(f, "network-firewall:DeleteVpcEndpointAssociation")
            }
            NetworkFirewallActions::DescribeFirewall => {
                write!(f, "network-firewall:DescribeFirewall")
            }
            NetworkFirewallActions::DescribeFirewallMetadata => {
                write!(f, "network-firewall:DescribeFirewallMetadata")
            }
            NetworkFirewallActions::DescribeFirewallPolicy => {
                write!(f, "network-firewall:DescribeFirewallPolicy")
            }
            NetworkFirewallActions::DescribeFlowOperation => {
                write!(f, "network-firewall:DescribeFlowOperation")
            }
            NetworkFirewallActions::DescribeLoggingConfiguration => {
                write!(f, "network-firewall:DescribeLoggingConfiguration")
            }
            NetworkFirewallActions::DescribeResourcePolicy => {
                write!(f, "network-firewall:DescribeResourcePolicy")
            }
            NetworkFirewallActions::DescribeRuleGroup => {
                write!(f, "network-firewall:DescribeRuleGroup")
            }
            NetworkFirewallActions::DescribeRuleGroupMetadata => {
                write!(f, "network-firewall:DescribeRuleGroupMetadata")
            }
            NetworkFirewallActions::DescribeRuleGroupSummary => {
                write!(f, "network-firewall:DescribeRuleGroupSummary")
            }
            NetworkFirewallActions::DescribeTlsInspectionConfiguration => {
                write!(f, "network-firewall:DescribeTLSInspectionConfiguration")
            }
            NetworkFirewallActions::DescribeVpcEndpointAssociation => {
                write!(f, "network-firewall:DescribeVpcEndpointAssociation")
            }
            NetworkFirewallActions::DisassociateAvailabilityZones => {
                write!(f, "network-firewall:DisassociateAvailabilityZones")
            }
            NetworkFirewallActions::DisassociateSubnets => {
                write!(f, "network-firewall:DisassociateSubnets")
            }
            NetworkFirewallActions::GetAnalysisReportResults => {
                write!(f, "network-firewall:GetAnalysisReportResults")
            }
            NetworkFirewallActions::ListAnalysisReports => {
                write!(f, "network-firewall:ListAnalysisReports")
            }
            NetworkFirewallActions::ListFirewallPolicies => {
                write!(f, "network-firewall:ListFirewallPolicies")
            }
            NetworkFirewallActions::ListFirewalls => write!(f, "network-firewall:ListFirewalls"),
            NetworkFirewallActions::ListFlowOperationResults => {
                write!(f, "network-firewall:ListFlowOperationResults")
            }
            NetworkFirewallActions::ListFlowOperations => {
                write!(f, "network-firewall:ListFlowOperations")
            }
            NetworkFirewallActions::ListRuleGroups => write!(f, "network-firewall:ListRuleGroups"),
            NetworkFirewallActions::ListTagsForResource => {
                write!(f, "network-firewall:ListTagsForResource")
            }
            NetworkFirewallActions::ListTlsInspectionConfigurations => {
                write!(f, "network-firewall:ListTLSInspectionConfigurations")
            }
            NetworkFirewallActions::ListVpcEndpointAssociations => {
                write!(f, "network-firewall:ListVpcEndpointAssociations")
            }
            NetworkFirewallActions::PutResourcePolicy => {
                write!(f, "network-firewall:PutResourcePolicy")
            }
            NetworkFirewallActions::RejectNetworkFirewallTransitGatewayAttachment => write!(
                f,
                "network-firewall:RejectNetworkFirewallTransitGatewayAttachment"
            ),
            NetworkFirewallActions::StartAnalysisReport => {
                write!(f, "network-firewall:StartAnalysisReport")
            }
            NetworkFirewallActions::StartFlowCapture => {
                write!(f, "network-firewall:StartFlowCapture")
            }
            NetworkFirewallActions::StartFlowFlush => write!(f, "network-firewall:StartFlowFlush"),
            NetworkFirewallActions::TagResource => write!(f, "network-firewall:TagResource"),
            NetworkFirewallActions::UntagResource => write!(f, "network-firewall:UntagResource"),
            NetworkFirewallActions::UpdateAvailabilityZoneChangeProtection => {
                write!(f, "network-firewall:UpdateAvailabilityZoneChangeProtection")
            }
            NetworkFirewallActions::UpdateFirewallAnalysisSettings => {
                write!(f, "network-firewall:UpdateFirewallAnalysisSettings")
            }
            NetworkFirewallActions::UpdateFirewallDeleteProtection => {
                write!(f, "network-firewall:UpdateFirewallDeleteProtection")
            }
            NetworkFirewallActions::UpdateFirewallDescription => {
                write!(f, "network-firewall:UpdateFirewallDescription")
            }
            NetworkFirewallActions::UpdateFirewallEncryptionConfiguration => {
                write!(f, "network-firewall:UpdateFirewallEncryptionConfiguration")
            }
            NetworkFirewallActions::UpdateFirewallPolicy => {
                write!(f, "network-firewall:UpdateFirewallPolicy")
            }
            NetworkFirewallActions::UpdateFirewallPolicyChangeProtection => {
                write!(f, "network-firewall:UpdateFirewallPolicyChangeProtection")
            }
            NetworkFirewallActions::UpdateLoggingConfiguration => {
                write!(f, "network-firewall:UpdateLoggingConfiguration")
            }
            NetworkFirewallActions::UpdateRuleGroup => {
                write!(f, "network-firewall:UpdateRuleGroup")
            }
            NetworkFirewallActions::UpdateSubnetChangeProtection => {
                write!(f, "network-firewall:UpdateSubnetChangeProtection")
            }
            NetworkFirewallActions::UpdateTlsInspectionConfiguration => {
                write!(f, "network-firewall:UpdateTLSInspectionConfiguration")
            }
        }
    }
}
