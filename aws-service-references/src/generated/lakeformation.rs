// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum LakeformationActions {
    AddLfTagsToResource,
    BatchGrantPermissions,
    BatchRevokePermissions,
    CancelTransaction,
    CommitTransaction,
    CreateDataCellsFilter,
    CreateLakeFormationIdentityCenterConfiguration,
    CreateLakeFormationOptIn,
    CreateLfTag,
    CreateLfTagExpression,
    DeleteDataCellsFilter,
    DeleteLakeFormationIdentityCenterConfiguration,
    DeleteLakeFormationOptIn,
    DeleteLfTag,
    DeleteLfTagExpression,
    DeleteObjectsOnCancel,
    DeregisterResource,
    DescribeLakeFormationIdentityCenterConfiguration,
    DescribeResource,
    DescribeTransaction,
    ExtendTransaction,
    GetDataAccess,
    GetDataCellsFilter,
    GetDataLakePrincipal,
    GetDataLakeSettings,
    GetEffectivePermissionsForPath,
    GetLfTag,
    GetLfTagExpression,
    GetQueryState,
    GetQueryStatistics,
    GetResourceLfTags,
    GetTableObjects,
    GetWorkUnitResults,
    GetWorkUnits,
    GrantPermissions,
    ListDataCellsFilter,
    ListLakeFormationOptIns,
    ListLfTagExpressions,
    ListLfTags,
    ListPermissions,
    ListResources,
    ListTableStorageOptimizers,
    ListTransactions,
    PutDataLakeSettings,
    RegisterResource,
    RegisterResourceWithPrivilegedAccess,
    RemoveLfTagsFromResource,
    RevokePermissions,
    SearchDatabasesByLfTags,
    SearchTablesByLfTags,
    StartQueryPlanning,
    StartTransaction,
    UpdateDataCellsFilter,
    UpdateLakeFormationIdentityCenterConfiguration,
    UpdateLfTag,
    UpdateLfTagExpression,
    UpdateResource,
    UpdateTableObjects,
    UpdateTableStorageOptimizer,
}
impl std::fmt::Display for LakeformationActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LakeformationActions::AddLfTagsToResource => {
                write!(f, "lakeformation:AddLFTagsToResource")
            }
            LakeformationActions::BatchGrantPermissions => {
                write!(f, "lakeformation:BatchGrantPermissions")
            }
            LakeformationActions::BatchRevokePermissions => {
                write!(f, "lakeformation:BatchRevokePermissions")
            }
            LakeformationActions::CancelTransaction => write!(f, "lakeformation:CancelTransaction"),
            LakeformationActions::CommitTransaction => write!(f, "lakeformation:CommitTransaction"),
            LakeformationActions::CreateDataCellsFilter => {
                write!(f, "lakeformation:CreateDataCellsFilter")
            }
            LakeformationActions::CreateLakeFormationIdentityCenterConfiguration => write!(
                f,
                "lakeformation:CreateLakeFormationIdentityCenterConfiguration"
            ),
            LakeformationActions::CreateLakeFormationOptIn => {
                write!(f, "lakeformation:CreateLakeFormationOptIn")
            }
            LakeformationActions::CreateLfTag => write!(f, "lakeformation:CreateLFTag"),
            LakeformationActions::CreateLfTagExpression => {
                write!(f, "lakeformation:CreateLFTagExpression")
            }
            LakeformationActions::DeleteDataCellsFilter => {
                write!(f, "lakeformation:DeleteDataCellsFilter")
            }
            LakeformationActions::DeleteLakeFormationIdentityCenterConfiguration => write!(
                f,
                "lakeformation:DeleteLakeFormationIdentityCenterConfiguration"
            ),
            LakeformationActions::DeleteLakeFormationOptIn => {
                write!(f, "lakeformation:DeleteLakeFormationOptIn")
            }
            LakeformationActions::DeleteLfTag => write!(f, "lakeformation:DeleteLFTag"),
            LakeformationActions::DeleteLfTagExpression => {
                write!(f, "lakeformation:DeleteLFTagExpression")
            }
            LakeformationActions::DeleteObjectsOnCancel => {
                write!(f, "lakeformation:DeleteObjectsOnCancel")
            }
            LakeformationActions::DeregisterResource => {
                write!(f, "lakeformation:DeregisterResource")
            }
            LakeformationActions::DescribeLakeFormationIdentityCenterConfiguration => write!(
                f,
                "lakeformation:DescribeLakeFormationIdentityCenterConfiguration"
            ),
            LakeformationActions::DescribeResource => write!(f, "lakeformation:DescribeResource"),
            LakeformationActions::DescribeTransaction => {
                write!(f, "lakeformation:DescribeTransaction")
            }
            LakeformationActions::ExtendTransaction => write!(f, "lakeformation:ExtendTransaction"),
            LakeformationActions::GetDataAccess => write!(f, "lakeformation:GetDataAccess"),
            LakeformationActions::GetDataCellsFilter => {
                write!(f, "lakeformation:GetDataCellsFilter")
            }
            LakeformationActions::GetDataLakePrincipal => {
                write!(f, "lakeformation:GetDataLakePrincipal")
            }
            LakeformationActions::GetDataLakeSettings => {
                write!(f, "lakeformation:GetDataLakeSettings")
            }
            LakeformationActions::GetEffectivePermissionsForPath => {
                write!(f, "lakeformation:GetEffectivePermissionsForPath")
            }
            LakeformationActions::GetLfTag => write!(f, "lakeformation:GetLFTag"),
            LakeformationActions::GetLfTagExpression => {
                write!(f, "lakeformation:GetLFTagExpression")
            }
            LakeformationActions::GetQueryState => write!(f, "lakeformation:GetQueryState"),
            LakeformationActions::GetQueryStatistics => {
                write!(f, "lakeformation:GetQueryStatistics")
            }
            LakeformationActions::GetResourceLfTags => write!(f, "lakeformation:GetResourceLFTags"),
            LakeformationActions::GetTableObjects => write!(f, "lakeformation:GetTableObjects"),
            LakeformationActions::GetWorkUnitResults => {
                write!(f, "lakeformation:GetWorkUnitResults")
            }
            LakeformationActions::GetWorkUnits => write!(f, "lakeformation:GetWorkUnits"),
            LakeformationActions::GrantPermissions => write!(f, "lakeformation:GrantPermissions"),
            LakeformationActions::ListDataCellsFilter => {
                write!(f, "lakeformation:ListDataCellsFilter")
            }
            LakeformationActions::ListLakeFormationOptIns => {
                write!(f, "lakeformation:ListLakeFormationOptIns")
            }
            LakeformationActions::ListLfTagExpressions => {
                write!(f, "lakeformation:ListLFTagExpressions")
            }
            LakeformationActions::ListLfTags => write!(f, "lakeformation:ListLFTags"),
            LakeformationActions::ListPermissions => write!(f, "lakeformation:ListPermissions"),
            LakeformationActions::ListResources => write!(f, "lakeformation:ListResources"),
            LakeformationActions::ListTableStorageOptimizers => {
                write!(f, "lakeformation:ListTableStorageOptimizers")
            }
            LakeformationActions::ListTransactions => write!(f, "lakeformation:ListTransactions"),
            LakeformationActions::PutDataLakeSettings => {
                write!(f, "lakeformation:PutDataLakeSettings")
            }
            LakeformationActions::RegisterResource => write!(f, "lakeformation:RegisterResource"),
            LakeformationActions::RegisterResourceWithPrivilegedAccess => {
                write!(f, "lakeformation:RegisterResourceWithPrivilegedAccess")
            }
            LakeformationActions::RemoveLfTagsFromResource => {
                write!(f, "lakeformation:RemoveLFTagsFromResource")
            }
            LakeformationActions::RevokePermissions => write!(f, "lakeformation:RevokePermissions"),
            LakeformationActions::SearchDatabasesByLfTags => {
                write!(f, "lakeformation:SearchDatabasesByLFTags")
            }
            LakeformationActions::SearchTablesByLfTags => {
                write!(f, "lakeformation:SearchTablesByLFTags")
            }
            LakeformationActions::StartQueryPlanning => {
                write!(f, "lakeformation:StartQueryPlanning")
            }
            LakeformationActions::StartTransaction => write!(f, "lakeformation:StartTransaction"),
            LakeformationActions::UpdateDataCellsFilter => {
                write!(f, "lakeformation:UpdateDataCellsFilter")
            }
            LakeformationActions::UpdateLakeFormationIdentityCenterConfiguration => write!(
                f,
                "lakeformation:UpdateLakeFormationIdentityCenterConfiguration"
            ),
            LakeformationActions::UpdateLfTag => write!(f, "lakeformation:UpdateLFTag"),
            LakeformationActions::UpdateLfTagExpression => {
                write!(f, "lakeformation:UpdateLFTagExpression")
            }
            LakeformationActions::UpdateResource => write!(f, "lakeformation:UpdateResource"),
            LakeformationActions::UpdateTableObjects => {
                write!(f, "lakeformation:UpdateTableObjects")
            }
            LakeformationActions::UpdateTableStorageOptimizer => {
                write!(f, "lakeformation:UpdateTableStorageOptimizer")
            }
        }
    }
}
