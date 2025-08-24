// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum AppfabricActions {
    BatchGetUserAccessTasks,
    ConnectAppAuthorization,
    CreateAppAuthorization,
    CreateAppBundle,
    CreateIngestion,
    CreateIngestionDestination,
    DeleteAppAuthorization,
    DeleteAppBundle,
    DeleteIngestion,
    DeleteIngestionDestination,
    GetAppAuthorization,
    GetAppBundle,
    GetIngestion,
    GetIngestionDestination,
    ListAppAuthorizations,
    ListAppBundles,
    ListIngestionDestinations,
    ListIngestions,
    ListTagsForResource,
    StartIngestion,
    StartUserAccessTasks,
    StopIngestion,
    TagResource,
    UntagResource,
    UpdateAppAuthorization,
    UpdateIngestionDestination,
}
impl std::fmt::Display for AppfabricActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppfabricActions::BatchGetUserAccessTasks => {
                write!(f, "appfabric:BatchGetUserAccessTasks")
            }
            AppfabricActions::ConnectAppAuthorization => {
                write!(f, "appfabric:ConnectAppAuthorization")
            }
            AppfabricActions::CreateAppAuthorization => {
                write!(f, "appfabric:CreateAppAuthorization")
            }
            AppfabricActions::CreateAppBundle => write!(f, "appfabric:CreateAppBundle"),
            AppfabricActions::CreateIngestion => write!(f, "appfabric:CreateIngestion"),
            AppfabricActions::CreateIngestionDestination => {
                write!(f, "appfabric:CreateIngestionDestination")
            }
            AppfabricActions::DeleteAppAuthorization => {
                write!(f, "appfabric:DeleteAppAuthorization")
            }
            AppfabricActions::DeleteAppBundle => write!(f, "appfabric:DeleteAppBundle"),
            AppfabricActions::DeleteIngestion => write!(f, "appfabric:DeleteIngestion"),
            AppfabricActions::DeleteIngestionDestination => {
                write!(f, "appfabric:DeleteIngestionDestination")
            }
            AppfabricActions::GetAppAuthorization => write!(f, "appfabric:GetAppAuthorization"),
            AppfabricActions::GetAppBundle => write!(f, "appfabric:GetAppBundle"),
            AppfabricActions::GetIngestion => write!(f, "appfabric:GetIngestion"),
            AppfabricActions::GetIngestionDestination => {
                write!(f, "appfabric:GetIngestionDestination")
            }
            AppfabricActions::ListAppAuthorizations => write!(f, "appfabric:ListAppAuthorizations"),
            AppfabricActions::ListAppBundles => write!(f, "appfabric:ListAppBundles"),
            AppfabricActions::ListIngestionDestinations => {
                write!(f, "appfabric:ListIngestionDestinations")
            }
            AppfabricActions::ListIngestions => write!(f, "appfabric:ListIngestions"),
            AppfabricActions::ListTagsForResource => write!(f, "appfabric:ListTagsForResource"),
            AppfabricActions::StartIngestion => write!(f, "appfabric:StartIngestion"),
            AppfabricActions::StartUserAccessTasks => write!(f, "appfabric:StartUserAccessTasks"),
            AppfabricActions::StopIngestion => write!(f, "appfabric:StopIngestion"),
            AppfabricActions::TagResource => write!(f, "appfabric:TagResource"),
            AppfabricActions::UntagResource => write!(f, "appfabric:UntagResource"),
            AppfabricActions::UpdateAppAuthorization => {
                write!(f, "appfabric:UpdateAppAuthorization")
            }
            AppfabricActions::UpdateIngestionDestination => {
                write!(f, "appfabric:UpdateIngestionDestination")
            }
        }
    }
}
