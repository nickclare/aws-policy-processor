// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum ComputeOptimizerActions {
    DeleteRecommendationPreferences,
    DescribeRecommendationExportJobs,
    ExportAutoScalingGroupRecommendations,
    ExportEbsVolumeRecommendations,
    ExportEc2InstanceRecommendations,
    ExportEcsServiceRecommendations,
    ExportIdleRecommendations,
    ExportLambdaFunctionRecommendations,
    ExportLicenseRecommendations,
    ExportRdsDatabaseRecommendations,
    GetAutoScalingGroupRecommendations,
    GetEbsVolumeRecommendations,
    GetEc2InstanceRecommendations,
    GetEc2RecommendationProjectedMetrics,
    GetEcsServiceRecommendationProjectedMetrics,
    GetEcsServiceRecommendations,
    GetEffectiveRecommendationPreferences,
    GetEnrollmentStatus,
    GetEnrollmentStatusesForOrganization,
    GetIdleRecommendations,
    GetLambdaFunctionRecommendations,
    GetLicenseRecommendations,
    GetRdsDatabaseRecommendationProjectedMetrics,
    GetRdsDatabaseRecommendations,
    GetRecommendationPreferences,
    GetRecommendationSummaries,
    PutRecommendationPreferences,
    UpdateEnrollmentStatus,
}
impl std::fmt::Display for ComputeOptimizerActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ComputeOptimizerActions::DeleteRecommendationPreferences => {
                write!(f, "compute-optimizer:DeleteRecommendationPreferences")
            }
            ComputeOptimizerActions::DescribeRecommendationExportJobs => {
                write!(f, "compute-optimizer:DescribeRecommendationExportJobs")
            }
            ComputeOptimizerActions::ExportAutoScalingGroupRecommendations => {
                write!(f, "compute-optimizer:ExportAutoScalingGroupRecommendations")
            }
            ComputeOptimizerActions::ExportEbsVolumeRecommendations => {
                write!(f, "compute-optimizer:ExportEBSVolumeRecommendations")
            }
            ComputeOptimizerActions::ExportEc2InstanceRecommendations => {
                write!(f, "compute-optimizer:ExportEC2InstanceRecommendations")
            }
            ComputeOptimizerActions::ExportEcsServiceRecommendations => {
                write!(f, "compute-optimizer:ExportECSServiceRecommendations")
            }
            ComputeOptimizerActions::ExportIdleRecommendations => {
                write!(f, "compute-optimizer:ExportIdleRecommendations")
            }
            ComputeOptimizerActions::ExportLambdaFunctionRecommendations => {
                write!(f, "compute-optimizer:ExportLambdaFunctionRecommendations")
            }
            ComputeOptimizerActions::ExportLicenseRecommendations => {
                write!(f, "compute-optimizer:ExportLicenseRecommendations")
            }
            ComputeOptimizerActions::ExportRdsDatabaseRecommendations => {
                write!(f, "compute-optimizer:ExportRDSDatabaseRecommendations")
            }
            ComputeOptimizerActions::GetAutoScalingGroupRecommendations => {
                write!(f, "compute-optimizer:GetAutoScalingGroupRecommendations")
            }
            ComputeOptimizerActions::GetEbsVolumeRecommendations => {
                write!(f, "compute-optimizer:GetEBSVolumeRecommendations")
            }
            ComputeOptimizerActions::GetEc2InstanceRecommendations => {
                write!(f, "compute-optimizer:GetEC2InstanceRecommendations")
            }
            ComputeOptimizerActions::GetEc2RecommendationProjectedMetrics => {
                write!(f, "compute-optimizer:GetEC2RecommendationProjectedMetrics")
            }
            ComputeOptimizerActions::GetEcsServiceRecommendationProjectedMetrics => write!(
                f,
                "compute-optimizer:GetECSServiceRecommendationProjectedMetrics"
            ),
            ComputeOptimizerActions::GetEcsServiceRecommendations => {
                write!(f, "compute-optimizer:GetECSServiceRecommendations")
            }
            ComputeOptimizerActions::GetEffectiveRecommendationPreferences => {
                write!(f, "compute-optimizer:GetEffectiveRecommendationPreferences")
            }
            ComputeOptimizerActions::GetEnrollmentStatus => {
                write!(f, "compute-optimizer:GetEnrollmentStatus")
            }
            ComputeOptimizerActions::GetEnrollmentStatusesForOrganization => {
                write!(f, "compute-optimizer:GetEnrollmentStatusesForOrganization")
            }
            ComputeOptimizerActions::GetIdleRecommendations => {
                write!(f, "compute-optimizer:GetIdleRecommendations")
            }
            ComputeOptimizerActions::GetLambdaFunctionRecommendations => {
                write!(f, "compute-optimizer:GetLambdaFunctionRecommendations")
            }
            ComputeOptimizerActions::GetLicenseRecommendations => {
                write!(f, "compute-optimizer:GetLicenseRecommendations")
            }
            ComputeOptimizerActions::GetRdsDatabaseRecommendationProjectedMetrics => write!(
                f,
                "compute-optimizer:GetRDSDatabaseRecommendationProjectedMetrics"
            ),
            ComputeOptimizerActions::GetRdsDatabaseRecommendations => {
                write!(f, "compute-optimizer:GetRDSDatabaseRecommendations")
            }
            ComputeOptimizerActions::GetRecommendationPreferences => {
                write!(f, "compute-optimizer:GetRecommendationPreferences")
            }
            ComputeOptimizerActions::GetRecommendationSummaries => {
                write!(f, "compute-optimizer:GetRecommendationSummaries")
            }
            ComputeOptimizerActions::PutRecommendationPreferences => {
                write!(f, "compute-optimizer:PutRecommendationPreferences")
            }
            ComputeOptimizerActions::UpdateEnrollmentStatus => {
                write!(f, "compute-optimizer:UpdateEnrollmentStatus")
            }
        }
    }
}
