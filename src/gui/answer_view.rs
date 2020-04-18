use crate::general;
use iced::{Command, Element, Row, Text};

pub struct AnswerView {}

impl AnswerView {
    pub fn new() -> Self {
        Self {}
    }
    pub fn update(&mut self, _message: general::Message) -> Command<general::Message> {
        Command::none()
    }

    pub fn view(&mut self, context: &general::Context) -> Element<general::Message> {
        context
            .options_transl
            .iter()
            .fold(Row::new().spacing(10), |row, value| {
                row.push(Text::new(value))
            })
            .into()
    }
}
