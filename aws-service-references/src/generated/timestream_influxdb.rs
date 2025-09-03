// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum TimestreamInfluxdbActions {
    CreateDbCluster,
    CreateDbInstance,
    CreateDbParameterGroup,
    DeleteDbCluster,
    DeleteDbInstance,
    GetDbCluster,
    GetDbInstance,
    GetDbParameterGroup,
    ListDbClusters,
    ListDbInstances,
    ListDbInstancesForCluster,
    ListDbParameterGroups,
    ListTagsForResource,
    TagResource,
    UntagResource,
    UpdateDbCluster,
    UpdateDbInstance,
}
impl std::fmt::Display for TimestreamInfluxdbActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TimestreamInfluxdbActions::CreateDbCluster => {
                write!(f, "timestream-influxdb:CreateDbCluster")
            }
            TimestreamInfluxdbActions::CreateDbInstance => {
                write!(f, "timestream-influxdb:CreateDbInstance")
            }
            TimestreamInfluxdbActions::CreateDbParameterGroup => {
                write!(f, "timestream-influxdb:CreateDbParameterGroup")
            }
            TimestreamInfluxdbActions::DeleteDbCluster => {
                write!(f, "timestream-influxdb:DeleteDbCluster")
            }
            TimestreamInfluxdbActions::DeleteDbInstance => {
                write!(f, "timestream-influxdb:DeleteDbInstance")
            }
            TimestreamInfluxdbActions::GetDbCluster => {
                write!(f, "timestream-influxdb:GetDbCluster")
            }
            TimestreamInfluxdbActions::GetDbInstance => {
                write!(f, "timestream-influxdb:GetDbInstance")
            }
            TimestreamInfluxdbActions::GetDbParameterGroup => {
                write!(f, "timestream-influxdb:GetDbParameterGroup")
            }
            TimestreamInfluxdbActions::ListDbClusters => {
                write!(f, "timestream-influxdb:ListDbClusters")
            }
            TimestreamInfluxdbActions::ListDbInstances => {
                write!(f, "timestream-influxdb:ListDbInstances")
            }
            TimestreamInfluxdbActions::ListDbInstancesForCluster => {
                write!(f, "timestream-influxdb:ListDbInstancesForCluster")
            }
            TimestreamInfluxdbActions::ListDbParameterGroups => {
                write!(f, "timestream-influxdb:ListDbParameterGroups")
            }
            TimestreamInfluxdbActions::ListTagsForResource => {
                write!(f, "timestream-influxdb:ListTagsForResource")
            }
            TimestreamInfluxdbActions::TagResource => write!(f, "timestream-influxdb:TagResource"),
            TimestreamInfluxdbActions::UntagResource => {
                write!(f, "timestream-influxdb:UntagResource")
            }
            TimestreamInfluxdbActions::UpdateDbCluster => {
                write!(f, "timestream-influxdb:UpdateDbCluster")
            }
            TimestreamInfluxdbActions::UpdateDbInstance => {
                write!(f, "timestream-influxdb:UpdateDbInstance")
            }
        }
    }
}
