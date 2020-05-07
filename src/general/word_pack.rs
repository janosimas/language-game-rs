use crate::general::resources;
use log::{error, info};
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs;
use std::path::Path;

/// Word pack with description and list of words available
///
/// A word pack is a list of words to be used in the app.
/// The pack could be of most common words in a language
/// or of an specific theme.
///
/// # Format
///
/// To add a new word pack, just create a json file following the format and add to the `resourses/word_pack` folder.
///
/// ```json
/// {
///     "language": "en",
///     "country": "us",
///     "description": "Description of the word pack.",
///     "source": "http://website_or_some_other_source.com/",
///     "words": [
///         {
///             "word": "Unknown"
///         },
///         ...
///     ]
/// }
/// ```
///
#[derive(Serialize, Deserialize, Clone)]
pub struct WordPack {
    pub language: String,
    pub country: String,
    pub description: String,
    pub source: String,
    pub words: Vec<Word>,
}

impl WordPack {
    /// Load the content of a word pack file
    fn load(word_pack_file: &Path) -> Option<WordPack> {
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

    /// Return a list of randomly selected Words
    pub fn choose_random(&self, number_of_words: usize) -> Vec<Word> {
        self.words
            .choose_multiple(&mut rand::thread_rng(), number_of_words)
            .cloned()
            .collect()
    }
}

/// Word and helper data for learning, translation and image locating
///
/// The `word` member has the proper word to be used but some languages
/// may have a need for prefix and sufix for the word.
///
/// For example, Portuguese and German have genders for the words.
/// So it's quite usefull to learn the word with the article in the correct gender.
///
/// The field `translation_aid` can be used for words that are hard to translate,
/// you can use an expression in english that will be used instead of the original word.
///
/// In the same way, the field `image_search_aid` is a text that helps finding an
/// apropriate descriptive image for the word.
///
/// # Format
///
/// ```json
/// {
///     "prefix": <"prefix">,
///     "word": "Word",
///     "sufix": <"sufix">,
///     "translation_aid": <"word in english">,
///     "image_search_aid": <"easier to find an image using this text">,
/// }
/// ```
///
#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct Word {
    pub prefix: Option<String>,
    pub word: String,
    pub sufix: Option<String>,
    pub translation_aid: Option<String>,
    pub image_search_aid: Option<String>,
}

impl fmt::Display for Word {
    /// Display method for a Word
    ///
    /// TODO: Use suffix and define if a space should be used or not
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.word
        )
    }
}

/// Return a list of WordPack found in the `resourses/word_pack` folder
pub fn load() -> Vec<WordPack> {
    fs::read_dir(resources::WORD_PACK_FOLDER)
        .expect("Unable to read word pack folder!")
        .into_iter()
        .filter_map(Result::ok)
        .filter_map(|entry| match entry.file_type() {
            Ok(file_type) => {
                if file_type.is_file() {
                    Some(WordPack::load(entry.path().as_path())?)
                } else {
                    None
                }
            }
            Err(_) => None,
        })
        .collect()
}

/// Return a list of available languages
///
/// Currently an available language is defined by having a icon in the `resources/icons` folder.
///
/// To add a new language option, just add a new icon. The icon name must be a valid language for the [yandex translate](https://translate.yandex.com/).
///
/// For example `en.png`.
///
pub fn available_knonw_languages() -> Vec<String> {
    fs::read_dir(resources::LANGUAGES_FOLDER)
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
