// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum SagemakerGeospatialActions {
    DeleteEarthObservationJob,
    DeleteVectorEnrichmentJob,
    ExportEarthObservationJob,
    ExportVectorEnrichmentJob,
    GetEarthObservationJob,
    GetRasterDataCollection,
    GetTile,
    GetVectorEnrichmentJob,
    ListEarthObservationJobs,
    ListRasterDataCollections,
    ListTagsForResource,
    ListVectorEnrichmentJobs,
    SearchRasterDataCollection,
    StartEarthObservationJob,
    StartVectorEnrichmentJob,
    StopEarthObservationJob,
    StopVectorEnrichmentJob,
    TagResource,
    UntagResource,
}
impl std::fmt::Display for SagemakerGeospatialActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SagemakerGeospatialActions::DeleteEarthObservationJob => {
                write!(f, "sagemaker-geospatial:DeleteEarthObservationJob")
            }
            SagemakerGeospatialActions::DeleteVectorEnrichmentJob => {
                write!(f, "sagemaker-geospatial:DeleteVectorEnrichmentJob")
            }
            SagemakerGeospatialActions::ExportEarthObservationJob => {
                write!(f, "sagemaker-geospatial:ExportEarthObservationJob")
            }
            SagemakerGeospatialActions::ExportVectorEnrichmentJob => {
                write!(f, "sagemaker-geospatial:ExportVectorEnrichmentJob")
            }
            SagemakerGeospatialActions::GetEarthObservationJob => {
                write!(f, "sagemaker-geospatial:GetEarthObservationJob")
            }
            SagemakerGeospatialActions::GetRasterDataCollection => {
                write!(f, "sagemaker-geospatial:GetRasterDataCollection")
            }
            SagemakerGeospatialActions::GetTile => write!(f, "sagemaker-geospatial:GetTile"),
            SagemakerGeospatialActions::GetVectorEnrichmentJob => {
                write!(f, "sagemaker-geospatial:GetVectorEnrichmentJob")
            }
            SagemakerGeospatialActions::ListEarthObservationJobs => {
                write!(f, "sagemaker-geospatial:ListEarthObservationJobs")
            }
            SagemakerGeospatialActions::ListRasterDataCollections => {
                write!(f, "sagemaker-geospatial:ListRasterDataCollections")
            }
            SagemakerGeospatialActions::ListTagsForResource => {
                write!(f, "sagemaker-geospatial:ListTagsForResource")
            }
            SagemakerGeospatialActions::ListVectorEnrichmentJobs => {
                write!(f, "sagemaker-geospatial:ListVectorEnrichmentJobs")
            }
            SagemakerGeospatialActions::SearchRasterDataCollection => {
                write!(f, "sagemaker-geospatial:SearchRasterDataCollection")
            }
            SagemakerGeospatialActions::StartEarthObservationJob => {
                write!(f, "sagemaker-geospatial:StartEarthObservationJob")
            }
            SagemakerGeospatialActions::StartVectorEnrichmentJob => {
                write!(f, "sagemaker-geospatial:StartVectorEnrichmentJob")
            }
            SagemakerGeospatialActions::StopEarthObservationJob => {
                write!(f, "sagemaker-geospatial:StopEarthObservationJob")
            }
            SagemakerGeospatialActions::StopVectorEnrichmentJob => {
                write!(f, "sagemaker-geospatial:StopVectorEnrichmentJob")
            }
            SagemakerGeospatialActions::TagResource => {
                write!(f, "sagemaker-geospatial:TagResource")
            }
            SagemakerGeospatialActions::UntagResource => {
                write!(f, "sagemaker-geospatial:UntagResource")
            }
        }
    }
}
