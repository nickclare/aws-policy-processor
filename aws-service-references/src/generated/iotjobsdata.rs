// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum IotjobsdataActions {
    DescribeJobExecution,
    GetPendingJobExecutions,
    StartNextPendingJobExecution,
    UpdateJobExecution,
}
impl std::fmt::Display for IotjobsdataActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IotjobsdataActions::DescribeJobExecution => {
                write!(f, "iotjobsdata:DescribeJobExecution")
            }
            IotjobsdataActions::GetPendingJobExecutions => {
                write!(f, "iotjobsdata:GetPendingJobExecutions")
            }
            IotjobsdataActions::StartNextPendingJobExecution => {
                write!(f, "iotjobsdata:StartNextPendingJobExecution")
            }
            IotjobsdataActions::UpdateJobExecution => write!(f, "iotjobsdata:UpdateJobExecution"),
        }
    }
}
