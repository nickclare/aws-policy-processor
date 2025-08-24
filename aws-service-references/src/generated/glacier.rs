// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum GlacierActions {
    AbortMultipartUpload,
    AbortVaultLock,
    AddTagsToVault,
    CompleteMultipartUpload,
    CompleteVaultLock,
    CreateVault,
    DeleteArchive,
    DeleteVault,
    DeleteVaultAccessPolicy,
    DeleteVaultNotifications,
    DescribeJob,
    DescribeVault,
    GetDataRetrievalPolicy,
    GetJobOutput,
    GetVaultAccessPolicy,
    GetVaultLock,
    GetVaultNotifications,
    InitiateJob,
    InitiateMultipartUpload,
    InitiateVaultLock,
    ListJobs,
    ListMultipartUploads,
    ListParts,
    ListProvisionedCapacity,
    ListTagsForVault,
    ListVaults,
    PurchaseProvisionedCapacity,
    RemoveTagsFromVault,
    SetDataRetrievalPolicy,
    SetVaultAccessPolicy,
    SetVaultNotifications,
    UploadArchive,
    UploadMultipartPart,
}
impl std::fmt::Display for GlacierActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GlacierActions::AbortMultipartUpload => write!(f, "glacier:AbortMultipartUpload"),
            GlacierActions::AbortVaultLock => write!(f, "glacier:AbortVaultLock"),
            GlacierActions::AddTagsToVault => write!(f, "glacier:AddTagsToVault"),
            GlacierActions::CompleteMultipartUpload => write!(f, "glacier:CompleteMultipartUpload"),
            GlacierActions::CompleteVaultLock => write!(f, "glacier:CompleteVaultLock"),
            GlacierActions::CreateVault => write!(f, "glacier:CreateVault"),
            GlacierActions::DeleteArchive => write!(f, "glacier:DeleteArchive"),
            GlacierActions::DeleteVault => write!(f, "glacier:DeleteVault"),
            GlacierActions::DeleteVaultAccessPolicy => write!(f, "glacier:DeleteVaultAccessPolicy"),
            GlacierActions::DeleteVaultNotifications => {
                write!(f, "glacier:DeleteVaultNotifications")
            }
            GlacierActions::DescribeJob => write!(f, "glacier:DescribeJob"),
            GlacierActions::DescribeVault => write!(f, "glacier:DescribeVault"),
            GlacierActions::GetDataRetrievalPolicy => write!(f, "glacier:GetDataRetrievalPolicy"),
            GlacierActions::GetJobOutput => write!(f, "glacier:GetJobOutput"),
            GlacierActions::GetVaultAccessPolicy => write!(f, "glacier:GetVaultAccessPolicy"),
            GlacierActions::GetVaultLock => write!(f, "glacier:GetVaultLock"),
            GlacierActions::GetVaultNotifications => write!(f, "glacier:GetVaultNotifications"),
            GlacierActions::InitiateJob => write!(f, "glacier:InitiateJob"),
            GlacierActions::InitiateMultipartUpload => write!(f, "glacier:InitiateMultipartUpload"),
            GlacierActions::InitiateVaultLock => write!(f, "glacier:InitiateVaultLock"),
            GlacierActions::ListJobs => write!(f, "glacier:ListJobs"),
            GlacierActions::ListMultipartUploads => write!(f, "glacier:ListMultipartUploads"),
            GlacierActions::ListParts => write!(f, "glacier:ListParts"),
            GlacierActions::ListProvisionedCapacity => write!(f, "glacier:ListProvisionedCapacity"),
            GlacierActions::ListTagsForVault => write!(f, "glacier:ListTagsForVault"),
            GlacierActions::ListVaults => write!(f, "glacier:ListVaults"),
            GlacierActions::PurchaseProvisionedCapacity => {
                write!(f, "glacier:PurchaseProvisionedCapacity")
            }
            GlacierActions::RemoveTagsFromVault => write!(f, "glacier:RemoveTagsFromVault"),
            GlacierActions::SetDataRetrievalPolicy => write!(f, "glacier:SetDataRetrievalPolicy"),
            GlacierActions::SetVaultAccessPolicy => write!(f, "glacier:SetVaultAccessPolicy"),
            GlacierActions::SetVaultNotifications => write!(f, "glacier:SetVaultNotifications"),
            GlacierActions::UploadArchive => write!(f, "glacier:UploadArchive"),
            GlacierActions::UploadMultipartPart => write!(f, "glacier:UploadMultipartPart"),
        }
    }
}
