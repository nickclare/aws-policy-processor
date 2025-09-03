// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum ScnActions {
    AssignAdminPermissionsToUser,
    CreateBillOfMaterialsImportJob,
    CreateDataIntegrationFlow,
    CreateDataLakeDataset,
    CreateDataLakeNamespace,
    CreateInstance,
    CreateSsoApplication,
    DeleteDataIntegrationFlow,
    DeleteDataLakeDataset,
    DeleteDataLakeNamespace,
    DeleteInstance,
    DeleteSsoApplication,
    DescribeInstance,
    GetBillOfMaterialsImportJob,
    GetDataIntegrationEvent,
    GetDataIntegrationFlow,
    GetDataIntegrationFlowExecution,
    GetDataLakeDataset,
    GetDataLakeNamespace,
    GetInstance,
    ListAdminUsers,
    ListDataIntegrationEvents,
    ListDataIntegrationFlowExecutions,
    ListDataIntegrationFlows,
    ListDataLakeDatasets,
    ListDataLakeNamespaces,
    ListInstances,
    ListTagsForResource,
    RemoveAdminPermissionsForUser,
    SendDataIntegrationEvent,
    TagResource,
    UntagResource,
    UpdateDataIntegrationFlow,
    UpdateDataLakeDataset,
    UpdateDataLakeNamespace,
    UpdateInstance,
}
impl std::fmt::Display for ScnActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScnActions::AssignAdminPermissionsToUser => {
                write!(f, "scn:AssignAdminPermissionsToUser")
            }
            ScnActions::CreateBillOfMaterialsImportJob => {
                write!(f, "scn:CreateBillOfMaterialsImportJob")
            }
            ScnActions::CreateDataIntegrationFlow => write!(f, "scn:CreateDataIntegrationFlow"),
            ScnActions::CreateDataLakeDataset => write!(f, "scn:CreateDataLakeDataset"),
            ScnActions::CreateDataLakeNamespace => write!(f, "scn:CreateDataLakeNamespace"),
            ScnActions::CreateInstance => write!(f, "scn:CreateInstance"),
            ScnActions::CreateSsoApplication => write!(f, "scn:CreateSSOApplication"),
            ScnActions::DeleteDataIntegrationFlow => write!(f, "scn:DeleteDataIntegrationFlow"),
            ScnActions::DeleteDataLakeDataset => write!(f, "scn:DeleteDataLakeDataset"),
            ScnActions::DeleteDataLakeNamespace => write!(f, "scn:DeleteDataLakeNamespace"),
            ScnActions::DeleteInstance => write!(f, "scn:DeleteInstance"),
            ScnActions::DeleteSsoApplication => write!(f, "scn:DeleteSSOApplication"),
            ScnActions::DescribeInstance => write!(f, "scn:DescribeInstance"),
            ScnActions::GetBillOfMaterialsImportJob => write!(f, "scn:GetBillOfMaterialsImportJob"),
            ScnActions::GetDataIntegrationEvent => write!(f, "scn:GetDataIntegrationEvent"),
            ScnActions::GetDataIntegrationFlow => write!(f, "scn:GetDataIntegrationFlow"),
            ScnActions::GetDataIntegrationFlowExecution => {
                write!(f, "scn:GetDataIntegrationFlowExecution")
            }
            ScnActions::GetDataLakeDataset => write!(f, "scn:GetDataLakeDataset"),
            ScnActions::GetDataLakeNamespace => write!(f, "scn:GetDataLakeNamespace"),
            ScnActions::GetInstance => write!(f, "scn:GetInstance"),
            ScnActions::ListAdminUsers => write!(f, "scn:ListAdminUsers"),
            ScnActions::ListDataIntegrationEvents => write!(f, "scn:ListDataIntegrationEvents"),
            ScnActions::ListDataIntegrationFlowExecutions => {
                write!(f, "scn:ListDataIntegrationFlowExecutions")
            }
            ScnActions::ListDataIntegrationFlows => write!(f, "scn:ListDataIntegrationFlows"),
            ScnActions::ListDataLakeDatasets => write!(f, "scn:ListDataLakeDatasets"),
            ScnActions::ListDataLakeNamespaces => write!(f, "scn:ListDataLakeNamespaces"),
            ScnActions::ListInstances => write!(f, "scn:ListInstances"),
            ScnActions::ListTagsForResource => write!(f, "scn:ListTagsForResource"),
            ScnActions::RemoveAdminPermissionsForUser => {
                write!(f, "scn:RemoveAdminPermissionsForUser")
            }
            ScnActions::SendDataIntegrationEvent => write!(f, "scn:SendDataIntegrationEvent"),
            ScnActions::TagResource => write!(f, "scn:TagResource"),
            ScnActions::UntagResource => write!(f, "scn:UntagResource"),
            ScnActions::UpdateDataIntegrationFlow => write!(f, "scn:UpdateDataIntegrationFlow"),
            ScnActions::UpdateDataLakeDataset => write!(f, "scn:UpdateDataLakeDataset"),
            ScnActions::UpdateDataLakeNamespace => write!(f, "scn:UpdateDataLakeNamespace"),
            ScnActions::UpdateInstance => write!(f, "scn:UpdateInstance"),
        }
    }
}
