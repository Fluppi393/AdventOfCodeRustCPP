use crate::file_parser::FileParser;

pub(crate) fn main(file_parser: &mut FileParser) -> (String, String) {
    let mut result = 0;
    let mut result_2 = 0;
    let mut mul_enabled = true;
    while let line_parser = file_parser.parse_line() {
        let mut line_parser = line_parser.unwrap();

        while !line_parser.is_done() {
            if line_parser.consume_string("do()") {
                mul_enabled = true;
                continue;
            }

            if line_parser.consume_string("don't()") {
                mul_enabled = false;
                continue;
            }

            if line_parser.consume_string("mul(") {
                let first_num = line_parser.consume_i32();
                if first_num.is_some() {
                    if line_parser.consume_char(',') {
                        let second_num = line_parser.consume_i32();
                        if second_num.is_some() {
                            if line_parser.consume_char(')') {
                                let mul = first_num.unwrap() * second_num.unwrap();
                                result += mul;

                                if mul_enabled {
                                    result_2 += mul;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    (result.to_string(), result_2.to_string())
}
