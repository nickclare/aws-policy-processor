// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum AppIntegrationsActions {
    CreateApplication,
    CreateApplicationAssociation,
    CreateDataIntegration,
    CreateDataIntegrationAssociation,
    CreateDataIntegrationSchedule,
    CreateEventIntegration,
    CreateEventIntegrationAssociation,
    DeleteApplication,
    DeleteApplicationAssociation,
    DeleteDataIntegration,
    DeleteDataIntegrationAssociation,
    DeleteEventIntegration,
    DeleteEventIntegrationAssociation,
    GetApplication,
    GetDataIntegration,
    GetDataIntegrationExecution,
    GetDataIntegrationSchedule,
    GetEventIntegration,
    ListApplicationAssociations,
    ListApplications,
    ListDataIntegrationAssociations,
    ListDataIntegrationExecutions,
    ListDataIntegrationSchedules,
    ListDataIntegrations,
    ListEventIntegrationAssociations,
    ListEventIntegrations,
    ListTagsForResource,
    StartDataIntegrationExecution,
    TagResource,
    UntagResource,
    UpdateApplication,
    UpdateDataIntegration,
    UpdateDataIntegrationAssociation,
    UpdateDataIntegrationSchedule,
    UpdateEventIntegration,
}
impl std::fmt::Display for AppIntegrationsActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppIntegrationsActions::CreateApplication => {
                write!(f, "app-integrations:CreateApplication")
            }
            AppIntegrationsActions::CreateApplicationAssociation => {
                write!(f, "app-integrations:CreateApplicationAssociation")
            }
            AppIntegrationsActions::CreateDataIntegration => {
                write!(f, "app-integrations:CreateDataIntegration")
            }
            AppIntegrationsActions::CreateDataIntegrationAssociation => {
                write!(f, "app-integrations:CreateDataIntegrationAssociation")
            }
            AppIntegrationsActions::CreateDataIntegrationSchedule => {
                write!(f, "app-integrations:CreateDataIntegrationSchedule")
            }
            AppIntegrationsActions::CreateEventIntegration => {
                write!(f, "app-integrations:CreateEventIntegration")
            }
            AppIntegrationsActions::CreateEventIntegrationAssociation => {
                write!(f, "app-integrations:CreateEventIntegrationAssociation")
            }
            AppIntegrationsActions::DeleteApplication => {
                write!(f, "app-integrations:DeleteApplication")
            }
            AppIntegrationsActions::DeleteApplicationAssociation => {
                write!(f, "app-integrations:DeleteApplicationAssociation")
            }
            AppIntegrationsActions::DeleteDataIntegration => {
                write!(f, "app-integrations:DeleteDataIntegration")
            }
            AppIntegrationsActions::DeleteDataIntegrationAssociation => {
                write!(f, "app-integrations:DeleteDataIntegrationAssociation")
            }
            AppIntegrationsActions::DeleteEventIntegration => {
                write!(f, "app-integrations:DeleteEventIntegration")
            }
            AppIntegrationsActions::DeleteEventIntegrationAssociation => {
                write!(f, "app-integrations:DeleteEventIntegrationAssociation")
            }
            AppIntegrationsActions::GetApplication => write!(f, "app-integrations:GetApplication"),
            AppIntegrationsActions::GetDataIntegration => {
                write!(f, "app-integrations:GetDataIntegration")
            }
            AppIntegrationsActions::GetDataIntegrationExecution => {
                write!(f, "app-integrations:GetDataIntegrationExecution")
            }
            AppIntegrationsActions::GetDataIntegrationSchedule => {
                write!(f, "app-integrations:GetDataIntegrationSchedule")
            }
            AppIntegrationsActions::GetEventIntegration => {
                write!(f, "app-integrations:GetEventIntegration")
            }
            AppIntegrationsActions::ListApplicationAssociations => {
                write!(f, "app-integrations:ListApplicationAssociations")
            }
            AppIntegrationsActions::ListApplications => {
                write!(f, "app-integrations:ListApplications")
            }
            AppIntegrationsActions::ListDataIntegrationAssociations => {
                write!(f, "app-integrations:ListDataIntegrationAssociations")
            }
            AppIntegrationsActions::ListDataIntegrationExecutions => {
                write!(f, "app-integrations:ListDataIntegrationExecutions")
            }
            AppIntegrationsActions::ListDataIntegrationSchedules => {
                write!(f, "app-integrations:ListDataIntegrationSchedules")
            }
            AppIntegrationsActions::ListDataIntegrations => {
                write!(f, "app-integrations:ListDataIntegrations")
            }
            AppIntegrationsActions::ListEventIntegrationAssociations => {
                write!(f, "app-integrations:ListEventIntegrationAssociations")
            }
            AppIntegrationsActions::ListEventIntegrations => {
                write!(f, "app-integrations:ListEventIntegrations")
            }
            AppIntegrationsActions::ListTagsForResource => {
                write!(f, "app-integrations:ListTagsForResource")
            }
            AppIntegrationsActions::StartDataIntegrationExecution => {
                write!(f, "app-integrations:StartDataIntegrationExecution")
            }
            AppIntegrationsActions::TagResource => write!(f, "app-integrations:TagResource"),
            AppIntegrationsActions::UntagResource => write!(f, "app-integrations:UntagResource"),
            AppIntegrationsActions::UpdateApplication => {
                write!(f, "app-integrations:UpdateApplication")
            }
            AppIntegrationsActions::UpdateDataIntegration => {
                write!(f, "app-integrations:UpdateDataIntegration")
            }
            AppIntegrationsActions::UpdateDataIntegrationAssociation => {
                write!(f, "app-integrations:UpdateDataIntegrationAssociation")
            }
            AppIntegrationsActions::UpdateDataIntegrationSchedule => {
                write!(f, "app-integrations:UpdateDataIntegrationSchedule")
            }
            AppIntegrationsActions::UpdateEventIntegration => {
                write!(f, "app-integrations:UpdateEventIntegration")
            }
        }
    }
}
