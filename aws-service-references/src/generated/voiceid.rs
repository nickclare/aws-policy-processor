// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum VoiceidActions {
    AssociateFraudster,
    CreateDomain,
    CreateWatchlist,
    DeleteDomain,
    DeleteFraudster,
    DeleteSpeaker,
    DeleteWatchlist,
    DescribeComplianceConsent,
    DescribeDomain,
    DescribeFraudster,
    DescribeFraudsterRegistrationJob,
    DescribeSpeaker,
    DescribeSpeakerEnrollmentJob,
    DescribeWatchlist,
    DisassociateFraudster,
    EvaluateSession,
    ListDomains,
    ListFraudsterRegistrationJobs,
    ListFraudsters,
    ListSpeakerEnrollmentJobs,
    ListSpeakers,
    ListTagsForResource,
    ListWatchlists,
    OptOutSpeaker,
    RegisterComplianceConsent,
    StartFraudsterRegistrationJob,
    StartSpeakerEnrollmentJob,
    TagResource,
    UntagResource,
    UpdateDomain,
    UpdateWatchlist,
}
impl std::fmt::Display for VoiceidActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VoiceidActions::AssociateFraudster => write!(f, "voiceid:AssociateFraudster"),
            VoiceidActions::CreateDomain => write!(f, "voiceid:CreateDomain"),
            VoiceidActions::CreateWatchlist => write!(f, "voiceid:CreateWatchlist"),
            VoiceidActions::DeleteDomain => write!(f, "voiceid:DeleteDomain"),
            VoiceidActions::DeleteFraudster => write!(f, "voiceid:DeleteFraudster"),
            VoiceidActions::DeleteSpeaker => write!(f, "voiceid:DeleteSpeaker"),
            VoiceidActions::DeleteWatchlist => write!(f, "voiceid:DeleteWatchlist"),
            VoiceidActions::DescribeComplianceConsent => {
                write!(f, "voiceid:DescribeComplianceConsent")
            }
            VoiceidActions::DescribeDomain => write!(f, "voiceid:DescribeDomain"),
            VoiceidActions::DescribeFraudster => write!(f, "voiceid:DescribeFraudster"),
            VoiceidActions::DescribeFraudsterRegistrationJob => {
                write!(f, "voiceid:DescribeFraudsterRegistrationJob")
            }
            VoiceidActions::DescribeSpeaker => write!(f, "voiceid:DescribeSpeaker"),
            VoiceidActions::DescribeSpeakerEnrollmentJob => {
                write!(f, "voiceid:DescribeSpeakerEnrollmentJob")
            }
            VoiceidActions::DescribeWatchlist => write!(f, "voiceid:DescribeWatchlist"),
            VoiceidActions::DisassociateFraudster => write!(f, "voiceid:DisassociateFraudster"),
            VoiceidActions::EvaluateSession => write!(f, "voiceid:EvaluateSession"),
            VoiceidActions::ListDomains => write!(f, "voiceid:ListDomains"),
            VoiceidActions::ListFraudsterRegistrationJobs => {
                write!(f, "voiceid:ListFraudsterRegistrationJobs")
            }
            VoiceidActions::ListFraudsters => write!(f, "voiceid:ListFraudsters"),
            VoiceidActions::ListSpeakerEnrollmentJobs => {
                write!(f, "voiceid:ListSpeakerEnrollmentJobs")
            }
            VoiceidActions::ListSpeakers => write!(f, "voiceid:ListSpeakers"),
            VoiceidActions::ListTagsForResource => write!(f, "voiceid:ListTagsForResource"),
            VoiceidActions::ListWatchlists => write!(f, "voiceid:ListWatchlists"),
            VoiceidActions::OptOutSpeaker => write!(f, "voiceid:OptOutSpeaker"),
            VoiceidActions::RegisterComplianceConsent => {
                write!(f, "voiceid:RegisterComplianceConsent")
            }
            VoiceidActions::StartFraudsterRegistrationJob => {
                write!(f, "voiceid:StartFraudsterRegistrationJob")
            }
            VoiceidActions::StartSpeakerEnrollmentJob => {
                write!(f, "voiceid:StartSpeakerEnrollmentJob")
            }
            VoiceidActions::TagResource => write!(f, "voiceid:TagResource"),
            VoiceidActions::UntagResource => write!(f, "voiceid:UntagResource"),
            VoiceidActions::UpdateDomain => write!(f, "voiceid:UpdateDomain"),
            VoiceidActions::UpdateWatchlist => write!(f, "voiceid:UpdateWatchlist"),
        }
    }
}
