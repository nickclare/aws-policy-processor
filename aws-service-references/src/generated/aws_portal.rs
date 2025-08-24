// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum AwsPortalActions {
    GetConsoleActionSetEnforced,
    ModifyAccount,
    ModifyBilling,
    ModifyPaymentMethods,
    UpdateConsoleActionSetEnforced,
    ViewAccount,
    ViewBilling,
    ViewPaymentMethods,
    ViewUsage,
}
impl std::fmt::Display for AwsPortalActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AwsPortalActions::GetConsoleActionSetEnforced => {
                write!(f, "aws-portal:GetConsoleActionSetEnforced")
            }
            AwsPortalActions::ModifyAccount => write!(f, "aws-portal:ModifyAccount"),
            AwsPortalActions::ModifyBilling => write!(f, "aws-portal:ModifyBilling"),
            AwsPortalActions::ModifyPaymentMethods => write!(f, "aws-portal:ModifyPaymentMethods"),
            AwsPortalActions::UpdateConsoleActionSetEnforced => {
                write!(f, "aws-portal:UpdateConsoleActionSetEnforced")
            }
            AwsPortalActions::ViewAccount => write!(f, "aws-portal:ViewAccount"),
            AwsPortalActions::ViewBilling => write!(f, "aws-portal:ViewBilling"),
            AwsPortalActions::ViewPaymentMethods => write!(f, "aws-portal:ViewPaymentMethods"),
            AwsPortalActions::ViewUsage => write!(f, "aws-portal:ViewUsage"),
        }
    }
}
