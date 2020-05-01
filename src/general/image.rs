use super::*;
use reqwest;
use serde::{Deserialize, Serialize};
use std::fs::{create_dir_all, OpenOptions};
use std::io::Write;

async fn checked_download_image(url: String) -> Option<String> {
    create_dir_all("temp").ok()?;
    let res = reqwest::Client::new().get(&url).send().await.ok()?;

    let path = format!(
        "temp/{}",
        reqwest::Url::parse(&url).ok()?.path_segments()?.last()?
    );
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(&path)
        .ok()?;

    file.write_all(&res.bytes().await.ok()?).ok()?;
    Some(path)
}

pub async fn download_image(url: String, index: usize) -> Message {
    match checked_download_image(url).await {
        Some(path) => Message::ImageDownloaded(index, path),
        None => Message::Error(Error::ErrorDownloadingImage(index)),
    }
}

async fn checked_get_images_url(word: Word, key: String) -> Option<Response> {
    let search_string = if let Some(search_string) = &word.image_search_aid {
        search_string
    } else {
        &word.word
    };

    let res = reqwest::Client::new()
        .get("https://pixabay.com/api/")
        .query(&[
            ("key", &key),
            ("per_page", &"4".to_string()),
            ("q", search_string),
        ])
        .send()
        .await
        .ok()?
        .json::<Response>()
        .await
        .ok()?;
    Some(res)
}

pub async fn get_images_url(word: Word, key: String) -> Message {
    match checked_get_images_url(word, key).await {
        Some(res) => {
            Message::RequestImages(res.hits.into_iter().map(|hit| hit.webformat_url).collect())
        }
        None => Message::Error(Error::ErrorRequestingImages),
    }
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
