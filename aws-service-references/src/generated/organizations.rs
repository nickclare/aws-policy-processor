// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum OrganizationsActions {
    AcceptHandshake,
    AttachPolicy,
    CancelHandshake,
    CloseAccount,
    CreateAccount,
    CreateGovCloudAccount,
    CreateOrganization,
    CreateOrganizationalUnit,
    CreatePolicy,
    DeclineHandshake,
    DeleteOrganization,
    DeleteOrganizationalUnit,
    DeletePolicy,
    DeleteResourcePolicy,
    DeregisterDelegatedAdministrator,
    DescribeAccount,
    DescribeCreateAccountStatus,
    DescribeEffectivePolicy,
    DescribeHandshake,
    DescribeOrganization,
    DescribeOrganizationalUnit,
    DescribePolicy,
    DescribeResourcePolicy,
    DetachPolicy,
    DisableAwsServiceAccess,
    DisablePolicyType,
    EnableAllFeatures,
    EnableAwsServiceAccess,
    EnablePolicyType,
    InviteAccountToOrganization,
    LeaveOrganization,
    ListAccounts,
    ListAccountsForParent,
    ListAwsServiceAccessForOrganization,
    ListChildren,
    ListCreateAccountStatus,
    ListDelegatedAdministrators,
    ListDelegatedServicesForAccount,
    ListHandshakesForAccount,
    ListHandshakesForOrganization,
    ListOrganizationalUnitsForParent,
    ListParents,
    ListPolicies,
    ListPoliciesForTarget,
    ListRoots,
    ListTagsForResource,
    ListTargetsForPolicy,
    MoveAccount,
    PutResourcePolicy,
    RegisterDelegatedAdministrator,
    RemoveAccountFromOrganization,
    TagResource,
    UntagResource,
    UpdateOrganizationalUnit,
    UpdatePolicy,
}
impl std::fmt::Display for OrganizationsActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrganizationsActions::AcceptHandshake => write!(f, "organizations:AcceptHandshake"),
            OrganizationsActions::AttachPolicy => write!(f, "organizations:AttachPolicy"),
            OrganizationsActions::CancelHandshake => write!(f, "organizations:CancelHandshake"),
            OrganizationsActions::CloseAccount => write!(f, "organizations:CloseAccount"),
            OrganizationsActions::CreateAccount => write!(f, "organizations:CreateAccount"),
            OrganizationsActions::CreateGovCloudAccount => {
                write!(f, "organizations:CreateGovCloudAccount")
            }
            OrganizationsActions::CreateOrganization => {
                write!(f, "organizations:CreateOrganization")
            }
            OrganizationsActions::CreateOrganizationalUnit => {
                write!(f, "organizations:CreateOrganizationalUnit")
            }
            OrganizationsActions::CreatePolicy => write!(f, "organizations:CreatePolicy"),
            OrganizationsActions::DeclineHandshake => write!(f, "organizations:DeclineHandshake"),
            OrganizationsActions::DeleteOrganization => {
                write!(f, "organizations:DeleteOrganization")
            }
            OrganizationsActions::DeleteOrganizationalUnit => {
                write!(f, "organizations:DeleteOrganizationalUnit")
            }
            OrganizationsActions::DeletePolicy => write!(f, "organizations:DeletePolicy"),
            OrganizationsActions::DeleteResourcePolicy => {
                write!(f, "organizations:DeleteResourcePolicy")
            }
            OrganizationsActions::DeregisterDelegatedAdministrator => {
                write!(f, "organizations:DeregisterDelegatedAdministrator")
            }
            OrganizationsActions::DescribeAccount => write!(f, "organizations:DescribeAccount"),
            OrganizationsActions::DescribeCreateAccountStatus => {
                write!(f, "organizations:DescribeCreateAccountStatus")
            }
            OrganizationsActions::DescribeEffectivePolicy => {
                write!(f, "organizations:DescribeEffectivePolicy")
            }
            OrganizationsActions::DescribeHandshake => write!(f, "organizations:DescribeHandshake"),
            OrganizationsActions::DescribeOrganization => {
                write!(f, "organizations:DescribeOrganization")
            }
            OrganizationsActions::DescribeOrganizationalUnit => {
                write!(f, "organizations:DescribeOrganizationalUnit")
            }
            OrganizationsActions::DescribePolicy => write!(f, "organizations:DescribePolicy"),
            OrganizationsActions::DescribeResourcePolicy => {
                write!(f, "organizations:DescribeResourcePolicy")
            }
            OrganizationsActions::DetachPolicy => write!(f, "organizations:DetachPolicy"),
            OrganizationsActions::DisableAwsServiceAccess => {
                write!(f, "organizations:DisableAWSServiceAccess")
            }
            OrganizationsActions::DisablePolicyType => write!(f, "organizations:DisablePolicyType"),
            OrganizationsActions::EnableAllFeatures => write!(f, "organizations:EnableAllFeatures"),
            OrganizationsActions::EnableAwsServiceAccess => {
                write!(f, "organizations:EnableAWSServiceAccess")
            }
            OrganizationsActions::EnablePolicyType => write!(f, "organizations:EnablePolicyType"),
            OrganizationsActions::InviteAccountToOrganization => {
                write!(f, "organizations:InviteAccountToOrganization")
            }
            OrganizationsActions::LeaveOrganization => write!(f, "organizations:LeaveOrganization"),
            OrganizationsActions::ListAccounts => write!(f, "organizations:ListAccounts"),
            OrganizationsActions::ListAccountsForParent => {
                write!(f, "organizations:ListAccountsForParent")
            }
            OrganizationsActions::ListAwsServiceAccessForOrganization => {
                write!(f, "organizations:ListAWSServiceAccessForOrganization")
            }
            OrganizationsActions::ListChildren => write!(f, "organizations:ListChildren"),
            OrganizationsActions::ListCreateAccountStatus => {
                write!(f, "organizations:ListCreateAccountStatus")
            }
            OrganizationsActions::ListDelegatedAdministrators => {
                write!(f, "organizations:ListDelegatedAdministrators")
            }
            OrganizationsActions::ListDelegatedServicesForAccount => {
                write!(f, "organizations:ListDelegatedServicesForAccount")
            }
            OrganizationsActions::ListHandshakesForAccount => {
                write!(f, "organizations:ListHandshakesForAccount")
            }
            OrganizationsActions::ListHandshakesForOrganization => {
                write!(f, "organizations:ListHandshakesForOrganization")
            }
            OrganizationsActions::ListOrganizationalUnitsForParent => {
                write!(f, "organizations:ListOrganizationalUnitsForParent")
            }
            OrganizationsActions::ListParents => write!(f, "organizations:ListParents"),
            OrganizationsActions::ListPolicies => write!(f, "organizations:ListPolicies"),
            OrganizationsActions::ListPoliciesForTarget => {
                write!(f, "organizations:ListPoliciesForTarget")
            }
            OrganizationsActions::ListRoots => write!(f, "organizations:ListRoots"),
            OrganizationsActions::ListTagsForResource => {
                write!(f, "organizations:ListTagsForResource")
            }
            OrganizationsActions::ListTargetsForPolicy => {
                write!(f, "organizations:ListTargetsForPolicy")
            }
            OrganizationsActions::MoveAccount => write!(f, "organizations:MoveAccount"),
            OrganizationsActions::PutResourcePolicy => write!(f, "organizations:PutResourcePolicy"),
            OrganizationsActions::RegisterDelegatedAdministrator => {
                write!(f, "organizations:RegisterDelegatedAdministrator")
            }
            OrganizationsActions::RemoveAccountFromOrganization => {
                write!(f, "organizations:RemoveAccountFromOrganization")
            }
            OrganizationsActions::TagResource => write!(f, "organizations:TagResource"),
            OrganizationsActions::UntagResource => write!(f, "organizations:UntagResource"),
            OrganizationsActions::UpdateOrganizationalUnit => {
                write!(f, "organizations:UpdateOrganizationalUnit")
            }
            OrganizationsActions::UpdatePolicy => write!(f, "organizations:UpdatePolicy"),
        }
    }
}
