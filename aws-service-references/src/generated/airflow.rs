// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum AirflowActions {
    CreateCliToken,
    CreateEnvironment,
    CreateWebLoginToken,
    DeleteEnvironment,
    GetEnvironment,
    InvokeRestApi,
    ListEnvironments,
    ListTagsForResource,
    PublishMetrics,
    TagResource,
    UntagResource,
    UpdateEnvironment,
}
impl std::fmt::Display for AirflowActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AirflowActions::CreateCliToken => write!(f, "airflow:CreateCliToken"),
            AirflowActions::CreateEnvironment => write!(f, "airflow:CreateEnvironment"),
            AirflowActions::CreateWebLoginToken => write!(f, "airflow:CreateWebLoginToken"),
            AirflowActions::DeleteEnvironment => write!(f, "airflow:DeleteEnvironment"),
            AirflowActions::GetEnvironment => write!(f, "airflow:GetEnvironment"),
            AirflowActions::InvokeRestApi => write!(f, "airflow:InvokeRestApi"),
            AirflowActions::ListEnvironments => write!(f, "airflow:ListEnvironments"),
            AirflowActions::ListTagsForResource => write!(f, "airflow:ListTagsForResource"),
            AirflowActions::PublishMetrics => write!(f, "airflow:PublishMetrics"),
            AirflowActions::TagResource => write!(f, "airflow:TagResource"),
            AirflowActions::UntagResource => write!(f, "airflow:UntagResource"),
            AirflowActions::UpdateEnvironment => write!(f, "airflow:UpdateEnvironment"),
        }
    }
}
