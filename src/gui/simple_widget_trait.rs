use iced::{Command, Element};

use crate::general;

pub trait SimpleWidget {
    fn update(&mut self, _message: general::Message) -> Command<general::Message> {
        Command::none()
    }

    fn view(&mut self) -> Element<general::Message>;
}
