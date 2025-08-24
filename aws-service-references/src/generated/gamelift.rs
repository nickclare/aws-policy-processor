// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum GameliftActions {
    AcceptMatch,
    ClaimGameServer,
    CreateAlias,
    CreateBuild,
    CreateContainerFleet,
    CreateContainerGroupDefinition,
    CreateFleet,
    CreateFleetLocations,
    CreateGameServerGroup,
    CreateGameSession,
    CreateGameSessionQueue,
    CreateLocation,
    CreateMatchmakingConfiguration,
    CreateMatchmakingRuleSet,
    CreatePlayerSession,
    CreatePlayerSessions,
    CreateScript,
    CreateVpcPeeringAuthorization,
    CreateVpcPeeringConnection,
    DeleteAlias,
    DeleteBuild,
    DeleteContainerFleet,
    DeleteContainerGroupDefinition,
    DeleteFleet,
    DeleteFleetLocations,
    DeleteGameServerGroup,
    DeleteGameSessionQueue,
    DeleteLocation,
    DeleteMatchmakingConfiguration,
    DeleteMatchmakingRuleSet,
    DeleteScalingPolicy,
    DeleteScript,
    DeleteVpcPeeringAuthorization,
    DeleteVpcPeeringConnection,
    DeregisterCompute,
    DeregisterGameServer,
    DescribeAlias,
    DescribeBuild,
    DescribeCompute,
    DescribeContainerFleet,
    DescribeContainerGroupDefinition,
    DescribeEc2InstanceLimits,
    DescribeFleetAttributes,
    DescribeFleetCapacity,
    DescribeFleetDeployment,
    DescribeFleetEvents,
    DescribeFleetLocationAttributes,
    DescribeFleetLocationCapacity,
    DescribeFleetLocationUtilization,
    DescribeFleetPortSettings,
    DescribeFleetUtilization,
    DescribeGameServer,
    DescribeGameServerGroup,
    DescribeGameServerInstances,
    DescribeGameSessionDetails,
    DescribeGameSessionPlacement,
    DescribeGameSessionQueues,
    DescribeGameSessions,
    DescribeInstances,
    DescribeMatchmaking,
    DescribeMatchmakingConfigurations,
    DescribeMatchmakingRuleSets,
    DescribePlayerSessions,
    DescribeRuntimeConfiguration,
    DescribeScalingPolicies,
    DescribeScript,
    DescribeVpcPeeringAuthorizations,
    DescribeVpcPeeringConnections,
    GetComputeAccess,
    GetComputeAuthToken,
    GetGameSessionLogUrl,
    GetInstanceAccess,
    ListAliases,
    ListBuilds,
    ListCompute,
    ListContainerFleets,
    ListContainerGroupDefinitionVersions,
    ListContainerGroupDefinitions,
    ListFleetDeployments,
    ListFleets,
    ListGameServerGroups,
    ListGameServers,
    ListLocations,
    ListScripts,
    ListTagsForResource,
    PutScalingPolicy,
    RegisterCompute,
    RegisterGameServer,
    RequestUploadCredentials,
    ResolveAlias,
    ResumeGameServerGroup,
    SearchGameSessions,
    StartFleetActions,
    StartGameSessionPlacement,
    StartMatchBackfill,
    StartMatchmaking,
    StopFleetActions,
    StopGameSessionPlacement,
    StopMatchmaking,
    SuspendGameServerGroup,
    TagResource,
    TerminateGameSession,
    UntagResource,
    UpdateAlias,
    UpdateBuild,
    UpdateContainerFleet,
    UpdateContainerGroupDefinition,
    UpdateFleetAttributes,
    UpdateFleetCapacity,
    UpdateFleetPortSettings,
    UpdateGameServer,
    UpdateGameServerGroup,
    UpdateGameSession,
    UpdateGameSessionQueue,
    UpdateMatchmakingConfiguration,
    UpdateRuntimeConfiguration,
    UpdateScript,
    ValidateMatchmakingRuleSet,
}
impl std::fmt::Display for GameliftActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GameliftActions::AcceptMatch => write!(f, "gamelift:AcceptMatch"),
            GameliftActions::ClaimGameServer => write!(f, "gamelift:ClaimGameServer"),
            GameliftActions::CreateAlias => write!(f, "gamelift:CreateAlias"),
            GameliftActions::CreateBuild => write!(f, "gamelift:CreateBuild"),
            GameliftActions::CreateContainerFleet => write!(f, "gamelift:CreateContainerFleet"),
            GameliftActions::CreateContainerGroupDefinition => {
                write!(f, "gamelift:CreateContainerGroupDefinition")
            }
            GameliftActions::CreateFleet => write!(f, "gamelift:CreateFleet"),
            GameliftActions::CreateFleetLocations => write!(f, "gamelift:CreateFleetLocations"),
            GameliftActions::CreateGameServerGroup => write!(f, "gamelift:CreateGameServerGroup"),
            GameliftActions::CreateGameSession => write!(f, "gamelift:CreateGameSession"),
            GameliftActions::CreateGameSessionQueue => write!(f, "gamelift:CreateGameSessionQueue"),
            GameliftActions::CreateLocation => write!(f, "gamelift:CreateLocation"),
            GameliftActions::CreateMatchmakingConfiguration => {
                write!(f, "gamelift:CreateMatchmakingConfiguration")
            }
            GameliftActions::CreateMatchmakingRuleSet => {
                write!(f, "gamelift:CreateMatchmakingRuleSet")
            }
            GameliftActions::CreatePlayerSession => write!(f, "gamelift:CreatePlayerSession"),
            GameliftActions::CreatePlayerSessions => write!(f, "gamelift:CreatePlayerSessions"),
            GameliftActions::CreateScript => write!(f, "gamelift:CreateScript"),
            GameliftActions::CreateVpcPeeringAuthorization => {
                write!(f, "gamelift:CreateVpcPeeringAuthorization")
            }
            GameliftActions::CreateVpcPeeringConnection => {
                write!(f, "gamelift:CreateVpcPeeringConnection")
            }
            GameliftActions::DeleteAlias => write!(f, "gamelift:DeleteAlias"),
            GameliftActions::DeleteBuild => write!(f, "gamelift:DeleteBuild"),
            GameliftActions::DeleteContainerFleet => write!(f, "gamelift:DeleteContainerFleet"),
            GameliftActions::DeleteContainerGroupDefinition => {
                write!(f, "gamelift:DeleteContainerGroupDefinition")
            }
            GameliftActions::DeleteFleet => write!(f, "gamelift:DeleteFleet"),
            GameliftActions::DeleteFleetLocations => write!(f, "gamelift:DeleteFleetLocations"),
            GameliftActions::DeleteGameServerGroup => write!(f, "gamelift:DeleteGameServerGroup"),
            GameliftActions::DeleteGameSessionQueue => write!(f, "gamelift:DeleteGameSessionQueue"),
            GameliftActions::DeleteLocation => write!(f, "gamelift:DeleteLocation"),
            GameliftActions::DeleteMatchmakingConfiguration => {
                write!(f, "gamelift:DeleteMatchmakingConfiguration")
            }
            GameliftActions::DeleteMatchmakingRuleSet => {
                write!(f, "gamelift:DeleteMatchmakingRuleSet")
            }
            GameliftActions::DeleteScalingPolicy => write!(f, "gamelift:DeleteScalingPolicy"),
            GameliftActions::DeleteScript => write!(f, "gamelift:DeleteScript"),
            GameliftActions::DeleteVpcPeeringAuthorization => {
                write!(f, "gamelift:DeleteVpcPeeringAuthorization")
            }
            GameliftActions::DeleteVpcPeeringConnection => {
                write!(f, "gamelift:DeleteVpcPeeringConnection")
            }
            GameliftActions::DeregisterCompute => write!(f, "gamelift:DeregisterCompute"),
            GameliftActions::DeregisterGameServer => write!(f, "gamelift:DeregisterGameServer"),
            GameliftActions::DescribeAlias => write!(f, "gamelift:DescribeAlias"),
            GameliftActions::DescribeBuild => write!(f, "gamelift:DescribeBuild"),
            GameliftActions::DescribeCompute => write!(f, "gamelift:DescribeCompute"),
            GameliftActions::DescribeContainerFleet => write!(f, "gamelift:DescribeContainerFleet"),
            GameliftActions::DescribeContainerGroupDefinition => {
                write!(f, "gamelift:DescribeContainerGroupDefinition")
            }
            GameliftActions::DescribeEc2InstanceLimits => {
                write!(f, "gamelift:DescribeEC2InstanceLimits")
            }
            GameliftActions::DescribeFleetAttributes => {
                write!(f, "gamelift:DescribeFleetAttributes")
            }
            GameliftActions::DescribeFleetCapacity => write!(f, "gamelift:DescribeFleetCapacity"),
            GameliftActions::DescribeFleetDeployment => {
                write!(f, "gamelift:DescribeFleetDeployment")
            }
            GameliftActions::DescribeFleetEvents => write!(f, "gamelift:DescribeFleetEvents"),
            GameliftActions::DescribeFleetLocationAttributes => {
                write!(f, "gamelift:DescribeFleetLocationAttributes")
            }
            GameliftActions::DescribeFleetLocationCapacity => {
                write!(f, "gamelift:DescribeFleetLocationCapacity")
            }
            GameliftActions::DescribeFleetLocationUtilization => {
                write!(f, "gamelift:DescribeFleetLocationUtilization")
            }
            GameliftActions::DescribeFleetPortSettings => {
                write!(f, "gamelift:DescribeFleetPortSettings")
            }
            GameliftActions::DescribeFleetUtilization => {
                write!(f, "gamelift:DescribeFleetUtilization")
            }
            GameliftActions::DescribeGameServer => write!(f, "gamelift:DescribeGameServer"),
            GameliftActions::DescribeGameServerGroup => {
                write!(f, "gamelift:DescribeGameServerGroup")
            }
            GameliftActions::DescribeGameServerInstances => {
                write!(f, "gamelift:DescribeGameServerInstances")
            }
            GameliftActions::DescribeGameSessionDetails => {
                write!(f, "gamelift:DescribeGameSessionDetails")
            }
            GameliftActions::DescribeGameSessionPlacement => {
                write!(f, "gamelift:DescribeGameSessionPlacement")
            }
            GameliftActions::DescribeGameSessionQueues => {
                write!(f, "gamelift:DescribeGameSessionQueues")
            }
            GameliftActions::DescribeGameSessions => write!(f, "gamelift:DescribeGameSessions"),
            GameliftActions::DescribeInstances => write!(f, "gamelift:DescribeInstances"),
            GameliftActions::DescribeMatchmaking => write!(f, "gamelift:DescribeMatchmaking"),
            GameliftActions::DescribeMatchmakingConfigurations => {
                write!(f, "gamelift:DescribeMatchmakingConfigurations")
            }
            GameliftActions::DescribeMatchmakingRuleSets => {
                write!(f, "gamelift:DescribeMatchmakingRuleSets")
            }
            GameliftActions::DescribePlayerSessions => write!(f, "gamelift:DescribePlayerSessions"),
            GameliftActions::DescribeRuntimeConfiguration => {
                write!(f, "gamelift:DescribeRuntimeConfiguration")
            }
            GameliftActions::DescribeScalingPolicies => {
                write!(f, "gamelift:DescribeScalingPolicies")
            }
            GameliftActions::DescribeScript => write!(f, "gamelift:DescribeScript"),
            GameliftActions::DescribeVpcPeeringAuthorizations => {
                write!(f, "gamelift:DescribeVpcPeeringAuthorizations")
            }
            GameliftActions::DescribeVpcPeeringConnections => {
                write!(f, "gamelift:DescribeVpcPeeringConnections")
            }
            GameliftActions::GetComputeAccess => write!(f, "gamelift:GetComputeAccess"),
            GameliftActions::GetComputeAuthToken => write!(f, "gamelift:GetComputeAuthToken"),
            GameliftActions::GetGameSessionLogUrl => write!(f, "gamelift:GetGameSessionLogUrl"),
            GameliftActions::GetInstanceAccess => write!(f, "gamelift:GetInstanceAccess"),
            GameliftActions::ListAliases => write!(f, "gamelift:ListAliases"),
            GameliftActions::ListBuilds => write!(f, "gamelift:ListBuilds"),
            GameliftActions::ListCompute => write!(f, "gamelift:ListCompute"),
            GameliftActions::ListContainerFleets => write!(f, "gamelift:ListContainerFleets"),
            GameliftActions::ListContainerGroupDefinitionVersions => {
                write!(f, "gamelift:ListContainerGroupDefinitionVersions")
            }
            GameliftActions::ListContainerGroupDefinitions => {
                write!(f, "gamelift:ListContainerGroupDefinitions")
            }
            GameliftActions::ListFleetDeployments => write!(f, "gamelift:ListFleetDeployments"),
            GameliftActions::ListFleets => write!(f, "gamelift:ListFleets"),
            GameliftActions::ListGameServerGroups => write!(f, "gamelift:ListGameServerGroups"),
            GameliftActions::ListGameServers => write!(f, "gamelift:ListGameServers"),
            GameliftActions::ListLocations => write!(f, "gamelift:ListLocations"),
            GameliftActions::ListScripts => write!(f, "gamelift:ListScripts"),
            GameliftActions::ListTagsForResource => write!(f, "gamelift:ListTagsForResource"),
            GameliftActions::PutScalingPolicy => write!(f, "gamelift:PutScalingPolicy"),
            GameliftActions::RegisterCompute => write!(f, "gamelift:RegisterCompute"),
            GameliftActions::RegisterGameServer => write!(f, "gamelift:RegisterGameServer"),
            GameliftActions::RequestUploadCredentials => {
                write!(f, "gamelift:RequestUploadCredentials")
            }
            GameliftActions::ResolveAlias => write!(f, "gamelift:ResolveAlias"),
            GameliftActions::ResumeGameServerGroup => write!(f, "gamelift:ResumeGameServerGroup"),
            GameliftActions::SearchGameSessions => write!(f, "gamelift:SearchGameSessions"),
            GameliftActions::StartFleetActions => write!(f, "gamelift:StartFleetActions"),
            GameliftActions::StartGameSessionPlacement => {
                write!(f, "gamelift:StartGameSessionPlacement")
            }
            GameliftActions::StartMatchBackfill => write!(f, "gamelift:StartMatchBackfill"),
            GameliftActions::StartMatchmaking => write!(f, "gamelift:StartMatchmaking"),
            GameliftActions::StopFleetActions => write!(f, "gamelift:StopFleetActions"),
            GameliftActions::StopGameSessionPlacement => {
                write!(f, "gamelift:StopGameSessionPlacement")
            }
            GameliftActions::StopMatchmaking => write!(f, "gamelift:StopMatchmaking"),
            GameliftActions::SuspendGameServerGroup => write!(f, "gamelift:SuspendGameServerGroup"),
            GameliftActions::TagResource => write!(f, "gamelift:TagResource"),
            GameliftActions::TerminateGameSession => write!(f, "gamelift:TerminateGameSession"),
            GameliftActions::UntagResource => write!(f, "gamelift:UntagResource"),
            GameliftActions::UpdateAlias => write!(f, "gamelift:UpdateAlias"),
            GameliftActions::UpdateBuild => write!(f, "gamelift:UpdateBuild"),
            GameliftActions::UpdateContainerFleet => write!(f, "gamelift:UpdateContainerFleet"),
            GameliftActions::UpdateContainerGroupDefinition => {
                write!(f, "gamelift:UpdateContainerGroupDefinition")
            }
            GameliftActions::UpdateFleetAttributes => write!(f, "gamelift:UpdateFleetAttributes"),
            GameliftActions::UpdateFleetCapacity => write!(f, "gamelift:UpdateFleetCapacity"),
            GameliftActions::UpdateFleetPortSettings => {
                write!(f, "gamelift:UpdateFleetPortSettings")
            }
            GameliftActions::UpdateGameServer => write!(f, "gamelift:UpdateGameServer"),
            GameliftActions::UpdateGameServerGroup => write!(f, "gamelift:UpdateGameServerGroup"),
            GameliftActions::UpdateGameSession => write!(f, "gamelift:UpdateGameSession"),
            GameliftActions::UpdateGameSessionQueue => write!(f, "gamelift:UpdateGameSessionQueue"),
            GameliftActions::UpdateMatchmakingConfiguration => {
                write!(f, "gamelift:UpdateMatchmakingConfiguration")
            }
            GameliftActions::UpdateRuntimeConfiguration => {
                write!(f, "gamelift:UpdateRuntimeConfiguration")
            }
            GameliftActions::UpdateScript => write!(f, "gamelift:UpdateScript"),
            GameliftActions::ValidateMatchmakingRuleSet => {
                write!(f, "gamelift:ValidateMatchmakingRuleSet")
            }
        }
    }
}
