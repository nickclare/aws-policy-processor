// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum ApprunnerActions {
    AssociateCustomDomain,
    AssociateWebAcl,
    CreateAutoScalingConfiguration,
    CreateConnection,
    CreateObservabilityConfiguration,
    CreateService,
    CreateVpcConnector,
    CreateVpcIngressConnection,
    DeleteAutoScalingConfiguration,
    DeleteConnection,
    DeleteObservabilityConfiguration,
    DeleteService,
    DeleteVpcConnector,
    DeleteVpcIngressConnection,
    DescribeAutoScalingConfiguration,
    DescribeCustomDomains,
    DescribeObservabilityConfiguration,
    DescribeOperation,
    DescribeService,
    DescribeVpcConnector,
    DescribeVpcIngressConnection,
    DescribeWebAclForService,
    DisassociateCustomDomain,
    DisassociateWebAcl,
    ListAssociatedServicesForWebAcl,
    ListAutoScalingConfigurations,
    ListConnections,
    ListObservabilityConfigurations,
    ListOperations,
    ListServices,
    ListServicesForAutoScalingConfiguration,
    ListTagsForResource,
    ListVpcConnectors,
    ListVpcIngressConnections,
    PauseService,
    ResumeService,
    StartDeployment,
    TagResource,
    UntagResource,
    UpdateDefaultAutoScalingConfiguration,
    UpdateService,
    UpdateVpcIngressConnection,
}
impl std::fmt::Display for ApprunnerActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApprunnerActions::AssociateCustomDomain => write!(f, "apprunner:AssociateCustomDomain"),
            ApprunnerActions::AssociateWebAcl => write!(f, "apprunner:AssociateWebAcl"),
            ApprunnerActions::CreateAutoScalingConfiguration => {
                write!(f, "apprunner:CreateAutoScalingConfiguration")
            }
            ApprunnerActions::CreateConnection => write!(f, "apprunner:CreateConnection"),
            ApprunnerActions::CreateObservabilityConfiguration => {
                write!(f, "apprunner:CreateObservabilityConfiguration")
            }
            ApprunnerActions::CreateService => write!(f, "apprunner:CreateService"),
            ApprunnerActions::CreateVpcConnector => write!(f, "apprunner:CreateVpcConnector"),
            ApprunnerActions::CreateVpcIngressConnection => {
                write!(f, "apprunner:CreateVpcIngressConnection")
            }
            ApprunnerActions::DeleteAutoScalingConfiguration => {
                write!(f, "apprunner:DeleteAutoScalingConfiguration")
            }
            ApprunnerActions::DeleteConnection => write!(f, "apprunner:DeleteConnection"),
            ApprunnerActions::DeleteObservabilityConfiguration => {
                write!(f, "apprunner:DeleteObservabilityConfiguration")
            }
            ApprunnerActions::DeleteService => write!(f, "apprunner:DeleteService"),
            ApprunnerActions::DeleteVpcConnector => write!(f, "apprunner:DeleteVpcConnector"),
            ApprunnerActions::DeleteVpcIngressConnection => {
                write!(f, "apprunner:DeleteVpcIngressConnection")
            }
            ApprunnerActions::DescribeAutoScalingConfiguration => {
                write!(f, "apprunner:DescribeAutoScalingConfiguration")
            }
            ApprunnerActions::DescribeCustomDomains => write!(f, "apprunner:DescribeCustomDomains"),
            ApprunnerActions::DescribeObservabilityConfiguration => {
                write!(f, "apprunner:DescribeObservabilityConfiguration")
            }
            ApprunnerActions::DescribeOperation => write!(f, "apprunner:DescribeOperation"),
            ApprunnerActions::DescribeService => write!(f, "apprunner:DescribeService"),
            ApprunnerActions::DescribeVpcConnector => write!(f, "apprunner:DescribeVpcConnector"),
            ApprunnerActions::DescribeVpcIngressConnection => {
                write!(f, "apprunner:DescribeVpcIngressConnection")
            }
            ApprunnerActions::DescribeWebAclForService => {
                write!(f, "apprunner:DescribeWebAclForService")
            }
            ApprunnerActions::DisassociateCustomDomain => {
                write!(f, "apprunner:DisassociateCustomDomain")
            }
            ApprunnerActions::DisassociateWebAcl => write!(f, "apprunner:DisassociateWebAcl"),
            ApprunnerActions::ListAssociatedServicesForWebAcl => {
                write!(f, "apprunner:ListAssociatedServicesForWebAcl")
            }
            ApprunnerActions::ListAutoScalingConfigurations => {
                write!(f, "apprunner:ListAutoScalingConfigurations")
            }
            ApprunnerActions::ListConnections => write!(f, "apprunner:ListConnections"),
            ApprunnerActions::ListObservabilityConfigurations => {
                write!(f, "apprunner:ListObservabilityConfigurations")
            }
            ApprunnerActions::ListOperations => write!(f, "apprunner:ListOperations"),
            ApprunnerActions::ListServices => write!(f, "apprunner:ListServices"),
            ApprunnerActions::ListServicesForAutoScalingConfiguration => {
                write!(f, "apprunner:ListServicesForAutoScalingConfiguration")
            }
            ApprunnerActions::ListTagsForResource => write!(f, "apprunner:ListTagsForResource"),
            ApprunnerActions::ListVpcConnectors => write!(f, "apprunner:ListVpcConnectors"),
            ApprunnerActions::ListVpcIngressConnections => {
                write!(f, "apprunner:ListVpcIngressConnections")
            }
            ApprunnerActions::PauseService => write!(f, "apprunner:PauseService"),
            ApprunnerActions::ResumeService => write!(f, "apprunner:ResumeService"),
            ApprunnerActions::StartDeployment => write!(f, "apprunner:StartDeployment"),
            ApprunnerActions::TagResource => write!(f, "apprunner:TagResource"),
            ApprunnerActions::UntagResource => write!(f, "apprunner:UntagResource"),
            ApprunnerActions::UpdateDefaultAutoScalingConfiguration => {
                write!(f, "apprunner:UpdateDefaultAutoScalingConfiguration")
            }
            ApprunnerActions::UpdateService => write!(f, "apprunner:UpdateService"),
            ApprunnerActions::UpdateVpcIngressConnection => {
                write!(f, "apprunner:UpdateVpcIngressConnection")
            }
        }
    }
}
