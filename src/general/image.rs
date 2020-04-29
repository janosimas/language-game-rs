use super::*;
use reqwest;
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::Write;

pub async fn download_image(url: String, name: String, index: usize) -> Message {
    let path = format!("{}.jpg", name);
    let res = reqwest::Client::new().get(&url).send().await.unwrap();
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(&path)
        .unwrap();
    file.write_all(&res.bytes().await.unwrap());
    Message::ImageDownloaded(index, path)
}

pub async fn get_images_url(word: Word, key: String) -> Message {
    let search_string = if let Some(search_string) = &word.image_search_aid {
        search_string
    } else {
        &word.word
    };

    // TODO: request error handling
    let res = reqwest::Client::new()
        .get("https://pixabay.com/api/")
        .query(&[
            ("key", &key),
            ("per_page", &"4".to_string()),
            ("q", search_string),
        ])
        .send()
        .await
        .unwrap()
        .json::<Response>()
        .await
        .unwrap();

    Message::RequestImages(res.hits.into_iter().map(|hit| hit.webformat_url).collect())
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
