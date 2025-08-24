// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum RekognitionActions {
    AssociateFaces,
    CompareFaces,
    CopyProjectVersion,
    CreateCollection,
    CreateDataset,
    CreateFaceLivenessSession,
    CreateProject,
    CreateProjectVersion,
    CreateStreamProcessor,
    CreateUser,
    DeleteCollection,
    DeleteDataset,
    DeleteFaces,
    DeleteProject,
    DeleteProjectPolicy,
    DeleteProjectVersion,
    DeleteStreamProcessor,
    DeleteUser,
    DescribeCollection,
    DescribeDataset,
    DescribeProjectVersions,
    DescribeProjects,
    DescribeStreamProcessor,
    DetectCustomLabels,
    DetectFaces,
    DetectLabels,
    DetectModerationLabels,
    DetectProtectiveEquipment,
    DetectText,
    DisassociateFaces,
    DistributeDatasetEntries,
    GetCelebrityInfo,
    GetCelebrityRecognition,
    GetContentModeration,
    GetFaceDetection,
    GetFaceLivenessSessionResults,
    GetFaceSearch,
    GetLabelDetection,
    GetMediaAnalysisJob,
    GetPersonTracking,
    GetSegmentDetection,
    GetTextDetection,
    IndexFaces,
    ListCollections,
    ListDatasetEntries,
    ListDatasetLabels,
    ListFaces,
    ListMediaAnalysisJobs,
    ListProjectPolicies,
    ListStreamProcessors,
    ListTagsForResource,
    ListUsers,
    PutProjectPolicy,
    RecognizeCelebrities,
    SearchFaces,
    SearchFacesByImage,
    SearchUsers,
    SearchUsersByImage,
    StartCelebrityRecognition,
    StartContentModeration,
    StartFaceDetection,
    StartFaceLivenessSession,
    StartFaceSearch,
    StartLabelDetection,
    StartMediaAnalysisJob,
    StartPersonTracking,
    StartProjectVersion,
    StartSegmentDetection,
    StartStreamProcessor,
    StartTextDetection,
    StopProjectVersion,
    StopStreamProcessor,
    TagResource,
    UntagResource,
    UpdateDatasetEntries,
    UpdateStreamProcessor,
}
impl std::fmt::Display for RekognitionActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RekognitionActions::AssociateFaces => write!(f, "rekognition:AssociateFaces"),
            RekognitionActions::CompareFaces => write!(f, "rekognition:CompareFaces"),
            RekognitionActions::CopyProjectVersion => write!(f, "rekognition:CopyProjectVersion"),
            RekognitionActions::CreateCollection => write!(f, "rekognition:CreateCollection"),
            RekognitionActions::CreateDataset => write!(f, "rekognition:CreateDataset"),
            RekognitionActions::CreateFaceLivenessSession => {
                write!(f, "rekognition:CreateFaceLivenessSession")
            }
            RekognitionActions::CreateProject => write!(f, "rekognition:CreateProject"),
            RekognitionActions::CreateProjectVersion => {
                write!(f, "rekognition:CreateProjectVersion")
            }
            RekognitionActions::CreateStreamProcessor => {
                write!(f, "rekognition:CreateStreamProcessor")
            }
            RekognitionActions::CreateUser => write!(f, "rekognition:CreateUser"),
            RekognitionActions::DeleteCollection => write!(f, "rekognition:DeleteCollection"),
            RekognitionActions::DeleteDataset => write!(f, "rekognition:DeleteDataset"),
            RekognitionActions::DeleteFaces => write!(f, "rekognition:DeleteFaces"),
            RekognitionActions::DeleteProject => write!(f, "rekognition:DeleteProject"),
            RekognitionActions::DeleteProjectPolicy => write!(f, "rekognition:DeleteProjectPolicy"),
            RekognitionActions::DeleteProjectVersion => {
                write!(f, "rekognition:DeleteProjectVersion")
            }
            RekognitionActions::DeleteStreamProcessor => {
                write!(f, "rekognition:DeleteStreamProcessor")
            }
            RekognitionActions::DeleteUser => write!(f, "rekognition:DeleteUser"),
            RekognitionActions::DescribeCollection => write!(f, "rekognition:DescribeCollection"),
            RekognitionActions::DescribeDataset => write!(f, "rekognition:DescribeDataset"),
            RekognitionActions::DescribeProjectVersions => {
                write!(f, "rekognition:DescribeProjectVersions")
            }
            RekognitionActions::DescribeProjects => write!(f, "rekognition:DescribeProjects"),
            RekognitionActions::DescribeStreamProcessor => {
                write!(f, "rekognition:DescribeStreamProcessor")
            }
            RekognitionActions::DetectCustomLabels => write!(f, "rekognition:DetectCustomLabels"),
            RekognitionActions::DetectFaces => write!(f, "rekognition:DetectFaces"),
            RekognitionActions::DetectLabels => write!(f, "rekognition:DetectLabels"),
            RekognitionActions::DetectModerationLabels => {
                write!(f, "rekognition:DetectModerationLabels")
            }
            RekognitionActions::DetectProtectiveEquipment => {
                write!(f, "rekognition:DetectProtectiveEquipment")
            }
            RekognitionActions::DetectText => write!(f, "rekognition:DetectText"),
            RekognitionActions::DisassociateFaces => write!(f, "rekognition:DisassociateFaces"),
            RekognitionActions::DistributeDatasetEntries => {
                write!(f, "rekognition:DistributeDatasetEntries")
            }
            RekognitionActions::GetCelebrityInfo => write!(f, "rekognition:GetCelebrityInfo"),
            RekognitionActions::GetCelebrityRecognition => {
                write!(f, "rekognition:GetCelebrityRecognition")
            }
            RekognitionActions::GetContentModeration => {
                write!(f, "rekognition:GetContentModeration")
            }
            RekognitionActions::GetFaceDetection => write!(f, "rekognition:GetFaceDetection"),
            RekognitionActions::GetFaceLivenessSessionResults => {
                write!(f, "rekognition:GetFaceLivenessSessionResults")
            }
            RekognitionActions::GetFaceSearch => write!(f, "rekognition:GetFaceSearch"),
            RekognitionActions::GetLabelDetection => write!(f, "rekognition:GetLabelDetection"),
            RekognitionActions::GetMediaAnalysisJob => write!(f, "rekognition:GetMediaAnalysisJob"),
            RekognitionActions::GetPersonTracking => write!(f, "rekognition:GetPersonTracking"),
            RekognitionActions::GetSegmentDetection => write!(f, "rekognition:GetSegmentDetection"),
            RekognitionActions::GetTextDetection => write!(f, "rekognition:GetTextDetection"),
            RekognitionActions::IndexFaces => write!(f, "rekognition:IndexFaces"),
            RekognitionActions::ListCollections => write!(f, "rekognition:ListCollections"),
            RekognitionActions::ListDatasetEntries => write!(f, "rekognition:ListDatasetEntries"),
            RekognitionActions::ListDatasetLabels => write!(f, "rekognition:ListDatasetLabels"),
            RekognitionActions::ListFaces => write!(f, "rekognition:ListFaces"),
            RekognitionActions::ListMediaAnalysisJobs => {
                write!(f, "rekognition:ListMediaAnalysisJobs")
            }
            RekognitionActions::ListProjectPolicies => write!(f, "rekognition:ListProjectPolicies"),
            RekognitionActions::ListStreamProcessors => {
                write!(f, "rekognition:ListStreamProcessors")
            }
            RekognitionActions::ListTagsForResource => write!(f, "rekognition:ListTagsForResource"),
            RekognitionActions::ListUsers => write!(f, "rekognition:ListUsers"),
            RekognitionActions::PutProjectPolicy => write!(f, "rekognition:PutProjectPolicy"),
            RekognitionActions::RecognizeCelebrities => {
                write!(f, "rekognition:RecognizeCelebrities")
            }
            RekognitionActions::SearchFaces => write!(f, "rekognition:SearchFaces"),
            RekognitionActions::SearchFacesByImage => write!(f, "rekognition:SearchFacesByImage"),
            RekognitionActions::SearchUsers => write!(f, "rekognition:SearchUsers"),
            RekognitionActions::SearchUsersByImage => write!(f, "rekognition:SearchUsersByImage"),
            RekognitionActions::StartCelebrityRecognition => {
                write!(f, "rekognition:StartCelebrityRecognition")
            }
            RekognitionActions::StartContentModeration => {
                write!(f, "rekognition:StartContentModeration")
            }
            RekognitionActions::StartFaceDetection => write!(f, "rekognition:StartFaceDetection"),
            RekognitionActions::StartFaceLivenessSession => {
                write!(f, "rekognition:StartFaceLivenessSession")
            }
            RekognitionActions::StartFaceSearch => write!(f, "rekognition:StartFaceSearch"),
            RekognitionActions::StartLabelDetection => write!(f, "rekognition:StartLabelDetection"),
            RekognitionActions::StartMediaAnalysisJob => {
                write!(f, "rekognition:StartMediaAnalysisJob")
            }
            RekognitionActions::StartPersonTracking => write!(f, "rekognition:StartPersonTracking"),
            RekognitionActions::StartProjectVersion => write!(f, "rekognition:StartProjectVersion"),
            RekognitionActions::StartSegmentDetection => {
                write!(f, "rekognition:StartSegmentDetection")
            }
            RekognitionActions::StartStreamProcessor => {
                write!(f, "rekognition:StartStreamProcessor")
            }
            RekognitionActions::StartTextDetection => write!(f, "rekognition:StartTextDetection"),
            RekognitionActions::StopProjectVersion => write!(f, "rekognition:StopProjectVersion"),
            RekognitionActions::StopStreamProcessor => write!(f, "rekognition:StopStreamProcessor"),
            RekognitionActions::TagResource => write!(f, "rekognition:TagResource"),
            RekognitionActions::UntagResource => write!(f, "rekognition:UntagResource"),
            RekognitionActions::UpdateDatasetEntries => {
                write!(f, "rekognition:UpdateDatasetEntries")
            }
            RekognitionActions::UpdateStreamProcessor => {
                write!(f, "rekognition:UpdateStreamProcessor")
            }
        }
    }
}
