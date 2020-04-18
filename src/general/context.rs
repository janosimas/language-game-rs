pub struct Context {
    pub word_orig: String,
    pub options_transl: Vec<String>,
    pub helper_tips: Vec<String>,
}

impl Context {
    pub fn new(word_orig: String, options_transl: Vec<String>, helper_tips: Vec<String>) -> Self {
        Self {
            word_orig,
            options_transl,
            helper_tips,
        }
    }
}
