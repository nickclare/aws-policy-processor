// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum QappsActions {
    AssociateLibraryItemReview,
    AssociateQAppWithUser,
    BatchCreateCategory,
    BatchDeleteCategory,
    BatchUpdateCategory,
    CopyQApp,
    CreateLibraryItem,
    CreateLibraryItemReview,
    CreateQApp,
    CreateSubscriptionToken,
    DeleteLibraryItem,
    DeleteQApp,
    DescribeQAppPermissions,
    DisassociateLibraryItemReview,
    DisassociateQAppFromUser,
    ExportQAppSessionData,
    GetLibraryItem,
    GetQApp,
    GetQAppSession,
    GetQAppSessionMetadata,
    ImportDocument,
    ListCategories,
    ListLibraryItems,
    ListQAppSessionData,
    ListQApps,
    ListTagsForResource,
    PredictProblemStatementFromConversation,
    PredictQApp,
    PredictQAppFromProblemStatement,
    StartQAppSession,
    StopQAppSession,
    TagResource,
    UntagResource,
    UpdateLibraryItem,
    UpdateLibraryItemMetadata,
    UpdateQApp,
    UpdateQAppPermissions,
    UpdateQAppSession,
    UpdateQAppSessionMetadata,
}
impl std::fmt::Display for QappsActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QappsActions::AssociateLibraryItemReview => {
                write!(f, "qapps:AssociateLibraryItemReview")
            }
            QappsActions::AssociateQAppWithUser => write!(f, "qapps:AssociateQAppWithUser"),
            QappsActions::BatchCreateCategory => write!(f, "qapps:BatchCreateCategory"),
            QappsActions::BatchDeleteCategory => write!(f, "qapps:BatchDeleteCategory"),
            QappsActions::BatchUpdateCategory => write!(f, "qapps:BatchUpdateCategory"),
            QappsActions::CopyQApp => write!(f, "qapps:CopyQApp"),
            QappsActions::CreateLibraryItem => write!(f, "qapps:CreateLibraryItem"),
            QappsActions::CreateLibraryItemReview => write!(f, "qapps:CreateLibraryItemReview"),
            QappsActions::CreateQApp => write!(f, "qapps:CreateQApp"),
            QappsActions::CreateSubscriptionToken => write!(f, "qapps:CreateSubscriptionToken"),
            QappsActions::DeleteLibraryItem => write!(f, "qapps:DeleteLibraryItem"),
            QappsActions::DeleteQApp => write!(f, "qapps:DeleteQApp"),
            QappsActions::DescribeQAppPermissions => write!(f, "qapps:DescribeQAppPermissions"),
            QappsActions::DisassociateLibraryItemReview => {
                write!(f, "qapps:DisassociateLibraryItemReview")
            }
            QappsActions::DisassociateQAppFromUser => write!(f, "qapps:DisassociateQAppFromUser"),
            QappsActions::ExportQAppSessionData => write!(f, "qapps:ExportQAppSessionData"),
            QappsActions::GetLibraryItem => write!(f, "qapps:GetLibraryItem"),
            QappsActions::GetQApp => write!(f, "qapps:GetQApp"),
            QappsActions::GetQAppSession => write!(f, "qapps:GetQAppSession"),
            QappsActions::GetQAppSessionMetadata => write!(f, "qapps:GetQAppSessionMetadata"),
            QappsActions::ImportDocument => write!(f, "qapps:ImportDocument"),
            QappsActions::ListCategories => write!(f, "qapps:ListCategories"),
            QappsActions::ListLibraryItems => write!(f, "qapps:ListLibraryItems"),
            QappsActions::ListQAppSessionData => write!(f, "qapps:ListQAppSessionData"),
            QappsActions::ListQApps => write!(f, "qapps:ListQApps"),
            QappsActions::ListTagsForResource => write!(f, "qapps:ListTagsForResource"),
            QappsActions::PredictProblemStatementFromConversation => {
                write!(f, "qapps:PredictProblemStatementFromConversation")
            }
            QappsActions::PredictQApp => write!(f, "qapps:PredictQApp"),
            QappsActions::PredictQAppFromProblemStatement => {
                write!(f, "qapps:PredictQAppFromProblemStatement")
            }
            QappsActions::StartQAppSession => write!(f, "qapps:StartQAppSession"),
            QappsActions::StopQAppSession => write!(f, "qapps:StopQAppSession"),
            QappsActions::TagResource => write!(f, "qapps:TagResource"),
            QappsActions::UntagResource => write!(f, "qapps:UntagResource"),
            QappsActions::UpdateLibraryItem => write!(f, "qapps:UpdateLibraryItem"),
            QappsActions::UpdateLibraryItemMetadata => write!(f, "qapps:UpdateLibraryItemMetadata"),
            QappsActions::UpdateQApp => write!(f, "qapps:UpdateQApp"),
            QappsActions::UpdateQAppPermissions => write!(f, "qapps:UpdateQAppPermissions"),
            QappsActions::UpdateQAppSession => write!(f, "qapps:UpdateQAppSession"),
            QappsActions::UpdateQAppSessionMetadata => write!(f, "qapps:UpdateQAppSessionMetadata"),
        }
    }
}
