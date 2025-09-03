// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum AwsMarketplaceManagementActions {
    GetAdditionalSellerNotificationRecipients,
    GetBankAccountVerificationDetails,
    GetSecondaryUserVerificationDetails,
    GetSellerVerificationDetails,
    PutAdditionalSellerNotificationRecipients,
    PutBankAccountVerificationDetails,
    PutSecondaryUserVerificationDetails,
    PutSellerVerificationDetails,
    UploadFiles,
    ViewMarketing,
    ViewReports,
    ViewSettings,
    ViewSupport,
}
impl std::fmt::Display for AwsMarketplaceManagementActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AwsMarketplaceManagementActions::GetAdditionalSellerNotificationRecipients => write!(
                f,
                "aws-marketplace-management:GetAdditionalSellerNotificationRecipients"
            ),
            AwsMarketplaceManagementActions::GetBankAccountVerificationDetails => write!(
                f,
                "aws-marketplace-management:GetBankAccountVerificationDetails"
            ),
            AwsMarketplaceManagementActions::GetSecondaryUserVerificationDetails => write!(
                f,
                "aws-marketplace-management:GetSecondaryUserVerificationDetails"
            ),
            AwsMarketplaceManagementActions::GetSellerVerificationDetails => {
                write!(f, "aws-marketplace-management:GetSellerVerificationDetails")
            }
            AwsMarketplaceManagementActions::PutAdditionalSellerNotificationRecipients => write!(
                f,
                "aws-marketplace-management:PutAdditionalSellerNotificationRecipients"
            ),
            AwsMarketplaceManagementActions::PutBankAccountVerificationDetails => write!(
                f,
                "aws-marketplace-management:PutBankAccountVerificationDetails"
            ),
            AwsMarketplaceManagementActions::PutSecondaryUserVerificationDetails => write!(
                f,
                "aws-marketplace-management:PutSecondaryUserVerificationDetails"
            ),
            AwsMarketplaceManagementActions::PutSellerVerificationDetails => {
                write!(f, "aws-marketplace-management:PutSellerVerificationDetails")
            }
            AwsMarketplaceManagementActions::UploadFiles => {
                write!(f, "aws-marketplace-management:uploadFiles")
            }
            AwsMarketplaceManagementActions::ViewMarketing => {
                write!(f, "aws-marketplace-management:viewMarketing")
            }
            AwsMarketplaceManagementActions::ViewReports => {
                write!(f, "aws-marketplace-management:viewReports")
            }
            AwsMarketplaceManagementActions::ViewSettings => {
                write!(f, "aws-marketplace-management:viewSettings")
            }
            AwsMarketplaceManagementActions::ViewSupport => {
                write!(f, "aws-marketplace-management:viewSupport")
            }
        }
    }
}
