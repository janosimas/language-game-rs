use crate::general;
use iced::{button, Button, Command, Element, Length, Row, Text};

pub struct AnswerView {
    button_states: Vec<button::State>,
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
        }
    }
    pub fn update(&mut self, _message: general::Message) -> Command<general::Message> {
        Command::none()
    }

    pub fn view(&mut self, context: &general::Context) -> Element<general::Message> {
        self.button_states
            .iter_mut()
            .zip(&context.options_translation)
            .enumerate()
            .fold(
                Row::new()
                    .spacing(10)
                    .width(Length::FillPortion(1))
                    .height(Length::FillPortion(1)),
                |row, (index, (state, value))| {
                    row.push(Button::new(state, Text::new(value)).on_press(
                        general::Message::UserInput(general::UserInput::OptionSelected(index)),
                    ))
                },
            )
            .into()
    }
}
