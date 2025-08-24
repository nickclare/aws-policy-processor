// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum KinesisActions {
    AddTagsToStream,
    CreateStream,
    DecreaseStreamRetentionPeriod,
    DeleteResourcePolicy,
    DeleteStream,
    DeregisterStreamConsumer,
    DescribeLimits,
    DescribeStream,
    DescribeStreamConsumer,
    DescribeStreamSummary,
    DisableEnhancedMonitoring,
    EnableEnhancedMonitoring,
    GetRecords,
    GetResourcePolicy,
    GetShardIterator,
    IncreaseStreamRetentionPeriod,
    ListShards,
    ListStreamConsumers,
    ListStreams,
    ListTagsForResource,
    ListTagsForStream,
    MergeShards,
    PutRecord,
    PutRecords,
    PutResourcePolicy,
    RegisterStreamConsumer,
    RemoveTagsFromStream,
    SplitShard,
    StartStreamEncryption,
    StopStreamEncryption,
    SubscribeToShard,
    TagResource,
    UntagResource,
    UpdateShardCount,
    UpdateStreamMode,
}
impl std::fmt::Display for KinesisActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KinesisActions::AddTagsToStream => write!(f, "kinesis:AddTagsToStream"),
            KinesisActions::CreateStream => write!(f, "kinesis:CreateStream"),
            KinesisActions::DecreaseStreamRetentionPeriod => {
                write!(f, "kinesis:DecreaseStreamRetentionPeriod")
            }
            KinesisActions::DeleteResourcePolicy => write!(f, "kinesis:DeleteResourcePolicy"),
            KinesisActions::DeleteStream => write!(f, "kinesis:DeleteStream"),
            KinesisActions::DeregisterStreamConsumer => {
                write!(f, "kinesis:DeregisterStreamConsumer")
            }
            KinesisActions::DescribeLimits => write!(f, "kinesis:DescribeLimits"),
            KinesisActions::DescribeStream => write!(f, "kinesis:DescribeStream"),
            KinesisActions::DescribeStreamConsumer => write!(f, "kinesis:DescribeStreamConsumer"),
            KinesisActions::DescribeStreamSummary => write!(f, "kinesis:DescribeStreamSummary"),
            KinesisActions::DisableEnhancedMonitoring => {
                write!(f, "kinesis:DisableEnhancedMonitoring")
            }
            KinesisActions::EnableEnhancedMonitoring => {
                write!(f, "kinesis:EnableEnhancedMonitoring")
            }
            KinesisActions::GetRecords => write!(f, "kinesis:GetRecords"),
            KinesisActions::GetResourcePolicy => write!(f, "kinesis:GetResourcePolicy"),
            KinesisActions::GetShardIterator => write!(f, "kinesis:GetShardIterator"),
            KinesisActions::IncreaseStreamRetentionPeriod => {
                write!(f, "kinesis:IncreaseStreamRetentionPeriod")
            }
            KinesisActions::ListShards => write!(f, "kinesis:ListShards"),
            KinesisActions::ListStreamConsumers => write!(f, "kinesis:ListStreamConsumers"),
            KinesisActions::ListStreams => write!(f, "kinesis:ListStreams"),
            KinesisActions::ListTagsForResource => write!(f, "kinesis:ListTagsForResource"),
            KinesisActions::ListTagsForStream => write!(f, "kinesis:ListTagsForStream"),
            KinesisActions::MergeShards => write!(f, "kinesis:MergeShards"),
            KinesisActions::PutRecord => write!(f, "kinesis:PutRecord"),
            KinesisActions::PutRecords => write!(f, "kinesis:PutRecords"),
            KinesisActions::PutResourcePolicy => write!(f, "kinesis:PutResourcePolicy"),
            KinesisActions::RegisterStreamConsumer => write!(f, "kinesis:RegisterStreamConsumer"),
            KinesisActions::RemoveTagsFromStream => write!(f, "kinesis:RemoveTagsFromStream"),
            KinesisActions::SplitShard => write!(f, "kinesis:SplitShard"),
            KinesisActions::StartStreamEncryption => write!(f, "kinesis:StartStreamEncryption"),
            KinesisActions::StopStreamEncryption => write!(f, "kinesis:StopStreamEncryption"),
            KinesisActions::SubscribeToShard => write!(f, "kinesis:SubscribeToShard"),
            KinesisActions::TagResource => write!(f, "kinesis:TagResource"),
            KinesisActions::UntagResource => write!(f, "kinesis:UntagResource"),
            KinesisActions::UpdateShardCount => write!(f, "kinesis:UpdateShardCount"),
            KinesisActions::UpdateStreamMode => write!(f, "kinesis:UpdateStreamMode"),
        }
    }
}
