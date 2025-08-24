// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum MediaimportActions {
    CreateDatabaseBinarySnapshot,
}
impl std::fmt::Display for MediaimportActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MediaimportActions::CreateDatabaseBinarySnapshot => {
                write!(f, "mediaimport:CreateDatabaseBinarySnapshot")
            }
        }
    }
}
