use super::*;
use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Response {
    code: u16,
    lang: String,
    text: Vec<String>,
}

async fn checked_translate(
    word: word_pack::Word,
    from: String,
    to: String,
    key: String,
) -> Option<String> {
    let response = reqwest::Client::new()
        .get("https://translate.yandex.net/api/v1.5/tr.json/translate")
        .query(&[
            ("key", &key),
            ("lang", &format!("{}-{}", from, to)),
            ("text", &word.word),
            ("format", &"plain".to_string()),
        ])
        .send()
        .await
        .ok()?
        .json::<Response>()
        .await
        .ok()?;

    Some(response.text.first()?.clone())
}

/// Request a translation for the `word` and signal the option widget at `index` when finished.
pub async fn translate(
    word: word_pack::Word,
    from: String,
    to: String,
    index: usize,
    key: String,
) -> Message {
    match checked_translate(word, from, to, key).await {
        Some(translation) => Message::TranslationDownloaded(index, translation),
        None => Message::Error(Error::ErrorDownloadingTranslation(index)),
    }
}
