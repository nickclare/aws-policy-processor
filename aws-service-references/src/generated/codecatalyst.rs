// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum CodecatalystActions {
    AcceptConnection,
    AssociateIamRoleToConnection,
    AssociateIdentityCenterApplicationToSpace,
    AssociateIdentityToIdentityCenterApplication,
    BatchAssociateIdentitiesToIdentityCenterApplication,
    BatchDisassociateIdentitiesFromIdentityCenterApplication,
    CreateIdentityCenterApplication,
    CreateSpace,
    CreateSpaceAdminRoleAssignment,
    DeleteConnection,
    DeleteIdentityCenterApplication,
    DisassociateIamRoleFromConnection,
    DisassociateIdentityCenterApplicationFromSpace,
    DisassociateIdentityFromIdentityCenterApplication,
    GetBillingAuthorization,
    GetConnection,
    GetIdentityCenterApplication,
    GetPendingConnection,
    ListConnections,
    ListIamRolesForConnection,
    ListIdentityCenterApplications,
    ListIdentityCenterApplicationsForSpace,
    ListSpacesForIdentityCenterApplication,
    ListTagsForResource,
    PutBillingAuthorization,
    RejectConnection,
    SynchronizeIdentityCenterApplication,
    TagResource,
    UntagResource,
    UpdateIdentityCenterApplication,
}
impl std::fmt::Display for CodecatalystActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CodecatalystActions::AcceptConnection => write!(f, "codecatalyst:AcceptConnection"),
            CodecatalystActions::AssociateIamRoleToConnection => {
                write!(f, "codecatalyst:AssociateIamRoleToConnection")
            }
            CodecatalystActions::AssociateIdentityCenterApplicationToSpace => {
                write!(f, "codecatalyst:AssociateIdentityCenterApplicationToSpace")
            }
            CodecatalystActions::AssociateIdentityToIdentityCenterApplication => write!(
                f,
                "codecatalyst:AssociateIdentityToIdentityCenterApplication"
            ),
            CodecatalystActions::BatchAssociateIdentitiesToIdentityCenterApplication => write!(
                f,
                "codecatalyst:BatchAssociateIdentitiesToIdentityCenterApplication"
            ),
            CodecatalystActions::BatchDisassociateIdentitiesFromIdentityCenterApplication => {
                write!(
                    f,
                    "codecatalyst:BatchDisassociateIdentitiesFromIdentityCenterApplication"
                )
            }
            CodecatalystActions::CreateIdentityCenterApplication => {
                write!(f, "codecatalyst:CreateIdentityCenterApplication")
            }
            CodecatalystActions::CreateSpace => write!(f, "codecatalyst:CreateSpace"),
            CodecatalystActions::CreateSpaceAdminRoleAssignment => {
                write!(f, "codecatalyst:CreateSpaceAdminRoleAssignment")
            }
            CodecatalystActions::DeleteConnection => write!(f, "codecatalyst:DeleteConnection"),
            CodecatalystActions::DeleteIdentityCenterApplication => {
                write!(f, "codecatalyst:DeleteIdentityCenterApplication")
            }
            CodecatalystActions::DisassociateIamRoleFromConnection => {
                write!(f, "codecatalyst:DisassociateIamRoleFromConnection")
            }
            CodecatalystActions::DisassociateIdentityCenterApplicationFromSpace => write!(
                f,
                "codecatalyst:DisassociateIdentityCenterApplicationFromSpace"
            ),
            CodecatalystActions::DisassociateIdentityFromIdentityCenterApplication => write!(
                f,
                "codecatalyst:DisassociateIdentityFromIdentityCenterApplication"
            ),
            CodecatalystActions::GetBillingAuthorization => {
                write!(f, "codecatalyst:GetBillingAuthorization")
            }
            CodecatalystActions::GetConnection => write!(f, "codecatalyst:GetConnection"),
            CodecatalystActions::GetIdentityCenterApplication => {
                write!(f, "codecatalyst:GetIdentityCenterApplication")
            }
            CodecatalystActions::GetPendingConnection => {
                write!(f, "codecatalyst:GetPendingConnection")
            }
            CodecatalystActions::ListConnections => write!(f, "codecatalyst:ListConnections"),
            CodecatalystActions::ListIamRolesForConnection => {
                write!(f, "codecatalyst:ListIamRolesForConnection")
            }
            CodecatalystActions::ListIdentityCenterApplications => {
                write!(f, "codecatalyst:ListIdentityCenterApplications")
            }
            CodecatalystActions::ListIdentityCenterApplicationsForSpace => {
                write!(f, "codecatalyst:ListIdentityCenterApplicationsForSpace")
            }
            CodecatalystActions::ListSpacesForIdentityCenterApplication => {
                write!(f, "codecatalyst:ListSpacesForIdentityCenterApplication")
            }
            CodecatalystActions::ListTagsForResource => {
                write!(f, "codecatalyst:ListTagsForResource")
            }
            CodecatalystActions::PutBillingAuthorization => {
                write!(f, "codecatalyst:PutBillingAuthorization")
            }
            CodecatalystActions::RejectConnection => write!(f, "codecatalyst:RejectConnection"),
            CodecatalystActions::SynchronizeIdentityCenterApplication => {
                write!(f, "codecatalyst:SynchronizeIdentityCenterApplication")
            }
            CodecatalystActions::TagResource => write!(f, "codecatalyst:TagResource"),
            CodecatalystActions::UntagResource => write!(f, "codecatalyst:UntagResource"),
            CodecatalystActions::UpdateIdentityCenterApplication => {
                write!(f, "codecatalyst:UpdateIdentityCenterApplication")
            }
        }
    }
}
