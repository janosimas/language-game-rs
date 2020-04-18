use iced::{Column, Command, Element, Text};
use crate::general;

pub struct AnswerView {}

impl AnswerView {
    pub fn new() -> Self {
        Self {}
    }
    pub fn update(&mut self, _message: general::Message) -> Command<general::Message> {
        Command::none()
    }

    pub fn view(&mut self) -> Element<general::Message> {
        Text::new("Answer").into()
    }
}
