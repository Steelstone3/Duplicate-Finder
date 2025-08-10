use crate::{commands::messages::Message, models::duplicate_finder::DuplicateFinder};
use iced::{
    Renderer, Theme,
    widget::{Column, button, column, text},
};
use iced_aw::Card;

impl DuplicateFinder {
    pub fn duplicate_finder_view(&self) -> Column<'_, Message> {
        let mut contents = column![];

        if !self.duplicate_lines.is_empty() {
            contents = contents
                .push(button("Remove Duplicates").on_press(Message::DuplicateRemovePressed))
                .padding(10)
                .spacing(10);

            for duplicate_lines in self.duplicate_line_cards() {
                contents = contents.push(duplicate_lines).spacing(10);
            }
        }

        contents
    }

    fn duplicate_line_cards(&self) -> Vec<Card<'_, Message, Theme, Renderer>> {
        let mut duplicate_line_cards = vec![];

        for duplicate_line in &self.duplicate_lines {
            duplicate_line_cards.push(Card::new(
                "Duplicate Line",
                text(duplicate_line.to_string()),
            ))
        }

        duplicate_line_cards
    }
}
