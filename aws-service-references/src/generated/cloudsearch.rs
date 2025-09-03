// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum CloudsearchActions {
    AddTags,
    BuildSuggesters,
    CreateDomain,
    DefineAnalysisScheme,
    DefineExpression,
    DefineIndexField,
    DefineSuggester,
    DeleteAnalysisScheme,
    DeleteDomain,
    DeleteExpression,
    DeleteIndexField,
    DeleteSuggester,
    DescribeAnalysisSchemes,
    DescribeAvailabilityOptions,
    DescribeDomainEndpointOptions,
    DescribeDomains,
    DescribeExpressions,
    DescribeIndexFields,
    DescribeScalingParameters,
    DescribeServiceAccessPolicies,
    DescribeSuggesters,
    Document,
    IndexDocuments,
    ListDomainNames,
    ListTags,
    RemoveTags,
    Search,
    Suggest,
    UpdateAvailabilityOptions,
    UpdateDomainEndpointOptions,
    UpdateScalingParameters,
    UpdateServiceAccessPolicies,
}
impl std::fmt::Display for CloudsearchActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CloudsearchActions::AddTags => write!(f, "cloudsearch:AddTags"),
            CloudsearchActions::BuildSuggesters => write!(f, "cloudsearch:BuildSuggesters"),
            CloudsearchActions::CreateDomain => write!(f, "cloudsearch:CreateDomain"),
            CloudsearchActions::DefineAnalysisScheme => {
                write!(f, "cloudsearch:DefineAnalysisScheme")
            }
            CloudsearchActions::DefineExpression => write!(f, "cloudsearch:DefineExpression"),
            CloudsearchActions::DefineIndexField => write!(f, "cloudsearch:DefineIndexField"),
            CloudsearchActions::DefineSuggester => write!(f, "cloudsearch:DefineSuggester"),
            CloudsearchActions::DeleteAnalysisScheme => {
                write!(f, "cloudsearch:DeleteAnalysisScheme")
            }
            CloudsearchActions::DeleteDomain => write!(f, "cloudsearch:DeleteDomain"),
            CloudsearchActions::DeleteExpression => write!(f, "cloudsearch:DeleteExpression"),
            CloudsearchActions::DeleteIndexField => write!(f, "cloudsearch:DeleteIndexField"),
            CloudsearchActions::DeleteSuggester => write!(f, "cloudsearch:DeleteSuggester"),
            CloudsearchActions::DescribeAnalysisSchemes => {
                write!(f, "cloudsearch:DescribeAnalysisSchemes")
            }
            CloudsearchActions::DescribeAvailabilityOptions => {
                write!(f, "cloudsearch:DescribeAvailabilityOptions")
            }
            CloudsearchActions::DescribeDomainEndpointOptions => {
                write!(f, "cloudsearch:DescribeDomainEndpointOptions")
            }
            CloudsearchActions::DescribeDomains => write!(f, "cloudsearch:DescribeDomains"),
            CloudsearchActions::DescribeExpressions => write!(f, "cloudsearch:DescribeExpressions"),
            CloudsearchActions::DescribeIndexFields => write!(f, "cloudsearch:DescribeIndexFields"),
            CloudsearchActions::DescribeScalingParameters => {
                write!(f, "cloudsearch:DescribeScalingParameters")
            }
            CloudsearchActions::DescribeServiceAccessPolicies => {
                write!(f, "cloudsearch:DescribeServiceAccessPolicies")
            }
            CloudsearchActions::DescribeSuggesters => write!(f, "cloudsearch:DescribeSuggesters"),
            CloudsearchActions::Document => write!(f, "cloudsearch:document"),
            CloudsearchActions::IndexDocuments => write!(f, "cloudsearch:IndexDocuments"),
            CloudsearchActions::ListDomainNames => write!(f, "cloudsearch:ListDomainNames"),
            CloudsearchActions::ListTags => write!(f, "cloudsearch:ListTags"),
            CloudsearchActions::RemoveTags => write!(f, "cloudsearch:RemoveTags"),
            CloudsearchActions::Search => write!(f, "cloudsearch:search"),
            CloudsearchActions::Suggest => write!(f, "cloudsearch:suggest"),
            CloudsearchActions::UpdateAvailabilityOptions => {
                write!(f, "cloudsearch:UpdateAvailabilityOptions")
            }
            CloudsearchActions::UpdateDomainEndpointOptions => {
                write!(f, "cloudsearch:UpdateDomainEndpointOptions")
            }
            CloudsearchActions::UpdateScalingParameters => {
                write!(f, "cloudsearch:UpdateScalingParameters")
            }
            CloudsearchActions::UpdateServiceAccessPolicies => {
                write!(f, "cloudsearch:UpdateServiceAccessPolicies")
            }
        }
    }
}
