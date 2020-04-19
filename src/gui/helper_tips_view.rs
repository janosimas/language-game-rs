use crate::general;
use iced::{Column, Command, Element, Image, Length, Row};

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
            .chunks(2)
            .fold(
                Column::new()
                    .spacing(10)
                    .width(Length::FillPortion(1))
                    .height(Length::FillPortion(1)),
                |col, images| {
                    col.push(
                        Row::new()
                            .spacing(10)
                            .width(Length::FillPortion(1))
                            .height(Length::FillPortion(1))
                            .push(Image::new(&images[0]).width(Length::FillPortion(1)))
                            .push(Image::new(&images[1]).width(Length::FillPortion(1))),
                    )
                },
            )
            .into()
    }
}
