// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum SsmIncidentsActions {
    BatchGetIncidentFindings,
    CreateReplicationSet,
    CreateResponsePlan,
    CreateTimelineEvent,
    DeleteIncidentRecord,
    DeleteReplicationSet,
    DeleteResourcePolicy,
    DeleteResponsePlan,
    DeleteTimelineEvent,
    GetIncidentRecord,
    GetReplicationSet,
    GetResourcePolicies,
    GetResponsePlan,
    GetTimelineEvent,
    ListIncidentFindings,
    ListIncidentRecords,
    ListRelatedItems,
    ListReplicationSets,
    ListResponsePlans,
    ListTagsForResource,
    ListTimelineEvents,
    PutResourcePolicy,
    StartIncident,
    TagResource,
    UntagResource,
    UpdateDeletionProtection,
    UpdateIncidentRecord,
    UpdateRelatedItems,
    UpdateReplicationSet,
    UpdateResponsePlan,
    UpdateTimelineEvent,
}
impl std::fmt::Display for SsmIncidentsActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SsmIncidentsActions::BatchGetIncidentFindings => {
                write!(f, "ssm-incidents:BatchGetIncidentFindings")
            }
            SsmIncidentsActions::CreateReplicationSet => {
                write!(f, "ssm-incidents:CreateReplicationSet")
            }
            SsmIncidentsActions::CreateResponsePlan => {
                write!(f, "ssm-incidents:CreateResponsePlan")
            }
            SsmIncidentsActions::CreateTimelineEvent => {
                write!(f, "ssm-incidents:CreateTimelineEvent")
            }
            SsmIncidentsActions::DeleteIncidentRecord => {
                write!(f, "ssm-incidents:DeleteIncidentRecord")
            }
            SsmIncidentsActions::DeleteReplicationSet => {
                write!(f, "ssm-incidents:DeleteReplicationSet")
            }
            SsmIncidentsActions::DeleteResourcePolicy => {
                write!(f, "ssm-incidents:DeleteResourcePolicy")
            }
            SsmIncidentsActions::DeleteResponsePlan => {
                write!(f, "ssm-incidents:DeleteResponsePlan")
            }
            SsmIncidentsActions::DeleteTimelineEvent => {
                write!(f, "ssm-incidents:DeleteTimelineEvent")
            }
            SsmIncidentsActions::GetIncidentRecord => write!(f, "ssm-incidents:GetIncidentRecord"),
            SsmIncidentsActions::GetReplicationSet => write!(f, "ssm-incidents:GetReplicationSet"),
            SsmIncidentsActions::GetResourcePolicies => {
                write!(f, "ssm-incidents:GetResourcePolicies")
            }
            SsmIncidentsActions::GetResponsePlan => write!(f, "ssm-incidents:GetResponsePlan"),
            SsmIncidentsActions::GetTimelineEvent => write!(f, "ssm-incidents:GetTimelineEvent"),
            SsmIncidentsActions::ListIncidentFindings => {
                write!(f, "ssm-incidents:ListIncidentFindings")
            }
            SsmIncidentsActions::ListIncidentRecords => {
                write!(f, "ssm-incidents:ListIncidentRecords")
            }
            SsmIncidentsActions::ListRelatedItems => write!(f, "ssm-incidents:ListRelatedItems"),
            SsmIncidentsActions::ListReplicationSets => {
                write!(f, "ssm-incidents:ListReplicationSets")
            }
            SsmIncidentsActions::ListResponsePlans => write!(f, "ssm-incidents:ListResponsePlans"),
            SsmIncidentsActions::ListTagsForResource => {
                write!(f, "ssm-incidents:ListTagsForResource")
            }
            SsmIncidentsActions::ListTimelineEvents => {
                write!(f, "ssm-incidents:ListTimelineEvents")
            }
            SsmIncidentsActions::PutResourcePolicy => write!(f, "ssm-incidents:PutResourcePolicy"),
            SsmIncidentsActions::StartIncident => write!(f, "ssm-incidents:StartIncident"),
            SsmIncidentsActions::TagResource => write!(f, "ssm-incidents:TagResource"),
            SsmIncidentsActions::UntagResource => write!(f, "ssm-incidents:UntagResource"),
            SsmIncidentsActions::UpdateDeletionProtection => {
                write!(f, "ssm-incidents:UpdateDeletionProtection")
            }
            SsmIncidentsActions::UpdateIncidentRecord => {
                write!(f, "ssm-incidents:UpdateIncidentRecord")
            }
            SsmIncidentsActions::UpdateRelatedItems => {
                write!(f, "ssm-incidents:UpdateRelatedItems")
            }
            SsmIncidentsActions::UpdateReplicationSet => {
                write!(f, "ssm-incidents:UpdateReplicationSet")
            }
            SsmIncidentsActions::UpdateResponsePlan => {
                write!(f, "ssm-incidents:UpdateResponsePlan")
            }
            SsmIncidentsActions::UpdateTimelineEvent => {
                write!(f, "ssm-incidents:UpdateTimelineEvent")
            }
        }
    }
}
