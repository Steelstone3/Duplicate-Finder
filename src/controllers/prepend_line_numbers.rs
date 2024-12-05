use iced::widget::text_editor::{Action, Edit};

use crate::models::duplicate_finder::DuplicateFinder;

impl DuplicateFinder {
    pub fn prepend_line_numbers(&mut self) {
        self.content = self.application_state.content.text();

        if !self.application_state.is_line_number_used {
            let updated_content = self
                .content
                .lines() // Split the string by '\n' into an iterator
                .enumerate() // Enumerate each line with its index
                .map(|(index, line)| format!("{}:: {}", index + 1, line.to_string())) // Prepend line number (1-based)
                .collect::<Vec<String>>() // Collect into a Vec<String>
                .join("\n"); // Join the lines back together with newlines

            self.application_state.content.perform(Action::SelectAll);
            self.application_state
                .content
                .perform(Action::Edit(Edit::Paste(updated_content.into())));
            self.application_state.is_line_number_used = true;
        } else {
            let updated_content = self
                .content
                .lines()
                .map(|line| format!("{}", line.splitn(2, ":: ").nth(1).unwrap_or(line)))
                .collect::<Vec<String>>()
                .join("\n");
            self.application_state.content.perform(Action::SelectAll);
            self.application_state
                .content
                .perform(Action::Edit(Edit::Paste(updated_content.into())));

            self.application_state.is_line_number_used = false;
        }
    }
}
