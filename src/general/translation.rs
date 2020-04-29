use super::*;
use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    code: u16,
    lang: String,
    pub text: Vec<String>,
}

pub async fn get_translation(
    word: Word,
    from: String,
    to: String,
    index: usize,
    key: String,
) -> Message {
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
        .unwrap()
        .json::<Response>()
        .await
        .unwrap();

    let translation = response.text.first().unwrap();
    Message::TranslationDownloaded(index, translation.clone())
}
