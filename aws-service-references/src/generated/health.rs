// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum HealthActions {
    DescribeAffectedAccountsForOrganization,
    DescribeAffectedEntities,
    DescribeAffectedEntitiesForOrganization,
    DescribeEntityAggregates,
    DescribeEntityAggregatesForOrganization,
    DescribeEventAggregates,
    DescribeEventDetails,
    DescribeEventDetailsForOrganization,
    DescribeEventTypes,
    DescribeEvents,
    DescribeEventsForOrganization,
    DescribeHealthServiceStatusForOrganization,
    DisableHealthServiceAccessForOrganization,
    EnableHealthServiceAccessForOrganization,
}
impl std::fmt::Display for HealthActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HealthActions::DescribeAffectedAccountsForOrganization => {
                write!(f, "health:DescribeAffectedAccountsForOrganization")
            }
            HealthActions::DescribeAffectedEntities => write!(f, "health:DescribeAffectedEntities"),
            HealthActions::DescribeAffectedEntitiesForOrganization => {
                write!(f, "health:DescribeAffectedEntitiesForOrganization")
            }
            HealthActions::DescribeEntityAggregates => write!(f, "health:DescribeEntityAggregates"),
            HealthActions::DescribeEntityAggregatesForOrganization => {
                write!(f, "health:DescribeEntityAggregatesForOrganization")
            }
            HealthActions::DescribeEventAggregates => write!(f, "health:DescribeEventAggregates"),
            HealthActions::DescribeEventDetails => write!(f, "health:DescribeEventDetails"),
            HealthActions::DescribeEventDetailsForOrganization => {
                write!(f, "health:DescribeEventDetailsForOrganization")
            }
            HealthActions::DescribeEventTypes => write!(f, "health:DescribeEventTypes"),
            HealthActions::DescribeEvents => write!(f, "health:DescribeEvents"),
            HealthActions::DescribeEventsForOrganization => {
                write!(f, "health:DescribeEventsForOrganization")
            }
            HealthActions::DescribeHealthServiceStatusForOrganization => {
                write!(f, "health:DescribeHealthServiceStatusForOrganization")
            }
            HealthActions::DisableHealthServiceAccessForOrganization => {
                write!(f, "health:DisableHealthServiceAccessForOrganization")
            }
            HealthActions::EnableHealthServiceAccessForOrganization => {
                write!(f, "health:EnableHealthServiceAccessForOrganization")
            }
        }
    }
}
