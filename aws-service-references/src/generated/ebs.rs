// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum EbsActions {
    CompleteSnapshot,
    GetSnapshotBlock,
    ListChangedBlocks,
    ListSnapshotBlocks,
    PutSnapshotBlock,
    StartSnapshot,
}
impl std::fmt::Display for EbsActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EbsActions::CompleteSnapshot => write!(f, "ebs:CompleteSnapshot"),
            EbsActions::GetSnapshotBlock => write!(f, "ebs:GetSnapshotBlock"),
            EbsActions::ListChangedBlocks => write!(f, "ebs:ListChangedBlocks"),
            EbsActions::ListSnapshotBlocks => write!(f, "ebs:ListSnapshotBlocks"),
            EbsActions::PutSnapshotBlock => write!(f, "ebs:PutSnapshotBlock"),
            EbsActions::StartSnapshot => write!(f, "ebs:StartSnapshot"),
        }
    }
}
