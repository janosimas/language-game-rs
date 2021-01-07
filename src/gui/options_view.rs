use iced::{Command, Element, Text};

use crate::general;

pub struct OptionsView {}

impl OptionsView {
    pub fn new() -> Self {
        Self {}
    }

    pub fn update(&mut self, _message: general::Message) -> Command<general::Message> {
        todo!()
    }
    pub fn view(&mut self) -> Element<general::Message> {
        Text::new("Options::view").into()
    }
}
