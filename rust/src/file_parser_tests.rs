#[cfg(test)]
mod file_parser_tests {
    use crate::file_parser::FileParser;

    #[test]
    fn test_file_parser() {
        let mut file_parser = FileParser::inline("line1\nline2\nline3\nline4\nline5\n1234abcd5678efgh");

        // Go to the first line
        assert_eq!(file_parser.next_line(), true);
        assert_eq!(file_parser.is_line_done(), false);

        // Test get_char, consume_char and consume_any_char
        assert_eq!(file_parser.get_char(0), Some('l'));
        assert_eq!(file_parser.get_char(1), Some('i'));
        assert_eq!(file_parser.consume_char('l'), true);
        assert_eq!(file_parser.consume_char('i'), true);
        assert_eq!(file_parser.consume_char('i'), false);
        assert_eq!(file_parser.consume_any_char(), Some('n'));
        assert_ne!(file_parser.consume_any_char(), Some('n'));
        assert_eq!(file_parser.consume_any_char(), Some('1'));
        // Line is at the end, there shouldn't be any more characters
        assert_eq!(file_parser.get_char(5), None);
        assert_eq!(file_parser.consume_char('1'), false);
        assert_eq!(file_parser.consume_any_char(), None);

        // Go to line 2
        assert_eq!(file_parser.next_line(), true);

        // Test next_char, get_next_char and is_char
        assert_eq!(file_parser.next_char(), true);
        assert_eq!(file_parser.get_next_char(), Some('i'));
        assert_ne!(file_parser.get_next_char(), Some('n'));
        assert_eq!(file_parser.next_char(), true);
        assert_eq!(file_parser.next_char(), true);
        assert_eq!(file_parser.is_char('e'), true);
        assert_eq!(file_parser.is_char('2'), false);

        // Go to line 3
        assert_eq!(file_parser.next_line(), true);

        // Test skip_num
        assert_eq!(file_parser.skip_num(0), false);
        assert_eq!(file_parser.skip_num(6), false);
        assert_eq!(file_parser.skip_num(5), true);

        // Go to line 4
        assert_eq!(file_parser.next_char_or_line(), true);

        // Test consume_num
        assert_eq!(file_parser.consume_num(0), None);
        assert_eq!(file_parser.consume_num(6), None);
        assert_eq!(file_parser.consume_num(5), Some("line4".to_string()));

        // Go to line 5
        assert_eq!(file_parser.next_char_or_line(), true);

        // Test is_string and consume_string
        assert_eq!(file_parser.is_string("lime"), false);
        assert_eq!(file_parser.is_string("line"), true);
        assert_eq!(file_parser.consume_string("lime"), false);
        assert_eq!(file_parser.consume_string("line"), true);

        // Go to line 6
        assert_eq!(file_parser.next_line(), true);

        // Test count_numeric and consume_i32
        assert_eq!(file_parser.count_numeric(), 4);
        assert_eq!(file_parser.consume_i32(), Some(1234));

        // Test consume_alphabetic and consume_alphabetic
        assert_eq!(file_parser.count_alphabetic(), 4);
        assert_eq!(file_parser.consume_alphabetic(), Some("abcd".to_string()));

        // Test consume_alphanumeric and consume_alphanumeric
        assert_eq!(file_parser.count_alphanumeric(), 8);
        assert_eq!(file_parser.consume_alphanumeric(), Some("5678efgh".to_string()));

        // Line should now be done
        assert_eq!(file_parser.is_line_done(), true);
        assert_eq!(file_parser.get_next_char(), None);
        // Make sure next_line fails after the first line
        assert_eq!(file_parser.next_line(), false);
    }
}