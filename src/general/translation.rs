use serde::{Deserialize, Serialize};
use attohttpc;
use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub code: u16,
    pub lang: String,
    pub text: Vec<String>,
}

pub fn get_translation(word: &Word, from: &str, to: &str, state: &State) -> Response {
    let res = attohttpc::get("https://translate.yandex.net/api/v1.5/tr.json/translate")
        .param("key", &state.tranlation_pair.1)
        .param("lang", format!("{}-{}", from, to))
        .param("text", &word.word)
        .param("format", "plain")
        .send()
        .unwrap();
    res.json().unwrap()
}
