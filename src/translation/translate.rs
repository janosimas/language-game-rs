use async_trait::async_trait;
use std::error::Error;

#[async_trait]
pub trait Translate {
    async fn translate(
        &self,
        word: String,
    ) -> Result<String, Box<dyn Error>>;

    fn from(&self) -> String;
    fn to(&self) -> String;
}
