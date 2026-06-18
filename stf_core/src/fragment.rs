/// Represent the text fragment directive
/// See: https://wicg.github.io/scroll-to-text-fragment/
pub struct TextFragment {
    pub text_start: String,
    pub text_end: Option<String>,
    pub prefix: Option<String>,
    pub suffix: Option<String>,
}
