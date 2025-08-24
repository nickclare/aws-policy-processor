// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum ApsActions {
    CreateAlertManagerAlerts,
    CreateAlertManagerDefinition,
    CreateLoggingConfiguration,
    CreateQueryLoggingConfiguration,
    CreateRuleGroupsNamespace,
    CreateScraper,
    CreateWorkspace,
    DeleteAlertManagerDefinition,
    DeleteAlertManagerSilence,
    DeleteLoggingConfiguration,
    DeleteQueryLoggingConfiguration,
    DeleteResourcePolicy,
    DeleteRuleGroupsNamespace,
    DeleteScraper,
    DeleteWorkspace,
    DescribeAlertManagerDefinition,
    DescribeLoggingConfiguration,
    DescribeQueryLoggingConfiguration,
    DescribeResourcePolicy,
    DescribeRuleGroupsNamespace,
    DescribeScraper,
    DescribeWorkspace,
    DescribeWorkspaceConfiguration,
    GetAlertManagerSilence,
    GetAlertManagerStatus,
    GetDefaultScraperConfiguration,
    GetLabels,
    GetMetricMetadata,
    GetSeries,
    ListAlertManagerAlertGroups,
    ListAlertManagerAlerts,
    ListAlertManagerReceivers,
    ListAlertManagerSilences,
    ListAlerts,
    ListRuleGroupsNamespaces,
    ListRules,
    ListScrapers,
    ListTagsForResource,
    ListWorkspaces,
    PutAlertManagerDefinition,
    PutAlertManagerSilences,
    PutResourcePolicy,
    PutRuleGroupsNamespace,
    QueryMetrics,
    RemoteWrite,
    TagResource,
    UntagResource,
    UpdateLoggingConfiguration,
    UpdateQueryLoggingConfiguration,
    UpdateScraper,
    UpdateWorkspaceAlias,
    UpdateWorkspaceConfiguration,
}
impl std::fmt::Display for ApsActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApsActions::CreateAlertManagerAlerts => write!(f, "aps:CreateAlertManagerAlerts"),
            ApsActions::CreateAlertManagerDefinition => {
                write!(f, "aps:CreateAlertManagerDefinition")
            }
            ApsActions::CreateLoggingConfiguration => write!(f, "aps:CreateLoggingConfiguration"),
            ApsActions::CreateQueryLoggingConfiguration => {
                write!(f, "aps:CreateQueryLoggingConfiguration")
            }
            ApsActions::CreateRuleGroupsNamespace => write!(f, "aps:CreateRuleGroupsNamespace"),
            ApsActions::CreateScraper => write!(f, "aps:CreateScraper"),
            ApsActions::CreateWorkspace => write!(f, "aps:CreateWorkspace"),
            ApsActions::DeleteAlertManagerDefinition => {
                write!(f, "aps:DeleteAlertManagerDefinition")
            }
            ApsActions::DeleteAlertManagerSilence => write!(f, "aps:DeleteAlertManagerSilence"),
            ApsActions::DeleteLoggingConfiguration => write!(f, "aps:DeleteLoggingConfiguration"),
            ApsActions::DeleteQueryLoggingConfiguration => {
                write!(f, "aps:DeleteQueryLoggingConfiguration")
            }
            ApsActions::DeleteResourcePolicy => write!(f, "aps:DeleteResourcePolicy"),
            ApsActions::DeleteRuleGroupsNamespace => write!(f, "aps:DeleteRuleGroupsNamespace"),
            ApsActions::DeleteScraper => write!(f, "aps:DeleteScraper"),
            ApsActions::DeleteWorkspace => write!(f, "aps:DeleteWorkspace"),
            ApsActions::DescribeAlertManagerDefinition => {
                write!(f, "aps:DescribeAlertManagerDefinition")
            }
            ApsActions::DescribeLoggingConfiguration => {
                write!(f, "aps:DescribeLoggingConfiguration")
            }
            ApsActions::DescribeQueryLoggingConfiguration => {
                write!(f, "aps:DescribeQueryLoggingConfiguration")
            }
            ApsActions::DescribeResourcePolicy => write!(f, "aps:DescribeResourcePolicy"),
            ApsActions::DescribeRuleGroupsNamespace => write!(f, "aps:DescribeRuleGroupsNamespace"),
            ApsActions::DescribeScraper => write!(f, "aps:DescribeScraper"),
            ApsActions::DescribeWorkspace => write!(f, "aps:DescribeWorkspace"),
            ApsActions::DescribeWorkspaceConfiguration => {
                write!(f, "aps:DescribeWorkspaceConfiguration")
            }
            ApsActions::GetAlertManagerSilence => write!(f, "aps:GetAlertManagerSilence"),
            ApsActions::GetAlertManagerStatus => write!(f, "aps:GetAlertManagerStatus"),
            ApsActions::GetDefaultScraperConfiguration => {
                write!(f, "aps:GetDefaultScraperConfiguration")
            }
            ApsActions::GetLabels => write!(f, "aps:GetLabels"),
            ApsActions::GetMetricMetadata => write!(f, "aps:GetMetricMetadata"),
            ApsActions::GetSeries => write!(f, "aps:GetSeries"),
            ApsActions::ListAlertManagerAlertGroups => write!(f, "aps:ListAlertManagerAlertGroups"),
            ApsActions::ListAlertManagerAlerts => write!(f, "aps:ListAlertManagerAlerts"),
            ApsActions::ListAlertManagerReceivers => write!(f, "aps:ListAlertManagerReceivers"),
            ApsActions::ListAlertManagerSilences => write!(f, "aps:ListAlertManagerSilences"),
            ApsActions::ListAlerts => write!(f, "aps:ListAlerts"),
            ApsActions::ListRuleGroupsNamespaces => write!(f, "aps:ListRuleGroupsNamespaces"),
            ApsActions::ListRules => write!(f, "aps:ListRules"),
            ApsActions::ListScrapers => write!(f, "aps:ListScrapers"),
            ApsActions::ListTagsForResource => write!(f, "aps:ListTagsForResource"),
            ApsActions::ListWorkspaces => write!(f, "aps:ListWorkspaces"),
            ApsActions::PutAlertManagerDefinition => write!(f, "aps:PutAlertManagerDefinition"),
            ApsActions::PutAlertManagerSilences => write!(f, "aps:PutAlertManagerSilences"),
            ApsActions::PutResourcePolicy => write!(f, "aps:PutResourcePolicy"),
            ApsActions::PutRuleGroupsNamespace => write!(f, "aps:PutRuleGroupsNamespace"),
            ApsActions::QueryMetrics => write!(f, "aps:QueryMetrics"),
            ApsActions::RemoteWrite => write!(f, "aps:RemoteWrite"),
            ApsActions::TagResource => write!(f, "aps:TagResource"),
            ApsActions::UntagResource => write!(f, "aps:UntagResource"),
            ApsActions::UpdateLoggingConfiguration => write!(f, "aps:UpdateLoggingConfiguration"),
            ApsActions::UpdateQueryLoggingConfiguration => {
                write!(f, "aps:UpdateQueryLoggingConfiguration")
            }
            ApsActions::UpdateScraper => write!(f, "aps:UpdateScraper"),
            ApsActions::UpdateWorkspaceAlias => write!(f, "aps:UpdateWorkspaceAlias"),
            ApsActions::UpdateWorkspaceConfiguration => {
                write!(f, "aps:UpdateWorkspaceConfiguration")
            }
        }
    }
}
