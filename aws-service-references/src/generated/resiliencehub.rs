// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum ResiliencehubActions {
    AcceptResourceGroupingRecommendations,
    AddDraftAppVersionResourceMappings,
    BatchUpdateRecommendationStatus,
    CreateApp,
    CreateAppVersionAppComponent,
    CreateAppVersionResource,
    CreateRecommendationTemplate,
    CreateResiliencyPolicy,
    DeleteApp,
    DeleteAppAssessment,
    DeleteAppInputSource,
    DeleteAppVersionAppComponent,
    DeleteAppVersionResource,
    DeleteRecommendationTemplate,
    DeleteResiliencyPolicy,
    DescribeApp,
    DescribeAppAssessment,
    DescribeAppVersion,
    DescribeAppVersionAppComponent,
    DescribeAppVersionResource,
    DescribeAppVersionResourcesResolutionStatus,
    DescribeAppVersionTemplate,
    DescribeDraftAppVersionResourcesImportStatus,
    DescribeMetricsExport,
    DescribeResiliencyPolicy,
    DescribeResourceGroupingRecommendationTask,
    ImportResourcesToDraftAppVersion,
    ListAlarmRecommendations,
    ListAppAssessmentComplianceDrifts,
    ListAppAssessmentResourceDrifts,
    ListAppAssessments,
    ListAppComponentCompliances,
    ListAppComponentRecommendations,
    ListAppInputSources,
    ListAppVersionAppComponents,
    ListAppVersionResourceMappings,
    ListAppVersionResources,
    ListAppVersions,
    ListApps,
    ListMetrics,
    ListRecommendationTemplates,
    ListResiliencyPolicies,
    ListResourceGroupingRecommendations,
    ListSopRecommendations,
    ListSuggestedResiliencyPolicies,
    ListTagsForResource,
    ListTestRecommendations,
    ListUnsupportedAppVersionResources,
    PublishAppVersion,
    PutDraftAppVersionTemplate,
    RejectResourceGroupingRecommendations,
    RemoveDraftAppVersionResourceMappings,
    ResolveAppVersionResources,
    StartAppAssessment,
    StartMetricsExport,
    StartResourceGroupingRecommendationTask,
    TagResource,
    UntagResource,
    UpdateApp,
    UpdateAppVersion,
    UpdateAppVersionAppComponent,
    UpdateAppVersionResource,
    UpdateResiliencyPolicy,
}
impl std::fmt::Display for ResiliencehubActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResiliencehubActions::AcceptResourceGroupingRecommendations => {
                write!(f, "resiliencehub:AcceptResourceGroupingRecommendations")
            }
            ResiliencehubActions::AddDraftAppVersionResourceMappings => {
                write!(f, "resiliencehub:AddDraftAppVersionResourceMappings")
            }
            ResiliencehubActions::BatchUpdateRecommendationStatus => {
                write!(f, "resiliencehub:BatchUpdateRecommendationStatus")
            }
            ResiliencehubActions::CreateApp => write!(f, "resiliencehub:CreateApp"),
            ResiliencehubActions::CreateAppVersionAppComponent => {
                write!(f, "resiliencehub:CreateAppVersionAppComponent")
            }
            ResiliencehubActions::CreateAppVersionResource => {
                write!(f, "resiliencehub:CreateAppVersionResource")
            }
            ResiliencehubActions::CreateRecommendationTemplate => {
                write!(f, "resiliencehub:CreateRecommendationTemplate")
            }
            ResiliencehubActions::CreateResiliencyPolicy => {
                write!(f, "resiliencehub:CreateResiliencyPolicy")
            }
            ResiliencehubActions::DeleteApp => write!(f, "resiliencehub:DeleteApp"),
            ResiliencehubActions::DeleteAppAssessment => {
                write!(f, "resiliencehub:DeleteAppAssessment")
            }
            ResiliencehubActions::DeleteAppInputSource => {
                write!(f, "resiliencehub:DeleteAppInputSource")
            }
            ResiliencehubActions::DeleteAppVersionAppComponent => {
                write!(f, "resiliencehub:DeleteAppVersionAppComponent")
            }
            ResiliencehubActions::DeleteAppVersionResource => {
                write!(f, "resiliencehub:DeleteAppVersionResource")
            }
            ResiliencehubActions::DeleteRecommendationTemplate => {
                write!(f, "resiliencehub:DeleteRecommendationTemplate")
            }
            ResiliencehubActions::DeleteResiliencyPolicy => {
                write!(f, "resiliencehub:DeleteResiliencyPolicy")
            }
            ResiliencehubActions::DescribeApp => write!(f, "resiliencehub:DescribeApp"),
            ResiliencehubActions::DescribeAppAssessment => {
                write!(f, "resiliencehub:DescribeAppAssessment")
            }
            ResiliencehubActions::DescribeAppVersion => {
                write!(f, "resiliencehub:DescribeAppVersion")
            }
            ResiliencehubActions::DescribeAppVersionAppComponent => {
                write!(f, "resiliencehub:DescribeAppVersionAppComponent")
            }
            ResiliencehubActions::DescribeAppVersionResource => {
                write!(f, "resiliencehub:DescribeAppVersionResource")
            }
            ResiliencehubActions::DescribeAppVersionResourcesResolutionStatus => write!(
                f,
                "resiliencehub:DescribeAppVersionResourcesResolutionStatus"
            ),
            ResiliencehubActions::DescribeAppVersionTemplate => {
                write!(f, "resiliencehub:DescribeAppVersionTemplate")
            }
            ResiliencehubActions::DescribeDraftAppVersionResourcesImportStatus => write!(
                f,
                "resiliencehub:DescribeDraftAppVersionResourcesImportStatus"
            ),
            ResiliencehubActions::DescribeMetricsExport => {
                write!(f, "resiliencehub:DescribeMetricsExport")
            }
            ResiliencehubActions::DescribeResiliencyPolicy => {
                write!(f, "resiliencehub:DescribeResiliencyPolicy")
            }
            ResiliencehubActions::DescribeResourceGroupingRecommendationTask => write!(
                f,
                "resiliencehub:DescribeResourceGroupingRecommendationTask"
            ),
            ResiliencehubActions::ImportResourcesToDraftAppVersion => {
                write!(f, "resiliencehub:ImportResourcesToDraftAppVersion")
            }
            ResiliencehubActions::ListAlarmRecommendations => {
                write!(f, "resiliencehub:ListAlarmRecommendations")
            }
            ResiliencehubActions::ListAppAssessmentComplianceDrifts => {
                write!(f, "resiliencehub:ListAppAssessmentComplianceDrifts")
            }
            ResiliencehubActions::ListAppAssessmentResourceDrifts => {
                write!(f, "resiliencehub:ListAppAssessmentResourceDrifts")
            }
            ResiliencehubActions::ListAppAssessments => {
                write!(f, "resiliencehub:ListAppAssessments")
            }
            ResiliencehubActions::ListAppComponentCompliances => {
                write!(f, "resiliencehub:ListAppComponentCompliances")
            }
            ResiliencehubActions::ListAppComponentRecommendations => {
                write!(f, "resiliencehub:ListAppComponentRecommendations")
            }
            ResiliencehubActions::ListAppInputSources => {
                write!(f, "resiliencehub:ListAppInputSources")
            }
            ResiliencehubActions::ListAppVersionAppComponents => {
                write!(f, "resiliencehub:ListAppVersionAppComponents")
            }
            ResiliencehubActions::ListAppVersionResourceMappings => {
                write!(f, "resiliencehub:ListAppVersionResourceMappings")
            }
            ResiliencehubActions::ListAppVersionResources => {
                write!(f, "resiliencehub:ListAppVersionResources")
            }
            ResiliencehubActions::ListAppVersions => write!(f, "resiliencehub:ListAppVersions"),
            ResiliencehubActions::ListApps => write!(f, "resiliencehub:ListApps"),
            ResiliencehubActions::ListMetrics => write!(f, "resiliencehub:ListMetrics"),
            ResiliencehubActions::ListRecommendationTemplates => {
                write!(f, "resiliencehub:ListRecommendationTemplates")
            }
            ResiliencehubActions::ListResiliencyPolicies => {
                write!(f, "resiliencehub:ListResiliencyPolicies")
            }
            ResiliencehubActions::ListResourceGroupingRecommendations => {
                write!(f, "resiliencehub:ListResourceGroupingRecommendations")
            }
            ResiliencehubActions::ListSopRecommendations => {
                write!(f, "resiliencehub:ListSopRecommendations")
            }
            ResiliencehubActions::ListSuggestedResiliencyPolicies => {
                write!(f, "resiliencehub:ListSuggestedResiliencyPolicies")
            }
            ResiliencehubActions::ListTagsForResource => {
                write!(f, "resiliencehub:ListTagsForResource")
            }
            ResiliencehubActions::ListTestRecommendations => {
                write!(f, "resiliencehub:ListTestRecommendations")
            }
            ResiliencehubActions::ListUnsupportedAppVersionResources => {
                write!(f, "resiliencehub:ListUnsupportedAppVersionResources")
            }
            ResiliencehubActions::PublishAppVersion => write!(f, "resiliencehub:PublishAppVersion"),
            ResiliencehubActions::PutDraftAppVersionTemplate => {
                write!(f, "resiliencehub:PutDraftAppVersionTemplate")
            }
            ResiliencehubActions::RejectResourceGroupingRecommendations => {
                write!(f, "resiliencehub:RejectResourceGroupingRecommendations")
            }
            ResiliencehubActions::RemoveDraftAppVersionResourceMappings => {
                write!(f, "resiliencehub:RemoveDraftAppVersionResourceMappings")
            }
            ResiliencehubActions::ResolveAppVersionResources => {
                write!(f, "resiliencehub:ResolveAppVersionResources")
            }
            ResiliencehubActions::StartAppAssessment => {
                write!(f, "resiliencehub:StartAppAssessment")
            }
            ResiliencehubActions::StartMetricsExport => {
                write!(f, "resiliencehub:StartMetricsExport")
            }
            ResiliencehubActions::StartResourceGroupingRecommendationTask => {
                write!(f, "resiliencehub:StartResourceGroupingRecommendationTask")
            }
            ResiliencehubActions::TagResource => write!(f, "resiliencehub:TagResource"),
            ResiliencehubActions::UntagResource => write!(f, "resiliencehub:UntagResource"),
            ResiliencehubActions::UpdateApp => write!(f, "resiliencehub:UpdateApp"),
            ResiliencehubActions::UpdateAppVersion => write!(f, "resiliencehub:UpdateAppVersion"),
            ResiliencehubActions::UpdateAppVersionAppComponent => {
                write!(f, "resiliencehub:UpdateAppVersionAppComponent")
            }
            ResiliencehubActions::UpdateAppVersionResource => {
                write!(f, "resiliencehub:UpdateAppVersionResource")
            }
            ResiliencehubActions::UpdateResiliencyPolicy => {
                write!(f, "resiliencehub:UpdateResiliencyPolicy")
            }
        }
    }
}
