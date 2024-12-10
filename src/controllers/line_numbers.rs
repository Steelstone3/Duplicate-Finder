use crate::models::duplicate_finder::DuplicateFinder;

impl DuplicateFinder {
    pub fn toggle_prepend_line_numbers(&mut self, editor_content: String) -> String {
        self.content = self.application_state.content.text();

        match self.application_state.is_line_number_used {
            true => self.remove_prepend(editor_content),
            false => self.add_prepend(editor_content),
        }
    }

    pub fn remove_prepend(&mut self, editor_content: String) -> String {
        if !self.application_state.is_line_number_used {
            return editor_content;
        }

        self.content = editor_content;
        self.application_state.is_line_number_used = false;

        self.content
            .lines()
            .map(|line| {
                line.split_once(":: ")
                    .map(|x| x.1)
                    .unwrap_or(line)
                    .to_string()
            })
            .collect::<Vec<String>>()
            .join("\n")
    }

    fn add_prepend(&mut self, editor_content: String) -> String {
        if self.application_state.is_line_number_used {
            return editor_content;
        }

        self.content = editor_content;
        self.application_state.is_line_number_used = true;

        self.content
            .lines()
            .enumerate()
            .map(|(index, line)| format!("{}:: {}", index + 1, line))
            .collect::<Vec<String>>()
            .join("\n")
    }
}

#[cfg(test)]
mod line_numbers_should {
}