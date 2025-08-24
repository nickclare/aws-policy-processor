// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum AppflowActions {
    CancelFlowExecutions,
    CreateConnectorProfile,
    CreateFlow,
    DeleteConnectorProfile,
    DeleteFlow,
    DescribeConnector,
    DescribeConnectorEntity,
    DescribeConnectorFields,
    DescribeConnectorProfiles,
    DescribeConnectors,
    DescribeFlow,
    DescribeFlowExecution,
    DescribeFlowExecutionRecords,
    DescribeFlows,
    ListConnectorEntities,
    ListConnectorFields,
    ListConnectors,
    ListFlows,
    ListTagsForResource,
    RegisterConnector,
    ResetConnectorMetadataCache,
    RunFlow,
    StartFlow,
    StopFlow,
    TagResource,
    UnRegisterConnector,
    UntagResource,
    UpdateConnectorProfile,
    UpdateConnectorRegistration,
    UpdateFlow,
    UseConnectorProfile,
}
impl std::fmt::Display for AppflowActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppflowActions::CancelFlowExecutions => write!(f, "appflow:CancelFlowExecutions"),
            AppflowActions::CreateConnectorProfile => write!(f, "appflow:CreateConnectorProfile"),
            AppflowActions::CreateFlow => write!(f, "appflow:CreateFlow"),
            AppflowActions::DeleteConnectorProfile => write!(f, "appflow:DeleteConnectorProfile"),
            AppflowActions::DeleteFlow => write!(f, "appflow:DeleteFlow"),
            AppflowActions::DescribeConnector => write!(f, "appflow:DescribeConnector"),
            AppflowActions::DescribeConnectorEntity => write!(f, "appflow:DescribeConnectorEntity"),
            AppflowActions::DescribeConnectorFields => write!(f, "appflow:DescribeConnectorFields"),
            AppflowActions::DescribeConnectorProfiles => {
                write!(f, "appflow:DescribeConnectorProfiles")
            }
            AppflowActions::DescribeConnectors => write!(f, "appflow:DescribeConnectors"),
            AppflowActions::DescribeFlow => write!(f, "appflow:DescribeFlow"),
            AppflowActions::DescribeFlowExecution => write!(f, "appflow:DescribeFlowExecution"),
            AppflowActions::DescribeFlowExecutionRecords => {
                write!(f, "appflow:DescribeFlowExecutionRecords")
            }
            AppflowActions::DescribeFlows => write!(f, "appflow:DescribeFlows"),
            AppflowActions::ListConnectorEntities => write!(f, "appflow:ListConnectorEntities"),
            AppflowActions::ListConnectorFields => write!(f, "appflow:ListConnectorFields"),
            AppflowActions::ListConnectors => write!(f, "appflow:ListConnectors"),
            AppflowActions::ListFlows => write!(f, "appflow:ListFlows"),
            AppflowActions::ListTagsForResource => write!(f, "appflow:ListTagsForResource"),
            AppflowActions::RegisterConnector => write!(f, "appflow:RegisterConnector"),
            AppflowActions::ResetConnectorMetadataCache => {
                write!(f, "appflow:ResetConnectorMetadataCache")
            }
            AppflowActions::RunFlow => write!(f, "appflow:RunFlow"),
            AppflowActions::StartFlow => write!(f, "appflow:StartFlow"),
            AppflowActions::StopFlow => write!(f, "appflow:StopFlow"),
            AppflowActions::TagResource => write!(f, "appflow:TagResource"),
            AppflowActions::UnRegisterConnector => write!(f, "appflow:UnRegisterConnector"),
            AppflowActions::UntagResource => write!(f, "appflow:UntagResource"),
            AppflowActions::UpdateConnectorProfile => write!(f, "appflow:UpdateConnectorProfile"),
            AppflowActions::UpdateConnectorRegistration => {
                write!(f, "appflow:UpdateConnectorRegistration")
            }
            AppflowActions::UpdateFlow => write!(f, "appflow:UpdateFlow"),
            AppflowActions::UseConnectorProfile => write!(f, "appflow:UseConnectorProfile"),
        }
    }
}
