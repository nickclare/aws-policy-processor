// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum SecretsmanagerActions {
    BatchGetSecretValue,
    CancelRotateSecret,
    CreateSecret,
    DeleteResourcePolicy,
    DeleteSecret,
    DescribeSecret,
    GetRandomPassword,
    GetResourcePolicy,
    GetSecretValue,
    ListSecretVersionIds,
    ListSecrets,
    PutResourcePolicy,
    PutSecretValue,
    RemoveRegionsFromReplication,
    ReplicateSecretToRegions,
    RestoreSecret,
    RotateSecret,
    StopReplicationToReplica,
    TagResource,
    UntagResource,
    UpdateSecret,
    UpdateSecretVersionStage,
    ValidateResourcePolicy,
}
impl std::fmt::Display for SecretsmanagerActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SecretsmanagerActions::BatchGetSecretValue => {
                write!(f, "secretsmanager:BatchGetSecretValue")
            }
            SecretsmanagerActions::CancelRotateSecret => {
                write!(f, "secretsmanager:CancelRotateSecret")
            }
            SecretsmanagerActions::CreateSecret => write!(f, "secretsmanager:CreateSecret"),
            SecretsmanagerActions::DeleteResourcePolicy => {
                write!(f, "secretsmanager:DeleteResourcePolicy")
            }
            SecretsmanagerActions::DeleteSecret => write!(f, "secretsmanager:DeleteSecret"),
            SecretsmanagerActions::DescribeSecret => write!(f, "secretsmanager:DescribeSecret"),
            SecretsmanagerActions::GetRandomPassword => {
                write!(f, "secretsmanager:GetRandomPassword")
            }
            SecretsmanagerActions::GetResourcePolicy => {
                write!(f, "secretsmanager:GetResourcePolicy")
            }
            SecretsmanagerActions::GetSecretValue => write!(f, "secretsmanager:GetSecretValue"),
            SecretsmanagerActions::ListSecretVersionIds => {
                write!(f, "secretsmanager:ListSecretVersionIds")
            }
            SecretsmanagerActions::ListSecrets => write!(f, "secretsmanager:ListSecrets"),
            SecretsmanagerActions::PutResourcePolicy => {
                write!(f, "secretsmanager:PutResourcePolicy")
            }
            SecretsmanagerActions::PutSecretValue => write!(f, "secretsmanager:PutSecretValue"),
            SecretsmanagerActions::RemoveRegionsFromReplication => {
                write!(f, "secretsmanager:RemoveRegionsFromReplication")
            }
            SecretsmanagerActions::ReplicateSecretToRegions => {
                write!(f, "secretsmanager:ReplicateSecretToRegions")
            }
            SecretsmanagerActions::RestoreSecret => write!(f, "secretsmanager:RestoreSecret"),
            SecretsmanagerActions::RotateSecret => write!(f, "secretsmanager:RotateSecret"),
            SecretsmanagerActions::StopReplicationToReplica => {
                write!(f, "secretsmanager:StopReplicationToReplica")
            }
            SecretsmanagerActions::TagResource => write!(f, "secretsmanager:TagResource"),
            SecretsmanagerActions::UntagResource => write!(f, "secretsmanager:UntagResource"),
            SecretsmanagerActions::UpdateSecret => write!(f, "secretsmanager:UpdateSecret"),
            SecretsmanagerActions::UpdateSecretVersionStage => {
                write!(f, "secretsmanager:UpdateSecretVersionStage")
            }
            SecretsmanagerActions::ValidateResourcePolicy => {
                write!(f, "secretsmanager:ValidateResourcePolicy")
            }
        }
    }
}
