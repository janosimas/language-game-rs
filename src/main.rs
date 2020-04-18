use iced::{executor, Application, Command, Element, Settings};
mod general;
mod gui;

pub fn main() {
    Game::run(Settings::default())
}

struct Game {
    game_view: gui::GameView
}

impl Game {
    fn new() -> Self {
        Self {
            game_view: gui::GameView::new()
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
        self.game_view.view()
    }
}
