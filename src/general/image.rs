use super::*;
use attohttpc;
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;

pub fn download_image(url: &str, name: &str) -> String {
    let path = format!("{}.jpg", name);
    let res = attohttpc::get(url).send().unwrap();
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(&path)
        .unwrap();
    res.write_to(file);
    path
}

pub fn get_images_url(word: &Word, state: &State) -> Vec<std::string::String> {
    let search_string = if let Some(search_string) = &word.image_search_aid {
        search_string
    } else {
        &word.word
    };

    // TODO: request error handling
    let res = attohttpc::get("https://pixabay.com/api/")
        .param("key", &state.image_pair.1)
        .param("per_page", 4)
        .param("q", search_string)
        .send()
        .unwrap();
    res.json::<Response>()
        .unwrap()
        .hits
        .iter()
        .map(|hit| hit.webformat_url.clone())
        .collect()
}

#[derive(Serialize, Deserialize)]
pub struct Response {
    total: i64,
    #[serde(rename = "totalHits")]
    total_hits: i64,
    hits: Vec<Hit>,
}

#[derive(Serialize, Deserialize)]
pub struct Hit {
    id: i64,
    #[serde(rename = "pageURL")]
    page_url: String,
    #[serde(rename = "type")]
    hit_type: String,
    tags: String,
    #[serde(rename = "previewURL")]
    preview_url: String,
    #[serde(rename = "previewWidth")]
    preview_width: i64,
    #[serde(rename = "previewHeight")]
    preview_height: i64,
    #[serde(rename = "webformatURL")]
    webformat_url: String,
    #[serde(rename = "webformatWidth")]
    webformat_width: i64,
    #[serde(rename = "webformatHeight")]
    webformat_height: i64,
    #[serde(rename = "largeImageURL")]
    large_image_url: String,
    #[serde(rename = "imageWidth")]
    image_width: i64,
    #[serde(rename = "imageHeight")]
    image_height: i64,
    #[serde(rename = "imageSize")]
    image_size: i64,
    views: i64,
    downloads: i64,
    favorites: i64,
    likes: i64,
    comments: i64,
    user_id: i64,
    user: String,
    #[serde(rename = "userImageURL")]
    user_image_url: String,
}
