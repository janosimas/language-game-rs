use log::{error, info};
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Clone)]
pub struct WordPack {
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

fn load_word_pack_file(word_pack_file: &Path) -> Option<WordPack> {
    match fs::read_to_string(word_pack_file) {
        Ok(content) => match serde_json::from_str(&content) {
            Ok(word_pack) => {
                info!("file loaded: {:?}", word_pack_file.to_str());
                return Some(word_pack);
            }
            Err(err) => error!("{}", err),
        },
        Err(err) => error!("{}", err),
    }
    None
}

pub fn select_random_words(word_pack: &WordPack, number_of_words: usize) -> Vec<Word> {
    word_pack
        .words
        .choose_multiple(&mut rand::thread_rng(), number_of_words)
        .cloned()
        .collect()
}

pub fn load() -> Vec<WordPack> {
    fs::read_dir("resources/word_pack")
        .expect("Unable to read word pack folder!")
        .into_iter()
        .filter_map(Result::ok)
        .filter_map(|entry| match entry.file_type() {
            Ok(file_type) => {
                if file_type.is_file() {
                    Some(load_word_pack_file(entry.path().as_path())?)
                } else {
                    None
                }
            }
            Err(_) => None,
        })
        .collect()
}

pub fn available_knonw_languages() -> Vec<String> {
    fs::read_dir("resources/icons/languages")
        .expect("Unable to read languages folder!")
        .into_iter()
        .filter_map(Result::ok)
        .filter_map(|entry| match entry.file_type() {
            Ok(file_type) => {
                if file_type.is_file() {
                    Some(
                        entry
                            .path()
                            .as_path()
                            .file_stem()
                            .unwrap()
                            .to_str()
                            .unwrap()
                            .into(),
                    )
                } else {
                    None
                }
            }
            Err(_) => None,
        })
        .collect()
}
