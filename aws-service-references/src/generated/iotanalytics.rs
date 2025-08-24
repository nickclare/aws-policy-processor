// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum IotanalyticsActions {
    BatchPutMessage,
    CancelPipelineReprocessing,
    CreateChannel,
    CreateDataset,
    CreateDatasetContent,
    CreateDatastore,
    CreatePipeline,
    DeleteChannel,
    DeleteDataset,
    DeleteDatasetContent,
    DeleteDatastore,
    DeletePipeline,
    DescribeChannel,
    DescribeDataset,
    DescribeDatastore,
    DescribeLoggingOptions,
    DescribePipeline,
    GetDatasetContent,
    ListChannels,
    ListDatasetContents,
    ListDatasets,
    ListDatastores,
    ListPipelines,
    ListTagsForResource,
    PutLoggingOptions,
    RunPipelineActivity,
    SampleChannelData,
    StartPipelineReprocessing,
    TagResource,
    UntagResource,
    UpdateChannel,
    UpdateDataset,
    UpdateDatastore,
    UpdatePipeline,
}
impl std::fmt::Display for IotanalyticsActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IotanalyticsActions::BatchPutMessage => write!(f, "iotanalytics:BatchPutMessage"),
            IotanalyticsActions::CancelPipelineReprocessing => {
                write!(f, "iotanalytics:CancelPipelineReprocessing")
            }
            IotanalyticsActions::CreateChannel => write!(f, "iotanalytics:CreateChannel"),
            IotanalyticsActions::CreateDataset => write!(f, "iotanalytics:CreateDataset"),
            IotanalyticsActions::CreateDatasetContent => {
                write!(f, "iotanalytics:CreateDatasetContent")
            }
            IotanalyticsActions::CreateDatastore => write!(f, "iotanalytics:CreateDatastore"),
            IotanalyticsActions::CreatePipeline => write!(f, "iotanalytics:CreatePipeline"),
            IotanalyticsActions::DeleteChannel => write!(f, "iotanalytics:DeleteChannel"),
            IotanalyticsActions::DeleteDataset => write!(f, "iotanalytics:DeleteDataset"),
            IotanalyticsActions::DeleteDatasetContent => {
                write!(f, "iotanalytics:DeleteDatasetContent")
            }
            IotanalyticsActions::DeleteDatastore => write!(f, "iotanalytics:DeleteDatastore"),
            IotanalyticsActions::DeletePipeline => write!(f, "iotanalytics:DeletePipeline"),
            IotanalyticsActions::DescribeChannel => write!(f, "iotanalytics:DescribeChannel"),
            IotanalyticsActions::DescribeDataset => write!(f, "iotanalytics:DescribeDataset"),
            IotanalyticsActions::DescribeDatastore => write!(f, "iotanalytics:DescribeDatastore"),
            IotanalyticsActions::DescribeLoggingOptions => {
                write!(f, "iotanalytics:DescribeLoggingOptions")
            }
            IotanalyticsActions::DescribePipeline => write!(f, "iotanalytics:DescribePipeline"),
            IotanalyticsActions::GetDatasetContent => write!(f, "iotanalytics:GetDatasetContent"),
            IotanalyticsActions::ListChannels => write!(f, "iotanalytics:ListChannels"),
            IotanalyticsActions::ListDatasetContents => {
                write!(f, "iotanalytics:ListDatasetContents")
            }
            IotanalyticsActions::ListDatasets => write!(f, "iotanalytics:ListDatasets"),
            IotanalyticsActions::ListDatastores => write!(f, "iotanalytics:ListDatastores"),
            IotanalyticsActions::ListPipelines => write!(f, "iotanalytics:ListPipelines"),
            IotanalyticsActions::ListTagsForResource => {
                write!(f, "iotanalytics:ListTagsForResource")
            }
            IotanalyticsActions::PutLoggingOptions => write!(f, "iotanalytics:PutLoggingOptions"),
            IotanalyticsActions::RunPipelineActivity => {
                write!(f, "iotanalytics:RunPipelineActivity")
            }
            IotanalyticsActions::SampleChannelData => write!(f, "iotanalytics:SampleChannelData"),
            IotanalyticsActions::StartPipelineReprocessing => {
                write!(f, "iotanalytics:StartPipelineReprocessing")
            }
            IotanalyticsActions::TagResource => write!(f, "iotanalytics:TagResource"),
            IotanalyticsActions::UntagResource => write!(f, "iotanalytics:UntagResource"),
            IotanalyticsActions::UpdateChannel => write!(f, "iotanalytics:UpdateChannel"),
            IotanalyticsActions::UpdateDataset => write!(f, "iotanalytics:UpdateDataset"),
            IotanalyticsActions::UpdateDatastore => write!(f, "iotanalytics:UpdateDatastore"),
            IotanalyticsActions::UpdatePipeline => write!(f, "iotanalytics:UpdatePipeline"),
        }
    }
}
