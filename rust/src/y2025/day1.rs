use crate::file_parser::FileParser;

fn solve(file_parser: &mut FileParser) -> (i32, i32) {
    let mut result = 0;
    let mut result2 = 0;

    let mut pos = 50;
    while file_parser.next_line() {
        let dir = file_parser.consume_any_char().unwrap();
        let dist = file_parser.consume_i32().unwrap();

        let sign = if dir == 'L' { -1 } else { 1 };
        for i in 0..dist {
            pos = pos + sign;

            if pos > 99 {
                pos = 0;
            } else if pos < 0 {
                pos = 99;
            }

            if pos == 0 {
                result2 += 1;
            }
        }

        if pos == 0 {
            result += 1;
        }
    }

    (result, result2)
}

#[test]
fn day1() {
    assert_eq!(solve(&mut FileParser::new("2025/day1_test.txt")), (3, 6));
    assert_eq!(solve(&mut FileParser::new("2025/day1.txt")), (1147, 6789));
}
