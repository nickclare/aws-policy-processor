// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum SesActions {
    AllowVendedLogDeliveryForResource,
    BatchGetMetricData,
    CancelExportJob,
    CloneReceiptRuleSet,
    CreateAddonInstance,
    CreateAddonSubscription,
    CreateAddressList,
    CreateAddressListImportJob,
    CreateArchive,
    CreateConfigurationSet,
    CreateConfigurationSetEventDestination,
    CreateConfigurationSetTrackingOptions,
    CreateContact,
    CreateContactList,
    CreateCustomVerificationEmailTemplate,
    CreateDedicatedIpPool,
    CreateDeliverabilityTestReport,
    CreateEmailIdentity,
    CreateEmailIdentityPolicy,
    CreateEmailTemplate,
    CreateExportJob,
    CreateImportJob,
    CreateIngressPoint,
    CreateMultiRegionEndpoint,
    CreateReceiptFilter,
    CreateReceiptRule,
    CreateReceiptRuleSet,
    CreateRelay,
    CreateRuleSet,
    CreateTemplate,
    CreateTenant,
    CreateTenantResourceAssociation,
    CreateTrafficPolicy,
    DeleteAddonInstance,
    DeleteAddonSubscription,
    DeleteAddressList,
    DeleteArchive,
    DeleteConfigurationSet,
    DeleteConfigurationSetEventDestination,
    DeleteConfigurationSetTrackingOptions,
    DeleteContact,
    DeleteContactList,
    DeleteCustomVerificationEmailTemplate,
    DeleteDedicatedIpPool,
    DeleteEmailIdentity,
    DeleteEmailIdentityPolicy,
    DeleteEmailTemplate,
    DeleteIdentity,
    DeleteIdentityPolicy,
    DeleteIngressPoint,
    DeleteMultiRegionEndpoint,
    DeleteReceiptFilter,
    DeleteReceiptRule,
    DeleteReceiptRuleSet,
    DeleteRelay,
    DeleteRuleSet,
    DeleteSuppressedDestination,
    DeleteTemplate,
    DeleteTenant,
    DeleteTenantResourceAssociation,
    DeleteTrafficPolicy,
    DeleteVerifiedEmailAddress,
    DeregisterMemberFromAddressList,
    DescribeActiveReceiptRuleSet,
    DescribeConfigurationSet,
    DescribeReceiptRule,
    DescribeReceiptRuleSet,
    GetAccount,
    GetAccountSendingEnabled,
    GetAddonInstance,
    GetAddonSubscription,
    GetAddressList,
    GetAddressListImportJob,
    GetArchive,
    GetArchiveExport,
    GetArchiveMessage,
    GetArchiveMessageContent,
    GetArchiveSearch,
    GetArchiveSearchResults,
    GetBlacklistReports,
    GetConfigurationSet,
    GetConfigurationSetEventDestinations,
    GetContact,
    GetContactList,
    GetCustomVerificationEmailTemplate,
    GetDedicatedIp,
    GetDedicatedIpPool,
    GetDedicatedIps,
    GetDeliverabilityDashboardOptions,
    GetDeliverabilityTestReport,
    GetDomainDeliverabilityCampaign,
    GetDomainStatisticsReport,
    GetEmailIdentity,
    GetEmailIdentityPolicies,
    GetEmailTemplate,
    GetExportJob,
    GetIdentityDkimAttributes,
    GetIdentityMailFromDomainAttributes,
    GetIdentityNotificationAttributes,
    GetIdentityPolicies,
    GetIdentityVerificationAttributes,
    GetImportJob,
    GetIngressPoint,
    GetMemberOfAddressList,
    GetMessageInsights,
    GetMultiRegionEndpoint,
    GetRelay,
    GetReputationEntity,
    GetRuleSet,
    GetSendQuota,
    GetSendStatistics,
    GetSuppressedDestination,
    GetTemplate,
    GetTenant,
    GetTrafficPolicy,
    ListAddonInstances,
    ListAddonSubscriptions,
    ListAddressListImportJobs,
    ListAddressLists,
    ListArchiveExports,
    ListArchiveSearches,
    ListArchives,
    ListConfigurationSets,
    ListContactLists,
    ListContacts,
    ListCustomVerificationEmailTemplates,
    ListDedicatedIpPools,
    ListDeliverabilityTestReports,
    ListDomainDeliverabilityCampaigns,
    ListEmailIdentities,
    ListEmailTemplates,
    ListExportJobs,
    ListIdentities,
    ListIdentityPolicies,
    ListImportJobs,
    ListIngressPoints,
    ListMembersOfAddressList,
    ListMultiRegionEndpoints,
    ListReceiptFilters,
    ListReceiptRuleSets,
    ListRecommendations,
    ListRelays,
    ListReputationEntities,
    ListResourceTenants,
    ListRuleSets,
    ListSuppressedDestinations,
    ListTagsForResource,
    ListTemplates,
    ListTenantResources,
    ListTenants,
    ListTrafficPolicies,
    ListVerifiedEmailAddresses,
    PutAccountDedicatedIpWarmupAttributes,
    PutAccountDetails,
    PutAccountSendingAttributes,
    PutAccountSuppressionAttributes,
    PutAccountVdmAttributes,
    PutConfigurationSetArchivingOptions,
    PutConfigurationSetDeliveryOptions,
    PutConfigurationSetReputationOptions,
    PutConfigurationSetSendingOptions,
    PutConfigurationSetSuppressionOptions,
    PutConfigurationSetTrackingOptions,
    PutConfigurationSetVdmOptions,
    PutDedicatedIpInPool,
    PutDedicatedIpPoolScalingAttributes,
    PutDedicatedIpWarmupAttributes,
    PutDeliverabilityDashboardOption,
    PutEmailIdentityConfigurationSetAttributes,
    PutEmailIdentityDkimAttributes,
    PutEmailIdentityDkimSigningAttributes,
    PutEmailIdentityFeedbackAttributes,
    PutEmailIdentityMailFromAttributes,
    PutIdentityPolicy,
    PutSuppressedDestination,
    RegisterMemberToAddressList,
    ReorderReceiptRuleSet,
    ReplicateEmailIdentityDkimSigningKey,
    SendBounce,
    SendBulkEmail,
    SendBulkTemplatedEmail,
    SendCustomVerificationEmail,
    SendEmail,
    SendRawEmail,
    SendTemplatedEmail,
    SetActiveReceiptRuleSet,
    SetIdentityDkimEnabled,
    SetIdentityFeedbackForwardingEnabled,
    SetIdentityHeadersInNotificationsEnabled,
    SetIdentityMailFromDomain,
    SetIdentityNotificationTopic,
    SetReceiptRulePosition,
    StartAddressListImportJob,
    StartArchiveExport,
    StartArchiveSearch,
    StopAddressListImportJob,
    StopArchiveExport,
    StopArchiveSearch,
    TagResource,
    TestRenderEmailTemplate,
    TestRenderTemplate,
    UntagResource,
    UpdateAccountSendingEnabled,
    UpdateArchive,
    UpdateConfigurationSetEventDestination,
    UpdateConfigurationSetReputationMetricsEnabled,
    UpdateConfigurationSetSendingEnabled,
    UpdateConfigurationSetTrackingOptions,
    UpdateContact,
    UpdateContactList,
    UpdateCustomVerificationEmailTemplate,
    UpdateEmailIdentityPolicy,
    UpdateEmailTemplate,
    UpdateIngressPoint,
    UpdateReceiptRule,
    UpdateRelay,
    UpdateReputationEntityCustomerManagedStatus,
    UpdateReputationEntityPolicy,
    UpdateRuleSet,
    UpdateTemplate,
    UpdateTrafficPolicy,
    VerifyDomainDkim,
    VerifyDomainIdentity,
    VerifyEmailAddress,
    VerifyEmailIdentity,
}
impl std::fmt::Display for SesActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SesActions::AllowVendedLogDeliveryForResource => {
                write!(f, "ses:AllowVendedLogDeliveryForResource")
            }
            SesActions::BatchGetMetricData => write!(f, "ses:BatchGetMetricData"),
            SesActions::CancelExportJob => write!(f, "ses:CancelExportJob"),
            SesActions::CloneReceiptRuleSet => write!(f, "ses:CloneReceiptRuleSet"),
            SesActions::CreateAddonInstance => write!(f, "ses:CreateAddonInstance"),
            SesActions::CreateAddonSubscription => write!(f, "ses:CreateAddonSubscription"),
            SesActions::CreateAddressList => write!(f, "ses:CreateAddressList"),
            SesActions::CreateAddressListImportJob => write!(f, "ses:CreateAddressListImportJob"),
            SesActions::CreateArchive => write!(f, "ses:CreateArchive"),
            SesActions::CreateConfigurationSet => write!(f, "ses:CreateConfigurationSet"),
            SesActions::CreateConfigurationSetEventDestination => {
                write!(f, "ses:CreateConfigurationSetEventDestination")
            }
            SesActions::CreateConfigurationSetTrackingOptions => {
                write!(f, "ses:CreateConfigurationSetTrackingOptions")
            }
            SesActions::CreateContact => write!(f, "ses:CreateContact"),
            SesActions::CreateContactList => write!(f, "ses:CreateContactList"),
            SesActions::CreateCustomVerificationEmailTemplate => {
                write!(f, "ses:CreateCustomVerificationEmailTemplate")
            }
            SesActions::CreateDedicatedIpPool => write!(f, "ses:CreateDedicatedIpPool"),
            SesActions::CreateDeliverabilityTestReport => {
                write!(f, "ses:CreateDeliverabilityTestReport")
            }
            SesActions::CreateEmailIdentity => write!(f, "ses:CreateEmailIdentity"),
            SesActions::CreateEmailIdentityPolicy => write!(f, "ses:CreateEmailIdentityPolicy"),
            SesActions::CreateEmailTemplate => write!(f, "ses:CreateEmailTemplate"),
            SesActions::CreateExportJob => write!(f, "ses:CreateExportJob"),
            SesActions::CreateImportJob => write!(f, "ses:CreateImportJob"),
            SesActions::CreateIngressPoint => write!(f, "ses:CreateIngressPoint"),
            SesActions::CreateMultiRegionEndpoint => write!(f, "ses:CreateMultiRegionEndpoint"),
            SesActions::CreateReceiptFilter => write!(f, "ses:CreateReceiptFilter"),
            SesActions::CreateReceiptRule => write!(f, "ses:CreateReceiptRule"),
            SesActions::CreateReceiptRuleSet => write!(f, "ses:CreateReceiptRuleSet"),
            SesActions::CreateRelay => write!(f, "ses:CreateRelay"),
            SesActions::CreateRuleSet => write!(f, "ses:CreateRuleSet"),
            SesActions::CreateTemplate => write!(f, "ses:CreateTemplate"),
            SesActions::CreateTenant => write!(f, "ses:CreateTenant"),
            SesActions::CreateTenantResourceAssociation => {
                write!(f, "ses:CreateTenantResourceAssociation")
            }
            SesActions::CreateTrafficPolicy => write!(f, "ses:CreateTrafficPolicy"),
            SesActions::DeleteAddonInstance => write!(f, "ses:DeleteAddonInstance"),
            SesActions::DeleteAddonSubscription => write!(f, "ses:DeleteAddonSubscription"),
            SesActions::DeleteAddressList => write!(f, "ses:DeleteAddressList"),
            SesActions::DeleteArchive => write!(f, "ses:DeleteArchive"),
            SesActions::DeleteConfigurationSet => write!(f, "ses:DeleteConfigurationSet"),
            SesActions::DeleteConfigurationSetEventDestination => {
                write!(f, "ses:DeleteConfigurationSetEventDestination")
            }
            SesActions::DeleteConfigurationSetTrackingOptions => {
                write!(f, "ses:DeleteConfigurationSetTrackingOptions")
            }
            SesActions::DeleteContact => write!(f, "ses:DeleteContact"),
            SesActions::DeleteContactList => write!(f, "ses:DeleteContactList"),
            SesActions::DeleteCustomVerificationEmailTemplate => {
                write!(f, "ses:DeleteCustomVerificationEmailTemplate")
            }
            SesActions::DeleteDedicatedIpPool => write!(f, "ses:DeleteDedicatedIpPool"),
            SesActions::DeleteEmailIdentity => write!(f, "ses:DeleteEmailIdentity"),
            SesActions::DeleteEmailIdentityPolicy => write!(f, "ses:DeleteEmailIdentityPolicy"),
            SesActions::DeleteEmailTemplate => write!(f, "ses:DeleteEmailTemplate"),
            SesActions::DeleteIdentity => write!(f, "ses:DeleteIdentity"),
            SesActions::DeleteIdentityPolicy => write!(f, "ses:DeleteIdentityPolicy"),
            SesActions::DeleteIngressPoint => write!(f, "ses:DeleteIngressPoint"),
            SesActions::DeleteMultiRegionEndpoint => write!(f, "ses:DeleteMultiRegionEndpoint"),
            SesActions::DeleteReceiptFilter => write!(f, "ses:DeleteReceiptFilter"),
            SesActions::DeleteReceiptRule => write!(f, "ses:DeleteReceiptRule"),
            SesActions::DeleteReceiptRuleSet => write!(f, "ses:DeleteReceiptRuleSet"),
            SesActions::DeleteRelay => write!(f, "ses:DeleteRelay"),
            SesActions::DeleteRuleSet => write!(f, "ses:DeleteRuleSet"),
            SesActions::DeleteSuppressedDestination => write!(f, "ses:DeleteSuppressedDestination"),
            SesActions::DeleteTemplate => write!(f, "ses:DeleteTemplate"),
            SesActions::DeleteTenant => write!(f, "ses:DeleteTenant"),
            SesActions::DeleteTenantResourceAssociation => {
                write!(f, "ses:DeleteTenantResourceAssociation")
            }
            SesActions::DeleteTrafficPolicy => write!(f, "ses:DeleteTrafficPolicy"),
            SesActions::DeleteVerifiedEmailAddress => write!(f, "ses:DeleteVerifiedEmailAddress"),
            SesActions::DeregisterMemberFromAddressList => {
                write!(f, "ses:DeregisterMemberFromAddressList")
            }
            SesActions::DescribeActiveReceiptRuleSet => {
                write!(f, "ses:DescribeActiveReceiptRuleSet")
            }
            SesActions::DescribeConfigurationSet => write!(f, "ses:DescribeConfigurationSet"),
            SesActions::DescribeReceiptRule => write!(f, "ses:DescribeReceiptRule"),
            SesActions::DescribeReceiptRuleSet => write!(f, "ses:DescribeReceiptRuleSet"),
            SesActions::GetAccount => write!(f, "ses:GetAccount"),
            SesActions::GetAccountSendingEnabled => write!(f, "ses:GetAccountSendingEnabled"),
            SesActions::GetAddonInstance => write!(f, "ses:GetAddonInstance"),
            SesActions::GetAddonSubscription => write!(f, "ses:GetAddonSubscription"),
            SesActions::GetAddressList => write!(f, "ses:GetAddressList"),
            SesActions::GetAddressListImportJob => write!(f, "ses:GetAddressListImportJob"),
            SesActions::GetArchive => write!(f, "ses:GetArchive"),
            SesActions::GetArchiveExport => write!(f, "ses:GetArchiveExport"),
            SesActions::GetArchiveMessage => write!(f, "ses:GetArchiveMessage"),
            SesActions::GetArchiveMessageContent => write!(f, "ses:GetArchiveMessageContent"),
            SesActions::GetArchiveSearch => write!(f, "ses:GetArchiveSearch"),
            SesActions::GetArchiveSearchResults => write!(f, "ses:GetArchiveSearchResults"),
            SesActions::GetBlacklistReports => write!(f, "ses:GetBlacklistReports"),
            SesActions::GetConfigurationSet => write!(f, "ses:GetConfigurationSet"),
            SesActions::GetConfigurationSetEventDestinations => {
                write!(f, "ses:GetConfigurationSetEventDestinations")
            }
            SesActions::GetContact => write!(f, "ses:GetContact"),
            SesActions::GetContactList => write!(f, "ses:GetContactList"),
            SesActions::GetCustomVerificationEmailTemplate => {
                write!(f, "ses:GetCustomVerificationEmailTemplate")
            }
            SesActions::GetDedicatedIp => write!(f, "ses:GetDedicatedIp"),
            SesActions::GetDedicatedIpPool => write!(f, "ses:GetDedicatedIpPool"),
            SesActions::GetDedicatedIps => write!(f, "ses:GetDedicatedIps"),
            SesActions::GetDeliverabilityDashboardOptions => {
                write!(f, "ses:GetDeliverabilityDashboardOptions")
            }
            SesActions::GetDeliverabilityTestReport => write!(f, "ses:GetDeliverabilityTestReport"),
            SesActions::GetDomainDeliverabilityCampaign => {
                write!(f, "ses:GetDomainDeliverabilityCampaign")
            }
            SesActions::GetDomainStatisticsReport => write!(f, "ses:GetDomainStatisticsReport"),
            SesActions::GetEmailIdentity => write!(f, "ses:GetEmailIdentity"),
            SesActions::GetEmailIdentityPolicies => write!(f, "ses:GetEmailIdentityPolicies"),
            SesActions::GetEmailTemplate => write!(f, "ses:GetEmailTemplate"),
            SesActions::GetExportJob => write!(f, "ses:GetExportJob"),
            SesActions::GetIdentityDkimAttributes => write!(f, "ses:GetIdentityDkimAttributes"),
            SesActions::GetIdentityMailFromDomainAttributes => {
                write!(f, "ses:GetIdentityMailFromDomainAttributes")
            }
            SesActions::GetIdentityNotificationAttributes => {
                write!(f, "ses:GetIdentityNotificationAttributes")
            }
            SesActions::GetIdentityPolicies => write!(f, "ses:GetIdentityPolicies"),
            SesActions::GetIdentityVerificationAttributes => {
                write!(f, "ses:GetIdentityVerificationAttributes")
            }
            SesActions::GetImportJob => write!(f, "ses:GetImportJob"),
            SesActions::GetIngressPoint => write!(f, "ses:GetIngressPoint"),
            SesActions::GetMemberOfAddressList => write!(f, "ses:GetMemberOfAddressList"),
            SesActions::GetMessageInsights => write!(f, "ses:GetMessageInsights"),
            SesActions::GetMultiRegionEndpoint => write!(f, "ses:GetMultiRegionEndpoint"),
            SesActions::GetRelay => write!(f, "ses:GetRelay"),
            SesActions::GetReputationEntity => write!(f, "ses:GetReputationEntity"),
            SesActions::GetRuleSet => write!(f, "ses:GetRuleSet"),
            SesActions::GetSendQuota => write!(f, "ses:GetSendQuota"),
            SesActions::GetSendStatistics => write!(f, "ses:GetSendStatistics"),
            SesActions::GetSuppressedDestination => write!(f, "ses:GetSuppressedDestination"),
            SesActions::GetTemplate => write!(f, "ses:GetTemplate"),
            SesActions::GetTenant => write!(f, "ses:GetTenant"),
            SesActions::GetTrafficPolicy => write!(f, "ses:GetTrafficPolicy"),
            SesActions::ListAddonInstances => write!(f, "ses:ListAddonInstances"),
            SesActions::ListAddonSubscriptions => write!(f, "ses:ListAddonSubscriptions"),
            SesActions::ListAddressListImportJobs => write!(f, "ses:ListAddressListImportJobs"),
            SesActions::ListAddressLists => write!(f, "ses:ListAddressLists"),
            SesActions::ListArchiveExports => write!(f, "ses:ListArchiveExports"),
            SesActions::ListArchiveSearches => write!(f, "ses:ListArchiveSearches"),
            SesActions::ListArchives => write!(f, "ses:ListArchives"),
            SesActions::ListConfigurationSets => write!(f, "ses:ListConfigurationSets"),
            SesActions::ListContactLists => write!(f, "ses:ListContactLists"),
            SesActions::ListContacts => write!(f, "ses:ListContacts"),
            SesActions::ListCustomVerificationEmailTemplates => {
                write!(f, "ses:ListCustomVerificationEmailTemplates")
            }
            SesActions::ListDedicatedIpPools => write!(f, "ses:ListDedicatedIpPools"),
            SesActions::ListDeliverabilityTestReports => {
                write!(f, "ses:ListDeliverabilityTestReports")
            }
            SesActions::ListDomainDeliverabilityCampaigns => {
                write!(f, "ses:ListDomainDeliverabilityCampaigns")
            }
            SesActions::ListEmailIdentities => write!(f, "ses:ListEmailIdentities"),
            SesActions::ListEmailTemplates => write!(f, "ses:ListEmailTemplates"),
            SesActions::ListExportJobs => write!(f, "ses:ListExportJobs"),
            SesActions::ListIdentities => write!(f, "ses:ListIdentities"),
            SesActions::ListIdentityPolicies => write!(f, "ses:ListIdentityPolicies"),
            SesActions::ListImportJobs => write!(f, "ses:ListImportJobs"),
            SesActions::ListIngressPoints => write!(f, "ses:ListIngressPoints"),
            SesActions::ListMembersOfAddressList => write!(f, "ses:ListMembersOfAddressList"),
            SesActions::ListMultiRegionEndpoints => write!(f, "ses:ListMultiRegionEndpoints"),
            SesActions::ListReceiptFilters => write!(f, "ses:ListReceiptFilters"),
            SesActions::ListReceiptRuleSets => write!(f, "ses:ListReceiptRuleSets"),
            SesActions::ListRecommendations => write!(f, "ses:ListRecommendations"),
            SesActions::ListRelays => write!(f, "ses:ListRelays"),
            SesActions::ListReputationEntities => write!(f, "ses:ListReputationEntities"),
            SesActions::ListResourceTenants => write!(f, "ses:ListResourceTenants"),
            SesActions::ListRuleSets => write!(f, "ses:ListRuleSets"),
            SesActions::ListSuppressedDestinations => write!(f, "ses:ListSuppressedDestinations"),
            SesActions::ListTagsForResource => write!(f, "ses:ListTagsForResource"),
            SesActions::ListTemplates => write!(f, "ses:ListTemplates"),
            SesActions::ListTenantResources => write!(f, "ses:ListTenantResources"),
            SesActions::ListTenants => write!(f, "ses:ListTenants"),
            SesActions::ListTrafficPolicies => write!(f, "ses:ListTrafficPolicies"),
            SesActions::ListVerifiedEmailAddresses => write!(f, "ses:ListVerifiedEmailAddresses"),
            SesActions::PutAccountDedicatedIpWarmupAttributes => {
                write!(f, "ses:PutAccountDedicatedIpWarmupAttributes")
            }
            SesActions::PutAccountDetails => write!(f, "ses:PutAccountDetails"),
            SesActions::PutAccountSendingAttributes => write!(f, "ses:PutAccountSendingAttributes"),
            SesActions::PutAccountSuppressionAttributes => {
                write!(f, "ses:PutAccountSuppressionAttributes")
            }
            SesActions::PutAccountVdmAttributes => write!(f, "ses:PutAccountVdmAttributes"),
            SesActions::PutConfigurationSetArchivingOptions => {
                write!(f, "ses:PutConfigurationSetArchivingOptions")
            }
            SesActions::PutConfigurationSetDeliveryOptions => {
                write!(f, "ses:PutConfigurationSetDeliveryOptions")
            }
            SesActions::PutConfigurationSetReputationOptions => {
                write!(f, "ses:PutConfigurationSetReputationOptions")
            }
            SesActions::PutConfigurationSetSendingOptions => {
                write!(f, "ses:PutConfigurationSetSendingOptions")
            }
            SesActions::PutConfigurationSetSuppressionOptions => {
                write!(f, "ses:PutConfigurationSetSuppressionOptions")
            }
            SesActions::PutConfigurationSetTrackingOptions => {
                write!(f, "ses:PutConfigurationSetTrackingOptions")
            }
            SesActions::PutConfigurationSetVdmOptions => {
                write!(f, "ses:PutConfigurationSetVdmOptions")
            }
            SesActions::PutDedicatedIpInPool => write!(f, "ses:PutDedicatedIpInPool"),
            SesActions::PutDedicatedIpPoolScalingAttributes => {
                write!(f, "ses:PutDedicatedIpPoolScalingAttributes")
            }
            SesActions::PutDedicatedIpWarmupAttributes => {
                write!(f, "ses:PutDedicatedIpWarmupAttributes")
            }
            SesActions::PutDeliverabilityDashboardOption => {
                write!(f, "ses:PutDeliverabilityDashboardOption")
            }
            SesActions::PutEmailIdentityConfigurationSetAttributes => {
                write!(f, "ses:PutEmailIdentityConfigurationSetAttributes")
            }
            SesActions::PutEmailIdentityDkimAttributes => {
                write!(f, "ses:PutEmailIdentityDkimAttributes")
            }
            SesActions::PutEmailIdentityDkimSigningAttributes => {
                write!(f, "ses:PutEmailIdentityDkimSigningAttributes")
            }
            SesActions::PutEmailIdentityFeedbackAttributes => {
                write!(f, "ses:PutEmailIdentityFeedbackAttributes")
            }
            SesActions::PutEmailIdentityMailFromAttributes => {
                write!(f, "ses:PutEmailIdentityMailFromAttributes")
            }
            SesActions::PutIdentityPolicy => write!(f, "ses:PutIdentityPolicy"),
            SesActions::PutSuppressedDestination => write!(f, "ses:PutSuppressedDestination"),
            SesActions::RegisterMemberToAddressList => write!(f, "ses:RegisterMemberToAddressList"),
            SesActions::ReorderReceiptRuleSet => write!(f, "ses:ReorderReceiptRuleSet"),
            SesActions::ReplicateEmailIdentityDkimSigningKey => {
                write!(f, "ses:ReplicateEmailIdentityDkimSigningKey")
            }
            SesActions::SendBounce => write!(f, "ses:SendBounce"),
            SesActions::SendBulkEmail => write!(f, "ses:SendBulkEmail"),
            SesActions::SendBulkTemplatedEmail => write!(f, "ses:SendBulkTemplatedEmail"),
            SesActions::SendCustomVerificationEmail => write!(f, "ses:SendCustomVerificationEmail"),
            SesActions::SendEmail => write!(f, "ses:SendEmail"),
            SesActions::SendRawEmail => write!(f, "ses:SendRawEmail"),
            SesActions::SendTemplatedEmail => write!(f, "ses:SendTemplatedEmail"),
            SesActions::SetActiveReceiptRuleSet => write!(f, "ses:SetActiveReceiptRuleSet"),
            SesActions::SetIdentityDkimEnabled => write!(f, "ses:SetIdentityDkimEnabled"),
            SesActions::SetIdentityFeedbackForwardingEnabled => {
                write!(f, "ses:SetIdentityFeedbackForwardingEnabled")
            }
            SesActions::SetIdentityHeadersInNotificationsEnabled => {
                write!(f, "ses:SetIdentityHeadersInNotificationsEnabled")
            }
            SesActions::SetIdentityMailFromDomain => write!(f, "ses:SetIdentityMailFromDomain"),
            SesActions::SetIdentityNotificationTopic => {
                write!(f, "ses:SetIdentityNotificationTopic")
            }
            SesActions::SetReceiptRulePosition => write!(f, "ses:SetReceiptRulePosition"),
            SesActions::StartAddressListImportJob => write!(f, "ses:StartAddressListImportJob"),
            SesActions::StartArchiveExport => write!(f, "ses:StartArchiveExport"),
            SesActions::StartArchiveSearch => write!(f, "ses:StartArchiveSearch"),
            SesActions::StopAddressListImportJob => write!(f, "ses:StopAddressListImportJob"),
            SesActions::StopArchiveExport => write!(f, "ses:StopArchiveExport"),
            SesActions::StopArchiveSearch => write!(f, "ses:StopArchiveSearch"),
            SesActions::TagResource => write!(f, "ses:TagResource"),
            SesActions::TestRenderEmailTemplate => write!(f, "ses:TestRenderEmailTemplate"),
            SesActions::TestRenderTemplate => write!(f, "ses:TestRenderTemplate"),
            SesActions::UntagResource => write!(f, "ses:UntagResource"),
            SesActions::UpdateAccountSendingEnabled => write!(f, "ses:UpdateAccountSendingEnabled"),
            SesActions::UpdateArchive => write!(f, "ses:UpdateArchive"),
            SesActions::UpdateConfigurationSetEventDestination => {
                write!(f, "ses:UpdateConfigurationSetEventDestination")
            }
            SesActions::UpdateConfigurationSetReputationMetricsEnabled => {
                write!(f, "ses:UpdateConfigurationSetReputationMetricsEnabled")
            }
            SesActions::UpdateConfigurationSetSendingEnabled => {
                write!(f, "ses:UpdateConfigurationSetSendingEnabled")
            }
            SesActions::UpdateConfigurationSetTrackingOptions => {
                write!(f, "ses:UpdateConfigurationSetTrackingOptions")
            }
            SesActions::UpdateContact => write!(f, "ses:UpdateContact"),
            SesActions::UpdateContactList => write!(f, "ses:UpdateContactList"),
            SesActions::UpdateCustomVerificationEmailTemplate => {
                write!(f, "ses:UpdateCustomVerificationEmailTemplate")
            }
            SesActions::UpdateEmailIdentityPolicy => write!(f, "ses:UpdateEmailIdentityPolicy"),
            SesActions::UpdateEmailTemplate => write!(f, "ses:UpdateEmailTemplate"),
            SesActions::UpdateIngressPoint => write!(f, "ses:UpdateIngressPoint"),
            SesActions::UpdateReceiptRule => write!(f, "ses:UpdateReceiptRule"),
            SesActions::UpdateRelay => write!(f, "ses:UpdateRelay"),
            SesActions::UpdateReputationEntityCustomerManagedStatus => {
                write!(f, "ses:UpdateReputationEntityCustomerManagedStatus")
            }
            SesActions::UpdateReputationEntityPolicy => {
                write!(f, "ses:UpdateReputationEntityPolicy")
            }
            SesActions::UpdateRuleSet => write!(f, "ses:UpdateRuleSet"),
            SesActions::UpdateTemplate => write!(f, "ses:UpdateTemplate"),
            SesActions::UpdateTrafficPolicy => write!(f, "ses:UpdateTrafficPolicy"),
            SesActions::VerifyDomainDkim => write!(f, "ses:VerifyDomainDkim"),
            SesActions::VerifyDomainIdentity => write!(f, "ses:VerifyDomainIdentity"),
            SesActions::VerifyEmailAddress => write!(f, "ses:VerifyEmailAddress"),
            SesActions::VerifyEmailIdentity => write!(f, "ses:VerifyEmailIdentity"),
        }
    }
}
