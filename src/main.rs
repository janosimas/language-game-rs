#![warn(clippy::all)]

use general::word_pack;
use gui::{full_acknowledgments, EndView};
use iced::{Application, Column, Command, Element, Length, Row, Settings, Text};
use log::{error, info};
use rand::prelude::SliceRandom;
use std::env;
use std::iter;
use std::sync::Arc;

mod general;
mod gui;
mod translation;

use translation::{get_translator, Translate};

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
        .level_for("wgpu_core", log::LevelFilter::Off)
        .level_for("wgpu", log::LevelFilter::Off)
        .level_for("tracing", log::LevelFilter::Off)
        // Output to stdout, files, and other Dispatch configurations
        .chain(std::io::stdout())
        // Apply globally
        .apply()
        .unwrap();

    Game::run(Settings::default()).expect("Unable to start UI.");
}

enum GuiState {
    START(gui::StartView),
    GAME(gui::GameView, word_pack::WordPack, general::Context),
    OPTIONS(gui::OptionsView),
    END(gui::EndView),
}

struct Game {
    gui_state: GuiState,
    state: general::State,
    translator: Option<Arc<dyn Translate + Sync + Send>>,
}

impl Game {
    fn new() -> Self {
        let mut tranlation_pair: (String, String) = (String::new(), String::new());
        let mut image_pair: (String, String) = (String::new(), String::new());
        for (key, value) in env::vars() {
            if key == "GOOGLE_KEY" {
                tranlation_pair = (key, value);
            } else if key == "PIXABAY_KEY" {
                image_pair = (key, value);
            }
        }

        let state = general::State::new(tranlation_pair, image_pair);

        Self {
            gui_state: GuiState::START(gui::StartView::new()),
            state,
            translator: None,
        }
    }

    pub fn new_context(word_pack: &word_pack::WordPack) -> general::Context {
        let mut options = word_pack.choose_random(5);

        // get the first word to use as "question word"
        // should never fail
        let current_word = options.first().unwrap().clone();

        // shuffle the options
        // we got the first one from the list, remember?
        options.shuffle(&mut rand::thread_rng());

        // Create a contex with the current word and the index of it in the options list
        general::Context::new(
            current_word.clone(),
            options
                .iter()
                .enumerate()
                .find_map(|(index, word)| {
                    if *word == current_word {
                        Some(index)
                    } else {
                        None
                    }
                })
                .unwrap(),
            options,
        )
    }

    fn get_remote_resources(&self) -> Command<general::Message> {
        if let GuiState::GAME(_, _, context) = &self.gui_state {
            let options = &context.options;
            let word_original = context.word_original.clone();

            // create a list of future for the translations
            // these futures will send a message to update the interface with the translations
            let translations = options
                .iter()
                .cloned()
                .enumerate()
                .map(|(index, word)| {
                    Self::translate(index, word, Arc::clone(&self.translator.as_ref().unwrap()))
                })
                .map(Command::from);

            // convert the futures to iced::Command
            // and create a future for the list of images
            Command::batch(translations.chain(iter::once(Command::from(
                general::image::get_images_url(word_original, self.state.image_pair.1.to_string()),
            ))))
        } else {
            Command::none()
        }
    }

    async fn translate(
        index: usize,
        word: general::word_pack::Word,
        translator: Arc<dyn Translate + Sync + Send>,
    ) -> general::Message {
        match translator.translate(word.word).await {
            Ok(val) => general::Message::TranslationDownloaded(index, val),
            Err(err) => {
                error!("Translation error: {}", err);
                general::Message::Error(general::Error::DownloadingTranslation(index))
            }
        }
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
        if let general::Message::Error(_) = message {
            error!("Some error happened!!!");
            return Command::none();
        }

        match &mut self.gui_state {
            GuiState::START(gui) => {
                match &message {
                    general::Message::GameBegin(target_language) => {
                        self.state.target_language = target_language.clone();
                        let word_pack = gui.word_pack();
                        self.translator = Some(get_translator(
                            &word_pack.language,
                            &self.state.target_language,
                            &self.state,
                        ));

                        let context = Game::new_context(&word_pack);
                        // update gui state
                        self.gui_state = GuiState::GAME(gui::GameView::new(), word_pack, context);

                        // Advance the turn in the game state
                        self.state.advance_turn();
                        self.get_remote_resources()
                    }
                    general::Message::UserInput(user_input) => {
                        if let general::UserInput::WordPackSelected(_) = user_input {
                            gui.update(general::Message::UserInput(user_input.clone()))
                        } else {
                            Command::none()
                        }
                    }
                    general::Message::Options(general::Options::Start) => {
                        self.gui_state = GuiState::OPTIONS(gui::OptionsView::new());
                        Command::none()
                    }
                    _ => Command::none(),
                }
            }
            GuiState::GAME(gui, word_pack, context) => match message {
                general::Message::TranslationDownloaded(_, _)
                | general::Message::ImageDownloaded(_, _)
                | general::Message::EndTurn(_) => gui.update(message),
                general::Message::NextTurn => {
                    gui.update(message);
                    *context = Game::new_context(word_pack);
                    self.state.advance_turn();
                    self.get_remote_resources()
                }
                general::Message::GameEnd => {
                    let messages = gui.update(message);
                    self.gui_state = GuiState::END(EndView::new());
                    info!("Game ended!!!");
                    messages
                }
                general::Message::RequestImages(images_uri) => {
                    let images = images_uri
                        .into_iter()
                        .enumerate()
                        .map(|(index, url)| general::image::download(url, index))
                        .map(Command::from);
                    Command::batch(images)
                }
                general::Message::UserInput(user_input) => {
                    if let general::UserInput::OptionSelected(index) = user_input {
                        if index == context.current_word_index {
                            self.state.advance_score();
                            self.state.add_correct_word(&context.word_original);
                            gui.update(general::Message::EndTurn(general::Answer::Correct));
                        } else {
                            self.state.add_wrong_word(&context.word_original);
                            gui.update(general::Message::EndTurn(general::Answer::Wrong));
                        }

                        if self.state.has_game_ended() {
                            return Command::from(async { general::Message::GameEnd });
                        }
                    }

                    Command::none()
                }
                _ => Command::none(),
            },
            GuiState::OPTIONS(gui) => match message {
                general::Message::Options(_) => gui.update(message),
                _ => Command::none(),
            },
            GuiState::END(_gui) => Command::none(),
        }
    }

    fn view(&mut self) -> Element<Self::Message> {
        match &mut self.gui_state {
            GuiState::START(gui) => gui.view(),
            GuiState::GAME(gui, _word_pack, context) => Column::new()
                .push(
                    Row::new()
                        .height(Length::FillPortion(9))
                        .push(gui.view(context))
                        .push(
                            Column::new()
                                .spacing(10)
                                .height(Length::FillPortion(1))
                                .width(Length::FillPortion(1))
                                .push(Text::new(format!("score: {}", &self.state.score())))
                                .push(Text::new(format!("turn: {}", &self.state.turn()))),
                        ),
                )
                .push(full_acknowledgments())
                .into(),
            GuiState::OPTIONS(gui) => gui.view(),
            GuiState::END(gui) => gui.view(),
        }
    }
}
