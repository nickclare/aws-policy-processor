// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum DynamodbActions {
    BatchGetItem,
    BatchWriteItem,
    ConditionCheckItem,
    CreateBackup,
    CreateGlobalTable,
    CreateGlobalTableWitness,
    CreateTable,
    CreateTableReplica,
    DeleteBackup,
    DeleteGlobalTableWitness,
    DeleteItem,
    DeleteResourcePolicy,
    DeleteTable,
    DeleteTableReplica,
    DescribeBackup,
    DescribeContinuousBackups,
    DescribeContributorInsights,
    DescribeEndpoints,
    DescribeExport,
    DescribeGlobalTable,
    DescribeGlobalTableSettings,
    DescribeImport,
    DescribeKinesisStreamingDestination,
    DescribeLimits,
    DescribeReservedCapacity,
    DescribeReservedCapacityOfferings,
    DescribeStream,
    DescribeTable,
    DescribeTableReplicaAutoScaling,
    DescribeTimeToLive,
    DisableKinesisStreamingDestination,
    EnableKinesisStreamingDestination,
    ExportTableToPointInTime,
    GetAbacStatus,
    GetItem,
    GetRecords,
    GetResourcePolicy,
    GetShardIterator,
    ImportTable,
    ListBackups,
    ListContributorInsights,
    ListExports,
    ListGlobalTables,
    ListImports,
    ListStreams,
    ListTables,
    ListTagsOfResource,
    PartiQlDelete,
    PartiQlInsert,
    PartiQlSelect,
    PartiQlUpdate,
    PurchaseReservedCapacityOfferings,
    PutItem,
    PutResourcePolicy,
    Query,
    RestoreTableFromAwsBackup,
    RestoreTableFromBackup,
    RestoreTableToPointInTime,
    Scan,
    StartAwsBackupJob,
    TagResource,
    UntagResource,
    UpdateAbacStatus,
    UpdateContinuousBackups,
    UpdateContributorInsights,
    UpdateGlobalTable,
    UpdateGlobalTableSettings,
    UpdateGlobalTableVersion,
    UpdateItem,
    UpdateKinesisStreamingDestination,
    UpdateTable,
    UpdateTableReplicaAutoScaling,
    UpdateTimeToLive,
}
impl std::fmt::Display for DynamodbActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DynamodbActions::BatchGetItem => write!(f, "dynamodb:BatchGetItem"),
            DynamodbActions::BatchWriteItem => write!(f, "dynamodb:BatchWriteItem"),
            DynamodbActions::ConditionCheckItem => write!(f, "dynamodb:ConditionCheckItem"),
            DynamodbActions::CreateBackup => write!(f, "dynamodb:CreateBackup"),
            DynamodbActions::CreateGlobalTable => write!(f, "dynamodb:CreateGlobalTable"),
            DynamodbActions::CreateGlobalTableWitness => {
                write!(f, "dynamodb:CreateGlobalTableWitness")
            }
            DynamodbActions::CreateTable => write!(f, "dynamodb:CreateTable"),
            DynamodbActions::CreateTableReplica => write!(f, "dynamodb:CreateTableReplica"),
            DynamodbActions::DeleteBackup => write!(f, "dynamodb:DeleteBackup"),
            DynamodbActions::DeleteGlobalTableWitness => {
                write!(f, "dynamodb:DeleteGlobalTableWitness")
            }
            DynamodbActions::DeleteItem => write!(f, "dynamodb:DeleteItem"),
            DynamodbActions::DeleteResourcePolicy => write!(f, "dynamodb:DeleteResourcePolicy"),
            DynamodbActions::DeleteTable => write!(f, "dynamodb:DeleteTable"),
            DynamodbActions::DeleteTableReplica => write!(f, "dynamodb:DeleteTableReplica"),
            DynamodbActions::DescribeBackup => write!(f, "dynamodb:DescribeBackup"),
            DynamodbActions::DescribeContinuousBackups => {
                write!(f, "dynamodb:DescribeContinuousBackups")
            }
            DynamodbActions::DescribeContributorInsights => {
                write!(f, "dynamodb:DescribeContributorInsights")
            }
            DynamodbActions::DescribeEndpoints => write!(f, "dynamodb:DescribeEndpoints"),
            DynamodbActions::DescribeExport => write!(f, "dynamodb:DescribeExport"),
            DynamodbActions::DescribeGlobalTable => write!(f, "dynamodb:DescribeGlobalTable"),
            DynamodbActions::DescribeGlobalTableSettings => {
                write!(f, "dynamodb:DescribeGlobalTableSettings")
            }
            DynamodbActions::DescribeImport => write!(f, "dynamodb:DescribeImport"),
            DynamodbActions::DescribeKinesisStreamingDestination => {
                write!(f, "dynamodb:DescribeKinesisStreamingDestination")
            }
            DynamodbActions::DescribeLimits => write!(f, "dynamodb:DescribeLimits"),
            DynamodbActions::DescribeReservedCapacity => {
                write!(f, "dynamodb:DescribeReservedCapacity")
            }
            DynamodbActions::DescribeReservedCapacityOfferings => {
                write!(f, "dynamodb:DescribeReservedCapacityOfferings")
            }
            DynamodbActions::DescribeStream => write!(f, "dynamodb:DescribeStream"),
            DynamodbActions::DescribeTable => write!(f, "dynamodb:DescribeTable"),
            DynamodbActions::DescribeTableReplicaAutoScaling => {
                write!(f, "dynamodb:DescribeTableReplicaAutoScaling")
            }
            DynamodbActions::DescribeTimeToLive => write!(f, "dynamodb:DescribeTimeToLive"),
            DynamodbActions::DisableKinesisStreamingDestination => {
                write!(f, "dynamodb:DisableKinesisStreamingDestination")
            }
            DynamodbActions::EnableKinesisStreamingDestination => {
                write!(f, "dynamodb:EnableKinesisStreamingDestination")
            }
            DynamodbActions::ExportTableToPointInTime => {
                write!(f, "dynamodb:ExportTableToPointInTime")
            }
            DynamodbActions::GetAbacStatus => write!(f, "dynamodb:GetAbacStatus"),
            DynamodbActions::GetItem => write!(f, "dynamodb:GetItem"),
            DynamodbActions::GetRecords => write!(f, "dynamodb:GetRecords"),
            DynamodbActions::GetResourcePolicy => write!(f, "dynamodb:GetResourcePolicy"),
            DynamodbActions::GetShardIterator => write!(f, "dynamodb:GetShardIterator"),
            DynamodbActions::ImportTable => write!(f, "dynamodb:ImportTable"),
            DynamodbActions::ListBackups => write!(f, "dynamodb:ListBackups"),
            DynamodbActions::ListContributorInsights => {
                write!(f, "dynamodb:ListContributorInsights")
            }
            DynamodbActions::ListExports => write!(f, "dynamodb:ListExports"),
            DynamodbActions::ListGlobalTables => write!(f, "dynamodb:ListGlobalTables"),
            DynamodbActions::ListImports => write!(f, "dynamodb:ListImports"),
            DynamodbActions::ListStreams => write!(f, "dynamodb:ListStreams"),
            DynamodbActions::ListTables => write!(f, "dynamodb:ListTables"),
            DynamodbActions::ListTagsOfResource => write!(f, "dynamodb:ListTagsOfResource"),
            DynamodbActions::PartiQlDelete => write!(f, "dynamodb:PartiQLDelete"),
            DynamodbActions::PartiQlInsert => write!(f, "dynamodb:PartiQLInsert"),
            DynamodbActions::PartiQlSelect => write!(f, "dynamodb:PartiQLSelect"),
            DynamodbActions::PartiQlUpdate => write!(f, "dynamodb:PartiQLUpdate"),
            DynamodbActions::PurchaseReservedCapacityOfferings => {
                write!(f, "dynamodb:PurchaseReservedCapacityOfferings")
            }
            DynamodbActions::PutItem => write!(f, "dynamodb:PutItem"),
            DynamodbActions::PutResourcePolicy => write!(f, "dynamodb:PutResourcePolicy"),
            DynamodbActions::Query => write!(f, "dynamodb:Query"),
            DynamodbActions::RestoreTableFromAwsBackup => {
                write!(f, "dynamodb:RestoreTableFromAwsBackup")
            }
            DynamodbActions::RestoreTableFromBackup => write!(f, "dynamodb:RestoreTableFromBackup"),
            DynamodbActions::RestoreTableToPointInTime => {
                write!(f, "dynamodb:RestoreTableToPointInTime")
            }
            DynamodbActions::Scan => write!(f, "dynamodb:Scan"),
            DynamodbActions::StartAwsBackupJob => write!(f, "dynamodb:StartAwsBackupJob"),
            DynamodbActions::TagResource => write!(f, "dynamodb:TagResource"),
            DynamodbActions::UntagResource => write!(f, "dynamodb:UntagResource"),
            DynamodbActions::UpdateAbacStatus => write!(f, "dynamodb:UpdateAbacStatus"),
            DynamodbActions::UpdateContinuousBackups => {
                write!(f, "dynamodb:UpdateContinuousBackups")
            }
            DynamodbActions::UpdateContributorInsights => {
                write!(f, "dynamodb:UpdateContributorInsights")
            }
            DynamodbActions::UpdateGlobalTable => write!(f, "dynamodb:UpdateGlobalTable"),
            DynamodbActions::UpdateGlobalTableSettings => {
                write!(f, "dynamodb:UpdateGlobalTableSettings")
            }
            DynamodbActions::UpdateGlobalTableVersion => {
                write!(f, "dynamodb:UpdateGlobalTableVersion")
            }
            DynamodbActions::UpdateItem => write!(f, "dynamodb:UpdateItem"),
            DynamodbActions::UpdateKinesisStreamingDestination => {
                write!(f, "dynamodb:UpdateKinesisStreamingDestination")
            }
            DynamodbActions::UpdateTable => write!(f, "dynamodb:UpdateTable"),
            DynamodbActions::UpdateTableReplicaAutoScaling => {
                write!(f, "dynamodb:UpdateTableReplicaAutoScaling")
            }
            DynamodbActions::UpdateTimeToLive => write!(f, "dynamodb:UpdateTimeToLive"),
        }
    }
}
