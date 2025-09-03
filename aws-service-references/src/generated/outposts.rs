// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum OutpostsActions {
    CancelCapacityTask,
    CancelOrder,
    CreateOrder,
    CreateOutpost,
    CreatePrivateConnectivityConfig,
    CreateSite,
    DeleteOutpost,
    DeleteSite,
    GetCapacityTask,
    GetCatalogItem,
    GetConnection,
    GetOrder,
    GetOutpost,
    GetOutpostBillingInformation,
    GetOutpostInstanceTypes,
    GetOutpostSupportedInstanceTypes,
    GetPrivateConnectivityConfig,
    GetSite,
    GetSiteAddress,
    ListAssetInstances,
    ListAssets,
    ListBlockingInstancesForCapacityTask,
    ListCapacityTasks,
    ListCatalogItems,
    ListOrders,
    ListOutposts,
    ListSites,
    ListTagsForResource,
    StartCapacityTask,
    StartConnection,
    TagResource,
    UntagResource,
    UpdateOutpost,
    UpdateSite,
    UpdateSiteAddress,
    UpdateSiteRackPhysicalProperties,
}
impl std::fmt::Display for OutpostsActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OutpostsActions::CancelCapacityTask => write!(f, "outposts:CancelCapacityTask"),
            OutpostsActions::CancelOrder => write!(f, "outposts:CancelOrder"),
            OutpostsActions::CreateOrder => write!(f, "outposts:CreateOrder"),
            OutpostsActions::CreateOutpost => write!(f, "outposts:CreateOutpost"),
            OutpostsActions::CreatePrivateConnectivityConfig => {
                write!(f, "outposts:CreatePrivateConnectivityConfig")
            }
            OutpostsActions::CreateSite => write!(f, "outposts:CreateSite"),
            OutpostsActions::DeleteOutpost => write!(f, "outposts:DeleteOutpost"),
            OutpostsActions::DeleteSite => write!(f, "outposts:DeleteSite"),
            OutpostsActions::GetCapacityTask => write!(f, "outposts:GetCapacityTask"),
            OutpostsActions::GetCatalogItem => write!(f, "outposts:GetCatalogItem"),
            OutpostsActions::GetConnection => write!(f, "outposts:GetConnection"),
            OutpostsActions::GetOrder => write!(f, "outposts:GetOrder"),
            OutpostsActions::GetOutpost => write!(f, "outposts:GetOutpost"),
            OutpostsActions::GetOutpostBillingInformation => {
                write!(f, "outposts:GetOutpostBillingInformation")
            }
            OutpostsActions::GetOutpostInstanceTypes => {
                write!(f, "outposts:GetOutpostInstanceTypes")
            }
            OutpostsActions::GetOutpostSupportedInstanceTypes => {
                write!(f, "outposts:GetOutpostSupportedInstanceTypes")
            }
            OutpostsActions::GetPrivateConnectivityConfig => {
                write!(f, "outposts:GetPrivateConnectivityConfig")
            }
            OutpostsActions::GetSite => write!(f, "outposts:GetSite"),
            OutpostsActions::GetSiteAddress => write!(f, "outposts:GetSiteAddress"),
            OutpostsActions::ListAssetInstances => write!(f, "outposts:ListAssetInstances"),
            OutpostsActions::ListAssets => write!(f, "outposts:ListAssets"),
            OutpostsActions::ListBlockingInstancesForCapacityTask => {
                write!(f, "outposts:ListBlockingInstancesForCapacityTask")
            }
            OutpostsActions::ListCapacityTasks => write!(f, "outposts:ListCapacityTasks"),
            OutpostsActions::ListCatalogItems => write!(f, "outposts:ListCatalogItems"),
            OutpostsActions::ListOrders => write!(f, "outposts:ListOrders"),
            OutpostsActions::ListOutposts => write!(f, "outposts:ListOutposts"),
            OutpostsActions::ListSites => write!(f, "outposts:ListSites"),
            OutpostsActions::ListTagsForResource => write!(f, "outposts:ListTagsForResource"),
            OutpostsActions::StartCapacityTask => write!(f, "outposts:StartCapacityTask"),
            OutpostsActions::StartConnection => write!(f, "outposts:StartConnection"),
            OutpostsActions::TagResource => write!(f, "outposts:TagResource"),
            OutpostsActions::UntagResource => write!(f, "outposts:UntagResource"),
            OutpostsActions::UpdateOutpost => write!(f, "outposts:UpdateOutpost"),
            OutpostsActions::UpdateSite => write!(f, "outposts:UpdateSite"),
            OutpostsActions::UpdateSiteAddress => write!(f, "outposts:UpdateSiteAddress"),
            OutpostsActions::UpdateSiteRackPhysicalProperties => {
                write!(f, "outposts:UpdateSiteRackPhysicalProperties")
            }
        }
    }
}
