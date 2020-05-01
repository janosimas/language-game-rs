use iced::{button, text_input, Align, Button, Command, Element, Length, Row, Text, TextInput};

use crate::general;

pub struct StartView {
    start_btn_state: button::State,
    language_input: text_input::State,
    known_language: String,
}

impl StartView {
    pub fn new() -> Self {
        Self {
            start_btn_state: button::State::new(),
            language_input: text_input::State::new(),
            known_language: "en".to_string(),
        }
    }

    pub fn update(&mut self, message: general::Message) -> Command<general::Message> {
        match message {
            general::Message::GuiUpdated(update) => match update {
                general::GuiUpdate::LanguageUpdated(language) => {
                    self.known_language = language;
                    Command::none()
                }
            },
            _ => Command::none(),
        }
    }

    pub fn view(&mut self) -> Element<general::Message> {
        Row::new()
            .spacing(10)
            .padding(50)
            .align_items(Align::Center)
            .height(Length::FillPortion(1))
            .width(Length::FillPortion(3))
            .push(TextInput::new(
                &mut self.language_input,
                "known language",
                &mut self.known_language,
                |new_text| {
                    general::Message::GuiUpdated(general::GuiUpdate::LanguageUpdated(new_text))
                },
            ))
            .push(
                Button::new(&mut self.start_btn_state, Text::new("Start"))
                    .on_press(general::Message::GameBegin(self.known_language.clone())),
            )
            .into()
    }
}
