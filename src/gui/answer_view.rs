use crate::general;
use iced::{button, Button, Command, Element, Row, Text};

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
            .zip(&context.options_transl)
            .enumerate()
            .fold(Row::new().spacing(10), |row, (index, (state, value))| {
                row.push(Button::new(state, Text::new(value)))
            })
            .into()
    }
}
