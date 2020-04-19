use dotenv;
use iced::{executor, Application, Column, Command, Element, Length, Row, Settings, Text};
use rand::seq::SliceRandom;
use std::env;

mod general;
mod gui;

pub fn main() {
    dotenv::dotenv().ok();

    Game::run(Settings::default())
}

struct Game {
    game_view: gui::GameView,
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
            language: general::load_language().unwrap(),
            state: general::State::new(tranlation_pair, image_pair),
            context: None,
        }
    }

    fn advance_turn(&mut self) {
        let options: Vec<&general::Word> = self
            .language
            .words
            .choose_multiple(&mut rand::thread_rng(), 5)
            .collect();

        let mut translations: Vec<String> = options
            .iter()
            .map(|word| {
                general::get_translation(word, &self.language.language, "en", &self.state)
                    .first()
                    .unwrap()
                    .clone()
            })
            .collect();

        let current_word = options.first().unwrap();
        let current_translation = translations.first().unwrap().clone();

        translations.shuffle(&mut rand::thread_rng());

        let images_uri: Vec<String> = general::get_images_url(current_word, &self.state);
        let images_path: Vec<String> = images_uri
            .iter()
            .enumerate()
            .map(|(index, url)| general::download_image(&url, &index.to_string()))
            .collect();

        self.context = Some(general::Context::new(
            current_word.to_string(),
            current_translation,
            translations,
            vec![],
        ));

        self.state.advance_turn();
    }
}

impl Application for Game {
    type Executor = executor::Null;
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
                self.advance_turn();
            }
            general::Message::GameEnd => {}
            general::Message::UserInput(user_input) => match user_input {
                general::UserInput::OptionSelected(index) => {
                    let context = self.context.as_ref().unwrap();

                    let selected_option = &context.options_translation[index];
                    if selected_option == &context.word_translation {
                        self.state.advance_score();
                        self.state.add_correct_word(&context.word_original);
                    } else {
                        self.state.add_wrong_word(&context.word_original);
                    }

                    self.advance_turn();
                }
                general::UserInput::HintSelected(_) => {}
                general::UserInput::OptionWritten(_) => {}
            },
        }
        Command::none()
    }

    fn view(&mut self) -> Element<Self::Message> {
        if self.context.is_none() {
            self.advance_turn();
        }
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
