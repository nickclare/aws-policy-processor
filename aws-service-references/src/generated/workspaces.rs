// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum WorkspacesActions {
    AcceptAccountLinkInvitation,
    AssociateConnectionAlias,
    AssociateIpGroups,
    AssociateWorkspaceApplication,
    AuthorizeIpRules,
    CopyWorkspaceImage,
    CreateAccountLinkInvitation,
    CreateConnectClientAddIn,
    CreateConnectionAlias,
    CreateIpGroup,
    CreateRootClientCertificate,
    CreateStandbyWorkspaces,
    CreateTags,
    CreateUpdatedWorkspaceImage,
    CreateWorkspaceBundle,
    CreateWorkspaceImage,
    CreateWorkspaces,
    CreateWorkspacesPool,
    DeleteAccountLinkInvitation,
    DeleteClientBranding,
    DeleteConnectClientAddIn,
    DeleteConnectionAlias,
    DeleteIpGroup,
    DeleteRootClientCertificate,
    DeleteTags,
    DeleteWorkspaceBundle,
    DeleteWorkspaceImage,
    DeployWorkspaceApplications,
    DeregisterWorkspaceDirectory,
    DescribeAccount,
    DescribeAccountModifications,
    DescribeApplicationAssociations,
    DescribeApplications,
    DescribeBundleAssociations,
    DescribeClientBranding,
    DescribeClientProperties,
    DescribeConnectClientAddIns,
    DescribeConnectionAliasPermissions,
    DescribeConnectionAliases,
    DescribeConsent,
    DescribeCustomWorkspaceImageImport,
    DescribeImageAssociations,
    DescribeIpGroups,
    DescribeRootClientCertificates,
    DescribeTags,
    DescribeWorkspaceAssociations,
    DescribeWorkspaceBundles,
    DescribeWorkspaceDirectories,
    DescribeWorkspaceImagePermissions,
    DescribeWorkspaceImages,
    DescribeWorkspaceSnapshots,
    DescribeWorkspaces,
    DescribeWorkspacesConnectionStatus,
    DescribeWorkspacesPoolSessions,
    DescribeWorkspacesPools,
    DirectoryAccessManagement,
    DisassociateConnectionAlias,
    DisassociateIpGroups,
    DisassociateWorkspaceApplication,
    GetAccountLink,
    ImportClientBranding,
    ImportCustomWorkspaceImage,
    ImportWorkspaceImage,
    ListAccountLinks,
    ListAvailableManagementCidrRanges,
    MigrateWorkspace,
    ModifyAccount,
    ModifyCertificateBasedAuthProperties,
    ModifyClientProperties,
    ModifyEndpointEncryptionMode,
    ModifySamlProperties,
    ModifySelfservicePermissions,
    ModifyStreamingProperties,
    ModifyWorkspaceAccessProperties,
    ModifyWorkspaceCreationProperties,
    ModifyWorkspaceProperties,
    ModifyWorkspaceState,
    RebootWorkspaces,
    RebuildWorkspaces,
    RegisterWorkspaceDirectory,
    RejectAccountLinkInvitation,
    RestoreWorkspace,
    RevokeIpRules,
    StartWorkspaces,
    StartWorkspacesPool,
    StopWorkspaces,
    StopWorkspacesPool,
    Stream,
    TerminateWorkspaces,
    TerminateWorkspacesPool,
    TerminateWorkspacesPoolSession,
    UpdateConnectClientAddIn,
    UpdateConnectionAliasPermission,
    UpdateConsent,
    UpdateRootClientCertificate,
    UpdateRulesOfIpGroup,
    UpdateWorkspaceBundle,
    UpdateWorkspaceImagePermission,
    UpdateWorkspacesPool,
}
impl std::fmt::Display for WorkspacesActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WorkspacesActions::AcceptAccountLinkInvitation => {
                write!(f, "workspaces:AcceptAccountLinkInvitation")
            }
            WorkspacesActions::AssociateConnectionAlias => {
                write!(f, "workspaces:AssociateConnectionAlias")
            }
            WorkspacesActions::AssociateIpGroups => write!(f, "workspaces:AssociateIpGroups"),
            WorkspacesActions::AssociateWorkspaceApplication => {
                write!(f, "workspaces:AssociateWorkspaceApplication")
            }
            WorkspacesActions::AuthorizeIpRules => write!(f, "workspaces:AuthorizeIpRules"),
            WorkspacesActions::CopyWorkspaceImage => write!(f, "workspaces:CopyWorkspaceImage"),
            WorkspacesActions::CreateAccountLinkInvitation => {
                write!(f, "workspaces:CreateAccountLinkInvitation")
            }
            WorkspacesActions::CreateConnectClientAddIn => {
                write!(f, "workspaces:CreateConnectClientAddIn")
            }
            WorkspacesActions::CreateConnectionAlias => {
                write!(f, "workspaces:CreateConnectionAlias")
            }
            WorkspacesActions::CreateIpGroup => write!(f, "workspaces:CreateIpGroup"),
            WorkspacesActions::CreateRootClientCertificate => {
                write!(f, "workspaces:CreateRootClientCertificate")
            }
            WorkspacesActions::CreateStandbyWorkspaces => {
                write!(f, "workspaces:CreateStandbyWorkspaces")
            }
            WorkspacesActions::CreateTags => write!(f, "workspaces:CreateTags"),
            WorkspacesActions::CreateUpdatedWorkspaceImage => {
                write!(f, "workspaces:CreateUpdatedWorkspaceImage")
            }
            WorkspacesActions::CreateWorkspaceBundle => {
                write!(f, "workspaces:CreateWorkspaceBundle")
            }
            WorkspacesActions::CreateWorkspaceImage => write!(f, "workspaces:CreateWorkspaceImage"),
            WorkspacesActions::CreateWorkspaces => write!(f, "workspaces:CreateWorkspaces"),
            WorkspacesActions::CreateWorkspacesPool => write!(f, "workspaces:CreateWorkspacesPool"),
            WorkspacesActions::DeleteAccountLinkInvitation => {
                write!(f, "workspaces:DeleteAccountLinkInvitation")
            }
            WorkspacesActions::DeleteClientBranding => write!(f, "workspaces:DeleteClientBranding"),
            WorkspacesActions::DeleteConnectClientAddIn => {
                write!(f, "workspaces:DeleteConnectClientAddIn")
            }
            WorkspacesActions::DeleteConnectionAlias => {
                write!(f, "workspaces:DeleteConnectionAlias")
            }
            WorkspacesActions::DeleteIpGroup => write!(f, "workspaces:DeleteIpGroup"),
            WorkspacesActions::DeleteRootClientCertificate => {
                write!(f, "workspaces:DeleteRootClientCertificate")
            }
            WorkspacesActions::DeleteTags => write!(f, "workspaces:DeleteTags"),
            WorkspacesActions::DeleteWorkspaceBundle => {
                write!(f, "workspaces:DeleteWorkspaceBundle")
            }
            WorkspacesActions::DeleteWorkspaceImage => write!(f, "workspaces:DeleteWorkspaceImage"),
            WorkspacesActions::DeployWorkspaceApplications => {
                write!(f, "workspaces:DeployWorkspaceApplications")
            }
            WorkspacesActions::DeregisterWorkspaceDirectory => {
                write!(f, "workspaces:DeregisterWorkspaceDirectory")
            }
            WorkspacesActions::DescribeAccount => write!(f, "workspaces:DescribeAccount"),
            WorkspacesActions::DescribeAccountModifications => {
                write!(f, "workspaces:DescribeAccountModifications")
            }
            WorkspacesActions::DescribeApplicationAssociations => {
                write!(f, "workspaces:DescribeApplicationAssociations")
            }
            WorkspacesActions::DescribeApplications => write!(f, "workspaces:DescribeApplications"),
            WorkspacesActions::DescribeBundleAssociations => {
                write!(f, "workspaces:DescribeBundleAssociations")
            }
            WorkspacesActions::DescribeClientBranding => {
                write!(f, "workspaces:DescribeClientBranding")
            }
            WorkspacesActions::DescribeClientProperties => {
                write!(f, "workspaces:DescribeClientProperties")
            }
            WorkspacesActions::DescribeConnectClientAddIns => {
                write!(f, "workspaces:DescribeConnectClientAddIns")
            }
            WorkspacesActions::DescribeConnectionAliasPermissions => {
                write!(f, "workspaces:DescribeConnectionAliasPermissions")
            }
            WorkspacesActions::DescribeConnectionAliases => {
                write!(f, "workspaces:DescribeConnectionAliases")
            }
            WorkspacesActions::DescribeConsent => write!(f, "workspaces:DescribeConsent"),
            WorkspacesActions::DescribeCustomWorkspaceImageImport => {
                write!(f, "workspaces:DescribeCustomWorkspaceImageImport")
            }
            WorkspacesActions::DescribeImageAssociations => {
                write!(f, "workspaces:DescribeImageAssociations")
            }
            WorkspacesActions::DescribeIpGroups => write!(f, "workspaces:DescribeIpGroups"),
            WorkspacesActions::DescribeRootClientCertificates => {
                write!(f, "workspaces:DescribeRootClientCertificates")
            }
            WorkspacesActions::DescribeTags => write!(f, "workspaces:DescribeTags"),
            WorkspacesActions::DescribeWorkspaceAssociations => {
                write!(f, "workspaces:DescribeWorkspaceAssociations")
            }
            WorkspacesActions::DescribeWorkspaceBundles => {
                write!(f, "workspaces:DescribeWorkspaceBundles")
            }
            WorkspacesActions::DescribeWorkspaceDirectories => {
                write!(f, "workspaces:DescribeWorkspaceDirectories")
            }
            WorkspacesActions::DescribeWorkspaceImagePermissions => {
                write!(f, "workspaces:DescribeWorkspaceImagePermissions")
            }
            WorkspacesActions::DescribeWorkspaceImages => {
                write!(f, "workspaces:DescribeWorkspaceImages")
            }
            WorkspacesActions::DescribeWorkspaceSnapshots => {
                write!(f, "workspaces:DescribeWorkspaceSnapshots")
            }
            WorkspacesActions::DescribeWorkspaces => write!(f, "workspaces:DescribeWorkspaces"),
            WorkspacesActions::DescribeWorkspacesConnectionStatus => {
                write!(f, "workspaces:DescribeWorkspacesConnectionStatus")
            }
            WorkspacesActions::DescribeWorkspacesPoolSessions => {
                write!(f, "workspaces:DescribeWorkspacesPoolSessions")
            }
            WorkspacesActions::DescribeWorkspacesPools => {
                write!(f, "workspaces:DescribeWorkspacesPools")
            }
            WorkspacesActions::DirectoryAccessManagement => {
                write!(f, "workspaces:DirectoryAccessManagement")
            }
            WorkspacesActions::DisassociateConnectionAlias => {
                write!(f, "workspaces:DisassociateConnectionAlias")
            }
            WorkspacesActions::DisassociateIpGroups => write!(f, "workspaces:DisassociateIpGroups"),
            WorkspacesActions::DisassociateWorkspaceApplication => {
                write!(f, "workspaces:DisassociateWorkspaceApplication")
            }
            WorkspacesActions::GetAccountLink => write!(f, "workspaces:GetAccountLink"),
            WorkspacesActions::ImportClientBranding => write!(f, "workspaces:ImportClientBranding"),
            WorkspacesActions::ImportCustomWorkspaceImage => {
                write!(f, "workspaces:ImportCustomWorkspaceImage")
            }
            WorkspacesActions::ImportWorkspaceImage => write!(f, "workspaces:ImportWorkspaceImage"),
            WorkspacesActions::ListAccountLinks => write!(f, "workspaces:ListAccountLinks"),
            WorkspacesActions::ListAvailableManagementCidrRanges => {
                write!(f, "workspaces:ListAvailableManagementCidrRanges")
            }
            WorkspacesActions::MigrateWorkspace => write!(f, "workspaces:MigrateWorkspace"),
            WorkspacesActions::ModifyAccount => write!(f, "workspaces:ModifyAccount"),
            WorkspacesActions::ModifyCertificateBasedAuthProperties => {
                write!(f, "workspaces:ModifyCertificateBasedAuthProperties")
            }
            WorkspacesActions::ModifyClientProperties => {
                write!(f, "workspaces:ModifyClientProperties")
            }
            WorkspacesActions::ModifyEndpointEncryptionMode => {
                write!(f, "workspaces:ModifyEndpointEncryptionMode")
            }
            WorkspacesActions::ModifySamlProperties => write!(f, "workspaces:ModifySamlProperties"),
            WorkspacesActions::ModifySelfservicePermissions => {
                write!(f, "workspaces:ModifySelfservicePermissions")
            }
            WorkspacesActions::ModifyStreamingProperties => {
                write!(f, "workspaces:ModifyStreamingProperties")
            }
            WorkspacesActions::ModifyWorkspaceAccessProperties => {
                write!(f, "workspaces:ModifyWorkspaceAccessProperties")
            }
            WorkspacesActions::ModifyWorkspaceCreationProperties => {
                write!(f, "workspaces:ModifyWorkspaceCreationProperties")
            }
            WorkspacesActions::ModifyWorkspaceProperties => {
                write!(f, "workspaces:ModifyWorkspaceProperties")
            }
            WorkspacesActions::ModifyWorkspaceState => write!(f, "workspaces:ModifyWorkspaceState"),
            WorkspacesActions::RebootWorkspaces => write!(f, "workspaces:RebootWorkspaces"),
            WorkspacesActions::RebuildWorkspaces => write!(f, "workspaces:RebuildWorkspaces"),
            WorkspacesActions::RegisterWorkspaceDirectory => {
                write!(f, "workspaces:RegisterWorkspaceDirectory")
            }
            WorkspacesActions::RejectAccountLinkInvitation => {
                write!(f, "workspaces:RejectAccountLinkInvitation")
            }
            WorkspacesActions::RestoreWorkspace => write!(f, "workspaces:RestoreWorkspace"),
            WorkspacesActions::RevokeIpRules => write!(f, "workspaces:RevokeIpRules"),
            WorkspacesActions::StartWorkspaces => write!(f, "workspaces:StartWorkspaces"),
            WorkspacesActions::StartWorkspacesPool => write!(f, "workspaces:StartWorkspacesPool"),
            WorkspacesActions::StopWorkspaces => write!(f, "workspaces:StopWorkspaces"),
            WorkspacesActions::StopWorkspacesPool => write!(f, "workspaces:StopWorkspacesPool"),
            WorkspacesActions::Stream => write!(f, "workspaces:Stream"),
            WorkspacesActions::TerminateWorkspaces => write!(f, "workspaces:TerminateWorkspaces"),
            WorkspacesActions::TerminateWorkspacesPool => {
                write!(f, "workspaces:TerminateWorkspacesPool")
            }
            WorkspacesActions::TerminateWorkspacesPoolSession => {
                write!(f, "workspaces:TerminateWorkspacesPoolSession")
            }
            WorkspacesActions::UpdateConnectClientAddIn => {
                write!(f, "workspaces:UpdateConnectClientAddIn")
            }
            WorkspacesActions::UpdateConnectionAliasPermission => {
                write!(f, "workspaces:UpdateConnectionAliasPermission")
            }
            WorkspacesActions::UpdateConsent => write!(f, "workspaces:UpdateConsent"),
            WorkspacesActions::UpdateRootClientCertificate => {
                write!(f, "workspaces:UpdateRootClientCertificate")
            }
            WorkspacesActions::UpdateRulesOfIpGroup => write!(f, "workspaces:UpdateRulesOfIpGroup"),
            WorkspacesActions::UpdateWorkspaceBundle => {
                write!(f, "workspaces:UpdateWorkspaceBundle")
            }
            WorkspacesActions::UpdateWorkspaceImagePermission => {
                write!(f, "workspaces:UpdateWorkspaceImagePermission")
            }
            WorkspacesActions::UpdateWorkspacesPool => write!(f, "workspaces:UpdateWorkspacesPool"),
        }
    }
}
