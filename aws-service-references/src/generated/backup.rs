// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum BackupActions {
    AssociateBackupVaultMpaApprovalTeam,
    CancelLegalHold,
    CopyFromBackupVault,
    CopyIntoBackupVault,
    CreateBackupPlan,
    CreateBackupSelection,
    CreateBackupVault,
    CreateFramework,
    CreateLegalHold,
    CreateLogicallyAirGappedBackupVault,
    CreateReportPlan,
    CreateRestoreAccessBackupVault,
    CreateRestoreTestingPlan,
    CreateRestoreTestingSelection,
    DeleteBackupPlan,
    DeleteBackupSelection,
    DeleteBackupVault,
    DeleteBackupVaultAccessPolicy,
    DeleteBackupVaultLockConfiguration,
    DeleteBackupVaultNotifications,
    DeleteBackupVaultSharingPolicy,
    DeleteFramework,
    DeleteRecoveryPoint,
    DeleteReportPlan,
    DeleteRestoreTestingPlan,
    DeleteRestoreTestingSelection,
    DescribeBackupJob,
    DescribeBackupVault,
    DescribeCopyJob,
    DescribeFramework,
    DescribeGlobalSettings,
    DescribeProtectedResource,
    DescribeRecoveryPoint,
    DescribeRegionSettings,
    DescribeReportJob,
    DescribeReportPlan,
    DescribeRestoreJob,
    DisassociateBackupVaultMpaApprovalTeam,
    DisassociateRecoveryPoint,
    DisassociateRecoveryPointFromParent,
    ExportBackupPlanTemplate,
    GetBackupPlan,
    GetBackupPlanFromJson,
    GetBackupPlanFromTemplate,
    GetBackupSelection,
    GetBackupVaultAccessPolicy,
    GetBackupVaultNotifications,
    GetBackupVaultSharingPolicy,
    GetLegalHold,
    GetRecoveryPointIndexDetails,
    GetRecoveryPointRestoreMetadata,
    GetRestoreJobMetadata,
    GetRestoreTestingInferredMetadata,
    GetRestoreTestingPlan,
    GetRestoreTestingSelection,
    GetSupportedResourceTypes,
    ListBackupJobSummaries,
    ListBackupJobs,
    ListBackupPlanTemplates,
    ListBackupPlanVersions,
    ListBackupPlans,
    ListBackupSelections,
    ListBackupVaults,
    ListCopyJobSummaries,
    ListCopyJobs,
    ListFrameworks,
    ListIndexedRecoveryPoints,
    ListIndexedRecoveryPointsForSearch,
    ListLegalHolds,
    ListProtectedResources,
    ListProtectedResourcesByBackupVault,
    ListRecoveryPointsByBackupVault,
    ListRecoveryPointsByLegalHold,
    ListRecoveryPointsByResource,
    ListReportJobs,
    ListReportPlans,
    ListRestoreAccessBackupVaults,
    ListRestoreJobSummaries,
    ListRestoreJobs,
    ListRestoreJobsByProtectedResource,
    ListRestoreTestingPlans,
    ListRestoreTestingSelections,
    ListTags,
    PutBackupVaultAccessPolicy,
    PutBackupVaultLockConfiguration,
    PutBackupVaultNotifications,
    PutBackupVaultSharingPolicy,
    PutRestoreValidationResult,
    RevokeRestoreAccessBackupVault,
    SearchRecoveryPoint,
    StartBackupJob,
    StartCopyJob,
    StartReportJob,
    StartRestoreJob,
    StopBackupJob,
    TagResource,
    UntagResource,
    UpdateBackupPlan,
    UpdateFramework,
    UpdateGlobalSettings,
    UpdateRecoveryPointIndexSettings,
    UpdateRecoveryPointLifecycle,
    UpdateRegionSettings,
    UpdateReportPlan,
    UpdateRestoreTestingPlan,
    UpdateRestoreTestingSelection,
}
impl std::fmt::Display for BackupActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BackupActions::AssociateBackupVaultMpaApprovalTeam => {
                write!(f, "backup:AssociateBackupVaultMpaApprovalTeam")
            }
            BackupActions::CancelLegalHold => write!(f, "backup:CancelLegalHold"),
            BackupActions::CopyFromBackupVault => write!(f, "backup:CopyFromBackupVault"),
            BackupActions::CopyIntoBackupVault => write!(f, "backup:CopyIntoBackupVault"),
            BackupActions::CreateBackupPlan => write!(f, "backup:CreateBackupPlan"),
            BackupActions::CreateBackupSelection => write!(f, "backup:CreateBackupSelection"),
            BackupActions::CreateBackupVault => write!(f, "backup:CreateBackupVault"),
            BackupActions::CreateFramework => write!(f, "backup:CreateFramework"),
            BackupActions::CreateLegalHold => write!(f, "backup:CreateLegalHold"),
            BackupActions::CreateLogicallyAirGappedBackupVault => {
                write!(f, "backup:CreateLogicallyAirGappedBackupVault")
            }
            BackupActions::CreateReportPlan => write!(f, "backup:CreateReportPlan"),
            BackupActions::CreateRestoreAccessBackupVault => {
                write!(f, "backup:CreateRestoreAccessBackupVault")
            }
            BackupActions::CreateRestoreTestingPlan => write!(f, "backup:CreateRestoreTestingPlan"),
            BackupActions::CreateRestoreTestingSelection => {
                write!(f, "backup:CreateRestoreTestingSelection")
            }
            BackupActions::DeleteBackupPlan => write!(f, "backup:DeleteBackupPlan"),
            BackupActions::DeleteBackupSelection => write!(f, "backup:DeleteBackupSelection"),
            BackupActions::DeleteBackupVault => write!(f, "backup:DeleteBackupVault"),
            BackupActions::DeleteBackupVaultAccessPolicy => {
                write!(f, "backup:DeleteBackupVaultAccessPolicy")
            }
            BackupActions::DeleteBackupVaultLockConfiguration => {
                write!(f, "backup:DeleteBackupVaultLockConfiguration")
            }
            BackupActions::DeleteBackupVaultNotifications => {
                write!(f, "backup:DeleteBackupVaultNotifications")
            }
            BackupActions::DeleteBackupVaultSharingPolicy => {
                write!(f, "backup:DeleteBackupVaultSharingPolicy")
            }
            BackupActions::DeleteFramework => write!(f, "backup:DeleteFramework"),
            BackupActions::DeleteRecoveryPoint => write!(f, "backup:DeleteRecoveryPoint"),
            BackupActions::DeleteReportPlan => write!(f, "backup:DeleteReportPlan"),
            BackupActions::DeleteRestoreTestingPlan => write!(f, "backup:DeleteRestoreTestingPlan"),
            BackupActions::DeleteRestoreTestingSelection => {
                write!(f, "backup:DeleteRestoreTestingSelection")
            }
            BackupActions::DescribeBackupJob => write!(f, "backup:DescribeBackupJob"),
            BackupActions::DescribeBackupVault => write!(f, "backup:DescribeBackupVault"),
            BackupActions::DescribeCopyJob => write!(f, "backup:DescribeCopyJob"),
            BackupActions::DescribeFramework => write!(f, "backup:DescribeFramework"),
            BackupActions::DescribeGlobalSettings => write!(f, "backup:DescribeGlobalSettings"),
            BackupActions::DescribeProtectedResource => {
                write!(f, "backup:DescribeProtectedResource")
            }
            BackupActions::DescribeRecoveryPoint => write!(f, "backup:DescribeRecoveryPoint"),
            BackupActions::DescribeRegionSettings => write!(f, "backup:DescribeRegionSettings"),
            BackupActions::DescribeReportJob => write!(f, "backup:DescribeReportJob"),
            BackupActions::DescribeReportPlan => write!(f, "backup:DescribeReportPlan"),
            BackupActions::DescribeRestoreJob => write!(f, "backup:DescribeRestoreJob"),
            BackupActions::DisassociateBackupVaultMpaApprovalTeam => {
                write!(f, "backup:DisassociateBackupVaultMpaApprovalTeam")
            }
            BackupActions::DisassociateRecoveryPoint => {
                write!(f, "backup:DisassociateRecoveryPoint")
            }
            BackupActions::DisassociateRecoveryPointFromParent => {
                write!(f, "backup:DisassociateRecoveryPointFromParent")
            }
            BackupActions::ExportBackupPlanTemplate => write!(f, "backup:ExportBackupPlanTemplate"),
            BackupActions::GetBackupPlan => write!(f, "backup:GetBackupPlan"),
            BackupActions::GetBackupPlanFromJson => write!(f, "backup:GetBackupPlanFromJSON"),
            BackupActions::GetBackupPlanFromTemplate => {
                write!(f, "backup:GetBackupPlanFromTemplate")
            }
            BackupActions::GetBackupSelection => write!(f, "backup:GetBackupSelection"),
            BackupActions::GetBackupVaultAccessPolicy => {
                write!(f, "backup:GetBackupVaultAccessPolicy")
            }
            BackupActions::GetBackupVaultNotifications => {
                write!(f, "backup:GetBackupVaultNotifications")
            }
            BackupActions::GetBackupVaultSharingPolicy => {
                write!(f, "backup:GetBackupVaultSharingPolicy")
            }
            BackupActions::GetLegalHold => write!(f, "backup:GetLegalHold"),
            BackupActions::GetRecoveryPointIndexDetails => {
                write!(f, "backup:GetRecoveryPointIndexDetails")
            }
            BackupActions::GetRecoveryPointRestoreMetadata => {
                write!(f, "backup:GetRecoveryPointRestoreMetadata")
            }
            BackupActions::GetRestoreJobMetadata => write!(f, "backup:GetRestoreJobMetadata"),
            BackupActions::GetRestoreTestingInferredMetadata => {
                write!(f, "backup:GetRestoreTestingInferredMetadata")
            }
            BackupActions::GetRestoreTestingPlan => write!(f, "backup:GetRestoreTestingPlan"),
            BackupActions::GetRestoreTestingSelection => {
                write!(f, "backup:GetRestoreTestingSelection")
            }
            BackupActions::GetSupportedResourceTypes => {
                write!(f, "backup:GetSupportedResourceTypes")
            }
            BackupActions::ListBackupJobSummaries => write!(f, "backup:ListBackupJobSummaries"),
            BackupActions::ListBackupJobs => write!(f, "backup:ListBackupJobs"),
            BackupActions::ListBackupPlanTemplates => write!(f, "backup:ListBackupPlanTemplates"),
            BackupActions::ListBackupPlanVersions => write!(f, "backup:ListBackupPlanVersions"),
            BackupActions::ListBackupPlans => write!(f, "backup:ListBackupPlans"),
            BackupActions::ListBackupSelections => write!(f, "backup:ListBackupSelections"),
            BackupActions::ListBackupVaults => write!(f, "backup:ListBackupVaults"),
            BackupActions::ListCopyJobSummaries => write!(f, "backup:ListCopyJobSummaries"),
            BackupActions::ListCopyJobs => write!(f, "backup:ListCopyJobs"),
            BackupActions::ListFrameworks => write!(f, "backup:ListFrameworks"),
            BackupActions::ListIndexedRecoveryPoints => {
                write!(f, "backup:ListIndexedRecoveryPoints")
            }
            BackupActions::ListIndexedRecoveryPointsForSearch => {
                write!(f, "backup:ListIndexedRecoveryPointsForSearch")
            }
            BackupActions::ListLegalHolds => write!(f, "backup:ListLegalHolds"),
            BackupActions::ListProtectedResources => write!(f, "backup:ListProtectedResources"),
            BackupActions::ListProtectedResourcesByBackupVault => {
                write!(f, "backup:ListProtectedResourcesByBackupVault")
            }
            BackupActions::ListRecoveryPointsByBackupVault => {
                write!(f, "backup:ListRecoveryPointsByBackupVault")
            }
            BackupActions::ListRecoveryPointsByLegalHold => {
                write!(f, "backup:ListRecoveryPointsByLegalHold")
            }
            BackupActions::ListRecoveryPointsByResource => {
                write!(f, "backup:ListRecoveryPointsByResource")
            }
            BackupActions::ListReportJobs => write!(f, "backup:ListReportJobs"),
            BackupActions::ListReportPlans => write!(f, "backup:ListReportPlans"),
            BackupActions::ListRestoreAccessBackupVaults => {
                write!(f, "backup:ListRestoreAccessBackupVaults")
            }
            BackupActions::ListRestoreJobSummaries => write!(f, "backup:ListRestoreJobSummaries"),
            BackupActions::ListRestoreJobs => write!(f, "backup:ListRestoreJobs"),
            BackupActions::ListRestoreJobsByProtectedResource => {
                write!(f, "backup:ListRestoreJobsByProtectedResource")
            }
            BackupActions::ListRestoreTestingPlans => write!(f, "backup:ListRestoreTestingPlans"),
            BackupActions::ListRestoreTestingSelections => {
                write!(f, "backup:ListRestoreTestingSelections")
            }
            BackupActions::ListTags => write!(f, "backup:ListTags"),
            BackupActions::PutBackupVaultAccessPolicy => {
                write!(f, "backup:PutBackupVaultAccessPolicy")
            }
            BackupActions::PutBackupVaultLockConfiguration => {
                write!(f, "backup:PutBackupVaultLockConfiguration")
            }
            BackupActions::PutBackupVaultNotifications => {
                write!(f, "backup:PutBackupVaultNotifications")
            }
            BackupActions::PutBackupVaultSharingPolicy => {
                write!(f, "backup:PutBackupVaultSharingPolicy")
            }
            BackupActions::PutRestoreValidationResult => {
                write!(f, "backup:PutRestoreValidationResult")
            }
            BackupActions::RevokeRestoreAccessBackupVault => {
                write!(f, "backup:RevokeRestoreAccessBackupVault")
            }
            BackupActions::SearchRecoveryPoint => write!(f, "backup:SearchRecoveryPoint"),
            BackupActions::StartBackupJob => write!(f, "backup:StartBackupJob"),
            BackupActions::StartCopyJob => write!(f, "backup:StartCopyJob"),
            BackupActions::StartReportJob => write!(f, "backup:StartReportJob"),
            BackupActions::StartRestoreJob => write!(f, "backup:StartRestoreJob"),
            BackupActions::StopBackupJob => write!(f, "backup:StopBackupJob"),
            BackupActions::TagResource => write!(f, "backup:TagResource"),
            BackupActions::UntagResource => write!(f, "backup:UntagResource"),
            BackupActions::UpdateBackupPlan => write!(f, "backup:UpdateBackupPlan"),
            BackupActions::UpdateFramework => write!(f, "backup:UpdateFramework"),
            BackupActions::UpdateGlobalSettings => write!(f, "backup:UpdateGlobalSettings"),
            BackupActions::UpdateRecoveryPointIndexSettings => {
                write!(f, "backup:UpdateRecoveryPointIndexSettings")
            }
            BackupActions::UpdateRecoveryPointLifecycle => {
                write!(f, "backup:UpdateRecoveryPointLifecycle")
            }
            BackupActions::UpdateRegionSettings => write!(f, "backup:UpdateRegionSettings"),
            BackupActions::UpdateReportPlan => write!(f, "backup:UpdateReportPlan"),
            BackupActions::UpdateRestoreTestingPlan => write!(f, "backup:UpdateRestoreTestingPlan"),
            BackupActions::UpdateRestoreTestingSelection => {
                write!(f, "backup:UpdateRestoreTestingSelection")
            }
        }
    }
}
