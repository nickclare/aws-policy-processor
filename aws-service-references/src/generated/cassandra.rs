// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum CassandraActions {
    Alter,
    AlterMultiRegionResource,
    Create,
    CreateMultiRegionResource,
    Drop,
    DropMultiRegionResource,
    GetRecords,
    GetShardIterator,
    GetStream,
    ListStreams,
    Modify,
    ModifyMultiRegionResource,
    Restore,
    RestoreMultiRegionTable,
    Select,
    SelectMultiRegionResource,
    TagMultiRegionResource,
    TagResource,
    UnTagMultiRegionResource,
    UntagResource,
    UpdatePartitioner,
}
impl std::fmt::Display for CassandraActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CassandraActions::Alter => write!(f, "cassandra:Alter"),
            CassandraActions::AlterMultiRegionResource => {
                write!(f, "cassandra:AlterMultiRegionResource")
            }
            CassandraActions::Create => write!(f, "cassandra:Create"),
            CassandraActions::CreateMultiRegionResource => {
                write!(f, "cassandra:CreateMultiRegionResource")
            }
            CassandraActions::Drop => write!(f, "cassandra:Drop"),
            CassandraActions::DropMultiRegionResource => {
                write!(f, "cassandra:DropMultiRegionResource")
            }
            CassandraActions::GetRecords => write!(f, "cassandra:GetRecords"),
            CassandraActions::GetShardIterator => write!(f, "cassandra:GetShardIterator"),
            CassandraActions::GetStream => write!(f, "cassandra:GetStream"),
            CassandraActions::ListStreams => write!(f, "cassandra:ListStreams"),
            CassandraActions::Modify => write!(f, "cassandra:Modify"),
            CassandraActions::ModifyMultiRegionResource => {
                write!(f, "cassandra:ModifyMultiRegionResource")
            }
            CassandraActions::Restore => write!(f, "cassandra:Restore"),
            CassandraActions::RestoreMultiRegionTable => {
                write!(f, "cassandra:RestoreMultiRegionTable")
            }
            CassandraActions::Select => write!(f, "cassandra:Select"),
            CassandraActions::SelectMultiRegionResource => {
                write!(f, "cassandra:SelectMultiRegionResource")
            }
            CassandraActions::TagMultiRegionResource => {
                write!(f, "cassandra:TagMultiRegionResource")
            }
            CassandraActions::TagResource => write!(f, "cassandra:TagResource"),
            CassandraActions::UnTagMultiRegionResource => {
                write!(f, "cassandra:UnTagMultiRegionResource")
            }
            CassandraActions::UntagResource => write!(f, "cassandra:UntagResource"),
            CassandraActions::UpdatePartitioner => write!(f, "cassandra:UpdatePartitioner"),
        }
    }
}
