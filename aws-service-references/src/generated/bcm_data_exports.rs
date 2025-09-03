// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum BcmDataExportsActions {
    CreateExport,
    DeleteExport,
    GetExecution,
    GetExport,
    GetTable,
    ListExecutions,
    ListExports,
    ListTables,
    ListTagsForResource,
    TagResource,
    UntagResource,
    UpdateExport,
}
impl std::fmt::Display for BcmDataExportsActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BcmDataExportsActions::CreateExport => write!(f, "bcm-data-exports:CreateExport"),
            BcmDataExportsActions::DeleteExport => write!(f, "bcm-data-exports:DeleteExport"),
            BcmDataExportsActions::GetExecution => write!(f, "bcm-data-exports:GetExecution"),
            BcmDataExportsActions::GetExport => write!(f, "bcm-data-exports:GetExport"),
            BcmDataExportsActions::GetTable => write!(f, "bcm-data-exports:GetTable"),
            BcmDataExportsActions::ListExecutions => write!(f, "bcm-data-exports:ListExecutions"),
            BcmDataExportsActions::ListExports => write!(f, "bcm-data-exports:ListExports"),
            BcmDataExportsActions::ListTables => write!(f, "bcm-data-exports:ListTables"),
            BcmDataExportsActions::ListTagsForResource => {
                write!(f, "bcm-data-exports:ListTagsForResource")
            }
            BcmDataExportsActions::TagResource => write!(f, "bcm-data-exports:TagResource"),
            BcmDataExportsActions::UntagResource => write!(f, "bcm-data-exports:UntagResource"),
            BcmDataExportsActions::UpdateExport => write!(f, "bcm-data-exports:UpdateExport"),
        }
    }
}
