pub struct State {
    turn_number: u16,
    score: usize,
    failed_words: Vec<String>,
    success_words: Vec<String>,
    pub tranlation_pair: (String, String),
    pub image_pair: (String, String),
}

impl State {
    pub fn new(tranlation_pair: (String, String), image_pair: (String, String)) -> Self {
        Self {
            turn_number: 0,
            score: 0,
            failed_words: vec![],
            success_words: vec![],
            tranlation_pair,
            image_pair,
        }
    }

    pub fn advance_turn(&mut self) {
        self.turn_number += 1;
    }

    pub fn advance_score(&mut self) {
        self.score += 1;
    }

    pub fn add_correct_word(&mut self, word: &str) {
        self.success_words.push(word.to_string());
    }

    pub fn add_wrong_word(&mut self, word: &str) {
        self.failed_words.push(word.to_string());
    }

    pub fn score(&self) -> usize {
        self.score
    }

    pub fn turn(&self) -> u16 {
        self.turn_number
    }
}
