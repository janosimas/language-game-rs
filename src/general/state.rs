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
}
