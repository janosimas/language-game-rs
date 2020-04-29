use crate::general;
use iced::{Column, Command, Element, Image, Length, Row};
use std::fs;
use std::iter;

pub struct HelperTipsView {
    helper_tips: Vec<Option<String>>,
}

impl HelperTipsView {
    pub fn new() -> Self {
        Self {
            helper_tips: vec![None, None, None, None],
        }
    }
    pub fn update(&mut self, message: general::Message) -> Command<general::Message> {
        match message {
            general::Message::ImageDownloaded(index, value) => {
                self.helper_tips[index] = Some(value);
            }
            general::Message::EndTurn => {
                self.helper_tips = self
                    .helper_tips
                    .iter()
                    .map(|file| {
                        // TODO: not working. Probably the gui must close the image first.
                        fs::remove_file(file.as_ref().unwrap());
                        None
                    })
                    .collect()
            }
            _ => {}
        }
        Command::none()
    }

    pub fn view(&mut self) -> Element<general::Message> {
        self.helper_tips
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
                            .push(
                                Image::new(
                                    &images[0]
                                        .as_ref()
                                        .unwrap_or(&"resources/loading.jpg".to_string()),
                                )
                                .width(Length::FillPortion(1)),
                            )
                            .push(
                                Image::new(
                                    &images[1]
                                        .as_ref()
                                        .unwrap_or(&"resources/loading.jpg".to_string()),
                                )
                                .width(Length::FillPortion(1)),
                            ),
                    )
                },
            )
            .into()
    }
}
