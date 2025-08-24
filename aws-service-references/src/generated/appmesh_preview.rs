// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum AppmeshPreviewActions {
    CreateGatewayRoute,
    CreateMesh,
    CreateRoute,
    CreateVirtualGateway,
    CreateVirtualNode,
    CreateVirtualRouter,
    CreateVirtualService,
    DeleteGatewayRoute,
    DeleteMesh,
    DeleteMeshPolicy,
    DeleteRoute,
    DeleteVirtualGateway,
    DeleteVirtualNode,
    DeleteVirtualRouter,
    DeleteVirtualService,
    DescribeGatewayRoute,
    DescribeMesh,
    DescribeRoute,
    DescribeVirtualGateway,
    DescribeVirtualNode,
    DescribeVirtualRouter,
    DescribeVirtualService,
    GetMeshPolicy,
    ListGatewayRoutes,
    ListMeshes,
    ListRoutes,
    ListVirtualGateways,
    ListVirtualNodes,
    ListVirtualRouters,
    ListVirtualServices,
    PutMeshPolicy,
    StreamAggregatedResources,
    UpdateGatewayRoute,
    UpdateMesh,
    UpdateRoute,
    UpdateVirtualGateway,
    UpdateVirtualNode,
    UpdateVirtualRouter,
    UpdateVirtualService,
}
impl std::fmt::Display for AppmeshPreviewActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppmeshPreviewActions::CreateGatewayRoute => {
                write!(f, "appmesh-preview:CreateGatewayRoute")
            }
            AppmeshPreviewActions::CreateMesh => write!(f, "appmesh-preview:CreateMesh"),
            AppmeshPreviewActions::CreateRoute => write!(f, "appmesh-preview:CreateRoute"),
            AppmeshPreviewActions::CreateVirtualGateway => {
                write!(f, "appmesh-preview:CreateVirtualGateway")
            }
            AppmeshPreviewActions::CreateVirtualNode => {
                write!(f, "appmesh-preview:CreateVirtualNode")
            }
            AppmeshPreviewActions::CreateVirtualRouter => {
                write!(f, "appmesh-preview:CreateVirtualRouter")
            }
            AppmeshPreviewActions::CreateVirtualService => {
                write!(f, "appmesh-preview:CreateVirtualService")
            }
            AppmeshPreviewActions::DeleteGatewayRoute => {
                write!(f, "appmesh-preview:DeleteGatewayRoute")
            }
            AppmeshPreviewActions::DeleteMesh => write!(f, "appmesh-preview:DeleteMesh"),
            AppmeshPreviewActions::DeleteMeshPolicy => {
                write!(f, "appmesh-preview:DeleteMeshPolicy")
            }
            AppmeshPreviewActions::DeleteRoute => write!(f, "appmesh-preview:DeleteRoute"),
            AppmeshPreviewActions::DeleteVirtualGateway => {
                write!(f, "appmesh-preview:DeleteVirtualGateway")
            }
            AppmeshPreviewActions::DeleteVirtualNode => {
                write!(f, "appmesh-preview:DeleteVirtualNode")
            }
            AppmeshPreviewActions::DeleteVirtualRouter => {
                write!(f, "appmesh-preview:DeleteVirtualRouter")
            }
            AppmeshPreviewActions::DeleteVirtualService => {
                write!(f, "appmesh-preview:DeleteVirtualService")
            }
            AppmeshPreviewActions::DescribeGatewayRoute => {
                write!(f, "appmesh-preview:DescribeGatewayRoute")
            }
            AppmeshPreviewActions::DescribeMesh => write!(f, "appmesh-preview:DescribeMesh"),
            AppmeshPreviewActions::DescribeRoute => write!(f, "appmesh-preview:DescribeRoute"),
            AppmeshPreviewActions::DescribeVirtualGateway => {
                write!(f, "appmesh-preview:DescribeVirtualGateway")
            }
            AppmeshPreviewActions::DescribeVirtualNode => {
                write!(f, "appmesh-preview:DescribeVirtualNode")
            }
            AppmeshPreviewActions::DescribeVirtualRouter => {
                write!(f, "appmesh-preview:DescribeVirtualRouter")
            }
            AppmeshPreviewActions::DescribeVirtualService => {
                write!(f, "appmesh-preview:DescribeVirtualService")
            }
            AppmeshPreviewActions::GetMeshPolicy => write!(f, "appmesh-preview:GetMeshPolicy"),
            AppmeshPreviewActions::ListGatewayRoutes => {
                write!(f, "appmesh-preview:ListGatewayRoutes")
            }
            AppmeshPreviewActions::ListMeshes => write!(f, "appmesh-preview:ListMeshes"),
            AppmeshPreviewActions::ListRoutes => write!(f, "appmesh-preview:ListRoutes"),
            AppmeshPreviewActions::ListVirtualGateways => {
                write!(f, "appmesh-preview:ListVirtualGateways")
            }
            AppmeshPreviewActions::ListVirtualNodes => {
                write!(f, "appmesh-preview:ListVirtualNodes")
            }
            AppmeshPreviewActions::ListVirtualRouters => {
                write!(f, "appmesh-preview:ListVirtualRouters")
            }
            AppmeshPreviewActions::ListVirtualServices => {
                write!(f, "appmesh-preview:ListVirtualServices")
            }
            AppmeshPreviewActions::PutMeshPolicy => write!(f, "appmesh-preview:PutMeshPolicy"),
            AppmeshPreviewActions::StreamAggregatedResources => {
                write!(f, "appmesh-preview:StreamAggregatedResources")
            }
            AppmeshPreviewActions::UpdateGatewayRoute => {
                write!(f, "appmesh-preview:UpdateGatewayRoute")
            }
            AppmeshPreviewActions::UpdateMesh => write!(f, "appmesh-preview:UpdateMesh"),
            AppmeshPreviewActions::UpdateRoute => write!(f, "appmesh-preview:UpdateRoute"),
            AppmeshPreviewActions::UpdateVirtualGateway => {
                write!(f, "appmesh-preview:UpdateVirtualGateway")
            }
            AppmeshPreviewActions::UpdateVirtualNode => {
                write!(f, "appmesh-preview:UpdateVirtualNode")
            }
            AppmeshPreviewActions::UpdateVirtualRouter => {
                write!(f, "appmesh-preview:UpdateVirtualRouter")
            }
            AppmeshPreviewActions::UpdateVirtualService => {
                write!(f, "appmesh-preview:UpdateVirtualService")
            }
        }
    }
}
