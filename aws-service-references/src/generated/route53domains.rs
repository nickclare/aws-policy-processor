// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum Route53domainsActions {
    AcceptDomainTransferFromAnotherAwsAccount,
    AssociateDelegationSignerToDomain,
    CancelDomainTransferToAnotherAwsAccount,
    CheckDomainAvailability,
    CheckDomainTransferability,
    DeleteDomain,
    DeleteTagsForDomain,
    DisableDomainAutoRenew,
    DisableDomainTransferLock,
    DisassociateDelegationSignerFromDomain,
    EnableDomainAutoRenew,
    EnableDomainTransferLock,
    GetContactReachabilityStatus,
    GetDomainDetail,
    GetDomainSuggestions,
    GetOperationDetail,
    ListDomains,
    ListOperations,
    ListPrices,
    ListTagsForDomain,
    PushDomain,
    RegisterDomain,
    RejectDomainTransferFromAnotherAwsAccount,
    RenewDomain,
    ResendContactReachabilityEmail,
    ResendOperationAuthorization,
    RetrieveDomainAuthCode,
    TransferDomain,
    TransferDomainToAnotherAwsAccount,
    UpdateDomainContact,
    UpdateDomainContactPrivacy,
    UpdateDomainNameservers,
    UpdateTagsForDomain,
    ViewBilling,
}
impl std::fmt::Display for Route53domainsActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Route53domainsActions::AcceptDomainTransferFromAnotherAwsAccount => write!(
                f,
                "route53domains:AcceptDomainTransferFromAnotherAwsAccount"
            ),
            Route53domainsActions::AssociateDelegationSignerToDomain => {
                write!(f, "route53domains:AssociateDelegationSignerToDomain")
            }
            Route53domainsActions::CancelDomainTransferToAnotherAwsAccount => {
                write!(f, "route53domains:CancelDomainTransferToAnotherAwsAccount")
            }
            Route53domainsActions::CheckDomainAvailability => {
                write!(f, "route53domains:CheckDomainAvailability")
            }
            Route53domainsActions::CheckDomainTransferability => {
                write!(f, "route53domains:CheckDomainTransferability")
            }
            Route53domainsActions::DeleteDomain => write!(f, "route53domains:DeleteDomain"),
            Route53domainsActions::DeleteTagsForDomain => {
                write!(f, "route53domains:DeleteTagsForDomain")
            }
            Route53domainsActions::DisableDomainAutoRenew => {
                write!(f, "route53domains:DisableDomainAutoRenew")
            }
            Route53domainsActions::DisableDomainTransferLock => {
                write!(f, "route53domains:DisableDomainTransferLock")
            }
            Route53domainsActions::DisassociateDelegationSignerFromDomain => {
                write!(f, "route53domains:DisassociateDelegationSignerFromDomain")
            }
            Route53domainsActions::EnableDomainAutoRenew => {
                write!(f, "route53domains:EnableDomainAutoRenew")
            }
            Route53domainsActions::EnableDomainTransferLock => {
                write!(f, "route53domains:EnableDomainTransferLock")
            }
            Route53domainsActions::GetContactReachabilityStatus => {
                write!(f, "route53domains:GetContactReachabilityStatus")
            }
            Route53domainsActions::GetDomainDetail => write!(f, "route53domains:GetDomainDetail"),
            Route53domainsActions::GetDomainSuggestions => {
                write!(f, "route53domains:GetDomainSuggestions")
            }
            Route53domainsActions::GetOperationDetail => {
                write!(f, "route53domains:GetOperationDetail")
            }
            Route53domainsActions::ListDomains => write!(f, "route53domains:ListDomains"),
            Route53domainsActions::ListOperations => write!(f, "route53domains:ListOperations"),
            Route53domainsActions::ListPrices => write!(f, "route53domains:ListPrices"),
            Route53domainsActions::ListTagsForDomain => {
                write!(f, "route53domains:ListTagsForDomain")
            }
            Route53domainsActions::PushDomain => write!(f, "route53domains:PushDomain"),
            Route53domainsActions::RegisterDomain => write!(f, "route53domains:RegisterDomain"),
            Route53domainsActions::RejectDomainTransferFromAnotherAwsAccount => write!(
                f,
                "route53domains:RejectDomainTransferFromAnotherAwsAccount"
            ),
            Route53domainsActions::RenewDomain => write!(f, "route53domains:RenewDomain"),
            Route53domainsActions::ResendContactReachabilityEmail => {
                write!(f, "route53domains:ResendContactReachabilityEmail")
            }
            Route53domainsActions::ResendOperationAuthorization => {
                write!(f, "route53domains:ResendOperationAuthorization")
            }
            Route53domainsActions::RetrieveDomainAuthCode => {
                write!(f, "route53domains:RetrieveDomainAuthCode")
            }
            Route53domainsActions::TransferDomain => write!(f, "route53domains:TransferDomain"),
            Route53domainsActions::TransferDomainToAnotherAwsAccount => {
                write!(f, "route53domains:TransferDomainToAnotherAwsAccount")
            }
            Route53domainsActions::UpdateDomainContact => {
                write!(f, "route53domains:UpdateDomainContact")
            }
            Route53domainsActions::UpdateDomainContactPrivacy => {
                write!(f, "route53domains:UpdateDomainContactPrivacy")
            }
            Route53domainsActions::UpdateDomainNameservers => {
                write!(f, "route53domains:UpdateDomainNameservers")
            }
            Route53domainsActions::UpdateTagsForDomain => {
                write!(f, "route53domains:UpdateTagsForDomain")
            }
            Route53domainsActions::ViewBilling => write!(f, "route53domains:ViewBilling"),
        }
    }
}
