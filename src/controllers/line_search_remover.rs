use crate::models::{duplicate_finder::DuplicateFinder, new_line::NewLine};

impl DuplicateFinder {
    pub fn find_search_keyword(&mut self, editor_text: String) {
        self.found_lines.clear();

        let new_lines = self.parse_editor_content(editor_text);

        for line in new_lines {
            if !self.search_keyword.is_empty() && line.content.contains(&self.search_keyword) {
                self.found_lines.push(line);
            }
        }
    }

    pub fn remove_search_keyword(&mut self, editor_text: String) -> String {
        let new_lines = self.parse_editor_content(editor_text);

        let mut flattened_filtered_lines = vec![];

        if self.search_keyword.is_empty() {
            for line in new_lines {
                flattened_filtered_lines.push(line.content);
            }
        } else {
            let filtered_lines: Vec<NewLine> = new_lines
                .into_iter()
                .filter(|found_line| !found_line.content.contains(&self.search_keyword))
                .collect();

            for line in filtered_lines {
                flattened_filtered_lines.push(line.content);
            }
        }

        flattened_filtered_lines.join("\n")
    }
}

#[cfg(test)]
mod line_search_should {
    use crate::models::duplicate_finder::DuplicateFinder;
    use crate::models::new_line::NewLine;
    use rstest::rstest;

    #[rstest]
    #[case("", "", vec![])]
    #[case("", "\n\n", vec![])]
    #[case("hi", "hi\nhi", vec![NewLine { line_number: 1, content: "hi".to_string() }, NewLine { line_number: 2, content: "hi".to_string() }])]
    #[case("hi", "hi\nhi\nlow", vec![NewLine { line_number: 1, content: "hi".to_string() }, NewLine { line_number: 2, content: "hi".to_string() }])]
    #[case("", "hi\nhi\nlow", vec![])]
    #[case("high", "high\nhi\nhi\nlow", vec![NewLine { line_number: 1, content: "high".to_string() }])]
    #[case("hi", "\n\nhi\nhi\nlow\ntnt\nlow\nspade\nhi", vec![NewLine { line_number: 3, content: "hi".to_string() }, NewLine { line_number: 4, content: "hi".to_string() }, NewLine { line_number: 9, content: "hi".to_string() }])]
    #[case("spade", "\n\nhi\nhi\nlow\ntnt\nlow\nspade\nhi", vec![NewLine { line_number: 8, content: "spade".to_string() }])]
    fn test_find_search_keyword(
        #[case] search_keyword: String,
        #[case] editor_text: String,
        #[case] expected_found_lines: Vec<NewLine>,
    ) {
        // Given
        let mut duplicate_finder = DuplicateFinder {
            search_keyword,
            ..Default::default()
        };

        // When
        duplicate_finder.find_search_keyword(editor_text);

        // Then
        pretty_assertions::assert_eq!(expected_found_lines, duplicate_finder.found_lines);
    }

    #[rstest]
    #[case("", "", "")]
    #[case("", "\n", "")]
    #[case("", "\n\n", "\n")]
    #[case("hi", "hi\nhi", "")]
    #[case("hi", "hi\nhi\nlow", "low")]
    #[case("", "hi\nhi\nlow", "hi\nhi\nlow")]
    #[case("high", "high\nhi\nhi\nlow", "hi\nhi\nlow")]
    #[case(
        "hi",
        "\n\nhi\nhi\nlow\ntnt\nlow\nspade\nhi",
        "\n\nlow\ntnt\nlow\nspade"
    )]
    #[case(
        "spade",
        "\n\nhi\nhi\nlow\ntnt\nlow\nspade\nhi",
        "\n\nhi\nhi\nlow\ntnt\nlow\nhi"
    )]
    fn test_remove_duplicates(
        #[case] search_keyword: String,
        #[case] editor_text: String,
        #[case] expected_content: String,
    ) {
        // Given
        let mut duplicate_finder = DuplicateFinder {
            search_keyword,
            ..Default::default()
        };

        // When
        let content = duplicate_finder.remove_search_keyword(editor_text);

        // Then
        pretty_assertions::assert_eq!(expected_content, content);
    }
}
