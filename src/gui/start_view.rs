use iced::{
    button, text_input, Align, Button, Command, Element, Image, Length, Row, Text, TextInput,
};

use crate::general;

pub struct StartView {
    start_btn_state1: button::State,
    start_btn_state2: button::State,
}

impl StartView {
    pub fn new() -> Self {
        Self {
            start_btn_state1: button::State::new(),
            start_btn_state2: button::State::new(),
        }
    }

    pub fn view(&mut self) -> Element<general::Message> {
        Row::new()
            .spacing(10)
            .padding(50)
            .align_items(Align::Center)
            .height(Length::FillPortion(1))
            .push(Text::new("Select the known language:"))
            .push(
                Button::new(
                    &mut self.start_btn_state1,
                    Image::new("resources/icons/en.png").width(Length::Units(50)),
                )
                .on_press(general::Message::GameBegin("en".to_string())),
            )
            .push(
                Button::new(
                    &mut self.start_btn_state2,
                    Image::new("resources/icons/pt.png").width(Length::Units(50)),
                )
                .on_press(general::Message::GameBegin("pt".to_string())),
            )
            .into()
    }
}
