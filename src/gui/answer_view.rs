use crate::general;
use iced::{button, Button, Command, Element, Length, Row, Text};
use std::iter;

use super::simple_widget_trait;

pub struct AnswerView {
    button_states: Vec<button::State>,
    options: Vec<Option<String>>,
}

impl AnswerView {
    pub fn new() -> Self {
        Self {
            button_states: vec![
                button::State::new(),
                button::State::new(),
                button::State::new(),
                button::State::new(),
                button::State::new(),
            ],
            options: vec![None, None, None, None, None],
        }
    }

    pub fn option(&self, index: usize) -> Option<&String> {
        self.options[index].as_ref()
    }
}

impl simple_widget_trait::SimpleWidget for AnswerView {
    fn view(&mut self) -> Element<general::Message> {
        self.button_states
            .iter_mut()
            .zip(&self.options)
            .enumerate()
            .fold(
                Row::new().spacing(10).width(Length::FillPortion(1)),
                |row, (index, (state, value))| {
                    row.push(
                        Button::new(state, Text::new(value.as_ref().unwrap_or(&"?".to_string())))
                            .width(Length::FillPortion(1))
                            .on_press(general::Message::UserInput(
                                general::UserInput::OptionSelected(index),
                            )),
                    )
                },
            )
            .into()
    }

    fn update(&mut self, message: general::Message) -> Command<general::Message> {
        match message {
            general::Message::TranslationDownloaded(index, value) => {
                self.options[index] = Some(value);
            }
            general::Message::EndTurn(_) => {
                self.options = iter::repeat(None).take(5).collect();
            }
            _ => {}
        }
        Command::none()
    }
}
