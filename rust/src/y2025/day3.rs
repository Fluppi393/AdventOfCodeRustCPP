use crate::file_parser::FileParser;

fn calc_joltage(input: &str, num_digits: usize) -> u64 {
    let mut result = 0;

    // Convert the input into a vector of digits
    let digits: Vec<u32> = input.chars().map(|c| c.to_digit(10).unwrap()).collect();

    let mut last_index: Option<usize> = None;
    for i in 0..num_digits {
        let start = last_index.map_or(0, |s| s + 1);
        let end = input.len() - (num_digits - i - 1);
        let slice = &digits[start..end];

        let (index, digit) = slice
            .iter()
            .enumerate()
            .rev()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .unwrap();

        // NOTE: Since we used a slice, we have to retarget the index to the original vector
        last_index = Some(index + start);

        result += *digit as u64 * 10_u64.pow((num_digits - i - 1) as u32);
    }

    result
}

fn solve(file_parser: &mut FileParser) -> (u64, u64) {
    let mut result = 0;
    let mut result2 = 0;

    while file_parser.next_line() {
        result += calc_joltage(file_parser.get_line(), 2);
        result2 += calc_joltage(file_parser.get_line(), 12);
    }

    (result, result2)
}

#[test]
fn day3() {
    assert_eq!(
        solve(&mut FileParser::new("2025/day3_test.txt")),
        (357, 3121910778619)
    );
    assert_eq!(
        solve(&mut FileParser::new("2025/day3.txt")),
        (17196, 171039099596062)
    );
}
