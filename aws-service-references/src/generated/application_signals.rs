// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum ApplicationSignalsActions {
    BatchGetServiceLevelObjectiveBudgetReport,
    BatchUpdateExclusionWindows,
    CreateServiceLevelObjective,
    DeleteServiceLevelObjective,
    GetService,
    GetServiceLevelObjective,
    Link,
    ListObservedEntities,
    ListServiceDependencies,
    ListServiceDependents,
    ListServiceLevelObjectiveExclusionWindows,
    ListServiceLevelObjectives,
    ListServiceOperations,
    ListServices,
    ListTagsForResource,
    StartDiscovery,
    TagResource,
    UntagResource,
    UpdateServiceLevelObjective,
}
impl std::fmt::Display for ApplicationSignalsActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApplicationSignalsActions::BatchGetServiceLevelObjectiveBudgetReport => write!(
                f,
                "application-signals:BatchGetServiceLevelObjectiveBudgetReport"
            ),
            ApplicationSignalsActions::BatchUpdateExclusionWindows => {
                write!(f, "application-signals:BatchUpdateExclusionWindows")
            }
            ApplicationSignalsActions::CreateServiceLevelObjective => {
                write!(f, "application-signals:CreateServiceLevelObjective")
            }
            ApplicationSignalsActions::DeleteServiceLevelObjective => {
                write!(f, "application-signals:DeleteServiceLevelObjective")
            }
            ApplicationSignalsActions::GetService => write!(f, "application-signals:GetService"),
            ApplicationSignalsActions::GetServiceLevelObjective => {
                write!(f, "application-signals:GetServiceLevelObjective")
            }
            ApplicationSignalsActions::Link => write!(f, "application-signals:Link"),
            ApplicationSignalsActions::ListObservedEntities => {
                write!(f, "application-signals:ListObservedEntities")
            }
            ApplicationSignalsActions::ListServiceDependencies => {
                write!(f, "application-signals:ListServiceDependencies")
            }
            ApplicationSignalsActions::ListServiceDependents => {
                write!(f, "application-signals:ListServiceDependents")
            }
            ApplicationSignalsActions::ListServiceLevelObjectiveExclusionWindows => write!(
                f,
                "application-signals:ListServiceLevelObjectiveExclusionWindows"
            ),
            ApplicationSignalsActions::ListServiceLevelObjectives => {
                write!(f, "application-signals:ListServiceLevelObjectives")
            }
            ApplicationSignalsActions::ListServiceOperations => {
                write!(f, "application-signals:ListServiceOperations")
            }
            ApplicationSignalsActions::ListServices => {
                write!(f, "application-signals:ListServices")
            }
            ApplicationSignalsActions::ListTagsForResource => {
                write!(f, "application-signals:ListTagsForResource")
            }
            ApplicationSignalsActions::StartDiscovery => {
                write!(f, "application-signals:StartDiscovery")
            }
            ApplicationSignalsActions::TagResource => write!(f, "application-signals:TagResource"),
            ApplicationSignalsActions::UntagResource => {
                write!(f, "application-signals:UntagResource")
            }
            ApplicationSignalsActions::UpdateServiceLevelObjective => {
                write!(f, "application-signals:UpdateServiceLevelObjective")
            }
        }
    }
}
