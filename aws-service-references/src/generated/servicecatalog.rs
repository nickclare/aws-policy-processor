// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum ServicecatalogActions {
    AcceptPortfolioShare,
    AssociateAttributeGroup,
    AssociateBudgetWithResource,
    AssociatePrincipalWithPortfolio,
    AssociateProductWithPortfolio,
    AssociateResource,
    AssociateServiceActionWithProvisioningArtifact,
    AssociateTagOptionWithResource,
    BatchAssociateServiceActionWithProvisioningArtifact,
    BatchDisassociateServiceActionFromProvisioningArtifact,
    CopyProduct,
    CreateApplication,
    CreateAttributeGroup,
    CreateConstraint,
    CreatePortfolio,
    CreatePortfolioShare,
    CreateProduct,
    CreateProvisionedProductPlan,
    CreateProvisioningArtifact,
    CreateServiceAction,
    CreateTagOption,
    DeleteApplication,
    DeleteAttributeGroup,
    DeleteConstraint,
    DeletePortfolio,
    DeletePortfolioShare,
    DeleteProduct,
    DeleteProvisionedProductPlan,
    DeleteProvisioningArtifact,
    DeleteResourcePolicy,
    DeleteServiceAction,
    DeleteTagOption,
    DescribeConstraint,
    DescribeCopyProductStatus,
    DescribePortfolio,
    DescribePortfolioShareStatus,
    DescribePortfolioShares,
    DescribeProduct,
    DescribeProductAsAdmin,
    DescribeProductView,
    DescribeProvisionedProduct,
    DescribeProvisionedProductPlan,
    DescribeProvisioningArtifact,
    DescribeProvisioningParameters,
    DescribeRecord,
    DescribeServiceAction,
    DescribeServiceActionExecutionParameters,
    DescribeTagOption,
    DisableAwsOrganizationsAccess,
    DisassociateAttributeGroup,
    DisassociateBudgetFromResource,
    DisassociatePrincipalFromPortfolio,
    DisassociateProductFromPortfolio,
    DisassociateResource,
    DisassociateServiceActionFromProvisioningArtifact,
    DisassociateTagOptionFromResource,
    EnableAwsOrganizationsAccess,
    ExecuteProvisionedProductPlan,
    ExecuteProvisionedProductServiceAction,
    GetApplication,
    GetAssociatedResource,
    GetAttributeGroup,
    GetAwsOrganizationsAccessStatus,
    GetConfiguration,
    GetProvisionedProductOutputs,
    GetResourcePolicy,
    ImportAsProvisionedProduct,
    ListAcceptedPortfolioShares,
    ListApplications,
    ListAssociatedAttributeGroups,
    ListAssociatedResources,
    ListAttributeGroups,
    ListAttributeGroupsForApplication,
    ListBudgetsForResource,
    ListConstraintsForPortfolio,
    ListLaunchPaths,
    ListOrganizationPortfolioAccess,
    ListPortfolioAccess,
    ListPortfolios,
    ListPortfoliosForProduct,
    ListPrincipalsForPortfolio,
    ListProvisionedProductPlans,
    ListProvisioningArtifacts,
    ListProvisioningArtifactsForServiceAction,
    ListRecordHistory,
    ListResourcesForTagOption,
    ListServiceActions,
    ListServiceActionsForProvisioningArtifact,
    ListStackInstancesForProvisionedProduct,
    ListTagOptions,
    ListTagsForResource,
    NotifyProvisionProductEngineWorkflowResult,
    NotifyTerminateProvisionedProductEngineWorkflowResult,
    NotifyUpdateProvisionedProductEngineWorkflowResult,
    ProvisionProduct,
    PutConfiguration,
    PutResourcePolicy,
    RejectPortfolioShare,
    ScanProvisionedProducts,
    SearchProducts,
    SearchProductsAsAdmin,
    SearchProvisionedProducts,
    SyncResource,
    TagResource,
    TerminateProvisionedProduct,
    UntagResource,
    UpdateApplication,
    UpdateAttributeGroup,
    UpdateConstraint,
    UpdatePortfolio,
    UpdatePortfolioShare,
    UpdateProduct,
    UpdateProvisionedProduct,
    UpdateProvisionedProductProperties,
    UpdateProvisioningArtifact,
    UpdateServiceAction,
    UpdateTagOption,
}
impl std::fmt::Display for ServicecatalogActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServicecatalogActions::AcceptPortfolioShare => {
                write!(f, "servicecatalog:AcceptPortfolioShare")
            }
            ServicecatalogActions::AssociateAttributeGroup => {
                write!(f, "servicecatalog:AssociateAttributeGroup")
            }
            ServicecatalogActions::AssociateBudgetWithResource => {
                write!(f, "servicecatalog:AssociateBudgetWithResource")
            }
            ServicecatalogActions::AssociatePrincipalWithPortfolio => {
                write!(f, "servicecatalog:AssociatePrincipalWithPortfolio")
            }
            ServicecatalogActions::AssociateProductWithPortfolio => {
                write!(f, "servicecatalog:AssociateProductWithPortfolio")
            }
            ServicecatalogActions::AssociateResource => {
                write!(f, "servicecatalog:AssociateResource")
            }
            ServicecatalogActions::AssociateServiceActionWithProvisioningArtifact => write!(
                f,
                "servicecatalog:AssociateServiceActionWithProvisioningArtifact"
            ),
            ServicecatalogActions::AssociateTagOptionWithResource => {
                write!(f, "servicecatalog:AssociateTagOptionWithResource")
            }
            ServicecatalogActions::BatchAssociateServiceActionWithProvisioningArtifact => write!(
                f,
                "servicecatalog:BatchAssociateServiceActionWithProvisioningArtifact"
            ),
            ServicecatalogActions::BatchDisassociateServiceActionFromProvisioningArtifact => {
                write!(
                    f,
                    "servicecatalog:BatchDisassociateServiceActionFromProvisioningArtifact"
                )
            }
            ServicecatalogActions::CopyProduct => write!(f, "servicecatalog:CopyProduct"),
            ServicecatalogActions::CreateApplication => {
                write!(f, "servicecatalog:CreateApplication")
            }
            ServicecatalogActions::CreateAttributeGroup => {
                write!(f, "servicecatalog:CreateAttributeGroup")
            }
            ServicecatalogActions::CreateConstraint => write!(f, "servicecatalog:CreateConstraint"),
            ServicecatalogActions::CreatePortfolio => write!(f, "servicecatalog:CreatePortfolio"),
            ServicecatalogActions::CreatePortfolioShare => {
                write!(f, "servicecatalog:CreatePortfolioShare")
            }
            ServicecatalogActions::CreateProduct => write!(f, "servicecatalog:CreateProduct"),
            ServicecatalogActions::CreateProvisionedProductPlan => {
                write!(f, "servicecatalog:CreateProvisionedProductPlan")
            }
            ServicecatalogActions::CreateProvisioningArtifact => {
                write!(f, "servicecatalog:CreateProvisioningArtifact")
            }
            ServicecatalogActions::CreateServiceAction => {
                write!(f, "servicecatalog:CreateServiceAction")
            }
            ServicecatalogActions::CreateTagOption => write!(f, "servicecatalog:CreateTagOption"),
            ServicecatalogActions::DeleteApplication => {
                write!(f, "servicecatalog:DeleteApplication")
            }
            ServicecatalogActions::DeleteAttributeGroup => {
                write!(f, "servicecatalog:DeleteAttributeGroup")
            }
            ServicecatalogActions::DeleteConstraint => write!(f, "servicecatalog:DeleteConstraint"),
            ServicecatalogActions::DeletePortfolio => write!(f, "servicecatalog:DeletePortfolio"),
            ServicecatalogActions::DeletePortfolioShare => {
                write!(f, "servicecatalog:DeletePortfolioShare")
            }
            ServicecatalogActions::DeleteProduct => write!(f, "servicecatalog:DeleteProduct"),
            ServicecatalogActions::DeleteProvisionedProductPlan => {
                write!(f, "servicecatalog:DeleteProvisionedProductPlan")
            }
            ServicecatalogActions::DeleteProvisioningArtifact => {
                write!(f, "servicecatalog:DeleteProvisioningArtifact")
            }
            ServicecatalogActions::DeleteResourcePolicy => {
                write!(f, "servicecatalog:DeleteResourcePolicy")
            }
            ServicecatalogActions::DeleteServiceAction => {
                write!(f, "servicecatalog:DeleteServiceAction")
            }
            ServicecatalogActions::DeleteTagOption => write!(f, "servicecatalog:DeleteTagOption"),
            ServicecatalogActions::DescribeConstraint => {
                write!(f, "servicecatalog:DescribeConstraint")
            }
            ServicecatalogActions::DescribeCopyProductStatus => {
                write!(f, "servicecatalog:DescribeCopyProductStatus")
            }
            ServicecatalogActions::DescribePortfolio => {
                write!(f, "servicecatalog:DescribePortfolio")
            }
            ServicecatalogActions::DescribePortfolioShareStatus => {
                write!(f, "servicecatalog:DescribePortfolioShareStatus")
            }
            ServicecatalogActions::DescribePortfolioShares => {
                write!(f, "servicecatalog:DescribePortfolioShares")
            }
            ServicecatalogActions::DescribeProduct => write!(f, "servicecatalog:DescribeProduct"),
            ServicecatalogActions::DescribeProductAsAdmin => {
                write!(f, "servicecatalog:DescribeProductAsAdmin")
            }
            ServicecatalogActions::DescribeProductView => {
                write!(f, "servicecatalog:DescribeProductView")
            }
            ServicecatalogActions::DescribeProvisionedProduct => {
                write!(f, "servicecatalog:DescribeProvisionedProduct")
            }
            ServicecatalogActions::DescribeProvisionedProductPlan => {
                write!(f, "servicecatalog:DescribeProvisionedProductPlan")
            }
            ServicecatalogActions::DescribeProvisioningArtifact => {
                write!(f, "servicecatalog:DescribeProvisioningArtifact")
            }
            ServicecatalogActions::DescribeProvisioningParameters => {
                write!(f, "servicecatalog:DescribeProvisioningParameters")
            }
            ServicecatalogActions::DescribeRecord => write!(f, "servicecatalog:DescribeRecord"),
            ServicecatalogActions::DescribeServiceAction => {
                write!(f, "servicecatalog:DescribeServiceAction")
            }
            ServicecatalogActions::DescribeServiceActionExecutionParameters => {
                write!(f, "servicecatalog:DescribeServiceActionExecutionParameters")
            }
            ServicecatalogActions::DescribeTagOption => {
                write!(f, "servicecatalog:DescribeTagOption")
            }
            ServicecatalogActions::DisableAwsOrganizationsAccess => {
                write!(f, "servicecatalog:DisableAWSOrganizationsAccess")
            }
            ServicecatalogActions::DisassociateAttributeGroup => {
                write!(f, "servicecatalog:DisassociateAttributeGroup")
            }
            ServicecatalogActions::DisassociateBudgetFromResource => {
                write!(f, "servicecatalog:DisassociateBudgetFromResource")
            }
            ServicecatalogActions::DisassociatePrincipalFromPortfolio => {
                write!(f, "servicecatalog:DisassociatePrincipalFromPortfolio")
            }
            ServicecatalogActions::DisassociateProductFromPortfolio => {
                write!(f, "servicecatalog:DisassociateProductFromPortfolio")
            }
            ServicecatalogActions::DisassociateResource => {
                write!(f, "servicecatalog:DisassociateResource")
            }
            ServicecatalogActions::DisassociateServiceActionFromProvisioningArtifact => write!(
                f,
                "servicecatalog:DisassociateServiceActionFromProvisioningArtifact"
            ),
            ServicecatalogActions::DisassociateTagOptionFromResource => {
                write!(f, "servicecatalog:DisassociateTagOptionFromResource")
            }
            ServicecatalogActions::EnableAwsOrganizationsAccess => {
                write!(f, "servicecatalog:EnableAWSOrganizationsAccess")
            }
            ServicecatalogActions::ExecuteProvisionedProductPlan => {
                write!(f, "servicecatalog:ExecuteProvisionedProductPlan")
            }
            ServicecatalogActions::ExecuteProvisionedProductServiceAction => {
                write!(f, "servicecatalog:ExecuteProvisionedProductServiceAction")
            }
            ServicecatalogActions::GetApplication => write!(f, "servicecatalog:GetApplication"),
            ServicecatalogActions::GetAssociatedResource => {
                write!(f, "servicecatalog:GetAssociatedResource")
            }
            ServicecatalogActions::GetAttributeGroup => {
                write!(f, "servicecatalog:GetAttributeGroup")
            }
            ServicecatalogActions::GetAwsOrganizationsAccessStatus => {
                write!(f, "servicecatalog:GetAWSOrganizationsAccessStatus")
            }
            ServicecatalogActions::GetConfiguration => write!(f, "servicecatalog:GetConfiguration"),
            ServicecatalogActions::GetProvisionedProductOutputs => {
                write!(f, "servicecatalog:GetProvisionedProductOutputs")
            }
            ServicecatalogActions::GetResourcePolicy => {
                write!(f, "servicecatalog:GetResourcePolicy")
            }
            ServicecatalogActions::ImportAsProvisionedProduct => {
                write!(f, "servicecatalog:ImportAsProvisionedProduct")
            }
            ServicecatalogActions::ListAcceptedPortfolioShares => {
                write!(f, "servicecatalog:ListAcceptedPortfolioShares")
            }
            ServicecatalogActions::ListApplications => write!(f, "servicecatalog:ListApplications"),
            ServicecatalogActions::ListAssociatedAttributeGroups => {
                write!(f, "servicecatalog:ListAssociatedAttributeGroups")
            }
            ServicecatalogActions::ListAssociatedResources => {
                write!(f, "servicecatalog:ListAssociatedResources")
            }
            ServicecatalogActions::ListAttributeGroups => {
                write!(f, "servicecatalog:ListAttributeGroups")
            }
            ServicecatalogActions::ListAttributeGroupsForApplication => {
                write!(f, "servicecatalog:ListAttributeGroupsForApplication")
            }
            ServicecatalogActions::ListBudgetsForResource => {
                write!(f, "servicecatalog:ListBudgetsForResource")
            }
            ServicecatalogActions::ListConstraintsForPortfolio => {
                write!(f, "servicecatalog:ListConstraintsForPortfolio")
            }
            ServicecatalogActions::ListLaunchPaths => write!(f, "servicecatalog:ListLaunchPaths"),
            ServicecatalogActions::ListOrganizationPortfolioAccess => {
                write!(f, "servicecatalog:ListOrganizationPortfolioAccess")
            }
            ServicecatalogActions::ListPortfolioAccess => {
                write!(f, "servicecatalog:ListPortfolioAccess")
            }
            ServicecatalogActions::ListPortfolios => write!(f, "servicecatalog:ListPortfolios"),
            ServicecatalogActions::ListPortfoliosForProduct => {
                write!(f, "servicecatalog:ListPortfoliosForProduct")
            }
            ServicecatalogActions::ListPrincipalsForPortfolio => {
                write!(f, "servicecatalog:ListPrincipalsForPortfolio")
            }
            ServicecatalogActions::ListProvisionedProductPlans => {
                write!(f, "servicecatalog:ListProvisionedProductPlans")
            }
            ServicecatalogActions::ListProvisioningArtifacts => {
                write!(f, "servicecatalog:ListProvisioningArtifacts")
            }
            ServicecatalogActions::ListProvisioningArtifactsForServiceAction => write!(
                f,
                "servicecatalog:ListProvisioningArtifactsForServiceAction"
            ),
            ServicecatalogActions::ListRecordHistory => {
                write!(f, "servicecatalog:ListRecordHistory")
            }
            ServicecatalogActions::ListResourcesForTagOption => {
                write!(f, "servicecatalog:ListResourcesForTagOption")
            }
            ServicecatalogActions::ListServiceActions => {
                write!(f, "servicecatalog:ListServiceActions")
            }
            ServicecatalogActions::ListServiceActionsForProvisioningArtifact => write!(
                f,
                "servicecatalog:ListServiceActionsForProvisioningArtifact"
            ),
            ServicecatalogActions::ListStackInstancesForProvisionedProduct => {
                write!(f, "servicecatalog:ListStackInstancesForProvisionedProduct")
            }
            ServicecatalogActions::ListTagOptions => write!(f, "servicecatalog:ListTagOptions"),
            ServicecatalogActions::ListTagsForResource => {
                write!(f, "servicecatalog:ListTagsForResource")
            }
            ServicecatalogActions::NotifyProvisionProductEngineWorkflowResult => write!(
                f,
                "servicecatalog:NotifyProvisionProductEngineWorkflowResult"
            ),
            ServicecatalogActions::NotifyTerminateProvisionedProductEngineWorkflowResult => write!(
                f,
                "servicecatalog:NotifyTerminateProvisionedProductEngineWorkflowResult"
            ),
            ServicecatalogActions::NotifyUpdateProvisionedProductEngineWorkflowResult => write!(
                f,
                "servicecatalog:NotifyUpdateProvisionedProductEngineWorkflowResult"
            ),
            ServicecatalogActions::ProvisionProduct => write!(f, "servicecatalog:ProvisionProduct"),
            ServicecatalogActions::PutConfiguration => write!(f, "servicecatalog:PutConfiguration"),
            ServicecatalogActions::PutResourcePolicy => {
                write!(f, "servicecatalog:PutResourcePolicy")
            }
            ServicecatalogActions::RejectPortfolioShare => {
                write!(f, "servicecatalog:RejectPortfolioShare")
            }
            ServicecatalogActions::ScanProvisionedProducts => {
                write!(f, "servicecatalog:ScanProvisionedProducts")
            }
            ServicecatalogActions::SearchProducts => write!(f, "servicecatalog:SearchProducts"),
            ServicecatalogActions::SearchProductsAsAdmin => {
                write!(f, "servicecatalog:SearchProductsAsAdmin")
            }
            ServicecatalogActions::SearchProvisionedProducts => {
                write!(f, "servicecatalog:SearchProvisionedProducts")
            }
            ServicecatalogActions::SyncResource => write!(f, "servicecatalog:SyncResource"),
            ServicecatalogActions::TagResource => write!(f, "servicecatalog:TagResource"),
            ServicecatalogActions::TerminateProvisionedProduct => {
                write!(f, "servicecatalog:TerminateProvisionedProduct")
            }
            ServicecatalogActions::UntagResource => write!(f, "servicecatalog:UntagResource"),
            ServicecatalogActions::UpdateApplication => {
                write!(f, "servicecatalog:UpdateApplication")
            }
            ServicecatalogActions::UpdateAttributeGroup => {
                write!(f, "servicecatalog:UpdateAttributeGroup")
            }
            ServicecatalogActions::UpdateConstraint => write!(f, "servicecatalog:UpdateConstraint"),
            ServicecatalogActions::UpdatePortfolio => write!(f, "servicecatalog:UpdatePortfolio"),
            ServicecatalogActions::UpdatePortfolioShare => {
                write!(f, "servicecatalog:UpdatePortfolioShare")
            }
            ServicecatalogActions::UpdateProduct => write!(f, "servicecatalog:UpdateProduct"),
            ServicecatalogActions::UpdateProvisionedProduct => {
                write!(f, "servicecatalog:UpdateProvisionedProduct")
            }
            ServicecatalogActions::UpdateProvisionedProductProperties => {
                write!(f, "servicecatalog:UpdateProvisionedProductProperties")
            }
            ServicecatalogActions::UpdateProvisioningArtifact => {
                write!(f, "servicecatalog:UpdateProvisioningArtifact")
            }
            ServicecatalogActions::UpdateServiceAction => {
                write!(f, "servicecatalog:UpdateServiceAction")
            }
            ServicecatalogActions::UpdateTagOption => write!(f, "servicecatalog:UpdateTagOption"),
        }
    }
}
