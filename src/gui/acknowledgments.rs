use crate::general;
use iced::{Column, Element, Text};

pub fn full_acknowledgments() -> Element<'static, general::Message> {
    Column::new()
        .spacing(10)
        .push(Text::new("Powered by Google Translate. (https://cloud.google.com/translate)"))
        .push(Text::new("Powered by Pixabay (https://pixabay.com/)"))
        .push(Text::new(
            "Flag Icons made by Freepik from https://www.flaticon.com/",
        ))
        .push(Text::new(
            "Download image by Papirus (https://github.com/PapirusDevelopmentTeam)",
        ))
        .into()
}

