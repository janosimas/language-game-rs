use chrono;
use dotenv;
use fern;
use iced::{Application, Column, Command, Element, Length, Row, Settings, Text};
use log::{error, info};
use rand::seq::SliceRandom;
use std::env;
use std::iter;

mod general;
mod gui;

fn main() {
    dotenv::dotenv().ok();

    // Configure logger at runtime
    fern::Dispatch::new()
        // Perform allocation-free log formatting
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        // Add blanket level filter -
        .level(log::LevelFilter::Debug)
        // disable libraries logging
        .level_for("hyper", log::LevelFilter::Off)
        // .level_for("reqwest", log::LevelFilter::Off)
        .level_for("iced_winit", log::LevelFilter::Off)
        .level_for("iced_wgpu", log::LevelFilter::Off)
        .level_for("wgpu_native", log::LevelFilter::Off)
        .level_for("gfx_backend_vulkan", log::LevelFilter::Off)
        .level_for("gfx_backend_dx11", log::LevelFilter::Off)
        // Output to stdout, files, and other Dispatch configurations
        .chain(std::io::stdout())
        // Apply globally
        .apply()
        .unwrap();

    Game::run(Settings::default())
}

struct Game {
    game_view: gui::GameView,
    start_view: gui::StartView,
    end_view: gui::EndView,
    word_pack: Option<general::word_pack::WordPack>,
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
            end_view: gui::EndView::new(),
            word_pack: None,
            state: general::State::new(tranlation_pair, image_pair),
            context: None,
        }
    }

    fn advance_turn(&mut self) -> Command<general::Message> {
        let mut options =
            general::word_pack::select_random_words(&self.word_pack.as_ref().unwrap(), 5);

        // get the first word to use as "question word"
        // should never fail
        let current_word = options.first().unwrap().clone();

        // shuffle the options
        // we got the first one from the list, remember?
        options.shuffle(&mut rand::thread_rng());

        // Create a contex with the current word and the index of it in the options list
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

        // Advance the turn in the game state
        self.state.advance_turn();

        // create a list of future for the translations
        // these futures will send a message to update the interface with the translations
        let translations = options
            .iter()
            .cloned()
            .enumerate()
            .map(|(index, word)| {
                general::translation::get_translation(
                    word,
                    self.word_pack.as_ref().unwrap().language.clone(),
                    self.state.known_language.clone(),
                    index,
                    self.state.tranlation_pair.1.clone(),
                )
            })
            .map(Command::from)
            .collect::<Vec<_>>();
        // convert the futures to iced::Command
        // and create a future for the list of images
        Command::batch(translations.into_iter().chain(iter::once(Command::from(
            general::image::get_images_url(
                current_word.clone(),
                self.state.image_pair.1.to_string(),
            ),
        ))))
    }
}

impl Drop for Game {
    fn drop(&mut self) {
        self.update(general::Message::GameEnd);
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
            general::Message::GameBegin(known_language) => {
                self.state.known_language = known_language;
                self.word_pack = Some(self.start_view.word_pack());
                self.state.start();
                self.advance_turn()
            }
            general::Message::RequestImages(images_uri) => {
                let images = images_uri
                    .into_iter()
                    .enumerate()
                    .map(|(index, url)| general::image::download_image(url, index))
                    .map(Command::from)
                    .collect::<Vec<_>>();
                Command::batch(images.into_iter())
            }
            general::Message::TranslationDownloaded(_, _) => self.game_view.update(message),
            general::Message::ImageDownloaded(_, _) => self.game_view.update(message),
            general::Message::EndTurn => self.game_view.update(message),
            general::Message::GameEnd => {
                self.game_view.update(general::Message::EndTurn);
                info!("Game ended!!!");
                Command::none()
            }
            general::Message::Error(_) => {
                error!("Some error happened!!!");
                Command::none()
            }
            general::Message::UserInput(user_input) => match user_input {
                general::UserInput::OptionSelected(index) => {
                    let context = self.context.as_ref().unwrap();

                    if index == context.current_word_index {
                        self.state.advance_score();
                        self.state.add_correct_word(&context.word_original);
                    } else {
                        self.state.add_wrong_word(&context.word_original);
                    }

                    if self.state.has_game_ended() {
                        return Command::from((|| async { general::Message::GameEnd })());
                    }

                    self.game_view.update(general::Message::EndTurn);
                    self.advance_turn()
                }
                general::UserInput::HintSelected(_) => Command::none(),
                general::UserInput::OptionWritten(_) => Command::none(),
                general::UserInput::WordPackSelected(_) => self
                    .start_view
                    .update(general::Message::UserInput(user_input)),
            },
        }
    }

    fn view(&mut self) -> Element<Self::Message> {
        match self.state.game_state() {
            general::GameState::NotRunning => self.start_view.view().into(),
            general::GameState::Ended => self.end_view.view().into(),
            general::GameState::Running => Row::new()
                .push(self.game_view.view(&self.context))
                .push(
                    Column::new()
                        .spacing(10)
                        .height(Length::FillPortion(1))
                        .width(Length::FillPortion(1))
                        .push(Text::new(format!("score: {}", &self.state.score())))
                        .push(Text::new(format!("turn: {}", &self.state.turn()))),
                )
                .into(),
        }
    }
}
