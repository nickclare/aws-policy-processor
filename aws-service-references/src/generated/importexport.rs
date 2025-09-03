// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum ImportexportActions {
    CancelJob,
    CreateJob,
    GetShippingLabel,
    GetStatus,
    ListJobs,
    UpdateJob,
}
impl std::fmt::Display for ImportexportActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImportexportActions::CancelJob => write!(f, "importexport:CancelJob"),
            ImportexportActions::CreateJob => write!(f, "importexport:CreateJob"),
            ImportexportActions::GetShippingLabel => write!(f, "importexport:GetShippingLabel"),
            ImportexportActions::GetStatus => write!(f, "importexport:GetStatus"),
            ImportexportActions::ListJobs => write!(f, "importexport:ListJobs"),
            ImportexportActions::UpdateJob => write!(f, "importexport:UpdateJob"),
        }
    }
}
