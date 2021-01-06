use iced::{Align, Element, Length, Row, Text};

use crate::general;

use super::simple_widget_trait::SimpleWidget;

pub struct EndView {}

impl EndView {
    pub fn new() -> Self {
        Self {}
    }
}

impl SimpleWidget for EndView {
    fn view(&mut self) -> Element<general::Message> {
        Row::new()
            .spacing(10)
            .padding(50)
            .align_items(Align::Center)
            .height(Length::FillPortion(1))
            .width(Length::FillPortion(3))
            .push(Text::new("Good job!\nYou got 10 correct answers!"))
            .into()
    }
}
