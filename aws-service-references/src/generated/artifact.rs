// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum ArtifactActions {
    AcceptAgreement,
    AcceptNdaForAgreement,
    GetAccountSettings,
    GetAgreement,
    GetCustomerAgreement,
    GetNdaForAgreement,
    GetReport,
    GetReportMetadata,
    GetTermForReport,
    ListAgreements,
    ListCustomerAgreements,
    ListReports,
    PutAccountSettings,
    TerminateAgreement,
}
impl std::fmt::Display for ArtifactActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArtifactActions::AcceptAgreement => write!(f, "artifact:AcceptAgreement"),
            ArtifactActions::AcceptNdaForAgreement => write!(f, "artifact:AcceptNdaForAgreement"),
            ArtifactActions::GetAccountSettings => write!(f, "artifact:GetAccountSettings"),
            ArtifactActions::GetAgreement => write!(f, "artifact:GetAgreement"),
            ArtifactActions::GetCustomerAgreement => write!(f, "artifact:GetCustomerAgreement"),
            ArtifactActions::GetNdaForAgreement => write!(f, "artifact:GetNdaForAgreement"),
            ArtifactActions::GetReport => write!(f, "artifact:GetReport"),
            ArtifactActions::GetReportMetadata => write!(f, "artifact:GetReportMetadata"),
            ArtifactActions::GetTermForReport => write!(f, "artifact:GetTermForReport"),
            ArtifactActions::ListAgreements => write!(f, "artifact:ListAgreements"),
            ArtifactActions::ListCustomerAgreements => write!(f, "artifact:ListCustomerAgreements"),
            ArtifactActions::ListReports => write!(f, "artifact:ListReports"),
            ArtifactActions::PutAccountSettings => write!(f, "artifact:PutAccountSettings"),
            ArtifactActions::TerminateAgreement => write!(f, "artifact:TerminateAgreement"),
        }
    }
}
