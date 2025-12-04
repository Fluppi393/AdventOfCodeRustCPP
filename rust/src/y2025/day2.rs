use crate::file_parser::FileParser;

fn count_digits(num: u64) -> u64 {
    let mut current = num;
    let mut result = 0;
    while current > 0 {
        current /= 10;
        result += 1;
    }

    result
}

fn mask_int(value: u64, start: u64, length: u64) -> u64 {
    let mut result = value / 10_u64.pow(start as u32);
    result -= result / 10_u64.pow(length as u32) * 10_u64.pow(length as u32);
    result
}

// First return value is for part 1, second for part 2
fn is_silly_number(value: u64) -> (bool, bool) {
    let digits = count_digits(value);
    let half_digits = digits / 2;
    let digits_even = half_digits * 2 == digits;

    // Try out all possible pattern lengths
    // Iterate in reverse so we can catch solutions for part 1
    for pattern_length in (1..=half_digits).rev() {
        let num_patterns = digits / pattern_length;
        if num_patterns * pattern_length != digits {
            continue;
        }

        let pattern = mask_int(value, 0, pattern_length);

        // Check if any pattern has a mismatch
        let mut mismatch = false;
        for i in 0..num_patterns {
            let other_pattern = mask_int(value, pattern_length * i, pattern_length);
            if other_pattern != pattern {
                mismatch = true;
                break;
            }
        }

        if mismatch {
            continue;
        }

        return (pattern_length == half_digits && digits_even, true);
    }

    (false, false)
}

fn solve(file_parser: &mut FileParser) -> (u64, u64) {
    let mut result = 0;
    let mut result2 = 0;

    file_parser.next_line();

    while !file_parser.is_line_done() {
        let start: u64 = file_parser.consume_numeric().unwrap();
        file_parser.consume_char('-');
        let end: u64 = file_parser.consume_numeric().unwrap();

        for num in start..=end {
            let (is_silly_1, is_silly_2) = is_silly_number(num);
            if is_silly_1 {
                result += num;
            }
            if is_silly_2 {
                result2 += num;
            }
        }

        file_parser.consume_char(',');
    }

    (result, result2)
}

#[test]
fn day2() {
    assert_eq!(mask_int(12345, 2, 1), 3);
    assert_eq!(mask_int(12345, 1, 3), 234);
    assert_eq!(is_silly_number(123123), (true, true));
    assert_eq!(is_silly_number(1231231), (false, false));
    assert_eq!(is_silly_number(123123123), (false, true));

    assert_eq!(
        solve(&mut FileParser::new("2025/day2_test.txt")),
        (1227775554, 4174379265)
    );
    assert_eq!(
        solve(&mut FileParser::new("2025/day2.txt")),
        (31000881061, 46769308485)
    );
}
