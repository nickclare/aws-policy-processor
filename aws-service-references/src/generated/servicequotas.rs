// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum ServicequotasActions {
    AssociateServiceQuotaTemplate,
    CreateSupportCase,
    DeleteServiceQuotaIncreaseRequestFromTemplate,
    DisassociateServiceQuotaTemplate,
    GetAssociationForServiceQuotaTemplate,
    GetAwsDefaultServiceQuota,
    GetRequestedServiceQuotaChange,
    GetServiceQuota,
    GetServiceQuotaIncreaseRequestFromTemplate,
    ListAwsDefaultServiceQuotas,
    ListRequestedServiceQuotaChangeHistory,
    ListRequestedServiceQuotaChangeHistoryByQuota,
    ListServiceQuotaIncreaseRequestsInTemplate,
    ListServiceQuotas,
    ListServices,
    ListTagsForResource,
    PutServiceQuotaIncreaseRequestIntoTemplate,
    RequestServiceQuotaIncrease,
    TagResource,
    UntagResource,
}
impl std::fmt::Display for ServicequotasActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServicequotasActions::AssociateServiceQuotaTemplate => {
                write!(f, "servicequotas:AssociateServiceQuotaTemplate")
            }
            ServicequotasActions::CreateSupportCase => write!(f, "servicequotas:CreateSupportCase"),
            ServicequotasActions::DeleteServiceQuotaIncreaseRequestFromTemplate => write!(
                f,
                "servicequotas:DeleteServiceQuotaIncreaseRequestFromTemplate"
            ),
            ServicequotasActions::DisassociateServiceQuotaTemplate => {
                write!(f, "servicequotas:DisassociateServiceQuotaTemplate")
            }
            ServicequotasActions::GetAssociationForServiceQuotaTemplate => {
                write!(f, "servicequotas:GetAssociationForServiceQuotaTemplate")
            }
            ServicequotasActions::GetAwsDefaultServiceQuota => {
                write!(f, "servicequotas:GetAWSDefaultServiceQuota")
            }
            ServicequotasActions::GetRequestedServiceQuotaChange => {
                write!(f, "servicequotas:GetRequestedServiceQuotaChange")
            }
            ServicequotasActions::GetServiceQuota => write!(f, "servicequotas:GetServiceQuota"),
            ServicequotasActions::GetServiceQuotaIncreaseRequestFromTemplate => write!(
                f,
                "servicequotas:GetServiceQuotaIncreaseRequestFromTemplate"
            ),
            ServicequotasActions::ListAwsDefaultServiceQuotas => {
                write!(f, "servicequotas:ListAWSDefaultServiceQuotas")
            }
            ServicequotasActions::ListRequestedServiceQuotaChangeHistory => {
                write!(f, "servicequotas:ListRequestedServiceQuotaChangeHistory")
            }
            ServicequotasActions::ListRequestedServiceQuotaChangeHistoryByQuota => write!(
                f,
                "servicequotas:ListRequestedServiceQuotaChangeHistoryByQuota"
            ),
            ServicequotasActions::ListServiceQuotaIncreaseRequestsInTemplate => write!(
                f,
                "servicequotas:ListServiceQuotaIncreaseRequestsInTemplate"
            ),
            ServicequotasActions::ListServiceQuotas => write!(f, "servicequotas:ListServiceQuotas"),
            ServicequotasActions::ListServices => write!(f, "servicequotas:ListServices"),
            ServicequotasActions::ListTagsForResource => {
                write!(f, "servicequotas:ListTagsForResource")
            }
            ServicequotasActions::PutServiceQuotaIncreaseRequestIntoTemplate => write!(
                f,
                "servicequotas:PutServiceQuotaIncreaseRequestIntoTemplate"
            ),
            ServicequotasActions::RequestServiceQuotaIncrease => {
                write!(f, "servicequotas:RequestServiceQuotaIncrease")
            }
            ServicequotasActions::TagResource => write!(f, "servicequotas:TagResource"),
            ServicequotasActions::UntagResource => write!(f, "servicequotas:UntagResource"),
        }
    }
}
