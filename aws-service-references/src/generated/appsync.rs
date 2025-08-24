// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum AppsyncActions {
    AssociateApi,
    AssociateMergedGraphqlApi,
    AssociateSourceGraphqlApi,
    CreateApi,
    CreateApiCache,
    CreateApiKey,
    CreateChannelNamespace,
    CreateDataSource,
    CreateDomainName,
    CreateFunction,
    CreateGraphqlApi,
    CreateResolver,
    CreateType,
    DeleteApi,
    DeleteApiCache,
    DeleteApiKey,
    DeleteChannelNamespace,
    DeleteDataSource,
    DeleteDomainName,
    DeleteFunction,
    DeleteGraphqlApi,
    DeleteResolver,
    DeleteResourcePolicy,
    DeleteType,
    DisassociateApi,
    DisassociateMergedGraphqlApi,
    DisassociateSourceGraphqlApi,
    EvaluateCode,
    EvaluateMappingTemplate,
    EventConnect,
    EventPublish,
    EventSubscribe,
    FlushApiCache,
    GetApi,
    GetApiAssociation,
    GetApiCache,
    GetChannelNamespace,
    GetDataSource,
    GetDataSourceIntrospection,
    GetDomainName,
    GetFunction,
    GetGraphqlApi,
    GetGraphqlApiEnvironmentVariables,
    GetIntrospectionSchema,
    GetResolver,
    GetResourcePolicy,
    GetSchemaCreationStatus,
    GetSourceApiAssociation,
    GetType,
    GraphQl,
    ListApiKeys,
    ListApis,
    ListChannelNamespaces,
    ListDataSources,
    ListDomainNames,
    ListFunctions,
    ListGraphqlApis,
    ListResolvers,
    ListResolversByFunction,
    ListSourceApiAssociations,
    ListTagsForResource,
    ListTypes,
    ListTypesByAssociation,
    PutGraphqlApiEnvironmentVariables,
    PutResourcePolicy,
    SetWebAcl,
    SourceGraphQl,
    StartDataSourceIntrospection,
    StartSchemaCreation,
    StartSchemaMerge,
    TagResource,
    UntagResource,
    UpdateApi,
    UpdateApiCache,
    UpdateApiKey,
    UpdateChannelNamespace,
    UpdateDataSource,
    UpdateDomainName,
    UpdateFunction,
    UpdateGraphqlApi,
    UpdateResolver,
    UpdateSourceApiAssociation,
    UpdateType,
}
impl std::fmt::Display for AppsyncActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppsyncActions::AssociateApi => write!(f, "appsync:AssociateApi"),
            AppsyncActions::AssociateMergedGraphqlApi => {
                write!(f, "appsync:AssociateMergedGraphqlApi")
            }
            AppsyncActions::AssociateSourceGraphqlApi => {
                write!(f, "appsync:AssociateSourceGraphqlApi")
            }
            AppsyncActions::CreateApi => write!(f, "appsync:CreateApi"),
            AppsyncActions::CreateApiCache => write!(f, "appsync:CreateApiCache"),
            AppsyncActions::CreateApiKey => write!(f, "appsync:CreateApiKey"),
            AppsyncActions::CreateChannelNamespace => write!(f, "appsync:CreateChannelNamespace"),
            AppsyncActions::CreateDataSource => write!(f, "appsync:CreateDataSource"),
            AppsyncActions::CreateDomainName => write!(f, "appsync:CreateDomainName"),
            AppsyncActions::CreateFunction => write!(f, "appsync:CreateFunction"),
            AppsyncActions::CreateGraphqlApi => write!(f, "appsync:CreateGraphqlApi"),
            AppsyncActions::CreateResolver => write!(f, "appsync:CreateResolver"),
            AppsyncActions::CreateType => write!(f, "appsync:CreateType"),
            AppsyncActions::DeleteApi => write!(f, "appsync:DeleteApi"),
            AppsyncActions::DeleteApiCache => write!(f, "appsync:DeleteApiCache"),
            AppsyncActions::DeleteApiKey => write!(f, "appsync:DeleteApiKey"),
            AppsyncActions::DeleteChannelNamespace => write!(f, "appsync:DeleteChannelNamespace"),
            AppsyncActions::DeleteDataSource => write!(f, "appsync:DeleteDataSource"),
            AppsyncActions::DeleteDomainName => write!(f, "appsync:DeleteDomainName"),
            AppsyncActions::DeleteFunction => write!(f, "appsync:DeleteFunction"),
            AppsyncActions::DeleteGraphqlApi => write!(f, "appsync:DeleteGraphqlApi"),
            AppsyncActions::DeleteResolver => write!(f, "appsync:DeleteResolver"),
            AppsyncActions::DeleteResourcePolicy => write!(f, "appsync:DeleteResourcePolicy"),
            AppsyncActions::DeleteType => write!(f, "appsync:DeleteType"),
            AppsyncActions::DisassociateApi => write!(f, "appsync:DisassociateApi"),
            AppsyncActions::DisassociateMergedGraphqlApi => {
                write!(f, "appsync:DisassociateMergedGraphqlApi")
            }
            AppsyncActions::DisassociateSourceGraphqlApi => {
                write!(f, "appsync:DisassociateSourceGraphqlApi")
            }
            AppsyncActions::EvaluateCode => write!(f, "appsync:EvaluateCode"),
            AppsyncActions::EvaluateMappingTemplate => write!(f, "appsync:EvaluateMappingTemplate"),
            AppsyncActions::EventConnect => write!(f, "appsync:EventConnect"),
            AppsyncActions::EventPublish => write!(f, "appsync:EventPublish"),
            AppsyncActions::EventSubscribe => write!(f, "appsync:EventSubscribe"),
            AppsyncActions::FlushApiCache => write!(f, "appsync:FlushApiCache"),
            AppsyncActions::GetApi => write!(f, "appsync:GetApi"),
            AppsyncActions::GetApiAssociation => write!(f, "appsync:GetApiAssociation"),
            AppsyncActions::GetApiCache => write!(f, "appsync:GetApiCache"),
            AppsyncActions::GetChannelNamespace => write!(f, "appsync:GetChannelNamespace"),
            AppsyncActions::GetDataSource => write!(f, "appsync:GetDataSource"),
            AppsyncActions::GetDataSourceIntrospection => {
                write!(f, "appsync:GetDataSourceIntrospection")
            }
            AppsyncActions::GetDomainName => write!(f, "appsync:GetDomainName"),
            AppsyncActions::GetFunction => write!(f, "appsync:GetFunction"),
            AppsyncActions::GetGraphqlApi => write!(f, "appsync:GetGraphqlApi"),
            AppsyncActions::GetGraphqlApiEnvironmentVariables => {
                write!(f, "appsync:GetGraphqlApiEnvironmentVariables")
            }
            AppsyncActions::GetIntrospectionSchema => write!(f, "appsync:GetIntrospectionSchema"),
            AppsyncActions::GetResolver => write!(f, "appsync:GetResolver"),
            AppsyncActions::GetResourcePolicy => write!(f, "appsync:GetResourcePolicy"),
            AppsyncActions::GetSchemaCreationStatus => write!(f, "appsync:GetSchemaCreationStatus"),
            AppsyncActions::GetSourceApiAssociation => write!(f, "appsync:GetSourceApiAssociation"),
            AppsyncActions::GetType => write!(f, "appsync:GetType"),
            AppsyncActions::GraphQl => write!(f, "appsync:GraphQL"),
            AppsyncActions::ListApiKeys => write!(f, "appsync:ListApiKeys"),
            AppsyncActions::ListApis => write!(f, "appsync:ListApis"),
            AppsyncActions::ListChannelNamespaces => write!(f, "appsync:ListChannelNamespaces"),
            AppsyncActions::ListDataSources => write!(f, "appsync:ListDataSources"),
            AppsyncActions::ListDomainNames => write!(f, "appsync:ListDomainNames"),
            AppsyncActions::ListFunctions => write!(f, "appsync:ListFunctions"),
            AppsyncActions::ListGraphqlApis => write!(f, "appsync:ListGraphqlApis"),
            AppsyncActions::ListResolvers => write!(f, "appsync:ListResolvers"),
            AppsyncActions::ListResolversByFunction => write!(f, "appsync:ListResolversByFunction"),
            AppsyncActions::ListSourceApiAssociations => {
                write!(f, "appsync:ListSourceApiAssociations")
            }
            AppsyncActions::ListTagsForResource => write!(f, "appsync:ListTagsForResource"),
            AppsyncActions::ListTypes => write!(f, "appsync:ListTypes"),
            AppsyncActions::ListTypesByAssociation => write!(f, "appsync:ListTypesByAssociation"),
            AppsyncActions::PutGraphqlApiEnvironmentVariables => {
                write!(f, "appsync:PutGraphqlApiEnvironmentVariables")
            }
            AppsyncActions::PutResourcePolicy => write!(f, "appsync:PutResourcePolicy"),
            AppsyncActions::SetWebAcl => write!(f, "appsync:SetWebACL"),
            AppsyncActions::SourceGraphQl => write!(f, "appsync:SourceGraphQL"),
            AppsyncActions::StartDataSourceIntrospection => {
                write!(f, "appsync:StartDataSourceIntrospection")
            }
            AppsyncActions::StartSchemaCreation => write!(f, "appsync:StartSchemaCreation"),
            AppsyncActions::StartSchemaMerge => write!(f, "appsync:StartSchemaMerge"),
            AppsyncActions::TagResource => write!(f, "appsync:TagResource"),
            AppsyncActions::UntagResource => write!(f, "appsync:UntagResource"),
            AppsyncActions::UpdateApi => write!(f, "appsync:UpdateApi"),
            AppsyncActions::UpdateApiCache => write!(f, "appsync:UpdateApiCache"),
            AppsyncActions::UpdateApiKey => write!(f, "appsync:UpdateApiKey"),
            AppsyncActions::UpdateChannelNamespace => write!(f, "appsync:UpdateChannelNamespace"),
            AppsyncActions::UpdateDataSource => write!(f, "appsync:UpdateDataSource"),
            AppsyncActions::UpdateDomainName => write!(f, "appsync:UpdateDomainName"),
            AppsyncActions::UpdateFunction => write!(f, "appsync:UpdateFunction"),
            AppsyncActions::UpdateGraphqlApi => write!(f, "appsync:UpdateGraphqlApi"),
            AppsyncActions::UpdateResolver => write!(f, "appsync:UpdateResolver"),
            AppsyncActions::UpdateSourceApiAssociation => {
                write!(f, "appsync:UpdateSourceApiAssociation")
            }
            AppsyncActions::UpdateType => write!(f, "appsync:UpdateType"),
        }
    }
}
