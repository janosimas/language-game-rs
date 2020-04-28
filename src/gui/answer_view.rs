use crate::general;
use iced::{button, Button, Command, Element, Length, Row, Text};

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
    pub fn update(&mut self, message: general::Message) -> Command<general::Message> {
        match message {
            general::Message::GameBegin => {}
            general::Message::GameEnd => {}
            general::Message::TranslationFinished(index, value) => {
                self.options[index] = Some(value);
            }
            general::Message::UserInput(_) => {}
        }
        Command::none()
    }

    pub fn view(&mut self, context: &general::Context) -> Element<general::Message> {
        self.button_states
            .iter_mut()
            .zip(&self.options)
            .enumerate()
            .fold(
                Row::new()
                    .spacing(10)
                    .width(Length::FillPortion(1))
                    .height(Length::FillPortion(1)),
                |row, (index, (state, value))| {
                    row.push(
                        Button::new(state, Text::new(value.as_ref().unwrap_or(&"?".to_string())))
                            .on_press(general::Message::UserInput(
                                general::UserInput::OptionSelected(index),
                            )),
                    )
                },
            )
            .into()
    }
}
