// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum IotfleetwiseActions {
    AssociateVehicleFleet,
    CreateCampaign,
    CreateDecoderManifest,
    CreateFleet,
    CreateModelManifest,
    CreateSignalCatalog,
    CreateStateTemplate,
    CreateVehicle,
    DeleteCampaign,
    DeleteDecoderManifest,
    DeleteFleet,
    DeleteModelManifest,
    DeleteSignalCatalog,
    DeleteStateTemplate,
    DeleteVehicle,
    DisassociateVehicleFleet,
    GenerateCommandPayload,
    GetCampaign,
    GetDecoderManifest,
    GetEncryptionConfiguration,
    GetFleet,
    GetLoggingOptions,
    GetModelManifest,
    GetRegisterAccountStatus,
    GetSignalCatalog,
    GetStateTemplate,
    GetVehicle,
    GetVehicleStatus,
    ImportDecoderManifest,
    ImportSignalCatalog,
    ListCampaigns,
    ListDecoderManifestNetworkInterfaces,
    ListDecoderManifestSignals,
    ListDecoderManifests,
    ListFleets,
    ListFleetsForVehicle,
    ListModelManifestNodes,
    ListModelManifests,
    ListSignalCatalogNodes,
    ListSignalCatalogs,
    ListStateTemplates,
    ListTagsForResource,
    ListVehicles,
    ListVehiclesInFleet,
    PutEncryptionConfiguration,
    PutLoggingOptions,
    RegisterAccount,
    TagResource,
    UntagResource,
    UpdateCampaign,
    UpdateDecoderManifest,
    UpdateFleet,
    UpdateModelManifest,
    UpdateSignalCatalog,
    UpdateStateTemplate,
    UpdateVehicle,
}
impl std::fmt::Display for IotfleetwiseActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IotfleetwiseActions::AssociateVehicleFleet => {
                write!(f, "iotfleetwise:AssociateVehicleFleet")
            }
            IotfleetwiseActions::CreateCampaign => write!(f, "iotfleetwise:CreateCampaign"),
            IotfleetwiseActions::CreateDecoderManifest => {
                write!(f, "iotfleetwise:CreateDecoderManifest")
            }
            IotfleetwiseActions::CreateFleet => write!(f, "iotfleetwise:CreateFleet"),
            IotfleetwiseActions::CreateModelManifest => {
                write!(f, "iotfleetwise:CreateModelManifest")
            }
            IotfleetwiseActions::CreateSignalCatalog => {
                write!(f, "iotfleetwise:CreateSignalCatalog")
            }
            IotfleetwiseActions::CreateStateTemplate => {
                write!(f, "iotfleetwise:CreateStateTemplate")
            }
            IotfleetwiseActions::CreateVehicle => write!(f, "iotfleetwise:CreateVehicle"),
            IotfleetwiseActions::DeleteCampaign => write!(f, "iotfleetwise:DeleteCampaign"),
            IotfleetwiseActions::DeleteDecoderManifest => {
                write!(f, "iotfleetwise:DeleteDecoderManifest")
            }
            IotfleetwiseActions::DeleteFleet => write!(f, "iotfleetwise:DeleteFleet"),
            IotfleetwiseActions::DeleteModelManifest => {
                write!(f, "iotfleetwise:DeleteModelManifest")
            }
            IotfleetwiseActions::DeleteSignalCatalog => {
                write!(f, "iotfleetwise:DeleteSignalCatalog")
            }
            IotfleetwiseActions::DeleteStateTemplate => {
                write!(f, "iotfleetwise:DeleteStateTemplate")
            }
            IotfleetwiseActions::DeleteVehicle => write!(f, "iotfleetwise:DeleteVehicle"),
            IotfleetwiseActions::DisassociateVehicleFleet => {
                write!(f, "iotfleetwise:DisassociateVehicleFleet")
            }
            IotfleetwiseActions::GenerateCommandPayload => {
                write!(f, "iotfleetwise:GenerateCommandPayload")
            }
            IotfleetwiseActions::GetCampaign => write!(f, "iotfleetwise:GetCampaign"),
            IotfleetwiseActions::GetDecoderManifest => write!(f, "iotfleetwise:GetDecoderManifest"),
            IotfleetwiseActions::GetEncryptionConfiguration => {
                write!(f, "iotfleetwise:GetEncryptionConfiguration")
            }
            IotfleetwiseActions::GetFleet => write!(f, "iotfleetwise:GetFleet"),
            IotfleetwiseActions::GetLoggingOptions => write!(f, "iotfleetwise:GetLoggingOptions"),
            IotfleetwiseActions::GetModelManifest => write!(f, "iotfleetwise:GetModelManifest"),
            IotfleetwiseActions::GetRegisterAccountStatus => {
                write!(f, "iotfleetwise:GetRegisterAccountStatus")
            }
            IotfleetwiseActions::GetSignalCatalog => write!(f, "iotfleetwise:GetSignalCatalog"),
            IotfleetwiseActions::GetStateTemplate => write!(f, "iotfleetwise:GetStateTemplate"),
            IotfleetwiseActions::GetVehicle => write!(f, "iotfleetwise:GetVehicle"),
            IotfleetwiseActions::GetVehicleStatus => write!(f, "iotfleetwise:GetVehicleStatus"),
            IotfleetwiseActions::ImportDecoderManifest => {
                write!(f, "iotfleetwise:ImportDecoderManifest")
            }
            IotfleetwiseActions::ImportSignalCatalog => {
                write!(f, "iotfleetwise:ImportSignalCatalog")
            }
            IotfleetwiseActions::ListCampaigns => write!(f, "iotfleetwise:ListCampaigns"),
            IotfleetwiseActions::ListDecoderManifestNetworkInterfaces => {
                write!(f, "iotfleetwise:ListDecoderManifestNetworkInterfaces")
            }
            IotfleetwiseActions::ListDecoderManifestSignals => {
                write!(f, "iotfleetwise:ListDecoderManifestSignals")
            }
            IotfleetwiseActions::ListDecoderManifests => {
                write!(f, "iotfleetwise:ListDecoderManifests")
            }
            IotfleetwiseActions::ListFleets => write!(f, "iotfleetwise:ListFleets"),
            IotfleetwiseActions::ListFleetsForVehicle => {
                write!(f, "iotfleetwise:ListFleetsForVehicle")
            }
            IotfleetwiseActions::ListModelManifestNodes => {
                write!(f, "iotfleetwise:ListModelManifestNodes")
            }
            IotfleetwiseActions::ListModelManifests => write!(f, "iotfleetwise:ListModelManifests"),
            IotfleetwiseActions::ListSignalCatalogNodes => {
                write!(f, "iotfleetwise:ListSignalCatalogNodes")
            }
            IotfleetwiseActions::ListSignalCatalogs => write!(f, "iotfleetwise:ListSignalCatalogs"),
            IotfleetwiseActions::ListStateTemplates => write!(f, "iotfleetwise:ListStateTemplates"),
            IotfleetwiseActions::ListTagsForResource => {
                write!(f, "iotfleetwise:ListTagsForResource")
            }
            IotfleetwiseActions::ListVehicles => write!(f, "iotfleetwise:ListVehicles"),
            IotfleetwiseActions::ListVehiclesInFleet => {
                write!(f, "iotfleetwise:ListVehiclesInFleet")
            }
            IotfleetwiseActions::PutEncryptionConfiguration => {
                write!(f, "iotfleetwise:PutEncryptionConfiguration")
            }
            IotfleetwiseActions::PutLoggingOptions => write!(f, "iotfleetwise:PutLoggingOptions"),
            IotfleetwiseActions::RegisterAccount => write!(f, "iotfleetwise:RegisterAccount"),
            IotfleetwiseActions::TagResource => write!(f, "iotfleetwise:TagResource"),
            IotfleetwiseActions::UntagResource => write!(f, "iotfleetwise:UntagResource"),
            IotfleetwiseActions::UpdateCampaign => write!(f, "iotfleetwise:UpdateCampaign"),
            IotfleetwiseActions::UpdateDecoderManifest => {
                write!(f, "iotfleetwise:UpdateDecoderManifest")
            }
            IotfleetwiseActions::UpdateFleet => write!(f, "iotfleetwise:UpdateFleet"),
            IotfleetwiseActions::UpdateModelManifest => {
                write!(f, "iotfleetwise:UpdateModelManifest")
            }
            IotfleetwiseActions::UpdateSignalCatalog => {
                write!(f, "iotfleetwise:UpdateSignalCatalog")
            }
            IotfleetwiseActions::UpdateStateTemplate => {
                write!(f, "iotfleetwise:UpdateStateTemplate")
            }
            IotfleetwiseActions::UpdateVehicle => write!(f, "iotfleetwise:UpdateVehicle"),
        }
    }
}
