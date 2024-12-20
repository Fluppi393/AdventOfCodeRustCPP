use crate::file_parser::FileParser;
fn solve(file_parser: &mut FileParser) -> (u64, u64) {
    let mut result = 0;
    let mut result_2 = 0;
    let mut operants: Vec<u64> = Vec::new();

    fn concatenate_numbers(left: u64, right: u64) -> u64 {
        let mut left = left;
        let mut temp = right;
        while temp > 0 {
            temp /= 10;
            left *= 10;
        }
        left + right
    }

    fn is_equation_valid(
        result: u64,
        operants: &Vec<u64>,
        index: usize,
        temp_result: u64,
        allow_concat: bool,
    ) -> bool {
        if index == operants.len() {
            return result == temp_result;
        }

        let next = operants[index];
        is_equation_valid(
            result,
            operants,
            index + 1,
            temp_result + next,
            allow_concat,
        ) || is_equation_valid(
            result,
            operants,
            index + 1,
            temp_result * next,
            allow_concat,
        ) || allow_concat
            && is_equation_valid(
                result,
                operants,
                index + 1,
                concatenate_numbers(temp_result, next),
                allow_concat,
            )
    }

    while file_parser.next_line() {
        let left = file_parser.consume_numeric().unwrap();
        file_parser.consume_char(':');
        file_parser.consume_whitespace();

        operants.clear();
        while let Some(n) = file_parser.consume_numeric() {
            operants.push(n);
            file_parser.consume_whitespace();
        }

        if is_equation_valid(left, &operants, 1, operants[0], false) {
            result += left;
        }

        if is_equation_valid(left, &operants, 1, operants[0], true) {
            result_2 += left;
        }
    }

    (result, result_2)
}

#[test]
fn day7() {
    assert_eq!(
        solve(&mut FileParser::new("input/day7_test.txt")),
        (3749, 11387)
    );
    assert_eq!(
        solve(&mut FileParser::new("input/day7.txt")),
        (11387, 92148721834692)
    );
}
