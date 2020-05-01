pub struct State {
    game_running: bool,
    turn_number: u16,
    score: usize,
    final_score: Option<usize>,
    failed_words: Vec<String>,
    success_words: Vec<String>,
    pub tranlation_pair: (String, String),
    pub image_pair: (String, String),
}

impl State {
    pub fn new(tranlation_pair: (String, String), image_pair: (String, String)) -> Self {
        Self {
            game_running: false,
            turn_number: 0,
            score: 0,
            final_score: Some(10),
            failed_words: vec![],
            success_words: vec![],
            tranlation_pair,
            image_pair,
        }
    }

    pub fn start(&mut self) {
        self.game_running = true;
    }

    pub fn is_game_running(&self) -> bool {
        self.game_running
    }

    pub fn has_game_ended(&self) -> bool {
        if let Some(final_score) = self.final_score {
            return self.score() == final_score;
        }
        false
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
