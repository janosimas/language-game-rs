use crate::general;
use iced::{Element, HorizontalAlignment, Length, Text};

pub struct QuestionView {}

impl QuestionView {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view(&mut self, context: &general::Context) -> Element<general::Message> {
        Text::new(&context.word_original)
            .height(Length::Units(80))
            .width(Length::FillPortion(1))
            .horizontal_alignment(HorizontalAlignment::Center)
            .size(40)
            .into()
    }
}
