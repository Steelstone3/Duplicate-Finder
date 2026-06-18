use super::{application_state::ApplicationState, new_line::NewLine};

#[derive(Default)]
pub struct DuplicateFinder {
    pub application_state: ApplicationState,
    pub search_keyword: String,
    pub content: String,
    pub duplicate_lines: Vec<NewLine>,
    pub found_lines: Vec<NewLine>,
}
