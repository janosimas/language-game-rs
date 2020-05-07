use crate::general;
use iced::{Align, Element, Length, Row, Space, Text};
use log::debug;

pub struct QuestionView {}

impl QuestionView {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view(&mut self, context: &general::Context) -> Element<general::Message> {
        let mut row = Row::new()
            .width(Length::FillPortion(1))
            .spacing(10)
            .height(Length::Units(80))
            .align_items(Align::End);

        row = row.push(Space::new(Length::Fill, Length::Shrink));

        if let Some(prefix) = context.word_original.prefix.as_ref() {
            debug!("current word prefix: {}", prefix);
            row = row.push(Text::new(prefix).size(40));
        }

        debug!("current word: {}", context.word_original.to_string());
        row = row.push(
            Text::new(&context.word_original.to_string())
                .size(40)
                .width(Length::Shrink),
        );

        if let Some(sufix) = context.word_original.sufix.as_ref() {
            debug!("current word sufix: {}", sufix);
            row = row.push(Text::new(sufix).size(40));
        }

        row = row.push(Space::new(Length::Fill, Length::Shrink));

        row.into()
    }
}
