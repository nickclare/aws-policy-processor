// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum FirehoseActions {
    CreateDeliveryStream,
    DeleteDeliveryStream,
    DescribeDeliveryStream,
    ListDeliveryStreams,
    ListTagsForDeliveryStream,
    PutRecord,
    PutRecordBatch,
    StartDeliveryStreamEncryption,
    StopDeliveryStreamEncryption,
    TagDeliveryStream,
    UntagDeliveryStream,
    UpdateDestination,
}
impl std::fmt::Display for FirehoseActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FirehoseActions::CreateDeliveryStream => write!(f, "firehose:CreateDeliveryStream"),
            FirehoseActions::DeleteDeliveryStream => write!(f, "firehose:DeleteDeliveryStream"),
            FirehoseActions::DescribeDeliveryStream => write!(f, "firehose:DescribeDeliveryStream"),
            FirehoseActions::ListDeliveryStreams => write!(f, "firehose:ListDeliveryStreams"),
            FirehoseActions::ListTagsForDeliveryStream => {
                write!(f, "firehose:ListTagsForDeliveryStream")
            }
            FirehoseActions::PutRecord => write!(f, "firehose:PutRecord"),
            FirehoseActions::PutRecordBatch => write!(f, "firehose:PutRecordBatch"),
            FirehoseActions::StartDeliveryStreamEncryption => {
                write!(f, "firehose:StartDeliveryStreamEncryption")
            }
            FirehoseActions::StopDeliveryStreamEncryption => {
                write!(f, "firehose:StopDeliveryStreamEncryption")
            }
            FirehoseActions::TagDeliveryStream => write!(f, "firehose:TagDeliveryStream"),
            FirehoseActions::UntagDeliveryStream => write!(f, "firehose:UntagDeliveryStream"),
            FirehoseActions::UpdateDestination => write!(f, "firehose:UpdateDestination"),
        }
    }
}
