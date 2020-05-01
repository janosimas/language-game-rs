use crate::general;
use iced::{Element, Text, Length};

pub struct QuestionView {}

impl QuestionView {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view(&mut self, context: &general::Context) -> Element<general::Message> {
        Text::new(&context.word_original)
            .width(Length::FillPortion(1))
            .into()
    }
}
