// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum ProfileActions {
    AddProfileKey,
    BatchGetCalculatedAttributeForProfile,
    BatchGetProfile,
    CreateCalculatedAttributeDefinition,
    CreateDomain,
    CreateDomainLayout,
    CreateEventStream,
    CreateEventTrigger,
    CreateIntegrationWorkflow,
    CreateProfile,
    CreateSegmentDefinition,
    CreateSegmentEstimate,
    CreateSegmentSnapshot,
    CreateSnapshot,
    CreateUploadJob,
    DeleteCalculatedAttributeDefinition,
    DeleteDomain,
    DeleteDomainLayout,
    DeleteEventStream,
    DeleteEventTrigger,
    DeleteIntegration,
    DeleteProfile,
    DeleteProfileKey,
    DeleteProfileObject,
    DeleteProfileObjectType,
    DeleteSegmentDefinition,
    DeleteWorkflow,
    DetectProfileObjectType,
    GetAutoMergingPreview,
    GetCalculatedAttributeDefinition,
    GetCalculatedAttributeForProfile,
    GetDomain,
    GetDomainLayout,
    GetEventStream,
    GetEventTrigger,
    GetIdentityResolutionJob,
    GetIntegration,
    GetMatches,
    GetProfileObjectType,
    GetProfileObjectTypeTemplate,
    GetSegmentDefinition,
    GetSegmentEstimate,
    GetSegmentMembership,
    GetSegmentSnapshot,
    GetSimilarProfiles,
    GetSnapshot,
    GetUploadJob,
    GetUploadJobPath,
    GetWorkflow,
    GetWorkflowSteps,
    ListAccountIntegrations,
    ListCalculatedAttributeDefinitions,
    ListCalculatedAttributesForProfile,
    ListDomainLayouts,
    ListDomains,
    ListEventStreams,
    ListEventTriggers,
    ListIdentityResolutionJobs,
    ListIntegrations,
    ListObjectTypeAttributes,
    ListProfileAttributeValues,
    ListProfileObjectTypeTemplates,
    ListProfileObjectTypes,
    ListProfileObjects,
    ListRuleBasedMatches,
    ListSegmentDefinitions,
    ListTagsForResource,
    ListUploadJobs,
    ListWorkflows,
    MergeProfiles,
    PutIntegration,
    PutProfileObject,
    PutProfileObjectType,
    SearchProfiles,
    StartUploadJob,
    StopUploadJob,
    TagResource,
    UntagResource,
    UpdateCalculatedAttributeDefinition,
    UpdateDomain,
    UpdateDomainLayout,
    UpdateEventTrigger,
    UpdateProfile,
}
impl std::fmt::Display for ProfileActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProfileActions::AddProfileKey => write!(f, "profile:AddProfileKey"),
            ProfileActions::BatchGetCalculatedAttributeForProfile => {
                write!(f, "profile:BatchGetCalculatedAttributeForProfile")
            }
            ProfileActions::BatchGetProfile => write!(f, "profile:BatchGetProfile"),
            ProfileActions::CreateCalculatedAttributeDefinition => {
                write!(f, "profile:CreateCalculatedAttributeDefinition")
            }
            ProfileActions::CreateDomain => write!(f, "profile:CreateDomain"),
            ProfileActions::CreateDomainLayout => write!(f, "profile:CreateDomainLayout"),
            ProfileActions::CreateEventStream => write!(f, "profile:CreateEventStream"),
            ProfileActions::CreateEventTrigger => write!(f, "profile:CreateEventTrigger"),
            ProfileActions::CreateIntegrationWorkflow => {
                write!(f, "profile:CreateIntegrationWorkflow")
            }
            ProfileActions::CreateProfile => write!(f, "profile:CreateProfile"),
            ProfileActions::CreateSegmentDefinition => write!(f, "profile:CreateSegmentDefinition"),
            ProfileActions::CreateSegmentEstimate => write!(f, "profile:CreateSegmentEstimate"),
            ProfileActions::CreateSegmentSnapshot => write!(f, "profile:CreateSegmentSnapshot"),
            ProfileActions::CreateSnapshot => write!(f, "profile:CreateSnapshot"),
            ProfileActions::CreateUploadJob => write!(f, "profile:CreateUploadJob"),
            ProfileActions::DeleteCalculatedAttributeDefinition => {
                write!(f, "profile:DeleteCalculatedAttributeDefinition")
            }
            ProfileActions::DeleteDomain => write!(f, "profile:DeleteDomain"),
            ProfileActions::DeleteDomainLayout => write!(f, "profile:DeleteDomainLayout"),
            ProfileActions::DeleteEventStream => write!(f, "profile:DeleteEventStream"),
            ProfileActions::DeleteEventTrigger => write!(f, "profile:DeleteEventTrigger"),
            ProfileActions::DeleteIntegration => write!(f, "profile:DeleteIntegration"),
            ProfileActions::DeleteProfile => write!(f, "profile:DeleteProfile"),
            ProfileActions::DeleteProfileKey => write!(f, "profile:DeleteProfileKey"),
            ProfileActions::DeleteProfileObject => write!(f, "profile:DeleteProfileObject"),
            ProfileActions::DeleteProfileObjectType => write!(f, "profile:DeleteProfileObjectType"),
            ProfileActions::DeleteSegmentDefinition => write!(f, "profile:DeleteSegmentDefinition"),
            ProfileActions::DeleteWorkflow => write!(f, "profile:DeleteWorkflow"),
            ProfileActions::DetectProfileObjectType => write!(f, "profile:DetectProfileObjectType"),
            ProfileActions::GetAutoMergingPreview => write!(f, "profile:GetAutoMergingPreview"),
            ProfileActions::GetCalculatedAttributeDefinition => {
                write!(f, "profile:GetCalculatedAttributeDefinition")
            }
            ProfileActions::GetCalculatedAttributeForProfile => {
                write!(f, "profile:GetCalculatedAttributeForProfile")
            }
            ProfileActions::GetDomain => write!(f, "profile:GetDomain"),
            ProfileActions::GetDomainLayout => write!(f, "profile:GetDomainLayout"),
            ProfileActions::GetEventStream => write!(f, "profile:GetEventStream"),
            ProfileActions::GetEventTrigger => write!(f, "profile:GetEventTrigger"),
            ProfileActions::GetIdentityResolutionJob => {
                write!(f, "profile:GetIdentityResolutionJob")
            }
            ProfileActions::GetIntegration => write!(f, "profile:GetIntegration"),
            ProfileActions::GetMatches => write!(f, "profile:GetMatches"),
            ProfileActions::GetProfileObjectType => write!(f, "profile:GetProfileObjectType"),
            ProfileActions::GetProfileObjectTypeTemplate => {
                write!(f, "profile:GetProfileObjectTypeTemplate")
            }
            ProfileActions::GetSegmentDefinition => write!(f, "profile:GetSegmentDefinition"),
            ProfileActions::GetSegmentEstimate => write!(f, "profile:GetSegmentEstimate"),
            ProfileActions::GetSegmentMembership => write!(f, "profile:GetSegmentMembership"),
            ProfileActions::GetSegmentSnapshot => write!(f, "profile:GetSegmentSnapshot"),
            ProfileActions::GetSimilarProfiles => write!(f, "profile:GetSimilarProfiles"),
            ProfileActions::GetSnapshot => write!(f, "profile:GetSnapshot"),
            ProfileActions::GetUploadJob => write!(f, "profile:GetUploadJob"),
            ProfileActions::GetUploadJobPath => write!(f, "profile:GetUploadJobPath"),
            ProfileActions::GetWorkflow => write!(f, "profile:GetWorkflow"),
            ProfileActions::GetWorkflowSteps => write!(f, "profile:GetWorkflowSteps"),
            ProfileActions::ListAccountIntegrations => write!(f, "profile:ListAccountIntegrations"),
            ProfileActions::ListCalculatedAttributeDefinitions => {
                write!(f, "profile:ListCalculatedAttributeDefinitions")
            }
            ProfileActions::ListCalculatedAttributesForProfile => {
                write!(f, "profile:ListCalculatedAttributesForProfile")
            }
            ProfileActions::ListDomainLayouts => write!(f, "profile:ListDomainLayouts"),
            ProfileActions::ListDomains => write!(f, "profile:ListDomains"),
            ProfileActions::ListEventStreams => write!(f, "profile:ListEventStreams"),
            ProfileActions::ListEventTriggers => write!(f, "profile:ListEventTriggers"),
            ProfileActions::ListIdentityResolutionJobs => {
                write!(f, "profile:ListIdentityResolutionJobs")
            }
            ProfileActions::ListIntegrations => write!(f, "profile:ListIntegrations"),
            ProfileActions::ListObjectTypeAttributes => {
                write!(f, "profile:ListObjectTypeAttributes")
            }
            ProfileActions::ListProfileAttributeValues => {
                write!(f, "profile:ListProfileAttributeValues")
            }
            ProfileActions::ListProfileObjectTypeTemplates => {
                write!(f, "profile:ListProfileObjectTypeTemplates")
            }
            ProfileActions::ListProfileObjectTypes => write!(f, "profile:ListProfileObjectTypes"),
            ProfileActions::ListProfileObjects => write!(f, "profile:ListProfileObjects"),
            ProfileActions::ListRuleBasedMatches => write!(f, "profile:ListRuleBasedMatches"),
            ProfileActions::ListSegmentDefinitions => write!(f, "profile:ListSegmentDefinitions"),
            ProfileActions::ListTagsForResource => write!(f, "profile:ListTagsForResource"),
            ProfileActions::ListUploadJobs => write!(f, "profile:ListUploadJobs"),
            ProfileActions::ListWorkflows => write!(f, "profile:ListWorkflows"),
            ProfileActions::MergeProfiles => write!(f, "profile:MergeProfiles"),
            ProfileActions::PutIntegration => write!(f, "profile:PutIntegration"),
            ProfileActions::PutProfileObject => write!(f, "profile:PutProfileObject"),
            ProfileActions::PutProfileObjectType => write!(f, "profile:PutProfileObjectType"),
            ProfileActions::SearchProfiles => write!(f, "profile:SearchProfiles"),
            ProfileActions::StartUploadJob => write!(f, "profile:StartUploadJob"),
            ProfileActions::StopUploadJob => write!(f, "profile:StopUploadJob"),
            ProfileActions::TagResource => write!(f, "profile:TagResource"),
            ProfileActions::UntagResource => write!(f, "profile:UntagResource"),
            ProfileActions::UpdateCalculatedAttributeDefinition => {
                write!(f, "profile:UpdateCalculatedAttributeDefinition")
            }
            ProfileActions::UpdateDomain => write!(f, "profile:UpdateDomain"),
            ProfileActions::UpdateDomainLayout => write!(f, "profile:UpdateDomainLayout"),
            ProfileActions::UpdateEventTrigger => write!(f, "profile:UpdateEventTrigger"),
            ProfileActions::UpdateProfile => write!(f, "profile:UpdateProfile"),
        }
    }
}
