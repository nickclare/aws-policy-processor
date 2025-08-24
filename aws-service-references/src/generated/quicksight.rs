// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum QuicksightActions {
    AccountConfigurations,
    BatchCreateTopicReviewedAnswer,
    BatchDeleteTopicReviewedAnswer,
    CancelIngestion,
    CreateAccountCustomization,
    CreateAccountSubscription,
    CreateAdmin,
    CreateAnalysis,
    CreateBrand,
    CreateCustomPermissions,
    CreateDashboard,
    CreateDataSet,
    CreateDataSource,
    CreateEmailCustomizationTemplate,
    CreateFolder,
    CreateFolderMembership,
    CreateGroup,
    CreateGroupMembership,
    CreateIamPolicyAssignment,
    CreateIngestion,
    CreateNamespace,
    CreateReader,
    CreateRefreshSchedule,
    CreateRoleMembership,
    CreateTemplate,
    CreateTemplateAlias,
    CreateTheme,
    CreateThemeAlias,
    CreateTopic,
    CreateTopicRefreshSchedule,
    CreateUser,
    CreateVpcConnection,
    DeleteAccountCustomization,
    DeleteAccountSubscription,
    DeleteAnalysis,
    DeleteBrand,
    DeleteBrandAssignment,
    DeleteCustomPermissions,
    DeleteDashboard,
    DeleteDataSet,
    DeleteDataSetRefreshProperties,
    DeleteDataSource,
    DeleteDefaultQBusinessApplication,
    DeleteEmailCustomizationTemplate,
    DeleteFolder,
    DeleteFolderMembership,
    DeleteGroup,
    DeleteGroupMembership,
    DeleteIamPolicyAssignment,
    DeleteIdentityPropagationConfig,
    DeleteNamespace,
    DeleteRefreshSchedule,
    DeleteRoleCustomPermission,
    DeleteRoleMembership,
    DeleteTemplate,
    DeleteTemplateAlias,
    DeleteTheme,
    DeleteThemeAlias,
    DeleteTopic,
    DeleteTopicRefreshSchedule,
    DeleteUser,
    DeleteUserByPrincipalId,
    DeleteUserCustomPermission,
    DeleteVpcConnection,
    DescribeAccountCustomization,
    DescribeAccountSettings,
    DescribeAccountSubscription,
    DescribeAnalysis,
    DescribeAnalysisPermissions,
    DescribeAssetBundleExportJob,
    DescribeAssetBundleImportJob,
    DescribeBrand,
    DescribeBrandAssignment,
    DescribeBrandPublishedVersion,
    DescribeCustomPermissions,
    DescribeDashboard,
    DescribeDashboardPermissions,
    DescribeDashboardSnapshotJob,
    DescribeDashboardSnapshotJobResult,
    DescribeDashboardsQaConfiguration,
    DescribeDataSet,
    DescribeDataSetPermissions,
    DescribeDataSetRefreshProperties,
    DescribeDataSource,
    DescribeDataSourcePermissions,
    DescribeDefaultQBusinessApplication,
    DescribeEmailCustomizationTemplate,
    DescribeFolder,
    DescribeFolderPermissions,
    DescribeFolderResolvedPermissions,
    DescribeGroup,
    DescribeGroupMembership,
    DescribeIamPolicyAssignment,
    DescribeIngestion,
    DescribeIpRestriction,
    DescribeKeyRegistration,
    DescribeNamespace,
    DescribeQPersonalizationConfiguration,
    DescribeQuickSightQSearchConfiguration,
    DescribeRefreshSchedule,
    DescribeRoleCustomPermission,
    DescribeTemplate,
    DescribeTemplateAlias,
    DescribeTemplatePermissions,
    DescribeTheme,
    DescribeThemeAlias,
    DescribeThemePermissions,
    DescribeTopic,
    DescribeTopicPermissions,
    DescribeTopicRefresh,
    DescribeTopicRefreshSchedule,
    DescribeUser,
    DescribeVpcConnection,
    GenerateEmbedUrlForAnonymousUser,
    GenerateEmbedUrlForRegisteredUser,
    GenerateEmbedUrlForRegisteredUserWithIdentity,
    GetAnonymousUserEmbedUrl,
    GetAuthCode,
    GetDashboardEmbedUrl,
    GetGroupMapping,
    GetSessionEmbedUrl,
    ListAnalyses,
    ListAssetBundleExportJobs,
    ListAssetBundleImportJobs,
    ListBrands,
    ListCustomPermissions,
    ListCustomerManagedKeys,
    ListDashboardVersions,
    ListDashboards,
    ListDataSets,
    ListDataSources,
    ListFolderMembers,
    ListFolders,
    ListFoldersForResource,
    ListGroupMemberships,
    ListGroups,
    ListIamPolicyAssignments,
    ListIamPolicyAssignmentsForUser,
    ListIdentityPropagationConfigs,
    ListIngestions,
    ListKmsKeysForUser,
    ListNamespaces,
    ListRefreshSchedules,
    ListRoleMemberships,
    ListTagsForResource,
    ListTemplateAliases,
    ListTemplateVersions,
    ListTemplates,
    ListThemeAliases,
    ListThemeVersions,
    ListThemes,
    ListTopicRefreshSchedules,
    ListTopicReviewedAnswers,
    ListTopics,
    ListUserGroups,
    ListUsers,
    ListVpcConnections,
    PassDataSet,
    PassDataSource,
    PredictQaResults,
    PutDataSetRefreshProperties,
    RegisterCustomerManagedKey,
    RegisterUser,
    RemoveCustomerManagedKey,
    RestoreAnalysis,
    ScopeDownPolicy,
    SearchAnalyses,
    SearchDashboards,
    SearchDataSets,
    SearchDataSources,
    SearchDirectoryGroups,
    SearchFolders,
    SearchGroups,
    SearchTopics,
    SearchUsers,
    SetGroupMapping,
    StartAssetBundleExportJob,
    StartAssetBundleImportJob,
    StartDashboardSnapshotJob,
    StartDashboardSnapshotJobSchedule,
    Subscribe,
    TagResource,
    Unsubscribe,
    UntagResource,
    UpdateAccountCustomization,
    UpdateAccountSettings,
    UpdateAnalysis,
    UpdateAnalysisPermissions,
    UpdateApplicationWithTokenExchangeGrant,
    UpdateBrand,
    UpdateBrandAssignment,
    UpdateBrandPublishedVersion,
    UpdateCustomPermissions,
    UpdateDashboard,
    UpdateDashboardLinks,
    UpdateDashboardPermissions,
    UpdateDashboardPublishedVersion,
    UpdateDashboardsQaConfiguration,
    UpdateDataSet,
    UpdateDataSetPermissions,
    UpdateDataSource,
    UpdateDataSourcePermissions,
    UpdateDefaultQBusinessApplication,
    UpdateEmailCustomizationTemplate,
    UpdateFolder,
    UpdateFolderPermissions,
    UpdateGroup,
    UpdateIamPolicyAssignment,
    UpdateIdentityPropagationConfig,
    UpdateIpRestriction,
    UpdateKeyRegistration,
    UpdatePublicSharingSettings,
    UpdateQPersonalizationConfiguration,
    UpdateQuickSightQSearchConfiguration,
    UpdateRefreshSchedule,
    UpdateResourcePermissions,
    UpdateRoleCustomPermission,
    UpdateSpiceCapacityConfiguration,
    UpdateTemplate,
    UpdateTemplateAlias,
    UpdateTemplatePermissions,
    UpdateTheme,
    UpdateThemeAlias,
    UpdateThemePermissions,
    UpdateTopic,
    UpdateTopicPermissions,
    UpdateTopicRefreshSchedule,
    UpdateUser,
    UpdateUserCustomPermission,
    UpdateVpcConnection,
}
impl std::fmt::Display for QuicksightActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QuicksightActions::AccountConfigurations => {
                write!(f, "quicksight:AccountConfigurations")
            }
            QuicksightActions::BatchCreateTopicReviewedAnswer => {
                write!(f, "quicksight:BatchCreateTopicReviewedAnswer")
            }
            QuicksightActions::BatchDeleteTopicReviewedAnswer => {
                write!(f, "quicksight:BatchDeleteTopicReviewedAnswer")
            }
            QuicksightActions::CancelIngestion => write!(f, "quicksight:CancelIngestion"),
            QuicksightActions::CreateAccountCustomization => {
                write!(f, "quicksight:CreateAccountCustomization")
            }
            QuicksightActions::CreateAccountSubscription => {
                write!(f, "quicksight:CreateAccountSubscription")
            }
            QuicksightActions::CreateAdmin => write!(f, "quicksight:CreateAdmin"),
            QuicksightActions::CreateAnalysis => write!(f, "quicksight:CreateAnalysis"),
            QuicksightActions::CreateBrand => write!(f, "quicksight:CreateBrand"),
            QuicksightActions::CreateCustomPermissions => {
                write!(f, "quicksight:CreateCustomPermissions")
            }
            QuicksightActions::CreateDashboard => write!(f, "quicksight:CreateDashboard"),
            QuicksightActions::CreateDataSet => write!(f, "quicksight:CreateDataSet"),
            QuicksightActions::CreateDataSource => write!(f, "quicksight:CreateDataSource"),
            QuicksightActions::CreateEmailCustomizationTemplate => {
                write!(f, "quicksight:CreateEmailCustomizationTemplate")
            }
            QuicksightActions::CreateFolder => write!(f, "quicksight:CreateFolder"),
            QuicksightActions::CreateFolderMembership => {
                write!(f, "quicksight:CreateFolderMembership")
            }
            QuicksightActions::CreateGroup => write!(f, "quicksight:CreateGroup"),
            QuicksightActions::CreateGroupMembership => {
                write!(f, "quicksight:CreateGroupMembership")
            }
            QuicksightActions::CreateIamPolicyAssignment => {
                write!(f, "quicksight:CreateIAMPolicyAssignment")
            }
            QuicksightActions::CreateIngestion => write!(f, "quicksight:CreateIngestion"),
            QuicksightActions::CreateNamespace => write!(f, "quicksight:CreateNamespace"),
            QuicksightActions::CreateReader => write!(f, "quicksight:CreateReader"),
            QuicksightActions::CreateRefreshSchedule => {
                write!(f, "quicksight:CreateRefreshSchedule")
            }
            QuicksightActions::CreateRoleMembership => write!(f, "quicksight:CreateRoleMembership"),
            QuicksightActions::CreateTemplate => write!(f, "quicksight:CreateTemplate"),
            QuicksightActions::CreateTemplateAlias => write!(f, "quicksight:CreateTemplateAlias"),
            QuicksightActions::CreateTheme => write!(f, "quicksight:CreateTheme"),
            QuicksightActions::CreateThemeAlias => write!(f, "quicksight:CreateThemeAlias"),
            QuicksightActions::CreateTopic => write!(f, "quicksight:CreateTopic"),
            QuicksightActions::CreateTopicRefreshSchedule => {
                write!(f, "quicksight:CreateTopicRefreshSchedule")
            }
            QuicksightActions::CreateUser => write!(f, "quicksight:CreateUser"),
            QuicksightActions::CreateVpcConnection => write!(f, "quicksight:CreateVPCConnection"),
            QuicksightActions::DeleteAccountCustomization => {
                write!(f, "quicksight:DeleteAccountCustomization")
            }
            QuicksightActions::DeleteAccountSubscription => {
                write!(f, "quicksight:DeleteAccountSubscription")
            }
            QuicksightActions::DeleteAnalysis => write!(f, "quicksight:DeleteAnalysis"),
            QuicksightActions::DeleteBrand => write!(f, "quicksight:DeleteBrand"),
            QuicksightActions::DeleteBrandAssignment => {
                write!(f, "quicksight:DeleteBrandAssignment")
            }
            QuicksightActions::DeleteCustomPermissions => {
                write!(f, "quicksight:DeleteCustomPermissions")
            }
            QuicksightActions::DeleteDashboard => write!(f, "quicksight:DeleteDashboard"),
            QuicksightActions::DeleteDataSet => write!(f, "quicksight:DeleteDataSet"),
            QuicksightActions::DeleteDataSetRefreshProperties => {
                write!(f, "quicksight:DeleteDataSetRefreshProperties")
            }
            QuicksightActions::DeleteDataSource => write!(f, "quicksight:DeleteDataSource"),
            QuicksightActions::DeleteDefaultQBusinessApplication => {
                write!(f, "quicksight:DeleteDefaultQBusinessApplication")
            }
            QuicksightActions::DeleteEmailCustomizationTemplate => {
                write!(f, "quicksight:DeleteEmailCustomizationTemplate")
            }
            QuicksightActions::DeleteFolder => write!(f, "quicksight:DeleteFolder"),
            QuicksightActions::DeleteFolderMembership => {
                write!(f, "quicksight:DeleteFolderMembership")
            }
            QuicksightActions::DeleteGroup => write!(f, "quicksight:DeleteGroup"),
            QuicksightActions::DeleteGroupMembership => {
                write!(f, "quicksight:DeleteGroupMembership")
            }
            QuicksightActions::DeleteIamPolicyAssignment => {
                write!(f, "quicksight:DeleteIAMPolicyAssignment")
            }
            QuicksightActions::DeleteIdentityPropagationConfig => {
                write!(f, "quicksight:DeleteIdentityPropagationConfig")
            }
            QuicksightActions::DeleteNamespace => write!(f, "quicksight:DeleteNamespace"),
            QuicksightActions::DeleteRefreshSchedule => {
                write!(f, "quicksight:DeleteRefreshSchedule")
            }
            QuicksightActions::DeleteRoleCustomPermission => {
                write!(f, "quicksight:DeleteRoleCustomPermission")
            }
            QuicksightActions::DeleteRoleMembership => write!(f, "quicksight:DeleteRoleMembership"),
            QuicksightActions::DeleteTemplate => write!(f, "quicksight:DeleteTemplate"),
            QuicksightActions::DeleteTemplateAlias => write!(f, "quicksight:DeleteTemplateAlias"),
            QuicksightActions::DeleteTheme => write!(f, "quicksight:DeleteTheme"),
            QuicksightActions::DeleteThemeAlias => write!(f, "quicksight:DeleteThemeAlias"),
            QuicksightActions::DeleteTopic => write!(f, "quicksight:DeleteTopic"),
            QuicksightActions::DeleteTopicRefreshSchedule => {
                write!(f, "quicksight:DeleteTopicRefreshSchedule")
            }
            QuicksightActions::DeleteUser => write!(f, "quicksight:DeleteUser"),
            QuicksightActions::DeleteUserByPrincipalId => {
                write!(f, "quicksight:DeleteUserByPrincipalId")
            }
            QuicksightActions::DeleteUserCustomPermission => {
                write!(f, "quicksight:DeleteUserCustomPermission")
            }
            QuicksightActions::DeleteVpcConnection => write!(f, "quicksight:DeleteVPCConnection"),
            QuicksightActions::DescribeAccountCustomization => {
                write!(f, "quicksight:DescribeAccountCustomization")
            }
            QuicksightActions::DescribeAccountSettings => {
                write!(f, "quicksight:DescribeAccountSettings")
            }
            QuicksightActions::DescribeAccountSubscription => {
                write!(f, "quicksight:DescribeAccountSubscription")
            }
            QuicksightActions::DescribeAnalysis => write!(f, "quicksight:DescribeAnalysis"),
            QuicksightActions::DescribeAnalysisPermissions => {
                write!(f, "quicksight:DescribeAnalysisPermissions")
            }
            QuicksightActions::DescribeAssetBundleExportJob => {
                write!(f, "quicksight:DescribeAssetBundleExportJob")
            }
            QuicksightActions::DescribeAssetBundleImportJob => {
                write!(f, "quicksight:DescribeAssetBundleImportJob")
            }
            QuicksightActions::DescribeBrand => write!(f, "quicksight:DescribeBrand"),
            QuicksightActions::DescribeBrandAssignment => {
                write!(f, "quicksight:DescribeBrandAssignment")
            }
            QuicksightActions::DescribeBrandPublishedVersion => {
                write!(f, "quicksight:DescribeBrandPublishedVersion")
            }
            QuicksightActions::DescribeCustomPermissions => {
                write!(f, "quicksight:DescribeCustomPermissions")
            }
            QuicksightActions::DescribeDashboard => write!(f, "quicksight:DescribeDashboard"),
            QuicksightActions::DescribeDashboardPermissions => {
                write!(f, "quicksight:DescribeDashboardPermissions")
            }
            QuicksightActions::DescribeDashboardSnapshotJob => {
                write!(f, "quicksight:DescribeDashboardSnapshotJob")
            }
            QuicksightActions::DescribeDashboardSnapshotJobResult => {
                write!(f, "quicksight:DescribeDashboardSnapshotJobResult")
            }
            QuicksightActions::DescribeDashboardsQaConfiguration => {
                write!(f, "quicksight:DescribeDashboardsQAConfiguration")
            }
            QuicksightActions::DescribeDataSet => write!(f, "quicksight:DescribeDataSet"),
            QuicksightActions::DescribeDataSetPermissions => {
                write!(f, "quicksight:DescribeDataSetPermissions")
            }
            QuicksightActions::DescribeDataSetRefreshProperties => {
                write!(f, "quicksight:DescribeDataSetRefreshProperties")
            }
            QuicksightActions::DescribeDataSource => write!(f, "quicksight:DescribeDataSource"),
            QuicksightActions::DescribeDataSourcePermissions => {
                write!(f, "quicksight:DescribeDataSourcePermissions")
            }
            QuicksightActions::DescribeDefaultQBusinessApplication => {
                write!(f, "quicksight:DescribeDefaultQBusinessApplication")
            }
            QuicksightActions::DescribeEmailCustomizationTemplate => {
                write!(f, "quicksight:DescribeEmailCustomizationTemplate")
            }
            QuicksightActions::DescribeFolder => write!(f, "quicksight:DescribeFolder"),
            QuicksightActions::DescribeFolderPermissions => {
                write!(f, "quicksight:DescribeFolderPermissions")
            }
            QuicksightActions::DescribeFolderResolvedPermissions => {
                write!(f, "quicksight:DescribeFolderResolvedPermissions")
            }
            QuicksightActions::DescribeGroup => write!(f, "quicksight:DescribeGroup"),
            QuicksightActions::DescribeGroupMembership => {
                write!(f, "quicksight:DescribeGroupMembership")
            }
            QuicksightActions::DescribeIamPolicyAssignment => {
                write!(f, "quicksight:DescribeIAMPolicyAssignment")
            }
            QuicksightActions::DescribeIngestion => write!(f, "quicksight:DescribeIngestion"),
            QuicksightActions::DescribeIpRestriction => {
                write!(f, "quicksight:DescribeIpRestriction")
            }
            QuicksightActions::DescribeKeyRegistration => {
                write!(f, "quicksight:DescribeKeyRegistration")
            }
            QuicksightActions::DescribeNamespace => write!(f, "quicksight:DescribeNamespace"),
            QuicksightActions::DescribeQPersonalizationConfiguration => {
                write!(f, "quicksight:DescribeQPersonalizationConfiguration")
            }
            QuicksightActions::DescribeQuickSightQSearchConfiguration => {
                write!(f, "quicksight:DescribeQuickSightQSearchConfiguration")
            }
            QuicksightActions::DescribeRefreshSchedule => {
                write!(f, "quicksight:DescribeRefreshSchedule")
            }
            QuicksightActions::DescribeRoleCustomPermission => {
                write!(f, "quicksight:DescribeRoleCustomPermission")
            }
            QuicksightActions::DescribeTemplate => write!(f, "quicksight:DescribeTemplate"),
            QuicksightActions::DescribeTemplateAlias => {
                write!(f, "quicksight:DescribeTemplateAlias")
            }
            QuicksightActions::DescribeTemplatePermissions => {
                write!(f, "quicksight:DescribeTemplatePermissions")
            }
            QuicksightActions::DescribeTheme => write!(f, "quicksight:DescribeTheme"),
            QuicksightActions::DescribeThemeAlias => write!(f, "quicksight:DescribeThemeAlias"),
            QuicksightActions::DescribeThemePermissions => {
                write!(f, "quicksight:DescribeThemePermissions")
            }
            QuicksightActions::DescribeTopic => write!(f, "quicksight:DescribeTopic"),
            QuicksightActions::DescribeTopicPermissions => {
                write!(f, "quicksight:DescribeTopicPermissions")
            }
            QuicksightActions::DescribeTopicRefresh => write!(f, "quicksight:DescribeTopicRefresh"),
            QuicksightActions::DescribeTopicRefreshSchedule => {
                write!(f, "quicksight:DescribeTopicRefreshSchedule")
            }
            QuicksightActions::DescribeUser => write!(f, "quicksight:DescribeUser"),
            QuicksightActions::DescribeVpcConnection => {
                write!(f, "quicksight:DescribeVPCConnection")
            }
            QuicksightActions::GenerateEmbedUrlForAnonymousUser => {
                write!(f, "quicksight:GenerateEmbedUrlForAnonymousUser")
            }
            QuicksightActions::GenerateEmbedUrlForRegisteredUser => {
                write!(f, "quicksight:GenerateEmbedUrlForRegisteredUser")
            }
            QuicksightActions::GenerateEmbedUrlForRegisteredUserWithIdentity => write!(
                f,
                "quicksight:GenerateEmbedUrlForRegisteredUserWithIdentity"
            ),
            QuicksightActions::GetAnonymousUserEmbedUrl => {
                write!(f, "quicksight:GetAnonymousUserEmbedUrl")
            }
            QuicksightActions::GetAuthCode => write!(f, "quicksight:GetAuthCode"),
            QuicksightActions::GetDashboardEmbedUrl => write!(f, "quicksight:GetDashboardEmbedUrl"),
            QuicksightActions::GetGroupMapping => write!(f, "quicksight:GetGroupMapping"),
            QuicksightActions::GetSessionEmbedUrl => write!(f, "quicksight:GetSessionEmbedUrl"),
            QuicksightActions::ListAnalyses => write!(f, "quicksight:ListAnalyses"),
            QuicksightActions::ListAssetBundleExportJobs => {
                write!(f, "quicksight:ListAssetBundleExportJobs")
            }
            QuicksightActions::ListAssetBundleImportJobs => {
                write!(f, "quicksight:ListAssetBundleImportJobs")
            }
            QuicksightActions::ListBrands => write!(f, "quicksight:ListBrands"),
            QuicksightActions::ListCustomPermissions => {
                write!(f, "quicksight:ListCustomPermissions")
            }
            QuicksightActions::ListCustomerManagedKeys => {
                write!(f, "quicksight:ListCustomerManagedKeys")
            }
            QuicksightActions::ListDashboardVersions => {
                write!(f, "quicksight:ListDashboardVersions")
            }
            QuicksightActions::ListDashboards => write!(f, "quicksight:ListDashboards"),
            QuicksightActions::ListDataSets => write!(f, "quicksight:ListDataSets"),
            QuicksightActions::ListDataSources => write!(f, "quicksight:ListDataSources"),
            QuicksightActions::ListFolderMembers => write!(f, "quicksight:ListFolderMembers"),
            QuicksightActions::ListFolders => write!(f, "quicksight:ListFolders"),
            QuicksightActions::ListFoldersForResource => {
                write!(f, "quicksight:ListFoldersForResource")
            }
            QuicksightActions::ListGroupMemberships => write!(f, "quicksight:ListGroupMemberships"),
            QuicksightActions::ListGroups => write!(f, "quicksight:ListGroups"),
            QuicksightActions::ListIamPolicyAssignments => {
                write!(f, "quicksight:ListIAMPolicyAssignments")
            }
            QuicksightActions::ListIamPolicyAssignmentsForUser => {
                write!(f, "quicksight:ListIAMPolicyAssignmentsForUser")
            }
            QuicksightActions::ListIdentityPropagationConfigs => {
                write!(f, "quicksight:ListIdentityPropagationConfigs")
            }
            QuicksightActions::ListIngestions => write!(f, "quicksight:ListIngestions"),
            QuicksightActions::ListKmsKeysForUser => write!(f, "quicksight:ListKMSKeysForUser"),
            QuicksightActions::ListNamespaces => write!(f, "quicksight:ListNamespaces"),
            QuicksightActions::ListRefreshSchedules => write!(f, "quicksight:ListRefreshSchedules"),
            QuicksightActions::ListRoleMemberships => write!(f, "quicksight:ListRoleMemberships"),
            QuicksightActions::ListTagsForResource => write!(f, "quicksight:ListTagsForResource"),
            QuicksightActions::ListTemplateAliases => write!(f, "quicksight:ListTemplateAliases"),
            QuicksightActions::ListTemplateVersions => write!(f, "quicksight:ListTemplateVersions"),
            QuicksightActions::ListTemplates => write!(f, "quicksight:ListTemplates"),
            QuicksightActions::ListThemeAliases => write!(f, "quicksight:ListThemeAliases"),
            QuicksightActions::ListThemeVersions => write!(f, "quicksight:ListThemeVersions"),
            QuicksightActions::ListThemes => write!(f, "quicksight:ListThemes"),
            QuicksightActions::ListTopicRefreshSchedules => {
                write!(f, "quicksight:ListTopicRefreshSchedules")
            }
            QuicksightActions::ListTopicReviewedAnswers => {
                write!(f, "quicksight:ListTopicReviewedAnswers")
            }
            QuicksightActions::ListTopics => write!(f, "quicksight:ListTopics"),
            QuicksightActions::ListUserGroups => write!(f, "quicksight:ListUserGroups"),
            QuicksightActions::ListUsers => write!(f, "quicksight:ListUsers"),
            QuicksightActions::ListVpcConnections => write!(f, "quicksight:ListVPCConnections"),
            QuicksightActions::PassDataSet => write!(f, "quicksight:PassDataSet"),
            QuicksightActions::PassDataSource => write!(f, "quicksight:PassDataSource"),
            QuicksightActions::PredictQaResults => write!(f, "quicksight:PredictQAResults"),
            QuicksightActions::PutDataSetRefreshProperties => {
                write!(f, "quicksight:PutDataSetRefreshProperties")
            }
            QuicksightActions::RegisterCustomerManagedKey => {
                write!(f, "quicksight:RegisterCustomerManagedKey")
            }
            QuicksightActions::RegisterUser => write!(f, "quicksight:RegisterUser"),
            QuicksightActions::RemoveCustomerManagedKey => {
                write!(f, "quicksight:RemoveCustomerManagedKey")
            }
            QuicksightActions::RestoreAnalysis => write!(f, "quicksight:RestoreAnalysis"),
            QuicksightActions::ScopeDownPolicy => write!(f, "quicksight:ScopeDownPolicy"),
            QuicksightActions::SearchAnalyses => write!(f, "quicksight:SearchAnalyses"),
            QuicksightActions::SearchDashboards => write!(f, "quicksight:SearchDashboards"),
            QuicksightActions::SearchDataSets => write!(f, "quicksight:SearchDataSets"),
            QuicksightActions::SearchDataSources => write!(f, "quicksight:SearchDataSources"),
            QuicksightActions::SearchDirectoryGroups => {
                write!(f, "quicksight:SearchDirectoryGroups")
            }
            QuicksightActions::SearchFolders => write!(f, "quicksight:SearchFolders"),
            QuicksightActions::SearchGroups => write!(f, "quicksight:SearchGroups"),
            QuicksightActions::SearchTopics => write!(f, "quicksight:SearchTopics"),
            QuicksightActions::SearchUsers => write!(f, "quicksight:SearchUsers"),
            QuicksightActions::SetGroupMapping => write!(f, "quicksight:SetGroupMapping"),
            QuicksightActions::StartAssetBundleExportJob => {
                write!(f, "quicksight:StartAssetBundleExportJob")
            }
            QuicksightActions::StartAssetBundleImportJob => {
                write!(f, "quicksight:StartAssetBundleImportJob")
            }
            QuicksightActions::StartDashboardSnapshotJob => {
                write!(f, "quicksight:StartDashboardSnapshotJob")
            }
            QuicksightActions::StartDashboardSnapshotJobSchedule => {
                write!(f, "quicksight:StartDashboardSnapshotJobSchedule")
            }
            QuicksightActions::Subscribe => write!(f, "quicksight:Subscribe"),
            QuicksightActions::TagResource => write!(f, "quicksight:TagResource"),
            QuicksightActions::Unsubscribe => write!(f, "quicksight:Unsubscribe"),
            QuicksightActions::UntagResource => write!(f, "quicksight:UntagResource"),
            QuicksightActions::UpdateAccountCustomization => {
                write!(f, "quicksight:UpdateAccountCustomization")
            }
            QuicksightActions::UpdateAccountSettings => {
                write!(f, "quicksight:UpdateAccountSettings")
            }
            QuicksightActions::UpdateAnalysis => write!(f, "quicksight:UpdateAnalysis"),
            QuicksightActions::UpdateAnalysisPermissions => {
                write!(f, "quicksight:UpdateAnalysisPermissions")
            }
            QuicksightActions::UpdateApplicationWithTokenExchangeGrant => {
                write!(f, "quicksight:UpdateApplicationWithTokenExchangeGrant")
            }
            QuicksightActions::UpdateBrand => write!(f, "quicksight:UpdateBrand"),
            QuicksightActions::UpdateBrandAssignment => {
                write!(f, "quicksight:UpdateBrandAssignment")
            }
            QuicksightActions::UpdateBrandPublishedVersion => {
                write!(f, "quicksight:UpdateBrandPublishedVersion")
            }
            QuicksightActions::UpdateCustomPermissions => {
                write!(f, "quicksight:UpdateCustomPermissions")
            }
            QuicksightActions::UpdateDashboard => write!(f, "quicksight:UpdateDashboard"),
            QuicksightActions::UpdateDashboardLinks => write!(f, "quicksight:UpdateDashboardLinks"),
            QuicksightActions::UpdateDashboardPermissions => {
                write!(f, "quicksight:UpdateDashboardPermissions")
            }
            QuicksightActions::UpdateDashboardPublishedVersion => {
                write!(f, "quicksight:UpdateDashboardPublishedVersion")
            }
            QuicksightActions::UpdateDashboardsQaConfiguration => {
                write!(f, "quicksight:UpdateDashboardsQAConfiguration")
            }
            QuicksightActions::UpdateDataSet => write!(f, "quicksight:UpdateDataSet"),
            QuicksightActions::UpdateDataSetPermissions => {
                write!(f, "quicksight:UpdateDataSetPermissions")
            }
            QuicksightActions::UpdateDataSource => write!(f, "quicksight:UpdateDataSource"),
            QuicksightActions::UpdateDataSourcePermissions => {
                write!(f, "quicksight:UpdateDataSourcePermissions")
            }
            QuicksightActions::UpdateDefaultQBusinessApplication => {
                write!(f, "quicksight:UpdateDefaultQBusinessApplication")
            }
            QuicksightActions::UpdateEmailCustomizationTemplate => {
                write!(f, "quicksight:UpdateEmailCustomizationTemplate")
            }
            QuicksightActions::UpdateFolder => write!(f, "quicksight:UpdateFolder"),
            QuicksightActions::UpdateFolderPermissions => {
                write!(f, "quicksight:UpdateFolderPermissions")
            }
            QuicksightActions::UpdateGroup => write!(f, "quicksight:UpdateGroup"),
            QuicksightActions::UpdateIamPolicyAssignment => {
                write!(f, "quicksight:UpdateIAMPolicyAssignment")
            }
            QuicksightActions::UpdateIdentityPropagationConfig => {
                write!(f, "quicksight:UpdateIdentityPropagationConfig")
            }
            QuicksightActions::UpdateIpRestriction => write!(f, "quicksight:UpdateIpRestriction"),
            QuicksightActions::UpdateKeyRegistration => {
                write!(f, "quicksight:UpdateKeyRegistration")
            }
            QuicksightActions::UpdatePublicSharingSettings => {
                write!(f, "quicksight:UpdatePublicSharingSettings")
            }
            QuicksightActions::UpdateQPersonalizationConfiguration => {
                write!(f, "quicksight:UpdateQPersonalizationConfiguration")
            }
            QuicksightActions::UpdateQuickSightQSearchConfiguration => {
                write!(f, "quicksight:UpdateQuickSightQSearchConfiguration")
            }
            QuicksightActions::UpdateRefreshSchedule => {
                write!(f, "quicksight:UpdateRefreshSchedule")
            }
            QuicksightActions::UpdateResourcePermissions => {
                write!(f, "quicksight:UpdateResourcePermissions")
            }
            QuicksightActions::UpdateRoleCustomPermission => {
                write!(f, "quicksight:UpdateRoleCustomPermission")
            }
            QuicksightActions::UpdateSpiceCapacityConfiguration => {
                write!(f, "quicksight:UpdateSPICECapacityConfiguration")
            }
            QuicksightActions::UpdateTemplate => write!(f, "quicksight:UpdateTemplate"),
            QuicksightActions::UpdateTemplateAlias => write!(f, "quicksight:UpdateTemplateAlias"),
            QuicksightActions::UpdateTemplatePermissions => {
                write!(f, "quicksight:UpdateTemplatePermissions")
            }
            QuicksightActions::UpdateTheme => write!(f, "quicksight:UpdateTheme"),
            QuicksightActions::UpdateThemeAlias => write!(f, "quicksight:UpdateThemeAlias"),
            QuicksightActions::UpdateThemePermissions => {
                write!(f, "quicksight:UpdateThemePermissions")
            }
            QuicksightActions::UpdateTopic => write!(f, "quicksight:UpdateTopic"),
            QuicksightActions::UpdateTopicPermissions => {
                write!(f, "quicksight:UpdateTopicPermissions")
            }
            QuicksightActions::UpdateTopicRefreshSchedule => {
                write!(f, "quicksight:UpdateTopicRefreshSchedule")
            }
            QuicksightActions::UpdateUser => write!(f, "quicksight:UpdateUser"),
            QuicksightActions::UpdateUserCustomPermission => {
                write!(f, "quicksight:UpdateUserCustomPermission")
            }
            QuicksightActions::UpdateVpcConnection => write!(f, "quicksight:UpdateVPCConnection"),
        }
    }
}
