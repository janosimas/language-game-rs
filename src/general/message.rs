#[derive(Debug, Clone)]
pub enum Message {
    GameBegin(String),
    GameEnd,
    EndTurn(Answer),
    NextTurn,
    RequestImages(Vec<String>),
    ImageDownloaded(usize, String),
    TranslationDownloaded(usize, String),
    UserInput(UserInput),
    Error(Error),
    Options(Options)
}

#[derive(Debug, Clone)]
pub enum Options {
    Start
}

#[derive(Debug, Clone)]
pub enum Error {
    DownloadingTranslation(usize),
    DownloadingImage(usize),
    RequestingImages,
}

#[derive(Debug, Clone)]
pub enum UserInput {
    WordPackSelected(usize),
    OptionSelected(usize),
}

#[derive(Debug, Clone)]
pub enum Answer {
    Correct,
    Wrong
}
