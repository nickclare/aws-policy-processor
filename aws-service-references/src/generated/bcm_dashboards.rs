// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum BcmDashboardsActions {
    CreateDashboard,
    DeleteDashboard,
    GetDashboard,
    GetResourcePolicy,
    ListDashboards,
    ListTagsForResource,
    TagResource,
    UntagResource,
    UpdateDashboard,
}
impl std::fmt::Display for BcmDashboardsActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BcmDashboardsActions::CreateDashboard => write!(f, "bcm-dashboards:CreateDashboard"),
            BcmDashboardsActions::DeleteDashboard => write!(f, "bcm-dashboards:DeleteDashboard"),
            BcmDashboardsActions::GetDashboard => write!(f, "bcm-dashboards:GetDashboard"),
            BcmDashboardsActions::GetResourcePolicy => {
                write!(f, "bcm-dashboards:GetResourcePolicy")
            }
            BcmDashboardsActions::ListDashboards => write!(f, "bcm-dashboards:ListDashboards"),
            BcmDashboardsActions::ListTagsForResource => {
                write!(f, "bcm-dashboards:ListTagsForResource")
            }
            BcmDashboardsActions::TagResource => write!(f, "bcm-dashboards:TagResource"),
            BcmDashboardsActions::UntagResource => write!(f, "bcm-dashboards:UntagResource"),
            BcmDashboardsActions::UpdateDashboard => write!(f, "bcm-dashboards:UpdateDashboard"),
        }
    }
}
