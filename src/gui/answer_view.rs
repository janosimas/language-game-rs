use crate::general;
use iced::{Column, Command, Element, Text};

pub struct AnswerView {}

impl AnswerView {
    pub fn new() -> Self {
        Self {}
    }
    pub fn update(&mut self, _message: general::Message) -> Command<general::Message> {
        Command::none()
    }

    pub fn view(&mut self, context: &general::Context) -> Element<general::Message> {
        Text::new(context.options_transl.first().unwrap()).into()
    }
}
