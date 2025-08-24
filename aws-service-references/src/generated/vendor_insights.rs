// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum VendorInsightsActions {
    ActivateSecurityProfile,
    AssociateDataSource,
    CreateDataSource,
    CreateSecurityProfile,
    DeactivateSecurityProfile,
    DeleteDataSource,
    DisassociateDataSource,
    GetDataSource,
    GetEntitledSecurityProfileSnapshot,
    GetProfileAccessTerms,
    GetSecurityProfile,
    GetSecurityProfileSnapshot,
    ListDataSources,
    ListEntitledSecurityProfileSnapshots,
    ListEntitledSecurityProfiles,
    ListSecurityProfileSnapshots,
    ListSecurityProfiles,
    ListTagsForResource,
    TagResource,
    UntagResource,
    UpdateDataSource,
    UpdateSecurityProfile,
    UpdateSecurityProfileSnapshotCreationConfiguration,
    UpdateSecurityProfileSnapshotReleaseConfiguration,
}
impl std::fmt::Display for VendorInsightsActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VendorInsightsActions::ActivateSecurityProfile => {
                write!(f, "vendor-insights:ActivateSecurityProfile")
            }
            VendorInsightsActions::AssociateDataSource => {
                write!(f, "vendor-insights:AssociateDataSource")
            }
            VendorInsightsActions::CreateDataSource => {
                write!(f, "vendor-insights:CreateDataSource")
            }
            VendorInsightsActions::CreateSecurityProfile => {
                write!(f, "vendor-insights:CreateSecurityProfile")
            }
            VendorInsightsActions::DeactivateSecurityProfile => {
                write!(f, "vendor-insights:DeactivateSecurityProfile")
            }
            VendorInsightsActions::DeleteDataSource => {
                write!(f, "vendor-insights:DeleteDataSource")
            }
            VendorInsightsActions::DisassociateDataSource => {
                write!(f, "vendor-insights:DisassociateDataSource")
            }
            VendorInsightsActions::GetDataSource => write!(f, "vendor-insights:GetDataSource"),
            VendorInsightsActions::GetEntitledSecurityProfileSnapshot => {
                write!(f, "vendor-insights:GetEntitledSecurityProfileSnapshot")
            }
            VendorInsightsActions::GetProfileAccessTerms => {
                write!(f, "vendor-insights:GetProfileAccessTerms")
            }
            VendorInsightsActions::GetSecurityProfile => {
                write!(f, "vendor-insights:GetSecurityProfile")
            }
            VendorInsightsActions::GetSecurityProfileSnapshot => {
                write!(f, "vendor-insights:GetSecurityProfileSnapshot")
            }
            VendorInsightsActions::ListDataSources => write!(f, "vendor-insights:ListDataSources"),
            VendorInsightsActions::ListEntitledSecurityProfileSnapshots => {
                write!(f, "vendor-insights:ListEntitledSecurityProfileSnapshots")
            }
            VendorInsightsActions::ListEntitledSecurityProfiles => {
                write!(f, "vendor-insights:ListEntitledSecurityProfiles")
            }
            VendorInsightsActions::ListSecurityProfileSnapshots => {
                write!(f, "vendor-insights:ListSecurityProfileSnapshots")
            }
            VendorInsightsActions::ListSecurityProfiles => {
                write!(f, "vendor-insights:ListSecurityProfiles")
            }
            VendorInsightsActions::ListTagsForResource => {
                write!(f, "vendor-insights:ListTagsForResource")
            }
            VendorInsightsActions::TagResource => write!(f, "vendor-insights:TagResource"),
            VendorInsightsActions::UntagResource => write!(f, "vendor-insights:UntagResource"),
            VendorInsightsActions::UpdateDataSource => {
                write!(f, "vendor-insights:UpdateDataSource")
            }
            VendorInsightsActions::UpdateSecurityProfile => {
                write!(f, "vendor-insights:UpdateSecurityProfile")
            }
            VendorInsightsActions::UpdateSecurityProfileSnapshotCreationConfiguration => write!(
                f,
                "vendor-insights:UpdateSecurityProfileSnapshotCreationConfiguration"
            ),
            VendorInsightsActions::UpdateSecurityProfileSnapshotReleaseConfiguration => write!(
                f,
                "vendor-insights:UpdateSecurityProfileSnapshotReleaseConfiguration"
            ),
        }
    }
}
