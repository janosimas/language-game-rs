use iced::{Align, Column, Command, Element, Length, Text};

use super::*;
use crate::general;

pub struct GameView {
    question_view: QuestionView,
    helper_tips_view: HelperTipsView,
    answer_view: AnswerView,
    end_turn_view: EndTurnView,
}

impl GameView {
    pub fn new() -> Self {
        Self {
            question_view: QuestionView::new(),
            helper_tips_view: HelperTipsView::new(),
            answer_view: AnswerView::new(),
            end_turn_view: EndTurnView::new(),
        }
    }

    pub fn update(&mut self, message: general::Message) -> Command<general::Message> {
        match message {
            general::Message::TranslationDownloaded(_, _) => {
                self.answer_view.update(message);
            }
            general::Message::ImageDownloaded(_, _) => {
                self.helper_tips_view.update(message);
            }
            general::Message::NextTurn => {
                self.end_turn_view.update(message.clone());
                self.answer_view.update(message);
            }
            general::Message::EndTurn => {
                self.helper_tips_view.update(message.clone());
                self.end_turn_view.update(message);
            }
            _ => {}
        }
        Command::none()
    }

    pub fn view(&mut self, context: &Option<general::Context>) -> Element<general::Message> {
        if let Some(context) = context {
            if self.end_turn_view.is_end_of_turn() {
                return self.end_turn_view.view(
                    &context.word_original,
                    self.answer_view.option(context.current_word_index),
                );
            }

            Column::new()
                .spacing(10)
                .padding(50)
                .align_items(Align::Center)
                .height(Length::FillPortion(1))
                .width(Length::FillPortion(5))
                .push(self.question_view.view(&context))
                .push(self.helper_tips_view.view())
                .push(self.answer_view.view())
                .into()
        } else {
            Text::new("No context loaded").into()
        }
    }
}
