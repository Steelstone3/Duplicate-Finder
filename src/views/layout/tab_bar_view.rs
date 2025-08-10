use crate::{
    commands::{messages::Message, tab_identifier::TabIdentifier},
    models::duplicate_finder::DuplicateFinder,
};
use iced::widget::{Column, Scrollable, column};
use iced_aw::{TabBar, TabLabel};

impl DuplicateFinder {
    pub fn tab_bar_view(&self) -> Column<'_, Message> {
        match self.application_state.tab {
            TabIdentifier::Editor => {
                let tab_bar = selected_tab_bar(&TabIdentifier::Editor);

                let contents = Scrollable::new(column!().push(self.editor_view()));

                column!(self.menu_view(), tab_bar, contents)
            }
            TabIdentifier::DuplicateFinder => {
                let tab_bar = selected_tab_bar(&TabIdentifier::DuplicateFinder);

                let contents = Scrollable::new(column!().push(self.duplicate_finder_view()));

                column!(self.menu_view(), tab_bar, contents)
            }
        }
    }
}

fn selected_tab_bar(active_tab: &TabIdentifier) -> TabBar<'static, Message, TabIdentifier> {
    TabBar::new(Message::SelectedTabChanged)
        .push(
            TabIdentifier::Editor,
            TabLabel::IconText('\u{1F4D3}', "Editor".to_string()),
        )
        .push(
            TabIdentifier::DuplicateFinder,
            TabLabel::IconText('\u{1F50D}', "Duplicate Finder".to_string()),
        )
        .set_active_tab(active_tab)
}
