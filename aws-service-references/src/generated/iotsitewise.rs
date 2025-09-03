// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum IotsitewiseActions {
    AssociateAssets,
    AssociateTimeSeriesToAssetProperty,
    BatchAssociateProjectAssets,
    BatchDisassociateProjectAssets,
    BatchGetAssetPropertyAggregates,
    BatchGetAssetPropertyValue,
    BatchGetAssetPropertyValueHistory,
    BatchPutAssetPropertyValue,
    CreateAccessPolicy,
    CreateAsset,
    CreateAssetModel,
    CreateAssetModelCompositeModel,
    CreateBulkImportJob,
    CreateComputationModel,
    CreateDashboard,
    CreateDataset,
    CreateGateway,
    CreatePortal,
    CreateProject,
    DeleteAccessPolicy,
    DeleteAsset,
    DeleteAssetModel,
    DeleteAssetModelCompositeModel,
    DeleteAssetModelInterfaceRelationship,
    DeleteComputationModel,
    DeleteDashboard,
    DeleteDataset,
    DeleteGateway,
    DeletePortal,
    DeleteProject,
    DeleteTimeSeries,
    DescribeAccessPolicy,
    DescribeAction,
    DescribeAsset,
    DescribeAssetCompositeModel,
    DescribeAssetModel,
    DescribeAssetModelCompositeModel,
    DescribeAssetModelInterfaceRelationship,
    DescribeAssetProperty,
    DescribeBulkImportJob,
    DescribeComputationModel,
    DescribeComputationModelExecutionSummary,
    DescribeDashboard,
    DescribeDataset,
    DescribeDefaultEncryptionConfiguration,
    DescribeExecution,
    DescribeGateway,
    DescribeGatewayCapabilityConfiguration,
    DescribeLoggingOptions,
    DescribePortal,
    DescribeProject,
    DescribeStorageConfiguration,
    DescribeTimeSeries,
    DisassociateAssets,
    DisassociateTimeSeriesFromAssetProperty,
    EnableSiteWiseIntegration,
    ExecuteAction,
    ExecuteQuery,
    GetAssetPropertyAggregates,
    GetAssetPropertyValue,
    GetAssetPropertyValueHistory,
    GetInterpolatedAssetPropertyValues,
    InvokeAssistant,
    ListAccessPolicies,
    ListActions,
    ListAssetModelCompositeModels,
    ListAssetModelProperties,
    ListAssetModels,
    ListAssetProperties,
    ListAssetRelationships,
    ListAssets,
    ListAssociatedAssets,
    ListBulkImportJobs,
    ListCompositionRelationships,
    ListComputationModelDataBindingUsages,
    ListComputationModelResolveToResources,
    ListComputationModels,
    ListDashboards,
    ListDatasets,
    ListExecutions,
    ListGateways,
    ListInterfaceRelationships,
    ListPortals,
    ListProjectAssets,
    ListProjects,
    ListTagsForResource,
    ListTimeSeries,
    PutAssetModelInterfaceRelationship,
    PutDefaultEncryptionConfiguration,
    PutLoggingOptions,
    PutStorageConfiguration,
    TagResource,
    UntagResource,
    UpdateAccessPolicy,
    UpdateAsset,
    UpdateAssetModel,
    UpdateAssetModelCompositeModel,
    UpdateAssetModelPropertyRouting,
    UpdateAssetProperty,
    UpdateComputationModel,
    UpdateDashboard,
    UpdateDataset,
    UpdateGateway,
    UpdateGatewayCapabilityConfiguration,
    UpdatePortal,
    UpdateProject,
}
impl std::fmt::Display for IotsitewiseActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IotsitewiseActions::AssociateAssets => write!(f, "iotsitewise:AssociateAssets"),
            IotsitewiseActions::AssociateTimeSeriesToAssetProperty => {
                write!(f, "iotsitewise:AssociateTimeSeriesToAssetProperty")
            }
            IotsitewiseActions::BatchAssociateProjectAssets => {
                write!(f, "iotsitewise:BatchAssociateProjectAssets")
            }
            IotsitewiseActions::BatchDisassociateProjectAssets => {
                write!(f, "iotsitewise:BatchDisassociateProjectAssets")
            }
            IotsitewiseActions::BatchGetAssetPropertyAggregates => {
                write!(f, "iotsitewise:BatchGetAssetPropertyAggregates")
            }
            IotsitewiseActions::BatchGetAssetPropertyValue => {
                write!(f, "iotsitewise:BatchGetAssetPropertyValue")
            }
            IotsitewiseActions::BatchGetAssetPropertyValueHistory => {
                write!(f, "iotsitewise:BatchGetAssetPropertyValueHistory")
            }
            IotsitewiseActions::BatchPutAssetPropertyValue => {
                write!(f, "iotsitewise:BatchPutAssetPropertyValue")
            }
            IotsitewiseActions::CreateAccessPolicy => write!(f, "iotsitewise:CreateAccessPolicy"),
            IotsitewiseActions::CreateAsset => write!(f, "iotsitewise:CreateAsset"),
            IotsitewiseActions::CreateAssetModel => write!(f, "iotsitewise:CreateAssetModel"),
            IotsitewiseActions::CreateAssetModelCompositeModel => {
                write!(f, "iotsitewise:CreateAssetModelCompositeModel")
            }
            IotsitewiseActions::CreateBulkImportJob => write!(f, "iotsitewise:CreateBulkImportJob"),
            IotsitewiseActions::CreateComputationModel => {
                write!(f, "iotsitewise:CreateComputationModel")
            }
            IotsitewiseActions::CreateDashboard => write!(f, "iotsitewise:CreateDashboard"),
            IotsitewiseActions::CreateDataset => write!(f, "iotsitewise:CreateDataset"),
            IotsitewiseActions::CreateGateway => write!(f, "iotsitewise:CreateGateway"),
            IotsitewiseActions::CreatePortal => write!(f, "iotsitewise:CreatePortal"),
            IotsitewiseActions::CreateProject => write!(f, "iotsitewise:CreateProject"),
            IotsitewiseActions::DeleteAccessPolicy => write!(f, "iotsitewise:DeleteAccessPolicy"),
            IotsitewiseActions::DeleteAsset => write!(f, "iotsitewise:DeleteAsset"),
            IotsitewiseActions::DeleteAssetModel => write!(f, "iotsitewise:DeleteAssetModel"),
            IotsitewiseActions::DeleteAssetModelCompositeModel => {
                write!(f, "iotsitewise:DeleteAssetModelCompositeModel")
            }
            IotsitewiseActions::DeleteAssetModelInterfaceRelationship => {
                write!(f, "iotsitewise:DeleteAssetModelInterfaceRelationship")
            }
            IotsitewiseActions::DeleteComputationModel => {
                write!(f, "iotsitewise:DeleteComputationModel")
            }
            IotsitewiseActions::DeleteDashboard => write!(f, "iotsitewise:DeleteDashboard"),
            IotsitewiseActions::DeleteDataset => write!(f, "iotsitewise:DeleteDataset"),
            IotsitewiseActions::DeleteGateway => write!(f, "iotsitewise:DeleteGateway"),
            IotsitewiseActions::DeletePortal => write!(f, "iotsitewise:DeletePortal"),
            IotsitewiseActions::DeleteProject => write!(f, "iotsitewise:DeleteProject"),
            IotsitewiseActions::DeleteTimeSeries => write!(f, "iotsitewise:DeleteTimeSeries"),
            IotsitewiseActions::DescribeAccessPolicy => {
                write!(f, "iotsitewise:DescribeAccessPolicy")
            }
            IotsitewiseActions::DescribeAction => write!(f, "iotsitewise:DescribeAction"),
            IotsitewiseActions::DescribeAsset => write!(f, "iotsitewise:DescribeAsset"),
            IotsitewiseActions::DescribeAssetCompositeModel => {
                write!(f, "iotsitewise:DescribeAssetCompositeModel")
            }
            IotsitewiseActions::DescribeAssetModel => write!(f, "iotsitewise:DescribeAssetModel"),
            IotsitewiseActions::DescribeAssetModelCompositeModel => {
                write!(f, "iotsitewise:DescribeAssetModelCompositeModel")
            }
            IotsitewiseActions::DescribeAssetModelInterfaceRelationship => {
                write!(f, "iotsitewise:DescribeAssetModelInterfaceRelationship")
            }
            IotsitewiseActions::DescribeAssetProperty => {
                write!(f, "iotsitewise:DescribeAssetProperty")
            }
            IotsitewiseActions::DescribeBulkImportJob => {
                write!(f, "iotsitewise:DescribeBulkImportJob")
            }
            IotsitewiseActions::DescribeComputationModel => {
                write!(f, "iotsitewise:DescribeComputationModel")
            }
            IotsitewiseActions::DescribeComputationModelExecutionSummary => {
                write!(f, "iotsitewise:DescribeComputationModelExecutionSummary")
            }
            IotsitewiseActions::DescribeDashboard => write!(f, "iotsitewise:DescribeDashboard"),
            IotsitewiseActions::DescribeDataset => write!(f, "iotsitewise:DescribeDataset"),
            IotsitewiseActions::DescribeDefaultEncryptionConfiguration => {
                write!(f, "iotsitewise:DescribeDefaultEncryptionConfiguration")
            }
            IotsitewiseActions::DescribeExecution => write!(f, "iotsitewise:DescribeExecution"),
            IotsitewiseActions::DescribeGateway => write!(f, "iotsitewise:DescribeGateway"),
            IotsitewiseActions::DescribeGatewayCapabilityConfiguration => {
                write!(f, "iotsitewise:DescribeGatewayCapabilityConfiguration")
            }
            IotsitewiseActions::DescribeLoggingOptions => {
                write!(f, "iotsitewise:DescribeLoggingOptions")
            }
            IotsitewiseActions::DescribePortal => write!(f, "iotsitewise:DescribePortal"),
            IotsitewiseActions::DescribeProject => write!(f, "iotsitewise:DescribeProject"),
            IotsitewiseActions::DescribeStorageConfiguration => {
                write!(f, "iotsitewise:DescribeStorageConfiguration")
            }
            IotsitewiseActions::DescribeTimeSeries => write!(f, "iotsitewise:DescribeTimeSeries"),
            IotsitewiseActions::DisassociateAssets => write!(f, "iotsitewise:DisassociateAssets"),
            IotsitewiseActions::DisassociateTimeSeriesFromAssetProperty => {
                write!(f, "iotsitewise:DisassociateTimeSeriesFromAssetProperty")
            }
            IotsitewiseActions::EnableSiteWiseIntegration => {
                write!(f, "iotsitewise:EnableSiteWiseIntegration")
            }
            IotsitewiseActions::ExecuteAction => write!(f, "iotsitewise:ExecuteAction"),
            IotsitewiseActions::ExecuteQuery => write!(f, "iotsitewise:ExecuteQuery"),
            IotsitewiseActions::GetAssetPropertyAggregates => {
                write!(f, "iotsitewise:GetAssetPropertyAggregates")
            }
            IotsitewiseActions::GetAssetPropertyValue => {
                write!(f, "iotsitewise:GetAssetPropertyValue")
            }
            IotsitewiseActions::GetAssetPropertyValueHistory => {
                write!(f, "iotsitewise:GetAssetPropertyValueHistory")
            }
            IotsitewiseActions::GetInterpolatedAssetPropertyValues => {
                write!(f, "iotsitewise:GetInterpolatedAssetPropertyValues")
            }
            IotsitewiseActions::InvokeAssistant => write!(f, "iotsitewise:InvokeAssistant"),
            IotsitewiseActions::ListAccessPolicies => write!(f, "iotsitewise:ListAccessPolicies"),
            IotsitewiseActions::ListActions => write!(f, "iotsitewise:ListActions"),
            IotsitewiseActions::ListAssetModelCompositeModels => {
                write!(f, "iotsitewise:ListAssetModelCompositeModels")
            }
            IotsitewiseActions::ListAssetModelProperties => {
                write!(f, "iotsitewise:ListAssetModelProperties")
            }
            IotsitewiseActions::ListAssetModels => write!(f, "iotsitewise:ListAssetModels"),
            IotsitewiseActions::ListAssetProperties => write!(f, "iotsitewise:ListAssetProperties"),
            IotsitewiseActions::ListAssetRelationships => {
                write!(f, "iotsitewise:ListAssetRelationships")
            }
            IotsitewiseActions::ListAssets => write!(f, "iotsitewise:ListAssets"),
            IotsitewiseActions::ListAssociatedAssets => {
                write!(f, "iotsitewise:ListAssociatedAssets")
            }
            IotsitewiseActions::ListBulkImportJobs => write!(f, "iotsitewise:ListBulkImportJobs"),
            IotsitewiseActions::ListCompositionRelationships => {
                write!(f, "iotsitewise:ListCompositionRelationships")
            }
            IotsitewiseActions::ListComputationModelDataBindingUsages => {
                write!(f, "iotsitewise:ListComputationModelDataBindingUsages")
            }
            IotsitewiseActions::ListComputationModelResolveToResources => {
                write!(f, "iotsitewise:ListComputationModelResolveToResources")
            }
            IotsitewiseActions::ListComputationModels => {
                write!(f, "iotsitewise:ListComputationModels")
            }
            IotsitewiseActions::ListDashboards => write!(f, "iotsitewise:ListDashboards"),
            IotsitewiseActions::ListDatasets => write!(f, "iotsitewise:ListDatasets"),
            IotsitewiseActions::ListExecutions => write!(f, "iotsitewise:ListExecutions"),
            IotsitewiseActions::ListGateways => write!(f, "iotsitewise:ListGateways"),
            IotsitewiseActions::ListInterfaceRelationships => {
                write!(f, "iotsitewise:ListInterfaceRelationships")
            }
            IotsitewiseActions::ListPortals => write!(f, "iotsitewise:ListPortals"),
            IotsitewiseActions::ListProjectAssets => write!(f, "iotsitewise:ListProjectAssets"),
            IotsitewiseActions::ListProjects => write!(f, "iotsitewise:ListProjects"),
            IotsitewiseActions::ListTagsForResource => write!(f, "iotsitewise:ListTagsForResource"),
            IotsitewiseActions::ListTimeSeries => write!(f, "iotsitewise:ListTimeSeries"),
            IotsitewiseActions::PutAssetModelInterfaceRelationship => {
                write!(f, "iotsitewise:PutAssetModelInterfaceRelationship")
            }
            IotsitewiseActions::PutDefaultEncryptionConfiguration => {
                write!(f, "iotsitewise:PutDefaultEncryptionConfiguration")
            }
            IotsitewiseActions::PutLoggingOptions => write!(f, "iotsitewise:PutLoggingOptions"),
            IotsitewiseActions::PutStorageConfiguration => {
                write!(f, "iotsitewise:PutStorageConfiguration")
            }
            IotsitewiseActions::TagResource => write!(f, "iotsitewise:TagResource"),
            IotsitewiseActions::UntagResource => write!(f, "iotsitewise:UntagResource"),
            IotsitewiseActions::UpdateAccessPolicy => write!(f, "iotsitewise:UpdateAccessPolicy"),
            IotsitewiseActions::UpdateAsset => write!(f, "iotsitewise:UpdateAsset"),
            IotsitewiseActions::UpdateAssetModel => write!(f, "iotsitewise:UpdateAssetModel"),
            IotsitewiseActions::UpdateAssetModelCompositeModel => {
                write!(f, "iotsitewise:UpdateAssetModelCompositeModel")
            }
            IotsitewiseActions::UpdateAssetModelPropertyRouting => {
                write!(f, "iotsitewise:UpdateAssetModelPropertyRouting")
            }
            IotsitewiseActions::UpdateAssetProperty => write!(f, "iotsitewise:UpdateAssetProperty"),
            IotsitewiseActions::UpdateComputationModel => {
                write!(f, "iotsitewise:UpdateComputationModel")
            }
            IotsitewiseActions::UpdateDashboard => write!(f, "iotsitewise:UpdateDashboard"),
            IotsitewiseActions::UpdateDataset => write!(f, "iotsitewise:UpdateDataset"),
            IotsitewiseActions::UpdateGateway => write!(f, "iotsitewise:UpdateGateway"),
            IotsitewiseActions::UpdateGatewayCapabilityConfiguration => {
                write!(f, "iotsitewise:UpdateGatewayCapabilityConfiguration")
            }
            IotsitewiseActions::UpdatePortal => write!(f, "iotsitewise:UpdatePortal"),
            IotsitewiseActions::UpdateProject => write!(f, "iotsitewise:UpdateProject"),
        }
    }
}
