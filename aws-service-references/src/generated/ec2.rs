// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum Ec2Actions {
    AcceptAddressTransfer,
    AcceptCapacityReservationBillingOwnership,
    AcceptReservedInstancesExchangeQuote,
    AcceptTransitGatewayMulticastDomainAssociations,
    AcceptTransitGatewayPeeringAttachment,
    AcceptTransitGatewayVpcAttachment,
    AcceptVpcEndpointConnections,
    AcceptVpcPeeringConnection,
    AdvertiseByoipCidr,
    AllocateAddress,
    AllocateHosts,
    AllocateIpamPoolCidr,
    ApplySecurityGroupsToClientVpnTargetNetwork,
    AssignIpv6Addresses,
    AssignPrivateIpAddresses,
    AssignPrivateNatGatewayAddress,
    AssociateAddress,
    AssociateCapacityReservationBillingOwner,
    AssociateClientVpnTargetNetwork,
    AssociateDhcpOptions,
    AssociateEnclaveCertificateIamRole,
    AssociateIamInstanceProfile,
    AssociateInstanceEventWindow,
    AssociateIpamByoasn,
    AssociateIpamResourceDiscovery,
    AssociateNatGatewayAddress,
    AssociateRouteServer,
    AssociateRouteTable,
    AssociateSecurityGroupVpc,
    AssociateSubnetCidrBlock,
    AssociateTransitGatewayMulticastDomain,
    AssociateTransitGatewayPolicyTable,
    AssociateTransitGatewayRouteTable,
    AssociateTrunkInterface,
    AssociateVerifiedAccessInstanceWebAcl,
    AssociateVpcCidrBlock,
    AttachClassicLinkVpc,
    AttachInternetGateway,
    AttachNetworkInterface,
    AttachVerifiedAccessTrustProvider,
    AttachVolume,
    AttachVpnGateway,
    AuthorizeClientVpnIngress,
    AuthorizeSecurityGroupEgress,
    AuthorizeSecurityGroupIngress,
    BundleInstance,
    CancelBundleTask,
    CancelCapacityReservation,
    CancelCapacityReservationFleets,
    CancelConversionTask,
    CancelDeclarativePoliciesReport,
    CancelExportTask,
    CancelImageLaunchPermission,
    CancelImportTask,
    CancelReservedInstancesListing,
    CancelSpotFleetRequests,
    CancelSpotInstanceRequests,
    ConfirmProductInstance,
    CopyFpgaImage,
    CopyImage,
    CopySnapshot,
    CreateCapacityReservation,
    CreateCapacityReservationBySplitting,
    CreateCapacityReservationFleet,
    CreateCarrierGateway,
    CreateClientVpnEndpoint,
    CreateClientVpnRoute,
    CreateCoipCidr,
    CreateCoipPool,
    CreateCoipPoolPermission,
    CreateCustomerGateway,
    CreateDefaultSubnet,
    CreateDefaultVpc,
    CreateDelegateMacVolumeOwnershipTask,
    CreateDhcpOptions,
    CreateEgressOnlyInternetGateway,
    CreateFleet,
    CreateFlowLogs,
    CreateFpgaImage,
    CreateImage,
    CreateInstanceConnectEndpoint,
    CreateInstanceEventWindow,
    CreateInstanceExportTask,
    CreateInternetGateway,
    CreateIpam,
    CreateIpamExternalResourceVerificationToken,
    CreateIpamPool,
    CreateIpamResourceDiscovery,
    CreateIpamScope,
    CreateKeyPair,
    CreateLaunchTemplate,
    CreateLaunchTemplateVersion,
    CreateLocalGatewayRoute,
    CreateLocalGatewayRouteTable,
    CreateLocalGatewayRouteTablePermission,
    CreateLocalGatewayRouteTableVirtualInterfaceGroupAssociation,
    CreateLocalGatewayRouteTableVpcAssociation,
    CreateLocalGatewayVirtualInterface,
    CreateLocalGatewayVirtualInterfaceGroup,
    CreateMacSystemIntegrityProtectionModificationTask,
    CreateManagedPrefixList,
    CreateNatGateway,
    CreateNetworkAcl,
    CreateNetworkAclEntry,
    CreateNetworkInsightsAccessScope,
    CreateNetworkInsightsPath,
    CreateNetworkInterface,
    CreateNetworkInterfacePermission,
    CreatePlacementGroup,
    CreatePublicIpv4Pool,
    CreateReplaceRootVolumeTask,
    CreateReservedInstancesListing,
    CreateRestoreImageTask,
    CreateRoute,
    CreateRouteServer,
    CreateRouteServerEndpoint,
    CreateRouteServerPeer,
    CreateRouteTable,
    CreateSecurityGroup,
    CreateSnapshot,
    CreateSnapshots,
    CreateSpotDatafeedSubscription,
    CreateStoreImageTask,
    CreateSubnet,
    CreateSubnetCidrReservation,
    CreateTags,
    CreateTrafficMirrorFilter,
    CreateTrafficMirrorFilterRule,
    CreateTrafficMirrorSession,
    CreateTrafficMirrorTarget,
    CreateTransitGateway,
    CreateTransitGatewayConnect,
    CreateTransitGatewayConnectPeer,
    CreateTransitGatewayMulticastDomain,
    CreateTransitGatewayPeeringAttachment,
    CreateTransitGatewayPolicyTable,
    CreateTransitGatewayPrefixListReference,
    CreateTransitGatewayRoute,
    CreateTransitGatewayRouteTable,
    CreateTransitGatewayRouteTableAnnouncement,
    CreateTransitGatewayVpcAttachment,
    CreateVerifiedAccessEndpoint,
    CreateVerifiedAccessGroup,
    CreateVerifiedAccessInstance,
    CreateVerifiedAccessTrustProvider,
    CreateVolume,
    CreateVpc,
    CreateVpcBlockPublicAccessExclusion,
    CreateVpcEndpoint,
    CreateVpcEndpointConnectionNotification,
    CreateVpcEndpointServiceConfiguration,
    CreateVpcPeeringConnection,
    CreateVpnConnection,
    CreateVpnConnectionRoute,
    CreateVpnGateway,
    DeleteCarrierGateway,
    DeleteClientVpnEndpoint,
    DeleteClientVpnRoute,
    DeleteCoipCidr,
    DeleteCoipPool,
    DeleteCoipPoolPermission,
    DeleteCustomerGateway,
    DeleteDhcpOptions,
    DeleteEgressOnlyInternetGateway,
    DeleteFleets,
    DeleteFlowLogs,
    DeleteFpgaImage,
    DeleteInstanceConnectEndpoint,
    DeleteInstanceEventWindow,
    DeleteInternetGateway,
    DeleteIpam,
    DeleteIpamExternalResourceVerificationToken,
    DeleteIpamPool,
    DeleteIpamResourceDiscovery,
    DeleteIpamScope,
    DeleteKeyPair,
    DeleteLaunchTemplate,
    DeleteLaunchTemplateVersions,
    DeleteLocalGatewayRoute,
    DeleteLocalGatewayRouteTable,
    DeleteLocalGatewayRouteTablePermission,
    DeleteLocalGatewayRouteTableVirtualInterfaceGroupAssociation,
    DeleteLocalGatewayRouteTableVpcAssociation,
    DeleteLocalGatewayVirtualInterface,
    DeleteLocalGatewayVirtualInterfaceGroup,
    DeleteManagedPrefixList,
    DeleteNatGateway,
    DeleteNetworkAcl,
    DeleteNetworkAclEntry,
    DeleteNetworkInsightsAccessScope,
    DeleteNetworkInsightsAccessScopeAnalysis,
    DeleteNetworkInsightsAnalysis,
    DeleteNetworkInsightsPath,
    DeleteNetworkInterface,
    DeleteNetworkInterfacePermission,
    DeletePlacementGroup,
    DeletePublicIpv4Pool,
    DeleteQueuedReservedInstances,
    DeleteResourcePolicy,
    DeleteRoute,
    DeleteRouteServer,
    DeleteRouteServerEndpoint,
    DeleteRouteServerPeer,
    DeleteRouteTable,
    DeleteSecurityGroup,
    DeleteSnapshot,
    DeleteSpotDatafeedSubscription,
    DeleteSubnet,
    DeleteSubnetCidrReservation,
    DeleteTags,
    DeleteTrafficMirrorFilter,
    DeleteTrafficMirrorFilterRule,
    DeleteTrafficMirrorSession,
    DeleteTrafficMirrorTarget,
    DeleteTransitGateway,
    DeleteTransitGatewayConnect,
    DeleteTransitGatewayConnectPeer,
    DeleteTransitGatewayMulticastDomain,
    DeleteTransitGatewayPeeringAttachment,
    DeleteTransitGatewayPolicyTable,
    DeleteTransitGatewayPrefixListReference,
    DeleteTransitGatewayRoute,
    DeleteTransitGatewayRouteTable,
    DeleteTransitGatewayRouteTableAnnouncement,
    DeleteTransitGatewayVpcAttachment,
    DeleteVerifiedAccessEndpoint,
    DeleteVerifiedAccessGroup,
    DeleteVerifiedAccessInstance,
    DeleteVerifiedAccessTrustProvider,
    DeleteVolume,
    DeleteVpc,
    DeleteVpcBlockPublicAccessExclusion,
    DeleteVpcEndpointConnectionNotifications,
    DeleteVpcEndpointServiceConfigurations,
    DeleteVpcEndpoints,
    DeleteVpcPeeringConnection,
    DeleteVpnConnection,
    DeleteVpnConnectionRoute,
    DeleteVpnGateway,
    DeprovisionByoipCidr,
    DeprovisionIpamByoasn,
    DeprovisionIpamPoolCidr,
    DeprovisionPublicIpv4PoolCidr,
    DeregisterImage,
    DeregisterInstanceEventNotificationAttributes,
    DeregisterTransitGatewayMulticastGroupMembers,
    DeregisterTransitGatewayMulticastGroupSources,
    DescribeAccountAttributes,
    DescribeAddressTransfers,
    DescribeAddresses,
    DescribeAddressesAttribute,
    DescribeAggregateIdFormat,
    DescribeAvailabilityZones,
    DescribeAwsNetworkPerformanceMetricSubscriptions,
    DescribeBundleTasks,
    DescribeByoipCidrs,
    DescribeCapacityBlockExtensionHistory,
    DescribeCapacityBlockExtensionOfferings,
    DescribeCapacityBlockOfferings,
    DescribeCapacityBlockStatus,
    DescribeCapacityBlocks,
    DescribeCapacityReservationBillingRequests,
    DescribeCapacityReservationFleets,
    DescribeCapacityReservations,
    DescribeCarrierGateways,
    DescribeClassicLinkInstances,
    DescribeClientVpnAuthorizationRules,
    DescribeClientVpnConnections,
    DescribeClientVpnEndpoints,
    DescribeClientVpnRoutes,
    DescribeClientVpnTargetNetworks,
    DescribeCoipPools,
    DescribeConversionTasks,
    DescribeCustomerGateways,
    DescribeDeclarativePoliciesReports,
    DescribeDhcpOptions,
    DescribeEgressOnlyInternetGateways,
    DescribeElasticGpus,
    DescribeExportImageTasks,
    DescribeExportTasks,
    DescribeFastLaunchImages,
    DescribeFastSnapshotRestores,
    DescribeFleetHistory,
    DescribeFleetInstances,
    DescribeFleets,
    DescribeFlowLogs,
    DescribeFpgaImageAttribute,
    DescribeFpgaImages,
    DescribeHostReservationOfferings,
    DescribeHostReservations,
    DescribeHosts,
    DescribeIamInstanceProfileAssociations,
    DescribeIdFormat,
    DescribeIdentityIdFormat,
    DescribeImageAttribute,
    DescribeImages,
    DescribeImportImageTasks,
    DescribeImportSnapshotTasks,
    DescribeInstanceAttribute,
    DescribeInstanceConnectEndpoints,
    DescribeInstanceCreditSpecifications,
    DescribeInstanceEventNotificationAttributes,
    DescribeInstanceEventWindows,
    DescribeInstanceImageMetadata,
    DescribeInstanceStatus,
    DescribeInstanceTopology,
    DescribeInstanceTypeOfferings,
    DescribeInstanceTypes,
    DescribeInstances,
    DescribeInternetGateways,
    DescribeIpamByoasn,
    DescribeIpamExternalResourceVerificationTokens,
    DescribeIpamPools,
    DescribeIpamResourceDiscoveries,
    DescribeIpamResourceDiscoveryAssociations,
    DescribeIpamScopes,
    DescribeIpams,
    DescribeIpv6Pools,
    DescribeKeyPairs,
    DescribeLaunchTemplateVersions,
    DescribeLaunchTemplates,
    DescribeLocalGatewayRouteTablePermissions,
    DescribeLocalGatewayRouteTableVirtualInterfaceGroupAssociations,
    DescribeLocalGatewayRouteTableVpcAssociations,
    DescribeLocalGatewayRouteTables,
    DescribeLocalGatewayVirtualInterfaceGroups,
    DescribeLocalGatewayVirtualInterfaces,
    DescribeLocalGateways,
    DescribeLockedSnapshots,
    DescribeMacHosts,
    DescribeMacModificationTasks,
    DescribeManagedPrefixLists,
    DescribeMovingAddresses,
    DescribeNatGateways,
    DescribeNetworkAcls,
    DescribeNetworkInsightsAccessScopeAnalyses,
    DescribeNetworkInsightsAccessScopes,
    DescribeNetworkInsightsAnalyses,
    DescribeNetworkInsightsPaths,
    DescribeNetworkInterfaceAttribute,
    DescribeNetworkInterfacePermissions,
    DescribeNetworkInterfaces,
    DescribeOutpostLags,
    DescribePlacementGroups,
    DescribePrefixLists,
    DescribePrincipalIdFormat,
    DescribePublicIpv4Pools,
    DescribeRegions,
    DescribeReplaceRootVolumeTasks,
    DescribeReservedInstances,
    DescribeReservedInstancesListings,
    DescribeReservedInstancesModifications,
    DescribeReservedInstancesOfferings,
    DescribeRouteServerEndpoints,
    DescribeRouteServerPeers,
    DescribeRouteServers,
    DescribeRouteTables,
    DescribeScheduledInstanceAvailability,
    DescribeScheduledInstances,
    DescribeSecurityGroupReferences,
    DescribeSecurityGroupRules,
    DescribeSecurityGroupVpcAssociations,
    DescribeSecurityGroups,
    DescribeServiceLinkVirtualInterfaces,
    DescribeSnapshotAttribute,
    DescribeSnapshotTierStatus,
    DescribeSnapshots,
    DescribeSpotDatafeedSubscription,
    DescribeSpotFleetInstances,
    DescribeSpotFleetRequestHistory,
    DescribeSpotFleetRequests,
    DescribeSpotInstanceRequests,
    DescribeSpotPriceHistory,
    DescribeStaleSecurityGroups,
    DescribeStoreImageTasks,
    DescribeSubnets,
    DescribeTags,
    DescribeTrafficMirrorFilterRules,
    DescribeTrafficMirrorFilters,
    DescribeTrafficMirrorSessions,
    DescribeTrafficMirrorTargets,
    DescribeTransitGatewayAttachments,
    DescribeTransitGatewayConnectPeers,
    DescribeTransitGatewayConnects,
    DescribeTransitGatewayMulticastDomains,
    DescribeTransitGatewayPeeringAttachments,
    DescribeTransitGatewayPolicyTables,
    DescribeTransitGatewayRouteTableAnnouncements,
    DescribeTransitGatewayRouteTables,
    DescribeTransitGatewayVpcAttachments,
    DescribeTransitGateways,
    DescribeTrunkInterfaceAssociations,
    DescribeVerifiedAccessEndpoints,
    DescribeVerifiedAccessGroups,
    DescribeVerifiedAccessInstanceLoggingConfigurations,
    DescribeVerifiedAccessInstanceWebAclAssociations,
    DescribeVerifiedAccessInstances,
    DescribeVerifiedAccessTrustProviders,
    DescribeVolumeAttribute,
    DescribeVolumeStatus,
    DescribeVolumes,
    DescribeVolumesModifications,
    DescribeVpcAttribute,
    DescribeVpcBlockPublicAccessExclusions,
    DescribeVpcBlockPublicAccessOptions,
    DescribeVpcClassicLink,
    DescribeVpcClassicLinkDnsSupport,
    DescribeVpcEndpointAssociations,
    DescribeVpcEndpointConnectionNotifications,
    DescribeVpcEndpointConnections,
    DescribeVpcEndpointServiceConfigurations,
    DescribeVpcEndpointServicePermissions,
    DescribeVpcEndpointServices,
    DescribeVpcEndpoints,
    DescribeVpcPeeringConnections,
    DescribeVpcs,
    DescribeVpnConnections,
    DescribeVpnGateways,
    DetachClassicLinkVpc,
    DetachInternetGateway,
    DetachNetworkInterface,
    DetachVerifiedAccessTrustProvider,
    DetachVolume,
    DetachVpnGateway,
    DisableAddressTransfer,
    DisableAllowedImagesSettings,
    DisableAwsNetworkPerformanceMetricSubscription,
    DisableEbsEncryptionByDefault,
    DisableFastLaunch,
    DisableFastSnapshotRestores,
    DisableImage,
    DisableImageBlockPublicAccess,
    DisableImageDeprecation,
    DisableImageDeregistrationProtection,
    DisableIpamOrganizationAdminAccount,
    DisableRouteServerPropagation,
    DisableSerialConsoleAccess,
    DisableSnapshotBlockPublicAccess,
    DisableTransitGatewayRouteTablePropagation,
    DisableVgwRoutePropagation,
    DisableVpcClassicLink,
    DisableVpcClassicLinkDnsSupport,
    DisassociateAddress,
    DisassociateCapacityReservationBillingOwner,
    DisassociateClientVpnTargetNetwork,
    DisassociateEnclaveCertificateIamRole,
    DisassociateIamInstanceProfile,
    DisassociateInstanceEventWindow,
    DisassociateIpamByoasn,
    DisassociateIpamResourceDiscovery,
    DisassociateNatGatewayAddress,
    DisassociateRouteServer,
    DisassociateRouteTable,
    DisassociateSecurityGroupVpc,
    DisassociateSubnetCidrBlock,
    DisassociateTransitGatewayMulticastDomain,
    DisassociateTransitGatewayPolicyTable,
    DisassociateTransitGatewayRouteTable,
    DisassociateTrunkInterface,
    DisassociateVerifiedAccessInstanceWebAcl,
    DisassociateVpcCidrBlock,
    EnableAddressTransfer,
    EnableAllowedImagesSettings,
    EnableAwsNetworkPerformanceMetricSubscription,
    EnableEbsEncryptionByDefault,
    EnableFastLaunch,
    EnableFastSnapshotRestores,
    EnableImage,
    EnableImageBlockPublicAccess,
    EnableImageDeprecation,
    EnableImageDeregistrationProtection,
    EnableIpamOrganizationAdminAccount,
    EnableReachabilityAnalyzerOrganizationSharing,
    EnableRouteServerPropagation,
    EnableSerialConsoleAccess,
    EnableSnapshotBlockPublicAccess,
    EnableTransitGatewayRouteTablePropagation,
    EnableVgwRoutePropagation,
    EnableVolumeIo,
    EnableVpcClassicLink,
    EnableVpcClassicLinkDnsSupport,
    ExportClientVpnClientCertificateRevocationList,
    ExportClientVpnClientConfiguration,
    ExportImage,
    ExportTransitGatewayRoutes,
    ExportVerifiedAccessInstanceClientConfiguration,
    GetActiveVpnTunnelStatus,
    GetAllowedImagesSettings,
    GetAssociatedEnclaveCertificateIamRoles,
    GetAssociatedIpv6PoolCidrs,
    GetAwsNetworkPerformanceData,
    GetCapacityReservationUsage,
    GetCoipPoolUsage,
    GetConsoleOutput,
    GetConsoleScreenshot,
    GetDeclarativePoliciesReportSummary,
    GetDefaultCreditSpecification,
    GetEbsDefaultKmsKeyId,
    GetEbsEncryptionByDefault,
    GetFlowLogsIntegrationTemplate,
    GetGroupsForCapacityReservation,
    GetHostReservationPurchasePreview,
    GetImageBlockPublicAccessState,
    GetInstanceMetadataDefaults,
    GetInstanceTpmEkPub,
    GetInstanceTypesFromInstanceRequirements,
    GetInstanceUefiData,
    GetIpamAddressHistory,
    GetIpamDiscoveredAccounts,
    GetIpamDiscoveredPublicAddresses,
    GetIpamDiscoveredResourceCidrs,
    GetIpamPoolAllocations,
    GetIpamPoolCidrs,
    GetIpamResourceCidrs,
    GetLaunchTemplateData,
    GetManagedPrefixListAssociations,
    GetManagedPrefixListEntries,
    GetNetworkInsightsAccessScopeAnalysisFindings,
    GetNetworkInsightsAccessScopeContent,
    GetPasswordData,
    GetReservedInstancesExchangeQuote,
    GetResourcePolicy,
    GetRouteServerAssociations,
    GetRouteServerPropagations,
    GetRouteServerRoutingDatabase,
    GetSecurityGroupsForVpc,
    GetSerialConsoleAccessStatus,
    GetSnapshotBlockPublicAccessState,
    GetSpotPlacementScores,
    GetSubnetCidrReservations,
    GetTransitGatewayAttachmentPropagations,
    GetTransitGatewayMulticastDomainAssociations,
    GetTransitGatewayPolicyTableAssociations,
    GetTransitGatewayPolicyTableEntries,
    GetTransitGatewayPrefixListReferences,
    GetTransitGatewayRouteTableAssociations,
    GetTransitGatewayRouteTablePropagations,
    GetVerifiedAccessEndpointPolicy,
    GetVerifiedAccessEndpointTargets,
    GetVerifiedAccessGroupPolicy,
    GetVerifiedAccessInstanceWebAcl,
    GetVpnConnectionDeviceSampleConfiguration,
    GetVpnConnectionDeviceTypes,
    GetVpnTunnelReplacementStatus,
    ImportByoipCidrToIpam,
    ImportClientVpnClientCertificateRevocationList,
    ImportImage,
    ImportInstance,
    ImportKeyPair,
    ImportSnapshot,
    ImportVolume,
    InjectApiError,
    ListImagesInRecycleBin,
    ListSnapshotsInRecycleBin,
    LockSnapshot,
    ModifyAddressAttribute,
    ModifyAvailabilityZoneGroup,
    ModifyCapacityReservation,
    ModifyCapacityReservationFleet,
    ModifyClientVpnEndpoint,
    ModifyDefaultCreditSpecification,
    ModifyEbsDefaultKmsKeyId,
    ModifyFleet,
    ModifyFpgaImageAttribute,
    ModifyHosts,
    ModifyIdFormat,
    ModifyIdentityIdFormat,
    ModifyImageAttribute,
    ModifyInstanceAttribute,
    ModifyInstanceCapacityReservationAttributes,
    ModifyInstanceCpuOptions,
    ModifyInstanceCreditSpecification,
    ModifyInstanceEventStartTime,
    ModifyInstanceEventWindow,
    ModifyInstanceMaintenanceOptions,
    ModifyInstanceMetadataDefaults,
    ModifyInstanceMetadataOptions,
    ModifyInstanceNetworkPerformanceOptions,
    ModifyInstancePlacement,
    ModifyIpam,
    ModifyIpamPool,
    ModifyIpamResourceCidr,
    ModifyIpamResourceDiscovery,
    ModifyIpamScope,
    ModifyLaunchTemplate,
    ModifyLocalGatewayRoute,
    ModifyManagedPrefixList,
    ModifyNetworkInterfaceAttribute,
    ModifyPrivateDnsNameOptions,
    ModifyPublicIpDnsNameOptions,
    ModifyReservedInstances,
    ModifyRouteServer,
    ModifySecurityGroupRules,
    ModifySnapshotAttribute,
    ModifySnapshotTier,
    ModifySpotFleetRequest,
    ModifySubnetAttribute,
    ModifyTrafficMirrorFilterNetworkServices,
    ModifyTrafficMirrorFilterRule,
    ModifyTrafficMirrorSession,
    ModifyTransitGateway,
    ModifyTransitGatewayPrefixListReference,
    ModifyTransitGatewayVpcAttachment,
    ModifyVerifiedAccessEndpoint,
    ModifyVerifiedAccessEndpointPolicy,
    ModifyVerifiedAccessGroup,
    ModifyVerifiedAccessGroupPolicy,
    ModifyVerifiedAccessInstance,
    ModifyVerifiedAccessInstanceLoggingConfiguration,
    ModifyVerifiedAccessTrustProvider,
    ModifyVolume,
    ModifyVolumeAttribute,
    ModifyVpcAttribute,
    ModifyVpcBlockPublicAccessExclusion,
    ModifyVpcBlockPublicAccessOptions,
    ModifyVpcEndpoint,
    ModifyVpcEndpointConnectionNotification,
    ModifyVpcEndpointServiceConfiguration,
    ModifyVpcEndpointServicePayerResponsibility,
    ModifyVpcEndpointServicePermissions,
    ModifyVpcPeeringConnectionOptions,
    ModifyVpcTenancy,
    ModifyVpnConnection,
    ModifyVpnConnectionOptions,
    ModifyVpnTunnelCertificate,
    ModifyVpnTunnelOptions,
    MonitorInstances,
    MoveAddressToVpc,
    MoveByoipCidrToIpam,
    MoveCapacityReservationInstances,
    PauseVolumeIo,
    ProvisionByoipCidr,
    ProvisionIpamByoasn,
    ProvisionIpamPoolCidr,
    ProvisionPublicIpv4PoolCidr,
    PurchaseCapacityBlock,
    PurchaseCapacityBlockExtension,
    PurchaseHostReservation,
    PurchaseReservedInstancesOffering,
    PurchaseScheduledInstances,
    PutResourcePolicy,
    RebootInstances,
    RegisterImage,
    RegisterInstanceEventNotificationAttributes,
    RegisterTransitGatewayMulticastGroupMembers,
    RegisterTransitGatewayMulticastGroupSources,
    RejectCapacityReservationBillingOwnership,
    RejectTransitGatewayMulticastDomainAssociations,
    RejectTransitGatewayPeeringAttachment,
    RejectTransitGatewayVpcAttachment,
    RejectVpcEndpointConnections,
    RejectVpcPeeringConnection,
    ReleaseAddress,
    ReleaseHosts,
    ReleaseIpamPoolAllocation,
    ReplaceIamInstanceProfileAssociation,
    ReplaceImageCriteriaInAllowedImagesSettings,
    ReplaceNetworkAclAssociation,
    ReplaceNetworkAclEntry,
    ReplaceRoute,
    ReplaceRouteTableAssociation,
    ReplaceTransitGatewayRoute,
    ReplaceVpnTunnel,
    ReportInstanceStatus,
    RequestSpotFleet,
    RequestSpotInstances,
    ResetAddressAttribute,
    ResetEbsDefaultKmsKeyId,
    ResetFpgaImageAttribute,
    ResetImageAttribute,
    ResetInstanceAttribute,
    ResetNetworkInterfaceAttribute,
    ResetSnapshotAttribute,
    RestoreAddressToClassic,
    RestoreImageFromRecycleBin,
    RestoreManagedPrefixListVersion,
    RestoreSnapshotFromRecycleBin,
    RestoreSnapshotTier,
    RevokeClientVpnIngress,
    RevokeSecurityGroupEgress,
    RevokeSecurityGroupIngress,
    RunInstances,
    RunScheduledInstances,
    SearchLocalGatewayRoutes,
    SearchTransitGatewayMulticastGroups,
    SearchTransitGatewayRoutes,
    SendDiagnosticInterrupt,
    SendSpotInstanceInterruptions,
    StartDeclarativePoliciesReport,
    StartInstances,
    StartNetworkInsightsAccessScopeAnalysis,
    StartNetworkInsightsAnalysis,
    StartVpcEndpointServicePrivateDnsVerification,
    StopInstances,
    TerminateClientVpnConnections,
    TerminateInstances,
    UnassignIpv6Addresses,
    UnassignPrivateIpAddresses,
    UnassignPrivateNatGatewayAddress,
    UnlockSnapshot,
    UnmonitorInstances,
    UpdateSecurityGroupRuleDescriptionsEgress,
    UpdateSecurityGroupRuleDescriptionsIngress,
    WithdrawByoipCidr,
}
impl std::fmt::Display for Ec2Actions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Ec2Actions::AcceptAddressTransfer => write!(f, "ec2:AcceptAddressTransfer"),
            Ec2Actions::AcceptCapacityReservationBillingOwnership => {
                write!(f, "ec2:AcceptCapacityReservationBillingOwnership")
            }
            Ec2Actions::AcceptReservedInstancesExchangeQuote => {
                write!(f, "ec2:AcceptReservedInstancesExchangeQuote")
            }
            Ec2Actions::AcceptTransitGatewayMulticastDomainAssociations => {
                write!(f, "ec2:AcceptTransitGatewayMulticastDomainAssociations")
            }
            Ec2Actions::AcceptTransitGatewayPeeringAttachment => {
                write!(f, "ec2:AcceptTransitGatewayPeeringAttachment")
            }
            Ec2Actions::AcceptTransitGatewayVpcAttachment => {
                write!(f, "ec2:AcceptTransitGatewayVpcAttachment")
            }
            Ec2Actions::AcceptVpcEndpointConnections => {
                write!(f, "ec2:AcceptVpcEndpointConnections")
            }
            Ec2Actions::AcceptVpcPeeringConnection => write!(f, "ec2:AcceptVpcPeeringConnection"),
            Ec2Actions::AdvertiseByoipCidr => write!(f, "ec2:AdvertiseByoipCidr"),
            Ec2Actions::AllocateAddress => write!(f, "ec2:AllocateAddress"),
            Ec2Actions::AllocateHosts => write!(f, "ec2:AllocateHosts"),
            Ec2Actions::AllocateIpamPoolCidr => write!(f, "ec2:AllocateIpamPoolCidr"),
            Ec2Actions::ApplySecurityGroupsToClientVpnTargetNetwork => {
                write!(f, "ec2:ApplySecurityGroupsToClientVpnTargetNetwork")
            }
            Ec2Actions::AssignIpv6Addresses => write!(f, "ec2:AssignIpv6Addresses"),
            Ec2Actions::AssignPrivateIpAddresses => write!(f, "ec2:AssignPrivateIpAddresses"),
            Ec2Actions::AssignPrivateNatGatewayAddress => {
                write!(f, "ec2:AssignPrivateNatGatewayAddress")
            }
            Ec2Actions::AssociateAddress => write!(f, "ec2:AssociateAddress"),
            Ec2Actions::AssociateCapacityReservationBillingOwner => {
                write!(f, "ec2:AssociateCapacityReservationBillingOwner")
            }
            Ec2Actions::AssociateClientVpnTargetNetwork => {
                write!(f, "ec2:AssociateClientVpnTargetNetwork")
            }
            Ec2Actions::AssociateDhcpOptions => write!(f, "ec2:AssociateDhcpOptions"),
            Ec2Actions::AssociateEnclaveCertificateIamRole => {
                write!(f, "ec2:AssociateEnclaveCertificateIamRole")
            }
            Ec2Actions::AssociateIamInstanceProfile => write!(f, "ec2:AssociateIamInstanceProfile"),
            Ec2Actions::AssociateInstanceEventWindow => {
                write!(f, "ec2:AssociateInstanceEventWindow")
            }
            Ec2Actions::AssociateIpamByoasn => write!(f, "ec2:AssociateIpamByoasn"),
            Ec2Actions::AssociateIpamResourceDiscovery => {
                write!(f, "ec2:AssociateIpamResourceDiscovery")
            }
            Ec2Actions::AssociateNatGatewayAddress => write!(f, "ec2:AssociateNatGatewayAddress"),
            Ec2Actions::AssociateRouteServer => write!(f, "ec2:AssociateRouteServer"),
            Ec2Actions::AssociateRouteTable => write!(f, "ec2:AssociateRouteTable"),
            Ec2Actions::AssociateSecurityGroupVpc => write!(f, "ec2:AssociateSecurityGroupVpc"),
            Ec2Actions::AssociateSubnetCidrBlock => write!(f, "ec2:AssociateSubnetCidrBlock"),
            Ec2Actions::AssociateTransitGatewayMulticastDomain => {
                write!(f, "ec2:AssociateTransitGatewayMulticastDomain")
            }
            Ec2Actions::AssociateTransitGatewayPolicyTable => {
                write!(f, "ec2:AssociateTransitGatewayPolicyTable")
            }
            Ec2Actions::AssociateTransitGatewayRouteTable => {
                write!(f, "ec2:AssociateTransitGatewayRouteTable")
            }
            Ec2Actions::AssociateTrunkInterface => write!(f, "ec2:AssociateTrunkInterface"),
            Ec2Actions::AssociateVerifiedAccessInstanceWebAcl => {
                write!(f, "ec2:AssociateVerifiedAccessInstanceWebAcl")
            }
            Ec2Actions::AssociateVpcCidrBlock => write!(f, "ec2:AssociateVpcCidrBlock"),
            Ec2Actions::AttachClassicLinkVpc => write!(f, "ec2:AttachClassicLinkVpc"),
            Ec2Actions::AttachInternetGateway => write!(f, "ec2:AttachInternetGateway"),
            Ec2Actions::AttachNetworkInterface => write!(f, "ec2:AttachNetworkInterface"),
            Ec2Actions::AttachVerifiedAccessTrustProvider => {
                write!(f, "ec2:AttachVerifiedAccessTrustProvider")
            }
            Ec2Actions::AttachVolume => write!(f, "ec2:AttachVolume"),
            Ec2Actions::AttachVpnGateway => write!(f, "ec2:AttachVpnGateway"),
            Ec2Actions::AuthorizeClientVpnIngress => write!(f, "ec2:AuthorizeClientVpnIngress"),
            Ec2Actions::AuthorizeSecurityGroupEgress => {
                write!(f, "ec2:AuthorizeSecurityGroupEgress")
            }
            Ec2Actions::AuthorizeSecurityGroupIngress => {
                write!(f, "ec2:AuthorizeSecurityGroupIngress")
            }
            Ec2Actions::BundleInstance => write!(f, "ec2:BundleInstance"),
            Ec2Actions::CancelBundleTask => write!(f, "ec2:CancelBundleTask"),
            Ec2Actions::CancelCapacityReservation => write!(f, "ec2:CancelCapacityReservation"),
            Ec2Actions::CancelCapacityReservationFleets => {
                write!(f, "ec2:CancelCapacityReservationFleets")
            }
            Ec2Actions::CancelConversionTask => write!(f, "ec2:CancelConversionTask"),
            Ec2Actions::CancelDeclarativePoliciesReport => {
                write!(f, "ec2:CancelDeclarativePoliciesReport")
            }
            Ec2Actions::CancelExportTask => write!(f, "ec2:CancelExportTask"),
            Ec2Actions::CancelImageLaunchPermission => write!(f, "ec2:CancelImageLaunchPermission"),
            Ec2Actions::CancelImportTask => write!(f, "ec2:CancelImportTask"),
            Ec2Actions::CancelReservedInstancesListing => {
                write!(f, "ec2:CancelReservedInstancesListing")
            }
            Ec2Actions::CancelSpotFleetRequests => write!(f, "ec2:CancelSpotFleetRequests"),
            Ec2Actions::CancelSpotInstanceRequests => write!(f, "ec2:CancelSpotInstanceRequests"),
            Ec2Actions::ConfirmProductInstance => write!(f, "ec2:ConfirmProductInstance"),
            Ec2Actions::CopyFpgaImage => write!(f, "ec2:CopyFpgaImage"),
            Ec2Actions::CopyImage => write!(f, "ec2:CopyImage"),
            Ec2Actions::CopySnapshot => write!(f, "ec2:CopySnapshot"),
            Ec2Actions::CreateCapacityReservation => write!(f, "ec2:CreateCapacityReservation"),
            Ec2Actions::CreateCapacityReservationBySplitting => {
                write!(f, "ec2:CreateCapacityReservationBySplitting")
            }
            Ec2Actions::CreateCapacityReservationFleet => {
                write!(f, "ec2:CreateCapacityReservationFleet")
            }
            Ec2Actions::CreateCarrierGateway => write!(f, "ec2:CreateCarrierGateway"),
            Ec2Actions::CreateClientVpnEndpoint => write!(f, "ec2:CreateClientVpnEndpoint"),
            Ec2Actions::CreateClientVpnRoute => write!(f, "ec2:CreateClientVpnRoute"),
            Ec2Actions::CreateCoipCidr => write!(f, "ec2:CreateCoipCidr"),
            Ec2Actions::CreateCoipPool => write!(f, "ec2:CreateCoipPool"),
            Ec2Actions::CreateCoipPoolPermission => write!(f, "ec2:CreateCoipPoolPermission"),
            Ec2Actions::CreateCustomerGateway => write!(f, "ec2:CreateCustomerGateway"),
            Ec2Actions::CreateDefaultSubnet => write!(f, "ec2:CreateDefaultSubnet"),
            Ec2Actions::CreateDefaultVpc => write!(f, "ec2:CreateDefaultVpc"),
            Ec2Actions::CreateDelegateMacVolumeOwnershipTask => {
                write!(f, "ec2:CreateDelegateMacVolumeOwnershipTask")
            }
            Ec2Actions::CreateDhcpOptions => write!(f, "ec2:CreateDhcpOptions"),
            Ec2Actions::CreateEgressOnlyInternetGateway => {
                write!(f, "ec2:CreateEgressOnlyInternetGateway")
            }
            Ec2Actions::CreateFleet => write!(f, "ec2:CreateFleet"),
            Ec2Actions::CreateFlowLogs => write!(f, "ec2:CreateFlowLogs"),
            Ec2Actions::CreateFpgaImage => write!(f, "ec2:CreateFpgaImage"),
            Ec2Actions::CreateImage => write!(f, "ec2:CreateImage"),
            Ec2Actions::CreateInstanceConnectEndpoint => {
                write!(f, "ec2:CreateInstanceConnectEndpoint")
            }
            Ec2Actions::CreateInstanceEventWindow => write!(f, "ec2:CreateInstanceEventWindow"),
            Ec2Actions::CreateInstanceExportTask => write!(f, "ec2:CreateInstanceExportTask"),
            Ec2Actions::CreateInternetGateway => write!(f, "ec2:CreateInternetGateway"),
            Ec2Actions::CreateIpam => write!(f, "ec2:CreateIpam"),
            Ec2Actions::CreateIpamExternalResourceVerificationToken => {
                write!(f, "ec2:CreateIpamExternalResourceVerificationToken")
            }
            Ec2Actions::CreateIpamPool => write!(f, "ec2:CreateIpamPool"),
            Ec2Actions::CreateIpamResourceDiscovery => write!(f, "ec2:CreateIpamResourceDiscovery"),
            Ec2Actions::CreateIpamScope => write!(f, "ec2:CreateIpamScope"),
            Ec2Actions::CreateKeyPair => write!(f, "ec2:CreateKeyPair"),
            Ec2Actions::CreateLaunchTemplate => write!(f, "ec2:CreateLaunchTemplate"),
            Ec2Actions::CreateLaunchTemplateVersion => write!(f, "ec2:CreateLaunchTemplateVersion"),
            Ec2Actions::CreateLocalGatewayRoute => write!(f, "ec2:CreateLocalGatewayRoute"),
            Ec2Actions::CreateLocalGatewayRouteTable => {
                write!(f, "ec2:CreateLocalGatewayRouteTable")
            }
            Ec2Actions::CreateLocalGatewayRouteTablePermission => {
                write!(f, "ec2:CreateLocalGatewayRouteTablePermission")
            }
            Ec2Actions::CreateLocalGatewayRouteTableVirtualInterfaceGroupAssociation => write!(
                f,
                "ec2:CreateLocalGatewayRouteTableVirtualInterfaceGroupAssociation"
            ),
            Ec2Actions::CreateLocalGatewayRouteTableVpcAssociation => {
                write!(f, "ec2:CreateLocalGatewayRouteTableVpcAssociation")
            }
            Ec2Actions::CreateLocalGatewayVirtualInterface => {
                write!(f, "ec2:CreateLocalGatewayVirtualInterface")
            }
            Ec2Actions::CreateLocalGatewayVirtualInterfaceGroup => {
                write!(f, "ec2:CreateLocalGatewayVirtualInterfaceGroup")
            }
            Ec2Actions::CreateMacSystemIntegrityProtectionModificationTask => {
                write!(f, "ec2:CreateMacSystemIntegrityProtectionModificationTask")
            }
            Ec2Actions::CreateManagedPrefixList => write!(f, "ec2:CreateManagedPrefixList"),
            Ec2Actions::CreateNatGateway => write!(f, "ec2:CreateNatGateway"),
            Ec2Actions::CreateNetworkAcl => write!(f, "ec2:CreateNetworkAcl"),
            Ec2Actions::CreateNetworkAclEntry => write!(f, "ec2:CreateNetworkAclEntry"),
            Ec2Actions::CreateNetworkInsightsAccessScope => {
                write!(f, "ec2:CreateNetworkInsightsAccessScope")
            }
            Ec2Actions::CreateNetworkInsightsPath => write!(f, "ec2:CreateNetworkInsightsPath"),
            Ec2Actions::CreateNetworkInterface => write!(f, "ec2:CreateNetworkInterface"),
            Ec2Actions::CreateNetworkInterfacePermission => {
                write!(f, "ec2:CreateNetworkInterfacePermission")
            }
            Ec2Actions::CreatePlacementGroup => write!(f, "ec2:CreatePlacementGroup"),
            Ec2Actions::CreatePublicIpv4Pool => write!(f, "ec2:CreatePublicIpv4Pool"),
            Ec2Actions::CreateReplaceRootVolumeTask => write!(f, "ec2:CreateReplaceRootVolumeTask"),
            Ec2Actions::CreateReservedInstancesListing => {
                write!(f, "ec2:CreateReservedInstancesListing")
            }
            Ec2Actions::CreateRestoreImageTask => write!(f, "ec2:CreateRestoreImageTask"),
            Ec2Actions::CreateRoute => write!(f, "ec2:CreateRoute"),
            Ec2Actions::CreateRouteServer => write!(f, "ec2:CreateRouteServer"),
            Ec2Actions::CreateRouteServerEndpoint => write!(f, "ec2:CreateRouteServerEndpoint"),
            Ec2Actions::CreateRouteServerPeer => write!(f, "ec2:CreateRouteServerPeer"),
            Ec2Actions::CreateRouteTable => write!(f, "ec2:CreateRouteTable"),
            Ec2Actions::CreateSecurityGroup => write!(f, "ec2:CreateSecurityGroup"),
            Ec2Actions::CreateSnapshot => write!(f, "ec2:CreateSnapshot"),
            Ec2Actions::CreateSnapshots => write!(f, "ec2:CreateSnapshots"),
            Ec2Actions::CreateSpotDatafeedSubscription => {
                write!(f, "ec2:CreateSpotDatafeedSubscription")
            }
            Ec2Actions::CreateStoreImageTask => write!(f, "ec2:CreateStoreImageTask"),
            Ec2Actions::CreateSubnet => write!(f, "ec2:CreateSubnet"),
            Ec2Actions::CreateSubnetCidrReservation => write!(f, "ec2:CreateSubnetCidrReservation"),
            Ec2Actions::CreateTags => write!(f, "ec2:CreateTags"),
            Ec2Actions::CreateTrafficMirrorFilter => write!(f, "ec2:CreateTrafficMirrorFilter"),
            Ec2Actions::CreateTrafficMirrorFilterRule => {
                write!(f, "ec2:CreateTrafficMirrorFilterRule")
            }
            Ec2Actions::CreateTrafficMirrorSession => write!(f, "ec2:CreateTrafficMirrorSession"),
            Ec2Actions::CreateTrafficMirrorTarget => write!(f, "ec2:CreateTrafficMirrorTarget"),
            Ec2Actions::CreateTransitGateway => write!(f, "ec2:CreateTransitGateway"),
            Ec2Actions::CreateTransitGatewayConnect => write!(f, "ec2:CreateTransitGatewayConnect"),
            Ec2Actions::CreateTransitGatewayConnectPeer => {
                write!(f, "ec2:CreateTransitGatewayConnectPeer")
            }
            Ec2Actions::CreateTransitGatewayMulticastDomain => {
                write!(f, "ec2:CreateTransitGatewayMulticastDomain")
            }
            Ec2Actions::CreateTransitGatewayPeeringAttachment => {
                write!(f, "ec2:CreateTransitGatewayPeeringAttachment")
            }
            Ec2Actions::CreateTransitGatewayPolicyTable => {
                write!(f, "ec2:CreateTransitGatewayPolicyTable")
            }
            Ec2Actions::CreateTransitGatewayPrefixListReference => {
                write!(f, "ec2:CreateTransitGatewayPrefixListReference")
            }
            Ec2Actions::CreateTransitGatewayRoute => write!(f, "ec2:CreateTransitGatewayRoute"),
            Ec2Actions::CreateTransitGatewayRouteTable => {
                write!(f, "ec2:CreateTransitGatewayRouteTable")
            }
            Ec2Actions::CreateTransitGatewayRouteTableAnnouncement => {
                write!(f, "ec2:CreateTransitGatewayRouteTableAnnouncement")
            }
            Ec2Actions::CreateTransitGatewayVpcAttachment => {
                write!(f, "ec2:CreateTransitGatewayVpcAttachment")
            }
            Ec2Actions::CreateVerifiedAccessEndpoint => {
                write!(f, "ec2:CreateVerifiedAccessEndpoint")
            }
            Ec2Actions::CreateVerifiedAccessGroup => write!(f, "ec2:CreateVerifiedAccessGroup"),
            Ec2Actions::CreateVerifiedAccessInstance => {
                write!(f, "ec2:CreateVerifiedAccessInstance")
            }
            Ec2Actions::CreateVerifiedAccessTrustProvider => {
                write!(f, "ec2:CreateVerifiedAccessTrustProvider")
            }
            Ec2Actions::CreateVolume => write!(f, "ec2:CreateVolume"),
            Ec2Actions::CreateVpc => write!(f, "ec2:CreateVpc"),
            Ec2Actions::CreateVpcBlockPublicAccessExclusion => {
                write!(f, "ec2:CreateVpcBlockPublicAccessExclusion")
            }
            Ec2Actions::CreateVpcEndpoint => write!(f, "ec2:CreateVpcEndpoint"),
            Ec2Actions::CreateVpcEndpointConnectionNotification => {
                write!(f, "ec2:CreateVpcEndpointConnectionNotification")
            }
            Ec2Actions::CreateVpcEndpointServiceConfiguration => {
                write!(f, "ec2:CreateVpcEndpointServiceConfiguration")
            }
            Ec2Actions::CreateVpcPeeringConnection => write!(f, "ec2:CreateVpcPeeringConnection"),
            Ec2Actions::CreateVpnConnection => write!(f, "ec2:CreateVpnConnection"),
            Ec2Actions::CreateVpnConnectionRoute => write!(f, "ec2:CreateVpnConnectionRoute"),
            Ec2Actions::CreateVpnGateway => write!(f, "ec2:CreateVpnGateway"),
            Ec2Actions::DeleteCarrierGateway => write!(f, "ec2:DeleteCarrierGateway"),
            Ec2Actions::DeleteClientVpnEndpoint => write!(f, "ec2:DeleteClientVpnEndpoint"),
            Ec2Actions::DeleteClientVpnRoute => write!(f, "ec2:DeleteClientVpnRoute"),
            Ec2Actions::DeleteCoipCidr => write!(f, "ec2:DeleteCoipCidr"),
            Ec2Actions::DeleteCoipPool => write!(f, "ec2:DeleteCoipPool"),
            Ec2Actions::DeleteCoipPoolPermission => write!(f, "ec2:DeleteCoipPoolPermission"),
            Ec2Actions::DeleteCustomerGateway => write!(f, "ec2:DeleteCustomerGateway"),
            Ec2Actions::DeleteDhcpOptions => write!(f, "ec2:DeleteDhcpOptions"),
            Ec2Actions::DeleteEgressOnlyInternetGateway => {
                write!(f, "ec2:DeleteEgressOnlyInternetGateway")
            }
            Ec2Actions::DeleteFleets => write!(f, "ec2:DeleteFleets"),
            Ec2Actions::DeleteFlowLogs => write!(f, "ec2:DeleteFlowLogs"),
            Ec2Actions::DeleteFpgaImage => write!(f, "ec2:DeleteFpgaImage"),
            Ec2Actions::DeleteInstanceConnectEndpoint => {
                write!(f, "ec2:DeleteInstanceConnectEndpoint")
            }
            Ec2Actions::DeleteInstanceEventWindow => write!(f, "ec2:DeleteInstanceEventWindow"),
            Ec2Actions::DeleteInternetGateway => write!(f, "ec2:DeleteInternetGateway"),
            Ec2Actions::DeleteIpam => write!(f, "ec2:DeleteIpam"),
            Ec2Actions::DeleteIpamExternalResourceVerificationToken => {
                write!(f, "ec2:DeleteIpamExternalResourceVerificationToken")
            }
            Ec2Actions::DeleteIpamPool => write!(f, "ec2:DeleteIpamPool"),
            Ec2Actions::DeleteIpamResourceDiscovery => write!(f, "ec2:DeleteIpamResourceDiscovery"),
            Ec2Actions::DeleteIpamScope => write!(f, "ec2:DeleteIpamScope"),
            Ec2Actions::DeleteKeyPair => write!(f, "ec2:DeleteKeyPair"),
            Ec2Actions::DeleteLaunchTemplate => write!(f, "ec2:DeleteLaunchTemplate"),
            Ec2Actions::DeleteLaunchTemplateVersions => {
                write!(f, "ec2:DeleteLaunchTemplateVersions")
            }
            Ec2Actions::DeleteLocalGatewayRoute => write!(f, "ec2:DeleteLocalGatewayRoute"),
            Ec2Actions::DeleteLocalGatewayRouteTable => {
                write!(f, "ec2:DeleteLocalGatewayRouteTable")
            }
            Ec2Actions::DeleteLocalGatewayRouteTablePermission => {
                write!(f, "ec2:DeleteLocalGatewayRouteTablePermission")
            }
            Ec2Actions::DeleteLocalGatewayRouteTableVirtualInterfaceGroupAssociation => write!(
                f,
                "ec2:DeleteLocalGatewayRouteTableVirtualInterfaceGroupAssociation"
            ),
            Ec2Actions::DeleteLocalGatewayRouteTableVpcAssociation => {
                write!(f, "ec2:DeleteLocalGatewayRouteTableVpcAssociation")
            }
            Ec2Actions::DeleteLocalGatewayVirtualInterface => {
                write!(f, "ec2:DeleteLocalGatewayVirtualInterface")
            }
            Ec2Actions::DeleteLocalGatewayVirtualInterfaceGroup => {
                write!(f, "ec2:DeleteLocalGatewayVirtualInterfaceGroup")
            }
            Ec2Actions::DeleteManagedPrefixList => write!(f, "ec2:DeleteManagedPrefixList"),
            Ec2Actions::DeleteNatGateway => write!(f, "ec2:DeleteNatGateway"),
            Ec2Actions::DeleteNetworkAcl => write!(f, "ec2:DeleteNetworkAcl"),
            Ec2Actions::DeleteNetworkAclEntry => write!(f, "ec2:DeleteNetworkAclEntry"),
            Ec2Actions::DeleteNetworkInsightsAccessScope => {
                write!(f, "ec2:DeleteNetworkInsightsAccessScope")
            }
            Ec2Actions::DeleteNetworkInsightsAccessScopeAnalysis => {
                write!(f, "ec2:DeleteNetworkInsightsAccessScopeAnalysis")
            }
            Ec2Actions::DeleteNetworkInsightsAnalysis => {
                write!(f, "ec2:DeleteNetworkInsightsAnalysis")
            }
            Ec2Actions::DeleteNetworkInsightsPath => write!(f, "ec2:DeleteNetworkInsightsPath"),
            Ec2Actions::DeleteNetworkInterface => write!(f, "ec2:DeleteNetworkInterface"),
            Ec2Actions::DeleteNetworkInterfacePermission => {
                write!(f, "ec2:DeleteNetworkInterfacePermission")
            }
            Ec2Actions::DeletePlacementGroup => write!(f, "ec2:DeletePlacementGroup"),
            Ec2Actions::DeletePublicIpv4Pool => write!(f, "ec2:DeletePublicIpv4Pool"),
            Ec2Actions::DeleteQueuedReservedInstances => {
                write!(f, "ec2:DeleteQueuedReservedInstances")
            }
            Ec2Actions::DeleteResourcePolicy => write!(f, "ec2:DeleteResourcePolicy"),
            Ec2Actions::DeleteRoute => write!(f, "ec2:DeleteRoute"),
            Ec2Actions::DeleteRouteServer => write!(f, "ec2:DeleteRouteServer"),
            Ec2Actions::DeleteRouteServerEndpoint => write!(f, "ec2:DeleteRouteServerEndpoint"),
            Ec2Actions::DeleteRouteServerPeer => write!(f, "ec2:DeleteRouteServerPeer"),
            Ec2Actions::DeleteRouteTable => write!(f, "ec2:DeleteRouteTable"),
            Ec2Actions::DeleteSecurityGroup => write!(f, "ec2:DeleteSecurityGroup"),
            Ec2Actions::DeleteSnapshot => write!(f, "ec2:DeleteSnapshot"),
            Ec2Actions::DeleteSpotDatafeedSubscription => {
                write!(f, "ec2:DeleteSpotDatafeedSubscription")
            }
            Ec2Actions::DeleteSubnet => write!(f, "ec2:DeleteSubnet"),
            Ec2Actions::DeleteSubnetCidrReservation => write!(f, "ec2:DeleteSubnetCidrReservation"),
            Ec2Actions::DeleteTags => write!(f, "ec2:DeleteTags"),
            Ec2Actions::DeleteTrafficMirrorFilter => write!(f, "ec2:DeleteTrafficMirrorFilter"),
            Ec2Actions::DeleteTrafficMirrorFilterRule => {
                write!(f, "ec2:DeleteTrafficMirrorFilterRule")
            }
            Ec2Actions::DeleteTrafficMirrorSession => write!(f, "ec2:DeleteTrafficMirrorSession"),
            Ec2Actions::DeleteTrafficMirrorTarget => write!(f, "ec2:DeleteTrafficMirrorTarget"),
            Ec2Actions::DeleteTransitGateway => write!(f, "ec2:DeleteTransitGateway"),
            Ec2Actions::DeleteTransitGatewayConnect => write!(f, "ec2:DeleteTransitGatewayConnect"),
            Ec2Actions::DeleteTransitGatewayConnectPeer => {
                write!(f, "ec2:DeleteTransitGatewayConnectPeer")
            }
            Ec2Actions::DeleteTransitGatewayMulticastDomain => {
                write!(f, "ec2:DeleteTransitGatewayMulticastDomain")
            }
            Ec2Actions::DeleteTransitGatewayPeeringAttachment => {
                write!(f, "ec2:DeleteTransitGatewayPeeringAttachment")
            }
            Ec2Actions::DeleteTransitGatewayPolicyTable => {
                write!(f, "ec2:DeleteTransitGatewayPolicyTable")
            }
            Ec2Actions::DeleteTransitGatewayPrefixListReference => {
                write!(f, "ec2:DeleteTransitGatewayPrefixListReference")
            }
            Ec2Actions::DeleteTransitGatewayRoute => write!(f, "ec2:DeleteTransitGatewayRoute"),
            Ec2Actions::DeleteTransitGatewayRouteTable => {
                write!(f, "ec2:DeleteTransitGatewayRouteTable")
            }
            Ec2Actions::DeleteTransitGatewayRouteTableAnnouncement => {
                write!(f, "ec2:DeleteTransitGatewayRouteTableAnnouncement")
            }
            Ec2Actions::DeleteTransitGatewayVpcAttachment => {
                write!(f, "ec2:DeleteTransitGatewayVpcAttachment")
            }
            Ec2Actions::DeleteVerifiedAccessEndpoint => {
                write!(f, "ec2:DeleteVerifiedAccessEndpoint")
            }
            Ec2Actions::DeleteVerifiedAccessGroup => write!(f, "ec2:DeleteVerifiedAccessGroup"),
            Ec2Actions::DeleteVerifiedAccessInstance => {
                write!(f, "ec2:DeleteVerifiedAccessInstance")
            }
            Ec2Actions::DeleteVerifiedAccessTrustProvider => {
                write!(f, "ec2:DeleteVerifiedAccessTrustProvider")
            }
            Ec2Actions::DeleteVolume => write!(f, "ec2:DeleteVolume"),
            Ec2Actions::DeleteVpc => write!(f, "ec2:DeleteVpc"),
            Ec2Actions::DeleteVpcBlockPublicAccessExclusion => {
                write!(f, "ec2:DeleteVpcBlockPublicAccessExclusion")
            }
            Ec2Actions::DeleteVpcEndpointConnectionNotifications => {
                write!(f, "ec2:DeleteVpcEndpointConnectionNotifications")
            }
            Ec2Actions::DeleteVpcEndpointServiceConfigurations => {
                write!(f, "ec2:DeleteVpcEndpointServiceConfigurations")
            }
            Ec2Actions::DeleteVpcEndpoints => write!(f, "ec2:DeleteVpcEndpoints"),
            Ec2Actions::DeleteVpcPeeringConnection => write!(f, "ec2:DeleteVpcPeeringConnection"),
            Ec2Actions::DeleteVpnConnection => write!(f, "ec2:DeleteVpnConnection"),
            Ec2Actions::DeleteVpnConnectionRoute => write!(f, "ec2:DeleteVpnConnectionRoute"),
            Ec2Actions::DeleteVpnGateway => write!(f, "ec2:DeleteVpnGateway"),
            Ec2Actions::DeprovisionByoipCidr => write!(f, "ec2:DeprovisionByoipCidr"),
            Ec2Actions::DeprovisionIpamByoasn => write!(f, "ec2:DeprovisionIpamByoasn"),
            Ec2Actions::DeprovisionIpamPoolCidr => write!(f, "ec2:DeprovisionIpamPoolCidr"),
            Ec2Actions::DeprovisionPublicIpv4PoolCidr => {
                write!(f, "ec2:DeprovisionPublicIpv4PoolCidr")
            }
            Ec2Actions::DeregisterImage => write!(f, "ec2:DeregisterImage"),
            Ec2Actions::DeregisterInstanceEventNotificationAttributes => {
                write!(f, "ec2:DeregisterInstanceEventNotificationAttributes")
            }
            Ec2Actions::DeregisterTransitGatewayMulticastGroupMembers => {
                write!(f, "ec2:DeregisterTransitGatewayMulticastGroupMembers")
            }
            Ec2Actions::DeregisterTransitGatewayMulticastGroupSources => {
                write!(f, "ec2:DeregisterTransitGatewayMulticastGroupSources")
            }
            Ec2Actions::DescribeAccountAttributes => write!(f, "ec2:DescribeAccountAttributes"),
            Ec2Actions::DescribeAddressTransfers => write!(f, "ec2:DescribeAddressTransfers"),
            Ec2Actions::DescribeAddresses => write!(f, "ec2:DescribeAddresses"),
            Ec2Actions::DescribeAddressesAttribute => write!(f, "ec2:DescribeAddressesAttribute"),
            Ec2Actions::DescribeAggregateIdFormat => write!(f, "ec2:DescribeAggregateIdFormat"),
            Ec2Actions::DescribeAvailabilityZones => write!(f, "ec2:DescribeAvailabilityZones"),
            Ec2Actions::DescribeAwsNetworkPerformanceMetricSubscriptions => {
                write!(f, "ec2:DescribeAwsNetworkPerformanceMetricSubscriptions")
            }
            Ec2Actions::DescribeBundleTasks => write!(f, "ec2:DescribeBundleTasks"),
            Ec2Actions::DescribeByoipCidrs => write!(f, "ec2:DescribeByoipCidrs"),
            Ec2Actions::DescribeCapacityBlockExtensionHistory => {
                write!(f, "ec2:DescribeCapacityBlockExtensionHistory")
            }
            Ec2Actions::DescribeCapacityBlockExtensionOfferings => {
                write!(f, "ec2:DescribeCapacityBlockExtensionOfferings")
            }
            Ec2Actions::DescribeCapacityBlockOfferings => {
                write!(f, "ec2:DescribeCapacityBlockOfferings")
            }
            Ec2Actions::DescribeCapacityBlockStatus => write!(f, "ec2:DescribeCapacityBlockStatus"),
            Ec2Actions::DescribeCapacityBlocks => write!(f, "ec2:DescribeCapacityBlocks"),
            Ec2Actions::DescribeCapacityReservationBillingRequests => {
                write!(f, "ec2:DescribeCapacityReservationBillingRequests")
            }
            Ec2Actions::DescribeCapacityReservationFleets => {
                write!(f, "ec2:DescribeCapacityReservationFleets")
            }
            Ec2Actions::DescribeCapacityReservations => {
                write!(f, "ec2:DescribeCapacityReservations")
            }
            Ec2Actions::DescribeCarrierGateways => write!(f, "ec2:DescribeCarrierGateways"),
            Ec2Actions::DescribeClassicLinkInstances => {
                write!(f, "ec2:DescribeClassicLinkInstances")
            }
            Ec2Actions::DescribeClientVpnAuthorizationRules => {
                write!(f, "ec2:DescribeClientVpnAuthorizationRules")
            }
            Ec2Actions::DescribeClientVpnConnections => {
                write!(f, "ec2:DescribeClientVpnConnections")
            }
            Ec2Actions::DescribeClientVpnEndpoints => write!(f, "ec2:DescribeClientVpnEndpoints"),
            Ec2Actions::DescribeClientVpnRoutes => write!(f, "ec2:DescribeClientVpnRoutes"),
            Ec2Actions::DescribeClientVpnTargetNetworks => {
                write!(f, "ec2:DescribeClientVpnTargetNetworks")
            }
            Ec2Actions::DescribeCoipPools => write!(f, "ec2:DescribeCoipPools"),
            Ec2Actions::DescribeConversionTasks => write!(f, "ec2:DescribeConversionTasks"),
            Ec2Actions::DescribeCustomerGateways => write!(f, "ec2:DescribeCustomerGateways"),
            Ec2Actions::DescribeDeclarativePoliciesReports => {
                write!(f, "ec2:DescribeDeclarativePoliciesReports")
            }
            Ec2Actions::DescribeDhcpOptions => write!(f, "ec2:DescribeDhcpOptions"),
            Ec2Actions::DescribeEgressOnlyInternetGateways => {
                write!(f, "ec2:DescribeEgressOnlyInternetGateways")
            }
            Ec2Actions::DescribeElasticGpus => write!(f, "ec2:DescribeElasticGpus"),
            Ec2Actions::DescribeExportImageTasks => write!(f, "ec2:DescribeExportImageTasks"),
            Ec2Actions::DescribeExportTasks => write!(f, "ec2:DescribeExportTasks"),
            Ec2Actions::DescribeFastLaunchImages => write!(f, "ec2:DescribeFastLaunchImages"),
            Ec2Actions::DescribeFastSnapshotRestores => {
                write!(f, "ec2:DescribeFastSnapshotRestores")
            }
            Ec2Actions::DescribeFleetHistory => write!(f, "ec2:DescribeFleetHistory"),
            Ec2Actions::DescribeFleetInstances => write!(f, "ec2:DescribeFleetInstances"),
            Ec2Actions::DescribeFleets => write!(f, "ec2:DescribeFleets"),
            Ec2Actions::DescribeFlowLogs => write!(f, "ec2:DescribeFlowLogs"),
            Ec2Actions::DescribeFpgaImageAttribute => write!(f, "ec2:DescribeFpgaImageAttribute"),
            Ec2Actions::DescribeFpgaImages => write!(f, "ec2:DescribeFpgaImages"),
            Ec2Actions::DescribeHostReservationOfferings => {
                write!(f, "ec2:DescribeHostReservationOfferings")
            }
            Ec2Actions::DescribeHostReservations => write!(f, "ec2:DescribeHostReservations"),
            Ec2Actions::DescribeHosts => write!(f, "ec2:DescribeHosts"),
            Ec2Actions::DescribeIamInstanceProfileAssociations => {
                write!(f, "ec2:DescribeIamInstanceProfileAssociations")
            }
            Ec2Actions::DescribeIdFormat => write!(f, "ec2:DescribeIdFormat"),
            Ec2Actions::DescribeIdentityIdFormat => write!(f, "ec2:DescribeIdentityIdFormat"),
            Ec2Actions::DescribeImageAttribute => write!(f, "ec2:DescribeImageAttribute"),
            Ec2Actions::DescribeImages => write!(f, "ec2:DescribeImages"),
            Ec2Actions::DescribeImportImageTasks => write!(f, "ec2:DescribeImportImageTasks"),
            Ec2Actions::DescribeImportSnapshotTasks => write!(f, "ec2:DescribeImportSnapshotTasks"),
            Ec2Actions::DescribeInstanceAttribute => write!(f, "ec2:DescribeInstanceAttribute"),
            Ec2Actions::DescribeInstanceConnectEndpoints => {
                write!(f, "ec2:DescribeInstanceConnectEndpoints")
            }
            Ec2Actions::DescribeInstanceCreditSpecifications => {
                write!(f, "ec2:DescribeInstanceCreditSpecifications")
            }
            Ec2Actions::DescribeInstanceEventNotificationAttributes => {
                write!(f, "ec2:DescribeInstanceEventNotificationAttributes")
            }
            Ec2Actions::DescribeInstanceEventWindows => {
                write!(f, "ec2:DescribeInstanceEventWindows")
            }
            Ec2Actions::DescribeInstanceImageMetadata => {
                write!(f, "ec2:DescribeInstanceImageMetadata")
            }
            Ec2Actions::DescribeInstanceStatus => write!(f, "ec2:DescribeInstanceStatus"),
            Ec2Actions::DescribeInstanceTopology => write!(f, "ec2:DescribeInstanceTopology"),
            Ec2Actions::DescribeInstanceTypeOfferings => {
                write!(f, "ec2:DescribeInstanceTypeOfferings")
            }
            Ec2Actions::DescribeInstanceTypes => write!(f, "ec2:DescribeInstanceTypes"),
            Ec2Actions::DescribeInstances => write!(f, "ec2:DescribeInstances"),
            Ec2Actions::DescribeInternetGateways => write!(f, "ec2:DescribeInternetGateways"),
            Ec2Actions::DescribeIpamByoasn => write!(f, "ec2:DescribeIpamByoasn"),
            Ec2Actions::DescribeIpamExternalResourceVerificationTokens => {
                write!(f, "ec2:DescribeIpamExternalResourceVerificationTokens")
            }
            Ec2Actions::DescribeIpamPools => write!(f, "ec2:DescribeIpamPools"),
            Ec2Actions::DescribeIpamResourceDiscoveries => {
                write!(f, "ec2:DescribeIpamResourceDiscoveries")
            }
            Ec2Actions::DescribeIpamResourceDiscoveryAssociations => {
                write!(f, "ec2:DescribeIpamResourceDiscoveryAssociations")
            }
            Ec2Actions::DescribeIpamScopes => write!(f, "ec2:DescribeIpamScopes"),
            Ec2Actions::DescribeIpams => write!(f, "ec2:DescribeIpams"),
            Ec2Actions::DescribeIpv6Pools => write!(f, "ec2:DescribeIpv6Pools"),
            Ec2Actions::DescribeKeyPairs => write!(f, "ec2:DescribeKeyPairs"),
            Ec2Actions::DescribeLaunchTemplateVersions => {
                write!(f, "ec2:DescribeLaunchTemplateVersions")
            }
            Ec2Actions::DescribeLaunchTemplates => write!(f, "ec2:DescribeLaunchTemplates"),
            Ec2Actions::DescribeLocalGatewayRouteTablePermissions => {
                write!(f, "ec2:DescribeLocalGatewayRouteTablePermissions")
            }
            Ec2Actions::DescribeLocalGatewayRouteTableVirtualInterfaceGroupAssociations => write!(
                f,
                "ec2:DescribeLocalGatewayRouteTableVirtualInterfaceGroupAssociations"
            ),
            Ec2Actions::DescribeLocalGatewayRouteTableVpcAssociations => {
                write!(f, "ec2:DescribeLocalGatewayRouteTableVpcAssociations")
            }
            Ec2Actions::DescribeLocalGatewayRouteTables => {
                write!(f, "ec2:DescribeLocalGatewayRouteTables")
            }
            Ec2Actions::DescribeLocalGatewayVirtualInterfaceGroups => {
                write!(f, "ec2:DescribeLocalGatewayVirtualInterfaceGroups")
            }
            Ec2Actions::DescribeLocalGatewayVirtualInterfaces => {
                write!(f, "ec2:DescribeLocalGatewayVirtualInterfaces")
            }
            Ec2Actions::DescribeLocalGateways => write!(f, "ec2:DescribeLocalGateways"),
            Ec2Actions::DescribeLockedSnapshots => write!(f, "ec2:DescribeLockedSnapshots"),
            Ec2Actions::DescribeMacHosts => write!(f, "ec2:DescribeMacHosts"),
            Ec2Actions::DescribeMacModificationTasks => {
                write!(f, "ec2:DescribeMacModificationTasks")
            }
            Ec2Actions::DescribeManagedPrefixLists => write!(f, "ec2:DescribeManagedPrefixLists"),
            Ec2Actions::DescribeMovingAddresses => write!(f, "ec2:DescribeMovingAddresses"),
            Ec2Actions::DescribeNatGateways => write!(f, "ec2:DescribeNatGateways"),
            Ec2Actions::DescribeNetworkAcls => write!(f, "ec2:DescribeNetworkAcls"),
            Ec2Actions::DescribeNetworkInsightsAccessScopeAnalyses => {
                write!(f, "ec2:DescribeNetworkInsightsAccessScopeAnalyses")
            }
            Ec2Actions::DescribeNetworkInsightsAccessScopes => {
                write!(f, "ec2:DescribeNetworkInsightsAccessScopes")
            }
            Ec2Actions::DescribeNetworkInsightsAnalyses => {
                write!(f, "ec2:DescribeNetworkInsightsAnalyses")
            }
            Ec2Actions::DescribeNetworkInsightsPaths => {
                write!(f, "ec2:DescribeNetworkInsightsPaths")
            }
            Ec2Actions::DescribeNetworkInterfaceAttribute => {
                write!(f, "ec2:DescribeNetworkInterfaceAttribute")
            }
            Ec2Actions::DescribeNetworkInterfacePermissions => {
                write!(f, "ec2:DescribeNetworkInterfacePermissions")
            }
            Ec2Actions::DescribeNetworkInterfaces => write!(f, "ec2:DescribeNetworkInterfaces"),
            Ec2Actions::DescribeOutpostLags => write!(f, "ec2:DescribeOutpostLags"),
            Ec2Actions::DescribePlacementGroups => write!(f, "ec2:DescribePlacementGroups"),
            Ec2Actions::DescribePrefixLists => write!(f, "ec2:DescribePrefixLists"),
            Ec2Actions::DescribePrincipalIdFormat => write!(f, "ec2:DescribePrincipalIdFormat"),
            Ec2Actions::DescribePublicIpv4Pools => write!(f, "ec2:DescribePublicIpv4Pools"),
            Ec2Actions::DescribeRegions => write!(f, "ec2:DescribeRegions"),
            Ec2Actions::DescribeReplaceRootVolumeTasks => {
                write!(f, "ec2:DescribeReplaceRootVolumeTasks")
            }
            Ec2Actions::DescribeReservedInstances => write!(f, "ec2:DescribeReservedInstances"),
            Ec2Actions::DescribeReservedInstancesListings => {
                write!(f, "ec2:DescribeReservedInstancesListings")
            }
            Ec2Actions::DescribeReservedInstancesModifications => {
                write!(f, "ec2:DescribeReservedInstancesModifications")
            }
            Ec2Actions::DescribeReservedInstancesOfferings => {
                write!(f, "ec2:DescribeReservedInstancesOfferings")
            }
            Ec2Actions::DescribeRouteServerEndpoints => {
                write!(f, "ec2:DescribeRouteServerEndpoints")
            }
            Ec2Actions::DescribeRouteServerPeers => write!(f, "ec2:DescribeRouteServerPeers"),
            Ec2Actions::DescribeRouteServers => write!(f, "ec2:DescribeRouteServers"),
            Ec2Actions::DescribeRouteTables => write!(f, "ec2:DescribeRouteTables"),
            Ec2Actions::DescribeScheduledInstanceAvailability => {
                write!(f, "ec2:DescribeScheduledInstanceAvailability")
            }
            Ec2Actions::DescribeScheduledInstances => write!(f, "ec2:DescribeScheduledInstances"),
            Ec2Actions::DescribeSecurityGroupReferences => {
                write!(f, "ec2:DescribeSecurityGroupReferences")
            }
            Ec2Actions::DescribeSecurityGroupRules => write!(f, "ec2:DescribeSecurityGroupRules"),
            Ec2Actions::DescribeSecurityGroupVpcAssociations => {
                write!(f, "ec2:DescribeSecurityGroupVpcAssociations")
            }
            Ec2Actions::DescribeSecurityGroups => write!(f, "ec2:DescribeSecurityGroups"),
            Ec2Actions::DescribeServiceLinkVirtualInterfaces => {
                write!(f, "ec2:DescribeServiceLinkVirtualInterfaces")
            }
            Ec2Actions::DescribeSnapshotAttribute => write!(f, "ec2:DescribeSnapshotAttribute"),
            Ec2Actions::DescribeSnapshotTierStatus => write!(f, "ec2:DescribeSnapshotTierStatus"),
            Ec2Actions::DescribeSnapshots => write!(f, "ec2:DescribeSnapshots"),
            Ec2Actions::DescribeSpotDatafeedSubscription => {
                write!(f, "ec2:DescribeSpotDatafeedSubscription")
            }
            Ec2Actions::DescribeSpotFleetInstances => write!(f, "ec2:DescribeSpotFleetInstances"),
            Ec2Actions::DescribeSpotFleetRequestHistory => {
                write!(f, "ec2:DescribeSpotFleetRequestHistory")
            }
            Ec2Actions::DescribeSpotFleetRequests => write!(f, "ec2:DescribeSpotFleetRequests"),
            Ec2Actions::DescribeSpotInstanceRequests => {
                write!(f, "ec2:DescribeSpotInstanceRequests")
            }
            Ec2Actions::DescribeSpotPriceHistory => write!(f, "ec2:DescribeSpotPriceHistory"),
            Ec2Actions::DescribeStaleSecurityGroups => write!(f, "ec2:DescribeStaleSecurityGroups"),
            Ec2Actions::DescribeStoreImageTasks => write!(f, "ec2:DescribeStoreImageTasks"),
            Ec2Actions::DescribeSubnets => write!(f, "ec2:DescribeSubnets"),
            Ec2Actions::DescribeTags => write!(f, "ec2:DescribeTags"),
            Ec2Actions::DescribeTrafficMirrorFilterRules => {
                write!(f, "ec2:DescribeTrafficMirrorFilterRules")
            }
            Ec2Actions::DescribeTrafficMirrorFilters => {
                write!(f, "ec2:DescribeTrafficMirrorFilters")
            }
            Ec2Actions::DescribeTrafficMirrorSessions => {
                write!(f, "ec2:DescribeTrafficMirrorSessions")
            }
            Ec2Actions::DescribeTrafficMirrorTargets => {
                write!(f, "ec2:DescribeTrafficMirrorTargets")
            }
            Ec2Actions::DescribeTransitGatewayAttachments => {
                write!(f, "ec2:DescribeTransitGatewayAttachments")
            }
            Ec2Actions::DescribeTransitGatewayConnectPeers => {
                write!(f, "ec2:DescribeTransitGatewayConnectPeers")
            }
            Ec2Actions::DescribeTransitGatewayConnects => {
                write!(f, "ec2:DescribeTransitGatewayConnects")
            }
            Ec2Actions::DescribeTransitGatewayMulticastDomains => {
                write!(f, "ec2:DescribeTransitGatewayMulticastDomains")
            }
            Ec2Actions::DescribeTransitGatewayPeeringAttachments => {
                write!(f, "ec2:DescribeTransitGatewayPeeringAttachments")
            }
            Ec2Actions::DescribeTransitGatewayPolicyTables => {
                write!(f, "ec2:DescribeTransitGatewayPolicyTables")
            }
            Ec2Actions::DescribeTransitGatewayRouteTableAnnouncements => {
                write!(f, "ec2:DescribeTransitGatewayRouteTableAnnouncements")
            }
            Ec2Actions::DescribeTransitGatewayRouteTables => {
                write!(f, "ec2:DescribeTransitGatewayRouteTables")
            }
            Ec2Actions::DescribeTransitGatewayVpcAttachments => {
                write!(f, "ec2:DescribeTransitGatewayVpcAttachments")
            }
            Ec2Actions::DescribeTransitGateways => write!(f, "ec2:DescribeTransitGateways"),
            Ec2Actions::DescribeTrunkInterfaceAssociations => {
                write!(f, "ec2:DescribeTrunkInterfaceAssociations")
            }
            Ec2Actions::DescribeVerifiedAccessEndpoints => {
                write!(f, "ec2:DescribeVerifiedAccessEndpoints")
            }
            Ec2Actions::DescribeVerifiedAccessGroups => {
                write!(f, "ec2:DescribeVerifiedAccessGroups")
            }
            Ec2Actions::DescribeVerifiedAccessInstanceLoggingConfigurations => {
                write!(f, "ec2:DescribeVerifiedAccessInstanceLoggingConfigurations")
            }
            Ec2Actions::DescribeVerifiedAccessInstanceWebAclAssociations => {
                write!(f, "ec2:DescribeVerifiedAccessInstanceWebAclAssociations")
            }
            Ec2Actions::DescribeVerifiedAccessInstances => {
                write!(f, "ec2:DescribeVerifiedAccessInstances")
            }
            Ec2Actions::DescribeVerifiedAccessTrustProviders => {
                write!(f, "ec2:DescribeVerifiedAccessTrustProviders")
            }
            Ec2Actions::DescribeVolumeAttribute => write!(f, "ec2:DescribeVolumeAttribute"),
            Ec2Actions::DescribeVolumeStatus => write!(f, "ec2:DescribeVolumeStatus"),
            Ec2Actions::DescribeVolumes => write!(f, "ec2:DescribeVolumes"),
            Ec2Actions::DescribeVolumesModifications => {
                write!(f, "ec2:DescribeVolumesModifications")
            }
            Ec2Actions::DescribeVpcAttribute => write!(f, "ec2:DescribeVpcAttribute"),
            Ec2Actions::DescribeVpcBlockPublicAccessExclusions => {
                write!(f, "ec2:DescribeVpcBlockPublicAccessExclusions")
            }
            Ec2Actions::DescribeVpcBlockPublicAccessOptions => {
                write!(f, "ec2:DescribeVpcBlockPublicAccessOptions")
            }
            Ec2Actions::DescribeVpcClassicLink => write!(f, "ec2:DescribeVpcClassicLink"),
            Ec2Actions::DescribeVpcClassicLinkDnsSupport => {
                write!(f, "ec2:DescribeVpcClassicLinkDnsSupport")
            }
            Ec2Actions::DescribeVpcEndpointAssociations => {
                write!(f, "ec2:DescribeVpcEndpointAssociations")
            }
            Ec2Actions::DescribeVpcEndpointConnectionNotifications => {
                write!(f, "ec2:DescribeVpcEndpointConnectionNotifications")
            }
            Ec2Actions::DescribeVpcEndpointConnections => {
                write!(f, "ec2:DescribeVpcEndpointConnections")
            }
            Ec2Actions::DescribeVpcEndpointServiceConfigurations => {
                write!(f, "ec2:DescribeVpcEndpointServiceConfigurations")
            }
            Ec2Actions::DescribeVpcEndpointServicePermissions => {
                write!(f, "ec2:DescribeVpcEndpointServicePermissions")
            }
            Ec2Actions::DescribeVpcEndpointServices => write!(f, "ec2:DescribeVpcEndpointServices"),
            Ec2Actions::DescribeVpcEndpoints => write!(f, "ec2:DescribeVpcEndpoints"),
            Ec2Actions::DescribeVpcPeeringConnections => {
                write!(f, "ec2:DescribeVpcPeeringConnections")
            }
            Ec2Actions::DescribeVpcs => write!(f, "ec2:DescribeVpcs"),
            Ec2Actions::DescribeVpnConnections => write!(f, "ec2:DescribeVpnConnections"),
            Ec2Actions::DescribeVpnGateways => write!(f, "ec2:DescribeVpnGateways"),
            Ec2Actions::DetachClassicLinkVpc => write!(f, "ec2:DetachClassicLinkVpc"),
            Ec2Actions::DetachInternetGateway => write!(f, "ec2:DetachInternetGateway"),
            Ec2Actions::DetachNetworkInterface => write!(f, "ec2:DetachNetworkInterface"),
            Ec2Actions::DetachVerifiedAccessTrustProvider => {
                write!(f, "ec2:DetachVerifiedAccessTrustProvider")
            }
            Ec2Actions::DetachVolume => write!(f, "ec2:DetachVolume"),
            Ec2Actions::DetachVpnGateway => write!(f, "ec2:DetachVpnGateway"),
            Ec2Actions::DisableAddressTransfer => write!(f, "ec2:DisableAddressTransfer"),
            Ec2Actions::DisableAllowedImagesSettings => {
                write!(f, "ec2:DisableAllowedImagesSettings")
            }
            Ec2Actions::DisableAwsNetworkPerformanceMetricSubscription => {
                write!(f, "ec2:DisableAwsNetworkPerformanceMetricSubscription")
            }
            Ec2Actions::DisableEbsEncryptionByDefault => {
                write!(f, "ec2:DisableEbsEncryptionByDefault")
            }
            Ec2Actions::DisableFastLaunch => write!(f, "ec2:DisableFastLaunch"),
            Ec2Actions::DisableFastSnapshotRestores => write!(f, "ec2:DisableFastSnapshotRestores"),
            Ec2Actions::DisableImage => write!(f, "ec2:DisableImage"),
            Ec2Actions::DisableImageBlockPublicAccess => {
                write!(f, "ec2:DisableImageBlockPublicAccess")
            }
            Ec2Actions::DisableImageDeprecation => write!(f, "ec2:DisableImageDeprecation"),
            Ec2Actions::DisableImageDeregistrationProtection => {
                write!(f, "ec2:DisableImageDeregistrationProtection")
            }
            Ec2Actions::DisableIpamOrganizationAdminAccount => {
                write!(f, "ec2:DisableIpamOrganizationAdminAccount")
            }
            Ec2Actions::DisableRouteServerPropagation => {
                write!(f, "ec2:DisableRouteServerPropagation")
            }
            Ec2Actions::DisableSerialConsoleAccess => write!(f, "ec2:DisableSerialConsoleAccess"),
            Ec2Actions::DisableSnapshotBlockPublicAccess => {
                write!(f, "ec2:DisableSnapshotBlockPublicAccess")
            }
            Ec2Actions::DisableTransitGatewayRouteTablePropagation => {
                write!(f, "ec2:DisableTransitGatewayRouteTablePropagation")
            }
            Ec2Actions::DisableVgwRoutePropagation => write!(f, "ec2:DisableVgwRoutePropagation"),
            Ec2Actions::DisableVpcClassicLink => write!(f, "ec2:DisableVpcClassicLink"),
            Ec2Actions::DisableVpcClassicLinkDnsSupport => {
                write!(f, "ec2:DisableVpcClassicLinkDnsSupport")
            }
            Ec2Actions::DisassociateAddress => write!(f, "ec2:DisassociateAddress"),
            Ec2Actions::DisassociateCapacityReservationBillingOwner => {
                write!(f, "ec2:DisassociateCapacityReservationBillingOwner")
            }
            Ec2Actions::DisassociateClientVpnTargetNetwork => {
                write!(f, "ec2:DisassociateClientVpnTargetNetwork")
            }
            Ec2Actions::DisassociateEnclaveCertificateIamRole => {
                write!(f, "ec2:DisassociateEnclaveCertificateIamRole")
            }
            Ec2Actions::DisassociateIamInstanceProfile => {
                write!(f, "ec2:DisassociateIamInstanceProfile")
            }
            Ec2Actions::DisassociateInstanceEventWindow => {
                write!(f, "ec2:DisassociateInstanceEventWindow")
            }
            Ec2Actions::DisassociateIpamByoasn => write!(f, "ec2:DisassociateIpamByoasn"),
            Ec2Actions::DisassociateIpamResourceDiscovery => {
                write!(f, "ec2:DisassociateIpamResourceDiscovery")
            }
            Ec2Actions::DisassociateNatGatewayAddress => {
                write!(f, "ec2:DisassociateNatGatewayAddress")
            }
            Ec2Actions::DisassociateRouteServer => write!(f, "ec2:DisassociateRouteServer"),
            Ec2Actions::DisassociateRouteTable => write!(f, "ec2:DisassociateRouteTable"),
            Ec2Actions::DisassociateSecurityGroupVpc => {
                write!(f, "ec2:DisassociateSecurityGroupVpc")
            }
            Ec2Actions::DisassociateSubnetCidrBlock => write!(f, "ec2:DisassociateSubnetCidrBlock"),
            Ec2Actions::DisassociateTransitGatewayMulticastDomain => {
                write!(f, "ec2:DisassociateTransitGatewayMulticastDomain")
            }
            Ec2Actions::DisassociateTransitGatewayPolicyTable => {
                write!(f, "ec2:DisassociateTransitGatewayPolicyTable")
            }
            Ec2Actions::DisassociateTransitGatewayRouteTable => {
                write!(f, "ec2:DisassociateTransitGatewayRouteTable")
            }
            Ec2Actions::DisassociateTrunkInterface => write!(f, "ec2:DisassociateTrunkInterface"),
            Ec2Actions::DisassociateVerifiedAccessInstanceWebAcl => {
                write!(f, "ec2:DisassociateVerifiedAccessInstanceWebAcl")
            }
            Ec2Actions::DisassociateVpcCidrBlock => write!(f, "ec2:DisassociateVpcCidrBlock"),
            Ec2Actions::EnableAddressTransfer => write!(f, "ec2:EnableAddressTransfer"),
            Ec2Actions::EnableAllowedImagesSettings => write!(f, "ec2:EnableAllowedImagesSettings"),
            Ec2Actions::EnableAwsNetworkPerformanceMetricSubscription => {
                write!(f, "ec2:EnableAwsNetworkPerformanceMetricSubscription")
            }
            Ec2Actions::EnableEbsEncryptionByDefault => {
                write!(f, "ec2:EnableEbsEncryptionByDefault")
            }
            Ec2Actions::EnableFastLaunch => write!(f, "ec2:EnableFastLaunch"),
            Ec2Actions::EnableFastSnapshotRestores => write!(f, "ec2:EnableFastSnapshotRestores"),
            Ec2Actions::EnableImage => write!(f, "ec2:EnableImage"),
            Ec2Actions::EnableImageBlockPublicAccess => {
                write!(f, "ec2:EnableImageBlockPublicAccess")
            }
            Ec2Actions::EnableImageDeprecation => write!(f, "ec2:EnableImageDeprecation"),
            Ec2Actions::EnableImageDeregistrationProtection => {
                write!(f, "ec2:EnableImageDeregistrationProtection")
            }
            Ec2Actions::EnableIpamOrganizationAdminAccount => {
                write!(f, "ec2:EnableIpamOrganizationAdminAccount")
            }
            Ec2Actions::EnableReachabilityAnalyzerOrganizationSharing => {
                write!(f, "ec2:EnableReachabilityAnalyzerOrganizationSharing")
            }
            Ec2Actions::EnableRouteServerPropagation => {
                write!(f, "ec2:EnableRouteServerPropagation")
            }
            Ec2Actions::EnableSerialConsoleAccess => write!(f, "ec2:EnableSerialConsoleAccess"),
            Ec2Actions::EnableSnapshotBlockPublicAccess => {
                write!(f, "ec2:EnableSnapshotBlockPublicAccess")
            }
            Ec2Actions::EnableTransitGatewayRouteTablePropagation => {
                write!(f, "ec2:EnableTransitGatewayRouteTablePropagation")
            }
            Ec2Actions::EnableVgwRoutePropagation => write!(f, "ec2:EnableVgwRoutePropagation"),
            Ec2Actions::EnableVolumeIo => write!(f, "ec2:EnableVolumeIO"),
            Ec2Actions::EnableVpcClassicLink => write!(f, "ec2:EnableVpcClassicLink"),
            Ec2Actions::EnableVpcClassicLinkDnsSupport => {
                write!(f, "ec2:EnableVpcClassicLinkDnsSupport")
            }
            Ec2Actions::ExportClientVpnClientCertificateRevocationList => {
                write!(f, "ec2:ExportClientVpnClientCertificateRevocationList")
            }
            Ec2Actions::ExportClientVpnClientConfiguration => {
                write!(f, "ec2:ExportClientVpnClientConfiguration")
            }
            Ec2Actions::ExportImage => write!(f, "ec2:ExportImage"),
            Ec2Actions::ExportTransitGatewayRoutes => write!(f, "ec2:ExportTransitGatewayRoutes"),
            Ec2Actions::ExportVerifiedAccessInstanceClientConfiguration => {
                write!(f, "ec2:ExportVerifiedAccessInstanceClientConfiguration")
            }
            Ec2Actions::GetActiveVpnTunnelStatus => write!(f, "ec2:GetActiveVpnTunnelStatus"),
            Ec2Actions::GetAllowedImagesSettings => write!(f, "ec2:GetAllowedImagesSettings"),
            Ec2Actions::GetAssociatedEnclaveCertificateIamRoles => {
                write!(f, "ec2:GetAssociatedEnclaveCertificateIamRoles")
            }
            Ec2Actions::GetAssociatedIpv6PoolCidrs => write!(f, "ec2:GetAssociatedIpv6PoolCidrs"),
            Ec2Actions::GetAwsNetworkPerformanceData => {
                write!(f, "ec2:GetAwsNetworkPerformanceData")
            }
            Ec2Actions::GetCapacityReservationUsage => write!(f, "ec2:GetCapacityReservationUsage"),
            Ec2Actions::GetCoipPoolUsage => write!(f, "ec2:GetCoipPoolUsage"),
            Ec2Actions::GetConsoleOutput => write!(f, "ec2:GetConsoleOutput"),
            Ec2Actions::GetConsoleScreenshot => write!(f, "ec2:GetConsoleScreenshot"),
            Ec2Actions::GetDeclarativePoliciesReportSummary => {
                write!(f, "ec2:GetDeclarativePoliciesReportSummary")
            }
            Ec2Actions::GetDefaultCreditSpecification => {
                write!(f, "ec2:GetDefaultCreditSpecification")
            }
            Ec2Actions::GetEbsDefaultKmsKeyId => write!(f, "ec2:GetEbsDefaultKmsKeyId"),
            Ec2Actions::GetEbsEncryptionByDefault => write!(f, "ec2:GetEbsEncryptionByDefault"),
            Ec2Actions::GetFlowLogsIntegrationTemplate => {
                write!(f, "ec2:GetFlowLogsIntegrationTemplate")
            }
            Ec2Actions::GetGroupsForCapacityReservation => {
                write!(f, "ec2:GetGroupsForCapacityReservation")
            }
            Ec2Actions::GetHostReservationPurchasePreview => {
                write!(f, "ec2:GetHostReservationPurchasePreview")
            }
            Ec2Actions::GetImageBlockPublicAccessState => {
                write!(f, "ec2:GetImageBlockPublicAccessState")
            }
            Ec2Actions::GetInstanceMetadataDefaults => write!(f, "ec2:GetInstanceMetadataDefaults"),
            Ec2Actions::GetInstanceTpmEkPub => write!(f, "ec2:GetInstanceTpmEkPub"),
            Ec2Actions::GetInstanceTypesFromInstanceRequirements => {
                write!(f, "ec2:GetInstanceTypesFromInstanceRequirements")
            }
            Ec2Actions::GetInstanceUefiData => write!(f, "ec2:GetInstanceUefiData"),
            Ec2Actions::GetIpamAddressHistory => write!(f, "ec2:GetIpamAddressHistory"),
            Ec2Actions::GetIpamDiscoveredAccounts => write!(f, "ec2:GetIpamDiscoveredAccounts"),
            Ec2Actions::GetIpamDiscoveredPublicAddresses => {
                write!(f, "ec2:GetIpamDiscoveredPublicAddresses")
            }
            Ec2Actions::GetIpamDiscoveredResourceCidrs => {
                write!(f, "ec2:GetIpamDiscoveredResourceCidrs")
            }
            Ec2Actions::GetIpamPoolAllocations => write!(f, "ec2:GetIpamPoolAllocations"),
            Ec2Actions::GetIpamPoolCidrs => write!(f, "ec2:GetIpamPoolCidrs"),
            Ec2Actions::GetIpamResourceCidrs => write!(f, "ec2:GetIpamResourceCidrs"),
            Ec2Actions::GetLaunchTemplateData => write!(f, "ec2:GetLaunchTemplateData"),
            Ec2Actions::GetManagedPrefixListAssociations => {
                write!(f, "ec2:GetManagedPrefixListAssociations")
            }
            Ec2Actions::GetManagedPrefixListEntries => write!(f, "ec2:GetManagedPrefixListEntries"),
            Ec2Actions::GetNetworkInsightsAccessScopeAnalysisFindings => {
                write!(f, "ec2:GetNetworkInsightsAccessScopeAnalysisFindings")
            }
            Ec2Actions::GetNetworkInsightsAccessScopeContent => {
                write!(f, "ec2:GetNetworkInsightsAccessScopeContent")
            }
            Ec2Actions::GetPasswordData => write!(f, "ec2:GetPasswordData"),
            Ec2Actions::GetReservedInstancesExchangeQuote => {
                write!(f, "ec2:GetReservedInstancesExchangeQuote")
            }
            Ec2Actions::GetResourcePolicy => write!(f, "ec2:GetResourcePolicy"),
            Ec2Actions::GetRouteServerAssociations => write!(f, "ec2:GetRouteServerAssociations"),
            Ec2Actions::GetRouteServerPropagations => write!(f, "ec2:GetRouteServerPropagations"),
            Ec2Actions::GetRouteServerRoutingDatabase => {
                write!(f, "ec2:GetRouteServerRoutingDatabase")
            }
            Ec2Actions::GetSecurityGroupsForVpc => write!(f, "ec2:GetSecurityGroupsForVpc"),
            Ec2Actions::GetSerialConsoleAccessStatus => {
                write!(f, "ec2:GetSerialConsoleAccessStatus")
            }
            Ec2Actions::GetSnapshotBlockPublicAccessState => {
                write!(f, "ec2:GetSnapshotBlockPublicAccessState")
            }
            Ec2Actions::GetSpotPlacementScores => write!(f, "ec2:GetSpotPlacementScores"),
            Ec2Actions::GetSubnetCidrReservations => write!(f, "ec2:GetSubnetCidrReservations"),
            Ec2Actions::GetTransitGatewayAttachmentPropagations => {
                write!(f, "ec2:GetTransitGatewayAttachmentPropagations")
            }
            Ec2Actions::GetTransitGatewayMulticastDomainAssociations => {
                write!(f, "ec2:GetTransitGatewayMulticastDomainAssociations")
            }
            Ec2Actions::GetTransitGatewayPolicyTableAssociations => {
                write!(f, "ec2:GetTransitGatewayPolicyTableAssociations")
            }
            Ec2Actions::GetTransitGatewayPolicyTableEntries => {
                write!(f, "ec2:GetTransitGatewayPolicyTableEntries")
            }
            Ec2Actions::GetTransitGatewayPrefixListReferences => {
                write!(f, "ec2:GetTransitGatewayPrefixListReferences")
            }
            Ec2Actions::GetTransitGatewayRouteTableAssociations => {
                write!(f, "ec2:GetTransitGatewayRouteTableAssociations")
            }
            Ec2Actions::GetTransitGatewayRouteTablePropagations => {
                write!(f, "ec2:GetTransitGatewayRouteTablePropagations")
            }
            Ec2Actions::GetVerifiedAccessEndpointPolicy => {
                write!(f, "ec2:GetVerifiedAccessEndpointPolicy")
            }
            Ec2Actions::GetVerifiedAccessEndpointTargets => {
                write!(f, "ec2:GetVerifiedAccessEndpointTargets")
            }
            Ec2Actions::GetVerifiedAccessGroupPolicy => {
                write!(f, "ec2:GetVerifiedAccessGroupPolicy")
            }
            Ec2Actions::GetVerifiedAccessInstanceWebAcl => {
                write!(f, "ec2:GetVerifiedAccessInstanceWebAcl")
            }
            Ec2Actions::GetVpnConnectionDeviceSampleConfiguration => {
                write!(f, "ec2:GetVpnConnectionDeviceSampleConfiguration")
            }
            Ec2Actions::GetVpnConnectionDeviceTypes => write!(f, "ec2:GetVpnConnectionDeviceTypes"),
            Ec2Actions::GetVpnTunnelReplacementStatus => {
                write!(f, "ec2:GetVpnTunnelReplacementStatus")
            }
            Ec2Actions::ImportByoipCidrToIpam => write!(f, "ec2:ImportByoipCidrToIpam"),
            Ec2Actions::ImportClientVpnClientCertificateRevocationList => {
                write!(f, "ec2:ImportClientVpnClientCertificateRevocationList")
            }
            Ec2Actions::ImportImage => write!(f, "ec2:ImportImage"),
            Ec2Actions::ImportInstance => write!(f, "ec2:ImportInstance"),
            Ec2Actions::ImportKeyPair => write!(f, "ec2:ImportKeyPair"),
            Ec2Actions::ImportSnapshot => write!(f, "ec2:ImportSnapshot"),
            Ec2Actions::ImportVolume => write!(f, "ec2:ImportVolume"),
            Ec2Actions::InjectApiError => write!(f, "ec2:InjectApiError"),
            Ec2Actions::ListImagesInRecycleBin => write!(f, "ec2:ListImagesInRecycleBin"),
            Ec2Actions::ListSnapshotsInRecycleBin => write!(f, "ec2:ListSnapshotsInRecycleBin"),
            Ec2Actions::LockSnapshot => write!(f, "ec2:LockSnapshot"),
            Ec2Actions::ModifyAddressAttribute => write!(f, "ec2:ModifyAddressAttribute"),
            Ec2Actions::ModifyAvailabilityZoneGroup => write!(f, "ec2:ModifyAvailabilityZoneGroup"),
            Ec2Actions::ModifyCapacityReservation => write!(f, "ec2:ModifyCapacityReservation"),
            Ec2Actions::ModifyCapacityReservationFleet => {
                write!(f, "ec2:ModifyCapacityReservationFleet")
            }
            Ec2Actions::ModifyClientVpnEndpoint => write!(f, "ec2:ModifyClientVpnEndpoint"),
            Ec2Actions::ModifyDefaultCreditSpecification => {
                write!(f, "ec2:ModifyDefaultCreditSpecification")
            }
            Ec2Actions::ModifyEbsDefaultKmsKeyId => write!(f, "ec2:ModifyEbsDefaultKmsKeyId"),
            Ec2Actions::ModifyFleet => write!(f, "ec2:ModifyFleet"),
            Ec2Actions::ModifyFpgaImageAttribute => write!(f, "ec2:ModifyFpgaImageAttribute"),
            Ec2Actions::ModifyHosts => write!(f, "ec2:ModifyHosts"),
            Ec2Actions::ModifyIdFormat => write!(f, "ec2:ModifyIdFormat"),
            Ec2Actions::ModifyIdentityIdFormat => write!(f, "ec2:ModifyIdentityIdFormat"),
            Ec2Actions::ModifyImageAttribute => write!(f, "ec2:ModifyImageAttribute"),
            Ec2Actions::ModifyInstanceAttribute => write!(f, "ec2:ModifyInstanceAttribute"),
            Ec2Actions::ModifyInstanceCapacityReservationAttributes => {
                write!(f, "ec2:ModifyInstanceCapacityReservationAttributes")
            }
            Ec2Actions::ModifyInstanceCpuOptions => write!(f, "ec2:ModifyInstanceCpuOptions"),
            Ec2Actions::ModifyInstanceCreditSpecification => {
                write!(f, "ec2:ModifyInstanceCreditSpecification")
            }
            Ec2Actions::ModifyInstanceEventStartTime => {
                write!(f, "ec2:ModifyInstanceEventStartTime")
            }
            Ec2Actions::ModifyInstanceEventWindow => write!(f, "ec2:ModifyInstanceEventWindow"),
            Ec2Actions::ModifyInstanceMaintenanceOptions => {
                write!(f, "ec2:ModifyInstanceMaintenanceOptions")
            }
            Ec2Actions::ModifyInstanceMetadataDefaults => {
                write!(f, "ec2:ModifyInstanceMetadataDefaults")
            }
            Ec2Actions::ModifyInstanceMetadataOptions => {
                write!(f, "ec2:ModifyInstanceMetadataOptions")
            }
            Ec2Actions::ModifyInstanceNetworkPerformanceOptions => {
                write!(f, "ec2:ModifyInstanceNetworkPerformanceOptions")
            }
            Ec2Actions::ModifyInstancePlacement => write!(f, "ec2:ModifyInstancePlacement"),
            Ec2Actions::ModifyIpam => write!(f, "ec2:ModifyIpam"),
            Ec2Actions::ModifyIpamPool => write!(f, "ec2:ModifyIpamPool"),
            Ec2Actions::ModifyIpamResourceCidr => write!(f, "ec2:ModifyIpamResourceCidr"),
            Ec2Actions::ModifyIpamResourceDiscovery => write!(f, "ec2:ModifyIpamResourceDiscovery"),
            Ec2Actions::ModifyIpamScope => write!(f, "ec2:ModifyIpamScope"),
            Ec2Actions::ModifyLaunchTemplate => write!(f, "ec2:ModifyLaunchTemplate"),
            Ec2Actions::ModifyLocalGatewayRoute => write!(f, "ec2:ModifyLocalGatewayRoute"),
            Ec2Actions::ModifyManagedPrefixList => write!(f, "ec2:ModifyManagedPrefixList"),
            Ec2Actions::ModifyNetworkInterfaceAttribute => {
                write!(f, "ec2:ModifyNetworkInterfaceAttribute")
            }
            Ec2Actions::ModifyPrivateDnsNameOptions => write!(f, "ec2:ModifyPrivateDnsNameOptions"),
            Ec2Actions::ModifyPublicIpDnsNameOptions => {
                write!(f, "ec2:ModifyPublicIpDnsNameOptions")
            }
            Ec2Actions::ModifyReservedInstances => write!(f, "ec2:ModifyReservedInstances"),
            Ec2Actions::ModifyRouteServer => write!(f, "ec2:ModifyRouteServer"),
            Ec2Actions::ModifySecurityGroupRules => write!(f, "ec2:ModifySecurityGroupRules"),
            Ec2Actions::ModifySnapshotAttribute => write!(f, "ec2:ModifySnapshotAttribute"),
            Ec2Actions::ModifySnapshotTier => write!(f, "ec2:ModifySnapshotTier"),
            Ec2Actions::ModifySpotFleetRequest => write!(f, "ec2:ModifySpotFleetRequest"),
            Ec2Actions::ModifySubnetAttribute => write!(f, "ec2:ModifySubnetAttribute"),
            Ec2Actions::ModifyTrafficMirrorFilterNetworkServices => {
                write!(f, "ec2:ModifyTrafficMirrorFilterNetworkServices")
            }
            Ec2Actions::ModifyTrafficMirrorFilterRule => {
                write!(f, "ec2:ModifyTrafficMirrorFilterRule")
            }
            Ec2Actions::ModifyTrafficMirrorSession => write!(f, "ec2:ModifyTrafficMirrorSession"),
            Ec2Actions::ModifyTransitGateway => write!(f, "ec2:ModifyTransitGateway"),
            Ec2Actions::ModifyTransitGatewayPrefixListReference => {
                write!(f, "ec2:ModifyTransitGatewayPrefixListReference")
            }
            Ec2Actions::ModifyTransitGatewayVpcAttachment => {
                write!(f, "ec2:ModifyTransitGatewayVpcAttachment")
            }
            Ec2Actions::ModifyVerifiedAccessEndpoint => {
                write!(f, "ec2:ModifyVerifiedAccessEndpoint")
            }
            Ec2Actions::ModifyVerifiedAccessEndpointPolicy => {
                write!(f, "ec2:ModifyVerifiedAccessEndpointPolicy")
            }
            Ec2Actions::ModifyVerifiedAccessGroup => write!(f, "ec2:ModifyVerifiedAccessGroup"),
            Ec2Actions::ModifyVerifiedAccessGroupPolicy => {
                write!(f, "ec2:ModifyVerifiedAccessGroupPolicy")
            }
            Ec2Actions::ModifyVerifiedAccessInstance => {
                write!(f, "ec2:ModifyVerifiedAccessInstance")
            }
            Ec2Actions::ModifyVerifiedAccessInstanceLoggingConfiguration => {
                write!(f, "ec2:ModifyVerifiedAccessInstanceLoggingConfiguration")
            }
            Ec2Actions::ModifyVerifiedAccessTrustProvider => {
                write!(f, "ec2:ModifyVerifiedAccessTrustProvider")
            }
            Ec2Actions::ModifyVolume => write!(f, "ec2:ModifyVolume"),
            Ec2Actions::ModifyVolumeAttribute => write!(f, "ec2:ModifyVolumeAttribute"),
            Ec2Actions::ModifyVpcAttribute => write!(f, "ec2:ModifyVpcAttribute"),
            Ec2Actions::ModifyVpcBlockPublicAccessExclusion => {
                write!(f, "ec2:ModifyVpcBlockPublicAccessExclusion")
            }
            Ec2Actions::ModifyVpcBlockPublicAccessOptions => {
                write!(f, "ec2:ModifyVpcBlockPublicAccessOptions")
            }
            Ec2Actions::ModifyVpcEndpoint => write!(f, "ec2:ModifyVpcEndpoint"),
            Ec2Actions::ModifyVpcEndpointConnectionNotification => {
                write!(f, "ec2:ModifyVpcEndpointConnectionNotification")
            }
            Ec2Actions::ModifyVpcEndpointServiceConfiguration => {
                write!(f, "ec2:ModifyVpcEndpointServiceConfiguration")
            }
            Ec2Actions::ModifyVpcEndpointServicePayerResponsibility => {
                write!(f, "ec2:ModifyVpcEndpointServicePayerResponsibility")
            }
            Ec2Actions::ModifyVpcEndpointServicePermissions => {
                write!(f, "ec2:ModifyVpcEndpointServicePermissions")
            }
            Ec2Actions::ModifyVpcPeeringConnectionOptions => {
                write!(f, "ec2:ModifyVpcPeeringConnectionOptions")
            }
            Ec2Actions::ModifyVpcTenancy => write!(f, "ec2:ModifyVpcTenancy"),
            Ec2Actions::ModifyVpnConnection => write!(f, "ec2:ModifyVpnConnection"),
            Ec2Actions::ModifyVpnConnectionOptions => write!(f, "ec2:ModifyVpnConnectionOptions"),
            Ec2Actions::ModifyVpnTunnelCertificate => write!(f, "ec2:ModifyVpnTunnelCertificate"),
            Ec2Actions::ModifyVpnTunnelOptions => write!(f, "ec2:ModifyVpnTunnelOptions"),
            Ec2Actions::MonitorInstances => write!(f, "ec2:MonitorInstances"),
            Ec2Actions::MoveAddressToVpc => write!(f, "ec2:MoveAddressToVpc"),
            Ec2Actions::MoveByoipCidrToIpam => write!(f, "ec2:MoveByoipCidrToIpam"),
            Ec2Actions::MoveCapacityReservationInstances => {
                write!(f, "ec2:MoveCapacityReservationInstances")
            }
            Ec2Actions::PauseVolumeIo => write!(f, "ec2:PauseVolumeIO"),
            Ec2Actions::ProvisionByoipCidr => write!(f, "ec2:ProvisionByoipCidr"),
            Ec2Actions::ProvisionIpamByoasn => write!(f, "ec2:ProvisionIpamByoasn"),
            Ec2Actions::ProvisionIpamPoolCidr => write!(f, "ec2:ProvisionIpamPoolCidr"),
            Ec2Actions::ProvisionPublicIpv4PoolCidr => write!(f, "ec2:ProvisionPublicIpv4PoolCidr"),
            Ec2Actions::PurchaseCapacityBlock => write!(f, "ec2:PurchaseCapacityBlock"),
            Ec2Actions::PurchaseCapacityBlockExtension => {
                write!(f, "ec2:PurchaseCapacityBlockExtension")
            }
            Ec2Actions::PurchaseHostReservation => write!(f, "ec2:PurchaseHostReservation"),
            Ec2Actions::PurchaseReservedInstancesOffering => {
                write!(f, "ec2:PurchaseReservedInstancesOffering")
            }
            Ec2Actions::PurchaseScheduledInstances => write!(f, "ec2:PurchaseScheduledInstances"),
            Ec2Actions::PutResourcePolicy => write!(f, "ec2:PutResourcePolicy"),
            Ec2Actions::RebootInstances => write!(f, "ec2:RebootInstances"),
            Ec2Actions::RegisterImage => write!(f, "ec2:RegisterImage"),
            Ec2Actions::RegisterInstanceEventNotificationAttributes => {
                write!(f, "ec2:RegisterInstanceEventNotificationAttributes")
            }
            Ec2Actions::RegisterTransitGatewayMulticastGroupMembers => {
                write!(f, "ec2:RegisterTransitGatewayMulticastGroupMembers")
            }
            Ec2Actions::RegisterTransitGatewayMulticastGroupSources => {
                write!(f, "ec2:RegisterTransitGatewayMulticastGroupSources")
            }
            Ec2Actions::RejectCapacityReservationBillingOwnership => {
                write!(f, "ec2:RejectCapacityReservationBillingOwnership")
            }
            Ec2Actions::RejectTransitGatewayMulticastDomainAssociations => {
                write!(f, "ec2:RejectTransitGatewayMulticastDomainAssociations")
            }
            Ec2Actions::RejectTransitGatewayPeeringAttachment => {
                write!(f, "ec2:RejectTransitGatewayPeeringAttachment")
            }
            Ec2Actions::RejectTransitGatewayVpcAttachment => {
                write!(f, "ec2:RejectTransitGatewayVpcAttachment")
            }
            Ec2Actions::RejectVpcEndpointConnections => {
                write!(f, "ec2:RejectVpcEndpointConnections")
            }
            Ec2Actions::RejectVpcPeeringConnection => write!(f, "ec2:RejectVpcPeeringConnection"),
            Ec2Actions::ReleaseAddress => write!(f, "ec2:ReleaseAddress"),
            Ec2Actions::ReleaseHosts => write!(f, "ec2:ReleaseHosts"),
            Ec2Actions::ReleaseIpamPoolAllocation => write!(f, "ec2:ReleaseIpamPoolAllocation"),
            Ec2Actions::ReplaceIamInstanceProfileAssociation => {
                write!(f, "ec2:ReplaceIamInstanceProfileAssociation")
            }
            Ec2Actions::ReplaceImageCriteriaInAllowedImagesSettings => {
                write!(f, "ec2:ReplaceImageCriteriaInAllowedImagesSettings")
            }
            Ec2Actions::ReplaceNetworkAclAssociation => {
                write!(f, "ec2:ReplaceNetworkAclAssociation")
            }
            Ec2Actions::ReplaceNetworkAclEntry => write!(f, "ec2:ReplaceNetworkAclEntry"),
            Ec2Actions::ReplaceRoute => write!(f, "ec2:ReplaceRoute"),
            Ec2Actions::ReplaceRouteTableAssociation => {
                write!(f, "ec2:ReplaceRouteTableAssociation")
            }
            Ec2Actions::ReplaceTransitGatewayRoute => write!(f, "ec2:ReplaceTransitGatewayRoute"),
            Ec2Actions::ReplaceVpnTunnel => write!(f, "ec2:ReplaceVpnTunnel"),
            Ec2Actions::ReportInstanceStatus => write!(f, "ec2:ReportInstanceStatus"),
            Ec2Actions::RequestSpotFleet => write!(f, "ec2:RequestSpotFleet"),
            Ec2Actions::RequestSpotInstances => write!(f, "ec2:RequestSpotInstances"),
            Ec2Actions::ResetAddressAttribute => write!(f, "ec2:ResetAddressAttribute"),
            Ec2Actions::ResetEbsDefaultKmsKeyId => write!(f, "ec2:ResetEbsDefaultKmsKeyId"),
            Ec2Actions::ResetFpgaImageAttribute => write!(f, "ec2:ResetFpgaImageAttribute"),
            Ec2Actions::ResetImageAttribute => write!(f, "ec2:ResetImageAttribute"),
            Ec2Actions::ResetInstanceAttribute => write!(f, "ec2:ResetInstanceAttribute"),
            Ec2Actions::ResetNetworkInterfaceAttribute => {
                write!(f, "ec2:ResetNetworkInterfaceAttribute")
            }
            Ec2Actions::ResetSnapshotAttribute => write!(f, "ec2:ResetSnapshotAttribute"),
            Ec2Actions::RestoreAddressToClassic => write!(f, "ec2:RestoreAddressToClassic"),
            Ec2Actions::RestoreImageFromRecycleBin => write!(f, "ec2:RestoreImageFromRecycleBin"),
            Ec2Actions::RestoreManagedPrefixListVersion => {
                write!(f, "ec2:RestoreManagedPrefixListVersion")
            }
            Ec2Actions::RestoreSnapshotFromRecycleBin => {
                write!(f, "ec2:RestoreSnapshotFromRecycleBin")
            }
            Ec2Actions::RestoreSnapshotTier => write!(f, "ec2:RestoreSnapshotTier"),
            Ec2Actions::RevokeClientVpnIngress => write!(f, "ec2:RevokeClientVpnIngress"),
            Ec2Actions::RevokeSecurityGroupEgress => write!(f, "ec2:RevokeSecurityGroupEgress"),
            Ec2Actions::RevokeSecurityGroupIngress => write!(f, "ec2:RevokeSecurityGroupIngress"),
            Ec2Actions::RunInstances => write!(f, "ec2:RunInstances"),
            Ec2Actions::RunScheduledInstances => write!(f, "ec2:RunScheduledInstances"),
            Ec2Actions::SearchLocalGatewayRoutes => write!(f, "ec2:SearchLocalGatewayRoutes"),
            Ec2Actions::SearchTransitGatewayMulticastGroups => {
                write!(f, "ec2:SearchTransitGatewayMulticastGroups")
            }
            Ec2Actions::SearchTransitGatewayRoutes => write!(f, "ec2:SearchTransitGatewayRoutes"),
            Ec2Actions::SendDiagnosticInterrupt => write!(f, "ec2:SendDiagnosticInterrupt"),
            Ec2Actions::SendSpotInstanceInterruptions => {
                write!(f, "ec2:SendSpotInstanceInterruptions")
            }
            Ec2Actions::StartDeclarativePoliciesReport => {
                write!(f, "ec2:StartDeclarativePoliciesReport")
            }
            Ec2Actions::StartInstances => write!(f, "ec2:StartInstances"),
            Ec2Actions::StartNetworkInsightsAccessScopeAnalysis => {
                write!(f, "ec2:StartNetworkInsightsAccessScopeAnalysis")
            }
            Ec2Actions::StartNetworkInsightsAnalysis => {
                write!(f, "ec2:StartNetworkInsightsAnalysis")
            }
            Ec2Actions::StartVpcEndpointServicePrivateDnsVerification => {
                write!(f, "ec2:StartVpcEndpointServicePrivateDnsVerification")
            }
            Ec2Actions::StopInstances => write!(f, "ec2:StopInstances"),
            Ec2Actions::TerminateClientVpnConnections => {
                write!(f, "ec2:TerminateClientVpnConnections")
            }
            Ec2Actions::TerminateInstances => write!(f, "ec2:TerminateInstances"),
            Ec2Actions::UnassignIpv6Addresses => write!(f, "ec2:UnassignIpv6Addresses"),
            Ec2Actions::UnassignPrivateIpAddresses => write!(f, "ec2:UnassignPrivateIpAddresses"),
            Ec2Actions::UnassignPrivateNatGatewayAddress => {
                write!(f, "ec2:UnassignPrivateNatGatewayAddress")
            }
            Ec2Actions::UnlockSnapshot => write!(f, "ec2:UnlockSnapshot"),
            Ec2Actions::UnmonitorInstances => write!(f, "ec2:UnmonitorInstances"),
            Ec2Actions::UpdateSecurityGroupRuleDescriptionsEgress => {
                write!(f, "ec2:UpdateSecurityGroupRuleDescriptionsEgress")
            }
            Ec2Actions::UpdateSecurityGroupRuleDescriptionsIngress => {
                write!(f, "ec2:UpdateSecurityGroupRuleDescriptionsIngress")
            }
            Ec2Actions::WithdrawByoipCidr => write!(f, "ec2:WithdrawByoipCidr"),
        }
    }
}
