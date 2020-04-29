use dotenv;
use iced::{Application, Column, Command, Element, Length, Row, Settings, Text};
use rand::seq::SliceRandom;
use std::env;
use std::iter;

mod general;
mod gui;

fn main() {
    dotenv::dotenv().ok();

    Game::run(Settings::default())
}

struct Game {
    game_view: gui::GameView,
    start_view: gui::StartView,
    language: general::Language,
    state: general::State,
    context: Option<general::Context>,
}

impl Game {
    fn new() -> Self {
        let mut tranlation_pair: (String, String) = (String::new(), String::new());
        let mut image_pair: (String, String) = (String::new(), String::new());
        for (key, value) in env::vars() {
            if key == "YANDEX_KEY" {
                tranlation_pair = (key, value);
            } else if key == "PIXABAY_KEY" {
                image_pair = (key, value);
            }
        }

        Self {
            game_view: gui::GameView::new(),
            start_view: gui::StartView::new(),
            language: general::load_language().unwrap(),
            state: general::State::new(tranlation_pair, image_pair),
            context: None,
        }
    }

    fn advance_turn(&mut self) -> Command<general::Message> {
        let mut options: Vec<general::Word> = self
            .language
            .words
            .choose_multiple(&mut rand::thread_rng(), 5)
            .cloned()
            .collect();

        let current_word = options.first().unwrap().clone();

        options.shuffle(&mut rand::thread_rng());

        self.context = Some(general::Context::new(
            current_word.to_string(),
            options
                .iter()
                .enumerate()
                .filter_map(|(index, word)| {
                    if *word == current_word {
                        Some(index)
                    } else {
                        None
                    }
                })
                .next()
                .unwrap(),
        ));

        self.state.advance_turn();

        let translations = options
            .iter()
            .cloned()
            .enumerate()
            .map(|(index, word)| {
                general::translation::get_translation(
                    word,
                    self.language.language.clone(),
                    "en".to_string(),
                    index,
                    self.state.tranlation_pair.1.clone(),
                )
            })
            .map(Command::from)
            .collect::<Vec<_>>();
        Command::batch(translations.into_iter().chain(iter::once(Command::from(
            general::image::get_images_url(
                current_word.clone(),
                self.state.image_pair.1.to_string(),
            ),
        ))))
    }
}

impl Application for Game {
    type Executor = iced::executor::Default;
    type Message = general::Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        (Self::new(), Command::none())
    }

    fn title(&self) -> String {
        String::from("Language game")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            general::Message::GameBegin => {
                self.state.start();
                self.advance_turn()
            }
            general::Message::RequestImages(images_uri) => {
                let images = images_uri
                    .into_iter()
                    .enumerate()
                    .map(|(index, url)| {
                        general::image::download_image(url, index)
                    })
                    .map(Command::from)
                    .collect::<Vec<_>>();
                Command::batch(images.into_iter())
            }
            general::Message::TranslationDownloaded(_, _) => self.game_view.update(message),
            general::Message::ImageDownloaded(_, _) => self.game_view.update(message),
            general::Message::GameEnd => Command::none(),
            general::Message::UserInput(user_input) => match user_input {
                general::UserInput::OptionSelected(index) => {
                    let context = self.context.as_ref().unwrap();

                    if index == context.current_word_index {
                        self.state.advance_score();
                        self.state.add_correct_word(&context.word_original);
                    } else {
                        self.state.add_wrong_word(&context.word_original);
                    }

                    self.game_view.update(general::Message::EndTurn);
                    self.advance_turn()
                }
                general::UserInput::HintSelected(_) => Command::none(),
                general::UserInput::OptionWritten(_) => Command::none(),
            },
            general::Message::EndTurn => self.game_view.update(message),
        }
    }

    fn view(&mut self) -> Element<Self::Message> {
        if !self.state.is_game_running() {
            self.start_view.view().into()
        } else {
            Row::new()
                .push(self.game_view.view(&self.context))
                .push(
                    Column::new()
                        .spacing(10)
                        .height(Length::FillPortion(1))
                        .width(Length::FillPortion(1))
                        .push(Text::new(format!("score: {}", &self.state.score())))
                        .push(Text::new(format!("turn: {}", &self.state.turn()))),
                )
                .into()
        }
    }
}
