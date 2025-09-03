// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum WorkmailActions {
    AllowVendedLogDeliveryForResource,
    AssociateDelegateToResource,
    AssociateMemberToGroup,
    AssumeImpersonationRole,
    CancelMailboxExportJob,
    CreateAlias,
    CreateAvailabilityConfiguration,
    CreateGroup,
    CreateIdentityCenterApplication,
    CreateImpersonationRole,
    CreateInboundMailFlowRule,
    CreateMailDomain,
    CreateMobileDeviceAccessRule,
    CreateOrganization,
    CreateOutboundMailFlowRule,
    CreateResource,
    CreateSmtpGateway,
    CreateUser,
    DeleteAccessControlRule,
    DeleteAlias,
    DeleteAvailabilityConfiguration,
    DeleteEmailMonitoringConfiguration,
    DeleteGroup,
    DeleteIdentityCenterApplication,
    DeleteIdentityProviderConfiguration,
    DeleteImpersonationRole,
    DeleteInboundMailFlowRule,
    DeleteMailDomain,
    DeleteMailboxPermissions,
    DeleteMobileDevice,
    DeleteMobileDeviceAccessOverride,
    DeleteMobileDeviceAccessRule,
    DeleteOrganization,
    DeleteOutboundMailFlowRule,
    DeletePersonalAccessToken,
    DeleteResource,
    DeleteRetentionPolicy,
    DeleteSmtpGateway,
    DeleteUser,
    DeliverToMailbox,
    DeregisterFromWorkMail,
    DeregisterMailDomain,
    DescribeEmailMonitoringConfiguration,
    DescribeEntity,
    DescribeGroup,
    DescribeIdentityProviderConfiguration,
    DescribeInboundDmarcSettings,
    DescribeInboundMailFlowRule,
    DescribeMailDomains,
    DescribeMailboxExportJob,
    DescribeOrganization,
    DescribeOutboundMailFlowRule,
    DescribeResource,
    DescribeSmtpGateway,
    DescribeUser,
    DisassociateDelegateFromResource,
    DisassociateMemberFromGroup,
    EnableMailDomain,
    GetAccessControlEffect,
    GetDefaultRetentionPolicy,
    GetImpersonationRole,
    GetImpersonationRoleEffect,
    GetJournalingRules,
    GetMailDomain,
    GetMailDomainDetails,
    GetMailboxDetails,
    GetMobileDeviceAccessEffect,
    GetMobileDeviceAccessOverride,
    GetMobileDeviceDetails,
    GetMobileDevicesForUser,
    GetMobilePolicyDetails,
    GetPersonalAccessTokenMetadata,
    ListAccessControlRules,
    ListAliases,
    ListAvailabilityConfigurations,
    ListGroupMembers,
    ListGroups,
    ListGroupsForEntity,
    ListImpersonationRoles,
    ListInboundMailFlowRules,
    ListMailDomains,
    ListMailboxExportJobs,
    ListMailboxPermissions,
    ListMobileDeviceAccessOverrides,
    ListMobileDeviceAccessRules,
    ListOrganizations,
    ListOutboundMailFlowRules,
    ListPersonalAccessTokens,
    ListResourceDelegates,
    ListResources,
    ListSmtpGateways,
    ListTagsForResource,
    ListUsers,
    PutAccessControlRule,
    PutEmailMonitoringConfiguration,
    PutIdentityProviderConfiguration,
    PutInboundDmarcSettings,
    PutMailboxPermissions,
    PutMobileDeviceAccessOverride,
    PutRetentionPolicy,
    RegisterMailDomain,
    RegisterToWorkMail,
    ResetPassword,
    SearchMembers,
    SetDefaultMailDomain,
    SetJournalingRules,
    SetMobilePolicyDetails,
    StartMailboxExportJob,
    TagResource,
    TestAvailabilityConfiguration,
    TestInboundMailFlowRules,
    TestOutboundMailFlowRules,
    UntagResource,
    UpdateAvailabilityConfiguration,
    UpdateDefaultMailDomain,
    UpdateGroup,
    UpdateImpersonationRole,
    UpdateInboundMailFlowRule,
    UpdateMailboxQuota,
    UpdateMobileDeviceAccessRule,
    UpdateOutboundMailFlowRule,
    UpdatePrimaryEmailAddress,
    UpdateResource,
    UpdateSmtpGateway,
    UpdateUser,
    WipeMobileDevice,
}
impl std::fmt::Display for WorkmailActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WorkmailActions::AllowVendedLogDeliveryForResource => {
                write!(f, "workmail:AllowVendedLogDeliveryForResource")
            }
            WorkmailActions::AssociateDelegateToResource => {
                write!(f, "workmail:AssociateDelegateToResource")
            }
            WorkmailActions::AssociateMemberToGroup => write!(f, "workmail:AssociateMemberToGroup"),
            WorkmailActions::AssumeImpersonationRole => {
                write!(f, "workmail:AssumeImpersonationRole")
            }
            WorkmailActions::CancelMailboxExportJob => write!(f, "workmail:CancelMailboxExportJob"),
            WorkmailActions::CreateAlias => write!(f, "workmail:CreateAlias"),
            WorkmailActions::CreateAvailabilityConfiguration => {
                write!(f, "workmail:CreateAvailabilityConfiguration")
            }
            WorkmailActions::CreateGroup => write!(f, "workmail:CreateGroup"),
            WorkmailActions::CreateIdentityCenterApplication => {
                write!(f, "workmail:CreateIdentityCenterApplication")
            }
            WorkmailActions::CreateImpersonationRole => {
                write!(f, "workmail:CreateImpersonationRole")
            }
            WorkmailActions::CreateInboundMailFlowRule => {
                write!(f, "workmail:CreateInboundMailFlowRule")
            }
            WorkmailActions::CreateMailDomain => write!(f, "workmail:CreateMailDomain"),
            WorkmailActions::CreateMobileDeviceAccessRule => {
                write!(f, "workmail:CreateMobileDeviceAccessRule")
            }
            WorkmailActions::CreateOrganization => write!(f, "workmail:CreateOrganization"),
            WorkmailActions::CreateOutboundMailFlowRule => {
                write!(f, "workmail:CreateOutboundMailFlowRule")
            }
            WorkmailActions::CreateResource => write!(f, "workmail:CreateResource"),
            WorkmailActions::CreateSmtpGateway => write!(f, "workmail:CreateSmtpGateway"),
            WorkmailActions::CreateUser => write!(f, "workmail:CreateUser"),
            WorkmailActions::DeleteAccessControlRule => {
                write!(f, "workmail:DeleteAccessControlRule")
            }
            WorkmailActions::DeleteAlias => write!(f, "workmail:DeleteAlias"),
            WorkmailActions::DeleteAvailabilityConfiguration => {
                write!(f, "workmail:DeleteAvailabilityConfiguration")
            }
            WorkmailActions::DeleteEmailMonitoringConfiguration => {
                write!(f, "workmail:DeleteEmailMonitoringConfiguration")
            }
            WorkmailActions::DeleteGroup => write!(f, "workmail:DeleteGroup"),
            WorkmailActions::DeleteIdentityCenterApplication => {
                write!(f, "workmail:DeleteIdentityCenterApplication")
            }
            WorkmailActions::DeleteIdentityProviderConfiguration => {
                write!(f, "workmail:DeleteIdentityProviderConfiguration")
            }
            WorkmailActions::DeleteImpersonationRole => {
                write!(f, "workmail:DeleteImpersonationRole")
            }
            WorkmailActions::DeleteInboundMailFlowRule => {
                write!(f, "workmail:DeleteInboundMailFlowRule")
            }
            WorkmailActions::DeleteMailDomain => write!(f, "workmail:DeleteMailDomain"),
            WorkmailActions::DeleteMailboxPermissions => {
                write!(f, "workmail:DeleteMailboxPermissions")
            }
            WorkmailActions::DeleteMobileDevice => write!(f, "workmail:DeleteMobileDevice"),
            WorkmailActions::DeleteMobileDeviceAccessOverride => {
                write!(f, "workmail:DeleteMobileDeviceAccessOverride")
            }
            WorkmailActions::DeleteMobileDeviceAccessRule => {
                write!(f, "workmail:DeleteMobileDeviceAccessRule")
            }
            WorkmailActions::DeleteOrganization => write!(f, "workmail:DeleteOrganization"),
            WorkmailActions::DeleteOutboundMailFlowRule => {
                write!(f, "workmail:DeleteOutboundMailFlowRule")
            }
            WorkmailActions::DeletePersonalAccessToken => {
                write!(f, "workmail:DeletePersonalAccessToken")
            }
            WorkmailActions::DeleteResource => write!(f, "workmail:DeleteResource"),
            WorkmailActions::DeleteRetentionPolicy => write!(f, "workmail:DeleteRetentionPolicy"),
            WorkmailActions::DeleteSmtpGateway => write!(f, "workmail:DeleteSmtpGateway"),
            WorkmailActions::DeleteUser => write!(f, "workmail:DeleteUser"),
            WorkmailActions::DeliverToMailbox => write!(f, "workmail:DeliverToMailbox"),
            WorkmailActions::DeregisterFromWorkMail => write!(f, "workmail:DeregisterFromWorkMail"),
            WorkmailActions::DeregisterMailDomain => write!(f, "workmail:DeregisterMailDomain"),
            WorkmailActions::DescribeEmailMonitoringConfiguration => {
                write!(f, "workmail:DescribeEmailMonitoringConfiguration")
            }
            WorkmailActions::DescribeEntity => write!(f, "workmail:DescribeEntity"),
            WorkmailActions::DescribeGroup => write!(f, "workmail:DescribeGroup"),
            WorkmailActions::DescribeIdentityProviderConfiguration => {
                write!(f, "workmail:DescribeIdentityProviderConfiguration")
            }
            WorkmailActions::DescribeInboundDmarcSettings => {
                write!(f, "workmail:DescribeInboundDmarcSettings")
            }
            WorkmailActions::DescribeInboundMailFlowRule => {
                write!(f, "workmail:DescribeInboundMailFlowRule")
            }
            WorkmailActions::DescribeMailDomains => write!(f, "workmail:DescribeMailDomains"),
            WorkmailActions::DescribeMailboxExportJob => {
                write!(f, "workmail:DescribeMailboxExportJob")
            }
            WorkmailActions::DescribeOrganization => write!(f, "workmail:DescribeOrganization"),
            WorkmailActions::DescribeOutboundMailFlowRule => {
                write!(f, "workmail:DescribeOutboundMailFlowRule")
            }
            WorkmailActions::DescribeResource => write!(f, "workmail:DescribeResource"),
            WorkmailActions::DescribeSmtpGateway => write!(f, "workmail:DescribeSmtpGateway"),
            WorkmailActions::DescribeUser => write!(f, "workmail:DescribeUser"),
            WorkmailActions::DisassociateDelegateFromResource => {
                write!(f, "workmail:DisassociateDelegateFromResource")
            }
            WorkmailActions::DisassociateMemberFromGroup => {
                write!(f, "workmail:DisassociateMemberFromGroup")
            }
            WorkmailActions::EnableMailDomain => write!(f, "workmail:EnableMailDomain"),
            WorkmailActions::GetAccessControlEffect => write!(f, "workmail:GetAccessControlEffect"),
            WorkmailActions::GetDefaultRetentionPolicy => {
                write!(f, "workmail:GetDefaultRetentionPolicy")
            }
            WorkmailActions::GetImpersonationRole => write!(f, "workmail:GetImpersonationRole"),
            WorkmailActions::GetImpersonationRoleEffect => {
                write!(f, "workmail:GetImpersonationRoleEffect")
            }
            WorkmailActions::GetJournalingRules => write!(f, "workmail:GetJournalingRules"),
            WorkmailActions::GetMailDomain => write!(f, "workmail:GetMailDomain"),
            WorkmailActions::GetMailDomainDetails => write!(f, "workmail:GetMailDomainDetails"),
            WorkmailActions::GetMailboxDetails => write!(f, "workmail:GetMailboxDetails"),
            WorkmailActions::GetMobileDeviceAccessEffect => {
                write!(f, "workmail:GetMobileDeviceAccessEffect")
            }
            WorkmailActions::GetMobileDeviceAccessOverride => {
                write!(f, "workmail:GetMobileDeviceAccessOverride")
            }
            WorkmailActions::GetMobileDeviceDetails => write!(f, "workmail:GetMobileDeviceDetails"),
            WorkmailActions::GetMobileDevicesForUser => {
                write!(f, "workmail:GetMobileDevicesForUser")
            }
            WorkmailActions::GetMobilePolicyDetails => write!(f, "workmail:GetMobilePolicyDetails"),
            WorkmailActions::GetPersonalAccessTokenMetadata => {
                write!(f, "workmail:GetPersonalAccessTokenMetadata")
            }
            WorkmailActions::ListAccessControlRules => write!(f, "workmail:ListAccessControlRules"),
            WorkmailActions::ListAliases => write!(f, "workmail:ListAliases"),
            WorkmailActions::ListAvailabilityConfigurations => {
                write!(f, "workmail:ListAvailabilityConfigurations")
            }
            WorkmailActions::ListGroupMembers => write!(f, "workmail:ListGroupMembers"),
            WorkmailActions::ListGroups => write!(f, "workmail:ListGroups"),
            WorkmailActions::ListGroupsForEntity => write!(f, "workmail:ListGroupsForEntity"),
            WorkmailActions::ListImpersonationRoles => write!(f, "workmail:ListImpersonationRoles"),
            WorkmailActions::ListInboundMailFlowRules => {
                write!(f, "workmail:ListInboundMailFlowRules")
            }
            WorkmailActions::ListMailDomains => write!(f, "workmail:ListMailDomains"),
            WorkmailActions::ListMailboxExportJobs => write!(f, "workmail:ListMailboxExportJobs"),
            WorkmailActions::ListMailboxPermissions => write!(f, "workmail:ListMailboxPermissions"),
            WorkmailActions::ListMobileDeviceAccessOverrides => {
                write!(f, "workmail:ListMobileDeviceAccessOverrides")
            }
            WorkmailActions::ListMobileDeviceAccessRules => {
                write!(f, "workmail:ListMobileDeviceAccessRules")
            }
            WorkmailActions::ListOrganizations => write!(f, "workmail:ListOrganizations"),
            WorkmailActions::ListOutboundMailFlowRules => {
                write!(f, "workmail:ListOutboundMailFlowRules")
            }
            WorkmailActions::ListPersonalAccessTokens => {
                write!(f, "workmail:ListPersonalAccessTokens")
            }
            WorkmailActions::ListResourceDelegates => write!(f, "workmail:ListResourceDelegates"),
            WorkmailActions::ListResources => write!(f, "workmail:ListResources"),
            WorkmailActions::ListSmtpGateways => write!(f, "workmail:ListSmtpGateways"),
            WorkmailActions::ListTagsForResource => write!(f, "workmail:ListTagsForResource"),
            WorkmailActions::ListUsers => write!(f, "workmail:ListUsers"),
            WorkmailActions::PutAccessControlRule => write!(f, "workmail:PutAccessControlRule"),
            WorkmailActions::PutEmailMonitoringConfiguration => {
                write!(f, "workmail:PutEmailMonitoringConfiguration")
            }
            WorkmailActions::PutIdentityProviderConfiguration => {
                write!(f, "workmail:PutIdentityProviderConfiguration")
            }
            WorkmailActions::PutInboundDmarcSettings => {
                write!(f, "workmail:PutInboundDmarcSettings")
            }
            WorkmailActions::PutMailboxPermissions => write!(f, "workmail:PutMailboxPermissions"),
            WorkmailActions::PutMobileDeviceAccessOverride => {
                write!(f, "workmail:PutMobileDeviceAccessOverride")
            }
            WorkmailActions::PutRetentionPolicy => write!(f, "workmail:PutRetentionPolicy"),
            WorkmailActions::RegisterMailDomain => write!(f, "workmail:RegisterMailDomain"),
            WorkmailActions::RegisterToWorkMail => write!(f, "workmail:RegisterToWorkMail"),
            WorkmailActions::ResetPassword => write!(f, "workmail:ResetPassword"),
            WorkmailActions::SearchMembers => write!(f, "workmail:SearchMembers"),
            WorkmailActions::SetDefaultMailDomain => write!(f, "workmail:SetDefaultMailDomain"),
            WorkmailActions::SetJournalingRules => write!(f, "workmail:SetJournalingRules"),
            WorkmailActions::SetMobilePolicyDetails => write!(f, "workmail:SetMobilePolicyDetails"),
            WorkmailActions::StartMailboxExportJob => write!(f, "workmail:StartMailboxExportJob"),
            WorkmailActions::TagResource => write!(f, "workmail:TagResource"),
            WorkmailActions::TestAvailabilityConfiguration => {
                write!(f, "workmail:TestAvailabilityConfiguration")
            }
            WorkmailActions::TestInboundMailFlowRules => {
                write!(f, "workmail:TestInboundMailFlowRules")
            }
            WorkmailActions::TestOutboundMailFlowRules => {
                write!(f, "workmail:TestOutboundMailFlowRules")
            }
            WorkmailActions::UntagResource => write!(f, "workmail:UntagResource"),
            WorkmailActions::UpdateAvailabilityConfiguration => {
                write!(f, "workmail:UpdateAvailabilityConfiguration")
            }
            WorkmailActions::UpdateDefaultMailDomain => {
                write!(f, "workmail:UpdateDefaultMailDomain")
            }
            WorkmailActions::UpdateGroup => write!(f, "workmail:UpdateGroup"),
            WorkmailActions::UpdateImpersonationRole => {
                write!(f, "workmail:UpdateImpersonationRole")
            }
            WorkmailActions::UpdateInboundMailFlowRule => {
                write!(f, "workmail:UpdateInboundMailFlowRule")
            }
            WorkmailActions::UpdateMailboxQuota => write!(f, "workmail:UpdateMailboxQuota"),
            WorkmailActions::UpdateMobileDeviceAccessRule => {
                write!(f, "workmail:UpdateMobileDeviceAccessRule")
            }
            WorkmailActions::UpdateOutboundMailFlowRule => {
                write!(f, "workmail:UpdateOutboundMailFlowRule")
            }
            WorkmailActions::UpdatePrimaryEmailAddress => {
                write!(f, "workmail:UpdatePrimaryEmailAddress")
            }
            WorkmailActions::UpdateResource => write!(f, "workmail:UpdateResource"),
            WorkmailActions::UpdateSmtpGateway => write!(f, "workmail:UpdateSmtpGateway"),
            WorkmailActions::UpdateUser => write!(f, "workmail:UpdateUser"),
            WorkmailActions::WipeMobileDevice => write!(f, "workmail:WipeMobileDevice"),
        }
    }
}
