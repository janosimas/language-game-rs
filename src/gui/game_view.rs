use iced::{Column, Command, Element};

use super::*;
use crate::general;

pub struct GameView {
    question_view: QuestionView,
    helper_tips_view: HelperTipsView,
    answer_view: AnswerView,
}

impl GameView {
    pub fn new() -> Self {
        Self {
            question_view: QuestionView::new(),
            helper_tips_view: HelperTipsView::new(),
            answer_view: AnswerView::new(),
        }
    }

    pub fn update(&mut self, _message: general::Message) -> Command<general::Message> {
        Command::none()
    }

    pub fn view(&mut self) -> Element<general::Message> {
        Column::new()
            .spacing(10)
            .push(self.question_view.view())
            .push(self.helper_tips_view.view())
            .push(self.answer_view.view())
            .into()
    }
}
