pub struct Context {
    pub word_original: String,
    pub current_word_index: usize
}

impl Context {
    pub fn new(
        word_original: String,
        current_word_index: usize
    ) -> Self {
        Self {
            word_original,
            current_word_index: current_word_index
        }
    }
}
