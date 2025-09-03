// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum Route53profilesActions {
    AssociateProfile,
    AssociateResourceToProfile,
    CreateProfile,
    DeleteProfile,
    DisassociateProfile,
    DisassociateResourceFromProfile,
    GetProfile,
    GetProfileAssociation,
    GetProfilePolicy,
    GetProfileResourceAssociation,
    ListProfileAssociations,
    ListProfileResourceAssociations,
    ListProfiles,
    ListTagsForResource,
    PutProfilePolicy,
    TagResource,
    UntagResource,
    UpdateProfileResourceAssociation,
}
impl std::fmt::Display for Route53profilesActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Route53profilesActions::AssociateProfile => {
                write!(f, "route53profiles:AssociateProfile")
            }
            Route53profilesActions::AssociateResourceToProfile => {
                write!(f, "route53profiles:AssociateResourceToProfile")
            }
            Route53profilesActions::CreateProfile => write!(f, "route53profiles:CreateProfile"),
            Route53profilesActions::DeleteProfile => write!(f, "route53profiles:DeleteProfile"),
            Route53profilesActions::DisassociateProfile => {
                write!(f, "route53profiles:DisassociateProfile")
            }
            Route53profilesActions::DisassociateResourceFromProfile => {
                write!(f, "route53profiles:DisassociateResourceFromProfile")
            }
            Route53profilesActions::GetProfile => write!(f, "route53profiles:GetProfile"),
            Route53profilesActions::GetProfileAssociation => {
                write!(f, "route53profiles:GetProfileAssociation")
            }
            Route53profilesActions::GetProfilePolicy => {
                write!(f, "route53profiles:GetProfilePolicy")
            }
            Route53profilesActions::GetProfileResourceAssociation => {
                write!(f, "route53profiles:GetProfileResourceAssociation")
            }
            Route53profilesActions::ListProfileAssociations => {
                write!(f, "route53profiles:ListProfileAssociations")
            }
            Route53profilesActions::ListProfileResourceAssociations => {
                write!(f, "route53profiles:ListProfileResourceAssociations")
            }
            Route53profilesActions::ListProfiles => write!(f, "route53profiles:ListProfiles"),
            Route53profilesActions::ListTagsForResource => {
                write!(f, "route53profiles:ListTagsForResource")
            }
            Route53profilesActions::PutProfilePolicy => {
                write!(f, "route53profiles:PutProfilePolicy")
            }
            Route53profilesActions::TagResource => write!(f, "route53profiles:TagResource"),
            Route53profilesActions::UntagResource => write!(f, "route53profiles:UntagResource"),
            Route53profilesActions::UpdateProfileResourceAssociation => {
                write!(f, "route53profiles:UpdateProfileResourceAssociation")
            }
        }
    }
}
