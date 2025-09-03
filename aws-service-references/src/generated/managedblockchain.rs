// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum ManagedblockchainActions {
    CreateAccessor,
    CreateMember,
    CreateNetwork,
    CreateNode,
    CreateProposal,
    DeleteAccessor,
    DeleteMember,
    DeleteNode,
    Get,
    GetAccessor,
    GetMember,
    GetNetwork,
    GetNode,
    GetProposal,
    Invoke,
    InvokeRpcBitcoinMainnet,
    InvokeRpcBitcoinTestnet,
    InvokeRpcPolygonMainnet,
    InvokeRpcPolygonMumbaiTestnet,
    ListAccessors,
    ListInvitations,
    ListMembers,
    ListNetworks,
    ListNodes,
    ListProposalVotes,
    ListProposals,
    ListTagsForResource,
    Post,
    RejectInvitation,
    TagResource,
    UntagResource,
    UpdateMember,
    UpdateNode,
    VoteOnProposal,
}
impl std::fmt::Display for ManagedblockchainActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ManagedblockchainActions::CreateAccessor => {
                write!(f, "managedblockchain:CreateAccessor")
            }
            ManagedblockchainActions::CreateMember => write!(f, "managedblockchain:CreateMember"),
            ManagedblockchainActions::CreateNetwork => write!(f, "managedblockchain:CreateNetwork"),
            ManagedblockchainActions::CreateNode => write!(f, "managedblockchain:CreateNode"),
            ManagedblockchainActions::CreateProposal => {
                write!(f, "managedblockchain:CreateProposal")
            }
            ManagedblockchainActions::DeleteAccessor => {
                write!(f, "managedblockchain:DeleteAccessor")
            }
            ManagedblockchainActions::DeleteMember => write!(f, "managedblockchain:DeleteMember"),
            ManagedblockchainActions::DeleteNode => write!(f, "managedblockchain:DeleteNode"),
            ManagedblockchainActions::Get => write!(f, "managedblockchain:GET"),
            ManagedblockchainActions::GetAccessor => write!(f, "managedblockchain:GetAccessor"),
            ManagedblockchainActions::GetMember => write!(f, "managedblockchain:GetMember"),
            ManagedblockchainActions::GetNetwork => write!(f, "managedblockchain:GetNetwork"),
            ManagedblockchainActions::GetNode => write!(f, "managedblockchain:GetNode"),
            ManagedblockchainActions::GetProposal => write!(f, "managedblockchain:GetProposal"),
            ManagedblockchainActions::Invoke => write!(f, "managedblockchain:Invoke"),
            ManagedblockchainActions::InvokeRpcBitcoinMainnet => {
                write!(f, "managedblockchain:InvokeRpcBitcoinMainnet")
            }
            ManagedblockchainActions::InvokeRpcBitcoinTestnet => {
                write!(f, "managedblockchain:InvokeRpcBitcoinTestnet")
            }
            ManagedblockchainActions::InvokeRpcPolygonMainnet => {
                write!(f, "managedblockchain:InvokeRpcPolygonMainnet")
            }
            ManagedblockchainActions::InvokeRpcPolygonMumbaiTestnet => {
                write!(f, "managedblockchain:InvokeRpcPolygonMumbaiTestnet")
            }
            ManagedblockchainActions::ListAccessors => write!(f, "managedblockchain:ListAccessors"),
            ManagedblockchainActions::ListInvitations => {
                write!(f, "managedblockchain:ListInvitations")
            }
            ManagedblockchainActions::ListMembers => write!(f, "managedblockchain:ListMembers"),
            ManagedblockchainActions::ListNetworks => write!(f, "managedblockchain:ListNetworks"),
            ManagedblockchainActions::ListNodes => write!(f, "managedblockchain:ListNodes"),
            ManagedblockchainActions::ListProposalVotes => {
                write!(f, "managedblockchain:ListProposalVotes")
            }
            ManagedblockchainActions::ListProposals => write!(f, "managedblockchain:ListProposals"),
            ManagedblockchainActions::ListTagsForResource => {
                write!(f, "managedblockchain:ListTagsForResource")
            }
            ManagedblockchainActions::Post => write!(f, "managedblockchain:POST"),
            ManagedblockchainActions::RejectInvitation => {
                write!(f, "managedblockchain:RejectInvitation")
            }
            ManagedblockchainActions::TagResource => write!(f, "managedblockchain:TagResource"),
            ManagedblockchainActions::UntagResource => write!(f, "managedblockchain:UntagResource"),
            ManagedblockchainActions::UpdateMember => write!(f, "managedblockchain:UpdateMember"),
            ManagedblockchainActions::UpdateNode => write!(f, "managedblockchain:UpdateNode"),
            ManagedblockchainActions::VoteOnProposal => {
                write!(f, "managedblockchain:VoteOnProposal")
            }
        }
    }
}
