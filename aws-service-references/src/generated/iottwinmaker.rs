// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum IottwinmakerActions {
    BatchPutPropertyValues,
    CancelMetadataTransferJob,
    CreateComponentType,
    CreateEntity,
    CreateMetadataTransferJob,
    CreateScene,
    CreateSyncJob,
    CreateWorkspace,
    DeleteComponentType,
    DeleteEntity,
    DeleteScene,
    DeleteSyncJob,
    DeleteWorkspace,
    ExecuteQuery,
    GetComponentType,
    GetEntity,
    GetMetadataTransferJob,
    GetPricingPlan,
    GetPropertyValue,
    GetPropertyValueHistory,
    GetScene,
    GetSyncJob,
    GetWorkspace,
    ListComponentTypes,
    ListComponents,
    ListEntities,
    ListMetadataTransferJobs,
    ListProperties,
    ListScenes,
    ListSyncJobs,
    ListSyncResources,
    ListTagsForResource,
    ListWorkspaces,
    TagResource,
    UntagResource,
    UpdateComponentType,
    UpdateEntity,
    UpdatePricingPlan,
    UpdateScene,
    UpdateWorkspace,
}
impl std::fmt::Display for IottwinmakerActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IottwinmakerActions::BatchPutPropertyValues => {
                write!(f, "iottwinmaker:BatchPutPropertyValues")
            }
            IottwinmakerActions::CancelMetadataTransferJob => {
                write!(f, "iottwinmaker:CancelMetadataTransferJob")
            }
            IottwinmakerActions::CreateComponentType => {
                write!(f, "iottwinmaker:CreateComponentType")
            }
            IottwinmakerActions::CreateEntity => write!(f, "iottwinmaker:CreateEntity"),
            IottwinmakerActions::CreateMetadataTransferJob => {
                write!(f, "iottwinmaker:CreateMetadataTransferJob")
            }
            IottwinmakerActions::CreateScene => write!(f, "iottwinmaker:CreateScene"),
            IottwinmakerActions::CreateSyncJob => write!(f, "iottwinmaker:CreateSyncJob"),
            IottwinmakerActions::CreateWorkspace => write!(f, "iottwinmaker:CreateWorkspace"),
            IottwinmakerActions::DeleteComponentType => {
                write!(f, "iottwinmaker:DeleteComponentType")
            }
            IottwinmakerActions::DeleteEntity => write!(f, "iottwinmaker:DeleteEntity"),
            IottwinmakerActions::DeleteScene => write!(f, "iottwinmaker:DeleteScene"),
            IottwinmakerActions::DeleteSyncJob => write!(f, "iottwinmaker:DeleteSyncJob"),
            IottwinmakerActions::DeleteWorkspace => write!(f, "iottwinmaker:DeleteWorkspace"),
            IottwinmakerActions::ExecuteQuery => write!(f, "iottwinmaker:ExecuteQuery"),
            IottwinmakerActions::GetComponentType => write!(f, "iottwinmaker:GetComponentType"),
            IottwinmakerActions::GetEntity => write!(f, "iottwinmaker:GetEntity"),
            IottwinmakerActions::GetMetadataTransferJob => {
                write!(f, "iottwinmaker:GetMetadataTransferJob")
            }
            IottwinmakerActions::GetPricingPlan => write!(f, "iottwinmaker:GetPricingPlan"),
            IottwinmakerActions::GetPropertyValue => write!(f, "iottwinmaker:GetPropertyValue"),
            IottwinmakerActions::GetPropertyValueHistory => {
                write!(f, "iottwinmaker:GetPropertyValueHistory")
            }
            IottwinmakerActions::GetScene => write!(f, "iottwinmaker:GetScene"),
            IottwinmakerActions::GetSyncJob => write!(f, "iottwinmaker:GetSyncJob"),
            IottwinmakerActions::GetWorkspace => write!(f, "iottwinmaker:GetWorkspace"),
            IottwinmakerActions::ListComponentTypes => write!(f, "iottwinmaker:ListComponentTypes"),
            IottwinmakerActions::ListComponents => write!(f, "iottwinmaker:ListComponents"),
            IottwinmakerActions::ListEntities => write!(f, "iottwinmaker:ListEntities"),
            IottwinmakerActions::ListMetadataTransferJobs => {
                write!(f, "iottwinmaker:ListMetadataTransferJobs")
            }
            IottwinmakerActions::ListProperties => write!(f, "iottwinmaker:ListProperties"),
            IottwinmakerActions::ListScenes => write!(f, "iottwinmaker:ListScenes"),
            IottwinmakerActions::ListSyncJobs => write!(f, "iottwinmaker:ListSyncJobs"),
            IottwinmakerActions::ListSyncResources => write!(f, "iottwinmaker:ListSyncResources"),
            IottwinmakerActions::ListTagsForResource => {
                write!(f, "iottwinmaker:ListTagsForResource")
            }
            IottwinmakerActions::ListWorkspaces => write!(f, "iottwinmaker:ListWorkspaces"),
            IottwinmakerActions::TagResource => write!(f, "iottwinmaker:TagResource"),
            IottwinmakerActions::UntagResource => write!(f, "iottwinmaker:UntagResource"),
            IottwinmakerActions::UpdateComponentType => {
                write!(f, "iottwinmaker:UpdateComponentType")
            }
            IottwinmakerActions::UpdateEntity => write!(f, "iottwinmaker:UpdateEntity"),
            IottwinmakerActions::UpdatePricingPlan => write!(f, "iottwinmaker:UpdatePricingPlan"),
            IottwinmakerActions::UpdateScene => write!(f, "iottwinmaker:UpdateScene"),
            IottwinmakerActions::UpdateWorkspace => write!(f, "iottwinmaker:UpdateWorkspace"),
        }
    }
}
