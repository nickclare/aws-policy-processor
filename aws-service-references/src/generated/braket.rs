// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum BraketActions {
    AcceptUserAgreement,
    CancelJob,
    CancelQuantumTask,
    CreateJob,
    CreateQuantumTask,
    GetDevice,
    GetJob,
    GetQuantumTask,
    GetServiceLinkedRoleStatus,
    GetUserAgreementStatus,
    ListTagsForResource,
    SearchDevices,
    SearchJobs,
    SearchQuantumTasks,
    TagResource,
    UntagResource,
}
impl std::fmt::Display for BraketActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BraketActions::AcceptUserAgreement => write!(f, "braket:AcceptUserAgreement"),
            BraketActions::CancelJob => write!(f, "braket:CancelJob"),
            BraketActions::CancelQuantumTask => write!(f, "braket:CancelQuantumTask"),
            BraketActions::CreateJob => write!(f, "braket:CreateJob"),
            BraketActions::CreateQuantumTask => write!(f, "braket:CreateQuantumTask"),
            BraketActions::GetDevice => write!(f, "braket:GetDevice"),
            BraketActions::GetJob => write!(f, "braket:GetJob"),
            BraketActions::GetQuantumTask => write!(f, "braket:GetQuantumTask"),
            BraketActions::GetServiceLinkedRoleStatus => {
                write!(f, "braket:GetServiceLinkedRoleStatus")
            }
            BraketActions::GetUserAgreementStatus => write!(f, "braket:GetUserAgreementStatus"),
            BraketActions::ListTagsForResource => write!(f, "braket:ListTagsForResource"),
            BraketActions::SearchDevices => write!(f, "braket:SearchDevices"),
            BraketActions::SearchJobs => write!(f, "braket:SearchJobs"),
            BraketActions::SearchQuantumTasks => write!(f, "braket:SearchQuantumTasks"),
            BraketActions::TagResource => write!(f, "braket:TagResource"),
            BraketActions::UntagResource => write!(f, "braket:UntagResource"),
        }
    }
}
