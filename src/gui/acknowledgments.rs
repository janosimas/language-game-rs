use crate::general;
use iced::{Column, Element, Length, Text};

pub fn full_acknowledgments() -> Element<'static, general::Message> {
    Column::new()
        .height(Length::FillPortion(1))
        .spacing(10)
        .push(
            Text::new("Powered by Google Translate. (https://cloud.google.com/translate)").size(10),
        )
        .push(Text::new("Powered by Pixabay (https://pixabay.com/)").size(10))
        .push(Text::new("Flag Icons made by Freepik from https://www.flaticon.com/").size(10))
        .push(
            Text::new("Download image by Papirus (https://github.com/PapirusDevelopmentTeam)")
                .size(10),
        )
        .into()
}
