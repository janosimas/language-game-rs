use iced::{Command, Element, Text};
use crate::general;

pub struct QuestionView {}

impl QuestionView {
    pub fn new() -> Self {
        Self {}
    }
    pub fn update(&mut self, _message: general::Message) -> Command<general::Message> {
        Command::none()
    }

    pub fn view(&mut self, context: &general::Context) -> Element<general::Message> {
        Text::new(&context.word_orig).into()
    }
}
