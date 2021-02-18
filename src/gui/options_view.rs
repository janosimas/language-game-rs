use general::Options;
use iced::{text_input, Column, Command, Element, Text, TextInput};

use crate::general;

pub struct OptionsView {
    target_score_state: text_input::State,
    temp_target_score: String,
}

impl OptionsView {
    pub fn new() -> Self {
        Self {
            target_score_state: text_input::State::default(),
            temp_target_score: String::default(),
        }
    }

    pub fn update(&mut self, message: general::Message) -> Command<general::Message> {
        if let general::Message::Options(Options::AlterTargetScore(new_target_score)) = message {
            if new_target_score.parse::<u32>().is_ok() || new_target_score.is_empty() {
                self.temp_target_score = new_target_score;
            }
        }

        Command::none()
    }
    pub fn view(&mut self) -> Element<general::Message> {
        Column::new()
            .push(Text::new("Options::view"))
            .push(TextInput::new(
                &mut self.target_score_state,
                "Target score value",
                &self.temp_target_score,
                |new_target_score| {
                    general::Message::Options(Options::AlterTargetScore(new_target_score))
                },
            ))
            .into()
    }
}
