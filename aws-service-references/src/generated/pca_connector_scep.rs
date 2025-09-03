// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum PcaConnectorScepActions {
    CreateChallenge,
    CreateConnector,
    DeleteChallenge,
    DeleteConnector,
    GetChallengeMetadata,
    GetChallengePassword,
    GetConnector,
    ListChallengeMetadata,
    ListConnectors,
    ListTagsForResource,
    TagResource,
    UntagResource,
}
impl std::fmt::Display for PcaConnectorScepActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PcaConnectorScepActions::CreateChallenge => {
                write!(f, "pca-connector-scep:CreateChallenge")
            }
            PcaConnectorScepActions::CreateConnector => {
                write!(f, "pca-connector-scep:CreateConnector")
            }
            PcaConnectorScepActions::DeleteChallenge => {
                write!(f, "pca-connector-scep:DeleteChallenge")
            }
            PcaConnectorScepActions::DeleteConnector => {
                write!(f, "pca-connector-scep:DeleteConnector")
            }
            PcaConnectorScepActions::GetChallengeMetadata => {
                write!(f, "pca-connector-scep:GetChallengeMetadata")
            }
            PcaConnectorScepActions::GetChallengePassword => {
                write!(f, "pca-connector-scep:GetChallengePassword")
            }
            PcaConnectorScepActions::GetConnector => write!(f, "pca-connector-scep:GetConnector"),
            PcaConnectorScepActions::ListChallengeMetadata => {
                write!(f, "pca-connector-scep:ListChallengeMetadata")
            }
            PcaConnectorScepActions::ListConnectors => {
                write!(f, "pca-connector-scep:ListConnectors")
            }
            PcaConnectorScepActions::ListTagsForResource => {
                write!(f, "pca-connector-scep:ListTagsForResource")
            }
            PcaConnectorScepActions::TagResource => write!(f, "pca-connector-scep:TagResource"),
            PcaConnectorScepActions::UntagResource => write!(f, "pca-connector-scep:UntagResource"),
        }
    }
}
