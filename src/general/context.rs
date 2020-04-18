pub struct Context {
    pub word_original: String,
    pub word_translation: String,
    pub options_translation: Vec<String>,
    pub helper_tips: Vec<String>,
}

impl Context {
    pub fn new(
        word_original: String,
        word_translation: String,
        options_translation: Vec<String>,
        helper_tips: Vec<String>,
    ) -> Self {
        Self {
            word_original,
            word_translation,
            options_translation,
            helper_tips,
        }
    }
}
