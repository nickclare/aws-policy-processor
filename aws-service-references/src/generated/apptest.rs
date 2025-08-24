// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum ApptestActions {
    CreateTestCase,
    CreateTestConfiguration,
    CreateTestSuite,
    DeleteTestCase,
    DeleteTestConfiguration,
    DeleteTestRun,
    DeleteTestSuite,
    GetTestCase,
    GetTestConfiguration,
    GetTestRunStep,
    GetTestSuite,
    ListTagsForResource,
    ListTestCases,
    ListTestConfigurations,
    ListTestRunSteps,
    ListTestRunTestCases,
    ListTestRuns,
    ListTestSuites,
    StartTestRun,
    TagResource,
    UntagResource,
    UpdateTestCase,
    UpdateTestConfiguration,
    UpdateTestSuite,
}
impl std::fmt::Display for ApptestActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApptestActions::CreateTestCase => write!(f, "apptest:CreateTestCase"),
            ApptestActions::CreateTestConfiguration => write!(f, "apptest:CreateTestConfiguration"),
            ApptestActions::CreateTestSuite => write!(f, "apptest:CreateTestSuite"),
            ApptestActions::DeleteTestCase => write!(f, "apptest:DeleteTestCase"),
            ApptestActions::DeleteTestConfiguration => write!(f, "apptest:DeleteTestConfiguration"),
            ApptestActions::DeleteTestRun => write!(f, "apptest:DeleteTestRun"),
            ApptestActions::DeleteTestSuite => write!(f, "apptest:DeleteTestSuite"),
            ApptestActions::GetTestCase => write!(f, "apptest:GetTestCase"),
            ApptestActions::GetTestConfiguration => write!(f, "apptest:GetTestConfiguration"),
            ApptestActions::GetTestRunStep => write!(f, "apptest:GetTestRunStep"),
            ApptestActions::GetTestSuite => write!(f, "apptest:GetTestSuite"),
            ApptestActions::ListTagsForResource => write!(f, "apptest:ListTagsForResource"),
            ApptestActions::ListTestCases => write!(f, "apptest:ListTestCases"),
            ApptestActions::ListTestConfigurations => write!(f, "apptest:ListTestConfigurations"),
            ApptestActions::ListTestRunSteps => write!(f, "apptest:ListTestRunSteps"),
            ApptestActions::ListTestRunTestCases => write!(f, "apptest:ListTestRunTestCases"),
            ApptestActions::ListTestRuns => write!(f, "apptest:ListTestRuns"),
            ApptestActions::ListTestSuites => write!(f, "apptest:ListTestSuites"),
            ApptestActions::StartTestRun => write!(f, "apptest:StartTestRun"),
            ApptestActions::TagResource => write!(f, "apptest:TagResource"),
            ApptestActions::UntagResource => write!(f, "apptest:UntagResource"),
            ApptestActions::UpdateTestCase => write!(f, "apptest:UpdateTestCase"),
            ApptestActions::UpdateTestConfiguration => write!(f, "apptest:UpdateTestConfiguration"),
            ApptestActions::UpdateTestSuite => write!(f, "apptest:UpdateTestSuite"),
        }
    }
}
