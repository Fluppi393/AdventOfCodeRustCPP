mod day1;
mod day2;
mod day3;
mod file_parser;

use std::io::{self};
use chrono;
use chrono::Datelike;
use crate::file_parser::FileParser;

fn main() -> io::Result<()> {
    let mut day = chrono::offset::Local::now().day();
   day = 3;

    for i in 0..2 {
        let is_test = i == 0;
        let suffix = if is_test { "_test" } else { "" };
        let filename = format!("input/day{}{}.txt", day, suffix);
        let mut file_parser = FileParser::new(&filename);

        let result = match day {
            //1 => day1::main(file_parser),
            //2 => day2::main(file_parser),
            3 => day3::main(&mut file_parser),
            _ => ("Not implemented".to_string(), "Not implemented".to_string()),
        };

        let text = if is_test { "Test" } else { "Result" };
        println!("{} for day {}: Part1 = {} Part2 = {}", text, day, result.0, result.1);
    }

    Ok(())
}
