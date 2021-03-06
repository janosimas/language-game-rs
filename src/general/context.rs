use super::word_pack::Word;

pub struct Context {
    pub word_original: Word,
    pub current_word_index: usize,
}

impl Context {
    pub fn new(word_original: Word, current_word_index: usize) -> Self {
        Self {
            word_original,
            current_word_index,
        }
    }
}
