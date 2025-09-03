// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum EntityresolutionActions {
    AddPolicyStatement,
    BatchDeleteUniqueId,
    CreateIdMappingWorkflow,
    CreateIdNamespace,
    CreateMatchingWorkflow,
    CreateSchemaMapping,
    DeleteIdMappingWorkflow,
    DeleteIdNamespace,
    DeleteMatchingWorkflow,
    DeletePolicyStatement,
    DeleteSchemaMapping,
    GenerateMatchId,
    GetIdMappingJob,
    GetIdMappingWorkflow,
    GetIdNamespace,
    GetMatchId,
    GetMatchingJob,
    GetMatchingWorkflow,
    GetPolicy,
    GetProviderService,
    GetSchemaMapping,
    ListIdMappingJobs,
    ListIdMappingWorkflows,
    ListIdNamespaces,
    ListMatchingJobs,
    ListMatchingWorkflows,
    ListProviderServices,
    ListSchemaMappings,
    ListTagsForResource,
    PutPolicy,
    StartIdMappingJob,
    StartMatchingJob,
    TagResource,
    UntagResource,
    UpdateIdMappingWorkflow,
    UpdateIdNamespace,
    UpdateMatchingWorkflow,
    UpdateSchemaMapping,
    UseIdNamespace,
    UseWorkflow,
}
impl std::fmt::Display for EntityresolutionActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EntityresolutionActions::AddPolicyStatement => {
                write!(f, "entityresolution:AddPolicyStatement")
            }
            EntityresolutionActions::BatchDeleteUniqueId => {
                write!(f, "entityresolution:BatchDeleteUniqueId")
            }
            EntityresolutionActions::CreateIdMappingWorkflow => {
                write!(f, "entityresolution:CreateIdMappingWorkflow")
            }
            EntityresolutionActions::CreateIdNamespace => {
                write!(f, "entityresolution:CreateIdNamespace")
            }
            EntityresolutionActions::CreateMatchingWorkflow => {
                write!(f, "entityresolution:CreateMatchingWorkflow")
            }
            EntityresolutionActions::CreateSchemaMapping => {
                write!(f, "entityresolution:CreateSchemaMapping")
            }
            EntityresolutionActions::DeleteIdMappingWorkflow => {
                write!(f, "entityresolution:DeleteIdMappingWorkflow")
            }
            EntityresolutionActions::DeleteIdNamespace => {
                write!(f, "entityresolution:DeleteIdNamespace")
            }
            EntityresolutionActions::DeleteMatchingWorkflow => {
                write!(f, "entityresolution:DeleteMatchingWorkflow")
            }
            EntityresolutionActions::DeletePolicyStatement => {
                write!(f, "entityresolution:DeletePolicyStatement")
            }
            EntityresolutionActions::DeleteSchemaMapping => {
                write!(f, "entityresolution:DeleteSchemaMapping")
            }
            EntityresolutionActions::GenerateMatchId => {
                write!(f, "entityresolution:GenerateMatchId")
            }
            EntityresolutionActions::GetIdMappingJob => {
                write!(f, "entityresolution:GetIdMappingJob")
            }
            EntityresolutionActions::GetIdMappingWorkflow => {
                write!(f, "entityresolution:GetIdMappingWorkflow")
            }
            EntityresolutionActions::GetIdNamespace => write!(f, "entityresolution:GetIdNamespace"),
            EntityresolutionActions::GetMatchId => write!(f, "entityresolution:GetMatchId"),
            EntityresolutionActions::GetMatchingJob => write!(f, "entityresolution:GetMatchingJob"),
            EntityresolutionActions::GetMatchingWorkflow => {
                write!(f, "entityresolution:GetMatchingWorkflow")
            }
            EntityresolutionActions::GetPolicy => write!(f, "entityresolution:GetPolicy"),
            EntityresolutionActions::GetProviderService => {
                write!(f, "entityresolution:GetProviderService")
            }
            EntityresolutionActions::GetSchemaMapping => {
                write!(f, "entityresolution:GetSchemaMapping")
            }
            EntityresolutionActions::ListIdMappingJobs => {
                write!(f, "entityresolution:ListIdMappingJobs")
            }
            EntityresolutionActions::ListIdMappingWorkflows => {
                write!(f, "entityresolution:ListIdMappingWorkflows")
            }
            EntityresolutionActions::ListIdNamespaces => {
                write!(f, "entityresolution:ListIdNamespaces")
            }
            EntityresolutionActions::ListMatchingJobs => {
                write!(f, "entityresolution:ListMatchingJobs")
            }
            EntityresolutionActions::ListMatchingWorkflows => {
                write!(f, "entityresolution:ListMatchingWorkflows")
            }
            EntityresolutionActions::ListProviderServices => {
                write!(f, "entityresolution:ListProviderServices")
            }
            EntityresolutionActions::ListSchemaMappings => {
                write!(f, "entityresolution:ListSchemaMappings")
            }
            EntityresolutionActions::ListTagsForResource => {
                write!(f, "entityresolution:ListTagsForResource")
            }
            EntityresolutionActions::PutPolicy => write!(f, "entityresolution:PutPolicy"),
            EntityresolutionActions::StartIdMappingJob => {
                write!(f, "entityresolution:StartIdMappingJob")
            }
            EntityresolutionActions::StartMatchingJob => {
                write!(f, "entityresolution:StartMatchingJob")
            }
            EntityresolutionActions::TagResource => write!(f, "entityresolution:TagResource"),
            EntityresolutionActions::UntagResource => write!(f, "entityresolution:UntagResource"),
            EntityresolutionActions::UpdateIdMappingWorkflow => {
                write!(f, "entityresolution:UpdateIdMappingWorkflow")
            }
            EntityresolutionActions::UpdateIdNamespace => {
                write!(f, "entityresolution:UpdateIdNamespace")
            }
            EntityresolutionActions::UpdateMatchingWorkflow => {
                write!(f, "entityresolution:UpdateMatchingWorkflow")
            }
            EntityresolutionActions::UpdateSchemaMapping => {
                write!(f, "entityresolution:UpdateSchemaMapping")
            }
            EntityresolutionActions::UseIdNamespace => write!(f, "entityresolution:UseIdNamespace"),
            EntityresolutionActions::UseWorkflow => write!(f, "entityresolution:UseWorkflow"),
        }
    }
}
