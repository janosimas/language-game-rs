use super::*;
use std::fs;

pub fn load() -> Vec<language::Language> {
    fs::read_dir("resources/word_pack")
        .expect("Unable to read character folder!")
        .into_iter()
        .filter_map(Result::ok)
        .filter_map(|entry| match entry.file_type() {
            Ok(file_type) => {
                if file_type.is_file() {
                    Some(language::load(entry.path().as_path())?)
                } else {
                    None
                }
            }
            Err(_) => None,
        })
        .collect()
}
