// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum TextractActions {
    AnalyzeDocument,
    AnalyzeExpense,
    AnalyzeId,
    CreateAdapter,
    CreateAdapterVersion,
    DeleteAdapter,
    DeleteAdapterVersion,
    DetectDocumentText,
    GetAdapter,
    GetAdapterVersion,
    GetDocumentAnalysis,
    GetDocumentTextDetection,
    GetExpenseAnalysis,
    GetLendingAnalysis,
    GetLendingAnalysisSummary,
    ListAdapterVersions,
    ListAdapters,
    ListTagsForResource,
    StartDocumentAnalysis,
    StartDocumentTextDetection,
    StartExpenseAnalysis,
    StartLendingAnalysis,
    TagResource,
    UntagResource,
    UpdateAdapter,
}
impl std::fmt::Display for TextractActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TextractActions::AnalyzeDocument => write!(f, "textract:AnalyzeDocument"),
            TextractActions::AnalyzeExpense => write!(f, "textract:AnalyzeExpense"),
            TextractActions::AnalyzeId => write!(f, "textract:AnalyzeID"),
            TextractActions::CreateAdapter => write!(f, "textract:CreateAdapter"),
            TextractActions::CreateAdapterVersion => write!(f, "textract:CreateAdapterVersion"),
            TextractActions::DeleteAdapter => write!(f, "textract:DeleteAdapter"),
            TextractActions::DeleteAdapterVersion => write!(f, "textract:DeleteAdapterVersion"),
            TextractActions::DetectDocumentText => write!(f, "textract:DetectDocumentText"),
            TextractActions::GetAdapter => write!(f, "textract:GetAdapter"),
            TextractActions::GetAdapterVersion => write!(f, "textract:GetAdapterVersion"),
            TextractActions::GetDocumentAnalysis => write!(f, "textract:GetDocumentAnalysis"),
            TextractActions::GetDocumentTextDetection => {
                write!(f, "textract:GetDocumentTextDetection")
            }
            TextractActions::GetExpenseAnalysis => write!(f, "textract:GetExpenseAnalysis"),
            TextractActions::GetLendingAnalysis => write!(f, "textract:GetLendingAnalysis"),
            TextractActions::GetLendingAnalysisSummary => {
                write!(f, "textract:GetLendingAnalysisSummary")
            }
            TextractActions::ListAdapterVersions => write!(f, "textract:ListAdapterVersions"),
            TextractActions::ListAdapters => write!(f, "textract:ListAdapters"),
            TextractActions::ListTagsForResource => write!(f, "textract:ListTagsForResource"),
            TextractActions::StartDocumentAnalysis => write!(f, "textract:StartDocumentAnalysis"),
            TextractActions::StartDocumentTextDetection => {
                write!(f, "textract:StartDocumentTextDetection")
            }
            TextractActions::StartExpenseAnalysis => write!(f, "textract:StartExpenseAnalysis"),
            TextractActions::StartLendingAnalysis => write!(f, "textract:StartLendingAnalysis"),
            TextractActions::TagResource => write!(f, "textract:TagResource"),
            TextractActions::UntagResource => write!(f, "textract:UntagResource"),
            TextractActions::UpdateAdapter => write!(f, "textract:UpdateAdapter"),
        }
    }
}
