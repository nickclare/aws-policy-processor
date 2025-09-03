// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum KinesisanalyticsActions {
    AddApplicationCloudWatchLoggingOption,
    AddApplicationInput,
    AddApplicationInputProcessingConfiguration,
    AddApplicationOutput,
    AddApplicationReferenceDataSource,
    AddApplicationVpcConfiguration,
    CreateApplication,
    CreateApplicationPresignedUrl,
    CreateApplicationSnapshot,
    DeleteApplication,
    DeleteApplicationCloudWatchLoggingOption,
    DeleteApplicationInputProcessingConfiguration,
    DeleteApplicationOutput,
    DeleteApplicationReferenceDataSource,
    DeleteApplicationSnapshot,
    DeleteApplicationVpcConfiguration,
    DescribeApplication,
    DescribeApplicationOperation,
    DescribeApplicationSnapshot,
    DescribeApplicationVersion,
    DiscoverInputSchema,
    GetApplicationState,
    ListApplicationOperations,
    ListApplicationSnapshots,
    ListApplicationVersions,
    ListApplications,
    ListTagsForResource,
    RollbackApplication,
    StartApplication,
    StopApplication,
    TagResource,
    UntagResource,
    UpdateApplication,
    UpdateApplicationMaintenanceConfiguration,
}
impl std::fmt::Display for KinesisanalyticsActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KinesisanalyticsActions::AddApplicationCloudWatchLoggingOption => {
                write!(f, "kinesisanalytics:AddApplicationCloudWatchLoggingOption")
            }
            KinesisanalyticsActions::AddApplicationInput => {
                write!(f, "kinesisanalytics:AddApplicationInput")
            }
            KinesisanalyticsActions::AddApplicationInputProcessingConfiguration => write!(
                f,
                "kinesisanalytics:AddApplicationInputProcessingConfiguration"
            ),
            KinesisanalyticsActions::AddApplicationOutput => {
                write!(f, "kinesisanalytics:AddApplicationOutput")
            }
            KinesisanalyticsActions::AddApplicationReferenceDataSource => {
                write!(f, "kinesisanalytics:AddApplicationReferenceDataSource")
            }
            KinesisanalyticsActions::AddApplicationVpcConfiguration => {
                write!(f, "kinesisanalytics:AddApplicationVpcConfiguration")
            }
            KinesisanalyticsActions::CreateApplication => {
                write!(f, "kinesisanalytics:CreateApplication")
            }
            KinesisanalyticsActions::CreateApplicationPresignedUrl => {
                write!(f, "kinesisanalytics:CreateApplicationPresignedUrl")
            }
            KinesisanalyticsActions::CreateApplicationSnapshot => {
                write!(f, "kinesisanalytics:CreateApplicationSnapshot")
            }
            KinesisanalyticsActions::DeleteApplication => {
                write!(f, "kinesisanalytics:DeleteApplication")
            }
            KinesisanalyticsActions::DeleteApplicationCloudWatchLoggingOption => write!(
                f,
                "kinesisanalytics:DeleteApplicationCloudWatchLoggingOption"
            ),
            KinesisanalyticsActions::DeleteApplicationInputProcessingConfiguration => write!(
                f,
                "kinesisanalytics:DeleteApplicationInputProcessingConfiguration"
            ),
            KinesisanalyticsActions::DeleteApplicationOutput => {
                write!(f, "kinesisanalytics:DeleteApplicationOutput")
            }
            KinesisanalyticsActions::DeleteApplicationReferenceDataSource => {
                write!(f, "kinesisanalytics:DeleteApplicationReferenceDataSource")
            }
            KinesisanalyticsActions::DeleteApplicationSnapshot => {
                write!(f, "kinesisanalytics:DeleteApplicationSnapshot")
            }
            KinesisanalyticsActions::DeleteApplicationVpcConfiguration => {
                write!(f, "kinesisanalytics:DeleteApplicationVpcConfiguration")
            }
            KinesisanalyticsActions::DescribeApplication => {
                write!(f, "kinesisanalytics:DescribeApplication")
            }
            KinesisanalyticsActions::DescribeApplicationOperation => {
                write!(f, "kinesisanalytics:DescribeApplicationOperation")
            }
            KinesisanalyticsActions::DescribeApplicationSnapshot => {
                write!(f, "kinesisanalytics:DescribeApplicationSnapshot")
            }
            KinesisanalyticsActions::DescribeApplicationVersion => {
                write!(f, "kinesisanalytics:DescribeApplicationVersion")
            }
            KinesisanalyticsActions::DiscoverInputSchema => {
                write!(f, "kinesisanalytics:DiscoverInputSchema")
            }
            KinesisanalyticsActions::GetApplicationState => {
                write!(f, "kinesisanalytics:GetApplicationState")
            }
            KinesisanalyticsActions::ListApplicationOperations => {
                write!(f, "kinesisanalytics:ListApplicationOperations")
            }
            KinesisanalyticsActions::ListApplicationSnapshots => {
                write!(f, "kinesisanalytics:ListApplicationSnapshots")
            }
            KinesisanalyticsActions::ListApplicationVersions => {
                write!(f, "kinesisanalytics:ListApplicationVersions")
            }
            KinesisanalyticsActions::ListApplications => {
                write!(f, "kinesisanalytics:ListApplications")
            }
            KinesisanalyticsActions::ListTagsForResource => {
                write!(f, "kinesisanalytics:ListTagsForResource")
            }
            KinesisanalyticsActions::RollbackApplication => {
                write!(f, "kinesisanalytics:RollbackApplication")
            }
            KinesisanalyticsActions::StartApplication => {
                write!(f, "kinesisanalytics:StartApplication")
            }
            KinesisanalyticsActions::StopApplication => {
                write!(f, "kinesisanalytics:StopApplication")
            }
            KinesisanalyticsActions::TagResource => write!(f, "kinesisanalytics:TagResource"),
            KinesisanalyticsActions::UntagResource => write!(f, "kinesisanalytics:UntagResource"),
            KinesisanalyticsActions::UpdateApplication => {
                write!(f, "kinesisanalytics:UpdateApplication")
            }
            KinesisanalyticsActions::UpdateApplicationMaintenanceConfiguration => write!(
                f,
                "kinesisanalytics:UpdateApplicationMaintenanceConfiguration"
            ),
        }
    }
}
