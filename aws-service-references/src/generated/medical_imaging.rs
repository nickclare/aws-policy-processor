// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum MedicalImagingActions {
    CopyImageSet,
    CreateDatastore,
    DeleteDatastore,
    DeleteImageSet,
    GetDatastore,
    GetDicomBulkdata,
    GetDicomImportJob,
    GetDicomInstance,
    GetDicomInstanceFrames,
    GetDicomInstanceMetadata,
    GetDicomSeriesMetadata,
    GetImageFrame,
    GetImageSet,
    GetImageSetMetadata,
    ListDatastores,
    ListDicomImportJobs,
    ListImageSetVersions,
    ListTagsForResource,
    SearchDicomInstances,
    SearchDicomSeries,
    SearchDicomStudies,
    SearchImageSets,
    StartDicomImportJob,
    StoreDicom,
    StoreDicomStudy,
    TagResource,
    UntagResource,
    UpdateImageSetMetadata,
}
impl std::fmt::Display for MedicalImagingActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MedicalImagingActions::CopyImageSet => write!(f, "medical-imaging:CopyImageSet"),
            MedicalImagingActions::CreateDatastore => write!(f, "medical-imaging:CreateDatastore"),
            MedicalImagingActions::DeleteDatastore => write!(f, "medical-imaging:DeleteDatastore"),
            MedicalImagingActions::DeleteImageSet => write!(f, "medical-imaging:DeleteImageSet"),
            MedicalImagingActions::GetDatastore => write!(f, "medical-imaging:GetDatastore"),
            MedicalImagingActions::GetDicomBulkdata => {
                write!(f, "medical-imaging:GetDICOMBulkdata")
            }
            MedicalImagingActions::GetDicomImportJob => {
                write!(f, "medical-imaging:GetDICOMImportJob")
            }
            MedicalImagingActions::GetDicomInstance => {
                write!(f, "medical-imaging:GetDICOMInstance")
            }
            MedicalImagingActions::GetDicomInstanceFrames => {
                write!(f, "medical-imaging:GetDICOMInstanceFrames")
            }
            MedicalImagingActions::GetDicomInstanceMetadata => {
                write!(f, "medical-imaging:GetDICOMInstanceMetadata")
            }
            MedicalImagingActions::GetDicomSeriesMetadata => {
                write!(f, "medical-imaging:GetDICOMSeriesMetadata")
            }
            MedicalImagingActions::GetImageFrame => write!(f, "medical-imaging:GetImageFrame"),
            MedicalImagingActions::GetImageSet => write!(f, "medical-imaging:GetImageSet"),
            MedicalImagingActions::GetImageSetMetadata => {
                write!(f, "medical-imaging:GetImageSetMetadata")
            }
            MedicalImagingActions::ListDatastores => write!(f, "medical-imaging:ListDatastores"),
            MedicalImagingActions::ListDicomImportJobs => {
                write!(f, "medical-imaging:ListDICOMImportJobs")
            }
            MedicalImagingActions::ListImageSetVersions => {
                write!(f, "medical-imaging:ListImageSetVersions")
            }
            MedicalImagingActions::ListTagsForResource => {
                write!(f, "medical-imaging:ListTagsForResource")
            }
            MedicalImagingActions::SearchDicomInstances => {
                write!(f, "medical-imaging:SearchDICOMInstances")
            }
            MedicalImagingActions::SearchDicomSeries => {
                write!(f, "medical-imaging:SearchDICOMSeries")
            }
            MedicalImagingActions::SearchDicomStudies => {
                write!(f, "medical-imaging:SearchDICOMStudies")
            }
            MedicalImagingActions::SearchImageSets => write!(f, "medical-imaging:SearchImageSets"),
            MedicalImagingActions::StartDicomImportJob => {
                write!(f, "medical-imaging:StartDICOMImportJob")
            }
            MedicalImagingActions::StoreDicom => write!(f, "medical-imaging:StoreDICOM"),
            MedicalImagingActions::StoreDicomStudy => write!(f, "medical-imaging:StoreDICOMStudy"),
            MedicalImagingActions::TagResource => write!(f, "medical-imaging:TagResource"),
            MedicalImagingActions::UntagResource => write!(f, "medical-imaging:UntagResource"),
            MedicalImagingActions::UpdateImageSetMetadata => {
                write!(f, "medical-imaging:UpdateImageSetMetadata")
            }
        }
    }
}
