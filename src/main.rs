use iced::{executor, Application, Command, Element, Settings};
use rand::seq::SliceRandom;

mod general;
mod gui;

pub fn main() {
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
        Self {
            game_view: gui::GameView::new(),
            language: general::load_language().unwrap(),
            state: general::State::default(),
            context: None,
        }
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
        self.game_view.update(message)
    }

    fn view(&mut self) -> Element<Self::Message> {
        if self.context.is_none() {
            let current_word = self.language.words.choose(&mut rand::thread_rng()).unwrap();

            let options: Vec<&general::Word> = self
                .language
                .words
                .choose_multiple(&mut rand::thread_rng(), 5)
                .collect();

            self.context = Some(general::Context::new(
                current_word.to_string(),
                vec![],
                vec![],
            ))
        }
        self.game_view.view()
    }
}
