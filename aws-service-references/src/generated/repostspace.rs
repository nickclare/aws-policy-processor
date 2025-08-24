// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum RepostspaceActions {
    BatchAddChannelRoleToAccessors,
    BatchAddRole,
    BatchRemoveChannelRoleFromAccessors,
    BatchRemoveRole,
    CreateChannel,
    CreateSpace,
    DeleteSpace,
    DeregisterAdmin,
    GetChannel,
    GetSpace,
    ListChannels,
    ListSpaces,
    ListTagsForResource,
    RegisterAdmin,
    SendInvites,
    TagResource,
    UntagResource,
    UpdateChannel,
    UpdateSpace,
}
impl std::fmt::Display for RepostspaceActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RepostspaceActions::BatchAddChannelRoleToAccessors => {
                write!(f, "repostspace:BatchAddChannelRoleToAccessors")
            }
            RepostspaceActions::BatchAddRole => write!(f, "repostspace:BatchAddRole"),
            RepostspaceActions::BatchRemoveChannelRoleFromAccessors => {
                write!(f, "repostspace:BatchRemoveChannelRoleFromAccessors")
            }
            RepostspaceActions::BatchRemoveRole => write!(f, "repostspace:BatchRemoveRole"),
            RepostspaceActions::CreateChannel => write!(f, "repostspace:CreateChannel"),
            RepostspaceActions::CreateSpace => write!(f, "repostspace:CreateSpace"),
            RepostspaceActions::DeleteSpace => write!(f, "repostspace:DeleteSpace"),
            RepostspaceActions::DeregisterAdmin => write!(f, "repostspace:DeregisterAdmin"),
            RepostspaceActions::GetChannel => write!(f, "repostspace:GetChannel"),
            RepostspaceActions::GetSpace => write!(f, "repostspace:GetSpace"),
            RepostspaceActions::ListChannels => write!(f, "repostspace:ListChannels"),
            RepostspaceActions::ListSpaces => write!(f, "repostspace:ListSpaces"),
            RepostspaceActions::ListTagsForResource => write!(f, "repostspace:ListTagsForResource"),
            RepostspaceActions::RegisterAdmin => write!(f, "repostspace:RegisterAdmin"),
            RepostspaceActions::SendInvites => write!(f, "repostspace:SendInvites"),
            RepostspaceActions::TagResource => write!(f, "repostspace:TagResource"),
            RepostspaceActions::UntagResource => write!(f, "repostspace:UntagResource"),
            RepostspaceActions::UpdateChannel => write!(f, "repostspace:UpdateChannel"),
            RepostspaceActions::UpdateSpace => write!(f, "repostspace:UpdateSpace"),
        }
    }
}
