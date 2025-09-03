// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum CleanroomsActions {
    BatchGetCollaborationAnalysisTemplate,
    BatchGetSchema,
    BatchGetSchemaAnalysisRule,
    CreateAnalysisTemplate,
    CreateCollaboration,
    CreateConfiguredAudienceModelAssociation,
    CreateConfiguredTable,
    CreateConfiguredTableAnalysisRule,
    CreateConfiguredTableAssociation,
    CreateConfiguredTableAssociationAnalysisRule,
    CreateIdMappingTable,
    CreateIdNamespaceAssociation,
    CreateMembership,
    CreatePrivacyBudgetTemplate,
    DeleteAnalysisTemplate,
    DeleteCollaboration,
    DeleteConfiguredAudienceModelAssociation,
    DeleteConfiguredTable,
    DeleteConfiguredTableAnalysisRule,
    DeleteConfiguredTableAssociation,
    DeleteConfiguredTableAssociationAnalysisRule,
    DeleteIdMappingTable,
    DeleteIdNamespaceAssociation,
    DeleteMember,
    DeleteMembership,
    DeletePrivacyBudgetTemplate,
    GetAnalysisTemplate,
    GetCollaboration,
    GetCollaborationAnalysisTemplate,
    GetCollaborationConfiguredAudienceModelAssociation,
    GetCollaborationIdNamespaceAssociation,
    GetCollaborationPrivacyBudgetTemplate,
    GetConfiguredAudienceModelAssociation,
    GetConfiguredTable,
    GetConfiguredTableAnalysisRule,
    GetConfiguredTableAssociation,
    GetConfiguredTableAssociationAnalysisRule,
    GetIdMappingTable,
    GetIdNamespaceAssociation,
    GetMembership,
    GetPrivacyBudgetTemplate,
    GetProtectedJob,
    GetProtectedQuery,
    GetSchema,
    GetSchemaAnalysisRule,
    ListAnalysisTemplates,
    ListCollaborationAnalysisTemplates,
    ListCollaborationConfiguredAudienceModelAssociations,
    ListCollaborationIdNamespaceAssociations,
    ListCollaborationPrivacyBudgetTemplates,
    ListCollaborationPrivacyBudgets,
    ListCollaborations,
    ListConfiguredAudienceModelAssociations,
    ListConfiguredTableAssociations,
    ListConfiguredTables,
    ListIdMappingTables,
    ListIdNamespaceAssociations,
    ListMembers,
    ListMemberships,
    ListPrivacyBudgetTemplates,
    ListPrivacyBudgets,
    ListProtectedJobs,
    ListProtectedQueries,
    ListSchemas,
    ListTagsForResource,
    PassCollaboration,
    PassMembership,
    PopulateIdMappingTable,
    PreviewPrivacyImpact,
    StartProtectedJob,
    StartProtectedQuery,
    TagResource,
    UntagResource,
    UpdateAnalysisTemplate,
    UpdateCollaboration,
    UpdateConfiguredAudienceModelAssociation,
    UpdateConfiguredTable,
    UpdateConfiguredTableAllowedColumns,
    UpdateConfiguredTableAnalysisRule,
    UpdateConfiguredTableAssociation,
    UpdateConfiguredTableAssociationAnalysisRule,
    UpdateConfiguredTableReference,
    UpdateIdMappingTable,
    UpdateIdNamespaceAssociation,
    UpdateMembership,
    UpdatePrivacyBudgetTemplate,
    UpdateProtectedJob,
    UpdateProtectedQuery,
}
impl std::fmt::Display for CleanroomsActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CleanroomsActions::BatchGetCollaborationAnalysisTemplate => {
                write!(f, "cleanrooms:BatchGetCollaborationAnalysisTemplate")
            }
            CleanroomsActions::BatchGetSchema => write!(f, "cleanrooms:BatchGetSchema"),
            CleanroomsActions::BatchGetSchemaAnalysisRule => {
                write!(f, "cleanrooms:BatchGetSchemaAnalysisRule")
            }
            CleanroomsActions::CreateAnalysisTemplate => {
                write!(f, "cleanrooms:CreateAnalysisTemplate")
            }
            CleanroomsActions::CreateCollaboration => write!(f, "cleanrooms:CreateCollaboration"),
            CleanroomsActions::CreateConfiguredAudienceModelAssociation => {
                write!(f, "cleanrooms:CreateConfiguredAudienceModelAssociation")
            }
            CleanroomsActions::CreateConfiguredTable => {
                write!(f, "cleanrooms:CreateConfiguredTable")
            }
            CleanroomsActions::CreateConfiguredTableAnalysisRule => {
                write!(f, "cleanrooms:CreateConfiguredTableAnalysisRule")
            }
            CleanroomsActions::CreateConfiguredTableAssociation => {
                write!(f, "cleanrooms:CreateConfiguredTableAssociation")
            }
            CleanroomsActions::CreateConfiguredTableAssociationAnalysisRule => {
                write!(f, "cleanrooms:CreateConfiguredTableAssociationAnalysisRule")
            }
            CleanroomsActions::CreateIdMappingTable => write!(f, "cleanrooms:CreateIdMappingTable"),
            CleanroomsActions::CreateIdNamespaceAssociation => {
                write!(f, "cleanrooms:CreateIdNamespaceAssociation")
            }
            CleanroomsActions::CreateMembership => write!(f, "cleanrooms:CreateMembership"),
            CleanroomsActions::CreatePrivacyBudgetTemplate => {
                write!(f, "cleanrooms:CreatePrivacyBudgetTemplate")
            }
            CleanroomsActions::DeleteAnalysisTemplate => {
                write!(f, "cleanrooms:DeleteAnalysisTemplate")
            }
            CleanroomsActions::DeleteCollaboration => write!(f, "cleanrooms:DeleteCollaboration"),
            CleanroomsActions::DeleteConfiguredAudienceModelAssociation => {
                write!(f, "cleanrooms:DeleteConfiguredAudienceModelAssociation")
            }
            CleanroomsActions::DeleteConfiguredTable => {
                write!(f, "cleanrooms:DeleteConfiguredTable")
            }
            CleanroomsActions::DeleteConfiguredTableAnalysisRule => {
                write!(f, "cleanrooms:DeleteConfiguredTableAnalysisRule")
            }
            CleanroomsActions::DeleteConfiguredTableAssociation => {
                write!(f, "cleanrooms:DeleteConfiguredTableAssociation")
            }
            CleanroomsActions::DeleteConfiguredTableAssociationAnalysisRule => {
                write!(f, "cleanrooms:DeleteConfiguredTableAssociationAnalysisRule")
            }
            CleanroomsActions::DeleteIdMappingTable => write!(f, "cleanrooms:DeleteIdMappingTable"),
            CleanroomsActions::DeleteIdNamespaceAssociation => {
                write!(f, "cleanrooms:DeleteIdNamespaceAssociation")
            }
            CleanroomsActions::DeleteMember => write!(f, "cleanrooms:DeleteMember"),
            CleanroomsActions::DeleteMembership => write!(f, "cleanrooms:DeleteMembership"),
            CleanroomsActions::DeletePrivacyBudgetTemplate => {
                write!(f, "cleanrooms:DeletePrivacyBudgetTemplate")
            }
            CleanroomsActions::GetAnalysisTemplate => write!(f, "cleanrooms:GetAnalysisTemplate"),
            CleanroomsActions::GetCollaboration => write!(f, "cleanrooms:GetCollaboration"),
            CleanroomsActions::GetCollaborationAnalysisTemplate => {
                write!(f, "cleanrooms:GetCollaborationAnalysisTemplate")
            }
            CleanroomsActions::GetCollaborationConfiguredAudienceModelAssociation => write!(
                f,
                "cleanrooms:GetCollaborationConfiguredAudienceModelAssociation"
            ),
            CleanroomsActions::GetCollaborationIdNamespaceAssociation => {
                write!(f, "cleanrooms:GetCollaborationIdNamespaceAssociation")
            }
            CleanroomsActions::GetCollaborationPrivacyBudgetTemplate => {
                write!(f, "cleanrooms:GetCollaborationPrivacyBudgetTemplate")
            }
            CleanroomsActions::GetConfiguredAudienceModelAssociation => {
                write!(f, "cleanrooms:GetConfiguredAudienceModelAssociation")
            }
            CleanroomsActions::GetConfiguredTable => write!(f, "cleanrooms:GetConfiguredTable"),
            CleanroomsActions::GetConfiguredTableAnalysisRule => {
                write!(f, "cleanrooms:GetConfiguredTableAnalysisRule")
            }
            CleanroomsActions::GetConfiguredTableAssociation => {
                write!(f, "cleanrooms:GetConfiguredTableAssociation")
            }
            CleanroomsActions::GetConfiguredTableAssociationAnalysisRule => {
                write!(f, "cleanrooms:GetConfiguredTableAssociationAnalysisRule")
            }
            CleanroomsActions::GetIdMappingTable => write!(f, "cleanrooms:GetIdMappingTable"),
            CleanroomsActions::GetIdNamespaceAssociation => {
                write!(f, "cleanrooms:GetIdNamespaceAssociation")
            }
            CleanroomsActions::GetMembership => write!(f, "cleanrooms:GetMembership"),
            CleanroomsActions::GetPrivacyBudgetTemplate => {
                write!(f, "cleanrooms:GetPrivacyBudgetTemplate")
            }
            CleanroomsActions::GetProtectedJob => write!(f, "cleanrooms:GetProtectedJob"),
            CleanroomsActions::GetProtectedQuery => write!(f, "cleanrooms:GetProtectedQuery"),
            CleanroomsActions::GetSchema => write!(f, "cleanrooms:GetSchema"),
            CleanroomsActions::GetSchemaAnalysisRule => {
                write!(f, "cleanrooms:GetSchemaAnalysisRule")
            }
            CleanroomsActions::ListAnalysisTemplates => {
                write!(f, "cleanrooms:ListAnalysisTemplates")
            }
            CleanroomsActions::ListCollaborationAnalysisTemplates => {
                write!(f, "cleanrooms:ListCollaborationAnalysisTemplates")
            }
            CleanroomsActions::ListCollaborationConfiguredAudienceModelAssociations => write!(
                f,
                "cleanrooms:ListCollaborationConfiguredAudienceModelAssociations"
            ),
            CleanroomsActions::ListCollaborationIdNamespaceAssociations => {
                write!(f, "cleanrooms:ListCollaborationIdNamespaceAssociations")
            }
            CleanroomsActions::ListCollaborationPrivacyBudgetTemplates => {
                write!(f, "cleanrooms:ListCollaborationPrivacyBudgetTemplates")
            }
            CleanroomsActions::ListCollaborationPrivacyBudgets => {
                write!(f, "cleanrooms:ListCollaborationPrivacyBudgets")
            }
            CleanroomsActions::ListCollaborations => write!(f, "cleanrooms:ListCollaborations"),
            CleanroomsActions::ListConfiguredAudienceModelAssociations => {
                write!(f, "cleanrooms:ListConfiguredAudienceModelAssociations")
            }
            CleanroomsActions::ListConfiguredTableAssociations => {
                write!(f, "cleanrooms:ListConfiguredTableAssociations")
            }
            CleanroomsActions::ListConfiguredTables => write!(f, "cleanrooms:ListConfiguredTables"),
            CleanroomsActions::ListIdMappingTables => write!(f, "cleanrooms:ListIdMappingTables"),
            CleanroomsActions::ListIdNamespaceAssociations => {
                write!(f, "cleanrooms:ListIdNamespaceAssociations")
            }
            CleanroomsActions::ListMembers => write!(f, "cleanrooms:ListMembers"),
            CleanroomsActions::ListMemberships => write!(f, "cleanrooms:ListMemberships"),
            CleanroomsActions::ListPrivacyBudgetTemplates => {
                write!(f, "cleanrooms:ListPrivacyBudgetTemplates")
            }
            CleanroomsActions::ListPrivacyBudgets => write!(f, "cleanrooms:ListPrivacyBudgets"),
            CleanroomsActions::ListProtectedJobs => write!(f, "cleanrooms:ListProtectedJobs"),
            CleanroomsActions::ListProtectedQueries => write!(f, "cleanrooms:ListProtectedQueries"),
            CleanroomsActions::ListSchemas => write!(f, "cleanrooms:ListSchemas"),
            CleanroomsActions::ListTagsForResource => write!(f, "cleanrooms:ListTagsForResource"),
            CleanroomsActions::PassCollaboration => write!(f, "cleanrooms:PassCollaboration"),
            CleanroomsActions::PassMembership => write!(f, "cleanrooms:PassMembership"),
            CleanroomsActions::PopulateIdMappingTable => {
                write!(f, "cleanrooms:PopulateIdMappingTable")
            }
            CleanroomsActions::PreviewPrivacyImpact => write!(f, "cleanrooms:PreviewPrivacyImpact"),
            CleanroomsActions::StartProtectedJob => write!(f, "cleanrooms:StartProtectedJob"),
            CleanroomsActions::StartProtectedQuery => write!(f, "cleanrooms:StartProtectedQuery"),
            CleanroomsActions::TagResource => write!(f, "cleanrooms:TagResource"),
            CleanroomsActions::UntagResource => write!(f, "cleanrooms:UntagResource"),
            CleanroomsActions::UpdateAnalysisTemplate => {
                write!(f, "cleanrooms:UpdateAnalysisTemplate")
            }
            CleanroomsActions::UpdateCollaboration => write!(f, "cleanrooms:UpdateCollaboration"),
            CleanroomsActions::UpdateConfiguredAudienceModelAssociation => {
                write!(f, "cleanrooms:UpdateConfiguredAudienceModelAssociation")
            }
            CleanroomsActions::UpdateConfiguredTable => {
                write!(f, "cleanrooms:UpdateConfiguredTable")
            }
            CleanroomsActions::UpdateConfiguredTableAllowedColumns => {
                write!(f, "cleanrooms:UpdateConfiguredTableAllowedColumns")
            }
            CleanroomsActions::UpdateConfiguredTableAnalysisRule => {
                write!(f, "cleanrooms:UpdateConfiguredTableAnalysisRule")
            }
            CleanroomsActions::UpdateConfiguredTableAssociation => {
                write!(f, "cleanrooms:UpdateConfiguredTableAssociation")
            }
            CleanroomsActions::UpdateConfiguredTableAssociationAnalysisRule => {
                write!(f, "cleanrooms:UpdateConfiguredTableAssociationAnalysisRule")
            }
            CleanroomsActions::UpdateConfiguredTableReference => {
                write!(f, "cleanrooms:UpdateConfiguredTableReference")
            }
            CleanroomsActions::UpdateIdMappingTable => write!(f, "cleanrooms:UpdateIdMappingTable"),
            CleanroomsActions::UpdateIdNamespaceAssociation => {
                write!(f, "cleanrooms:UpdateIdNamespaceAssociation")
            }
            CleanroomsActions::UpdateMembership => write!(f, "cleanrooms:UpdateMembership"),
            CleanroomsActions::UpdatePrivacyBudgetTemplate => {
                write!(f, "cleanrooms:UpdatePrivacyBudgetTemplate")
            }
            CleanroomsActions::UpdateProtectedJob => write!(f, "cleanrooms:UpdateProtectedJob"),
            CleanroomsActions::UpdateProtectedQuery => write!(f, "cleanrooms:UpdateProtectedQuery"),
        }
    }
}
