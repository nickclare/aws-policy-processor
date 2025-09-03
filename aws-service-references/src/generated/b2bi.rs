// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum B2biActions {
    CreateCapability,
    CreatePartnership,
    CreateProfile,
    CreateStarterMappingTemplate,
    CreateTransformer,
    DeleteCapability,
    DeletePartnership,
    DeleteProfile,
    DeleteTransformer,
    GenerateMapping,
    GetCapability,
    GetPartnership,
    GetProfile,
    GetTransformer,
    GetTransformerJob,
    ListCapabilities,
    ListPartnerships,
    ListProfiles,
    ListTagsForResource,
    ListTransformers,
    StartTransformerJob,
    TagResource,
    TestConversion,
    TestMapping,
    TestParsing,
    UntagResource,
    UpdateCapability,
    UpdatePartnership,
    UpdateProfile,
    UpdateTransformer,
}
impl std::fmt::Display for B2biActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            B2biActions::CreateCapability => write!(f, "b2bi:CreateCapability"),
            B2biActions::CreatePartnership => write!(f, "b2bi:CreatePartnership"),
            B2biActions::CreateProfile => write!(f, "b2bi:CreateProfile"),
            B2biActions::CreateStarterMappingTemplate => {
                write!(f, "b2bi:CreateStarterMappingTemplate")
            }
            B2biActions::CreateTransformer => write!(f, "b2bi:CreateTransformer"),
            B2biActions::DeleteCapability => write!(f, "b2bi:DeleteCapability"),
            B2biActions::DeletePartnership => write!(f, "b2bi:DeletePartnership"),
            B2biActions::DeleteProfile => write!(f, "b2bi:DeleteProfile"),
            B2biActions::DeleteTransformer => write!(f, "b2bi:DeleteTransformer"),
            B2biActions::GenerateMapping => write!(f, "b2bi:GenerateMapping"),
            B2biActions::GetCapability => write!(f, "b2bi:GetCapability"),
            B2biActions::GetPartnership => write!(f, "b2bi:GetPartnership"),
            B2biActions::GetProfile => write!(f, "b2bi:GetProfile"),
            B2biActions::GetTransformer => write!(f, "b2bi:GetTransformer"),
            B2biActions::GetTransformerJob => write!(f, "b2bi:GetTransformerJob"),
            B2biActions::ListCapabilities => write!(f, "b2bi:ListCapabilities"),
            B2biActions::ListPartnerships => write!(f, "b2bi:ListPartnerships"),
            B2biActions::ListProfiles => write!(f, "b2bi:ListProfiles"),
            B2biActions::ListTagsForResource => write!(f, "b2bi:ListTagsForResource"),
            B2biActions::ListTransformers => write!(f, "b2bi:ListTransformers"),
            B2biActions::StartTransformerJob => write!(f, "b2bi:StartTransformerJob"),
            B2biActions::TagResource => write!(f, "b2bi:TagResource"),
            B2biActions::TestConversion => write!(f, "b2bi:TestConversion"),
            B2biActions::TestMapping => write!(f, "b2bi:TestMapping"),
            B2biActions::TestParsing => write!(f, "b2bi:TestParsing"),
            B2biActions::UntagResource => write!(f, "b2bi:UntagResource"),
            B2biActions::UpdateCapability => write!(f, "b2bi:UpdateCapability"),
            B2biActions::UpdatePartnership => write!(f, "b2bi:UpdatePartnership"),
            B2biActions::UpdateProfile => write!(f, "b2bi:UpdateProfile"),
            B2biActions::UpdateTransformer => write!(f, "b2bi:UpdateTransformer"),
        }
    }
}
