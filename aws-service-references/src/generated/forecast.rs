// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum ForecastActions {
    CreateAutoPredictor,
    CreateDataset,
    CreateDatasetGroup,
    CreateDatasetImportJob,
    CreateExplainability,
    CreateExplainabilityExport,
    CreateForecast,
    CreateForecastEndpoint,
    CreateForecastExportJob,
    CreateMonitor,
    CreatePredictor,
    CreatePredictorBacktestExportJob,
    CreateWhatIfAnalysis,
    CreateWhatIfForecast,
    CreateWhatIfForecastExport,
    DeleteDataset,
    DeleteDatasetGroup,
    DeleteDatasetImportJob,
    DeleteExplainability,
    DeleteExplainabilityExport,
    DeleteForecast,
    DeleteForecastEndpoint,
    DeleteForecastExportJob,
    DeleteMonitor,
    DeletePredictor,
    DeletePredictorBacktestExportJob,
    DeleteResourceTree,
    DeleteWhatIfAnalysis,
    DeleteWhatIfForecast,
    DeleteWhatIfForecastExport,
    DescribeAutoPredictor,
    DescribeDataset,
    DescribeDatasetGroup,
    DescribeDatasetImportJob,
    DescribeExplainability,
    DescribeExplainabilityExport,
    DescribeForecast,
    DescribeForecastEndpoint,
    DescribeForecastExportJob,
    DescribeMonitor,
    DescribePredictor,
    DescribePredictorBacktestExportJob,
    DescribeWhatIfAnalysis,
    DescribeWhatIfForecast,
    DescribeWhatIfForecastExport,
    GetAccuracyMetrics,
    GetRecentForecastContext,
    InvokeForecastEndpoint,
    ListDatasetGroups,
    ListDatasetImportJobs,
    ListDatasets,
    ListExplainabilities,
    ListExplainabilityExports,
    ListForecastExportJobs,
    ListForecasts,
    ListMonitorEvaluations,
    ListMonitors,
    ListPredictorBacktestExportJobs,
    ListPredictors,
    ListTagsForResource,
    ListWhatIfAnalyses,
    ListWhatIfForecastExports,
    ListWhatIfForecasts,
    QueryForecast,
    QueryWhatIfForecast,
    ResumeResource,
    StopResource,
    TagResource,
    UntagResource,
    UpdateDatasetGroup,
}
impl std::fmt::Display for ForecastActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ForecastActions::CreateAutoPredictor => write!(f, "forecast:CreateAutoPredictor"),
            ForecastActions::CreateDataset => write!(f, "forecast:CreateDataset"),
            ForecastActions::CreateDatasetGroup => write!(f, "forecast:CreateDatasetGroup"),
            ForecastActions::CreateDatasetImportJob => write!(f, "forecast:CreateDatasetImportJob"),
            ForecastActions::CreateExplainability => write!(f, "forecast:CreateExplainability"),
            ForecastActions::CreateExplainabilityExport => {
                write!(f, "forecast:CreateExplainabilityExport")
            }
            ForecastActions::CreateForecast => write!(f, "forecast:CreateForecast"),
            ForecastActions::CreateForecastEndpoint => write!(f, "forecast:CreateForecastEndpoint"),
            ForecastActions::CreateForecastExportJob => {
                write!(f, "forecast:CreateForecastExportJob")
            }
            ForecastActions::CreateMonitor => write!(f, "forecast:CreateMonitor"),
            ForecastActions::CreatePredictor => write!(f, "forecast:CreatePredictor"),
            ForecastActions::CreatePredictorBacktestExportJob => {
                write!(f, "forecast:CreatePredictorBacktestExportJob")
            }
            ForecastActions::CreateWhatIfAnalysis => write!(f, "forecast:CreateWhatIfAnalysis"),
            ForecastActions::CreateWhatIfForecast => write!(f, "forecast:CreateWhatIfForecast"),
            ForecastActions::CreateWhatIfForecastExport => {
                write!(f, "forecast:CreateWhatIfForecastExport")
            }
            ForecastActions::DeleteDataset => write!(f, "forecast:DeleteDataset"),
            ForecastActions::DeleteDatasetGroup => write!(f, "forecast:DeleteDatasetGroup"),
            ForecastActions::DeleteDatasetImportJob => write!(f, "forecast:DeleteDatasetImportJob"),
            ForecastActions::DeleteExplainability => write!(f, "forecast:DeleteExplainability"),
            ForecastActions::DeleteExplainabilityExport => {
                write!(f, "forecast:DeleteExplainabilityExport")
            }
            ForecastActions::DeleteForecast => write!(f, "forecast:DeleteForecast"),
            ForecastActions::DeleteForecastEndpoint => write!(f, "forecast:DeleteForecastEndpoint"),
            ForecastActions::DeleteForecastExportJob => {
                write!(f, "forecast:DeleteForecastExportJob")
            }
            ForecastActions::DeleteMonitor => write!(f, "forecast:DeleteMonitor"),
            ForecastActions::DeletePredictor => write!(f, "forecast:DeletePredictor"),
            ForecastActions::DeletePredictorBacktestExportJob => {
                write!(f, "forecast:DeletePredictorBacktestExportJob")
            }
            ForecastActions::DeleteResourceTree => write!(f, "forecast:DeleteResourceTree"),
            ForecastActions::DeleteWhatIfAnalysis => write!(f, "forecast:DeleteWhatIfAnalysis"),
            ForecastActions::DeleteWhatIfForecast => write!(f, "forecast:DeleteWhatIfForecast"),
            ForecastActions::DeleteWhatIfForecastExport => {
                write!(f, "forecast:DeleteWhatIfForecastExport")
            }
            ForecastActions::DescribeAutoPredictor => write!(f, "forecast:DescribeAutoPredictor"),
            ForecastActions::DescribeDataset => write!(f, "forecast:DescribeDataset"),
            ForecastActions::DescribeDatasetGroup => write!(f, "forecast:DescribeDatasetGroup"),
            ForecastActions::DescribeDatasetImportJob => {
                write!(f, "forecast:DescribeDatasetImportJob")
            }
            ForecastActions::DescribeExplainability => write!(f, "forecast:DescribeExplainability"),
            ForecastActions::DescribeExplainabilityExport => {
                write!(f, "forecast:DescribeExplainabilityExport")
            }
            ForecastActions::DescribeForecast => write!(f, "forecast:DescribeForecast"),
            ForecastActions::DescribeForecastEndpoint => {
                write!(f, "forecast:DescribeForecastEndpoint")
            }
            ForecastActions::DescribeForecastExportJob => {
                write!(f, "forecast:DescribeForecastExportJob")
            }
            ForecastActions::DescribeMonitor => write!(f, "forecast:DescribeMonitor"),
            ForecastActions::DescribePredictor => write!(f, "forecast:DescribePredictor"),
            ForecastActions::DescribePredictorBacktestExportJob => {
                write!(f, "forecast:DescribePredictorBacktestExportJob")
            }
            ForecastActions::DescribeWhatIfAnalysis => write!(f, "forecast:DescribeWhatIfAnalysis"),
            ForecastActions::DescribeWhatIfForecast => write!(f, "forecast:DescribeWhatIfForecast"),
            ForecastActions::DescribeWhatIfForecastExport => {
                write!(f, "forecast:DescribeWhatIfForecastExport")
            }
            ForecastActions::GetAccuracyMetrics => write!(f, "forecast:GetAccuracyMetrics"),
            ForecastActions::GetRecentForecastContext => {
                write!(f, "forecast:GetRecentForecastContext")
            }
            ForecastActions::InvokeForecastEndpoint => write!(f, "forecast:InvokeForecastEndpoint"),
            ForecastActions::ListDatasetGroups => write!(f, "forecast:ListDatasetGroups"),
            ForecastActions::ListDatasetImportJobs => write!(f, "forecast:ListDatasetImportJobs"),
            ForecastActions::ListDatasets => write!(f, "forecast:ListDatasets"),
            ForecastActions::ListExplainabilities => write!(f, "forecast:ListExplainabilities"),
            ForecastActions::ListExplainabilityExports => {
                write!(f, "forecast:ListExplainabilityExports")
            }
            ForecastActions::ListForecastExportJobs => write!(f, "forecast:ListForecastExportJobs"),
            ForecastActions::ListForecasts => write!(f, "forecast:ListForecasts"),
            ForecastActions::ListMonitorEvaluations => write!(f, "forecast:ListMonitorEvaluations"),
            ForecastActions::ListMonitors => write!(f, "forecast:ListMonitors"),
            ForecastActions::ListPredictorBacktestExportJobs => {
                write!(f, "forecast:ListPredictorBacktestExportJobs")
            }
            ForecastActions::ListPredictors => write!(f, "forecast:ListPredictors"),
            ForecastActions::ListTagsForResource => write!(f, "forecast:ListTagsForResource"),
            ForecastActions::ListWhatIfAnalyses => write!(f, "forecast:ListWhatIfAnalyses"),
            ForecastActions::ListWhatIfForecastExports => {
                write!(f, "forecast:ListWhatIfForecastExports")
            }
            ForecastActions::ListWhatIfForecasts => write!(f, "forecast:ListWhatIfForecasts"),
            ForecastActions::QueryForecast => write!(f, "forecast:QueryForecast"),
            ForecastActions::QueryWhatIfForecast => write!(f, "forecast:QueryWhatIfForecast"),
            ForecastActions::ResumeResource => write!(f, "forecast:ResumeResource"),
            ForecastActions::StopResource => write!(f, "forecast:StopResource"),
            ForecastActions::TagResource => write!(f, "forecast:TagResource"),
            ForecastActions::UntagResource => write!(f, "forecast:UntagResource"),
            ForecastActions::UpdateDatasetGroup => write!(f, "forecast:UpdateDatasetGroup"),
        }
    }
}
