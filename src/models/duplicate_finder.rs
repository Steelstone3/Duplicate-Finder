
use super::{application_state::ApplicationState, new_line::NewLine};

#[derive(Default)]
pub struct DuplicateFinder {
    pub application_state: ApplicationState,
    pub content: String,
    pub duplicate_lines: Vec<NewLine>,
}
