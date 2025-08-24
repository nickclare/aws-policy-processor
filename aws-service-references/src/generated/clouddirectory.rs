// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum ClouddirectoryActions {
    AddFacetToObject,
    ApplySchema,
    AttachObject,
    AttachPolicy,
    AttachToIndex,
    AttachTypedLink,
    BatchRead,
    BatchWrite,
    CreateDirectory,
    CreateFacet,
    CreateIndex,
    CreateObject,
    CreateSchema,
    CreateTypedLinkFacet,
    DeleteDirectory,
    DeleteFacet,
    DeleteObject,
    DeleteSchema,
    DeleteTypedLinkFacet,
    DetachFromIndex,
    DetachObject,
    DetachPolicy,
    DetachTypedLink,
    DisableDirectory,
    EnableDirectory,
    GetAppliedSchemaVersion,
    GetDirectory,
    GetFacet,
    GetLinkAttributes,
    GetObjectAttributes,
    GetObjectInformation,
    GetSchemaAsJson,
    GetTypedLinkFacetInformation,
    ListAppliedSchemaArns,
    ListAttachedIndices,
    ListDevelopmentSchemaArns,
    ListDirectories,
    ListFacetAttributes,
    ListFacetNames,
    ListIncomingTypedLinks,
    ListIndex,
    ListManagedSchemaArns,
    ListObjectAttributes,
    ListObjectChildren,
    ListObjectParentPaths,
    ListObjectParents,
    ListObjectPolicies,
    ListOutgoingTypedLinks,
    ListPolicyAttachments,
    ListPublishedSchemaArns,
    ListTagsForResource,
    ListTypedLinkFacetAttributes,
    ListTypedLinkFacetNames,
    LookupPolicy,
    PublishSchema,
    PutSchemaFromJson,
    RemoveFacetFromObject,
    TagResource,
    UntagResource,
    UpdateFacet,
    UpdateLinkAttributes,
    UpdateObjectAttributes,
    UpdateSchema,
    UpdateTypedLinkFacet,
    UpgradeAppliedSchema,
    UpgradePublishedSchema,
}
impl std::fmt::Display for ClouddirectoryActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClouddirectoryActions::AddFacetToObject => write!(f, "clouddirectory:AddFacetToObject"),
            ClouddirectoryActions::ApplySchema => write!(f, "clouddirectory:ApplySchema"),
            ClouddirectoryActions::AttachObject => write!(f, "clouddirectory:AttachObject"),
            ClouddirectoryActions::AttachPolicy => write!(f, "clouddirectory:AttachPolicy"),
            ClouddirectoryActions::AttachToIndex => write!(f, "clouddirectory:AttachToIndex"),
            ClouddirectoryActions::AttachTypedLink => write!(f, "clouddirectory:AttachTypedLink"),
            ClouddirectoryActions::BatchRead => write!(f, "clouddirectory:BatchRead"),
            ClouddirectoryActions::BatchWrite => write!(f, "clouddirectory:BatchWrite"),
            ClouddirectoryActions::CreateDirectory => write!(f, "clouddirectory:CreateDirectory"),
            ClouddirectoryActions::CreateFacet => write!(f, "clouddirectory:CreateFacet"),
            ClouddirectoryActions::CreateIndex => write!(f, "clouddirectory:CreateIndex"),
            ClouddirectoryActions::CreateObject => write!(f, "clouddirectory:CreateObject"),
            ClouddirectoryActions::CreateSchema => write!(f, "clouddirectory:CreateSchema"),
            ClouddirectoryActions::CreateTypedLinkFacet => {
                write!(f, "clouddirectory:CreateTypedLinkFacet")
            }
            ClouddirectoryActions::DeleteDirectory => write!(f, "clouddirectory:DeleteDirectory"),
            ClouddirectoryActions::DeleteFacet => write!(f, "clouddirectory:DeleteFacet"),
            ClouddirectoryActions::DeleteObject => write!(f, "clouddirectory:DeleteObject"),
            ClouddirectoryActions::DeleteSchema => write!(f, "clouddirectory:DeleteSchema"),
            ClouddirectoryActions::DeleteTypedLinkFacet => {
                write!(f, "clouddirectory:DeleteTypedLinkFacet")
            }
            ClouddirectoryActions::DetachFromIndex => write!(f, "clouddirectory:DetachFromIndex"),
            ClouddirectoryActions::DetachObject => write!(f, "clouddirectory:DetachObject"),
            ClouddirectoryActions::DetachPolicy => write!(f, "clouddirectory:DetachPolicy"),
            ClouddirectoryActions::DetachTypedLink => write!(f, "clouddirectory:DetachTypedLink"),
            ClouddirectoryActions::DisableDirectory => write!(f, "clouddirectory:DisableDirectory"),
            ClouddirectoryActions::EnableDirectory => write!(f, "clouddirectory:EnableDirectory"),
            ClouddirectoryActions::GetAppliedSchemaVersion => {
                write!(f, "clouddirectory:GetAppliedSchemaVersion")
            }
            ClouddirectoryActions::GetDirectory => write!(f, "clouddirectory:GetDirectory"),
            ClouddirectoryActions::GetFacet => write!(f, "clouddirectory:GetFacet"),
            ClouddirectoryActions::GetLinkAttributes => {
                write!(f, "clouddirectory:GetLinkAttributes")
            }
            ClouddirectoryActions::GetObjectAttributes => {
                write!(f, "clouddirectory:GetObjectAttributes")
            }
            ClouddirectoryActions::GetObjectInformation => {
                write!(f, "clouddirectory:GetObjectInformation")
            }
            ClouddirectoryActions::GetSchemaAsJson => write!(f, "clouddirectory:GetSchemaAsJson"),
            ClouddirectoryActions::GetTypedLinkFacetInformation => {
                write!(f, "clouddirectory:GetTypedLinkFacetInformation")
            }
            ClouddirectoryActions::ListAppliedSchemaArns => {
                write!(f, "clouddirectory:ListAppliedSchemaArns")
            }
            ClouddirectoryActions::ListAttachedIndices => {
                write!(f, "clouddirectory:ListAttachedIndices")
            }
            ClouddirectoryActions::ListDevelopmentSchemaArns => {
                write!(f, "clouddirectory:ListDevelopmentSchemaArns")
            }
            ClouddirectoryActions::ListDirectories => write!(f, "clouddirectory:ListDirectories"),
            ClouddirectoryActions::ListFacetAttributes => {
                write!(f, "clouddirectory:ListFacetAttributes")
            }
            ClouddirectoryActions::ListFacetNames => write!(f, "clouddirectory:ListFacetNames"),
            ClouddirectoryActions::ListIncomingTypedLinks => {
                write!(f, "clouddirectory:ListIncomingTypedLinks")
            }
            ClouddirectoryActions::ListIndex => write!(f, "clouddirectory:ListIndex"),
            ClouddirectoryActions::ListManagedSchemaArns => {
                write!(f, "clouddirectory:ListManagedSchemaArns")
            }
            ClouddirectoryActions::ListObjectAttributes => {
                write!(f, "clouddirectory:ListObjectAttributes")
            }
            ClouddirectoryActions::ListObjectChildren => {
                write!(f, "clouddirectory:ListObjectChildren")
            }
            ClouddirectoryActions::ListObjectParentPaths => {
                write!(f, "clouddirectory:ListObjectParentPaths")
            }
            ClouddirectoryActions::ListObjectParents => {
                write!(f, "clouddirectory:ListObjectParents")
            }
            ClouddirectoryActions::ListObjectPolicies => {
                write!(f, "clouddirectory:ListObjectPolicies")
            }
            ClouddirectoryActions::ListOutgoingTypedLinks => {
                write!(f, "clouddirectory:ListOutgoingTypedLinks")
            }
            ClouddirectoryActions::ListPolicyAttachments => {
                write!(f, "clouddirectory:ListPolicyAttachments")
            }
            ClouddirectoryActions::ListPublishedSchemaArns => {
                write!(f, "clouddirectory:ListPublishedSchemaArns")
            }
            ClouddirectoryActions::ListTagsForResource => {
                write!(f, "clouddirectory:ListTagsForResource")
            }
            ClouddirectoryActions::ListTypedLinkFacetAttributes => {
                write!(f, "clouddirectory:ListTypedLinkFacetAttributes")
            }
            ClouddirectoryActions::ListTypedLinkFacetNames => {
                write!(f, "clouddirectory:ListTypedLinkFacetNames")
            }
            ClouddirectoryActions::LookupPolicy => write!(f, "clouddirectory:LookupPolicy"),
            ClouddirectoryActions::PublishSchema => write!(f, "clouddirectory:PublishSchema"),
            ClouddirectoryActions::PutSchemaFromJson => {
                write!(f, "clouddirectory:PutSchemaFromJson")
            }
            ClouddirectoryActions::RemoveFacetFromObject => {
                write!(f, "clouddirectory:RemoveFacetFromObject")
            }
            ClouddirectoryActions::TagResource => write!(f, "clouddirectory:TagResource"),
            ClouddirectoryActions::UntagResource => write!(f, "clouddirectory:UntagResource"),
            ClouddirectoryActions::UpdateFacet => write!(f, "clouddirectory:UpdateFacet"),
            ClouddirectoryActions::UpdateLinkAttributes => {
                write!(f, "clouddirectory:UpdateLinkAttributes")
            }
            ClouddirectoryActions::UpdateObjectAttributes => {
                write!(f, "clouddirectory:UpdateObjectAttributes")
            }
            ClouddirectoryActions::UpdateSchema => write!(f, "clouddirectory:UpdateSchema"),
            ClouddirectoryActions::UpdateTypedLinkFacet => {
                write!(f, "clouddirectory:UpdateTypedLinkFacet")
            }
            ClouddirectoryActions::UpgradeAppliedSchema => {
                write!(f, "clouddirectory:UpgradeAppliedSchema")
            }
            ClouddirectoryActions::UpgradePublishedSchema => {
                write!(f, "clouddirectory:UpgradePublishedSchema")
            }
        }
    }
}
