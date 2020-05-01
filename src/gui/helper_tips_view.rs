use crate::general;
use iced::{Column, Command, Element, Image, Length, Row};
use log::{error};
use std::fs;

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
                        if let Some(path) = file.as_ref() {
                            if let Err(err) = fs::remove_file(path) {
                                error!("{}", err);
                            }
                        }
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
                                        .unwrap_or(&"resources/icons/download.png".to_string()),
                                )
                                .width(Length::FillPortion(1)),
                            )
                            .push(
                                Image::new(
                                    &images[1]
                                        .as_ref()
                                        .unwrap_or(&"resources/icons/download.png".to_string()),
                                )
                                .width(Length::FillPortion(1)),
                            ),
                    )
                },
            )
            .into()
    }
}
