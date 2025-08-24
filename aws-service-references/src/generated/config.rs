// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum ConfigActions {
    AssociateResourceTypes,
    BatchGetAggregateResourceConfig,
    BatchGetResourceConfig,
    DeleteAggregationAuthorization,
    DeleteConfigRule,
    DeleteConfigurationAggregator,
    DeleteConfigurationRecorder,
    DeleteConformancePack,
    DeleteDeliveryChannel,
    DeleteEvaluationResults,
    DeleteOrganizationConfigRule,
    DeleteOrganizationConformancePack,
    DeletePendingAggregationRequest,
    DeleteRemediationConfiguration,
    DeleteRemediationExceptions,
    DeleteResourceConfig,
    DeleteRetentionConfiguration,
    DeleteServiceLinkedConfigurationRecorder,
    DeleteStoredQuery,
    DeliverConfigSnapshot,
    DescribeAggregateComplianceByConfigRules,
    DescribeAggregateComplianceByConformancePacks,
    DescribeAggregationAuthorizations,
    DescribeComplianceByConfigRule,
    DescribeComplianceByResource,
    DescribeConfigRuleEvaluationStatus,
    DescribeConfigRules,
    DescribeConfigurationAggregatorSourcesStatus,
    DescribeConfigurationAggregators,
    DescribeConfigurationRecorderStatus,
    DescribeConfigurationRecorders,
    DescribeConformancePackCompliance,
    DescribeConformancePackStatus,
    DescribeConformancePacks,
    DescribeDeliveryChannelStatus,
    DescribeDeliveryChannels,
    DescribeOrganizationConfigRuleStatuses,
    DescribeOrganizationConfigRules,
    DescribeOrganizationConformancePackStatuses,
    DescribeOrganizationConformancePacks,
    DescribePendingAggregationRequests,
    DescribeRemediationConfigurations,
    DescribeRemediationExceptions,
    DescribeRemediationExecutionStatus,
    DescribeRetentionConfigurations,
    DisassociateResourceTypes,
    GetAggregateComplianceDetailsByConfigRule,
    GetAggregateConfigRuleComplianceSummary,
    GetAggregateConformancePackComplianceSummary,
    GetAggregateDiscoveredResourceCounts,
    GetAggregateResourceConfig,
    GetComplianceDetailsByConfigRule,
    GetComplianceDetailsByResource,
    GetComplianceSummaryByConfigRule,
    GetComplianceSummaryByResourceType,
    GetConformancePackComplianceDetails,
    GetConformancePackComplianceSummary,
    GetCustomRulePolicy,
    GetDiscoveredResourceCounts,
    GetOrganizationConfigRuleDetailedStatus,
    GetOrganizationConformancePackDetailedStatus,
    GetOrganizationCustomRulePolicy,
    GetResourceConfigHistory,
    GetResourceEvaluationSummary,
    GetStoredQuery,
    ListAggregateDiscoveredResources,
    ListConfigurationRecorders,
    ListConformancePackComplianceScores,
    ListDiscoveredResources,
    ListResourceEvaluations,
    ListStoredQueries,
    ListTagsForResource,
    PutAggregationAuthorization,
    PutConfigRule,
    PutConfigurationAggregator,
    PutConfigurationRecorder,
    PutConformancePack,
    PutDeliveryChannel,
    PutEvaluations,
    PutExternalEvaluation,
    PutOrganizationConfigRule,
    PutOrganizationConformancePack,
    PutRemediationConfigurations,
    PutRemediationExceptions,
    PutResourceConfig,
    PutRetentionConfiguration,
    PutServiceLinkedConfigurationRecorder,
    PutStoredQuery,
    SelectAggregateResourceConfig,
    SelectResourceConfig,
    StartConfigRulesEvaluation,
    StartConfigurationRecorder,
    StartRemediationExecution,
    StartResourceEvaluation,
    StopConfigurationRecorder,
    TagResource,
    UntagResource,
}
impl std::fmt::Display for ConfigActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigActions::AssociateResourceTypes => write!(f, "config:AssociateResourceTypes"),
            ConfigActions::BatchGetAggregateResourceConfig => {
                write!(f, "config:BatchGetAggregateResourceConfig")
            }
            ConfigActions::BatchGetResourceConfig => write!(f, "config:BatchGetResourceConfig"),
            ConfigActions::DeleteAggregationAuthorization => {
                write!(f, "config:DeleteAggregationAuthorization")
            }
            ConfigActions::DeleteConfigRule => write!(f, "config:DeleteConfigRule"),
            ConfigActions::DeleteConfigurationAggregator => {
                write!(f, "config:DeleteConfigurationAggregator")
            }
            ConfigActions::DeleteConfigurationRecorder => {
                write!(f, "config:DeleteConfigurationRecorder")
            }
            ConfigActions::DeleteConformancePack => write!(f, "config:DeleteConformancePack"),
            ConfigActions::DeleteDeliveryChannel => write!(f, "config:DeleteDeliveryChannel"),
            ConfigActions::DeleteEvaluationResults => write!(f, "config:DeleteEvaluationResults"),
            ConfigActions::DeleteOrganizationConfigRule => {
                write!(f, "config:DeleteOrganizationConfigRule")
            }
            ConfigActions::DeleteOrganizationConformancePack => {
                write!(f, "config:DeleteOrganizationConformancePack")
            }
            ConfigActions::DeletePendingAggregationRequest => {
                write!(f, "config:DeletePendingAggregationRequest")
            }
            ConfigActions::DeleteRemediationConfiguration => {
                write!(f, "config:DeleteRemediationConfiguration")
            }
            ConfigActions::DeleteRemediationExceptions => {
                write!(f, "config:DeleteRemediationExceptions")
            }
            ConfigActions::DeleteResourceConfig => write!(f, "config:DeleteResourceConfig"),
            ConfigActions::DeleteRetentionConfiguration => {
                write!(f, "config:DeleteRetentionConfiguration")
            }
            ConfigActions::DeleteServiceLinkedConfigurationRecorder => {
                write!(f, "config:DeleteServiceLinkedConfigurationRecorder")
            }
            ConfigActions::DeleteStoredQuery => write!(f, "config:DeleteStoredQuery"),
            ConfigActions::DeliverConfigSnapshot => write!(f, "config:DeliverConfigSnapshot"),
            ConfigActions::DescribeAggregateComplianceByConfigRules => {
                write!(f, "config:DescribeAggregateComplianceByConfigRules")
            }
            ConfigActions::DescribeAggregateComplianceByConformancePacks => {
                write!(f, "config:DescribeAggregateComplianceByConformancePacks")
            }
            ConfigActions::DescribeAggregationAuthorizations => {
                write!(f, "config:DescribeAggregationAuthorizations")
            }
            ConfigActions::DescribeComplianceByConfigRule => {
                write!(f, "config:DescribeComplianceByConfigRule")
            }
            ConfigActions::DescribeComplianceByResource => {
                write!(f, "config:DescribeComplianceByResource")
            }
            ConfigActions::DescribeConfigRuleEvaluationStatus => {
                write!(f, "config:DescribeConfigRuleEvaluationStatus")
            }
            ConfigActions::DescribeConfigRules => write!(f, "config:DescribeConfigRules"),
            ConfigActions::DescribeConfigurationAggregatorSourcesStatus => {
                write!(f, "config:DescribeConfigurationAggregatorSourcesStatus")
            }
            ConfigActions::DescribeConfigurationAggregators => {
                write!(f, "config:DescribeConfigurationAggregators")
            }
            ConfigActions::DescribeConfigurationRecorderStatus => {
                write!(f, "config:DescribeConfigurationRecorderStatus")
            }
            ConfigActions::DescribeConfigurationRecorders => {
                write!(f, "config:DescribeConfigurationRecorders")
            }
            ConfigActions::DescribeConformancePackCompliance => {
                write!(f, "config:DescribeConformancePackCompliance")
            }
            ConfigActions::DescribeConformancePackStatus => {
                write!(f, "config:DescribeConformancePackStatus")
            }
            ConfigActions::DescribeConformancePacks => write!(f, "config:DescribeConformancePacks"),
            ConfigActions::DescribeDeliveryChannelStatus => {
                write!(f, "config:DescribeDeliveryChannelStatus")
            }
            ConfigActions::DescribeDeliveryChannels => write!(f, "config:DescribeDeliveryChannels"),
            ConfigActions::DescribeOrganizationConfigRuleStatuses => {
                write!(f, "config:DescribeOrganizationConfigRuleStatuses")
            }
            ConfigActions::DescribeOrganizationConfigRules => {
                write!(f, "config:DescribeOrganizationConfigRules")
            }
            ConfigActions::DescribeOrganizationConformancePackStatuses => {
                write!(f, "config:DescribeOrganizationConformancePackStatuses")
            }
            ConfigActions::DescribeOrganizationConformancePacks => {
                write!(f, "config:DescribeOrganizationConformancePacks")
            }
            ConfigActions::DescribePendingAggregationRequests => {
                write!(f, "config:DescribePendingAggregationRequests")
            }
            ConfigActions::DescribeRemediationConfigurations => {
                write!(f, "config:DescribeRemediationConfigurations")
            }
            ConfigActions::DescribeRemediationExceptions => {
                write!(f, "config:DescribeRemediationExceptions")
            }
            ConfigActions::DescribeRemediationExecutionStatus => {
                write!(f, "config:DescribeRemediationExecutionStatus")
            }
            ConfigActions::DescribeRetentionConfigurations => {
                write!(f, "config:DescribeRetentionConfigurations")
            }
            ConfigActions::DisassociateResourceTypes => {
                write!(f, "config:DisassociateResourceTypes")
            }
            ConfigActions::GetAggregateComplianceDetailsByConfigRule => {
                write!(f, "config:GetAggregateComplianceDetailsByConfigRule")
            }
            ConfigActions::GetAggregateConfigRuleComplianceSummary => {
                write!(f, "config:GetAggregateConfigRuleComplianceSummary")
            }
            ConfigActions::GetAggregateConformancePackComplianceSummary => {
                write!(f, "config:GetAggregateConformancePackComplianceSummary")
            }
            ConfigActions::GetAggregateDiscoveredResourceCounts => {
                write!(f, "config:GetAggregateDiscoveredResourceCounts")
            }
            ConfigActions::GetAggregateResourceConfig => {
                write!(f, "config:GetAggregateResourceConfig")
            }
            ConfigActions::GetComplianceDetailsByConfigRule => {
                write!(f, "config:GetComplianceDetailsByConfigRule")
            }
            ConfigActions::GetComplianceDetailsByResource => {
                write!(f, "config:GetComplianceDetailsByResource")
            }
            ConfigActions::GetComplianceSummaryByConfigRule => {
                write!(f, "config:GetComplianceSummaryByConfigRule")
            }
            ConfigActions::GetComplianceSummaryByResourceType => {
                write!(f, "config:GetComplianceSummaryByResourceType")
            }
            ConfigActions::GetConformancePackComplianceDetails => {
                write!(f, "config:GetConformancePackComplianceDetails")
            }
            ConfigActions::GetConformancePackComplianceSummary => {
                write!(f, "config:GetConformancePackComplianceSummary")
            }
            ConfigActions::GetCustomRulePolicy => write!(f, "config:GetCustomRulePolicy"),
            ConfigActions::GetDiscoveredResourceCounts => {
                write!(f, "config:GetDiscoveredResourceCounts")
            }
            ConfigActions::GetOrganizationConfigRuleDetailedStatus => {
                write!(f, "config:GetOrganizationConfigRuleDetailedStatus")
            }
            ConfigActions::GetOrganizationConformancePackDetailedStatus => {
                write!(f, "config:GetOrganizationConformancePackDetailedStatus")
            }
            ConfigActions::GetOrganizationCustomRulePolicy => {
                write!(f, "config:GetOrganizationCustomRulePolicy")
            }
            ConfigActions::GetResourceConfigHistory => write!(f, "config:GetResourceConfigHistory"),
            ConfigActions::GetResourceEvaluationSummary => {
                write!(f, "config:GetResourceEvaluationSummary")
            }
            ConfigActions::GetStoredQuery => write!(f, "config:GetStoredQuery"),
            ConfigActions::ListAggregateDiscoveredResources => {
                write!(f, "config:ListAggregateDiscoveredResources")
            }
            ConfigActions::ListConfigurationRecorders => {
                write!(f, "config:ListConfigurationRecorders")
            }
            ConfigActions::ListConformancePackComplianceScores => {
                write!(f, "config:ListConformancePackComplianceScores")
            }
            ConfigActions::ListDiscoveredResources => write!(f, "config:ListDiscoveredResources"),
            ConfigActions::ListResourceEvaluations => write!(f, "config:ListResourceEvaluations"),
            ConfigActions::ListStoredQueries => write!(f, "config:ListStoredQueries"),
            ConfigActions::ListTagsForResource => write!(f, "config:ListTagsForResource"),
            ConfigActions::PutAggregationAuthorization => {
                write!(f, "config:PutAggregationAuthorization")
            }
            ConfigActions::PutConfigRule => write!(f, "config:PutConfigRule"),
            ConfigActions::PutConfigurationAggregator => {
                write!(f, "config:PutConfigurationAggregator")
            }
            ConfigActions::PutConfigurationRecorder => write!(f, "config:PutConfigurationRecorder"),
            ConfigActions::PutConformancePack => write!(f, "config:PutConformancePack"),
            ConfigActions::PutDeliveryChannel => write!(f, "config:PutDeliveryChannel"),
            ConfigActions::PutEvaluations => write!(f, "config:PutEvaluations"),
            ConfigActions::PutExternalEvaluation => write!(f, "config:PutExternalEvaluation"),
            ConfigActions::PutOrganizationConfigRule => {
                write!(f, "config:PutOrganizationConfigRule")
            }
            ConfigActions::PutOrganizationConformancePack => {
                write!(f, "config:PutOrganizationConformancePack")
            }
            ConfigActions::PutRemediationConfigurations => {
                write!(f, "config:PutRemediationConfigurations")
            }
            ConfigActions::PutRemediationExceptions => write!(f, "config:PutRemediationExceptions"),
            ConfigActions::PutResourceConfig => write!(f, "config:PutResourceConfig"),
            ConfigActions::PutRetentionConfiguration => {
                write!(f, "config:PutRetentionConfiguration")
            }
            ConfigActions::PutServiceLinkedConfigurationRecorder => {
                write!(f, "config:PutServiceLinkedConfigurationRecorder")
            }
            ConfigActions::PutStoredQuery => write!(f, "config:PutStoredQuery"),
            ConfigActions::SelectAggregateResourceConfig => {
                write!(f, "config:SelectAggregateResourceConfig")
            }
            ConfigActions::SelectResourceConfig => write!(f, "config:SelectResourceConfig"),
            ConfigActions::StartConfigRulesEvaluation => {
                write!(f, "config:StartConfigRulesEvaluation")
            }
            ConfigActions::StartConfigurationRecorder => {
                write!(f, "config:StartConfigurationRecorder")
            }
            ConfigActions::StartRemediationExecution => {
                write!(f, "config:StartRemediationExecution")
            }
            ConfigActions::StartResourceEvaluation => write!(f, "config:StartResourceEvaluation"),
            ConfigActions::StopConfigurationRecorder => {
                write!(f, "config:StopConfigurationRecorder")
            }
            ConfigActions::TagResource => write!(f, "config:TagResource"),
            ConfigActions::UntagResource => write!(f, "config:UntagResource"),
        }
    }
}
