// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum AppstreamActions {
    AssociateAppBlockBuilderAppBlock,
    AssociateApplicationFleet,
    AssociateApplicationToEntitlement,
    AssociateFleet,
    BatchAssociateUserStack,
    BatchDisassociateUserStack,
    CopyImage,
    CreateAppBlock,
    CreateAppBlockBuilder,
    CreateAppBlockBuilderStreamingUrl,
    CreateApplication,
    CreateDirectoryConfig,
    CreateEntitlement,
    CreateFleet,
    CreateImageBuilder,
    CreateImageBuilderStreamingUrl,
    CreateStack,
    CreateStreamingUrl,
    CreateThemeForStack,
    CreateUpdatedImage,
    CreateUsageReportSubscription,
    CreateUser,
    DeleteAppBlock,
    DeleteAppBlockBuilder,
    DeleteApplication,
    DeleteDirectoryConfig,
    DeleteEntitlement,
    DeleteFleet,
    DeleteImage,
    DeleteImageBuilder,
    DeleteImagePermissions,
    DeleteStack,
    DeleteThemeForStack,
    DeleteUsageReportSubscription,
    DeleteUser,
    DescribeAppBlockBuilderAppBlockAssociations,
    DescribeAppBlockBuilders,
    DescribeAppBlocks,
    DescribeApplicationFleetAssociations,
    DescribeApplications,
    DescribeDirectoryConfigs,
    DescribeEntitlements,
    DescribeFleets,
    DescribeImageBuilders,
    DescribeImagePermissions,
    DescribeImages,
    DescribeSessions,
    DescribeStacks,
    DescribeThemeForStack,
    DescribeUsageReportSubscriptions,
    DescribeUserStackAssociations,
    DescribeUsers,
    DisableUser,
    DisassociateAppBlockBuilderAppBlock,
    DisassociateApplicationFleet,
    DisassociateApplicationFromEntitlement,
    DisassociateFleet,
    EnableUser,
    ExpireSession,
    ListAssociatedFleets,
    ListAssociatedStacks,
    ListEntitledApplications,
    ListTagsForResource,
    StartAppBlockBuilder,
    StartFleet,
    StartImageBuilder,
    StopAppBlockBuilder,
    StopFleet,
    StopImageBuilder,
    Stream,
    TagResource,
    UntagResource,
    UpdateAppBlockBuilder,
    UpdateApplication,
    UpdateDirectoryConfig,
    UpdateEntitlement,
    UpdateFleet,
    UpdateImagePermissions,
    UpdateStack,
    UpdateThemeForStack,
}
impl std::fmt::Display for AppstreamActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppstreamActions::AssociateAppBlockBuilderAppBlock => {
                write!(f, "appstream:AssociateAppBlockBuilderAppBlock")
            }
            AppstreamActions::AssociateApplicationFleet => {
                write!(f, "appstream:AssociateApplicationFleet")
            }
            AppstreamActions::AssociateApplicationToEntitlement => {
                write!(f, "appstream:AssociateApplicationToEntitlement")
            }
            AppstreamActions::AssociateFleet => write!(f, "appstream:AssociateFleet"),
            AppstreamActions::BatchAssociateUserStack => {
                write!(f, "appstream:BatchAssociateUserStack")
            }
            AppstreamActions::BatchDisassociateUserStack => {
                write!(f, "appstream:BatchDisassociateUserStack")
            }
            AppstreamActions::CopyImage => write!(f, "appstream:CopyImage"),
            AppstreamActions::CreateAppBlock => write!(f, "appstream:CreateAppBlock"),
            AppstreamActions::CreateAppBlockBuilder => write!(f, "appstream:CreateAppBlockBuilder"),
            AppstreamActions::CreateAppBlockBuilderStreamingUrl => {
                write!(f, "appstream:CreateAppBlockBuilderStreamingURL")
            }
            AppstreamActions::CreateApplication => write!(f, "appstream:CreateApplication"),
            AppstreamActions::CreateDirectoryConfig => write!(f, "appstream:CreateDirectoryConfig"),
            AppstreamActions::CreateEntitlement => write!(f, "appstream:CreateEntitlement"),
            AppstreamActions::CreateFleet => write!(f, "appstream:CreateFleet"),
            AppstreamActions::CreateImageBuilder => write!(f, "appstream:CreateImageBuilder"),
            AppstreamActions::CreateImageBuilderStreamingUrl => {
                write!(f, "appstream:CreateImageBuilderStreamingURL")
            }
            AppstreamActions::CreateStack => write!(f, "appstream:CreateStack"),
            AppstreamActions::CreateStreamingUrl => write!(f, "appstream:CreateStreamingURL"),
            AppstreamActions::CreateThemeForStack => write!(f, "appstream:CreateThemeForStack"),
            AppstreamActions::CreateUpdatedImage => write!(f, "appstream:CreateUpdatedImage"),
            AppstreamActions::CreateUsageReportSubscription => {
                write!(f, "appstream:CreateUsageReportSubscription")
            }
            AppstreamActions::CreateUser => write!(f, "appstream:CreateUser"),
            AppstreamActions::DeleteAppBlock => write!(f, "appstream:DeleteAppBlock"),
            AppstreamActions::DeleteAppBlockBuilder => write!(f, "appstream:DeleteAppBlockBuilder"),
            AppstreamActions::DeleteApplication => write!(f, "appstream:DeleteApplication"),
            AppstreamActions::DeleteDirectoryConfig => write!(f, "appstream:DeleteDirectoryConfig"),
            AppstreamActions::DeleteEntitlement => write!(f, "appstream:DeleteEntitlement"),
            AppstreamActions::DeleteFleet => write!(f, "appstream:DeleteFleet"),
            AppstreamActions::DeleteImage => write!(f, "appstream:DeleteImage"),
            AppstreamActions::DeleteImageBuilder => write!(f, "appstream:DeleteImageBuilder"),
            AppstreamActions::DeleteImagePermissions => {
                write!(f, "appstream:DeleteImagePermissions")
            }
            AppstreamActions::DeleteStack => write!(f, "appstream:DeleteStack"),
            AppstreamActions::DeleteThemeForStack => write!(f, "appstream:DeleteThemeForStack"),
            AppstreamActions::DeleteUsageReportSubscription => {
                write!(f, "appstream:DeleteUsageReportSubscription")
            }
            AppstreamActions::DeleteUser => write!(f, "appstream:DeleteUser"),
            AppstreamActions::DescribeAppBlockBuilderAppBlockAssociations => {
                write!(f, "appstream:DescribeAppBlockBuilderAppBlockAssociations")
            }
            AppstreamActions::DescribeAppBlockBuilders => {
                write!(f, "appstream:DescribeAppBlockBuilders")
            }
            AppstreamActions::DescribeAppBlocks => write!(f, "appstream:DescribeAppBlocks"),
            AppstreamActions::DescribeApplicationFleetAssociations => {
                write!(f, "appstream:DescribeApplicationFleetAssociations")
            }
            AppstreamActions::DescribeApplications => write!(f, "appstream:DescribeApplications"),
            AppstreamActions::DescribeDirectoryConfigs => {
                write!(f, "appstream:DescribeDirectoryConfigs")
            }
            AppstreamActions::DescribeEntitlements => write!(f, "appstream:DescribeEntitlements"),
            AppstreamActions::DescribeFleets => write!(f, "appstream:DescribeFleets"),
            AppstreamActions::DescribeImageBuilders => write!(f, "appstream:DescribeImageBuilders"),
            AppstreamActions::DescribeImagePermissions => {
                write!(f, "appstream:DescribeImagePermissions")
            }
            AppstreamActions::DescribeImages => write!(f, "appstream:DescribeImages"),
            AppstreamActions::DescribeSessions => write!(f, "appstream:DescribeSessions"),
            AppstreamActions::DescribeStacks => write!(f, "appstream:DescribeStacks"),
            AppstreamActions::DescribeThemeForStack => write!(f, "appstream:DescribeThemeForStack"),
            AppstreamActions::DescribeUsageReportSubscriptions => {
                write!(f, "appstream:DescribeUsageReportSubscriptions")
            }
            AppstreamActions::DescribeUserStackAssociations => {
                write!(f, "appstream:DescribeUserStackAssociations")
            }
            AppstreamActions::DescribeUsers => write!(f, "appstream:DescribeUsers"),
            AppstreamActions::DisableUser => write!(f, "appstream:DisableUser"),
            AppstreamActions::DisassociateAppBlockBuilderAppBlock => {
                write!(f, "appstream:DisassociateAppBlockBuilderAppBlock")
            }
            AppstreamActions::DisassociateApplicationFleet => {
                write!(f, "appstream:DisassociateApplicationFleet")
            }
            AppstreamActions::DisassociateApplicationFromEntitlement => {
                write!(f, "appstream:DisassociateApplicationFromEntitlement")
            }
            AppstreamActions::DisassociateFleet => write!(f, "appstream:DisassociateFleet"),
            AppstreamActions::EnableUser => write!(f, "appstream:EnableUser"),
            AppstreamActions::ExpireSession => write!(f, "appstream:ExpireSession"),
            AppstreamActions::ListAssociatedFleets => write!(f, "appstream:ListAssociatedFleets"),
            AppstreamActions::ListAssociatedStacks => write!(f, "appstream:ListAssociatedStacks"),
            AppstreamActions::ListEntitledApplications => {
                write!(f, "appstream:ListEntitledApplications")
            }
            AppstreamActions::ListTagsForResource => write!(f, "appstream:ListTagsForResource"),
            AppstreamActions::StartAppBlockBuilder => write!(f, "appstream:StartAppBlockBuilder"),
            AppstreamActions::StartFleet => write!(f, "appstream:StartFleet"),
            AppstreamActions::StartImageBuilder => write!(f, "appstream:StartImageBuilder"),
            AppstreamActions::StopAppBlockBuilder => write!(f, "appstream:StopAppBlockBuilder"),
            AppstreamActions::StopFleet => write!(f, "appstream:StopFleet"),
            AppstreamActions::StopImageBuilder => write!(f, "appstream:StopImageBuilder"),
            AppstreamActions::Stream => write!(f, "appstream:Stream"),
            AppstreamActions::TagResource => write!(f, "appstream:TagResource"),
            AppstreamActions::UntagResource => write!(f, "appstream:UntagResource"),
            AppstreamActions::UpdateAppBlockBuilder => write!(f, "appstream:UpdateAppBlockBuilder"),
            AppstreamActions::UpdateApplication => write!(f, "appstream:UpdateApplication"),
            AppstreamActions::UpdateDirectoryConfig => write!(f, "appstream:UpdateDirectoryConfig"),
            AppstreamActions::UpdateEntitlement => write!(f, "appstream:UpdateEntitlement"),
            AppstreamActions::UpdateFleet => write!(f, "appstream:UpdateFleet"),
            AppstreamActions::UpdateImagePermissions => {
                write!(f, "appstream:UpdateImagePermissions")
            }
            AppstreamActions::UpdateStack => write!(f, "appstream:UpdateStack"),
            AppstreamActions::UpdateThemeForStack => write!(f, "appstream:UpdateThemeForStack"),
        }
    }
}
