// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum DbqmsActions {
    CreateFavoriteQuery,
    CreateQueryHistory,
    CreateTab,
    DeleteFavoriteQueries,
    DeleteQueryHistory,
    DeleteTab,
    DescribeFavoriteQueries,
    DescribeQueryHistory,
    DescribeTabs,
    GetQueryString,
    UpdateFavoriteQuery,
    UpdateQueryHistory,
    UpdateTab,
}
impl std::fmt::Display for DbqmsActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DbqmsActions::CreateFavoriteQuery => write!(f, "dbqms:CreateFavoriteQuery"),
            DbqmsActions::CreateQueryHistory => write!(f, "dbqms:CreateQueryHistory"),
            DbqmsActions::CreateTab => write!(f, "dbqms:CreateTab"),
            DbqmsActions::DeleteFavoriteQueries => write!(f, "dbqms:DeleteFavoriteQueries"),
            DbqmsActions::DeleteQueryHistory => write!(f, "dbqms:DeleteQueryHistory"),
            DbqmsActions::DeleteTab => write!(f, "dbqms:DeleteTab"),
            DbqmsActions::DescribeFavoriteQueries => write!(f, "dbqms:DescribeFavoriteQueries"),
            DbqmsActions::DescribeQueryHistory => write!(f, "dbqms:DescribeQueryHistory"),
            DbqmsActions::DescribeTabs => write!(f, "dbqms:DescribeTabs"),
            DbqmsActions::GetQueryString => write!(f, "dbqms:GetQueryString"),
            DbqmsActions::UpdateFavoriteQuery => write!(f, "dbqms:UpdateFavoriteQuery"),
            DbqmsActions::UpdateQueryHistory => write!(f, "dbqms:UpdateQueryHistory"),
            DbqmsActions::UpdateTab => write!(f, "dbqms:UpdateTab"),
        }
    }
}
