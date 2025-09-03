// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum InspectorScanActions {
    ScanSbom,
}
impl std::fmt::Display for InspectorScanActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InspectorScanActions::ScanSbom => write!(f, "inspector-scan:ScanSbom"),
        }
    }
}
