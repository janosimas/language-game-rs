use crate::general;
use iced::{
    button, Align, Button, Column, Command, Element, HorizontalAlignment, Length, Space, Text,
};

pub struct EndTurnView {
    btn_state: button::State,
    answer: Option<general::Answer>,
}

impl EndTurnView {
    pub fn new() -> Self {
        Self {
            btn_state: button::State::new(),
            answer: None,
        }
    }

    pub fn is_end_of_turn(&self) -> bool {
        self.answer.is_some()
    }

    pub fn update(&mut self, message: general::Message) -> Command<general::Message> {
        match message {
            general::Message::NextTurn => {
                self.answer = None;
            }
            general::Message::GameEnd => {
                self.answer = None;
            }
            general::Message::EndTurn(answer) => {
                self.answer = Some(answer);
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
            .push(Space::new(Length::Fill, Length::FillPortion(3)))
            .push(
                Text::new(format!("{:?}", self.answer.as_ref().unwrap()))
                    .width(Length::Fill)
                    .horizontal_alignment(HorizontalAlignment::Center)
                    .size(20),
            ) // TODO: use a proper print method
            .push(Space::new(Length::Fill, Length::FillPortion(3)))
            .push(
                Text::new(word_original.to_string())
                    .width(Length::Fill)
                    .horizontal_alignment(HorizontalAlignment::Center)
                    .size(60),
            )
            .push(Space::new(Length::Fill, Length::Fill))
            .push(
                Text::new(word_translated.unwrap_or(&"Error loading translation".to_string()))
                    .width(Length::Fill)
                    .horizontal_alignment(HorizontalAlignment::Center)
                    .size(40),
            )
            .push(Space::new(Length::Fill, Length::FillPortion(3)))
            .push(
                Button::new(&mut self.btn_state, Text::new("next!"))
                    .on_press(general::Message::NextTurn),
            )
            .push(Space::new(Length::Fill, Length::FillPortion(3)))
            .into()
    }
}
