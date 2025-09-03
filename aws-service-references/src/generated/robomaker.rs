// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum RobomakerActions {
    BatchDeleteWorlds,
    BatchDescribeSimulationJob,
    CancelDeploymentJob,
    CancelSimulationJob,
    CancelSimulationJobBatch,
    CancelWorldExportJob,
    CancelWorldGenerationJob,
    CreateDeploymentJob,
    CreateFleet,
    CreateRobot,
    CreateRobotApplication,
    CreateRobotApplicationVersion,
    CreateSimulationApplication,
    CreateSimulationApplicationVersion,
    CreateSimulationJob,
    CreateWorldExportJob,
    CreateWorldGenerationJob,
    CreateWorldTemplate,
    DeleteFleet,
    DeleteRobot,
    DeleteRobotApplication,
    DeleteSimulationApplication,
    DeleteWorldTemplate,
    DeregisterRobot,
    DescribeDeploymentJob,
    DescribeFleet,
    DescribeRobot,
    DescribeRobotApplication,
    DescribeSimulationApplication,
    DescribeSimulationJob,
    DescribeSimulationJobBatch,
    DescribeWorld,
    DescribeWorldExportJob,
    DescribeWorldGenerationJob,
    DescribeWorldTemplate,
    GetWorldTemplateBody,
    ListDeploymentJobs,
    ListFleets,
    ListRobotApplications,
    ListRobots,
    ListSimulationApplications,
    ListSimulationJobBatches,
    ListSimulationJobs,
    ListSupportedAvailabilityZones,
    ListTagsForResource,
    ListWorldExportJobs,
    ListWorldGenerationJobs,
    ListWorldTemplates,
    ListWorlds,
    RegisterRobot,
    RestartSimulationJob,
    StartSimulationJobBatch,
    SyncDeploymentJob,
    TagResource,
    UntagResource,
    UpdateRobotApplication,
    UpdateRobotDeployment,
    UpdateSimulationApplication,
    UpdateWorldTemplate,
}
impl std::fmt::Display for RobomakerActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RobomakerActions::BatchDeleteWorlds => write!(f, "robomaker:BatchDeleteWorlds"),
            RobomakerActions::BatchDescribeSimulationJob => {
                write!(f, "robomaker:BatchDescribeSimulationJob")
            }
            RobomakerActions::CancelDeploymentJob => write!(f, "robomaker:CancelDeploymentJob"),
            RobomakerActions::CancelSimulationJob => write!(f, "robomaker:CancelSimulationJob"),
            RobomakerActions::CancelSimulationJobBatch => {
                write!(f, "robomaker:CancelSimulationJobBatch")
            }
            RobomakerActions::CancelWorldExportJob => write!(f, "robomaker:CancelWorldExportJob"),
            RobomakerActions::CancelWorldGenerationJob => {
                write!(f, "robomaker:CancelWorldGenerationJob")
            }
            RobomakerActions::CreateDeploymentJob => write!(f, "robomaker:CreateDeploymentJob"),
            RobomakerActions::CreateFleet => write!(f, "robomaker:CreateFleet"),
            RobomakerActions::CreateRobot => write!(f, "robomaker:CreateRobot"),
            RobomakerActions::CreateRobotApplication => {
                write!(f, "robomaker:CreateRobotApplication")
            }
            RobomakerActions::CreateRobotApplicationVersion => {
                write!(f, "robomaker:CreateRobotApplicationVersion")
            }
            RobomakerActions::CreateSimulationApplication => {
                write!(f, "robomaker:CreateSimulationApplication")
            }
            RobomakerActions::CreateSimulationApplicationVersion => {
                write!(f, "robomaker:CreateSimulationApplicationVersion")
            }
            RobomakerActions::CreateSimulationJob => write!(f, "robomaker:CreateSimulationJob"),
            RobomakerActions::CreateWorldExportJob => write!(f, "robomaker:CreateWorldExportJob"),
            RobomakerActions::CreateWorldGenerationJob => {
                write!(f, "robomaker:CreateWorldGenerationJob")
            }
            RobomakerActions::CreateWorldTemplate => write!(f, "robomaker:CreateWorldTemplate"),
            RobomakerActions::DeleteFleet => write!(f, "robomaker:DeleteFleet"),
            RobomakerActions::DeleteRobot => write!(f, "robomaker:DeleteRobot"),
            RobomakerActions::DeleteRobotApplication => {
                write!(f, "robomaker:DeleteRobotApplication")
            }
            RobomakerActions::DeleteSimulationApplication => {
                write!(f, "robomaker:DeleteSimulationApplication")
            }
            RobomakerActions::DeleteWorldTemplate => write!(f, "robomaker:DeleteWorldTemplate"),
            RobomakerActions::DeregisterRobot => write!(f, "robomaker:DeregisterRobot"),
            RobomakerActions::DescribeDeploymentJob => write!(f, "robomaker:DescribeDeploymentJob"),
            RobomakerActions::DescribeFleet => write!(f, "robomaker:DescribeFleet"),
            RobomakerActions::DescribeRobot => write!(f, "robomaker:DescribeRobot"),
            RobomakerActions::DescribeRobotApplication => {
                write!(f, "robomaker:DescribeRobotApplication")
            }
            RobomakerActions::DescribeSimulationApplication => {
                write!(f, "robomaker:DescribeSimulationApplication")
            }
            RobomakerActions::DescribeSimulationJob => write!(f, "robomaker:DescribeSimulationJob"),
            RobomakerActions::DescribeSimulationJobBatch => {
                write!(f, "robomaker:DescribeSimulationJobBatch")
            }
            RobomakerActions::DescribeWorld => write!(f, "robomaker:DescribeWorld"),
            RobomakerActions::DescribeWorldExportJob => {
                write!(f, "robomaker:DescribeWorldExportJob")
            }
            RobomakerActions::DescribeWorldGenerationJob => {
                write!(f, "robomaker:DescribeWorldGenerationJob")
            }
            RobomakerActions::DescribeWorldTemplate => write!(f, "robomaker:DescribeWorldTemplate"),
            RobomakerActions::GetWorldTemplateBody => write!(f, "robomaker:GetWorldTemplateBody"),
            RobomakerActions::ListDeploymentJobs => write!(f, "robomaker:ListDeploymentJobs"),
            RobomakerActions::ListFleets => write!(f, "robomaker:ListFleets"),
            RobomakerActions::ListRobotApplications => write!(f, "robomaker:ListRobotApplications"),
            RobomakerActions::ListRobots => write!(f, "robomaker:ListRobots"),
            RobomakerActions::ListSimulationApplications => {
                write!(f, "robomaker:ListSimulationApplications")
            }
            RobomakerActions::ListSimulationJobBatches => {
                write!(f, "robomaker:ListSimulationJobBatches")
            }
            RobomakerActions::ListSimulationJobs => write!(f, "robomaker:ListSimulationJobs"),
            RobomakerActions::ListSupportedAvailabilityZones => {
                write!(f, "robomaker:ListSupportedAvailabilityZones")
            }
            RobomakerActions::ListTagsForResource => write!(f, "robomaker:ListTagsForResource"),
            RobomakerActions::ListWorldExportJobs => write!(f, "robomaker:ListWorldExportJobs"),
            RobomakerActions::ListWorldGenerationJobs => {
                write!(f, "robomaker:ListWorldGenerationJobs")
            }
            RobomakerActions::ListWorldTemplates => write!(f, "robomaker:ListWorldTemplates"),
            RobomakerActions::ListWorlds => write!(f, "robomaker:ListWorlds"),
            RobomakerActions::RegisterRobot => write!(f, "robomaker:RegisterRobot"),
            RobomakerActions::RestartSimulationJob => write!(f, "robomaker:RestartSimulationJob"),
            RobomakerActions::StartSimulationJobBatch => {
                write!(f, "robomaker:StartSimulationJobBatch")
            }
            RobomakerActions::SyncDeploymentJob => write!(f, "robomaker:SyncDeploymentJob"),
            RobomakerActions::TagResource => write!(f, "robomaker:TagResource"),
            RobomakerActions::UntagResource => write!(f, "robomaker:UntagResource"),
            RobomakerActions::UpdateRobotApplication => {
                write!(f, "robomaker:UpdateRobotApplication")
            }
            RobomakerActions::UpdateRobotDeployment => write!(f, "robomaker:UpdateRobotDeployment"),
            RobomakerActions::UpdateSimulationApplication => {
                write!(f, "robomaker:UpdateSimulationApplication")
            }
            RobomakerActions::UpdateWorldTemplate => write!(f, "robomaker:UpdateWorldTemplate"),
        }
    }
}
