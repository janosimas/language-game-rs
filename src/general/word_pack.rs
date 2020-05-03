use super::*;
use std::fs;

pub fn load() -> Vec<language::Language> {
    fs::read_dir("resources/word_pack")
        .expect("Unable to read word pack folder!")
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

pub fn available_languages() -> Vec<String> {
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
