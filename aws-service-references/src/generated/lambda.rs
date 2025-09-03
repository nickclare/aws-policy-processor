// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum LambdaActions {
    AddLayerVersionPermission,
    AddPermission,
    CreateAlias,
    CreateCodeSigningConfig,
    CreateEventSourceMapping,
    CreateFunction,
    CreateFunctionUrlConfig,
    DeleteAlias,
    DeleteCodeSigningConfig,
    DeleteEventSourceMapping,
    DeleteFunction,
    DeleteFunctionCodeSigningConfig,
    DeleteFunctionConcurrency,
    DeleteFunctionEventInvokeConfig,
    DeleteFunctionUrlConfig,
    DeleteLayerVersion,
    DeleteProvisionedConcurrencyConfig,
    DisableReplication,
    EnableReplication,
    GetAccountSettings,
    GetAlias,
    GetCodeSigningConfig,
    GetEventSourceMapping,
    GetFunction,
    GetFunctionCodeSigningConfig,
    GetFunctionConcurrency,
    GetFunctionConfiguration,
    GetFunctionEventInvokeConfig,
    GetFunctionRecursionConfig,
    GetFunctionUrlConfig,
    GetLayerVersion,
    GetLayerVersionPolicy,
    GetPolicy,
    GetProvisionedConcurrencyConfig,
    GetRuntimeManagementConfig,
    InvokeAsync,
    InvokeFunction,
    InvokeFunctionUrl,
    ListAliases,
    ListCodeSigningConfigs,
    ListEventSourceMappings,
    ListFunctionEventInvokeConfigs,
    ListFunctionUrlConfigs,
    ListFunctions,
    ListFunctionsByCodeSigningConfig,
    ListLayerVersions,
    ListLayers,
    ListProvisionedConcurrencyConfigs,
    ListTags,
    ListVersionsByFunction,
    PublishLayerVersion,
    PublishVersion,
    PutFunctionCodeSigningConfig,
    PutFunctionConcurrency,
    PutFunctionEventInvokeConfig,
    PutFunctionRecursionConfig,
    PutProvisionedConcurrencyConfig,
    PutRuntimeManagementConfig,
    RemoveLayerVersionPermission,
    RemovePermission,
    TagResource,
    UntagResource,
    UpdateAlias,
    UpdateCodeSigningConfig,
    UpdateEventSourceMapping,
    UpdateFunctionCode,
    UpdateFunctionCodeSigningConfig,
    UpdateFunctionConfiguration,
    UpdateFunctionEventInvokeConfig,
    UpdateFunctionUrlConfig,
}
impl std::fmt::Display for LambdaActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LambdaActions::AddLayerVersionPermission => {
                write!(f, "lambda:AddLayerVersionPermission")
            }
            LambdaActions::AddPermission => write!(f, "lambda:AddPermission"),
            LambdaActions::CreateAlias => write!(f, "lambda:CreateAlias"),
            LambdaActions::CreateCodeSigningConfig => write!(f, "lambda:CreateCodeSigningConfig"),
            LambdaActions::CreateEventSourceMapping => write!(f, "lambda:CreateEventSourceMapping"),
            LambdaActions::CreateFunction => write!(f, "lambda:CreateFunction"),
            LambdaActions::CreateFunctionUrlConfig => write!(f, "lambda:CreateFunctionUrlConfig"),
            LambdaActions::DeleteAlias => write!(f, "lambda:DeleteAlias"),
            LambdaActions::DeleteCodeSigningConfig => write!(f, "lambda:DeleteCodeSigningConfig"),
            LambdaActions::DeleteEventSourceMapping => write!(f, "lambda:DeleteEventSourceMapping"),
            LambdaActions::DeleteFunction => write!(f, "lambda:DeleteFunction"),
            LambdaActions::DeleteFunctionCodeSigningConfig => {
                write!(f, "lambda:DeleteFunctionCodeSigningConfig")
            }
            LambdaActions::DeleteFunctionConcurrency => {
                write!(f, "lambda:DeleteFunctionConcurrency")
            }
            LambdaActions::DeleteFunctionEventInvokeConfig => {
                write!(f, "lambda:DeleteFunctionEventInvokeConfig")
            }
            LambdaActions::DeleteFunctionUrlConfig => write!(f, "lambda:DeleteFunctionUrlConfig"),
            LambdaActions::DeleteLayerVersion => write!(f, "lambda:DeleteLayerVersion"),
            LambdaActions::DeleteProvisionedConcurrencyConfig => {
                write!(f, "lambda:DeleteProvisionedConcurrencyConfig")
            }
            LambdaActions::DisableReplication => write!(f, "lambda:DisableReplication"),
            LambdaActions::EnableReplication => write!(f, "lambda:EnableReplication"),
            LambdaActions::GetAccountSettings => write!(f, "lambda:GetAccountSettings"),
            LambdaActions::GetAlias => write!(f, "lambda:GetAlias"),
            LambdaActions::GetCodeSigningConfig => write!(f, "lambda:GetCodeSigningConfig"),
            LambdaActions::GetEventSourceMapping => write!(f, "lambda:GetEventSourceMapping"),
            LambdaActions::GetFunction => write!(f, "lambda:GetFunction"),
            LambdaActions::GetFunctionCodeSigningConfig => {
                write!(f, "lambda:GetFunctionCodeSigningConfig")
            }
            LambdaActions::GetFunctionConcurrency => write!(f, "lambda:GetFunctionConcurrency"),
            LambdaActions::GetFunctionConfiguration => write!(f, "lambda:GetFunctionConfiguration"),
            LambdaActions::GetFunctionEventInvokeConfig => {
                write!(f, "lambda:GetFunctionEventInvokeConfig")
            }
            LambdaActions::GetFunctionRecursionConfig => {
                write!(f, "lambda:GetFunctionRecursionConfig")
            }
            LambdaActions::GetFunctionUrlConfig => write!(f, "lambda:GetFunctionUrlConfig"),
            LambdaActions::GetLayerVersion => write!(f, "lambda:GetLayerVersion"),
            LambdaActions::GetLayerVersionPolicy => write!(f, "lambda:GetLayerVersionPolicy"),
            LambdaActions::GetPolicy => write!(f, "lambda:GetPolicy"),
            LambdaActions::GetProvisionedConcurrencyConfig => {
                write!(f, "lambda:GetProvisionedConcurrencyConfig")
            }
            LambdaActions::GetRuntimeManagementConfig => {
                write!(f, "lambda:GetRuntimeManagementConfig")
            }
            LambdaActions::InvokeAsync => write!(f, "lambda:InvokeAsync"),
            LambdaActions::InvokeFunction => write!(f, "lambda:InvokeFunction"),
            LambdaActions::InvokeFunctionUrl => write!(f, "lambda:InvokeFunctionUrl"),
            LambdaActions::ListAliases => write!(f, "lambda:ListAliases"),
            LambdaActions::ListCodeSigningConfigs => write!(f, "lambda:ListCodeSigningConfigs"),
            LambdaActions::ListEventSourceMappings => write!(f, "lambda:ListEventSourceMappings"),
            LambdaActions::ListFunctionEventInvokeConfigs => {
                write!(f, "lambda:ListFunctionEventInvokeConfigs")
            }
            LambdaActions::ListFunctionUrlConfigs => write!(f, "lambda:ListFunctionUrlConfigs"),
            LambdaActions::ListFunctions => write!(f, "lambda:ListFunctions"),
            LambdaActions::ListFunctionsByCodeSigningConfig => {
                write!(f, "lambda:ListFunctionsByCodeSigningConfig")
            }
            LambdaActions::ListLayerVersions => write!(f, "lambda:ListLayerVersions"),
            LambdaActions::ListLayers => write!(f, "lambda:ListLayers"),
            LambdaActions::ListProvisionedConcurrencyConfigs => {
                write!(f, "lambda:ListProvisionedConcurrencyConfigs")
            }
            LambdaActions::ListTags => write!(f, "lambda:ListTags"),
            LambdaActions::ListVersionsByFunction => write!(f, "lambda:ListVersionsByFunction"),
            LambdaActions::PublishLayerVersion => write!(f, "lambda:PublishLayerVersion"),
            LambdaActions::PublishVersion => write!(f, "lambda:PublishVersion"),
            LambdaActions::PutFunctionCodeSigningConfig => {
                write!(f, "lambda:PutFunctionCodeSigningConfig")
            }
            LambdaActions::PutFunctionConcurrency => write!(f, "lambda:PutFunctionConcurrency"),
            LambdaActions::PutFunctionEventInvokeConfig => {
                write!(f, "lambda:PutFunctionEventInvokeConfig")
            }
            LambdaActions::PutFunctionRecursionConfig => {
                write!(f, "lambda:PutFunctionRecursionConfig")
            }
            LambdaActions::PutProvisionedConcurrencyConfig => {
                write!(f, "lambda:PutProvisionedConcurrencyConfig")
            }
            LambdaActions::PutRuntimeManagementConfig => {
                write!(f, "lambda:PutRuntimeManagementConfig")
            }
            LambdaActions::RemoveLayerVersionPermission => {
                write!(f, "lambda:RemoveLayerVersionPermission")
            }
            LambdaActions::RemovePermission => write!(f, "lambda:RemovePermission"),
            LambdaActions::TagResource => write!(f, "lambda:TagResource"),
            LambdaActions::UntagResource => write!(f, "lambda:UntagResource"),
            LambdaActions::UpdateAlias => write!(f, "lambda:UpdateAlias"),
            LambdaActions::UpdateCodeSigningConfig => write!(f, "lambda:UpdateCodeSigningConfig"),
            LambdaActions::UpdateEventSourceMapping => write!(f, "lambda:UpdateEventSourceMapping"),
            LambdaActions::UpdateFunctionCode => write!(f, "lambda:UpdateFunctionCode"),
            LambdaActions::UpdateFunctionCodeSigningConfig => {
                write!(f, "lambda:UpdateFunctionCodeSigningConfig")
            }
            LambdaActions::UpdateFunctionConfiguration => {
                write!(f, "lambda:UpdateFunctionConfiguration")
            }
            LambdaActions::UpdateFunctionEventInvokeConfig => {
                write!(f, "lambda:UpdateFunctionEventInvokeConfig")
            }
            LambdaActions::UpdateFunctionUrlConfig => write!(f, "lambda:UpdateFunctionUrlConfig"),
        }
    }
}
