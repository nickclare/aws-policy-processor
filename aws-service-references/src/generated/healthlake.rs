// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum HealthlakeActions {
    CancelFhirExportJobWithDelete,
    CreateFhirDatastore,
    CreateResource,
    DeleteFhirDatastore,
    DeleteResource,
    DescribeFhirDatastore,
    DescribeFhirExportJob,
    DescribeFhirExportJobWithGet,
    DescribeFhirImportJob,
    GetCapabilities,
    GetExportedFile,
    GetHistoryByResourceId,
    ListFhirDatastores,
    ListFhirExportJobs,
    ListFhirImportJobs,
    ListTagsForResource,
    ProcessBundle,
    ReadResource,
    SearchEverything,
    SearchWithGet,
    SearchWithPost,
    StartFhirExportJob,
    StartFhirExportJobWithGet,
    StartFhirExportJobWithPost,
    StartFhirImportJob,
    TagResource,
    UntagResource,
    UpdateResource,
    VersionReadResource,
}
impl std::fmt::Display for HealthlakeActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HealthlakeActions::CancelFhirExportJobWithDelete => {
                write!(f, "healthlake:CancelFHIRExportJobWithDelete")
            }
            HealthlakeActions::CreateFhirDatastore => write!(f, "healthlake:CreateFHIRDatastore"),
            HealthlakeActions::CreateResource => write!(f, "healthlake:CreateResource"),
            HealthlakeActions::DeleteFhirDatastore => write!(f, "healthlake:DeleteFHIRDatastore"),
            HealthlakeActions::DeleteResource => write!(f, "healthlake:DeleteResource"),
            HealthlakeActions::DescribeFhirDatastore => {
                write!(f, "healthlake:DescribeFHIRDatastore")
            }
            HealthlakeActions::DescribeFhirExportJob => {
                write!(f, "healthlake:DescribeFHIRExportJob")
            }
            HealthlakeActions::DescribeFhirExportJobWithGet => {
                write!(f, "healthlake:DescribeFHIRExportJobWithGet")
            }
            HealthlakeActions::DescribeFhirImportJob => {
                write!(f, "healthlake:DescribeFHIRImportJob")
            }
            HealthlakeActions::GetCapabilities => write!(f, "healthlake:GetCapabilities"),
            HealthlakeActions::GetExportedFile => write!(f, "healthlake:GetExportedFile"),
            HealthlakeActions::GetHistoryByResourceId => {
                write!(f, "healthlake:GetHistoryByResourceId")
            }
            HealthlakeActions::ListFhirDatastores => write!(f, "healthlake:ListFHIRDatastores"),
            HealthlakeActions::ListFhirExportJobs => write!(f, "healthlake:ListFHIRExportJobs"),
            HealthlakeActions::ListFhirImportJobs => write!(f, "healthlake:ListFHIRImportJobs"),
            HealthlakeActions::ListTagsForResource => write!(f, "healthlake:ListTagsForResource"),
            HealthlakeActions::ProcessBundle => write!(f, "healthlake:ProcessBundle"),
            HealthlakeActions::ReadResource => write!(f, "healthlake:ReadResource"),
            HealthlakeActions::SearchEverything => write!(f, "healthlake:SearchEverything"),
            HealthlakeActions::SearchWithGet => write!(f, "healthlake:SearchWithGet"),
            HealthlakeActions::SearchWithPost => write!(f, "healthlake:SearchWithPost"),
            HealthlakeActions::StartFhirExportJob => write!(f, "healthlake:StartFHIRExportJob"),
            HealthlakeActions::StartFhirExportJobWithGet => {
                write!(f, "healthlake:StartFHIRExportJobWithGet")
            }
            HealthlakeActions::StartFhirExportJobWithPost => {
                write!(f, "healthlake:StartFHIRExportJobWithPost")
            }
            HealthlakeActions::StartFhirImportJob => write!(f, "healthlake:StartFHIRImportJob"),
            HealthlakeActions::TagResource => write!(f, "healthlake:TagResource"),
            HealthlakeActions::UntagResource => write!(f, "healthlake:UntagResource"),
            HealthlakeActions::UpdateResource => write!(f, "healthlake:UpdateResource"),
            HealthlakeActions::VersionReadResource => write!(f, "healthlake:VersionReadResource"),
        }
    }
}
