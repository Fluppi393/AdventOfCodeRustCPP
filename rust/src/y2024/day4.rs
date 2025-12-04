use crate::file_parser::FileParser;

fn solve(file_parser: &mut FileParser) -> (u32, u32) {
    fn get_dir(dir: i32) -> (i32, i32) {
        match dir {
            0 => (0, -1),
            1 => (1, -1),
            2 => (1, 0),
            3 => (1, 1),
            4 => (0, 1),
            5 => (-1, 1),
            6 => (-1, 0),
            7 => (-1, -1),
            _ => (0, 0),
        }
    }

    // Build a matrix of the characters
    let mut matrix: Vec<char> = Vec::new();
    let mut width: i32 = 0;
    let mut height: i32 = 0;
    while file_parser.next_line() {
        height += 1;
        width = 0;
        while !file_parser.is_line_done() {
            matrix.push(file_parser.consume_any_char().unwrap());
            width += 1;
        }
    }

    // Helper to get the character at coordinates
    let get_char = |x: i32, y: i32| -> Option<char> {
        if x < 0 || x >= width || y < 0 || y >= matrix.len() as i32 / width {
            None
        } else {
            Some(matrix[(x + y * width) as usize])
        }
    };

    // Search for all occurrences of XMAS
    let mut result: u32 = 0;
    let search_chars: Vec<char> = "XMAS".chars().collect();
    for x in 0..width {
        for y in 0..height {
            if get_char(x, y) != Some(search_chars[0]) {
                continue;
            }

            // Test in 8 directions
            for dir in 0..8 {
                let (dx, dy) = get_dir(dir);
                for dist in 1..4 {
                    let new_x = x + dx * dist;
                    let new_y = y + dy * dist;

                    if get_char(new_x, new_y) != Some(search_chars[dist as usize]) {
                        break;
                    }

                    // Count if we found the last char
                    if dist == 3 {
                        result += 1;
                    }
                }
            }
        }
    }

    // Search for all occurrences of X-MAS
    let mut result_2 = 0;
    for x in 1..width - 1 {
        for y in 1..height - 1 {
            // Search for the center A
            if get_char(x, y) != Some('A') {
                continue;
            }

            let test_diagonal = |x_dir: i32, y_dir: i32| -> bool {
                let first_char = get_char(x + x_dir, y + y_dir);
                let second_char = get_char(x - x_dir, y - y_dir);
                first_char == Some('M') && second_char == Some('S')
                    || first_char == Some('S') && second_char == Some('M')
            };

            if test_diagonal(1, 1) && test_diagonal(-1, 1) {
                result_2 += 1;
            }
        }
    }

    (result, result_2)
}

#[test]
fn day4() {
    assert_eq!(solve(&mut FileParser::new("2024/day4_test.txt")), (18, 9));
    assert_eq!(solve(&mut FileParser::new("2024/day4.txt")), (2551, 1985));
}
