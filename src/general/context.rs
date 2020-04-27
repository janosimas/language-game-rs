pub struct Context {
    pub word_original: String,
    pub current_word_index: usize,
    pub options_translation: Vec<String>,
    pub helper_tips: Vec<String>,
}

impl Context {
    pub fn new(
        word_original: String,
        word_translation: usize,
        options_translation: Vec<String>,
        helper_tips: Vec<String>,
    ) -> Self {
        Self {
            word_original,
            current_word_index: word_translation,
            options_translation,
            helper_tips,
        }
    }
}
