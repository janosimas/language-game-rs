#[derive(Debug, Clone)]
pub enum Message {
    GameBegin,
    GameEnd,
    RequestImages(Vec<String>),
    ImageDownloaded(usize, String),
    TranslationDownloaded(usize, String),
    UserInput(UserInput),
}

#[derive(Debug, Clone)]
pub enum UserInput {
    OptionSelected(usize),
    HintSelected(usize),
    OptionWritten(String),
}
