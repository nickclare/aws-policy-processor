// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum CognitoSyncActions {
    BulkPublish,
    DeleteDataset,
    DescribeDataset,
    DescribeIdentityPoolUsage,
    DescribeIdentityUsage,
    GetBulkPublishDetails,
    GetCognitoEvents,
    GetIdentityPoolConfiguration,
    ListDatasets,
    ListIdentityPoolUsage,
    ListRecords,
    QueryRecords,
    RegisterDevice,
    SetCognitoEvents,
    SetDatasetConfiguration,
    SetIdentityPoolConfiguration,
    SubscribeToDataset,
    UnsubscribeFromDataset,
    UpdateRecords,
}
impl std::fmt::Display for CognitoSyncActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CognitoSyncActions::BulkPublish => write!(f, "cognito-sync:BulkPublish"),
            CognitoSyncActions::DeleteDataset => write!(f, "cognito-sync:DeleteDataset"),
            CognitoSyncActions::DescribeDataset => write!(f, "cognito-sync:DescribeDataset"),
            CognitoSyncActions::DescribeIdentityPoolUsage => {
                write!(f, "cognito-sync:DescribeIdentityPoolUsage")
            }
            CognitoSyncActions::DescribeIdentityUsage => {
                write!(f, "cognito-sync:DescribeIdentityUsage")
            }
            CognitoSyncActions::GetBulkPublishDetails => {
                write!(f, "cognito-sync:GetBulkPublishDetails")
            }
            CognitoSyncActions::GetCognitoEvents => write!(f, "cognito-sync:GetCognitoEvents"),
            CognitoSyncActions::GetIdentityPoolConfiguration => {
                write!(f, "cognito-sync:GetIdentityPoolConfiguration")
            }
            CognitoSyncActions::ListDatasets => write!(f, "cognito-sync:ListDatasets"),
            CognitoSyncActions::ListIdentityPoolUsage => {
                write!(f, "cognito-sync:ListIdentityPoolUsage")
            }
            CognitoSyncActions::ListRecords => write!(f, "cognito-sync:ListRecords"),
            CognitoSyncActions::QueryRecords => write!(f, "cognito-sync:QueryRecords"),
            CognitoSyncActions::RegisterDevice => write!(f, "cognito-sync:RegisterDevice"),
            CognitoSyncActions::SetCognitoEvents => write!(f, "cognito-sync:SetCognitoEvents"),
            CognitoSyncActions::SetDatasetConfiguration => {
                write!(f, "cognito-sync:SetDatasetConfiguration")
            }
            CognitoSyncActions::SetIdentityPoolConfiguration => {
                write!(f, "cognito-sync:SetIdentityPoolConfiguration")
            }
            CognitoSyncActions::SubscribeToDataset => write!(f, "cognito-sync:SubscribeToDataset"),
            CognitoSyncActions::UnsubscribeFromDataset => {
                write!(f, "cognito-sync:UnsubscribeFromDataset")
            }
            CognitoSyncActions::UpdateRecords => write!(f, "cognito-sync:UpdateRecords"),
        }
    }
}
