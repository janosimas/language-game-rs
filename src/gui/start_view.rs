use iced::{button, Align, Button, Column, Command, Element, Image, Length, Row, Text};

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

struct WordPackButton {
    index: usize,
    language: general::language::Language,
    state: button::State,
}

impl WordPackButton {
    fn view(&mut self) -> Element<general::Message> {
        Button::new(&mut self.state, Text::new(&self.language.description))
            .on_press(general::Message::UserInput(
                general::UserInput::WordPackSelected(self.index),
            ))
            .into()
    }

    fn word_pack(&self) -> general::language::Language {
        self.language.clone()
    }
}

pub struct StartView {
    available_word_packs: Vec<WordPackButton>,
    known_languages: Vec<LanguageButton>,
    selected_word_pack: Option<usize>,
}

impl StartView {
    pub fn new() -> Self {
        let known_languages_str = vec!["en".to_string(), "pt".to_string()];
        let known_languages = known_languages_str
            .into_iter()
            .map(|language| LanguageButton {
                language,
                state: button::State::new(),
            })
            .collect();
        let available_word_packs_load = general::word_pack::load();
        let available_word_packs = available_word_packs_load
            .into_iter()
            .enumerate()
            .map(|(index, language)| WordPackButton {
                index,
                language,
                state: button::State::new(),
            })
            .collect();
        Self {
            available_word_packs,
            known_languages,
            selected_word_pack: None,
        }
    }

    pub fn word_pack(&self) -> general::language::Language {
        self.available_word_packs[self.selected_word_pack.unwrap()].word_pack()
    }

    fn page1(&mut self) -> Element<general::Message> {
        self.available_word_packs
            .iter_mut()
            .fold(Column::new(), |col, button| col.push(button.view()))
            .into()
    }

    fn page2(&mut self) -> Element<general::Message> {
        Column::new()
            .spacing(10)
            .padding(50)
            .align_items(Align::Center)
            .height(Length::FillPortion(1))
            .push(Text::new("Select the known language:"))
            .push(
                self.known_languages
                    .iter_mut()
                    .fold(Row::new(), |row, button| row.push(button.view())),
            )
            .into()
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

    pub fn view(&mut self) -> Element<general::Message> {
        match self.selected_word_pack {
            None => self.page1(),
            Some(_) => self.page2(),
        }
    }
}
