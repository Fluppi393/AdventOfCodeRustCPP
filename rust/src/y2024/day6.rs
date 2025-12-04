use crate::file_parser::FileParser;
use std::cmp::PartialEq;

#[derive(Clone, Copy, Eq, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum Field {
    Empty,
    Wall,
    Visited(Direction),
}

fn solve(file_parser: &mut FileParser) -> (i32, i32) {
    let mut start_x = 0;
    let mut start_y = 0;
    let mut start_index = 0;

    // Build the map
    let mut map: Vec<Field> = Vec::new();
    let mut width: i32 = 0;
    let mut height: i32 = 0;
    while file_parser.next_line() {
        height += 1;
        width = 0;
        while let Some(c) = file_parser.consume_any_char() {
            map.push(if c == '#' { Field::Wall } else { Field::Empty });
            width += 1;

            if c == '^' {
                start_x = width - 1;
                start_y = height - 1;
                start_index = map.len() - 1;
            }
        }
    }

    fn get_index(width: i32, height: i32, x: i32, y: i32) -> Option<usize> {
        if x >= 0 && x < width && y >= 0 && y < height {
            Some((x + y * width) as usize)
        } else {
            None
        }
    }

    fn do_the_walk(
        map: &mut Vec<Field>,
        width: i32,
        height: i32,
        start_x: i32,
        start_y: i32,
    ) -> Option<i32> {
        let mut num_visited = 1;
        let mut found_loop = false;
        let mut dir = Direction::Up;
        let mut x = start_x;
        let mut y = start_y;
        map[get_index(width, height, x, y).unwrap()] = Field::Visited(dir);

        loop {
            let (new_x, new_y) = match dir {
                Direction::Up => (x, y - 1),
                Direction::Right => (x + 1, y),
                Direction::Down => (x, y + 1),
                Direction::Left => (x - 1, y),
            };

            let index = get_index(width, height, new_x, new_y);
            if index.is_none() {
                break;
            }

            let index = index.unwrap();
            match map[index] {
                Field::Wall => {
                    dir = match dir {
                        Direction::Up => Direction::Right,
                        Direction::Right => Direction::Down,
                        Direction::Down => Direction::Left,
                        Direction::Left => Direction::Up,
                    };
                }
                Field::Empty => {
                    x = new_x;
                    y = new_y;
                    map[index] = Field::Visited(dir);
                    num_visited += 1;
                }
                Field::Visited(visited_dir) => {
                    if visited_dir == dir {
                        found_loop = true;
                        break;
                    } else {
                        x = new_x;
                        y = new_y;
                    }
                }
            }
        }

        //print_map(map, width, height);

        // Reset the map
        for i in 0..map.len() {
            map[i] = match map[i] {
                Field::Visited(_) => Field::Empty,
                _ => map[i],
            };
        }

        if found_loop {
            None
        } else {
            Some(num_visited)
        }
    }

    fn print_map(map: &mut Vec<Field>, width: i32, height: i32) {
        for y in 0..height {
            for x in 0..width {
                let index = get_index(width, height, x, y).unwrap();
                let c = match map[index] {
                    Field::Empty => '.',
                    Field::Wall => '#',
                    Field::Visited(Direction::Up) => '^',
                    Field::Visited(Direction::Right) => '>',
                    Field::Visited(Direction::Down) => 'v',
                    Field::Visited(Direction::Left) => '<',
                };
                print!("{}", c);
            }
            println!();
        }
    }

    // Count steps
    let num_steps = do_the_walk(&mut map, width, height, start_x, start_y).unwrap();

    // Test obstructions
    let mut num_obstructions = 0;
    for i in 0..map.len() {
        if i == start_index || map[i] != Field::Empty {
            continue;
        }

        // Temporarily mark the field as an obstacle
        map[i] = Field::Wall;
        num_obstructions += if do_the_walk(&mut map, width, height, start_x, start_y).is_some() {
            0
        } else {
            1
        };
        map[i] = Field::Empty;
    }

    (num_steps, num_obstructions)
}

#[test]
fn day6() {
    assert_eq!(solve(&mut FileParser::new("2024/day6_test.txt")), (41, 6));
    assert_eq!(solve(&mut FileParser::new("2024/day6.txt")), (5101, 1951));
}
