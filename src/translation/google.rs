use std::error::Error;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use super::Translate;

pub struct Google {
    key: String,
    source_language: String,
    target_language: String,
}

impl Google {
    pub fn new(source_language: &str, target_language: &str, key: String) -> Self {
        Self {
            key,
            source_language: source_language.to_string(),
            target_language: target_language.to_string(),
        }
    }
}

#[async_trait]
impl Translate for Google {
    async fn translate(&self, word: String) -> Result<String, Box<dyn Error>> {
        let response = reqwest::Client::new()
            .post("https://translation.googleapis.com/language/translate/v2")
            .query(&[("key", &self.key)])
            .body(format!(
                r#"{{
                "q": "{}",
                "source": "{}",
                "target": "{}",
                "format": "text"
              }}"#,
                word, self.source_language, self.target_language
            ))
            .send()
            .await?
            .json::<GoogleResponse>()
            .await?;

        Ok(response
            .data
            .translations
            .first()
            .map_or(Err("Error acquiring translation"), Ok)?
            .translated_text
            .clone())
    }

    fn from(&self) -> String {
        self.source_language.clone()
    }

    fn to(&self) -> String {
        self.target_language.clone()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GoogleResponse {
    data: Data,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    translations: Vec<Translation>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Translation {
    #[serde(rename = "translatedText")]
    translated_text: String,
}
