use crate::models::duplicate_finder::DuplicateFinder;

impl DuplicateFinder {
    pub fn find_search_keyword(&mut self, get_editor_text: String) {
        todo!()
    }

    pub fn remove_search_keyword(&mut self, get_editor_text: String) -> String {
        todo!()
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
    #[case("hi", "\n\nhi\nhi\nlow\ntnt\nlow\nspade\nhi", vec![NewLine { line_number: 3, content: "hi".to_string() }, NewLine { line_number: 4, content: "hi".to_string() }, NewLine { line_number: 9, content: "hi".to_string() }, NewLine { line_number: 5, content: "low".to_string() }, NewLine { line_number: 7, content: "low".to_string() }])]
    #[case("spade", "\n\nhi\nhi\nlow\ntnt\nlow\nspade\nhi", vec![NewLine { line_number: 6, content: "spade".to_string() }])]
    fn test_find_search_keyword(
        #[case] search_keyword: String,
        #[case] editor_text: String,
        #[case] expected_duplicates: Vec<NewLine>,
    ) {
        // Given
        let mut duplicate_finder = DuplicateFinder {
            search_keyword,
            ..Default::default()
        };

        // When
        duplicate_finder.find_search_keyword(editor_text);

        // Then
        assert_eq!(
            expected_duplicates.len(),
            duplicate_finder.duplicate_lines.len()
        );
        for expected_duplicate in expected_duplicates {
            assert!(
                duplicate_finder
                    .duplicate_lines
                    .contains(&expected_duplicate)
            );
        }
    }

    #[rstest]
    #[case("", "", "")]
    #[case("", "\n\n", "\n\n")]
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
        assert_eq!(expected_content, content);
    }
}
