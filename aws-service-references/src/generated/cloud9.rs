// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum Cloud9Actions {
    ActivateEc2Remote,
    CreateEnvironmentEc2,
    CreateEnvironmentMembership,
    CreateEnvironmentSsh,
    CreateEnvironmentToken,
    DeleteEnvironment,
    DeleteEnvironmentMembership,
    DescribeEc2Remote,
    DescribeEnvironmentMemberships,
    DescribeEnvironmentStatus,
    DescribeEnvironments,
    DescribeSshRemote,
    GetEnvironmentConfig,
    GetEnvironmentSettings,
    GetMembershipSettings,
    GetMigrationExperiences,
    GetUserPublicKey,
    GetUserSettings,
    ListEnvironments,
    ListTagsForResource,
    ModifyTemporaryCredentialsOnEnvironmentEc2,
    TagResource,
    UntagResource,
    UpdateEnvironment,
    UpdateEnvironmentMembership,
    UpdateEnvironmentSettings,
    UpdateMembershipSettings,
    UpdateSshRemote,
    UpdateUserSettings,
}
impl std::fmt::Display for Cloud9Actions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cloud9Actions::ActivateEc2Remote => write!(f, "cloud9:ActivateEC2Remote"),
            Cloud9Actions::CreateEnvironmentEc2 => write!(f, "cloud9:CreateEnvironmentEC2"),
            Cloud9Actions::CreateEnvironmentMembership => {
                write!(f, "cloud9:CreateEnvironmentMembership")
            }
            Cloud9Actions::CreateEnvironmentSsh => write!(f, "cloud9:CreateEnvironmentSSH"),
            Cloud9Actions::CreateEnvironmentToken => write!(f, "cloud9:CreateEnvironmentToken"),
            Cloud9Actions::DeleteEnvironment => write!(f, "cloud9:DeleteEnvironment"),
            Cloud9Actions::DeleteEnvironmentMembership => {
                write!(f, "cloud9:DeleteEnvironmentMembership")
            }
            Cloud9Actions::DescribeEc2Remote => write!(f, "cloud9:DescribeEC2Remote"),
            Cloud9Actions::DescribeEnvironmentMemberships => {
                write!(f, "cloud9:DescribeEnvironmentMemberships")
            }
            Cloud9Actions::DescribeEnvironmentStatus => {
                write!(f, "cloud9:DescribeEnvironmentStatus")
            }
            Cloud9Actions::DescribeEnvironments => write!(f, "cloud9:DescribeEnvironments"),
            Cloud9Actions::DescribeSshRemote => write!(f, "cloud9:DescribeSSHRemote"),
            Cloud9Actions::GetEnvironmentConfig => write!(f, "cloud9:GetEnvironmentConfig"),
            Cloud9Actions::GetEnvironmentSettings => write!(f, "cloud9:GetEnvironmentSettings"),
            Cloud9Actions::GetMembershipSettings => write!(f, "cloud9:GetMembershipSettings"),
            Cloud9Actions::GetMigrationExperiences => write!(f, "cloud9:GetMigrationExperiences"),
            Cloud9Actions::GetUserPublicKey => write!(f, "cloud9:GetUserPublicKey"),
            Cloud9Actions::GetUserSettings => write!(f, "cloud9:GetUserSettings"),
            Cloud9Actions::ListEnvironments => write!(f, "cloud9:ListEnvironments"),
            Cloud9Actions::ListTagsForResource => write!(f, "cloud9:ListTagsForResource"),
            Cloud9Actions::ModifyTemporaryCredentialsOnEnvironmentEc2 => {
                write!(f, "cloud9:ModifyTemporaryCredentialsOnEnvironmentEC2")
            }
            Cloud9Actions::TagResource => write!(f, "cloud9:TagResource"),
            Cloud9Actions::UntagResource => write!(f, "cloud9:UntagResource"),
            Cloud9Actions::UpdateEnvironment => write!(f, "cloud9:UpdateEnvironment"),
            Cloud9Actions::UpdateEnvironmentMembership => {
                write!(f, "cloud9:UpdateEnvironmentMembership")
            }
            Cloud9Actions::UpdateEnvironmentSettings => {
                write!(f, "cloud9:UpdateEnvironmentSettings")
            }
            Cloud9Actions::UpdateMembershipSettings => write!(f, "cloud9:UpdateMembershipSettings"),
            Cloud9Actions::UpdateSshRemote => write!(f, "cloud9:UpdateSSHRemote"),
            Cloud9Actions::UpdateUserSettings => write!(f, "cloud9:UpdateUserSettings"),
        }
    }
}
