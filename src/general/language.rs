use log::{error, info};
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::fmt;
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

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct Word {
    pub prefix: Option<String>,
    pub word: String,
    pub sufix: Option<String>,
    pub translation_aid: Option<String>,
    pub image_search_aid: Option<String>,
}

impl fmt::Display for Word {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {}",
            self.prefix.as_ref().unwrap_or(&String::new()),
            self.word
        )
    }
}

pub fn load_language() -> Option<Language> {
    let language_file = Path::new("./resources/de-de.json");
    match fs::read_to_string(language_file) {
        Ok(content) => match serde_json::from_str(&content) {
            Ok(language) => {
                info!("file loaded: {:?}", language_file.to_str());
                return Some(language);
            }
            Err(err) => error!("{}", err),
        },
        Err(err) => error!("{}", err),
    }
    None
}

pub fn select_random_words(language: &Language, number_of_words: usize) -> Vec<Word> {
    language
        .words
        .choose_multiple(&mut rand::thread_rng(), number_of_words)
        .cloned()
        .collect()
}
