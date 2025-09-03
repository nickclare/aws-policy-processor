// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum TimestreamActions {
    CancelQuery,
    CreateBatchLoadTask,
    CreateDatabase,
    CreateScheduledQuery,
    CreateTable,
    DeleteDatabase,
    DeleteScheduledQuery,
    DeleteTable,
    DescribeAccountSettings,
    DescribeBatchLoadTask,
    DescribeDatabase,
    DescribeEndpoints,
    DescribeScheduledQuery,
    DescribeTable,
    ExecuteScheduledQuery,
    GetAwsBackupStatus,
    GetAwsRestoreStatus,
    ListBatchLoadTasks,
    ListDatabases,
    ListMeasures,
    ListScheduledQueries,
    ListTables,
    ListTagsForResource,
    PrepareQuery,
    ResumeBatchLoadTask,
    Select,
    SelectValues,
    StartAwsBackupJob,
    StartAwsRestoreJob,
    TagResource,
    Unload,
    UntagResource,
    UpdateAccountSettings,
    UpdateDatabase,
    UpdateScheduledQuery,
    UpdateTable,
    WriteRecords,
}
impl std::fmt::Display for TimestreamActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TimestreamActions::CancelQuery => write!(f, "timestream:CancelQuery"),
            TimestreamActions::CreateBatchLoadTask => write!(f, "timestream:CreateBatchLoadTask"),
            TimestreamActions::CreateDatabase => write!(f, "timestream:CreateDatabase"),
            TimestreamActions::CreateScheduledQuery => write!(f, "timestream:CreateScheduledQuery"),
            TimestreamActions::CreateTable => write!(f, "timestream:CreateTable"),
            TimestreamActions::DeleteDatabase => write!(f, "timestream:DeleteDatabase"),
            TimestreamActions::DeleteScheduledQuery => write!(f, "timestream:DeleteScheduledQuery"),
            TimestreamActions::DeleteTable => write!(f, "timestream:DeleteTable"),
            TimestreamActions::DescribeAccountSettings => {
                write!(f, "timestream:DescribeAccountSettings")
            }
            TimestreamActions::DescribeBatchLoadTask => {
                write!(f, "timestream:DescribeBatchLoadTask")
            }
            TimestreamActions::DescribeDatabase => write!(f, "timestream:DescribeDatabase"),
            TimestreamActions::DescribeEndpoints => write!(f, "timestream:DescribeEndpoints"),
            TimestreamActions::DescribeScheduledQuery => {
                write!(f, "timestream:DescribeScheduledQuery")
            }
            TimestreamActions::DescribeTable => write!(f, "timestream:DescribeTable"),
            TimestreamActions::ExecuteScheduledQuery => {
                write!(f, "timestream:ExecuteScheduledQuery")
            }
            TimestreamActions::GetAwsBackupStatus => write!(f, "timestream:GetAwsBackupStatus"),
            TimestreamActions::GetAwsRestoreStatus => write!(f, "timestream:GetAwsRestoreStatus"),
            TimestreamActions::ListBatchLoadTasks => write!(f, "timestream:ListBatchLoadTasks"),
            TimestreamActions::ListDatabases => write!(f, "timestream:ListDatabases"),
            TimestreamActions::ListMeasures => write!(f, "timestream:ListMeasures"),
            TimestreamActions::ListScheduledQueries => write!(f, "timestream:ListScheduledQueries"),
            TimestreamActions::ListTables => write!(f, "timestream:ListTables"),
            TimestreamActions::ListTagsForResource => write!(f, "timestream:ListTagsForResource"),
            TimestreamActions::PrepareQuery => write!(f, "timestream:PrepareQuery"),
            TimestreamActions::ResumeBatchLoadTask => write!(f, "timestream:ResumeBatchLoadTask"),
            TimestreamActions::Select => write!(f, "timestream:Select"),
            TimestreamActions::SelectValues => write!(f, "timestream:SelectValues"),
            TimestreamActions::StartAwsBackupJob => write!(f, "timestream:StartAwsBackupJob"),
            TimestreamActions::StartAwsRestoreJob => write!(f, "timestream:StartAwsRestoreJob"),
            TimestreamActions::TagResource => write!(f, "timestream:TagResource"),
            TimestreamActions::Unload => write!(f, "timestream:Unload"),
            TimestreamActions::UntagResource => write!(f, "timestream:UntagResource"),
            TimestreamActions::UpdateAccountSettings => {
                write!(f, "timestream:UpdateAccountSettings")
            }
            TimestreamActions::UpdateDatabase => write!(f, "timestream:UpdateDatabase"),
            TimestreamActions::UpdateScheduledQuery => write!(f, "timestream:UpdateScheduledQuery"),
            TimestreamActions::UpdateTable => write!(f, "timestream:UpdateTable"),
            TimestreamActions::WriteRecords => write!(f, "timestream:WriteRecords"),
        }
    }
}
