use crate::general;
use iced::{button, Align, Button, Column, Command, Element, Length, Text};

pub struct EndTurnView {
    is_end_of_turn: bool,
    btn_state: button::State,
}

impl EndTurnView {
    pub fn new() -> Self {
        Self {
            is_end_of_turn: false,
            btn_state: button::State::new(),
        }
    }

    pub fn is_end_of_turn(&self) -> bool {
        self.is_end_of_turn
    }

    pub fn update(&mut self, message: general::Message) -> Command<general::Message> {
        match message {
            general::Message::NextTurn => {
                self.is_end_of_turn = false;
            }
            general::Message::EndTurn => {
                self.is_end_of_turn = true;
            }
            _ => {}
        }
        Command::none()
    }

    pub fn view(
        &mut self,
        word_original: &general::word_pack::Word,
        word_translated: Option<&String>,
    ) -> Element<general::Message> {
        Column::new()
            .spacing(10)
            .padding(50)
            .align_items(Align::Center)
            .height(Length::FillPortion(1))
            .width(Length::FillPortion(5))
            .push(Text::new(word_original.to_string()))
            .push(Text::new(
                word_translated.unwrap_or(&"Error loading translation".to_string()),
            ))
            .push(
                Button::new(&mut self.btn_state, Text::new("next!"))
                    .on_press(general::Message::NextTurn),
            )
            .into()
    }
}
