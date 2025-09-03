// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum AmplifybackendActions {
    CloneBackend,
    CreateBackend,
    CreateBackendApi,
    CreateBackendAuth,
    CreateBackendConfig,
    CreateBackendStorage,
    CreateToken,
    DeleteBackend,
    DeleteBackendApi,
    DeleteBackendAuth,
    DeleteBackendStorage,
    DeleteToken,
    GenerateBackendApiModels,
    GetBackend,
    GetBackendApi,
    GetBackendApiModels,
    GetBackendAuth,
    GetBackendJob,
    GetBackendStorage,
    GetToken,
    ImportBackendAuth,
    ImportBackendStorage,
    ListBackendJobs,
    ListS3Buckets,
    RemoveAllBackends,
    RemoveBackendConfig,
    UpdateBackendApi,
    UpdateBackendAuth,
    UpdateBackendConfig,
    UpdateBackendJob,
    UpdateBackendStorage,
}
impl std::fmt::Display for AmplifybackendActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AmplifybackendActions::CloneBackend => write!(f, "amplifybackend:CloneBackend"),
            AmplifybackendActions::CreateBackend => write!(f, "amplifybackend:CreateBackend"),
            AmplifybackendActions::CreateBackendApi => write!(f, "amplifybackend:CreateBackendAPI"),
            AmplifybackendActions::CreateBackendAuth => {
                write!(f, "amplifybackend:CreateBackendAuth")
            }
            AmplifybackendActions::CreateBackendConfig => {
                write!(f, "amplifybackend:CreateBackendConfig")
            }
            AmplifybackendActions::CreateBackendStorage => {
                write!(f, "amplifybackend:CreateBackendStorage")
            }
            AmplifybackendActions::CreateToken => write!(f, "amplifybackend:CreateToken"),
            AmplifybackendActions::DeleteBackend => write!(f, "amplifybackend:DeleteBackend"),
            AmplifybackendActions::DeleteBackendApi => write!(f, "amplifybackend:DeleteBackendAPI"),
            AmplifybackendActions::DeleteBackendAuth => {
                write!(f, "amplifybackend:DeleteBackendAuth")
            }
            AmplifybackendActions::DeleteBackendStorage => {
                write!(f, "amplifybackend:DeleteBackendStorage")
            }
            AmplifybackendActions::DeleteToken => write!(f, "amplifybackend:DeleteToken"),
            AmplifybackendActions::GenerateBackendApiModels => {
                write!(f, "amplifybackend:GenerateBackendAPIModels")
            }
            AmplifybackendActions::GetBackend => write!(f, "amplifybackend:GetBackend"),
            AmplifybackendActions::GetBackendApi => write!(f, "amplifybackend:GetBackendAPI"),
            AmplifybackendActions::GetBackendApiModels => {
                write!(f, "amplifybackend:GetBackendAPIModels")
            }
            AmplifybackendActions::GetBackendAuth => write!(f, "amplifybackend:GetBackendAuth"),
            AmplifybackendActions::GetBackendJob => write!(f, "amplifybackend:GetBackendJob"),
            AmplifybackendActions::GetBackendStorage => {
                write!(f, "amplifybackend:GetBackendStorage")
            }
            AmplifybackendActions::GetToken => write!(f, "amplifybackend:GetToken"),
            AmplifybackendActions::ImportBackendAuth => {
                write!(f, "amplifybackend:ImportBackendAuth")
            }
            AmplifybackendActions::ImportBackendStorage => {
                write!(f, "amplifybackend:ImportBackendStorage")
            }
            AmplifybackendActions::ListBackendJobs => write!(f, "amplifybackend:ListBackendJobs"),
            AmplifybackendActions::ListS3Buckets => write!(f, "amplifybackend:ListS3Buckets"),
            AmplifybackendActions::RemoveAllBackends => {
                write!(f, "amplifybackend:RemoveAllBackends")
            }
            AmplifybackendActions::RemoveBackendConfig => {
                write!(f, "amplifybackend:RemoveBackendConfig")
            }
            AmplifybackendActions::UpdateBackendApi => write!(f, "amplifybackend:UpdateBackendAPI"),
            AmplifybackendActions::UpdateBackendAuth => {
                write!(f, "amplifybackend:UpdateBackendAuth")
            }
            AmplifybackendActions::UpdateBackendConfig => {
                write!(f, "amplifybackend:UpdateBackendConfig")
            }
            AmplifybackendActions::UpdateBackendJob => write!(f, "amplifybackend:UpdateBackendJob"),
            AmplifybackendActions::UpdateBackendStorage => {
                write!(f, "amplifybackend:UpdateBackendStorage")
            }
        }
    }
}
