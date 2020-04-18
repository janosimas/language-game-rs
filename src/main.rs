use iced::{executor, Application, Column, Command, Element, Settings, Text};

pub fn main() {
    Game::run(Settings::default())
}

struct Game {
}

impl Game {
    fn new() -> Self {
        Self {
        }
    }
}

impl Application for Game {
    type Executor = executor::Null;
    type Message = ();
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        (Self::new(), Command::none())
    }

    fn title(&self) -> String {
        String::from("Language game")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&mut self) -> Element<Self::Message> {
        Column::new()
            .spacing(10)
            .push(
                Text::new("test")
            )
            .into()
    }
}
