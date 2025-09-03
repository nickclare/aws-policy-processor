// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum ControltowerActions {
    CreateLandingZone,
    CreateManagedAccount,
    DeleteLandingZone,
    DeregisterManagedAccount,
    DeregisterOrganizationalUnit,
    DescribeAccountFactoryConfig,
    DescribeCoreService,
    DescribeGuardrail,
    DescribeGuardrailForTarget,
    DescribeLandingZoneConfiguration,
    DescribeManagedAccount,
    DescribeManagedOrganizationalUnit,
    DescribeRegisterOrganizationalUnitOperation,
    DescribeSingleSignOn,
    DisableBaseline,
    DisableControl,
    DisableGuardrail,
    EnableBaseline,
    EnableControl,
    EnableGuardrail,
    GetAccountInfo,
    GetAvailableUpdates,
    GetBaseline,
    GetBaselineOperation,
    GetControlOperation,
    GetEnabledBaseline,
    GetEnabledControl,
    GetGuardrailComplianceStatus,
    GetHomeRegion,
    GetLandingZone,
    GetLandingZoneDriftStatus,
    GetLandingZoneOperation,
    GetLandingZoneStatus,
    ListBaselines,
    ListControlOperations,
    ListDirectoryGroups,
    ListDriftDetails,
    ListEnabledBaselines,
    ListEnabledControls,
    ListEnabledGuardrails,
    ListExtendGovernancePrecheckDetails,
    ListExternalConfigRuleCompliance,
    ListGuardrailViolations,
    ListGuardrails,
    ListGuardrailsForTarget,
    ListLandingZoneOperations,
    ListLandingZones,
    ListManagedAccounts,
    ListManagedAccountsForGuardrail,
    ListManagedAccountsForParent,
    ListManagedOrganizationalUnits,
    ListManagedOrganizationalUnitsForGuardrail,
    ListTagsForResource,
    ManageOrganizationalUnit,
    PerformPreLaunchChecks,
    ResetEnabledBaseline,
    ResetEnabledControl,
    ResetLandingZone,
    SetupLandingZone,
    TagResource,
    UntagResource,
    UpdateAccountFactoryConfig,
    UpdateEnabledBaseline,
    UpdateEnabledControl,
    UpdateLandingZone,
}
impl std::fmt::Display for ControltowerActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ControltowerActions::CreateLandingZone => write!(f, "controltower:CreateLandingZone"),
            ControltowerActions::CreateManagedAccount => {
                write!(f, "controltower:CreateManagedAccount")
            }
            ControltowerActions::DeleteLandingZone => write!(f, "controltower:DeleteLandingZone"),
            ControltowerActions::DeregisterManagedAccount => {
                write!(f, "controltower:DeregisterManagedAccount")
            }
            ControltowerActions::DeregisterOrganizationalUnit => {
                write!(f, "controltower:DeregisterOrganizationalUnit")
            }
            ControltowerActions::DescribeAccountFactoryConfig => {
                write!(f, "controltower:DescribeAccountFactoryConfig")
            }
            ControltowerActions::DescribeCoreService => {
                write!(f, "controltower:DescribeCoreService")
            }
            ControltowerActions::DescribeGuardrail => write!(f, "controltower:DescribeGuardrail"),
            ControltowerActions::DescribeGuardrailForTarget => {
                write!(f, "controltower:DescribeGuardrailForTarget")
            }
            ControltowerActions::DescribeLandingZoneConfiguration => {
                write!(f, "controltower:DescribeLandingZoneConfiguration")
            }
            ControltowerActions::DescribeManagedAccount => {
                write!(f, "controltower:DescribeManagedAccount")
            }
            ControltowerActions::DescribeManagedOrganizationalUnit => {
                write!(f, "controltower:DescribeManagedOrganizationalUnit")
            }
            ControltowerActions::DescribeRegisterOrganizationalUnitOperation => write!(
                f,
                "controltower:DescribeRegisterOrganizationalUnitOperation"
            ),
            ControltowerActions::DescribeSingleSignOn => {
                write!(f, "controltower:DescribeSingleSignOn")
            }
            ControltowerActions::DisableBaseline => write!(f, "controltower:DisableBaseline"),
            ControltowerActions::DisableControl => write!(f, "controltower:DisableControl"),
            ControltowerActions::DisableGuardrail => write!(f, "controltower:DisableGuardrail"),
            ControltowerActions::EnableBaseline => write!(f, "controltower:EnableBaseline"),
            ControltowerActions::EnableControl => write!(f, "controltower:EnableControl"),
            ControltowerActions::EnableGuardrail => write!(f, "controltower:EnableGuardrail"),
            ControltowerActions::GetAccountInfo => write!(f, "controltower:GetAccountInfo"),
            ControltowerActions::GetAvailableUpdates => {
                write!(f, "controltower:GetAvailableUpdates")
            }
            ControltowerActions::GetBaseline => write!(f, "controltower:GetBaseline"),
            ControltowerActions::GetBaselineOperation => {
                write!(f, "controltower:GetBaselineOperation")
            }
            ControltowerActions::GetControlOperation => {
                write!(f, "controltower:GetControlOperation")
            }
            ControltowerActions::GetEnabledBaseline => write!(f, "controltower:GetEnabledBaseline"),
            ControltowerActions::GetEnabledControl => write!(f, "controltower:GetEnabledControl"),
            ControltowerActions::GetGuardrailComplianceStatus => {
                write!(f, "controltower:GetGuardrailComplianceStatus")
            }
            ControltowerActions::GetHomeRegion => write!(f, "controltower:GetHomeRegion"),
            ControltowerActions::GetLandingZone => write!(f, "controltower:GetLandingZone"),
            ControltowerActions::GetLandingZoneDriftStatus => {
                write!(f, "controltower:GetLandingZoneDriftStatus")
            }
            ControltowerActions::GetLandingZoneOperation => {
                write!(f, "controltower:GetLandingZoneOperation")
            }
            ControltowerActions::GetLandingZoneStatus => {
                write!(f, "controltower:GetLandingZoneStatus")
            }
            ControltowerActions::ListBaselines => write!(f, "controltower:ListBaselines"),
            ControltowerActions::ListControlOperations => {
                write!(f, "controltower:ListControlOperations")
            }
            ControltowerActions::ListDirectoryGroups => {
                write!(f, "controltower:ListDirectoryGroups")
            }
            ControltowerActions::ListDriftDetails => write!(f, "controltower:ListDriftDetails"),
            ControltowerActions::ListEnabledBaselines => {
                write!(f, "controltower:ListEnabledBaselines")
            }
            ControltowerActions::ListEnabledControls => {
                write!(f, "controltower:ListEnabledControls")
            }
            ControltowerActions::ListEnabledGuardrails => {
                write!(f, "controltower:ListEnabledGuardrails")
            }
            ControltowerActions::ListExtendGovernancePrecheckDetails => {
                write!(f, "controltower:ListExtendGovernancePrecheckDetails")
            }
            ControltowerActions::ListExternalConfigRuleCompliance => {
                write!(f, "controltower:ListExternalConfigRuleCompliance")
            }
            ControltowerActions::ListGuardrailViolations => {
                write!(f, "controltower:ListGuardrailViolations")
            }
            ControltowerActions::ListGuardrails => write!(f, "controltower:ListGuardrails"),
            ControltowerActions::ListGuardrailsForTarget => {
                write!(f, "controltower:ListGuardrailsForTarget")
            }
            ControltowerActions::ListLandingZoneOperations => {
                write!(f, "controltower:ListLandingZoneOperations")
            }
            ControltowerActions::ListLandingZones => write!(f, "controltower:ListLandingZones"),
            ControltowerActions::ListManagedAccounts => {
                write!(f, "controltower:ListManagedAccounts")
            }
            ControltowerActions::ListManagedAccountsForGuardrail => {
                write!(f, "controltower:ListManagedAccountsForGuardrail")
            }
            ControltowerActions::ListManagedAccountsForParent => {
                write!(f, "controltower:ListManagedAccountsForParent")
            }
            ControltowerActions::ListManagedOrganizationalUnits => {
                write!(f, "controltower:ListManagedOrganizationalUnits")
            }
            ControltowerActions::ListManagedOrganizationalUnitsForGuardrail => {
                write!(f, "controltower:ListManagedOrganizationalUnitsForGuardrail")
            }
            ControltowerActions::ListTagsForResource => {
                write!(f, "controltower:ListTagsForResource")
            }
            ControltowerActions::ManageOrganizationalUnit => {
                write!(f, "controltower:ManageOrganizationalUnit")
            }
            ControltowerActions::PerformPreLaunchChecks => {
                write!(f, "controltower:PerformPreLaunchChecks")
            }
            ControltowerActions::ResetEnabledBaseline => {
                write!(f, "controltower:ResetEnabledBaseline")
            }
            ControltowerActions::ResetEnabledControl => {
                write!(f, "controltower:ResetEnabledControl")
            }
            ControltowerActions::ResetLandingZone => write!(f, "controltower:ResetLandingZone"),
            ControltowerActions::SetupLandingZone => write!(f, "controltower:SetupLandingZone"),
            ControltowerActions::TagResource => write!(f, "controltower:TagResource"),
            ControltowerActions::UntagResource => write!(f, "controltower:UntagResource"),
            ControltowerActions::UpdateAccountFactoryConfig => {
                write!(f, "controltower:UpdateAccountFactoryConfig")
            }
            ControltowerActions::UpdateEnabledBaseline => {
                write!(f, "controltower:UpdateEnabledBaseline")
            }
            ControltowerActions::UpdateEnabledControl => {
                write!(f, "controltower:UpdateEnabledControl")
            }
            ControltowerActions::UpdateLandingZone => write!(f, "controltower:UpdateLandingZone"),
        }
    }
}
