use crate::{commands::messages::Message, models::duplicate_finder::DuplicateFinder};

impl DuplicateFinder {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::MenuBar => {}
            Message::SelectedTabChanged(tab_identifier) => self.switch_tab(tab_identifier),
            Message::FileNew => *self = DuplicateFinder::default(),
            Message::EditUndo => {}
            Message::EditRedo => {}
            Message::ViewPrependLineNumbersPressed => {
                let editor_content = self.get_editor_text();
                let content = self.toggle_prepend_line_numbers(editor_content);
                self.refresh_editor(content);
            }
            Message::ViewThemeChanged => self.switch_theme(),
            Message::EditorChanged(action) => self.application_state.content.perform(action),
            Message::DuplicateSearchPressed => {
                let editor_content = self.get_editor_text();
                let content = self.remove_prepend(editor_content);
                self.refresh_editor(content);
                self.find_duplicates(self.get_editor_text());
            }
            Message::DuplicateRemovePressed => {
                let editor_content = self.get_editor_text();
                let content = self.remove_prepend(editor_content);
                self.refresh_editor(content);
                let filtered_content = self.remove_duplicates(self.get_editor_text());
                self.refresh_editor(filtered_content);
                self.find_duplicates(self.get_editor_text());
            }
            Message::SearchKeywordChanged(search_keyword) => {
                self.search_keyword = search_keyword;
                self.find_search_keyword(self.get_editor_text());
            }
            Message::FoundLinesRemovedPressed => {
                let editor_content = self.get_editor_text();
                let content = self.remove_prepend(editor_content);
                self.refresh_editor(content);
                let filtered_content = self.remove_search_keyword(self.get_editor_text());
                self.refresh_editor(filtered_content);
                self.find_search_keyword(self.get_editor_text());
            }
        }
    }
}
