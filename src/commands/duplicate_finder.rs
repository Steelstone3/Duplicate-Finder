use crate::{commands::messages::Message, models::duplicate_finder::DuplicateFinder};

impl DuplicateFinder {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::MenuBar => {}
            Message::SelectedTabChanged(tab_identifier) => self.switch_tab(tab_identifier),
            Message::FileNew => *self = DuplicateFinder::default(),
            Message::EditUndo => {}
            Message::EditRedo => {}
            Message::ViewPrependLineNumbersPressed => self.toggle_prepend_line_numbers(),
            Message::ViewThemeChanged => self.switch_theme(),
            Message::EditorChanged(action) => self.application_state.content.perform(action),
            Message::DuplicateSearchPressed => {
                self.remove_prepend();
                self.find_duplicates();
            }
        }
    }
}
