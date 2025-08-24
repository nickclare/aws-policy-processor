// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum CodeartifactActions {
    AssociateExternalConnection,
    AssociateWithDownstreamRepository,
    CopyPackageVersions,
    CreateDomain,
    CreatePackageGroup,
    CreateRepository,
    DeleteDomain,
    DeleteDomainPermissionsPolicy,
    DeletePackage,
    DeletePackageGroup,
    DeletePackageVersions,
    DeleteRepository,
    DeleteRepositoryPermissionsPolicy,
    DescribeDomain,
    DescribePackage,
    DescribePackageGroup,
    DescribePackageVersion,
    DescribeRepository,
    DisassociateExternalConnection,
    DisposePackageVersions,
    GetAssociatedPackageGroup,
    GetAuthorizationToken,
    GetDomainPermissionsPolicy,
    GetPackageVersionAsset,
    GetPackageVersionReadme,
    GetRepositoryEndpoint,
    GetRepositoryPermissionsPolicy,
    ListAllowedRepositoriesForGroup,
    ListAssociatedPackages,
    ListDomains,
    ListPackageGroups,
    ListPackageVersionAssets,
    ListPackageVersionDependencies,
    ListPackageVersions,
    ListPackages,
    ListRepositories,
    ListRepositoriesInDomain,
    ListSubPackageGroups,
    ListTagsForResource,
    PublishPackageVersion,
    PutDomainPermissionsPolicy,
    PutPackageMetadata,
    PutPackageOriginConfiguration,
    PutRepositoryPermissionsPolicy,
    ReadFromRepository,
    TagResource,
    UntagResource,
    UpdatePackageGroup,
    UpdatePackageGroupOriginConfiguration,
    UpdatePackageVersionsStatus,
    UpdateRepository,
}
impl std::fmt::Display for CodeartifactActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CodeartifactActions::AssociateExternalConnection => {
                write!(f, "codeartifact:AssociateExternalConnection")
            }
            CodeartifactActions::AssociateWithDownstreamRepository => {
                write!(f, "codeartifact:AssociateWithDownstreamRepository")
            }
            CodeartifactActions::CopyPackageVersions => {
                write!(f, "codeartifact:CopyPackageVersions")
            }
            CodeartifactActions::CreateDomain => write!(f, "codeartifact:CreateDomain"),
            CodeartifactActions::CreatePackageGroup => write!(f, "codeartifact:CreatePackageGroup"),
            CodeartifactActions::CreateRepository => write!(f, "codeartifact:CreateRepository"),
            CodeartifactActions::DeleteDomain => write!(f, "codeartifact:DeleteDomain"),
            CodeartifactActions::DeleteDomainPermissionsPolicy => {
                write!(f, "codeartifact:DeleteDomainPermissionsPolicy")
            }
            CodeartifactActions::DeletePackage => write!(f, "codeartifact:DeletePackage"),
            CodeartifactActions::DeletePackageGroup => write!(f, "codeartifact:DeletePackageGroup"),
            CodeartifactActions::DeletePackageVersions => {
                write!(f, "codeartifact:DeletePackageVersions")
            }
            CodeartifactActions::DeleteRepository => write!(f, "codeartifact:DeleteRepository"),
            CodeartifactActions::DeleteRepositoryPermissionsPolicy => {
                write!(f, "codeartifact:DeleteRepositoryPermissionsPolicy")
            }
            CodeartifactActions::DescribeDomain => write!(f, "codeartifact:DescribeDomain"),
            CodeartifactActions::DescribePackage => write!(f, "codeartifact:DescribePackage"),
            CodeartifactActions::DescribePackageGroup => {
                write!(f, "codeartifact:DescribePackageGroup")
            }
            CodeartifactActions::DescribePackageVersion => {
                write!(f, "codeartifact:DescribePackageVersion")
            }
            CodeartifactActions::DescribeRepository => write!(f, "codeartifact:DescribeRepository"),
            CodeartifactActions::DisassociateExternalConnection => {
                write!(f, "codeartifact:DisassociateExternalConnection")
            }
            CodeartifactActions::DisposePackageVersions => {
                write!(f, "codeartifact:DisposePackageVersions")
            }
            CodeartifactActions::GetAssociatedPackageGroup => {
                write!(f, "codeartifact:GetAssociatedPackageGroup")
            }
            CodeartifactActions::GetAuthorizationToken => {
                write!(f, "codeartifact:GetAuthorizationToken")
            }
            CodeartifactActions::GetDomainPermissionsPolicy => {
                write!(f, "codeartifact:GetDomainPermissionsPolicy")
            }
            CodeartifactActions::GetPackageVersionAsset => {
                write!(f, "codeartifact:GetPackageVersionAsset")
            }
            CodeartifactActions::GetPackageVersionReadme => {
                write!(f, "codeartifact:GetPackageVersionReadme")
            }
            CodeartifactActions::GetRepositoryEndpoint => {
                write!(f, "codeartifact:GetRepositoryEndpoint")
            }
            CodeartifactActions::GetRepositoryPermissionsPolicy => {
                write!(f, "codeartifact:GetRepositoryPermissionsPolicy")
            }
            CodeartifactActions::ListAllowedRepositoriesForGroup => {
                write!(f, "codeartifact:ListAllowedRepositoriesForGroup")
            }
            CodeartifactActions::ListAssociatedPackages => {
                write!(f, "codeartifact:ListAssociatedPackages")
            }
            CodeartifactActions::ListDomains => write!(f, "codeartifact:ListDomains"),
            CodeartifactActions::ListPackageGroups => write!(f, "codeartifact:ListPackageGroups"),
            CodeartifactActions::ListPackageVersionAssets => {
                write!(f, "codeartifact:ListPackageVersionAssets")
            }
            CodeartifactActions::ListPackageVersionDependencies => {
                write!(f, "codeartifact:ListPackageVersionDependencies")
            }
            CodeartifactActions::ListPackageVersions => {
                write!(f, "codeartifact:ListPackageVersions")
            }
            CodeartifactActions::ListPackages => write!(f, "codeartifact:ListPackages"),
            CodeartifactActions::ListRepositories => write!(f, "codeartifact:ListRepositories"),
            CodeartifactActions::ListRepositoriesInDomain => {
                write!(f, "codeartifact:ListRepositoriesInDomain")
            }
            CodeartifactActions::ListSubPackageGroups => {
                write!(f, "codeartifact:ListSubPackageGroups")
            }
            CodeartifactActions::ListTagsForResource => {
                write!(f, "codeartifact:ListTagsForResource")
            }
            CodeartifactActions::PublishPackageVersion => {
                write!(f, "codeartifact:PublishPackageVersion")
            }
            CodeartifactActions::PutDomainPermissionsPolicy => {
                write!(f, "codeartifact:PutDomainPermissionsPolicy")
            }
            CodeartifactActions::PutPackageMetadata => write!(f, "codeartifact:PutPackageMetadata"),
            CodeartifactActions::PutPackageOriginConfiguration => {
                write!(f, "codeartifact:PutPackageOriginConfiguration")
            }
            CodeartifactActions::PutRepositoryPermissionsPolicy => {
                write!(f, "codeartifact:PutRepositoryPermissionsPolicy")
            }
            CodeartifactActions::ReadFromRepository => write!(f, "codeartifact:ReadFromRepository"),
            CodeartifactActions::TagResource => write!(f, "codeartifact:TagResource"),
            CodeartifactActions::UntagResource => write!(f, "codeartifact:UntagResource"),
            CodeartifactActions::UpdatePackageGroup => write!(f, "codeartifact:UpdatePackageGroup"),
            CodeartifactActions::UpdatePackageGroupOriginConfiguration => {
                write!(f, "codeartifact:UpdatePackageGroupOriginConfiguration")
            }
            CodeartifactActions::UpdatePackageVersionsStatus => {
                write!(f, "codeartifact:UpdatePackageVersionsStatus")
            }
            CodeartifactActions::UpdateRepository => write!(f, "codeartifact:UpdateRepository"),
        }
    }
}
