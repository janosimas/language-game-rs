pub struct Context {
    word_orig: String,
    options_transl: Vec<String>,
    helper_tips: Vec<String>,
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
