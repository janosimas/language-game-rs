use serde::{Deserialize, Serialize};

use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize)]
pub struct Language {
    pub language: String,
    pub country: String,
    pub description: String,
    pub source: String,
    pub words: Vec<Word>,
}

#[derive(Serialize, Deserialize)]
pub struct Word {
    pub prefix: Option<String>,
    pub word: String,
    pub sufix: Option<String>,
    pub translation_aid: Option<String>,
    pub image_search_aid: Option<String>,
}

pub fn load_language() -> Option<Language> {
    let language_file = Path::new("./resources/de-de.json");
    let content = fs::read_to_string(language_file).ok()?;
    serde_json::from_str(&content).ok()
}
