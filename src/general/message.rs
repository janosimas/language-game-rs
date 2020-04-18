#[derive(Debug, Clone)]
pub enum Message {
    GameBegin,
    GameEnd,
    UserInput(UserInput),
}

#[derive(Debug, Clone)]
pub enum UserInput {
    OptionSelected(usize),
    HintSelected(usize),
    OptionWritten(String),
}
