// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum SdbActions {
    BatchDeleteAttributes,
    BatchPutAttributes,
    CreateDomain,
    DeleteAttributes,
    DeleteDomain,
    DomainMetadata,
    GetAttributes,
    ListDomains,
    PutAttributes,
    Select,
}
impl std::fmt::Display for SdbActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SdbActions::BatchDeleteAttributes => write!(f, "sdb:BatchDeleteAttributes"),
            SdbActions::BatchPutAttributes => write!(f, "sdb:BatchPutAttributes"),
            SdbActions::CreateDomain => write!(f, "sdb:CreateDomain"),
            SdbActions::DeleteAttributes => write!(f, "sdb:DeleteAttributes"),
            SdbActions::DeleteDomain => write!(f, "sdb:DeleteDomain"),
            SdbActions::DomainMetadata => write!(f, "sdb:DomainMetadata"),
            SdbActions::GetAttributes => write!(f, "sdb:GetAttributes"),
            SdbActions::ListDomains => write!(f, "sdb:ListDomains"),
            SdbActions::PutAttributes => write!(f, "sdb:PutAttributes"),
            SdbActions::Select => write!(f, "sdb:Select"),
        }
    }
}
