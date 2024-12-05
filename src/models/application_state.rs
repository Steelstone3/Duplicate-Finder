use crate::commands::tab_identifier::TabIdentifier;
use iced::widget::text_editor;

#[derive(Default)]
pub struct ApplicationState {
    pub is_line_number_used: bool,
    pub is_light_theme: bool,
    pub tab: TabIdentifier,
    pub content: text_editor::Content,
}