use iced::{Row, Element, Length, button, Button, Align, Text};

use crate::general;

pub struct StartView {
    start_btn_state: button::State
}

impl StartView {
    pub fn new() -> Self {
        Self {
            start_btn_state: button::State::new()
        }
    }

    pub fn view(&mut self) -> Element<general::Message> {
        Row::new()
        .spacing(10)
        .padding(50)
        .align_items(Align::Center)
        .height(Length::FillPortion(1))
        .width(Length::FillPortion(3))
        .push(Button::new(&mut self.start_btn_state, Text::new("Start")).on_press(
            general::Message::GameBegin
        ))
        .into()
    }
}
