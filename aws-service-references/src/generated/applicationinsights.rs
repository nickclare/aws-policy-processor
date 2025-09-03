// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum ApplicationinsightsActions {
    AddWorkload,
    CreateApplication,
    CreateComponent,
    CreateLogPattern,
    DeleteApplication,
    DeleteComponent,
    DeleteLogPattern,
    DescribeApplication,
    DescribeComponent,
    DescribeComponentConfiguration,
    DescribeComponentConfigurationRecommendation,
    DescribeLogPattern,
    DescribeObservation,
    DescribeProblem,
    DescribeProblemObservations,
    DescribeWorkload,
    Link,
    ListApplications,
    ListComponents,
    ListConfigurationHistory,
    ListLogPatternSets,
    ListLogPatterns,
    ListProblems,
    ListTagsForResource,
    ListWorkloads,
    RemoveWorkload,
    TagResource,
    UntagResource,
    UpdateApplication,
    UpdateComponent,
    UpdateComponentConfiguration,
    UpdateLogPattern,
    UpdateProblem,
    UpdateWorkload,
}
impl std::fmt::Display for ApplicationinsightsActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApplicationinsightsActions::AddWorkload => write!(f, "applicationinsights:AddWorkload"),
            ApplicationinsightsActions::CreateApplication => {
                write!(f, "applicationinsights:CreateApplication")
            }
            ApplicationinsightsActions::CreateComponent => {
                write!(f, "applicationinsights:CreateComponent")
            }
            ApplicationinsightsActions::CreateLogPattern => {
                write!(f, "applicationinsights:CreateLogPattern")
            }
            ApplicationinsightsActions::DeleteApplication => {
                write!(f, "applicationinsights:DeleteApplication")
            }
            ApplicationinsightsActions::DeleteComponent => {
                write!(f, "applicationinsights:DeleteComponent")
            }
            ApplicationinsightsActions::DeleteLogPattern => {
                write!(f, "applicationinsights:DeleteLogPattern")
            }
            ApplicationinsightsActions::DescribeApplication => {
                write!(f, "applicationinsights:DescribeApplication")
            }
            ApplicationinsightsActions::DescribeComponent => {
                write!(f, "applicationinsights:DescribeComponent")
            }
            ApplicationinsightsActions::DescribeComponentConfiguration => {
                write!(f, "applicationinsights:DescribeComponentConfiguration")
            }
            ApplicationinsightsActions::DescribeComponentConfigurationRecommendation => write!(
                f,
                "applicationinsights:DescribeComponentConfigurationRecommendation"
            ),
            ApplicationinsightsActions::DescribeLogPattern => {
                write!(f, "applicationinsights:DescribeLogPattern")
            }
            ApplicationinsightsActions::DescribeObservation => {
                write!(f, "applicationinsights:DescribeObservation")
            }
            ApplicationinsightsActions::DescribeProblem => {
                write!(f, "applicationinsights:DescribeProblem")
            }
            ApplicationinsightsActions::DescribeProblemObservations => {
                write!(f, "applicationinsights:DescribeProblemObservations")
            }
            ApplicationinsightsActions::DescribeWorkload => {
                write!(f, "applicationinsights:DescribeWorkload")
            }
            ApplicationinsightsActions::Link => write!(f, "applicationinsights:Link"),
            ApplicationinsightsActions::ListApplications => {
                write!(f, "applicationinsights:ListApplications")
            }
            ApplicationinsightsActions::ListComponents => {
                write!(f, "applicationinsights:ListComponents")
            }
            ApplicationinsightsActions::ListConfigurationHistory => {
                write!(f, "applicationinsights:ListConfigurationHistory")
            }
            ApplicationinsightsActions::ListLogPatternSets => {
                write!(f, "applicationinsights:ListLogPatternSets")
            }
            ApplicationinsightsActions::ListLogPatterns => {
                write!(f, "applicationinsights:ListLogPatterns")
            }
            ApplicationinsightsActions::ListProblems => {
                write!(f, "applicationinsights:ListProblems")
            }
            ApplicationinsightsActions::ListTagsForResource => {
                write!(f, "applicationinsights:ListTagsForResource")
            }
            ApplicationinsightsActions::ListWorkloads => {
                write!(f, "applicationinsights:ListWorkloads")
            }
            ApplicationinsightsActions::RemoveWorkload => {
                write!(f, "applicationinsights:RemoveWorkload")
            }
            ApplicationinsightsActions::TagResource => write!(f, "applicationinsights:TagResource"),
            ApplicationinsightsActions::UntagResource => {
                write!(f, "applicationinsights:UntagResource")
            }
            ApplicationinsightsActions::UpdateApplication => {
                write!(f, "applicationinsights:UpdateApplication")
            }
            ApplicationinsightsActions::UpdateComponent => {
                write!(f, "applicationinsights:UpdateComponent")
            }
            ApplicationinsightsActions::UpdateComponentConfiguration => {
                write!(f, "applicationinsights:UpdateComponentConfiguration")
            }
            ApplicationinsightsActions::UpdateLogPattern => {
                write!(f, "applicationinsights:UpdateLogPattern")
            }
            ApplicationinsightsActions::UpdateProblem => {
                write!(f, "applicationinsights:UpdateProblem")
            }
            ApplicationinsightsActions::UpdateWorkload => {
                write!(f, "applicationinsights:UpdateWorkload")
            }
        }
    }
}
