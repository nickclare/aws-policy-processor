// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum CloudfrontKeyvaluestoreActions {
    DeleteKey,
    DescribeKeyValueStore,
    GetKey,
    ListKeys,
    PutKey,
    UpdateKeys,
}
impl std::fmt::Display for CloudfrontKeyvaluestoreActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CloudfrontKeyvaluestoreActions::DeleteKey => {
                write!(f, "cloudfront-keyvaluestore:DeleteKey")
            }
            CloudfrontKeyvaluestoreActions::DescribeKeyValueStore => {
                write!(f, "cloudfront-keyvaluestore:DescribeKeyValueStore")
            }
            CloudfrontKeyvaluestoreActions::GetKey => write!(f, "cloudfront-keyvaluestore:GetKey"),
            CloudfrontKeyvaluestoreActions::ListKeys => {
                write!(f, "cloudfront-keyvaluestore:ListKeys")
            }
            CloudfrontKeyvaluestoreActions::PutKey => write!(f, "cloudfront-keyvaluestore:PutKey"),
            CloudfrontKeyvaluestoreActions::UpdateKeys => {
                write!(f, "cloudfront-keyvaluestore:UpdateKeys")
            }
        }
    }
}
