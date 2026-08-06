use crate::models::duplicate_finder::DuplicateFinder;

impl DuplicateFinder {
    pub fn trim_lines(&mut self, editor_text: String) -> String {
        let new_lines = self.parse_editor_content(editor_text);

        new_lines
            .into_iter()
            .map(|line| line.content)
            .filter(|content| !content.trim().is_empty())
            .collect::<Vec<_>>()
            .join("\n")
    }
}

#[cfg(test)]
mod line_search_should {
    use crate::models::duplicate_finder::DuplicateFinder;
    use rstest::rstest;

    #[rstest]
    #[case("", "")]
    #[case("hi\nhi", "hi\nhi")]
    #[case("hi\n\nhi", "hi\nhi")]
    #[case("hi\n\n\nhi", "hi\nhi")]
    fn test_trim_lines(#[case] editor_text: String, #[case] expected_editor_text: String) {
        // Given
        let mut duplicate_finder = DuplicateFinder {
            content: editor_text.clone(),
            ..Default::default()
        };

        // When
        let content = duplicate_finder.trim_lines(editor_text.clone());

        // Then
        pretty_assertions::assert_eq!(expected_editor_text, content)
    }
}
