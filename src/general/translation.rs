use super::*;
use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    code: u16,
    lang: String,
    pub text: Vec<String>,
}

async fn checked_get_translation(
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

pub async fn get_translation(
    word: word_pack::Word,
    from: String,
    to: String,
    index: usize,
    key: String,
) -> Message {
    match checked_get_translation(word, from, to, key).await {
        Some(translation) => Message::TranslationDownloaded(index, translation),
        None => Message::Error(Error::ErrorDownloadingTranslation(index)),
    }
}
