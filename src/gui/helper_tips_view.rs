use crate::general;
use iced::{Command, Element, Length, Row, Text};

pub struct HelperTipsView {}

impl HelperTipsView {
    pub fn new() -> Self {
        Self {}
    }
    pub fn update(&mut self, _message: general::Message) -> Command<general::Message> {
        Command::none()
    }

    pub fn view(&mut self, context: &general::Context) -> Element<general::Message> {
        context
            .helper_tips
            .iter()
            .fold(
                Row::new().spacing(10).width(Length::FillPortion(1)),
                |row, value| row.push(Text::new(value)),
            )
            .into()
    }
}
