use iced::{Column, Command, Element, Text};
use crate::general;

pub struct HelperTipsView {}

impl HelperTipsView {
    pub fn new() -> Self {
        Self {}
    }
    pub fn update(&mut self, _message: general::Message) -> Command<general::Message> {
        Command::none()
    }

    pub fn view(&mut self, context: &general::Context) -> Element<general::Message> {
        Text::new("Help").into()
    }
}
