
use iced::widget::text_editor;

use super::tab_identifier::TabIdentifier;

#[derive(Debug, Clone, PartialEq)]
pub enum Message {
    MenuBar,
    FileNew,
    #[allow(dead_code)]
    EditUndo,
    #[allow(dead_code)]
    EditRedo,
    SelectedTabChanged(TabIdentifier),
    ViewPrependLineNumbersPressed,
    ViewThemeChanged,
    EditorChanged(text_editor::Action),
    DuplicateSearchPressed
}
