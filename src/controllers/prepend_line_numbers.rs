use iced::widget::text_editor::{Action, Edit};

use crate::models::duplicate_finder::DuplicateFinder;

impl DuplicateFinder {
    pub fn toggle_prepend_line_numbers(&mut self) {
        self.content = self.application_state.content.text();

        if !self.application_state.is_line_number_used {
            self.add_prepend();
        } else {
            self.remove_prepend();
        }
    }

    pub fn remove_prepend(&mut self) {
        if !self.application_state.is_line_number_used {
            return;
        }

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

    fn add_prepend(&mut self) {
        if self.application_state.is_line_number_used {
            return;
        }

        let updated_content = self
            .content
            .lines()
            .enumerate()
            .map(|(index, line)| format!("{}:: {}", index + 1, line.to_string()))
            .collect::<Vec<String>>()
            .join("\n");

        self.application_state.content.perform(Action::SelectAll);
        self.application_state
            .content
            .perform(Action::Edit(Edit::Paste(updated_content.into())));
        self.application_state.is_line_number_used = true;
    }
}
