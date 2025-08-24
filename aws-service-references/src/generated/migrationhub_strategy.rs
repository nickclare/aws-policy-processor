// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum MigrationhubStrategyActions {
    GetAntiPattern,
    GetApplicationComponentDetails,
    GetApplicationComponentStrategies,
    GetAssessment,
    GetImportFileTask,
    GetLatestAssessmentId,
    GetMessage,
    GetPortfolioPreferences,
    GetPortfolioSummary,
    GetRecommendationReportDetails,
    GetServerDetails,
    GetServerStrategies,
    ListAnalyzableServers,
    ListAntiPatterns,
    ListApplicationComponents,
    ListCollectors,
    ListImportFileTask,
    ListJarArtifacts,
    ListServers,
    PutLogData,
    PutMetricData,
    PutPortfolioPreferences,
    RegisterCollector,
    SendMessage,
    StartAssessment,
    StartImportFileTask,
    StartRecommendationReportGeneration,
    StopAssessment,
    UpdateApplicationComponentConfig,
    UpdateCollectorConfiguration,
    UpdateServerConfig,
}
impl std::fmt::Display for MigrationhubStrategyActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MigrationhubStrategyActions::GetAntiPattern => {
                write!(f, "migrationhub-strategy:GetAntiPattern")
            }
            MigrationhubStrategyActions::GetApplicationComponentDetails => {
                write!(f, "migrationhub-strategy:GetApplicationComponentDetails")
            }
            MigrationhubStrategyActions::GetApplicationComponentStrategies => {
                write!(f, "migrationhub-strategy:GetApplicationComponentStrategies")
            }
            MigrationhubStrategyActions::GetAssessment => {
                write!(f, "migrationhub-strategy:GetAssessment")
            }
            MigrationhubStrategyActions::GetImportFileTask => {
                write!(f, "migrationhub-strategy:GetImportFileTask")
            }
            MigrationhubStrategyActions::GetLatestAssessmentId => {
                write!(f, "migrationhub-strategy:GetLatestAssessmentId")
            }
            MigrationhubStrategyActions::GetMessage => {
                write!(f, "migrationhub-strategy:GetMessage")
            }
            MigrationhubStrategyActions::GetPortfolioPreferences => {
                write!(f, "migrationhub-strategy:GetPortfolioPreferences")
            }
            MigrationhubStrategyActions::GetPortfolioSummary => {
                write!(f, "migrationhub-strategy:GetPortfolioSummary")
            }
            MigrationhubStrategyActions::GetRecommendationReportDetails => {
                write!(f, "migrationhub-strategy:GetRecommendationReportDetails")
            }
            MigrationhubStrategyActions::GetServerDetails => {
                write!(f, "migrationhub-strategy:GetServerDetails")
            }
            MigrationhubStrategyActions::GetServerStrategies => {
                write!(f, "migrationhub-strategy:GetServerStrategies")
            }
            MigrationhubStrategyActions::ListAnalyzableServers => {
                write!(f, "migrationhub-strategy:ListAnalyzableServers")
            }
            MigrationhubStrategyActions::ListAntiPatterns => {
                write!(f, "migrationhub-strategy:ListAntiPatterns")
            }
            MigrationhubStrategyActions::ListApplicationComponents => {
                write!(f, "migrationhub-strategy:ListApplicationComponents")
            }
            MigrationhubStrategyActions::ListCollectors => {
                write!(f, "migrationhub-strategy:ListCollectors")
            }
            MigrationhubStrategyActions::ListImportFileTask => {
                write!(f, "migrationhub-strategy:ListImportFileTask")
            }
            MigrationhubStrategyActions::ListJarArtifacts => {
                write!(f, "migrationhub-strategy:ListJarArtifacts")
            }
            MigrationhubStrategyActions::ListServers => {
                write!(f, "migrationhub-strategy:ListServers")
            }
            MigrationhubStrategyActions::PutLogData => {
                write!(f, "migrationhub-strategy:PutLogData")
            }
            MigrationhubStrategyActions::PutMetricData => {
                write!(f, "migrationhub-strategy:PutMetricData")
            }
            MigrationhubStrategyActions::PutPortfolioPreferences => {
                write!(f, "migrationhub-strategy:PutPortfolioPreferences")
            }
            MigrationhubStrategyActions::RegisterCollector => {
                write!(f, "migrationhub-strategy:RegisterCollector")
            }
            MigrationhubStrategyActions::SendMessage => {
                write!(f, "migrationhub-strategy:SendMessage")
            }
            MigrationhubStrategyActions::StartAssessment => {
                write!(f, "migrationhub-strategy:StartAssessment")
            }
            MigrationhubStrategyActions::StartImportFileTask => {
                write!(f, "migrationhub-strategy:StartImportFileTask")
            }
            MigrationhubStrategyActions::StartRecommendationReportGeneration => write!(
                f,
                "migrationhub-strategy:StartRecommendationReportGeneration"
            ),
            MigrationhubStrategyActions::StopAssessment => {
                write!(f, "migrationhub-strategy:StopAssessment")
            }
            MigrationhubStrategyActions::UpdateApplicationComponentConfig => {
                write!(f, "migrationhub-strategy:UpdateApplicationComponentConfig")
            }
            MigrationhubStrategyActions::UpdateCollectorConfiguration => {
                write!(f, "migrationhub-strategy:UpdateCollectorConfiguration")
            }
            MigrationhubStrategyActions::UpdateServerConfig => {
                write!(f, "migrationhub-strategy:UpdateServerConfig")
            }
        }
    }
}
