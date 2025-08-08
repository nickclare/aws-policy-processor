use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Service {
    /// The name of the service, e.g. ec2, rds, iam, etc.
    pub name: String,
    /// The list of actions that this service supports.
    pub actions: Vec<Action>,
    /// The service-level condition keys.
    pub condition_keys: Vec<ConditionKey>,
    /// The resources supported by this service.
    pub resources: Vec<Resource>,
    /// The version of this Service Reference file
    pub version: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Action {
    pub name: String,
    #[serde(default)]
    pub action_condition_keys: Vec<String>,
    pub annotations: Annotations,
    #[serde(default)]
    pub resources: Vec<Resource>,
    pub supported_by: SupportedBy,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConditionKey {
    pub name: String,
    pub types: Vec<ConditionKeyType>,
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ConditionKeyType {
    ARN,
    ArrayOfString,
    Bool,
    Date,
    Numeric,
    String,
    /// Will have this for now, ideally should never happen
    #[serde(other)]
    Other,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Resource {
    pub name: String,
    #[serde(rename = "ARNFormats", default)]
    pub arn_formats: Vec<String>,
    #[serde(default)]
    pub condition_keys: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Annotations {
    pub properties: Properties,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Properties {
    pub is_list: bool,
    pub is_permission_management: bool,
    pub is_tagging_only: bool,
    pub is_write: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SupportedBy {
    #[serde(rename = "IAM Access Analyzer Policy Generation")]
    pub access_analyser_policy_generate: bool,
    #[serde(rename = "IAM Action Last Accessed")]
    pub action_last_accessed: bool,
}
