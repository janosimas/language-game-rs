use super::Translate;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::error::Error;

pub struct MyMemory {
    source_language: String,
    target_language: String,
}

impl MyMemory {
    pub fn new(source_language: &str, target_language: &str) -> Self {
        Self {
            source_language: source_language.to_string(),
            target_language: target_language.to_string(),
        }
    }
}

#[async_trait]
impl Translate for MyMemory {
    async fn translate(&self, word: String) -> Result<String, Box<dyn Error>> {
        let response = reqwest::Client::new()
            .get("https://api.mymemory.translated.net/get")
            .query(&[
                ("langpair", &format!("{}|{}", self.from(), self.to())),
                ("q", &word),
            ])
            .send()
            .await?
            .json::<MyMemoryResponse>()
            .await?;

        Ok(response.response_data.translated_text)
    }
    fn from(&self) -> String {
        self.source_language.clone()
    }

    fn to(&self) -> String {
        self.target_language.clone()
    }
}


#[derive(Serialize, Deserialize)]
pub struct MyMemoryResponse {
    #[serde(rename = "responseData")]
    response_data: ResponseData,
    #[serde(rename = "quotaFinished")]
    quota_finished: bool,
    #[serde(rename = "mtLangSupported")]
    mt_lang_supported: Option<serde_json::Value>,
    #[serde(rename = "responseDetails")]
    response_details: String,
    #[serde(rename = "responseStatus")]
    response_status: i64,
    #[serde(rename = "responderId")]
    responder_id: String,
    exception_code: Option<serde_json::Value>,
    matches: Vec<Match>,
}

#[derive(Serialize, Deserialize)]
pub struct Match {
    id: Id,
    segment: String,
    translation: String,
    source: String,
    target: String,
    quality: Id,
    reference: Option<String>,
    #[serde(rename = "usage-count")]
    usage_count: i64,
    subject: Subject,
    #[serde(rename = "created-by")]
    created_by: String,
    #[serde(rename = "last-updated-by")]
    last_updated_by: String,
    #[serde(rename = "create-date")]
    create_date: String,
    #[serde(rename = "last-update-date")]
    last_update_date: String,
    #[serde(rename = "match")]
    match_match: f64,
    model: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseData {
    #[serde(rename = "translatedText")]
    translated_text: String,
    #[serde(rename = "match")]
    response_data_match: f64,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Id {
    Integer(i64),
    String(String),
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Subject {
    Bool(bool),
    String(String),
}
