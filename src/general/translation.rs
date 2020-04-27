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
    word: &Word,
    from: &str,
    to: &str,
    state: &State,
) -> Result<Response, reqwest::Error> {
    Ok(reqwest::Client::new()
        .get("https://translate.yandex.net/api/v1.5/tr.json/translate")
        .query(&[
            ("key", &state.tranlation_pair.1),
            ("lang", &format!("{}-{}", from, to)),
            ("text", &word.word),
            ("format", &"plain".to_string()),
        ])
        .send()
        .await?
        .json::<Response>()
        .await?)
}
