use iced::{Column, Command, Element, Length, Row, Text};

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

    pub fn view(&mut self, context: &Option<general::Context>) -> Element<general::Message> {
        if let Some(context) = context {
            Column::new()
                .spacing(10)
                .height(Length::FillPortion(1))
                .width(Length::FillPortion(3))
                .push(self.question_view.view(&context))
                .push(self.helper_tips_view.view(&context))
                .push(self.answer_view.view(&context))
                .into()
        } else {
            Text::new("Not started").into()
        }
    }
}
