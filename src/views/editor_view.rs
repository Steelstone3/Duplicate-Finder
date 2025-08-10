use crate::{commands::messages::Message, models::duplicate_finder::DuplicateFinder};
use iced::widget::{Column, button, column, text, text_editor};
use iced_aw::Card;

impl DuplicateFinder {
    pub fn editor_view(&self) -> Column<'_, Message> {
        let contents = column!()
            .push(text("Duplicate Text"))
            .spacing(10)
            .push(button("Refresh Duplicate Search").on_press(Message::DuplicateSearchPressed))
            .padding(10)
            .spacing(10)
            .push(
                text_editor(&self.application_state.content)
                    .placeholder("Type something here...")
                    .on_action(Message::EditorChanged),
            )
            .spacing(10)
            .padding(10);

        column!()
            .push(Card::new("Editor", contents))
            .spacing(10)
            .padding(10)
    }
}
