use crate::file_parser::FileParser;

fn solve(file_parser: &mut FileParser) -> (i32, i32) {
    let mut result = 0;
    let mut result_2 = 0;

    // Parse the rules until we find the dividing blanc line
    let mut rules: Vec<(i32, i32)> = Vec::new();
    while file_parser.next_line() && !file_parser.is_line_done() {
        let left = file_parser.consume_i32().unwrap();
        file_parser.consume_char('|');
        let right = file_parser.consume_i32().unwrap();
        rules.push((left, right));
    }

    fn are_numbers_ordered(rules: &Vec<(i32, i32)>, left: i32, right: i32) -> bool {
        !rules.contains(&(right, left))
    }

    fn is_update_in_order(rules: &Vec<(i32, i32)>, update: &Vec<i32>) -> bool {
        for i in 0..update.len() - 1 {
            for j in i + 1..update.len() {
                if !are_numbers_ordered(rules, update[i], update[j]) {
                    return false;
                }
            }
        }

        true
    }

    fn reorder_update(rules: &Vec<(i32, i32)>, update: &mut Vec<i32>) {
        for i in 0..update.len() - 1 {
            for j in i + 1..update.len() {
                if !are_numbers_ordered(rules, update[i], update[j]) {
                    // Exchange
                    let temp = update[i];
                    update[i] = update[j];
                    update[j] = temp;
                }
            }
        }
    }

    // Parse the updates
    let mut update: Vec<i32> = Vec::new();
    while file_parser.next_line() {
        update.clear();

        loop {
            let num = file_parser.consume_i32().unwrap();
            update.push(num);

            // Check if we reached the last number
            if !file_parser.consume_char(',') {
                if is_update_in_order(&rules, &update) {
                    result += update[update.len() / 2];
                } else {
                    reorder_update(&rules, &mut update);
                    result_2 += update[update.len() / 2];
                }

                break;
            }
        }
    }

    (result, result_2)
}

#[test]
fn day5() {
    assert_eq!(
        solve(&mut FileParser::new("input/day5_test.txt")),
        (143, 123)
    );
    assert_eq!(solve(&mut FileParser::new("input/day5.txt")), (5208, 6732));
}
