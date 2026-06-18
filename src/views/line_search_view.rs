use crate::{commands::messages::Message, models::duplicate_finder::DuplicateFinder};
use iced::widget::{column, text_input};
use iced::{
    Renderer, Theme,
    widget::{Column, button, text},
};
use iced_aw::Card;

impl DuplicateFinder {
    pub fn line_search_view(&self) -> Column<'_, Message> {
        let mut contents = column![];

        contents = contents
            .push(text("Line Search:"))
            .padding(10)
            .spacing(10)
            .push(
                text_input("Line Search Term", &self.search_keyword)
                    .on_input(Message::SearchKeywordChanged),
            )
            .push(button("Search").on_press(Message::FindLinesPressed))
            .padding(10)
            .spacing(10)
            .push(button("Remove Found Lines").on_press(Message::FoundLinesRemovedPressed))
            .padding(10)
            .spacing(10);

        for found_lines in self.line_search_cards() {
            contents = contents.push(found_lines).spacing(10);
        }

        contents
    }

    fn line_search_cards(&self) -> Vec<Card<'_, Message, Theme, Renderer>> {
        let mut found_line_cards = vec![];

        for found_line in &self.found_lines {
            found_line_cards.push(Card::new("Found Line", text(found_line.to_string())))
        }

        found_line_cards
    }
}
