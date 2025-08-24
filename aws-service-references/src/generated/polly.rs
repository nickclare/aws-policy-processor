// NOTE: This code is automatically generated - do not update it manually

#[derive(Debug, Clone, Copy, std :: cmp :: PartialEq, std :: cmp :: Eq, std :: hash :: Hash)]
pub enum PollyActions {
    DeleteLexicon,
    DescribeVoices,
    GetLexicon,
    GetSpeechSynthesisTask,
    ListLexicons,
    ListSpeechSynthesisTasks,
    PutLexicon,
    StartSpeechSynthesisTask,
    SynthesizeSpeech,
}
impl std::fmt::Display for PollyActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PollyActions::DeleteLexicon => write!(f, "polly:DeleteLexicon"),
            PollyActions::DescribeVoices => write!(f, "polly:DescribeVoices"),
            PollyActions::GetLexicon => write!(f, "polly:GetLexicon"),
            PollyActions::GetSpeechSynthesisTask => write!(f, "polly:GetSpeechSynthesisTask"),
            PollyActions::ListLexicons => write!(f, "polly:ListLexicons"),
            PollyActions::ListSpeechSynthesisTasks => write!(f, "polly:ListSpeechSynthesisTasks"),
            PollyActions::PutLexicon => write!(f, "polly:PutLexicon"),
            PollyActions::StartSpeechSynthesisTask => write!(f, "polly:StartSpeechSynthesisTask"),
            PollyActions::SynthesizeSpeech => write!(f, "polly:SynthesizeSpeech"),
        }
    }
}
