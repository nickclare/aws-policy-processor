// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum QldbActions {
    CancelJournalKinesisStream,
    CreateLedger,
    DeleteLedger,
    DescribeJournalKinesisStream,
    DescribeJournalS3Export,
    DescribeLedger,
    ExecuteStatement,
    ExportJournalToS3,
    GetBlock,
    GetDigest,
    GetRevision,
    InsertSampleData,
    ListJournalKinesisStreamsForLedger,
    ListJournalS3Exports,
    ListJournalS3ExportsForLedger,
    ListLedgers,
    ListTagsForResource,
    PartiQlCreateIndex,
    PartiQlCreateTable,
    PartiQlDelete,
    PartiQlDropIndex,
    PartiQlDropTable,
    PartiQlHistoryFunction,
    PartiQlInsert,
    PartiQlRedact,
    PartiQlSelect,
    PartiQlUndropTable,
    PartiQlUpdate,
    SendCommand,
    ShowCatalog,
    StreamJournalToKinesis,
    TagResource,
    UntagResource,
    UpdateLedger,
    UpdateLedgerPermissionsMode,
}
impl std::fmt::Display for QldbActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QldbActions::CancelJournalKinesisStream => write!(f, "qldb:CancelJournalKinesisStream"),
            QldbActions::CreateLedger => write!(f, "qldb:CreateLedger"),
            QldbActions::DeleteLedger => write!(f, "qldb:DeleteLedger"),
            QldbActions::DescribeJournalKinesisStream => {
                write!(f, "qldb:DescribeJournalKinesisStream")
            }
            QldbActions::DescribeJournalS3Export => write!(f, "qldb:DescribeJournalS3Export"),
            QldbActions::DescribeLedger => write!(f, "qldb:DescribeLedger"),
            QldbActions::ExecuteStatement => write!(f, "qldb:ExecuteStatement"),
            QldbActions::ExportJournalToS3 => write!(f, "qldb:ExportJournalToS3"),
            QldbActions::GetBlock => write!(f, "qldb:GetBlock"),
            QldbActions::GetDigest => write!(f, "qldb:GetDigest"),
            QldbActions::GetRevision => write!(f, "qldb:GetRevision"),
            QldbActions::InsertSampleData => write!(f, "qldb:InsertSampleData"),
            QldbActions::ListJournalKinesisStreamsForLedger => {
                write!(f, "qldb:ListJournalKinesisStreamsForLedger")
            }
            QldbActions::ListJournalS3Exports => write!(f, "qldb:ListJournalS3Exports"),
            QldbActions::ListJournalS3ExportsForLedger => {
                write!(f, "qldb:ListJournalS3ExportsForLedger")
            }
            QldbActions::ListLedgers => write!(f, "qldb:ListLedgers"),
            QldbActions::ListTagsForResource => write!(f, "qldb:ListTagsForResource"),
            QldbActions::PartiQlCreateIndex => write!(f, "qldb:PartiQLCreateIndex"),
            QldbActions::PartiQlCreateTable => write!(f, "qldb:PartiQLCreateTable"),
            QldbActions::PartiQlDelete => write!(f, "qldb:PartiQLDelete"),
            QldbActions::PartiQlDropIndex => write!(f, "qldb:PartiQLDropIndex"),
            QldbActions::PartiQlDropTable => write!(f, "qldb:PartiQLDropTable"),
            QldbActions::PartiQlHistoryFunction => write!(f, "qldb:PartiQLHistoryFunction"),
            QldbActions::PartiQlInsert => write!(f, "qldb:PartiQLInsert"),
            QldbActions::PartiQlRedact => write!(f, "qldb:PartiQLRedact"),
            QldbActions::PartiQlSelect => write!(f, "qldb:PartiQLSelect"),
            QldbActions::PartiQlUndropTable => write!(f, "qldb:PartiQLUndropTable"),
            QldbActions::PartiQlUpdate => write!(f, "qldb:PartiQLUpdate"),
            QldbActions::SendCommand => write!(f, "qldb:SendCommand"),
            QldbActions::ShowCatalog => write!(f, "qldb:ShowCatalog"),
            QldbActions::StreamJournalToKinesis => write!(f, "qldb:StreamJournalToKinesis"),
            QldbActions::TagResource => write!(f, "qldb:TagResource"),
            QldbActions::UntagResource => write!(f, "qldb:UntagResource"),
            QldbActions::UpdateLedger => write!(f, "qldb:UpdateLedger"),
            QldbActions::UpdateLedgerPermissionsMode => {
                write!(f, "qldb:UpdateLedgerPermissionsMode")
            }
        }
    }
}
