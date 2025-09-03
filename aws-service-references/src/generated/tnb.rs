// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum TnbActions {
    CancelSolNetworkOperation,
    CreateSolFunctionPackage,
    CreateSolNetworkInstance,
    CreateSolNetworkPackage,
    DeleteSolFunctionPackage,
    DeleteSolNetworkInstance,
    DeleteSolNetworkPackage,
    GetSolFunctionInstance,
    GetSolFunctionPackage,
    GetSolFunctionPackageContent,
    GetSolFunctionPackageDescriptor,
    GetSolNetworkInstance,
    GetSolNetworkOperation,
    GetSolNetworkPackage,
    GetSolNetworkPackageContent,
    GetSolNetworkPackageDescriptor,
    InstantiateSolNetworkInstance,
    ListSolFunctionInstances,
    ListSolFunctionPackages,
    ListSolNetworkInstances,
    ListSolNetworkOperations,
    ListSolNetworkPackages,
    ListTagsForResource,
    PutSolFunctionPackageContent,
    PutSolNetworkPackageContent,
    TagResource,
    TerminateSolNetworkInstance,
    UntagResource,
    UpdateSolFunctionPackage,
    UpdateSolNetworkInstance,
    UpdateSolNetworkPackage,
    ValidateSolFunctionPackageContent,
    ValidateSolNetworkPackageContent,
}
impl std::fmt::Display for TnbActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TnbActions::CancelSolNetworkOperation => write!(f, "tnb:CancelSolNetworkOperation"),
            TnbActions::CreateSolFunctionPackage => write!(f, "tnb:CreateSolFunctionPackage"),
            TnbActions::CreateSolNetworkInstance => write!(f, "tnb:CreateSolNetworkInstance"),
            TnbActions::CreateSolNetworkPackage => write!(f, "tnb:CreateSolNetworkPackage"),
            TnbActions::DeleteSolFunctionPackage => write!(f, "tnb:DeleteSolFunctionPackage"),
            TnbActions::DeleteSolNetworkInstance => write!(f, "tnb:DeleteSolNetworkInstance"),
            TnbActions::DeleteSolNetworkPackage => write!(f, "tnb:DeleteSolNetworkPackage"),
            TnbActions::GetSolFunctionInstance => write!(f, "tnb:GetSolFunctionInstance"),
            TnbActions::GetSolFunctionPackage => write!(f, "tnb:GetSolFunctionPackage"),
            TnbActions::GetSolFunctionPackageContent => {
                write!(f, "tnb:GetSolFunctionPackageContent")
            }
            TnbActions::GetSolFunctionPackageDescriptor => {
                write!(f, "tnb:GetSolFunctionPackageDescriptor")
            }
            TnbActions::GetSolNetworkInstance => write!(f, "tnb:GetSolNetworkInstance"),
            TnbActions::GetSolNetworkOperation => write!(f, "tnb:GetSolNetworkOperation"),
            TnbActions::GetSolNetworkPackage => write!(f, "tnb:GetSolNetworkPackage"),
            TnbActions::GetSolNetworkPackageContent => write!(f, "tnb:GetSolNetworkPackageContent"),
            TnbActions::GetSolNetworkPackageDescriptor => {
                write!(f, "tnb:GetSolNetworkPackageDescriptor")
            }
            TnbActions::InstantiateSolNetworkInstance => {
                write!(f, "tnb:InstantiateSolNetworkInstance")
            }
            TnbActions::ListSolFunctionInstances => write!(f, "tnb:ListSolFunctionInstances"),
            TnbActions::ListSolFunctionPackages => write!(f, "tnb:ListSolFunctionPackages"),
            TnbActions::ListSolNetworkInstances => write!(f, "tnb:ListSolNetworkInstances"),
            TnbActions::ListSolNetworkOperations => write!(f, "tnb:ListSolNetworkOperations"),
            TnbActions::ListSolNetworkPackages => write!(f, "tnb:ListSolNetworkPackages"),
            TnbActions::ListTagsForResource => write!(f, "tnb:ListTagsForResource"),
            TnbActions::PutSolFunctionPackageContent => {
                write!(f, "tnb:PutSolFunctionPackageContent")
            }
            TnbActions::PutSolNetworkPackageContent => write!(f, "tnb:PutSolNetworkPackageContent"),
            TnbActions::TagResource => write!(f, "tnb:TagResource"),
            TnbActions::TerminateSolNetworkInstance => write!(f, "tnb:TerminateSolNetworkInstance"),
            TnbActions::UntagResource => write!(f, "tnb:UntagResource"),
            TnbActions::UpdateSolFunctionPackage => write!(f, "tnb:UpdateSolFunctionPackage"),
            TnbActions::UpdateSolNetworkInstance => write!(f, "tnb:UpdateSolNetworkInstance"),
            TnbActions::UpdateSolNetworkPackage => write!(f, "tnb:UpdateSolNetworkPackage"),
            TnbActions::ValidateSolFunctionPackageContent => {
                write!(f, "tnb:ValidateSolFunctionPackageContent")
            }
            TnbActions::ValidateSolNetworkPackageContent => {
                write!(f, "tnb:ValidateSolNetworkPackageContent")
            }
        }
    }
}
