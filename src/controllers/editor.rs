use crate::models::{duplicate_finder::DuplicateFinder, new_line::NewLine};
use iced::widget::text_editor::{Action, Edit};

impl DuplicateFinder {
    pub fn get_editor_text(&self) -> String {
        self.application_state.content.text()
    }

    pub fn refresh_editor(&mut self, content: String) {
        self.application_state.content.perform(Action::SelectAll);

        self.application_state
            .content
            .perform(Action::Edit(Edit::Paste(content.into())));
    }

    pub fn parse_editor_content(&mut self, editor_text: String) -> Vec<NewLine> {
        self.content = editor_text;

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

#[cfg(test)]
mod editor_should {
    use crate::models::{duplicate_finder::DuplicateFinder, new_line::NewLine};
    use rstest::rstest;

    #[rstest]
    #[case("", vec![])]
    #[case("\n", vec![NewLine{ line_number: 1, content: "".to_string() }])]
    #[case("hi\nhi", vec![NewLine{ line_number: 1, content: "hi".to_string() }, NewLine{ line_number: 2, content: "hi".to_string() }])]
    #[case("hi\n\nthere", vec![NewLine{ line_number: 1, content: "hi".to_string() },NewLine{ line_number: 2, content: "".to_string() },  NewLine{ line_number: 3, content: "there".to_string() }])]
    #[case("\nhi\n\nthere\n", vec![NewLine { line_number: 1, content: "".to_string() }, NewLine { line_number: 2, content: "hi".to_string() }, NewLine { line_number: 3, content: "".to_string()}, NewLine { line_number: 4, content: "there".to_string() }])]
    #[case("\n\nhi\nhi\nlow\ntnt\nlow\nspade\nhi", vec![NewLine { line_number: 1, content: "".to_string() }, NewLine { line_number: 2, content: "".to_string() }, NewLine { line_number: 3, content: "hi".to_string() }, NewLine { line_number: 4, content: "hi".to_string() }, NewLine { line_number: 5, content: "low".to_string() },  NewLine { line_number: 6, content: "tnt".to_string() }, NewLine { line_number: 7, content: "low".to_string() },  NewLine { line_number: 8, content: "spade".to_string() }, NewLine { line_number: 9, content: "hi".to_string() }])]
    fn test_parse_editor_content(
        #[case] editor_text: String,
        #[case] expected_new_lines: Vec<NewLine>,
    ) {
        // Given
        let mut duplicate_finder = DuplicateFinder {
            ..Default::default()
        };

        // When
        let new_lines = duplicate_finder.parse_editor_content(editor_text);

        // Then
        pretty_assertions::assert_eq!(expected_new_lines.len(), new_lines.len());
        for expected_new_line in expected_new_lines {
            assert!(new_lines.contains(&expected_new_line));
        }
    }
}
