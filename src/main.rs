mod day1;
mod day2;

use std::fs::File;
use std::io::{self, BufReader};
use chrono;
use chrono::Datelike;

fn main() -> io::Result<()> {
    let mut day = chrono::offset::Local::now().day();
   // day = 2;

    for i in 0..2 {
        let is_test = i == 0;
        let suffix = if is_test { "_test" } else { "" };
        let filename = format!("input/day{}{}.txt", day, suffix);
        let file = File::open(filename)?;
        let reader = BufReader::new(file);

        let result = match day {
            1 => day1::main(reader),
            2 => day2::main(reader),
            _ => ("Not implemented".to_string(), "Not implemented".to_string()),
        };

        let text = if is_test { "Test" } else { "Result" };
        println!("{} for day {}: Part1 = {} Part2 = {}", text, day, result.0, result.1);
    }

    Ok(())
}
