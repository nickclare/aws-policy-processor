// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum RedshiftServerlessActions {
    ConvertRecoveryPointToSnapshot,
    CreateCustomDomainAssociation,
    CreateEndpointAccess,
    CreateNamespace,
    CreateReservation,
    CreateScheduledAction,
    CreateSnapshot,
    CreateSnapshotCopyConfiguration,
    CreateUsageLimit,
    CreateWorkgroup,
    DeleteCustomDomainAssociation,
    DeleteEndpointAccess,
    DeleteNamespace,
    DeleteResourcePolicy,
    DeleteScheduledAction,
    DeleteSnapshot,
    DeleteSnapshotCopyConfiguration,
    DeleteUsageLimit,
    DeleteWorkgroup,
    DescribeOneTimeCredit,
    GetCredentials,
    GetCustomDomainAssociation,
    GetEndpointAccess,
    GetManagedWorkgroup,
    GetNamespace,
    GetRecoveryPoint,
    GetReservation,
    GetReservationOffering,
    GetResourcePolicy,
    GetScheduledAction,
    GetSnapshot,
    GetTableRestoreStatus,
    GetTrack,
    GetUsageLimit,
    GetWorkgroup,
    ListCustomDomainAssociations,
    ListEndpointAccess,
    ListManagedWorkgroups,
    ListNamespaces,
    ListRecoveryPoints,
    ListReservationOfferings,
    ListReservations,
    ListScheduledActions,
    ListSnapshotCopyConfigurations,
    ListSnapshots,
    ListTableRestoreStatus,
    ListTagsForResource,
    ListTracks,
    ListUsageLimits,
    ListWorkgroups,
    PutResourcePolicy,
    RestoreFromRecoveryPoint,
    RestoreFromSnapshot,
    RestoreTableFromRecoveryPoint,
    RestoreTableFromSnapshot,
    TagResource,
    UntagResource,
    UpdateCustomDomainAssociation,
    UpdateEndpointAccess,
    UpdateNamespace,
    UpdateScheduledAction,
    UpdateSnapshot,
    UpdateSnapshotCopyConfiguration,
    UpdateUsageLimit,
    UpdateWorkgroup,
}
impl std::fmt::Display for RedshiftServerlessActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RedshiftServerlessActions::ConvertRecoveryPointToSnapshot => {
                write!(f, "redshift-serverless:ConvertRecoveryPointToSnapshot")
            }
            RedshiftServerlessActions::CreateCustomDomainAssociation => {
                write!(f, "redshift-serverless:CreateCustomDomainAssociation")
            }
            RedshiftServerlessActions::CreateEndpointAccess => {
                write!(f, "redshift-serverless:CreateEndpointAccess")
            }
            RedshiftServerlessActions::CreateNamespace => {
                write!(f, "redshift-serverless:CreateNamespace")
            }
            RedshiftServerlessActions::CreateReservation => {
                write!(f, "redshift-serverless:CreateReservation")
            }
            RedshiftServerlessActions::CreateScheduledAction => {
                write!(f, "redshift-serverless:CreateScheduledAction")
            }
            RedshiftServerlessActions::CreateSnapshot => {
                write!(f, "redshift-serverless:CreateSnapshot")
            }
            RedshiftServerlessActions::CreateSnapshotCopyConfiguration => {
                write!(f, "redshift-serverless:CreateSnapshotCopyConfiguration")
            }
            RedshiftServerlessActions::CreateUsageLimit => {
                write!(f, "redshift-serverless:CreateUsageLimit")
            }
            RedshiftServerlessActions::CreateWorkgroup => {
                write!(f, "redshift-serverless:CreateWorkgroup")
            }
            RedshiftServerlessActions::DeleteCustomDomainAssociation => {
                write!(f, "redshift-serverless:DeleteCustomDomainAssociation")
            }
            RedshiftServerlessActions::DeleteEndpointAccess => {
                write!(f, "redshift-serverless:DeleteEndpointAccess")
            }
            RedshiftServerlessActions::DeleteNamespace => {
                write!(f, "redshift-serverless:DeleteNamespace")
            }
            RedshiftServerlessActions::DeleteResourcePolicy => {
                write!(f, "redshift-serverless:DeleteResourcePolicy")
            }
            RedshiftServerlessActions::DeleteScheduledAction => {
                write!(f, "redshift-serverless:DeleteScheduledAction")
            }
            RedshiftServerlessActions::DeleteSnapshot => {
                write!(f, "redshift-serverless:DeleteSnapshot")
            }
            RedshiftServerlessActions::DeleteSnapshotCopyConfiguration => {
                write!(f, "redshift-serverless:DeleteSnapshotCopyConfiguration")
            }
            RedshiftServerlessActions::DeleteUsageLimit => {
                write!(f, "redshift-serverless:DeleteUsageLimit")
            }
            RedshiftServerlessActions::DeleteWorkgroup => {
                write!(f, "redshift-serverless:DeleteWorkgroup")
            }
            RedshiftServerlessActions::DescribeOneTimeCredit => {
                write!(f, "redshift-serverless:DescribeOneTimeCredit")
            }
            RedshiftServerlessActions::GetCredentials => {
                write!(f, "redshift-serverless:GetCredentials")
            }
            RedshiftServerlessActions::GetCustomDomainAssociation => {
                write!(f, "redshift-serverless:GetCustomDomainAssociation")
            }
            RedshiftServerlessActions::GetEndpointAccess => {
                write!(f, "redshift-serverless:GetEndpointAccess")
            }
            RedshiftServerlessActions::GetManagedWorkgroup => {
                write!(f, "redshift-serverless:GetManagedWorkgroup")
            }
            RedshiftServerlessActions::GetNamespace => {
                write!(f, "redshift-serverless:GetNamespace")
            }
            RedshiftServerlessActions::GetRecoveryPoint => {
                write!(f, "redshift-serverless:GetRecoveryPoint")
            }
            RedshiftServerlessActions::GetReservation => {
                write!(f, "redshift-serverless:GetReservation")
            }
            RedshiftServerlessActions::GetReservationOffering => {
                write!(f, "redshift-serverless:GetReservationOffering")
            }
            RedshiftServerlessActions::GetResourcePolicy => {
                write!(f, "redshift-serverless:GetResourcePolicy")
            }
            RedshiftServerlessActions::GetScheduledAction => {
                write!(f, "redshift-serverless:GetScheduledAction")
            }
            RedshiftServerlessActions::GetSnapshot => write!(f, "redshift-serverless:GetSnapshot"),
            RedshiftServerlessActions::GetTableRestoreStatus => {
                write!(f, "redshift-serverless:GetTableRestoreStatus")
            }
            RedshiftServerlessActions::GetTrack => write!(f, "redshift-serverless:GetTrack"),
            RedshiftServerlessActions::GetUsageLimit => {
                write!(f, "redshift-serverless:GetUsageLimit")
            }
            RedshiftServerlessActions::GetWorkgroup => {
                write!(f, "redshift-serverless:GetWorkgroup")
            }
            RedshiftServerlessActions::ListCustomDomainAssociations => {
                write!(f, "redshift-serverless:ListCustomDomainAssociations")
            }
            RedshiftServerlessActions::ListEndpointAccess => {
                write!(f, "redshift-serverless:ListEndpointAccess")
            }
            RedshiftServerlessActions::ListManagedWorkgroups => {
                write!(f, "redshift-serverless:ListManagedWorkgroups")
            }
            RedshiftServerlessActions::ListNamespaces => {
                write!(f, "redshift-serverless:ListNamespaces")
            }
            RedshiftServerlessActions::ListRecoveryPoints => {
                write!(f, "redshift-serverless:ListRecoveryPoints")
            }
            RedshiftServerlessActions::ListReservationOfferings => {
                write!(f, "redshift-serverless:ListReservationOfferings")
            }
            RedshiftServerlessActions::ListReservations => {
                write!(f, "redshift-serverless:ListReservations")
            }
            RedshiftServerlessActions::ListScheduledActions => {
                write!(f, "redshift-serverless:ListScheduledActions")
            }
            RedshiftServerlessActions::ListSnapshotCopyConfigurations => {
                write!(f, "redshift-serverless:ListSnapshotCopyConfigurations")
            }
            RedshiftServerlessActions::ListSnapshots => {
                write!(f, "redshift-serverless:ListSnapshots")
            }
            RedshiftServerlessActions::ListTableRestoreStatus => {
                write!(f, "redshift-serverless:ListTableRestoreStatus")
            }
            RedshiftServerlessActions::ListTagsForResource => {
                write!(f, "redshift-serverless:ListTagsForResource")
            }
            RedshiftServerlessActions::ListTracks => write!(f, "redshift-serverless:ListTracks"),
            RedshiftServerlessActions::ListUsageLimits => {
                write!(f, "redshift-serverless:ListUsageLimits")
            }
            RedshiftServerlessActions::ListWorkgroups => {
                write!(f, "redshift-serverless:ListWorkgroups")
            }
            RedshiftServerlessActions::PutResourcePolicy => {
                write!(f, "redshift-serverless:PutResourcePolicy")
            }
            RedshiftServerlessActions::RestoreFromRecoveryPoint => {
                write!(f, "redshift-serverless:RestoreFromRecoveryPoint")
            }
            RedshiftServerlessActions::RestoreFromSnapshot => {
                write!(f, "redshift-serverless:RestoreFromSnapshot")
            }
            RedshiftServerlessActions::RestoreTableFromRecoveryPoint => {
                write!(f, "redshift-serverless:RestoreTableFromRecoveryPoint")
            }
            RedshiftServerlessActions::RestoreTableFromSnapshot => {
                write!(f, "redshift-serverless:RestoreTableFromSnapshot")
            }
            RedshiftServerlessActions::TagResource => write!(f, "redshift-serverless:TagResource"),
            RedshiftServerlessActions::UntagResource => {
                write!(f, "redshift-serverless:UntagResource")
            }
            RedshiftServerlessActions::UpdateCustomDomainAssociation => {
                write!(f, "redshift-serverless:UpdateCustomDomainAssociation")
            }
            RedshiftServerlessActions::UpdateEndpointAccess => {
                write!(f, "redshift-serverless:UpdateEndpointAccess")
            }
            RedshiftServerlessActions::UpdateNamespace => {
                write!(f, "redshift-serverless:UpdateNamespace")
            }
            RedshiftServerlessActions::UpdateScheduledAction => {
                write!(f, "redshift-serverless:UpdateScheduledAction")
            }
            RedshiftServerlessActions::UpdateSnapshot => {
                write!(f, "redshift-serverless:UpdateSnapshot")
            }
            RedshiftServerlessActions::UpdateSnapshotCopyConfiguration => {
                write!(f, "redshift-serverless:UpdateSnapshotCopyConfiguration")
            }
            RedshiftServerlessActions::UpdateUsageLimit => {
                write!(f, "redshift-serverless:UpdateUsageLimit")
            }
            RedshiftServerlessActions::UpdateWorkgroup => {
                write!(f, "redshift-serverless:UpdateWorkgroup")
            }
        }
    }
}
