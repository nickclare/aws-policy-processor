// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum MediaconvertActions {
    AssociateCertificate,
    CancelJob,
    CreateJob,
    CreateJobTemplate,
    CreatePreset,
    CreateQueue,
    DeleteJobTemplate,
    DeletePolicy,
    DeletePreset,
    DeleteQueue,
    DescribeEndpoints,
    DisassociateCertificate,
    GetJob,
    GetJobTemplate,
    GetPolicy,
    GetPreset,
    GetQueue,
    ListJobTemplates,
    ListJobs,
    ListPresets,
    ListQueues,
    ListTagsForResource,
    ListVersions,
    Probe,
    PutPolicy,
    SearchJobs,
    TagResource,
    UntagResource,
    UpdateJobTemplate,
    UpdatePreset,
    UpdateQueue,
}
impl std::fmt::Display for MediaconvertActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MediaconvertActions::AssociateCertificate => {
                write!(f, "mediaconvert:AssociateCertificate")
            }
            MediaconvertActions::CancelJob => write!(f, "mediaconvert:CancelJob"),
            MediaconvertActions::CreateJob => write!(f, "mediaconvert:CreateJob"),
            MediaconvertActions::CreateJobTemplate => write!(f, "mediaconvert:CreateJobTemplate"),
            MediaconvertActions::CreatePreset => write!(f, "mediaconvert:CreatePreset"),
            MediaconvertActions::CreateQueue => write!(f, "mediaconvert:CreateQueue"),
            MediaconvertActions::DeleteJobTemplate => write!(f, "mediaconvert:DeleteJobTemplate"),
            MediaconvertActions::DeletePolicy => write!(f, "mediaconvert:DeletePolicy"),
            MediaconvertActions::DeletePreset => write!(f, "mediaconvert:DeletePreset"),
            MediaconvertActions::DeleteQueue => write!(f, "mediaconvert:DeleteQueue"),
            MediaconvertActions::DescribeEndpoints => write!(f, "mediaconvert:DescribeEndpoints"),
            MediaconvertActions::DisassociateCertificate => {
                write!(f, "mediaconvert:DisassociateCertificate")
            }
            MediaconvertActions::GetJob => write!(f, "mediaconvert:GetJob"),
            MediaconvertActions::GetJobTemplate => write!(f, "mediaconvert:GetJobTemplate"),
            MediaconvertActions::GetPolicy => write!(f, "mediaconvert:GetPolicy"),
            MediaconvertActions::GetPreset => write!(f, "mediaconvert:GetPreset"),
            MediaconvertActions::GetQueue => write!(f, "mediaconvert:GetQueue"),
            MediaconvertActions::ListJobTemplates => write!(f, "mediaconvert:ListJobTemplates"),
            MediaconvertActions::ListJobs => write!(f, "mediaconvert:ListJobs"),
            MediaconvertActions::ListPresets => write!(f, "mediaconvert:ListPresets"),
            MediaconvertActions::ListQueues => write!(f, "mediaconvert:ListQueues"),
            MediaconvertActions::ListTagsForResource => {
                write!(f, "mediaconvert:ListTagsForResource")
            }
            MediaconvertActions::ListVersions => write!(f, "mediaconvert:ListVersions"),
            MediaconvertActions::Probe => write!(f, "mediaconvert:Probe"),
            MediaconvertActions::PutPolicy => write!(f, "mediaconvert:PutPolicy"),
            MediaconvertActions::SearchJobs => write!(f, "mediaconvert:SearchJobs"),
            MediaconvertActions::TagResource => write!(f, "mediaconvert:TagResource"),
            MediaconvertActions::UntagResource => write!(f, "mediaconvert:UntagResource"),
            MediaconvertActions::UpdateJobTemplate => write!(f, "mediaconvert:UpdateJobTemplate"),
            MediaconvertActions::UpdatePreset => write!(f, "mediaconvert:UpdatePreset"),
            MediaconvertActions::UpdateQueue => write!(f, "mediaconvert:UpdateQueue"),
        }
    }
}
