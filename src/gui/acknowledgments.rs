use crate::general;
use iced::{Column, Element, Text};

pub fn acknowledgments() -> Element<'static, general::Message> {
    Column::new()
        .push(Text::new(
            "Powered by Yandex.Translate (http://translate.yandex.com)",
        ))
        .push(Text::new("Powered by Pixabay (https://pixabay.com/)"))
        .into()
}
