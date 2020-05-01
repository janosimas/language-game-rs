#[derive(Debug, Clone)]
pub enum Message {
    GameBegin(String),
    GameEnd,
    EndTurn,
    RequestImages(Vec<String>),
    ImageDownloaded(usize, String),
    TranslationDownloaded(usize, String),
    UserInput(UserInput),
    Error(Error),
}

#[derive(Debug, Clone)]
pub enum Error {
    ErrorDownloadingTranslation(usize),
    ErrorDownloadingImage(usize),
    ErrorRequestingImages,
}

#[derive(Debug, Clone)]
pub enum UserInput {
    OptionSelected(usize),
    HintSelected(usize),
    OptionWritten(String),
}
