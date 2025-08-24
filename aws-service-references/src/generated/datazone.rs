// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum DatazoneActions {
    AcceptPredictions,
    AcceptSubscriptionRequest,
    AddEntityOwner,
    AddPolicyGrant,
    AssociateEnvironmentRole,
    AssociateGovernedTerms,
    BatchDeleteLinkedTypes,
    BatchPutLinkedTypes,
    CancelMetadataGenerationRun,
    CancelSubscription,
    CreateAccountPool,
    CreateAsset,
    CreateAssetFilter,
    CreateAssetRevision,
    CreateAssetType,
    CreateConnection,
    CreateDataProduct,
    CreateDataProductRevision,
    CreateDataSource,
    CreateDomain,
    CreateDomainUnit,
    CreateEnvironment,
    CreateEnvironmentAction,
    CreateEnvironmentBlueprint,
    CreateEnvironmentProfile,
    CreateFormType,
    CreateGlossary,
    CreateGlossaryTerm,
    CreateGroupProfile,
    CreateListingChangeSet,
    CreateProject,
    CreateProjectMembership,
    CreateProjectProfile,
    CreateRule,
    CreateSubscriptionGrant,
    CreateSubscriptionRequest,
    CreateSubscriptionTarget,
    CreateUserProfile,
    DeleteAccountPool,
    DeleteAsset,
    DeleteAssetFilter,
    DeleteAssetType,
    DeleteConnection,
    DeleteDataProduct,
    DeleteDataSource,
    DeleteDomain,
    DeleteDomainSharingPolicy,
    DeleteDomainUnit,
    DeleteEnvironment,
    DeleteEnvironmentAction,
    DeleteEnvironmentBlueprint,
    DeleteEnvironmentBlueprintConfiguration,
    DeleteEnvironmentProfile,
    DeleteFormType,
    DeleteGlossary,
    DeleteGlossaryTerm,
    DeleteListing,
    DeleteProject,
    DeleteProjectMembership,
    DeleteProjectProfile,
    DeleteRule,
    DeleteSubscriptionGrant,
    DeleteSubscriptionRequest,
    DeleteSubscriptionTarget,
    DeleteTimeSeriesDataPoints,
    DisassociateEnvironmentRole,
    DisassociateGovernedTerms,
    GetAccountPool,
    GetAsset,
    GetAssetFilter,
    GetAssetType,
    GetConnection,
    GetDataProduct,
    GetDataSource,
    GetDataSourceRun,
    GetDomain,
    GetDomainExecutionRoleCredentials,
    GetDomainSharingPolicy,
    GetDomainUnit,
    GetEnvironment,
    GetEnvironmentAction,
    GetEnvironmentActionLink,
    GetEnvironmentBlueprint,
    GetEnvironmentBlueprintConfiguration,
    GetEnvironmentCredentials,
    GetEnvironmentProfile,
    GetFormType,
    GetGlossary,
    GetGlossaryTerm,
    GetGroupProfile,
    GetIamPortalLoginUrl,
    GetJobRun,
    GetLineageEvent,
    GetLineageNode,
    GetListing,
    GetMetadataGenerationRun,
    GetProject,
    GetProjectProfile,
    GetRule,
    GetSubscription,
    GetSubscriptionEligibility,
    GetSubscriptionGrant,
    GetSubscriptionRequestDetails,
    GetSubscriptionTarget,
    GetTimeSeriesDataPoint,
    GetUpdateEligibility,
    GetUserProfile,
    ListAccountEnvironments,
    ListAccountPools,
    ListAccountsInAccountPool,
    ListAssetFilters,
    ListAssetRevisions,
    ListConnections,
    ListDataProductRevisions,
    ListDataSourceRunActivities,
    ListDataSourceRuns,
    ListDataSources,
    ListDomainUnitsForParent,
    ListDomains,
    ListEntityOwners,
    ListEnvironmentActions,
    ListEnvironmentBlueprintConfigurationSummaries,
    ListEnvironmentBlueprintConfigurations,
    ListEnvironmentBlueprints,
    ListEnvironmentProfiles,
    ListEnvironments,
    ListGroupsForUser,
    ListJobRuns,
    ListLineageEvents,
    ListLineageNodeHistory,
    ListLinkedTypes,
    ListMetadataGenerationRuns,
    ListNotifications,
    ListPolicyGrants,
    ListProjectMemberships,
    ListProjectProfiles,
    ListProjects,
    ListRules,
    ListSubscriptionGrants,
    ListSubscriptionRequests,
    ListSubscriptionTargets,
    ListSubscriptions,
    ListTagsForResource,
    ListTimeSeriesDataPoints,
    ListWarehouseMetadata,
    PostLineageEvent,
    PostTimeSeriesDataPoints,
    ProvisionDomain,
    PutDomainSharingPolicy,
    PutEnvironmentBlueprintConfiguration,
    RefreshToken,
    RejectPredictions,
    RejectSubscriptionRequest,
    RemoveEntityOwner,
    RemovePolicyGrant,
    RevokeSubscription,
    Search,
    SearchGroupProfiles,
    SearchListings,
    SearchRules,
    SearchTypes,
    SearchUserProfiles,
    SsoLogin,
    SsoLogout,
    StartAccountBootstrapAction,
    StartDataSourceRun,
    StartMetadataGenerationRun,
    StopMetadataGenerationRun,
    TagResource,
    UntagResource,
    UpdateAccountPool,
    UpdateAssetFilter,
    UpdateConnection,
    UpdateDataSource,
    UpdateDataSourceRunActivities,
    UpdateDomain,
    UpdateDomainUnit,
    UpdateEnvironment,
    UpdateEnvironmentAction,
    UpdateEnvironmentBlueprint,
    UpdateEnvironmentConfiguration,
    UpdateEnvironmentDeploymentStatus,
    UpdateEnvironmentProfile,
    UpdateGlossary,
    UpdateGlossaryTerm,
    UpdateGroupProfile,
    UpdateProject,
    UpdateProjectProfile,
    UpdateRule,
    UpdateSubscriptionGrantStatus,
    UpdateSubscriptionRequest,
    UpdateSubscriptionTarget,
    UpdateUserProfile,
    ValidatePassRole,
}
impl std::fmt::Display for DatazoneActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DatazoneActions::AcceptPredictions => write!(f, "datazone:AcceptPredictions"),
            DatazoneActions::AcceptSubscriptionRequest => {
                write!(f, "datazone:AcceptSubscriptionRequest")
            }
            DatazoneActions::AddEntityOwner => write!(f, "datazone:AddEntityOwner"),
            DatazoneActions::AddPolicyGrant => write!(f, "datazone:AddPolicyGrant"),
            DatazoneActions::AssociateEnvironmentRole => {
                write!(f, "datazone:AssociateEnvironmentRole")
            }
            DatazoneActions::AssociateGovernedTerms => write!(f, "datazone:AssociateGovernedTerms"),
            DatazoneActions::BatchDeleteLinkedTypes => write!(f, "datazone:BatchDeleteLinkedTypes"),
            DatazoneActions::BatchPutLinkedTypes => write!(f, "datazone:BatchPutLinkedTypes"),
            DatazoneActions::CancelMetadataGenerationRun => {
                write!(f, "datazone:CancelMetadataGenerationRun")
            }
            DatazoneActions::CancelSubscription => write!(f, "datazone:CancelSubscription"),
            DatazoneActions::CreateAccountPool => write!(f, "datazone:CreateAccountPool"),
            DatazoneActions::CreateAsset => write!(f, "datazone:CreateAsset"),
            DatazoneActions::CreateAssetFilter => write!(f, "datazone:CreateAssetFilter"),
            DatazoneActions::CreateAssetRevision => write!(f, "datazone:CreateAssetRevision"),
            DatazoneActions::CreateAssetType => write!(f, "datazone:CreateAssetType"),
            DatazoneActions::CreateConnection => write!(f, "datazone:CreateConnection"),
            DatazoneActions::CreateDataProduct => write!(f, "datazone:CreateDataProduct"),
            DatazoneActions::CreateDataProductRevision => {
                write!(f, "datazone:CreateDataProductRevision")
            }
            DatazoneActions::CreateDataSource => write!(f, "datazone:CreateDataSource"),
            DatazoneActions::CreateDomain => write!(f, "datazone:CreateDomain"),
            DatazoneActions::CreateDomainUnit => write!(f, "datazone:CreateDomainUnit"),
            DatazoneActions::CreateEnvironment => write!(f, "datazone:CreateEnvironment"),
            DatazoneActions::CreateEnvironmentAction => {
                write!(f, "datazone:CreateEnvironmentAction")
            }
            DatazoneActions::CreateEnvironmentBlueprint => {
                write!(f, "datazone:CreateEnvironmentBlueprint")
            }
            DatazoneActions::CreateEnvironmentProfile => {
                write!(f, "datazone:CreateEnvironmentProfile")
            }
            DatazoneActions::CreateFormType => write!(f, "datazone:CreateFormType"),
            DatazoneActions::CreateGlossary => write!(f, "datazone:CreateGlossary"),
            DatazoneActions::CreateGlossaryTerm => write!(f, "datazone:CreateGlossaryTerm"),
            DatazoneActions::CreateGroupProfile => write!(f, "datazone:CreateGroupProfile"),
            DatazoneActions::CreateListingChangeSet => write!(f, "datazone:CreateListingChangeSet"),
            DatazoneActions::CreateProject => write!(f, "datazone:CreateProject"),
            DatazoneActions::CreateProjectMembership => {
                write!(f, "datazone:CreateProjectMembership")
            }
            DatazoneActions::CreateProjectProfile => write!(f, "datazone:CreateProjectProfile"),
            DatazoneActions::CreateRule => write!(f, "datazone:CreateRule"),
            DatazoneActions::CreateSubscriptionGrant => {
                write!(f, "datazone:CreateSubscriptionGrant")
            }
            DatazoneActions::CreateSubscriptionRequest => {
                write!(f, "datazone:CreateSubscriptionRequest")
            }
            DatazoneActions::CreateSubscriptionTarget => {
                write!(f, "datazone:CreateSubscriptionTarget")
            }
            DatazoneActions::CreateUserProfile => write!(f, "datazone:CreateUserProfile"),
            DatazoneActions::DeleteAccountPool => write!(f, "datazone:DeleteAccountPool"),
            DatazoneActions::DeleteAsset => write!(f, "datazone:DeleteAsset"),
            DatazoneActions::DeleteAssetFilter => write!(f, "datazone:DeleteAssetFilter"),
            DatazoneActions::DeleteAssetType => write!(f, "datazone:DeleteAssetType"),
            DatazoneActions::DeleteConnection => write!(f, "datazone:DeleteConnection"),
            DatazoneActions::DeleteDataProduct => write!(f, "datazone:DeleteDataProduct"),
            DatazoneActions::DeleteDataSource => write!(f, "datazone:DeleteDataSource"),
            DatazoneActions::DeleteDomain => write!(f, "datazone:DeleteDomain"),
            DatazoneActions::DeleteDomainSharingPolicy => {
                write!(f, "datazone:DeleteDomainSharingPolicy")
            }
            DatazoneActions::DeleteDomainUnit => write!(f, "datazone:DeleteDomainUnit"),
            DatazoneActions::DeleteEnvironment => write!(f, "datazone:DeleteEnvironment"),
            DatazoneActions::DeleteEnvironmentAction => {
                write!(f, "datazone:DeleteEnvironmentAction")
            }
            DatazoneActions::DeleteEnvironmentBlueprint => {
                write!(f, "datazone:DeleteEnvironmentBlueprint")
            }
            DatazoneActions::DeleteEnvironmentBlueprintConfiguration => {
                write!(f, "datazone:DeleteEnvironmentBlueprintConfiguration")
            }
            DatazoneActions::DeleteEnvironmentProfile => {
                write!(f, "datazone:DeleteEnvironmentProfile")
            }
            DatazoneActions::DeleteFormType => write!(f, "datazone:DeleteFormType"),
            DatazoneActions::DeleteGlossary => write!(f, "datazone:DeleteGlossary"),
            DatazoneActions::DeleteGlossaryTerm => write!(f, "datazone:DeleteGlossaryTerm"),
            DatazoneActions::DeleteListing => write!(f, "datazone:DeleteListing"),
            DatazoneActions::DeleteProject => write!(f, "datazone:DeleteProject"),
            DatazoneActions::DeleteProjectMembership => {
                write!(f, "datazone:DeleteProjectMembership")
            }
            DatazoneActions::DeleteProjectProfile => write!(f, "datazone:DeleteProjectProfile"),
            DatazoneActions::DeleteRule => write!(f, "datazone:DeleteRule"),
            DatazoneActions::DeleteSubscriptionGrant => {
                write!(f, "datazone:DeleteSubscriptionGrant")
            }
            DatazoneActions::DeleteSubscriptionRequest => {
                write!(f, "datazone:DeleteSubscriptionRequest")
            }
            DatazoneActions::DeleteSubscriptionTarget => {
                write!(f, "datazone:DeleteSubscriptionTarget")
            }
            DatazoneActions::DeleteTimeSeriesDataPoints => {
                write!(f, "datazone:DeleteTimeSeriesDataPoints")
            }
            DatazoneActions::DisassociateEnvironmentRole => {
                write!(f, "datazone:DisassociateEnvironmentRole")
            }
            DatazoneActions::DisassociateGovernedTerms => {
                write!(f, "datazone:DisassociateGovernedTerms")
            }
            DatazoneActions::GetAccountPool => write!(f, "datazone:GetAccountPool"),
            DatazoneActions::GetAsset => write!(f, "datazone:GetAsset"),
            DatazoneActions::GetAssetFilter => write!(f, "datazone:GetAssetFilter"),
            DatazoneActions::GetAssetType => write!(f, "datazone:GetAssetType"),
            DatazoneActions::GetConnection => write!(f, "datazone:GetConnection"),
            DatazoneActions::GetDataProduct => write!(f, "datazone:GetDataProduct"),
            DatazoneActions::GetDataSource => write!(f, "datazone:GetDataSource"),
            DatazoneActions::GetDataSourceRun => write!(f, "datazone:GetDataSourceRun"),
            DatazoneActions::GetDomain => write!(f, "datazone:GetDomain"),
            DatazoneActions::GetDomainExecutionRoleCredentials => {
                write!(f, "datazone:GetDomainExecutionRoleCredentials")
            }
            DatazoneActions::GetDomainSharingPolicy => write!(f, "datazone:GetDomainSharingPolicy"),
            DatazoneActions::GetDomainUnit => write!(f, "datazone:GetDomainUnit"),
            DatazoneActions::GetEnvironment => write!(f, "datazone:GetEnvironment"),
            DatazoneActions::GetEnvironmentAction => write!(f, "datazone:GetEnvironmentAction"),
            DatazoneActions::GetEnvironmentActionLink => {
                write!(f, "datazone:GetEnvironmentActionLink")
            }
            DatazoneActions::GetEnvironmentBlueprint => {
                write!(f, "datazone:GetEnvironmentBlueprint")
            }
            DatazoneActions::GetEnvironmentBlueprintConfiguration => {
                write!(f, "datazone:GetEnvironmentBlueprintConfiguration")
            }
            DatazoneActions::GetEnvironmentCredentials => {
                write!(f, "datazone:GetEnvironmentCredentials")
            }
            DatazoneActions::GetEnvironmentProfile => write!(f, "datazone:GetEnvironmentProfile"),
            DatazoneActions::GetFormType => write!(f, "datazone:GetFormType"),
            DatazoneActions::GetGlossary => write!(f, "datazone:GetGlossary"),
            DatazoneActions::GetGlossaryTerm => write!(f, "datazone:GetGlossaryTerm"),
            DatazoneActions::GetGroupProfile => write!(f, "datazone:GetGroupProfile"),
            DatazoneActions::GetIamPortalLoginUrl => write!(f, "datazone:GetIamPortalLoginUrl"),
            DatazoneActions::GetJobRun => write!(f, "datazone:GetJobRun"),
            DatazoneActions::GetLineageEvent => write!(f, "datazone:GetLineageEvent"),
            DatazoneActions::GetLineageNode => write!(f, "datazone:GetLineageNode"),
            DatazoneActions::GetListing => write!(f, "datazone:GetListing"),
            DatazoneActions::GetMetadataGenerationRun => {
                write!(f, "datazone:GetMetadataGenerationRun")
            }
            DatazoneActions::GetProject => write!(f, "datazone:GetProject"),
            DatazoneActions::GetProjectProfile => write!(f, "datazone:GetProjectProfile"),
            DatazoneActions::GetRule => write!(f, "datazone:GetRule"),
            DatazoneActions::GetSubscription => write!(f, "datazone:GetSubscription"),
            DatazoneActions::GetSubscriptionEligibility => {
                write!(f, "datazone:GetSubscriptionEligibility")
            }
            DatazoneActions::GetSubscriptionGrant => write!(f, "datazone:GetSubscriptionGrant"),
            DatazoneActions::GetSubscriptionRequestDetails => {
                write!(f, "datazone:GetSubscriptionRequestDetails")
            }
            DatazoneActions::GetSubscriptionTarget => write!(f, "datazone:GetSubscriptionTarget"),
            DatazoneActions::GetTimeSeriesDataPoint => write!(f, "datazone:GetTimeSeriesDataPoint"),
            DatazoneActions::GetUpdateEligibility => write!(f, "datazone:GetUpdateEligibility"),
            DatazoneActions::GetUserProfile => write!(f, "datazone:GetUserProfile"),
            DatazoneActions::ListAccountEnvironments => {
                write!(f, "datazone:ListAccountEnvironments")
            }
            DatazoneActions::ListAccountPools => write!(f, "datazone:ListAccountPools"),
            DatazoneActions::ListAccountsInAccountPool => {
                write!(f, "datazone:ListAccountsInAccountPool")
            }
            DatazoneActions::ListAssetFilters => write!(f, "datazone:ListAssetFilters"),
            DatazoneActions::ListAssetRevisions => write!(f, "datazone:ListAssetRevisions"),
            DatazoneActions::ListConnections => write!(f, "datazone:ListConnections"),
            DatazoneActions::ListDataProductRevisions => {
                write!(f, "datazone:ListDataProductRevisions")
            }
            DatazoneActions::ListDataSourceRunActivities => {
                write!(f, "datazone:ListDataSourceRunActivities")
            }
            DatazoneActions::ListDataSourceRuns => write!(f, "datazone:ListDataSourceRuns"),
            DatazoneActions::ListDataSources => write!(f, "datazone:ListDataSources"),
            DatazoneActions::ListDomainUnitsForParent => {
                write!(f, "datazone:ListDomainUnitsForParent")
            }
            DatazoneActions::ListDomains => write!(f, "datazone:ListDomains"),
            DatazoneActions::ListEntityOwners => write!(f, "datazone:ListEntityOwners"),
            DatazoneActions::ListEnvironmentActions => write!(f, "datazone:ListEnvironmentActions"),
            DatazoneActions::ListEnvironmentBlueprintConfigurationSummaries => {
                write!(f, "datazone:ListEnvironmentBlueprintConfigurationSummaries")
            }
            DatazoneActions::ListEnvironmentBlueprintConfigurations => {
                write!(f, "datazone:ListEnvironmentBlueprintConfigurations")
            }
            DatazoneActions::ListEnvironmentBlueprints => {
                write!(f, "datazone:ListEnvironmentBlueprints")
            }
            DatazoneActions::ListEnvironmentProfiles => {
                write!(f, "datazone:ListEnvironmentProfiles")
            }
            DatazoneActions::ListEnvironments => write!(f, "datazone:ListEnvironments"),
            DatazoneActions::ListGroupsForUser => write!(f, "datazone:ListGroupsForUser"),
            DatazoneActions::ListJobRuns => write!(f, "datazone:ListJobRuns"),
            DatazoneActions::ListLineageEvents => write!(f, "datazone:ListLineageEvents"),
            DatazoneActions::ListLineageNodeHistory => write!(f, "datazone:ListLineageNodeHistory"),
            DatazoneActions::ListLinkedTypes => write!(f, "datazone:ListLinkedTypes"),
            DatazoneActions::ListMetadataGenerationRuns => {
                write!(f, "datazone:ListMetadataGenerationRuns")
            }
            DatazoneActions::ListNotifications => write!(f, "datazone:ListNotifications"),
            DatazoneActions::ListPolicyGrants => write!(f, "datazone:ListPolicyGrants"),
            DatazoneActions::ListProjectMemberships => write!(f, "datazone:ListProjectMemberships"),
            DatazoneActions::ListProjectProfiles => write!(f, "datazone:ListProjectProfiles"),
            DatazoneActions::ListProjects => write!(f, "datazone:ListProjects"),
            DatazoneActions::ListRules => write!(f, "datazone:ListRules"),
            DatazoneActions::ListSubscriptionGrants => write!(f, "datazone:ListSubscriptionGrants"),
            DatazoneActions::ListSubscriptionRequests => {
                write!(f, "datazone:ListSubscriptionRequests")
            }
            DatazoneActions::ListSubscriptionTargets => {
                write!(f, "datazone:ListSubscriptionTargets")
            }
            DatazoneActions::ListSubscriptions => write!(f, "datazone:ListSubscriptions"),
            DatazoneActions::ListTagsForResource => write!(f, "datazone:ListTagsForResource"),
            DatazoneActions::ListTimeSeriesDataPoints => {
                write!(f, "datazone:ListTimeSeriesDataPoints")
            }
            DatazoneActions::ListWarehouseMetadata => write!(f, "datazone:ListWarehouseMetadata"),
            DatazoneActions::PostLineageEvent => write!(f, "datazone:PostLineageEvent"),
            DatazoneActions::PostTimeSeriesDataPoints => {
                write!(f, "datazone:PostTimeSeriesDataPoints")
            }
            DatazoneActions::ProvisionDomain => write!(f, "datazone:ProvisionDomain"),
            DatazoneActions::PutDomainSharingPolicy => write!(f, "datazone:PutDomainSharingPolicy"),
            DatazoneActions::PutEnvironmentBlueprintConfiguration => {
                write!(f, "datazone:PutEnvironmentBlueprintConfiguration")
            }
            DatazoneActions::RefreshToken => write!(f, "datazone:RefreshToken"),
            DatazoneActions::RejectPredictions => write!(f, "datazone:RejectPredictions"),
            DatazoneActions::RejectSubscriptionRequest => {
                write!(f, "datazone:RejectSubscriptionRequest")
            }
            DatazoneActions::RemoveEntityOwner => write!(f, "datazone:RemoveEntityOwner"),
            DatazoneActions::RemovePolicyGrant => write!(f, "datazone:RemovePolicyGrant"),
            DatazoneActions::RevokeSubscription => write!(f, "datazone:RevokeSubscription"),
            DatazoneActions::Search => write!(f, "datazone:Search"),
            DatazoneActions::SearchGroupProfiles => write!(f, "datazone:SearchGroupProfiles"),
            DatazoneActions::SearchListings => write!(f, "datazone:SearchListings"),
            DatazoneActions::SearchRules => write!(f, "datazone:SearchRules"),
            DatazoneActions::SearchTypes => write!(f, "datazone:SearchTypes"),
            DatazoneActions::SearchUserProfiles => write!(f, "datazone:SearchUserProfiles"),
            DatazoneActions::SsoLogin => write!(f, "datazone:SsoLogin"),
            DatazoneActions::SsoLogout => write!(f, "datazone:SsoLogout"),
            DatazoneActions::StartAccountBootstrapAction => {
                write!(f, "datazone:StartAccountBootstrapAction")
            }
            DatazoneActions::StartDataSourceRun => write!(f, "datazone:StartDataSourceRun"),
            DatazoneActions::StartMetadataGenerationRun => {
                write!(f, "datazone:StartMetadataGenerationRun")
            }
            DatazoneActions::StopMetadataGenerationRun => {
                write!(f, "datazone:StopMetadataGenerationRun")
            }
            DatazoneActions::TagResource => write!(f, "datazone:TagResource"),
            DatazoneActions::UntagResource => write!(f, "datazone:UntagResource"),
            DatazoneActions::UpdateAccountPool => write!(f, "datazone:UpdateAccountPool"),
            DatazoneActions::UpdateAssetFilter => write!(f, "datazone:UpdateAssetFilter"),
            DatazoneActions::UpdateConnection => write!(f, "datazone:UpdateConnection"),
            DatazoneActions::UpdateDataSource => write!(f, "datazone:UpdateDataSource"),
            DatazoneActions::UpdateDataSourceRunActivities => {
                write!(f, "datazone:UpdateDataSourceRunActivities")
            }
            DatazoneActions::UpdateDomain => write!(f, "datazone:UpdateDomain"),
            DatazoneActions::UpdateDomainUnit => write!(f, "datazone:UpdateDomainUnit"),
            DatazoneActions::UpdateEnvironment => write!(f, "datazone:UpdateEnvironment"),
            DatazoneActions::UpdateEnvironmentAction => {
                write!(f, "datazone:UpdateEnvironmentAction")
            }
            DatazoneActions::UpdateEnvironmentBlueprint => {
                write!(f, "datazone:UpdateEnvironmentBlueprint")
            }
            DatazoneActions::UpdateEnvironmentConfiguration => {
                write!(f, "datazone:UpdateEnvironmentConfiguration")
            }
            DatazoneActions::UpdateEnvironmentDeploymentStatus => {
                write!(f, "datazone:UpdateEnvironmentDeploymentStatus")
            }
            DatazoneActions::UpdateEnvironmentProfile => {
                write!(f, "datazone:UpdateEnvironmentProfile")
            }
            DatazoneActions::UpdateGlossary => write!(f, "datazone:UpdateGlossary"),
            DatazoneActions::UpdateGlossaryTerm => write!(f, "datazone:UpdateGlossaryTerm"),
            DatazoneActions::UpdateGroupProfile => write!(f, "datazone:UpdateGroupProfile"),
            DatazoneActions::UpdateProject => write!(f, "datazone:UpdateProject"),
            DatazoneActions::UpdateProjectProfile => write!(f, "datazone:UpdateProjectProfile"),
            DatazoneActions::UpdateRule => write!(f, "datazone:UpdateRule"),
            DatazoneActions::UpdateSubscriptionGrantStatus => {
                write!(f, "datazone:UpdateSubscriptionGrantStatus")
            }
            DatazoneActions::UpdateSubscriptionRequest => {
                write!(f, "datazone:UpdateSubscriptionRequest")
            }
            DatazoneActions::UpdateSubscriptionTarget => {
                write!(f, "datazone:UpdateSubscriptionTarget")
            }
            DatazoneActions::UpdateUserProfile => write!(f, "datazone:UpdateUserProfile"),
            DatazoneActions::ValidatePassRole => write!(f, "datazone:ValidatePassRole"),
        }
    }
}
