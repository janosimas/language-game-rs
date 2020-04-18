use dotenv;
use iced::{executor, Application, Command, Element, Settings};
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

        println!("{}: {}", tranlation_pair.0, tranlation_pair.1);
        println!("{}: {}", image_pair.0, image_pair.1);

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

        let translations: Vec<String> = options
            .iter()
            .map(|word| {
                general::get_translation(word, "de", "en", &self.state)
                    .text
                    .first()
                    .unwrap()
                    .clone()
            })
            .collect();

        let current_word = options.first().unwrap();
        let current_translation = translations.first().unwrap().clone();

        self.context = Some(general::Context::new(
            current_word.to_string(),
            translations,
            vec![],
        ));
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
            general::Message::UserInputReceived => {}
        }
        self.game_view.update(message)
    }

    fn view(&mut self) -> Element<Self::Message> {
        if self.context.is_none() {
            self.advance_turn();
        }
        self.game_view.view(&self.context)
    }
}
