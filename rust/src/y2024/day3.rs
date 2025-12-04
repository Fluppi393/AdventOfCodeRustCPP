use crate::file_parser::FileParser;

fn solve(file_parser: &mut FileParser) -> (i32, i32) {
    let mut result = 0;
    let mut result_2 = 0;
    let mut mul_enabled = true;
    while file_parser.next_char_or_line() {
        if file_parser.consume_string("do()") {
            println!("do()");
            mul_enabled = true;
            continue;
        }

        if file_parser.consume_string("don't()") {
            println!("don't()");
            mul_enabled = false;
            continue;
        }

        if file_parser.consume_string("mul(") {
            let first_num = file_parser.consume_i32();
            if first_num.is_some() {
                if file_parser.consume_char(',') {
                    let second_num = file_parser.consume_i32();
                    if second_num.is_some() {
                        if file_parser.consume_char(')') {
                            let mul = first_num.unwrap() * second_num.unwrap();
                            result += mul;

                            println!(
                                "mul({}, {}) = {}",
                                first_num.unwrap(),
                                second_num.unwrap(),
                                mul
                            );

                            if mul_enabled {
                                result_2 += mul;
                            }
                        }
                    }
                }
            }
        }
    }

    (result, result_2)
}

#[test]
fn day3() {
    assert_eq!(solve(&mut FileParser::new("2024/day3_test.txt")), (161, 48));
    assert_eq!(
        solve(&mut FileParser::new("2024/day3.txt")),
        (178886550, 87163705)
    );
}
