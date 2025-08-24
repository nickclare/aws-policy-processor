// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum ComprehendmedicalActions {
    DescribeEntitiesDetectionV2Job,
    DescribeIcd10cmInferenceJob,
    DescribePhiDetectionJob,
    DescribeRxNormInferenceJob,
    DescribeSnomedctInferenceJob,
    DetectEntitiesV2,
    DetectPhi,
    InferIcd10cm,
    InferRxNorm,
    InferSnomedct,
    ListEntitiesDetectionV2Jobs,
    ListIcd10cmInferenceJobs,
    ListPhiDetectionJobs,
    ListRxNormInferenceJobs,
    ListSnomedctInferenceJobs,
    StartEntitiesDetectionV2Job,
    StartIcd10cmInferenceJob,
    StartPhiDetectionJob,
    StartRxNormInferenceJob,
    StartSnomedctInferenceJob,
    StopEntitiesDetectionV2Job,
    StopIcd10cmInferenceJob,
    StopPhiDetectionJob,
    StopRxNormInferenceJob,
    StopSnomedctInferenceJob,
}
impl std::fmt::Display for ComprehendmedicalActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ComprehendmedicalActions::DescribeEntitiesDetectionV2Job => {
                write!(f, "comprehendmedical:DescribeEntitiesDetectionV2Job")
            }
            ComprehendmedicalActions::DescribeIcd10cmInferenceJob => {
                write!(f, "comprehendmedical:DescribeICD10CMInferenceJob")
            }
            ComprehendmedicalActions::DescribePhiDetectionJob => {
                write!(f, "comprehendmedical:DescribePHIDetectionJob")
            }
            ComprehendmedicalActions::DescribeRxNormInferenceJob => {
                write!(f, "comprehendmedical:DescribeRxNormInferenceJob")
            }
            ComprehendmedicalActions::DescribeSnomedctInferenceJob => {
                write!(f, "comprehendmedical:DescribeSNOMEDCTInferenceJob")
            }
            ComprehendmedicalActions::DetectEntitiesV2 => {
                write!(f, "comprehendmedical:DetectEntitiesV2")
            }
            ComprehendmedicalActions::DetectPhi => write!(f, "comprehendmedical:DetectPHI"),
            ComprehendmedicalActions::InferIcd10cm => write!(f, "comprehendmedical:InferICD10CM"),
            ComprehendmedicalActions::InferRxNorm => write!(f, "comprehendmedical:InferRxNorm"),
            ComprehendmedicalActions::InferSnomedct => write!(f, "comprehendmedical:InferSNOMEDCT"),
            ComprehendmedicalActions::ListEntitiesDetectionV2Jobs => {
                write!(f, "comprehendmedical:ListEntitiesDetectionV2Jobs")
            }
            ComprehendmedicalActions::ListIcd10cmInferenceJobs => {
                write!(f, "comprehendmedical:ListICD10CMInferenceJobs")
            }
            ComprehendmedicalActions::ListPhiDetectionJobs => {
                write!(f, "comprehendmedical:ListPHIDetectionJobs")
            }
            ComprehendmedicalActions::ListRxNormInferenceJobs => {
                write!(f, "comprehendmedical:ListRxNormInferenceJobs")
            }
            ComprehendmedicalActions::ListSnomedctInferenceJobs => {
                write!(f, "comprehendmedical:ListSNOMEDCTInferenceJobs")
            }
            ComprehendmedicalActions::StartEntitiesDetectionV2Job => {
                write!(f, "comprehendmedical:StartEntitiesDetectionV2Job")
            }
            ComprehendmedicalActions::StartIcd10cmInferenceJob => {
                write!(f, "comprehendmedical:StartICD10CMInferenceJob")
            }
            ComprehendmedicalActions::StartPhiDetectionJob => {
                write!(f, "comprehendmedical:StartPHIDetectionJob")
            }
            ComprehendmedicalActions::StartRxNormInferenceJob => {
                write!(f, "comprehendmedical:StartRxNormInferenceJob")
            }
            ComprehendmedicalActions::StartSnomedctInferenceJob => {
                write!(f, "comprehendmedical:StartSNOMEDCTInferenceJob")
            }
            ComprehendmedicalActions::StopEntitiesDetectionV2Job => {
                write!(f, "comprehendmedical:StopEntitiesDetectionV2Job")
            }
            ComprehendmedicalActions::StopIcd10cmInferenceJob => {
                write!(f, "comprehendmedical:StopICD10CMInferenceJob")
            }
            ComprehendmedicalActions::StopPhiDetectionJob => {
                write!(f, "comprehendmedical:StopPHIDetectionJob")
            }
            ComprehendmedicalActions::StopRxNormInferenceJob => {
                write!(f, "comprehendmedical:StopRxNormInferenceJob")
            }
            ComprehendmedicalActions::StopSnomedctInferenceJob => {
                write!(f, "comprehendmedical:StopSNOMEDCTInferenceJob")
            }
        }
    }
}
