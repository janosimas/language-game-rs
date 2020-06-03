use super::Translate;
use async_trait::async_trait;
use reqwest;
use serde::{Deserialize, Serialize};
use std::error::Error;

pub struct Yandex {
    key: String,
    source_language: String,
    target_language: String,
}

impl Yandex {
    pub fn new(source_language: &str, target_language: &str, key: String) -> Self {
        Self {
            key,
            source_language: source_language.to_string(),
            target_language: target_language.to_string(),
        }
    }
}

#[async_trait]
impl Translate for Yandex {
    fn from(&self) -> String {
        self.source_language.clone()
    }

    fn to(&self) -> String {
        self.target_language.clone()
    }

    async fn translate(&self, word: String) -> Result<String, Box<dyn Error>> {
        let response = reqwest::Client::new()
            .get("https://translate.yandex.net/api/v1.5/tr.json/translate")
            .query(&[
                ("key", &self.key),
                ("lang", &format!("{}-{}", self.from(), self.to())),
                ("text", &word),
                ("format", &"plain".to_string()),
            ])
            .send()
            .await?
            .json::<YandexResponse>()
            .await?;

        Ok(response
            .text
            .first()
            .map_or(Err("Error acquiring translation"), |word| Ok(word))?
            .clone())
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct YandexResponse {
    code: u16,
    lang: String,
    text: Vec<String>,
}
