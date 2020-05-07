use super::word_pack::Word;

pub enum GameState {
    NotRunning,
    Running,
    Ended,
}

pub struct State {
    game_running: bool,
    turn_number: u16,
    score: usize,
    final_score: Option<usize>,
    failed_words: Vec<Word>,
    success_words: Vec<Word>,
    pub tranlation_pair: (String, String),
    pub image_pair: (String, String),
    pub known_language: String,
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
            known_language: "en".to_string(),
        }
    }

    pub fn game_state(&self) -> GameState {
        if !self.is_game_running() {
            return GameState::NotRunning;
        }

        if self.has_game_ended() {
            return GameState::Ended;
        }

        GameState::Running
    }

    pub fn start(&mut self) {
        self.game_running = true;
    }

    pub fn is_game_running(&self) -> bool {
        self.game_running
    }

    pub fn has_game_ended(&self) -> bool {
        if let Some(final_score) = self.final_score {
            return self.score() >= final_score;
        }
        false
    }

    pub fn advance_turn(&mut self) {
        self.turn_number += 1;
    }

    pub fn advance_score(&mut self) {
        self.score += 1;
    }

    pub fn add_correct_word(&mut self, word: &Word) {
        self.success_words.push(word.clone());
    }

    pub fn add_wrong_word(&mut self, word: &Word) {
        self.failed_words.push(word.clone());
    }

    pub fn score(&self) -> usize {
        self.score
    }

    pub fn turn(&self) -> u16 {
        self.turn_number
    }
}
