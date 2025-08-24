// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum BackupSearchActions {
    GetSearchJob,
    GetSearchResultExportJob,
    ListSearchJobBackups,
    ListSearchJobResults,
    ListSearchJobs,
    ListSearchResultExportJobs,
    ListTagsForResource,
    StartSearchJob,
    StartSearchResultExportJob,
    StopSearchJob,
    TagResource,
    UntagResource,
}
impl std::fmt::Display for BackupSearchActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BackupSearchActions::GetSearchJob => write!(f, "backup-search:GetSearchJob"),
            BackupSearchActions::GetSearchResultExportJob => {
                write!(f, "backup-search:GetSearchResultExportJob")
            }
            BackupSearchActions::ListSearchJobBackups => {
                write!(f, "backup-search:ListSearchJobBackups")
            }
            BackupSearchActions::ListSearchJobResults => {
                write!(f, "backup-search:ListSearchJobResults")
            }
            BackupSearchActions::ListSearchJobs => write!(f, "backup-search:ListSearchJobs"),
            BackupSearchActions::ListSearchResultExportJobs => {
                write!(f, "backup-search:ListSearchResultExportJobs")
            }
            BackupSearchActions::ListTagsForResource => {
                write!(f, "backup-search:ListTagsForResource")
            }
            BackupSearchActions::StartSearchJob => write!(f, "backup-search:StartSearchJob"),
            BackupSearchActions::StartSearchResultExportJob => {
                write!(f, "backup-search:StartSearchResultExportJob")
            }
            BackupSearchActions::StopSearchJob => write!(f, "backup-search:StopSearchJob"),
            BackupSearchActions::TagResource => write!(f, "backup-search:TagResource"),
            BackupSearchActions::UntagResource => write!(f, "backup-search:UntagResource"),
        }
    }
}
