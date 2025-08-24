// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum AwsMarketplaceActions {
    AcceptAgreementApprovalRequest,
    AcceptAgreementRequest,
    AssociateProductsWithPrivateMarketplace,
    BatchMeterUsage,
    CancelAgreement,
    CancelAgreementRequest,
    CancelChangeSet,
    CreateAgreementRequest,
    CreatePrivateMarketplaceRequests,
    DeleteResourcePolicy,
    DescribeAgreement,
    DescribeAssessment,
    DescribeBuilds,
    DescribeChangeSet,
    DescribeEntity,
    DescribePrivateMarketplaceRequests,
    DescribeProcurementSystemConfiguration,
    DisassociateProductsFromPrivateMarketplace,
    GetAgreementApprovalRequest,
    GetAgreementEntitlements,
    GetAgreementRequest,
    GetAgreementTerms,
    GetBuyerDashboard,
    GetEntitlements,
    GetResourcePolicy,
    GetSellerDashboard,
    ListAgreementApprovalRequests,
    ListAgreementCharges,
    ListAgreementRequests,
    ListAssessments,
    ListBuilds,
    ListChangeSets,
    ListEntities,
    ListEntitlementDetails,
    ListPrivateListings,
    ListPrivateMarketplaceRequests,
    ListTagsForResource,
    MeterUsage,
    PutDeploymentParameter,
    PutProcurementSystemConfiguration,
    PutResourcePolicy,
    RegisterUsage,
    RejectAgreementApprovalRequest,
    ResolveCustomer,
    SearchAgreements,
    StartBuild,
    StartChangeSet,
    Subscribe,
    TagResource,
    Unsubscribe,
    UntagResource,
    UpdateAgreementApprovalRequest,
    UpdatePurchaseOrders,
    ViewSubscriptions,
}
impl std::fmt::Display for AwsMarketplaceActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AwsMarketplaceActions::AcceptAgreementApprovalRequest => {
                write!(f, "aws-marketplace:AcceptAgreementApprovalRequest")
            }
            AwsMarketplaceActions::AcceptAgreementRequest => {
                write!(f, "aws-marketplace:AcceptAgreementRequest")
            }
            AwsMarketplaceActions::AssociateProductsWithPrivateMarketplace => {
                write!(f, "aws-marketplace:AssociateProductsWithPrivateMarketplace")
            }
            AwsMarketplaceActions::BatchMeterUsage => write!(f, "aws-marketplace:BatchMeterUsage"),
            AwsMarketplaceActions::CancelAgreement => write!(f, "aws-marketplace:CancelAgreement"),
            AwsMarketplaceActions::CancelAgreementRequest => {
                write!(f, "aws-marketplace:CancelAgreementRequest")
            }
            AwsMarketplaceActions::CancelChangeSet => write!(f, "aws-marketplace:CancelChangeSet"),
            AwsMarketplaceActions::CreateAgreementRequest => {
                write!(f, "aws-marketplace:CreateAgreementRequest")
            }
            AwsMarketplaceActions::CreatePrivateMarketplaceRequests => {
                write!(f, "aws-marketplace:CreatePrivateMarketplaceRequests")
            }
            AwsMarketplaceActions::DeleteResourcePolicy => {
                write!(f, "aws-marketplace:DeleteResourcePolicy")
            }
            AwsMarketplaceActions::DescribeAgreement => {
                write!(f, "aws-marketplace:DescribeAgreement")
            }
            AwsMarketplaceActions::DescribeAssessment => {
                write!(f, "aws-marketplace:DescribeAssessment")
            }
            AwsMarketplaceActions::DescribeBuilds => write!(f, "aws-marketplace:DescribeBuilds"),
            AwsMarketplaceActions::DescribeChangeSet => {
                write!(f, "aws-marketplace:DescribeChangeSet")
            }
            AwsMarketplaceActions::DescribeEntity => write!(f, "aws-marketplace:DescribeEntity"),
            AwsMarketplaceActions::DescribePrivateMarketplaceRequests => {
                write!(f, "aws-marketplace:DescribePrivateMarketplaceRequests")
            }
            AwsMarketplaceActions::DescribeProcurementSystemConfiguration => {
                write!(f, "aws-marketplace:DescribeProcurementSystemConfiguration")
            }
            AwsMarketplaceActions::DisassociateProductsFromPrivateMarketplace => write!(
                f,
                "aws-marketplace:DisassociateProductsFromPrivateMarketplace"
            ),
            AwsMarketplaceActions::GetAgreementApprovalRequest => {
                write!(f, "aws-marketplace:GetAgreementApprovalRequest")
            }
            AwsMarketplaceActions::GetAgreementEntitlements => {
                write!(f, "aws-marketplace:GetAgreementEntitlements")
            }
            AwsMarketplaceActions::GetAgreementRequest => {
                write!(f, "aws-marketplace:GetAgreementRequest")
            }
            AwsMarketplaceActions::GetAgreementTerms => {
                write!(f, "aws-marketplace:GetAgreementTerms")
            }
            AwsMarketplaceActions::GetBuyerDashboard => {
                write!(f, "aws-marketplace:GetBuyerDashboard")
            }
            AwsMarketplaceActions::GetEntitlements => write!(f, "aws-marketplace:GetEntitlements"),
            AwsMarketplaceActions::GetResourcePolicy => {
                write!(f, "aws-marketplace:GetResourcePolicy")
            }
            AwsMarketplaceActions::GetSellerDashboard => {
                write!(f, "aws-marketplace:GetSellerDashboard")
            }
            AwsMarketplaceActions::ListAgreementApprovalRequests => {
                write!(f, "aws-marketplace:ListAgreementApprovalRequests")
            }
            AwsMarketplaceActions::ListAgreementCharges => {
                write!(f, "aws-marketplace:ListAgreementCharges")
            }
            AwsMarketplaceActions::ListAgreementRequests => {
                write!(f, "aws-marketplace:ListAgreementRequests")
            }
            AwsMarketplaceActions::ListAssessments => write!(f, "aws-marketplace:ListAssessments"),
            AwsMarketplaceActions::ListBuilds => write!(f, "aws-marketplace:ListBuilds"),
            AwsMarketplaceActions::ListChangeSets => write!(f, "aws-marketplace:ListChangeSets"),
            AwsMarketplaceActions::ListEntities => write!(f, "aws-marketplace:ListEntities"),
            AwsMarketplaceActions::ListEntitlementDetails => {
                write!(f, "aws-marketplace:ListEntitlementDetails")
            }
            AwsMarketplaceActions::ListPrivateListings => {
                write!(f, "aws-marketplace:ListPrivateListings")
            }
            AwsMarketplaceActions::ListPrivateMarketplaceRequests => {
                write!(f, "aws-marketplace:ListPrivateMarketplaceRequests")
            }
            AwsMarketplaceActions::ListTagsForResource => {
                write!(f, "aws-marketplace:ListTagsForResource")
            }
            AwsMarketplaceActions::MeterUsage => write!(f, "aws-marketplace:MeterUsage"),
            AwsMarketplaceActions::PutDeploymentParameter => {
                write!(f, "aws-marketplace:PutDeploymentParameter")
            }
            AwsMarketplaceActions::PutProcurementSystemConfiguration => {
                write!(f, "aws-marketplace:PutProcurementSystemConfiguration")
            }
            AwsMarketplaceActions::PutResourcePolicy => {
                write!(f, "aws-marketplace:PutResourcePolicy")
            }
            AwsMarketplaceActions::RegisterUsage => write!(f, "aws-marketplace:RegisterUsage"),
            AwsMarketplaceActions::RejectAgreementApprovalRequest => {
                write!(f, "aws-marketplace:RejectAgreementApprovalRequest")
            }
            AwsMarketplaceActions::ResolveCustomer => write!(f, "aws-marketplace:ResolveCustomer"),
            AwsMarketplaceActions::SearchAgreements => {
                write!(f, "aws-marketplace:SearchAgreements")
            }
            AwsMarketplaceActions::StartBuild => write!(f, "aws-marketplace:StartBuild"),
            AwsMarketplaceActions::StartChangeSet => write!(f, "aws-marketplace:StartChangeSet"),
            AwsMarketplaceActions::Subscribe => write!(f, "aws-marketplace:Subscribe"),
            AwsMarketplaceActions::TagResource => write!(f, "aws-marketplace:TagResource"),
            AwsMarketplaceActions::Unsubscribe => write!(f, "aws-marketplace:Unsubscribe"),
            AwsMarketplaceActions::UntagResource => write!(f, "aws-marketplace:UntagResource"),
            AwsMarketplaceActions::UpdateAgreementApprovalRequest => {
                write!(f, "aws-marketplace:UpdateAgreementApprovalRequest")
            }
            AwsMarketplaceActions::UpdatePurchaseOrders => {
                write!(f, "aws-marketplace:UpdatePurchaseOrders")
            }
            AwsMarketplaceActions::ViewSubscriptions => {
                write!(f, "aws-marketplace:ViewSubscriptions")
            }
        }
    }
}
