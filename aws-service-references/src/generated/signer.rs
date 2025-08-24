// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum SignerActions {
    AddProfilePermission,
    CancelSigningProfile,
    DescribeSigningJob,
    GetRevocationStatus,
    GetSigningPlatform,
    GetSigningProfile,
    ListProfilePermissions,
    ListSigningJobs,
    ListSigningPlatforms,
    ListSigningProfiles,
    ListTagsForResource,
    PutSigningProfile,
    RemoveProfilePermission,
    RevokeSignature,
    RevokeSigningProfile,
    SignPayload,
    StartSigningJob,
    TagResource,
    UntagResource,
}
impl std::fmt::Display for SignerActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SignerActions::AddProfilePermission => write!(f, "signer:AddProfilePermission"),
            SignerActions::CancelSigningProfile => write!(f, "signer:CancelSigningProfile"),
            SignerActions::DescribeSigningJob => write!(f, "signer:DescribeSigningJob"),
            SignerActions::GetRevocationStatus => write!(f, "signer:GetRevocationStatus"),
            SignerActions::GetSigningPlatform => write!(f, "signer:GetSigningPlatform"),
            SignerActions::GetSigningProfile => write!(f, "signer:GetSigningProfile"),
            SignerActions::ListProfilePermissions => write!(f, "signer:ListProfilePermissions"),
            SignerActions::ListSigningJobs => write!(f, "signer:ListSigningJobs"),
            SignerActions::ListSigningPlatforms => write!(f, "signer:ListSigningPlatforms"),
            SignerActions::ListSigningProfiles => write!(f, "signer:ListSigningProfiles"),
            SignerActions::ListTagsForResource => write!(f, "signer:ListTagsForResource"),
            SignerActions::PutSigningProfile => write!(f, "signer:PutSigningProfile"),
            SignerActions::RemoveProfilePermission => write!(f, "signer:RemoveProfilePermission"),
            SignerActions::RevokeSignature => write!(f, "signer:RevokeSignature"),
            SignerActions::RevokeSigningProfile => write!(f, "signer:RevokeSigningProfile"),
            SignerActions::SignPayload => write!(f, "signer:SignPayload"),
            SignerActions::StartSigningJob => write!(f, "signer:StartSigningJob"),
            SignerActions::TagResource => write!(f, "signer:TagResource"),
            SignerActions::UntagResource => write!(f, "signer:UntagResource"),
        }
    }
}
