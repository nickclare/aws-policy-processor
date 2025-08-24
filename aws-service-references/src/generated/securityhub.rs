// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum SecurityhubActions {
    AcceptAdministratorInvitation,
    AcceptInvitation,
    BatchDeleteAutomationRules,
    BatchDisableStandards,
    BatchEnableStandards,
    BatchGetAutomationRules,
    BatchGetConfigurationPolicyAssociations,
    BatchGetControlEvaluations,
    BatchGetSecurityControls,
    BatchGetStandardsControlAssociations,
    BatchImportFindings,
    BatchUpdateAutomationRules,
    BatchUpdateFindings,
    BatchUpdateStandardsControlAssociations,
    ConnectorRegistrationsV2,
    CreateActionTarget,
    CreateAggregatorV2,
    CreateAutomationRule,
    CreateAutomationRuleV2,
    CreateConfigurationPolicy,
    CreateConnectorV2,
    CreateFindingAggregator,
    CreateInsight,
    CreateMembers,
    CreateTicketV2,
    DeclineInvitations,
    DeleteActionTarget,
    DeleteAggregatorV2,
    DeleteAutomationRuleV2,
    DeleteConfigurationPolicy,
    DeleteConnectorV2,
    DeleteFindingAggregator,
    DeleteInsight,
    DeleteInvitations,
    DeleteMembers,
    DescribeActionTargets,
    DescribeHub,
    DescribeOrganizationConfiguration,
    DescribeProducts,
    DescribeProductsV2,
    DescribeSecurityHubV2,
    DescribeStandards,
    DescribeStandardsControls,
    DisableImportFindingsForProduct,
    DisableOrganizationAdminAccount,
    DisableSecurityHub,
    DisableSecurityHubV2,
    DisassociateFromAdministratorAccount,
    DisassociateFromMasterAccount,
    DisassociateMembers,
    EnableImportFindingsForProduct,
    EnableOrganizationAdminAccount,
    EnableSecurityHub,
    EnableSecurityHubV2,
    GetAdhocInsightResults,
    GetAdministratorAccount,
    GetAggregatorV2,
    GetAutomationRuleV2,
    GetConfigurationPolicy,
    GetConfigurationPolicyAssociation,
    GetConnectorV2,
    GetControlFindingSummary,
    GetEnabledStandards,
    GetFindingAggregator,
    GetFindingHistory,
    GetFindings,
    GetFreeTrialEndDate,
    GetFreeTrialUsage,
    GetInsightFindingTrend,
    GetInsightResults,
    GetInsights,
    GetInvitationsCount,
    GetMasterAccount,
    GetMembers,
    GetResourcesStatisticsV2,
    GetResourcesV2,
    GetSecurityControlDefinition,
    GetUsage,
    InviteMembers,
    ListAggregatorsV2,
    ListAutomationRules,
    ListAutomationRulesV2,
    ListConfigurationPolicies,
    ListConfigurationPolicyAssociations,
    ListConnectorsV2,
    ListControlEvaluationSummaries,
    ListEnabledProductsForImport,
    ListFindingAggregators,
    ListInvitations,
    ListMembers,
    ListOrganizationAdminAccounts,
    ListSecurityControlDefinitions,
    ListStandardsControlAssociations,
    ListTagsForResource,
    SendFindingEvents,
    SendInsightEvents,
    StartConfigurationPolicyAssociation,
    StartConfigurationPolicyDisassociation,
    TagResource,
    UntagResource,
    UpdateActionTarget,
    UpdateAggregatorV2,
    UpdateAutomationRuleV2,
    UpdateConfigurationPolicy,
    UpdateConnectorV2,
    UpdateFindingAggregator,
    UpdateFindings,
    UpdateInsight,
    UpdateOrganizationConfiguration,
    UpdateSecurityControl,
    UpdateSecurityHubConfiguration,
    UpdateStandardsControl,
}
impl std::fmt::Display for SecurityhubActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SecurityhubActions::AcceptAdministratorInvitation => {
                write!(f, "securityhub:AcceptAdministratorInvitation")
            }
            SecurityhubActions::AcceptInvitation => write!(f, "securityhub:AcceptInvitation"),
            SecurityhubActions::BatchDeleteAutomationRules => {
                write!(f, "securityhub:BatchDeleteAutomationRules")
            }
            SecurityhubActions::BatchDisableStandards => {
                write!(f, "securityhub:BatchDisableStandards")
            }
            SecurityhubActions::BatchEnableStandards => {
                write!(f, "securityhub:BatchEnableStandards")
            }
            SecurityhubActions::BatchGetAutomationRules => {
                write!(f, "securityhub:BatchGetAutomationRules")
            }
            SecurityhubActions::BatchGetConfigurationPolicyAssociations => {
                write!(f, "securityhub:BatchGetConfigurationPolicyAssociations")
            }
            SecurityhubActions::BatchGetControlEvaluations => {
                write!(f, "securityhub:BatchGetControlEvaluations")
            }
            SecurityhubActions::BatchGetSecurityControls => {
                write!(f, "securityhub:BatchGetSecurityControls")
            }
            SecurityhubActions::BatchGetStandardsControlAssociations => {
                write!(f, "securityhub:BatchGetStandardsControlAssociations")
            }
            SecurityhubActions::BatchImportFindings => write!(f, "securityhub:BatchImportFindings"),
            SecurityhubActions::BatchUpdateAutomationRules => {
                write!(f, "securityhub:BatchUpdateAutomationRules")
            }
            SecurityhubActions::BatchUpdateFindings => write!(f, "securityhub:BatchUpdateFindings"),
            SecurityhubActions::BatchUpdateStandardsControlAssociations => {
                write!(f, "securityhub:BatchUpdateStandardsControlAssociations")
            }
            SecurityhubActions::ConnectorRegistrationsV2 => {
                write!(f, "securityhub:ConnectorRegistrationsV2")
            }
            SecurityhubActions::CreateActionTarget => write!(f, "securityhub:CreateActionTarget"),
            SecurityhubActions::CreateAggregatorV2 => write!(f, "securityhub:CreateAggregatorV2"),
            SecurityhubActions::CreateAutomationRule => {
                write!(f, "securityhub:CreateAutomationRule")
            }
            SecurityhubActions::CreateAutomationRuleV2 => {
                write!(f, "securityhub:CreateAutomationRuleV2")
            }
            SecurityhubActions::CreateConfigurationPolicy => {
                write!(f, "securityhub:CreateConfigurationPolicy")
            }
            SecurityhubActions::CreateConnectorV2 => write!(f, "securityhub:CreateConnectorV2"),
            SecurityhubActions::CreateFindingAggregator => {
                write!(f, "securityhub:CreateFindingAggregator")
            }
            SecurityhubActions::CreateInsight => write!(f, "securityhub:CreateInsight"),
            SecurityhubActions::CreateMembers => write!(f, "securityhub:CreateMembers"),
            SecurityhubActions::CreateTicketV2 => write!(f, "securityhub:CreateTicketV2"),
            SecurityhubActions::DeclineInvitations => write!(f, "securityhub:DeclineInvitations"),
            SecurityhubActions::DeleteActionTarget => write!(f, "securityhub:DeleteActionTarget"),
            SecurityhubActions::DeleteAggregatorV2 => write!(f, "securityhub:DeleteAggregatorV2"),
            SecurityhubActions::DeleteAutomationRuleV2 => {
                write!(f, "securityhub:DeleteAutomationRuleV2")
            }
            SecurityhubActions::DeleteConfigurationPolicy => {
                write!(f, "securityhub:DeleteConfigurationPolicy")
            }
            SecurityhubActions::DeleteConnectorV2 => write!(f, "securityhub:DeleteConnectorV2"),
            SecurityhubActions::DeleteFindingAggregator => {
                write!(f, "securityhub:DeleteFindingAggregator")
            }
            SecurityhubActions::DeleteInsight => write!(f, "securityhub:DeleteInsight"),
            SecurityhubActions::DeleteInvitations => write!(f, "securityhub:DeleteInvitations"),
            SecurityhubActions::DeleteMembers => write!(f, "securityhub:DeleteMembers"),
            SecurityhubActions::DescribeActionTargets => {
                write!(f, "securityhub:DescribeActionTargets")
            }
            SecurityhubActions::DescribeHub => write!(f, "securityhub:DescribeHub"),
            SecurityhubActions::DescribeOrganizationConfiguration => {
                write!(f, "securityhub:DescribeOrganizationConfiguration")
            }
            SecurityhubActions::DescribeProducts => write!(f, "securityhub:DescribeProducts"),
            SecurityhubActions::DescribeProductsV2 => write!(f, "securityhub:DescribeProductsV2"),
            SecurityhubActions::DescribeSecurityHubV2 => {
                write!(f, "securityhub:DescribeSecurityHubV2")
            }
            SecurityhubActions::DescribeStandards => write!(f, "securityhub:DescribeStandards"),
            SecurityhubActions::DescribeStandardsControls => {
                write!(f, "securityhub:DescribeStandardsControls")
            }
            SecurityhubActions::DisableImportFindingsForProduct => {
                write!(f, "securityhub:DisableImportFindingsForProduct")
            }
            SecurityhubActions::DisableOrganizationAdminAccount => {
                write!(f, "securityhub:DisableOrganizationAdminAccount")
            }
            SecurityhubActions::DisableSecurityHub => write!(f, "securityhub:DisableSecurityHub"),
            SecurityhubActions::DisableSecurityHubV2 => {
                write!(f, "securityhub:DisableSecurityHubV2")
            }
            SecurityhubActions::DisassociateFromAdministratorAccount => {
                write!(f, "securityhub:DisassociateFromAdministratorAccount")
            }
            SecurityhubActions::DisassociateFromMasterAccount => {
                write!(f, "securityhub:DisassociateFromMasterAccount")
            }
            SecurityhubActions::DisassociateMembers => write!(f, "securityhub:DisassociateMembers"),
            SecurityhubActions::EnableImportFindingsForProduct => {
                write!(f, "securityhub:EnableImportFindingsForProduct")
            }
            SecurityhubActions::EnableOrganizationAdminAccount => {
                write!(f, "securityhub:EnableOrganizationAdminAccount")
            }
            SecurityhubActions::EnableSecurityHub => write!(f, "securityhub:EnableSecurityHub"),
            SecurityhubActions::EnableSecurityHubV2 => write!(f, "securityhub:EnableSecurityHubV2"),
            SecurityhubActions::GetAdhocInsightResults => {
                write!(f, "securityhub:GetAdhocInsightResults")
            }
            SecurityhubActions::GetAdministratorAccount => {
                write!(f, "securityhub:GetAdministratorAccount")
            }
            SecurityhubActions::GetAggregatorV2 => write!(f, "securityhub:GetAggregatorV2"),
            SecurityhubActions::GetAutomationRuleV2 => write!(f, "securityhub:GetAutomationRuleV2"),
            SecurityhubActions::GetConfigurationPolicy => {
                write!(f, "securityhub:GetConfigurationPolicy")
            }
            SecurityhubActions::GetConfigurationPolicyAssociation => {
                write!(f, "securityhub:GetConfigurationPolicyAssociation")
            }
            SecurityhubActions::GetConnectorV2 => write!(f, "securityhub:GetConnectorV2"),
            SecurityhubActions::GetControlFindingSummary => {
                write!(f, "securityhub:GetControlFindingSummary")
            }
            SecurityhubActions::GetEnabledStandards => write!(f, "securityhub:GetEnabledStandards"),
            SecurityhubActions::GetFindingAggregator => {
                write!(f, "securityhub:GetFindingAggregator")
            }
            SecurityhubActions::GetFindingHistory => write!(f, "securityhub:GetFindingHistory"),
            SecurityhubActions::GetFindings => write!(f, "securityhub:GetFindings"),
            SecurityhubActions::GetFreeTrialEndDate => write!(f, "securityhub:GetFreeTrialEndDate"),
            SecurityhubActions::GetFreeTrialUsage => write!(f, "securityhub:GetFreeTrialUsage"),
            SecurityhubActions::GetInsightFindingTrend => {
                write!(f, "securityhub:GetInsightFindingTrend")
            }
            SecurityhubActions::GetInsightResults => write!(f, "securityhub:GetInsightResults"),
            SecurityhubActions::GetInsights => write!(f, "securityhub:GetInsights"),
            SecurityhubActions::GetInvitationsCount => write!(f, "securityhub:GetInvitationsCount"),
            SecurityhubActions::GetMasterAccount => write!(f, "securityhub:GetMasterAccount"),
            SecurityhubActions::GetMembers => write!(f, "securityhub:GetMembers"),
            SecurityhubActions::GetResourcesStatisticsV2 => {
                write!(f, "securityhub:GetResourcesStatisticsV2")
            }
            SecurityhubActions::GetResourcesV2 => write!(f, "securityhub:GetResourcesV2"),
            SecurityhubActions::GetSecurityControlDefinition => {
                write!(f, "securityhub:GetSecurityControlDefinition")
            }
            SecurityhubActions::GetUsage => write!(f, "securityhub:GetUsage"),
            SecurityhubActions::InviteMembers => write!(f, "securityhub:InviteMembers"),
            SecurityhubActions::ListAggregatorsV2 => write!(f, "securityhub:ListAggregatorsV2"),
            SecurityhubActions::ListAutomationRules => write!(f, "securityhub:ListAutomationRules"),
            SecurityhubActions::ListAutomationRulesV2 => {
                write!(f, "securityhub:ListAutomationRulesV2")
            }
            SecurityhubActions::ListConfigurationPolicies => {
                write!(f, "securityhub:ListConfigurationPolicies")
            }
            SecurityhubActions::ListConfigurationPolicyAssociations => {
                write!(f, "securityhub:ListConfigurationPolicyAssociations")
            }
            SecurityhubActions::ListConnectorsV2 => write!(f, "securityhub:ListConnectorsV2"),
            SecurityhubActions::ListControlEvaluationSummaries => {
                write!(f, "securityhub:ListControlEvaluationSummaries")
            }
            SecurityhubActions::ListEnabledProductsForImport => {
                write!(f, "securityhub:ListEnabledProductsForImport")
            }
            SecurityhubActions::ListFindingAggregators => {
                write!(f, "securityhub:ListFindingAggregators")
            }
            SecurityhubActions::ListInvitations => write!(f, "securityhub:ListInvitations"),
            SecurityhubActions::ListMembers => write!(f, "securityhub:ListMembers"),
            SecurityhubActions::ListOrganizationAdminAccounts => {
                write!(f, "securityhub:ListOrganizationAdminAccounts")
            }
            SecurityhubActions::ListSecurityControlDefinitions => {
                write!(f, "securityhub:ListSecurityControlDefinitions")
            }
            SecurityhubActions::ListStandardsControlAssociations => {
                write!(f, "securityhub:ListStandardsControlAssociations")
            }
            SecurityhubActions::ListTagsForResource => write!(f, "securityhub:ListTagsForResource"),
            SecurityhubActions::SendFindingEvents => write!(f, "securityhub:SendFindingEvents"),
            SecurityhubActions::SendInsightEvents => write!(f, "securityhub:SendInsightEvents"),
            SecurityhubActions::StartConfigurationPolicyAssociation => {
                write!(f, "securityhub:StartConfigurationPolicyAssociation")
            }
            SecurityhubActions::StartConfigurationPolicyDisassociation => {
                write!(f, "securityhub:StartConfigurationPolicyDisassociation")
            }
            SecurityhubActions::TagResource => write!(f, "securityhub:TagResource"),
            SecurityhubActions::UntagResource => write!(f, "securityhub:UntagResource"),
            SecurityhubActions::UpdateActionTarget => write!(f, "securityhub:UpdateActionTarget"),
            SecurityhubActions::UpdateAggregatorV2 => write!(f, "securityhub:UpdateAggregatorV2"),
            SecurityhubActions::UpdateAutomationRuleV2 => {
                write!(f, "securityhub:UpdateAutomationRuleV2")
            }
            SecurityhubActions::UpdateConfigurationPolicy => {
                write!(f, "securityhub:UpdateConfigurationPolicy")
            }
            SecurityhubActions::UpdateConnectorV2 => write!(f, "securityhub:UpdateConnectorV2"),
            SecurityhubActions::UpdateFindingAggregator => {
                write!(f, "securityhub:UpdateFindingAggregator")
            }
            SecurityhubActions::UpdateFindings => write!(f, "securityhub:UpdateFindings"),
            SecurityhubActions::UpdateInsight => write!(f, "securityhub:UpdateInsight"),
            SecurityhubActions::UpdateOrganizationConfiguration => {
                write!(f, "securityhub:UpdateOrganizationConfiguration")
            }
            SecurityhubActions::UpdateSecurityControl => {
                write!(f, "securityhub:UpdateSecurityControl")
            }
            SecurityhubActions::UpdateSecurityHubConfiguration => {
                write!(f, "securityhub:UpdateSecurityHubConfiguration")
            }
            SecurityhubActions::UpdateStandardsControl => {
                write!(f, "securityhub:UpdateStandardsControl")
            }
        }
    }
}
