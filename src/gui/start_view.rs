use iced::{
    button, scrollable, Align, Button, Column, Command, Element, HorizontalAlignment, Image,
    Length, Radio, Row, Scrollable, Text,
};

use crate::general;

struct LanguageButton {
    language: String,
    state: button::State,
}

impl LanguageButton {
    fn view(&mut self) -> Element<general::Message> {
        Button::new(
            &mut self.state,
            Image::new(format!(
                "{}/{}.png",
                general::resources::LANGUAGES_FOLDER,
                self.language
            ))
            .width(Length::Units(50)),
        )
        .on_press(general::Message::GameBegin(self.language.clone()))
        .into()
    }
}

pub struct StartView {
    available_word_packs: Vec<general::word_pack::WordPack>,
    known_languages: Vec<LanguageButton>,
    selected_word_pack: Option<usize>,
    scroll_state: scrollable::State,
    setting_btn_state: button::State,
}

impl StartView {
    pub fn new() -> Self {
        let known_languages_str = general::word_pack::available_knonw_languages();
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
            scroll_state: scrollable::State::new(),
            setting_btn_state: button::State::default(),
        }
    }

    pub fn word_pack(&self) -> general::word_pack::WordPack {
        self.available_word_packs[self.selected_word_pack.unwrap()].clone()
    }

    pub fn update(&mut self, message: general::Message) -> Command<general::Message> {
        if let general::Message::UserInput(general::UserInput::WordPackSelected(index)) = message {
            self.selected_word_pack = Some(index)
        }
        Command::none()
    }

    fn word_packs_radio<'a>(
        available_word_packs: &[general::word_pack::WordPack],
        option: &Option<usize>,
    ) -> Element<'a, general::Message> {
        available_word_packs
            .iter()
            .enumerate()
            .fold(
                Column::new().width(Length::Fill),
                |col, (index, language)| {
                    col.push(Radio::new(index, &language.description, *option, |index| {
                        general::Message::UserInput(general::UserInput::WordPackSelected(index))
                    }))
                },
            )
            .into()
    }

    fn known_languages_buttons(
        known_languages: &mut Vec<LanguageButton>,
    ) -> Element<general::Message> {
        Column::new()
            .spacing(10)
            .padding(50)
            .align_items(Align::Center)
            .height(Length::Fill)
            .width(Length::Fill)
            .align_items(Align::Start)
            .push(
                Text::new("Select the known language:")
                    .horizontal_alignment(HorizontalAlignment::Left),
            )
            .push(
                known_languages
                    .iter_mut()
                    .fold(Row::new().width(Length::Fill), |row, button| {
                        row.push(button.view())
                    }),
            )
            .into()
    }

    pub fn view(&mut self) -> Element<general::Message> {
        Column::new()
            .spacing(10)
            .padding(50)
            .align_items(Align::Center)
            .height(Length::Fill)
            .push(
                Row::new()
                    .height(Length::Fill)
                    .push(
                        Scrollable::new(&mut self.scroll_state)
                            .height(Length::Fill)
                            .width(Length::FillPortion(3))
                            .push(StartView::word_packs_radio(
                                &self.available_word_packs,
                                &self.selected_word_pack,
                            )),
                    )
                    .push(
                        Button::new(&mut self.setting_btn_state, Text::new("Settings"))
                            .on_press(general::Message::Options(general::Options::Start)),
                    ),
            )
            .push(StartView::known_languages_buttons(
                &mut self.known_languages,
            ))
            .into()
    }
}
