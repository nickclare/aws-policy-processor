// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum CodeguruSecurityActions {
    BatchGetFindings,
    CreateScan,
    CreateUploadUrl,
    DeleteScansByCategory,
    GetAccountConfiguration,
    GetFindings,
    GetMetricsSummary,
    GetScan,
    ListFindings,
    ListFindingsMetrics,
    ListScans,
    ListTagsForResource,
    TagResource,
    UntagResource,
    UpdateAccountConfiguration,
}
impl std::fmt::Display for CodeguruSecurityActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CodeguruSecurityActions::BatchGetFindings => {
                write!(f, "codeguru-security:BatchGetFindings")
            }
            CodeguruSecurityActions::CreateScan => write!(f, "codeguru-security:CreateScan"),
            CodeguruSecurityActions::CreateUploadUrl => {
                write!(f, "codeguru-security:CreateUploadUrl")
            }
            CodeguruSecurityActions::DeleteScansByCategory => {
                write!(f, "codeguru-security:DeleteScansByCategory")
            }
            CodeguruSecurityActions::GetAccountConfiguration => {
                write!(f, "codeguru-security:GetAccountConfiguration")
            }
            CodeguruSecurityActions::GetFindings => write!(f, "codeguru-security:GetFindings"),
            CodeguruSecurityActions::GetMetricsSummary => {
                write!(f, "codeguru-security:GetMetricsSummary")
            }
            CodeguruSecurityActions::GetScan => write!(f, "codeguru-security:GetScan"),
            CodeguruSecurityActions::ListFindings => write!(f, "codeguru-security:ListFindings"),
            CodeguruSecurityActions::ListFindingsMetrics => {
                write!(f, "codeguru-security:ListFindingsMetrics")
            }
            CodeguruSecurityActions::ListScans => write!(f, "codeguru-security:ListScans"),
            CodeguruSecurityActions::ListTagsForResource => {
                write!(f, "codeguru-security:ListTagsForResource")
            }
            CodeguruSecurityActions::TagResource => write!(f, "codeguru-security:TagResource"),
            CodeguruSecurityActions::UntagResource => write!(f, "codeguru-security:UntagResource"),
            CodeguruSecurityActions::UpdateAccountConfiguration => {
                write!(f, "codeguru-security:UpdateAccountConfiguration")
            }
        }
    }
}
