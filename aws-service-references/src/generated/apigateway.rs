// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum ApigatewayActions {
    AddCertificateToDomain,
    CreateAccessAssociation,
    CreateRoutingRule,
    Delete,
    DeleteRoutingRule,
    Get,
    GetRoutingRule,
    ListRoutingRules,
    Patch,
    Post,
    Put,
    RejectAccessAssociation,
    RemoveCertificateFromDomain,
    SetWebAcl,
    UpdateDomainNameManagementPolicy,
    UpdateDomainNamePolicy,
    UpdateRestApiPolicy,
    UpdateRoutingRule,
}
impl std::fmt::Display for ApigatewayActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApigatewayActions::AddCertificateToDomain => {
                write!(f, "apigateway:AddCertificateToDomain")
            }
            ApigatewayActions::CreateAccessAssociation => {
                write!(f, "apigateway:CreateAccessAssociation")
            }
            ApigatewayActions::CreateRoutingRule => write!(f, "apigateway:CreateRoutingRule"),
            ApigatewayActions::Delete => write!(f, "apigateway:DELETE"),
            ApigatewayActions::DeleteRoutingRule => write!(f, "apigateway:DeleteRoutingRule"),
            ApigatewayActions::Get => write!(f, "apigateway:GET"),
            ApigatewayActions::GetRoutingRule => write!(f, "apigateway:GetRoutingRule"),
            ApigatewayActions::ListRoutingRules => write!(f, "apigateway:ListRoutingRules"),
            ApigatewayActions::Patch => write!(f, "apigateway:PATCH"),
            ApigatewayActions::Post => write!(f, "apigateway:POST"),
            ApigatewayActions::Put => write!(f, "apigateway:PUT"),
            ApigatewayActions::RejectAccessAssociation => {
                write!(f, "apigateway:RejectAccessAssociation")
            }
            ApigatewayActions::RemoveCertificateFromDomain => {
                write!(f, "apigateway:RemoveCertificateFromDomain")
            }
            ApigatewayActions::SetWebAcl => write!(f, "apigateway:SetWebACL"),
            ApigatewayActions::UpdateDomainNameManagementPolicy => {
                write!(f, "apigateway:UpdateDomainNameManagementPolicy")
            }
            ApigatewayActions::UpdateDomainNamePolicy => {
                write!(f, "apigateway:UpdateDomainNamePolicy")
            }
            ApigatewayActions::UpdateRestApiPolicy => write!(f, "apigateway:UpdateRestApiPolicy"),
            ApigatewayActions::UpdateRoutingRule => write!(f, "apigateway:UpdateRoutingRule"),
        }
    }
}
