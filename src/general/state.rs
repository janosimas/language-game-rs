#[derive(Default)]
pub struct State {
    turn_number: u16,
    score: usize,
    failed_words: Vec<String>,
    success_words: Vec<String>,
}
