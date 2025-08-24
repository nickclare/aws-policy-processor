// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum VerifiedpermissionsActions {
    CreateIdentitySource,
    CreatePolicy,
    CreatePolicyStore,
    CreatePolicyTemplate,
    DeleteIdentitySource,
    DeletePolicy,
    DeletePolicyStore,
    DeletePolicyTemplate,
    GetIdentitySource,
    GetPolicy,
    GetPolicyStore,
    GetPolicyTemplate,
    GetSchema,
    IsAuthorized,
    IsAuthorizedWithToken,
    ListIdentitySources,
    ListPolicies,
    ListPolicyStores,
    ListPolicyTemplates,
    ListTagsForResource,
    PutSchema,
    TagResource,
    UntagResource,
    UpdateIdentitySource,
    UpdatePolicy,
    UpdatePolicyStore,
    UpdatePolicyTemplate,
}
impl std::fmt::Display for VerifiedpermissionsActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VerifiedpermissionsActions::CreateIdentitySource => {
                write!(f, "verifiedpermissions:CreateIdentitySource")
            }
            VerifiedpermissionsActions::CreatePolicy => {
                write!(f, "verifiedpermissions:CreatePolicy")
            }
            VerifiedpermissionsActions::CreatePolicyStore => {
                write!(f, "verifiedpermissions:CreatePolicyStore")
            }
            VerifiedpermissionsActions::CreatePolicyTemplate => {
                write!(f, "verifiedpermissions:CreatePolicyTemplate")
            }
            VerifiedpermissionsActions::DeleteIdentitySource => {
                write!(f, "verifiedpermissions:DeleteIdentitySource")
            }
            VerifiedpermissionsActions::DeletePolicy => {
                write!(f, "verifiedpermissions:DeletePolicy")
            }
            VerifiedpermissionsActions::DeletePolicyStore => {
                write!(f, "verifiedpermissions:DeletePolicyStore")
            }
            VerifiedpermissionsActions::DeletePolicyTemplate => {
                write!(f, "verifiedpermissions:DeletePolicyTemplate")
            }
            VerifiedpermissionsActions::GetIdentitySource => {
                write!(f, "verifiedpermissions:GetIdentitySource")
            }
            VerifiedpermissionsActions::GetPolicy => write!(f, "verifiedpermissions:GetPolicy"),
            VerifiedpermissionsActions::GetPolicyStore => {
                write!(f, "verifiedpermissions:GetPolicyStore")
            }
            VerifiedpermissionsActions::GetPolicyTemplate => {
                write!(f, "verifiedpermissions:GetPolicyTemplate")
            }
            VerifiedpermissionsActions::GetSchema => write!(f, "verifiedpermissions:GetSchema"),
            VerifiedpermissionsActions::IsAuthorized => {
                write!(f, "verifiedpermissions:IsAuthorized")
            }
            VerifiedpermissionsActions::IsAuthorizedWithToken => {
                write!(f, "verifiedpermissions:IsAuthorizedWithToken")
            }
            VerifiedpermissionsActions::ListIdentitySources => {
                write!(f, "verifiedpermissions:ListIdentitySources")
            }
            VerifiedpermissionsActions::ListPolicies => {
                write!(f, "verifiedpermissions:ListPolicies")
            }
            VerifiedpermissionsActions::ListPolicyStores => {
                write!(f, "verifiedpermissions:ListPolicyStores")
            }
            VerifiedpermissionsActions::ListPolicyTemplates => {
                write!(f, "verifiedpermissions:ListPolicyTemplates")
            }
            VerifiedpermissionsActions::ListTagsForResource => {
                write!(f, "verifiedpermissions:ListTagsForResource")
            }
            VerifiedpermissionsActions::PutSchema => write!(f, "verifiedpermissions:PutSchema"),
            VerifiedpermissionsActions::TagResource => write!(f, "verifiedpermissions:TagResource"),
            VerifiedpermissionsActions::UntagResource => {
                write!(f, "verifiedpermissions:UntagResource")
            }
            VerifiedpermissionsActions::UpdateIdentitySource => {
                write!(f, "verifiedpermissions:UpdateIdentitySource")
            }
            VerifiedpermissionsActions::UpdatePolicy => {
                write!(f, "verifiedpermissions:UpdatePolicy")
            }
            VerifiedpermissionsActions::UpdatePolicyStore => {
                write!(f, "verifiedpermissions:UpdatePolicyStore")
            }
            VerifiedpermissionsActions::UpdatePolicyTemplate => {
                write!(f, "verifiedpermissions:UpdatePolicyTemplate")
            }
        }
    }
}
