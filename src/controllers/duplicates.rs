use crate::models::{duplicate_finder::DuplicateFinder, new_line::NewLine};
use std::collections::{HashMap, HashSet};

impl DuplicateFinder {
    pub fn find_duplicates(&mut self, editor_text: String) {
        let new_lines = self.parse_editor_content(editor_text);

        self.duplicate_lines.clear();

        let flattened_new_lines: Vec<String> = new_lines
            .clone()
            .into_iter()
            .map(|line| line.content)
            .collect();

        let unique_contents: Vec<String> = flattened_new_lines
            .into_iter()
            .collect::<HashSet<String>>()
            .into_iter()
            .collect();

        for unique_content in unique_contents {
            let count = new_lines
                .clone()
                .iter()
                .filter(|&x| x.content == unique_content)
                .count();

            let duplicates: Vec<NewLine> = new_lines
                .clone()
                .into_iter()
                .filter(|line| {
                    !line.content.is_empty()
                        && line.content != "\n"
                        && line.content == unique_content
                        && count > 1
                })
                .collect();

            for duplicate in duplicates {
                self.duplicate_lines.push(duplicate);
            }

            self.duplicate_lines.sort_by_key(|line| line.line_number);
        }
    }

    pub fn remove_duplicates(&mut self, editor_text: String) -> String {
        let new_lines = self.parse_editor_content(editor_text);

        let mut seen = HashSet::new();
        let mut filtered_lines = Vec::new();

        for line in new_lines {
            if seen.insert(line.content.clone()) || line.content.clone() == "" {
                filtered_lines.push(line.content.clone());
            }
        }

        filtered_lines.join("\n")
    }
}

#[cfg(test)]
mod duplicates_should {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("", vec![])]
    #[case("\n\n", vec![])]
    #[case("hi\nhi", vec![NewLine { line_number: 1, content: "hi".to_string() }, NewLine { line_number: 2, content: "hi".to_string() }])]
    #[case("hi\nhi\nlow", vec![NewLine { line_number: 1, content: "hi".to_string() }, NewLine { line_number: 2, content: "hi".to_string() }])]
    #[case("high\nhi\nhi\nlow", vec![NewLine { line_number: 2, content: "hi".to_string() }, NewLine { line_number: 3, content: "hi".to_string() }])]
    #[case("\n\nhi\nhi\nlow\ntnt\nlow\nspade\nhi", vec![NewLine { line_number: 3, content: "hi".to_string() }, NewLine { line_number: 4, content: "hi".to_string() }, NewLine { line_number: 5, content: "low".to_string() }, NewLine { line_number: 7, content: "low".to_string() }, NewLine { line_number: 9, content: "hi".to_string() }])]
    fn test_find_duplicates(
        #[case] editor_text: String,
        #[case] expected_duplicates: Vec<NewLine>,
    ) {
        // Given
        let mut duplicate_finder = DuplicateFinder {
            ..Default::default()
        };

        // When
        duplicate_finder.find_duplicates(editor_text);

        // Then
        pretty_assertions::assert_eq!(expected_duplicates, duplicate_finder.duplicate_lines);
    }

    #[rstest]
    #[case("", "")]
    #[case("hi", "hi")]
    #[case("hi\nhi", "hi")]
    #[case("hi\nhi\nthere", "hi\nthere")]
    #[case("hi\nhi\nthere\nthere", "hi\nthere")]
    #[case("hi\nhi\nthere\nwhere", "hi\nthere\nwhere")]
    #[case("hi\n\nhi\nthere\n\nwhere", "hi\n\nthere\n\nwhere")]
    #[case("\n\nhi\nhi\nlow\ntnt\nlow\nspade\nhi", "\n\nhi\nlow\ntnt\nspade")]
    fn test_remove_duplicates(#[case] editor_text: String, #[case] expected_content: String) {
        // Given
        let mut duplicate_finder = DuplicateFinder {
            ..Default::default()
        };

        // When
        let content = duplicate_finder.remove_duplicates(editor_text);

        // Then
        pretty_assertions::assert_eq!(expected_content, content);
    }
}
