// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum LicenseManagerActions {
    AcceptGrant,
    CheckInLicense,
    CheckoutBorrowLicense,
    CheckoutLicense,
    CreateGrant,
    CreateGrantVersion,
    CreateLicense,
    CreateLicenseConfiguration,
    CreateLicenseConversionTaskForResource,
    CreateLicenseManagerReportGenerator,
    CreateLicenseVersion,
    CreateToken,
    DeleteGrant,
    DeleteLicense,
    DeleteLicenseConfiguration,
    DeleteLicenseManagerReportGenerator,
    DeleteToken,
    ExtendLicenseConsumption,
    GetAccessToken,
    GetGrant,
    GetLicense,
    GetLicenseConfiguration,
    GetLicenseConversionTask,
    GetLicenseManagerReportGenerator,
    GetLicenseUsage,
    GetServiceSettings,
    ListAssociationsForLicenseConfiguration,
    ListDistributedGrants,
    ListFailuresForLicenseConfigurationOperations,
    ListLicenseConfigurations,
    ListLicenseConversionTasks,
    ListLicenseManagerReportGenerators,
    ListLicenseSpecificationsForResource,
    ListLicenseVersions,
    ListLicenses,
    ListReceivedGrants,
    ListReceivedGrantsForOrganization,
    ListReceivedLicenses,
    ListReceivedLicensesForOrganization,
    ListResourceInventory,
    ListTagsForResource,
    ListTokens,
    ListUsageForLicenseConfiguration,
    RejectGrant,
    TagResource,
    UntagResource,
    UpdateLicenseConfiguration,
    UpdateLicenseManagerReportGenerator,
    UpdateLicenseSpecificationsForResource,
    UpdateServiceSettings,
}
impl std::fmt::Display for LicenseManagerActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LicenseManagerActions::AcceptGrant => write!(f, "license-manager:AcceptGrant"),
            LicenseManagerActions::CheckInLicense => write!(f, "license-manager:CheckInLicense"),
            LicenseManagerActions::CheckoutBorrowLicense => {
                write!(f, "license-manager:CheckoutBorrowLicense")
            }
            LicenseManagerActions::CheckoutLicense => write!(f, "license-manager:CheckoutLicense"),
            LicenseManagerActions::CreateGrant => write!(f, "license-manager:CreateGrant"),
            LicenseManagerActions::CreateGrantVersion => {
                write!(f, "license-manager:CreateGrantVersion")
            }
            LicenseManagerActions::CreateLicense => write!(f, "license-manager:CreateLicense"),
            LicenseManagerActions::CreateLicenseConfiguration => {
                write!(f, "license-manager:CreateLicenseConfiguration")
            }
            LicenseManagerActions::CreateLicenseConversionTaskForResource => {
                write!(f, "license-manager:CreateLicenseConversionTaskForResource")
            }
            LicenseManagerActions::CreateLicenseManagerReportGenerator => {
                write!(f, "license-manager:CreateLicenseManagerReportGenerator")
            }
            LicenseManagerActions::CreateLicenseVersion => {
                write!(f, "license-manager:CreateLicenseVersion")
            }
            LicenseManagerActions::CreateToken => write!(f, "license-manager:CreateToken"),
            LicenseManagerActions::DeleteGrant => write!(f, "license-manager:DeleteGrant"),
            LicenseManagerActions::DeleteLicense => write!(f, "license-manager:DeleteLicense"),
            LicenseManagerActions::DeleteLicenseConfiguration => {
                write!(f, "license-manager:DeleteLicenseConfiguration")
            }
            LicenseManagerActions::DeleteLicenseManagerReportGenerator => {
                write!(f, "license-manager:DeleteLicenseManagerReportGenerator")
            }
            LicenseManagerActions::DeleteToken => write!(f, "license-manager:DeleteToken"),
            LicenseManagerActions::ExtendLicenseConsumption => {
                write!(f, "license-manager:ExtendLicenseConsumption")
            }
            LicenseManagerActions::GetAccessToken => write!(f, "license-manager:GetAccessToken"),
            LicenseManagerActions::GetGrant => write!(f, "license-manager:GetGrant"),
            LicenseManagerActions::GetLicense => write!(f, "license-manager:GetLicense"),
            LicenseManagerActions::GetLicenseConfiguration => {
                write!(f, "license-manager:GetLicenseConfiguration")
            }
            LicenseManagerActions::GetLicenseConversionTask => {
                write!(f, "license-manager:GetLicenseConversionTask")
            }
            LicenseManagerActions::GetLicenseManagerReportGenerator => {
                write!(f, "license-manager:GetLicenseManagerReportGenerator")
            }
            LicenseManagerActions::GetLicenseUsage => write!(f, "license-manager:GetLicenseUsage"),
            LicenseManagerActions::GetServiceSettings => {
                write!(f, "license-manager:GetServiceSettings")
            }
            LicenseManagerActions::ListAssociationsForLicenseConfiguration => {
                write!(f, "license-manager:ListAssociationsForLicenseConfiguration")
            }
            LicenseManagerActions::ListDistributedGrants => {
                write!(f, "license-manager:ListDistributedGrants")
            }
            LicenseManagerActions::ListFailuresForLicenseConfigurationOperations => write!(
                f,
                "license-manager:ListFailuresForLicenseConfigurationOperations"
            ),
            LicenseManagerActions::ListLicenseConfigurations => {
                write!(f, "license-manager:ListLicenseConfigurations")
            }
            LicenseManagerActions::ListLicenseConversionTasks => {
                write!(f, "license-manager:ListLicenseConversionTasks")
            }
            LicenseManagerActions::ListLicenseManagerReportGenerators => {
                write!(f, "license-manager:ListLicenseManagerReportGenerators")
            }
            LicenseManagerActions::ListLicenseSpecificationsForResource => {
                write!(f, "license-manager:ListLicenseSpecificationsForResource")
            }
            LicenseManagerActions::ListLicenseVersions => {
                write!(f, "license-manager:ListLicenseVersions")
            }
            LicenseManagerActions::ListLicenses => write!(f, "license-manager:ListLicenses"),
            LicenseManagerActions::ListReceivedGrants => {
                write!(f, "license-manager:ListReceivedGrants")
            }
            LicenseManagerActions::ListReceivedGrantsForOrganization => {
                write!(f, "license-manager:ListReceivedGrantsForOrganization")
            }
            LicenseManagerActions::ListReceivedLicenses => {
                write!(f, "license-manager:ListReceivedLicenses")
            }
            LicenseManagerActions::ListReceivedLicensesForOrganization => {
                write!(f, "license-manager:ListReceivedLicensesForOrganization")
            }
            LicenseManagerActions::ListResourceInventory => {
                write!(f, "license-manager:ListResourceInventory")
            }
            LicenseManagerActions::ListTagsForResource => {
                write!(f, "license-manager:ListTagsForResource")
            }
            LicenseManagerActions::ListTokens => write!(f, "license-manager:ListTokens"),
            LicenseManagerActions::ListUsageForLicenseConfiguration => {
                write!(f, "license-manager:ListUsageForLicenseConfiguration")
            }
            LicenseManagerActions::RejectGrant => write!(f, "license-manager:RejectGrant"),
            LicenseManagerActions::TagResource => write!(f, "license-manager:TagResource"),
            LicenseManagerActions::UntagResource => write!(f, "license-manager:UntagResource"),
            LicenseManagerActions::UpdateLicenseConfiguration => {
                write!(f, "license-manager:UpdateLicenseConfiguration")
            }
            LicenseManagerActions::UpdateLicenseManagerReportGenerator => {
                write!(f, "license-manager:UpdateLicenseManagerReportGenerator")
            }
            LicenseManagerActions::UpdateLicenseSpecificationsForResource => {
                write!(f, "license-manager:UpdateLicenseSpecificationsForResource")
            }
            LicenseManagerActions::UpdateServiceSettings => {
                write!(f, "license-manager:UpdateServiceSettings")
            }
        }
    }
}
