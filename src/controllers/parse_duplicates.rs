use crate::models::{duplicate_finder::DuplicateFinder, new_line::NewLine};
use std::collections::HashMap;

impl DuplicateFinder {
    pub fn find_duplicates(&mut self) {
        let new_lines = self.parse_editor_content();

        let mut content_map: HashMap<String, Vec<NewLine>> = HashMap::new();

        for line in new_lines {
            content_map
                .entry(line.content.clone())
                .or_insert_with(Vec::new)
                .push(line);
        }

        self.duplicate_lines = content_map
            .into_iter()
            .flat_map(|(line, duplicates)| {
                if line.is_empty() {
                    vec![]
                }
                else if duplicates.len() > 1 {
                    duplicates
                } else {
                    vec![]
                }
            })
            .collect();
    }

    fn parse_editor_content(&mut self) -> Vec<NewLine> {
        self.content = self.application_state.content.text();

        self.content
            .lines()
            .enumerate()
            .map(|(index, content)| NewLine {
                line_number: index as u128 + 1,
                content: content.to_string(),
            })
            .collect()
    }
}
