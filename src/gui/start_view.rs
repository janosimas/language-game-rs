use iced::{button, Align, Button, Column, Command, Element, Image, Length, Radio, Row, Text};

use crate::general;

struct LanguageButton {
    language: String,
    state: button::State,
}

impl LanguageButton {
    fn view(&mut self) -> Element<general::Message> {
        Button::new(
            &mut self.state,
            Image::new(format!("resources/icons/{}.png", self.language)).width(Length::Units(50)),
        )
        .on_press(general::Message::GameBegin(self.language.clone()))
        .into()
    }
}

pub struct StartView {
    available_word_packs: Vec<general::language::Language>,
    known_languages: Vec<LanguageButton>,
    selected_word_pack: Option<usize>,
}

impl StartView {
    pub fn new() -> Self {
        let known_languages_str = vec!["en".to_string(), "pt".to_string(), "de".to_string()];
        let known_languages = known_languages_str
            .into_iter()
            .map(|language| LanguageButton {
                language,
                state: button::State::new(),
            })
            .collect();
        let available_word_packs = general::word_pack::load();
        Self {
            available_word_packs,
            known_languages,
            selected_word_pack: Some(0),
        }
    }

    pub fn word_pack(&self) -> general::language::Language {
        self.available_word_packs[self.selected_word_pack.unwrap()].clone()
    }

    pub fn update(&mut self, message: general::Message) -> Command<general::Message> {
        match message {
            general::Message::UserInput(general::UserInput::WordPackSelected(index)) => {
                self.selected_word_pack = Some(index)
            }
            _ => {}
        }
        Command::none()
    }

    fn word_packs_radio<'a>(
        available_word_packs: &Vec<general::language::Language>,
        option: &Option<usize>,
    ) -> Element<'a, general::Message> {
        available_word_packs
            .iter()
            .enumerate()
            .fold(Column::new(), |col, (index, language)| {
                col.push(Radio::new(
                    index,
                    &language.description,
                    option.clone(),
                    |index| {
                        general::Message::UserInput(general::UserInput::WordPackSelected(index))
                    },
                ))
            })
            .into()
    }

    fn known_languages_buttons(known_languages: &mut Vec<LanguageButton>) -> Element<general::Message> {
        Column::new()
            .spacing(10)
            .padding(50)
            .align_items(Align::Center)
            .height(Length::FillPortion(1))
            .push(Text::new("Select the known language:"))
            .push(
                known_languages
                    .iter_mut()
                    .fold(Row::new(), |row, button| row.push(button.view())),
            )
            .into()
    }

    pub fn view(&mut self) -> Element<general::Message> {
        Column::new()
            .spacing(10)
            .padding(50)
            .align_items(Align::Center)
            .height(Length::FillPortion(1))
            .push(StartView::word_packs_radio(
                &self.available_word_packs,
                &self.selected_word_pack,
            ))
            .push(StartView::known_languages_buttons(&mut self.known_languages))
            .into()
    }
}
