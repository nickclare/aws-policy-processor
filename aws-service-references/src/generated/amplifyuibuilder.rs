// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum AmplifyuibuilderActions {
    CreateComponent,
    CreateForm,
    CreateTheme,
    DeleteComponent,
    DeleteForm,
    DeleteTheme,
    ExchangeCodeForToken,
    ExportComponents,
    ExportForms,
    ExportThemes,
    GetCodegenJob,
    GetComponent,
    GetForm,
    GetMetadata,
    GetTheme,
    ListCodegenJobs,
    ListComponents,
    ListForms,
    ListTagsForResource,
    ListThemes,
    PutMetadataFlag,
    RefreshToken,
    ResetMetadataFlag,
    StartCodegenJob,
    TagResource,
    UntagResource,
    UpdateComponent,
    UpdateForm,
    UpdateTheme,
}
impl std::fmt::Display for AmplifyuibuilderActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AmplifyuibuilderActions::CreateComponent => {
                write!(f, "amplifyuibuilder:CreateComponent")
            }
            AmplifyuibuilderActions::CreateForm => write!(f, "amplifyuibuilder:CreateForm"),
            AmplifyuibuilderActions::CreateTheme => write!(f, "amplifyuibuilder:CreateTheme"),
            AmplifyuibuilderActions::DeleteComponent => {
                write!(f, "amplifyuibuilder:DeleteComponent")
            }
            AmplifyuibuilderActions::DeleteForm => write!(f, "amplifyuibuilder:DeleteForm"),
            AmplifyuibuilderActions::DeleteTheme => write!(f, "amplifyuibuilder:DeleteTheme"),
            AmplifyuibuilderActions::ExchangeCodeForToken => {
                write!(f, "amplifyuibuilder:ExchangeCodeForToken")
            }
            AmplifyuibuilderActions::ExportComponents => {
                write!(f, "amplifyuibuilder:ExportComponents")
            }
            AmplifyuibuilderActions::ExportForms => write!(f, "amplifyuibuilder:ExportForms"),
            AmplifyuibuilderActions::ExportThemes => write!(f, "amplifyuibuilder:ExportThemes"),
            AmplifyuibuilderActions::GetCodegenJob => write!(f, "amplifyuibuilder:GetCodegenJob"),
            AmplifyuibuilderActions::GetComponent => write!(f, "amplifyuibuilder:GetComponent"),
            AmplifyuibuilderActions::GetForm => write!(f, "amplifyuibuilder:GetForm"),
            AmplifyuibuilderActions::GetMetadata => write!(f, "amplifyuibuilder:GetMetadata"),
            AmplifyuibuilderActions::GetTheme => write!(f, "amplifyuibuilder:GetTheme"),
            AmplifyuibuilderActions::ListCodegenJobs => {
                write!(f, "amplifyuibuilder:ListCodegenJobs")
            }
            AmplifyuibuilderActions::ListComponents => write!(f, "amplifyuibuilder:ListComponents"),
            AmplifyuibuilderActions::ListForms => write!(f, "amplifyuibuilder:ListForms"),
            AmplifyuibuilderActions::ListTagsForResource => {
                write!(f, "amplifyuibuilder:ListTagsForResource")
            }
            AmplifyuibuilderActions::ListThemes => write!(f, "amplifyuibuilder:ListThemes"),
            AmplifyuibuilderActions::PutMetadataFlag => {
                write!(f, "amplifyuibuilder:PutMetadataFlag")
            }
            AmplifyuibuilderActions::RefreshToken => write!(f, "amplifyuibuilder:RefreshToken"),
            AmplifyuibuilderActions::ResetMetadataFlag => {
                write!(f, "amplifyuibuilder:ResetMetadataFlag")
            }
            AmplifyuibuilderActions::StartCodegenJob => {
                write!(f, "amplifyuibuilder:StartCodegenJob")
            }
            AmplifyuibuilderActions::TagResource => write!(f, "amplifyuibuilder:TagResource"),
            AmplifyuibuilderActions::UntagResource => write!(f, "amplifyuibuilder:UntagResource"),
            AmplifyuibuilderActions::UpdateComponent => {
                write!(f, "amplifyuibuilder:UpdateComponent")
            }
            AmplifyuibuilderActions::UpdateForm => write!(f, "amplifyuibuilder:UpdateForm"),
            AmplifyuibuilderActions::UpdateTheme => write!(f, "amplifyuibuilder:UpdateTheme"),
        }
    }
}
