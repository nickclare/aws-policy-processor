// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum DataexchangeActions {
    AcceptDataGrant,
    CancelJob,
    CreateAsset,
    CreateDataGrant,
    CreateDataSet,
    CreateEventAction,
    CreateJob,
    CreateRevision,
    DeleteAsset,
    DeleteDataGrant,
    DeleteDataSet,
    DeleteEventAction,
    DeleteRevision,
    GetAsset,
    GetDataGrant,
    GetDataSet,
    GetEventAction,
    GetJob,
    GetReceivedDataGrant,
    GetRevision,
    ListDataGrants,
    ListDataSetRevisions,
    ListDataSets,
    ListEventActions,
    ListJobs,
    ListReceivedDataGrants,
    ListRevisionAssets,
    ListTagsForResource,
    PublishDataSet,
    PublishToDataGrant,
    RevokeRevision,
    SendApiAsset,
    SendDataSetNotification,
    StartJob,
    TagResource,
    UntagResource,
    UpdateAsset,
    UpdateDataSet,
    UpdateEventAction,
    UpdateRevision,
}
impl std::fmt::Display for DataexchangeActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataexchangeActions::AcceptDataGrant => write!(f, "dataexchange:AcceptDataGrant"),
            DataexchangeActions::CancelJob => write!(f, "dataexchange:CancelJob"),
            DataexchangeActions::CreateAsset => write!(f, "dataexchange:CreateAsset"),
            DataexchangeActions::CreateDataGrant => write!(f, "dataexchange:CreateDataGrant"),
            DataexchangeActions::CreateDataSet => write!(f, "dataexchange:CreateDataSet"),
            DataexchangeActions::CreateEventAction => write!(f, "dataexchange:CreateEventAction"),
            DataexchangeActions::CreateJob => write!(f, "dataexchange:CreateJob"),
            DataexchangeActions::CreateRevision => write!(f, "dataexchange:CreateRevision"),
            DataexchangeActions::DeleteAsset => write!(f, "dataexchange:DeleteAsset"),
            DataexchangeActions::DeleteDataGrant => write!(f, "dataexchange:DeleteDataGrant"),
            DataexchangeActions::DeleteDataSet => write!(f, "dataexchange:DeleteDataSet"),
            DataexchangeActions::DeleteEventAction => write!(f, "dataexchange:DeleteEventAction"),
            DataexchangeActions::DeleteRevision => write!(f, "dataexchange:DeleteRevision"),
            DataexchangeActions::GetAsset => write!(f, "dataexchange:GetAsset"),
            DataexchangeActions::GetDataGrant => write!(f, "dataexchange:GetDataGrant"),
            DataexchangeActions::GetDataSet => write!(f, "dataexchange:GetDataSet"),
            DataexchangeActions::GetEventAction => write!(f, "dataexchange:GetEventAction"),
            DataexchangeActions::GetJob => write!(f, "dataexchange:GetJob"),
            DataexchangeActions::GetReceivedDataGrant => {
                write!(f, "dataexchange:GetReceivedDataGrant")
            }
            DataexchangeActions::GetRevision => write!(f, "dataexchange:GetRevision"),
            DataexchangeActions::ListDataGrants => write!(f, "dataexchange:ListDataGrants"),
            DataexchangeActions::ListDataSetRevisions => {
                write!(f, "dataexchange:ListDataSetRevisions")
            }
            DataexchangeActions::ListDataSets => write!(f, "dataexchange:ListDataSets"),
            DataexchangeActions::ListEventActions => write!(f, "dataexchange:ListEventActions"),
            DataexchangeActions::ListJobs => write!(f, "dataexchange:ListJobs"),
            DataexchangeActions::ListReceivedDataGrants => {
                write!(f, "dataexchange:ListReceivedDataGrants")
            }
            DataexchangeActions::ListRevisionAssets => write!(f, "dataexchange:ListRevisionAssets"),
            DataexchangeActions::ListTagsForResource => {
                write!(f, "dataexchange:ListTagsForResource")
            }
            DataexchangeActions::PublishDataSet => write!(f, "dataexchange:PublishDataSet"),
            DataexchangeActions::PublishToDataGrant => write!(f, "dataexchange:PublishToDataGrant"),
            DataexchangeActions::RevokeRevision => write!(f, "dataexchange:RevokeRevision"),
            DataexchangeActions::SendApiAsset => write!(f, "dataexchange:SendApiAsset"),
            DataexchangeActions::SendDataSetNotification => {
                write!(f, "dataexchange:SendDataSetNotification")
            }
            DataexchangeActions::StartJob => write!(f, "dataexchange:StartJob"),
            DataexchangeActions::TagResource => write!(f, "dataexchange:TagResource"),
            DataexchangeActions::UntagResource => write!(f, "dataexchange:UntagResource"),
            DataexchangeActions::UpdateAsset => write!(f, "dataexchange:UpdateAsset"),
            DataexchangeActions::UpdateDataSet => write!(f, "dataexchange:UpdateDataSet"),
            DataexchangeActions::UpdateEventAction => write!(f, "dataexchange:UpdateEventAction"),
            DataexchangeActions::UpdateRevision => write!(f, "dataexchange:UpdateRevision"),
        }
    }
}
