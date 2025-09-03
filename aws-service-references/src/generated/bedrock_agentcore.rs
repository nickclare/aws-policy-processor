// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum BedrockAgentcoreActions {
    AllowVendedLogDeliveryForResource,
    ConnectBrowserAutomationStream,
    ConnectBrowserLiveViewStream,
    CreateAgentRuntime,
    CreateAgentRuntimeEndpoint,
    CreateApiKeyCredentialProvider,
    CreateBrowser,
    CreateCodeInterpreter,
    CreateEvent,
    CreateGateway,
    CreateGatewayTarget,
    CreateMemory,
    CreateOauth2CredentialProvider,
    CreateWorkloadIdentity,
    DeleteAgentRuntime,
    DeleteAgentRuntimeEndpoint,
    DeleteApiKeyCredentialProvider,
    DeleteBrowser,
    DeleteCodeInterpreter,
    DeleteEvent,
    DeleteGateway,
    DeleteGatewayTarget,
    DeleteMemory,
    DeleteMemoryRecord,
    DeleteOauth2CredentialProvider,
    DeleteWorkloadIdentity,
    GetAgentRuntime,
    GetAgentRuntimeEndpoint,
    GetApiKeyCredentialProvider,
    GetBrowser,
    GetBrowserSession,
    GetCodeInterpreter,
    GetCodeInterpreterSession,
    GetEvent,
    GetGateway,
    GetGatewayTarget,
    GetMemory,
    GetMemoryRecord,
    GetOauth2CredentialProvider,
    GetResourceApiKey,
    GetResourceOauth2Token,
    GetTokenVault,
    GetWorkloadAccessToken,
    GetWorkloadAccessTokenForJwt,
    GetWorkloadAccessTokenForUserId,
    GetWorkloadIdentity,
    InvokeAgentRuntime,
    InvokeCodeInterpreter,
    ListActors,
    ListAgentRuntimeEndpoints,
    ListAgentRuntimeVersions,
    ListAgentRuntimes,
    ListApiKeyCredentialProviders,
    ListBrowserSessions,
    ListBrowsers,
    ListCodeInterpreterSessions,
    ListCodeInterpreters,
    ListEvents,
    ListGatewayTargets,
    ListGateways,
    ListMemories,
    ListMemoryRecords,
    ListOauth2CredentialProviders,
    ListSessions,
    ListWorkloadIdentities,
    RetrieveMemoryRecords,
    SetTokenVaultCmk,
    StartBrowserSession,
    StartCodeInterpreterSession,
    StopBrowserSession,
    StopCodeInterpreterSession,
    SynchronizeGatewayTargets,
    UpdateAgentRuntime,
    UpdateAgentRuntimeEndpoint,
    UpdateApiKeyCredentialProvider,
    UpdateBrowserStream,
    UpdateGateway,
    UpdateGatewayTarget,
    UpdateMemory,
    UpdateOauth2CredentialProvider,
    UpdateWorkloadIdentity,
}
impl std::fmt::Display for BedrockAgentcoreActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BedrockAgentcoreActions::AllowVendedLogDeliveryForResource => {
                write!(f, "bedrock-agentcore:AllowVendedLogDeliveryForResource")
            }
            BedrockAgentcoreActions::ConnectBrowserAutomationStream => {
                write!(f, "bedrock-agentcore:ConnectBrowserAutomationStream")
            }
            BedrockAgentcoreActions::ConnectBrowserLiveViewStream => {
                write!(f, "bedrock-agentcore:ConnectBrowserLiveViewStream")
            }
            BedrockAgentcoreActions::CreateAgentRuntime => {
                write!(f, "bedrock-agentcore:CreateAgentRuntime")
            }
            BedrockAgentcoreActions::CreateAgentRuntimeEndpoint => {
                write!(f, "bedrock-agentcore:CreateAgentRuntimeEndpoint")
            }
            BedrockAgentcoreActions::CreateApiKeyCredentialProvider => {
                write!(f, "bedrock-agentcore:CreateApiKeyCredentialProvider")
            }
            BedrockAgentcoreActions::CreateBrowser => write!(f, "bedrock-agentcore:CreateBrowser"),
            BedrockAgentcoreActions::CreateCodeInterpreter => {
                write!(f, "bedrock-agentcore:CreateCodeInterpreter")
            }
            BedrockAgentcoreActions::CreateEvent => write!(f, "bedrock-agentcore:CreateEvent"),
            BedrockAgentcoreActions::CreateGateway => write!(f, "bedrock-agentcore:CreateGateway"),
            BedrockAgentcoreActions::CreateGatewayTarget => {
                write!(f, "bedrock-agentcore:CreateGatewayTarget")
            }
            BedrockAgentcoreActions::CreateMemory => write!(f, "bedrock-agentcore:CreateMemory"),
            BedrockAgentcoreActions::CreateOauth2CredentialProvider => {
                write!(f, "bedrock-agentcore:CreateOauth2CredentialProvider")
            }
            BedrockAgentcoreActions::CreateWorkloadIdentity => {
                write!(f, "bedrock-agentcore:CreateWorkloadIdentity")
            }
            BedrockAgentcoreActions::DeleteAgentRuntime => {
                write!(f, "bedrock-agentcore:DeleteAgentRuntime")
            }
            BedrockAgentcoreActions::DeleteAgentRuntimeEndpoint => {
                write!(f, "bedrock-agentcore:DeleteAgentRuntimeEndpoint")
            }
            BedrockAgentcoreActions::DeleteApiKeyCredentialProvider => {
                write!(f, "bedrock-agentcore:DeleteApiKeyCredentialProvider")
            }
            BedrockAgentcoreActions::DeleteBrowser => write!(f, "bedrock-agentcore:DeleteBrowser"),
            BedrockAgentcoreActions::DeleteCodeInterpreter => {
                write!(f, "bedrock-agentcore:DeleteCodeInterpreter")
            }
            BedrockAgentcoreActions::DeleteEvent => write!(f, "bedrock-agentcore:DeleteEvent"),
            BedrockAgentcoreActions::DeleteGateway => write!(f, "bedrock-agentcore:DeleteGateway"),
            BedrockAgentcoreActions::DeleteGatewayTarget => {
                write!(f, "bedrock-agentcore:DeleteGatewayTarget")
            }
            BedrockAgentcoreActions::DeleteMemory => write!(f, "bedrock-agentcore:DeleteMemory"),
            BedrockAgentcoreActions::DeleteMemoryRecord => {
                write!(f, "bedrock-agentcore:DeleteMemoryRecord")
            }
            BedrockAgentcoreActions::DeleteOauth2CredentialProvider => {
                write!(f, "bedrock-agentcore:DeleteOauth2CredentialProvider")
            }
            BedrockAgentcoreActions::DeleteWorkloadIdentity => {
                write!(f, "bedrock-agentcore:DeleteWorkloadIdentity")
            }
            BedrockAgentcoreActions::GetAgentRuntime => {
                write!(f, "bedrock-agentcore:GetAgentRuntime")
            }
            BedrockAgentcoreActions::GetAgentRuntimeEndpoint => {
                write!(f, "bedrock-agentcore:GetAgentRuntimeEndpoint")
            }
            BedrockAgentcoreActions::GetApiKeyCredentialProvider => {
                write!(f, "bedrock-agentcore:GetApiKeyCredentialProvider")
            }
            BedrockAgentcoreActions::GetBrowser => write!(f, "bedrock-agentcore:GetBrowser"),
            BedrockAgentcoreActions::GetBrowserSession => {
                write!(f, "bedrock-agentcore:GetBrowserSession")
            }
            BedrockAgentcoreActions::GetCodeInterpreter => {
                write!(f, "bedrock-agentcore:GetCodeInterpreter")
            }
            BedrockAgentcoreActions::GetCodeInterpreterSession => {
                write!(f, "bedrock-agentcore:GetCodeInterpreterSession")
            }
            BedrockAgentcoreActions::GetEvent => write!(f, "bedrock-agentcore:GetEvent"),
            BedrockAgentcoreActions::GetGateway => write!(f, "bedrock-agentcore:GetGateway"),
            BedrockAgentcoreActions::GetGatewayTarget => {
                write!(f, "bedrock-agentcore:GetGatewayTarget")
            }
            BedrockAgentcoreActions::GetMemory => write!(f, "bedrock-agentcore:GetMemory"),
            BedrockAgentcoreActions::GetMemoryRecord => {
                write!(f, "bedrock-agentcore:GetMemoryRecord")
            }
            BedrockAgentcoreActions::GetOauth2CredentialProvider => {
                write!(f, "bedrock-agentcore:GetOauth2CredentialProvider")
            }
            BedrockAgentcoreActions::GetResourceApiKey => {
                write!(f, "bedrock-agentcore:GetResourceApiKey")
            }
            BedrockAgentcoreActions::GetResourceOauth2Token => {
                write!(f, "bedrock-agentcore:GetResourceOauth2Token")
            }
            BedrockAgentcoreActions::GetTokenVault => write!(f, "bedrock-agentcore:GetTokenVault"),
            BedrockAgentcoreActions::GetWorkloadAccessToken => {
                write!(f, "bedrock-agentcore:GetWorkloadAccessToken")
            }
            BedrockAgentcoreActions::GetWorkloadAccessTokenForJwt => {
                write!(f, "bedrock-agentcore:GetWorkloadAccessTokenForJWT")
            }
            BedrockAgentcoreActions::GetWorkloadAccessTokenForUserId => {
                write!(f, "bedrock-agentcore:GetWorkloadAccessTokenForUserId")
            }
            BedrockAgentcoreActions::GetWorkloadIdentity => {
                write!(f, "bedrock-agentcore:GetWorkloadIdentity")
            }
            BedrockAgentcoreActions::InvokeAgentRuntime => {
                write!(f, "bedrock-agentcore:InvokeAgentRuntime")
            }
            BedrockAgentcoreActions::InvokeCodeInterpreter => {
                write!(f, "bedrock-agentcore:InvokeCodeInterpreter")
            }
            BedrockAgentcoreActions::ListActors => write!(f, "bedrock-agentcore:ListActors"),
            BedrockAgentcoreActions::ListAgentRuntimeEndpoints => {
                write!(f, "bedrock-agentcore:ListAgentRuntimeEndpoints")
            }
            BedrockAgentcoreActions::ListAgentRuntimeVersions => {
                write!(f, "bedrock-agentcore:ListAgentRuntimeVersions")
            }
            BedrockAgentcoreActions::ListAgentRuntimes => {
                write!(f, "bedrock-agentcore:ListAgentRuntimes")
            }
            BedrockAgentcoreActions::ListApiKeyCredentialProviders => {
                write!(f, "bedrock-agentcore:ListApiKeyCredentialProviders")
            }
            BedrockAgentcoreActions::ListBrowserSessions => {
                write!(f, "bedrock-agentcore:ListBrowserSessions")
            }
            BedrockAgentcoreActions::ListBrowsers => write!(f, "bedrock-agentcore:ListBrowsers"),
            BedrockAgentcoreActions::ListCodeInterpreterSessions => {
                write!(f, "bedrock-agentcore:ListCodeInterpreterSessions")
            }
            BedrockAgentcoreActions::ListCodeInterpreters => {
                write!(f, "bedrock-agentcore:ListCodeInterpreters")
            }
            BedrockAgentcoreActions::ListEvents => write!(f, "bedrock-agentcore:ListEvents"),
            BedrockAgentcoreActions::ListGatewayTargets => {
                write!(f, "bedrock-agentcore:ListGatewayTargets")
            }
            BedrockAgentcoreActions::ListGateways => write!(f, "bedrock-agentcore:ListGateways"),
            BedrockAgentcoreActions::ListMemories => write!(f, "bedrock-agentcore:ListMemories"),
            BedrockAgentcoreActions::ListMemoryRecords => {
                write!(f, "bedrock-agentcore:ListMemoryRecords")
            }
            BedrockAgentcoreActions::ListOauth2CredentialProviders => {
                write!(f, "bedrock-agentcore:ListOauth2CredentialProviders")
            }
            BedrockAgentcoreActions::ListSessions => write!(f, "bedrock-agentcore:ListSessions"),
            BedrockAgentcoreActions::ListWorkloadIdentities => {
                write!(f, "bedrock-agentcore:ListWorkloadIdentities")
            }
            BedrockAgentcoreActions::RetrieveMemoryRecords => {
                write!(f, "bedrock-agentcore:RetrieveMemoryRecords")
            }
            BedrockAgentcoreActions::SetTokenVaultCmk => {
                write!(f, "bedrock-agentcore:SetTokenVaultCMK")
            }
            BedrockAgentcoreActions::StartBrowserSession => {
                write!(f, "bedrock-agentcore:StartBrowserSession")
            }
            BedrockAgentcoreActions::StartCodeInterpreterSession => {
                write!(f, "bedrock-agentcore:StartCodeInterpreterSession")
            }
            BedrockAgentcoreActions::StopBrowserSession => {
                write!(f, "bedrock-agentcore:StopBrowserSession")
            }
            BedrockAgentcoreActions::StopCodeInterpreterSession => {
                write!(f, "bedrock-agentcore:StopCodeInterpreterSession")
            }
            BedrockAgentcoreActions::SynchronizeGatewayTargets => {
                write!(f, "bedrock-agentcore:SynchronizeGatewayTargets")
            }
            BedrockAgentcoreActions::UpdateAgentRuntime => {
                write!(f, "bedrock-agentcore:UpdateAgentRuntime")
            }
            BedrockAgentcoreActions::UpdateAgentRuntimeEndpoint => {
                write!(f, "bedrock-agentcore:UpdateAgentRuntimeEndpoint")
            }
            BedrockAgentcoreActions::UpdateApiKeyCredentialProvider => {
                write!(f, "bedrock-agentcore:UpdateApiKeyCredentialProvider")
            }
            BedrockAgentcoreActions::UpdateBrowserStream => {
                write!(f, "bedrock-agentcore:UpdateBrowserStream")
            }
            BedrockAgentcoreActions::UpdateGateway => write!(f, "bedrock-agentcore:UpdateGateway"),
            BedrockAgentcoreActions::UpdateGatewayTarget => {
                write!(f, "bedrock-agentcore:UpdateGatewayTarget")
            }
            BedrockAgentcoreActions::UpdateMemory => write!(f, "bedrock-agentcore:UpdateMemory"),
            BedrockAgentcoreActions::UpdateOauth2CredentialProvider => {
                write!(f, "bedrock-agentcore:UpdateOauth2CredentialProvider")
            }
            BedrockAgentcoreActions::UpdateWorkloadIdentity => {
                write!(f, "bedrock-agentcore:UpdateWorkloadIdentity")
            }
        }
    }
}
